mod error;
mod font;
mod style;

use crate::style::Style;
use cairo::{Context, ImageSurface};
use error::CssError;
use rayon::prelude::*;
use std::collections::HashMap;

/// Parse CSS into a HashMap of images.
///
/// # Examples
///
/// ```
/// use css_image::parse;
///
/// let css = r#"body { background-color: #FFFFFF; width: 100px; height: 100px; }"#.to_string();
/// let result = parse(css);
/// ```
pub fn parse(css: String) -> Result<HashMap<String, Vec<u8>>, CssError<'static>> {
    if css.is_empty() {
        return Err(CssError::ContentError("Empty CSS"));
    }

    let styles = css
        .par_split('}')
        .filter_map(|s| {
            let mut parts = s.trim().split('{');

            let name = parts.next()?.trim();
            let styling = parts.next()?.trim();

            if name.is_empty() || styling.is_empty() {
                return Some(Err(CssError::ContentError("Empty style")));
            }

            let styling = styling
                .par_split(';')
                .filter_map(|s| {
                    if s.is_empty() {
                        return None;
                    }

                    s.split_once(':')
                        .map(|(k, v)| (k.trim(), v.replace(['"', '\''], "")))
                })
                .collect::<Vec<(&str, String)>>();
            Some(Style::new(
                name,
                styling.iter().map(|(k, v)| (*k, v.trim())).collect(),
            ))
        })
        .collect::<Result<Vec<_>, CssError>>()?;

    styles
        .par_iter()
        .map(|style| {
            let mut width = style.dimensions.0;
            let mut height = style.dimensions.1;
            let mut position = 0;

            let mut text_width = 0;

            if style.text.is_some() {
                let surface = ImageSurface::create(cairo::Format::ARgb32, 0, 0)
                    .map_err(|_| CssError::ContentError("Failed to create cairo surface"))?;
                let context = Context::new(&surface)
                    .map_err(|_| CssError::ContentError("Failed to create cairo context"))?;
                let text = style.text.as_ref().ok_or(CssError::ContentError(""))?;

                context.select_font_face(text.family.as_str(), text.slant, text.weight);
                context.set_font_size(text.size);
                let extents = context
                    .text_extents(text.text.as_str())
                    .map_err(|_| CssError::ContentError(""))?;

                if width.is_none() {
                    width = Some(extents.width() as i32);
                }
                if height.is_none() {
                    height = Some(extents.height() as i32);
                }
                text_width = extents.width() as i32;
                position = extents.y_bearing().abs() as i32;
            }

            let width = width.ok_or(CssError::SizeError(
                "Width not found while content not provided",
            ))?;
            let height = height.ok_or(CssError::SizeError(
                "Height not found while content not provided",
            ))?;

            let margin = style.margin;
            let padding = style.padding;

            let surface = ImageSurface::create(
                cairo::Format::ARgb32,
                width + margin[1] + margin[3] + padding[1] + padding[3],
                height + margin[0] + margin[2] + padding[0] + padding[2],
            )
            .map_err(|_| CssError::ContentError("Failed to create cairo surface"))?;
            let mut img =
                Vec::with_capacity(surface.width() as usize * surface.height() as usize * 4);

            let context = Context::new(&surface)
                .map_err(|_| CssError::ContentError("Failed to create cairo context"))?;

            context.set_source_rgba(
                style.background[0],
                style.background[1],
                style.background[2],
                style.background[3],
            );
            draw_rectangle(
                &context,
                margin[3] as f64,
                margin[0] as f64,
                width as f64 + padding[1] as f64 + padding[3] as f64,
                height as f64 + padding[0] as f64 + padding[2] as f64,
                style.border_radius.unwrap_or(0.),
            );
            context
                .fill_preserve()
                .map_err(|_| CssError::ContentError("Failed to paint the surface"))?;

            if let Some(text) = &style.text {
                context.select_font_face(text.family.as_str(), text.slant, text.weight);
                context.set_font_size(text.size);
                context.set_source_rgba(text.color[0], text.color[1], text.color[2], 1.0);
                match text.text_align.as_str() {
                    "center" => {
                        context.move_to(
                            (width / 2 - text_width / 2) as f64 + padding[3] as f64,
                            position as f64 + padding[0] as f64,
                        );
                    }
                    "right" => {
                        context.move_to(width as f64 - text_width as f64, position as f64);
                    }
                    "left" => {
                        context
                            .move_to(0.0 + padding[3] as f64, position as f64 + padding[0] as f64);
                    }
                    _ => return Err(CssError::ContentError("Invalid text-align")),
                }
                _ = context.show_text(text.text.as_str());
            }

            surface
                .write_to_png(&mut img)
                .map_err(|_| CssError::ContentError("Failed to write cairo surface as PNG"))?;

            Ok((style.name.clone(), img))
        })
        .collect::<Result<HashMap<_, _>, CssError>>()
}

fn draw_rectangle(context: &Context, x: f64, y: f64, width: f64, height: f64, border_radius: f64) {
    let border_radius = match border_radius > 20. {
        true => 20. / 3.33,
        false => border_radius / 3.33,
    };
    let degrees = std::f64::consts::PI / 180.0;

    context.new_sub_path();
    context.arc(
        x + width - border_radius,
        y + border_radius,
        border_radius,
        -90.0 * degrees,
        0.0 * degrees,
    );
    context.arc(
        x + width - border_radius,
        y + height - border_radius,
        border_radius,
        0.0 * degrees,
        90.0 * degrees,
    );
    context.arc(
        x + border_radius,
        y + height - border_radius,
        border_radius,
        90.0 * degrees,
        180.0 * degrees,
    );
    context.arc(
        x + border_radius,
        y + border_radius,
        border_radius,
        180.0 * degrees,
        270.0 * degrees,
    );
    context.close_path();
}
