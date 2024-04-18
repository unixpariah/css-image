mod error;
mod style;

use cairo::{Context, ImageSurface};
use error::CssError;
use rayon::prelude::*;
use std::collections::HashMap;
pub use style::Stylings;

pub fn parse(css: &str) -> Result<HashMap<String, Vec<u8>>, CssError<'static>> {
    let styles = Stylings::new(css).unwrap().styles;

    styles
        .par_iter()
        .map(|style| {
            let name = style.0;
            let style = style.1;

            let mut width = style.width;
            let mut height = style.height;
            let mut position = 0;

            let mut text_width = 0;

            if let Some(content) = &style.content {
                let surface = ImageSurface::create(cairo::Format::ARgb32, 0, 0)
                    .map_err(|_| CssError::ContentError("Failed to create cairo surface"))?;
                let context = Context::new(&surface)
                    .map_err(|_| CssError::ContentError("Failed to create cairo context"))?;
                let font = &style.font;

                context.select_font_face(font.family.as_str(), font.style, font.weight);
                context.set_font_size(font.size);
                let extents = context
                    .text_extents(content.as_str())
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

            let margin = style.margin;
            let padding = style.padding;

            let width = width.unwrap_or(5);
            let height = height.unwrap_or(5);

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
                style.background_color[0],
                style.background_color[1],
                style.background_color[2],
                style.background_color[3],
            );
            draw_rectangle(
                &context,
                margin[3] as f64,
                margin[0] as f64,
                width as f64 + padding[1] as f64 + padding[3] as f64,
                height as f64 + padding[0] as f64 + padding[2] as f64,
                style.border_radius,
            );
            context
                .fill_preserve()
                .map_err(|_| CssError::ContentError("Failed to paint the surface"))?;

            if let Some(text) = &style.content {
                let font = &style.font;
                context.select_font_face(font.family.as_str(), font.style, font.weight);
                context.set_font_size(font.size);
                context.set_source_rgba(font.color[0], font.color[1], font.color[2], 1.0);
                match font.text_align.as_str() {
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
                        context.move_to(
                            0.0 + padding[3] as f64 + margin[3] as f64,
                            position as f64 + padding[0] as f64 + margin[0] as f64,
                        );
                    }
                    _ => return Err(CssError::ContentError("Invalid text-align")),
                }
                _ = context.show_text(text.as_str());
            }

            surface
                .write_to_png(&mut img)
                .map_err(|_| CssError::ContentError("Failed to write cairo surface as PNG"))?;

            Ok((name.clone(), img))
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
