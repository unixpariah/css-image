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
                return None;
            }

            let styling = styling
                .par_split(';')
                .filter_map(|s| {
                    if s.is_empty() {
                        return None;
                    }
                    Some(s.trim().to_string())
                })
                .collect::<Vec<String>>();

            Style::new(name, styling).ok()
        })
        .collect::<Vec<_>>();

    if styles.is_empty() {
        return Err(CssError::ContentError("Invalid CSS"));
    }

    let result: Result<HashMap<_, _>, CssError> = styles
        .par_iter()
        .map(|style| {
            let mut width = style.dimensions.0;
            let mut height = style.dimensions.1;

            if (width.is_none() || height.is_none()) && style.text.is_some() {
                let surface = ImageSurface::create(cairo::Format::ARgb32, 0, 0)
                    .map_err(|_| CssError::ContentError(""))?;
                let context = Context::new(&surface).map_err(|_| CssError::ContentError(""))?;
                context.select_font_face(
                    style
                        .text
                        .as_ref()
                        .ok_or(CssError::ContentError(""))?
                        .family
                        .as_str(),
                    style.text.as_ref().ok_or(CssError::ContentError(""))?.slant,
                    style
                        .text
                        .as_ref()
                        .ok_or(CssError::ContentError(""))?
                        .weight,
                );
                context.set_font_size(style.text.as_ref().ok_or(CssError::ContentError(""))?.size);
                let extents = context
                    .text_extents(
                        style
                            .text
                            .as_ref()
                            .ok_or(CssError::ContentError(""))?
                            .text
                            .as_str(),
                    )
                    .map_err(|_| CssError::ContentError(""))?;

                width = Some(extents.width() as i32);
                height = Some(extents.height() as i32);
            }

            let width = width.ok_or(CssError::SizeError(""))?;
            let height = height.ok_or(CssError::SizeError(""))?;

            let surface = ImageSurface::create(cairo::Format::ARgb32, width, height)
                .map_err(|_| CssError::ContentError(""))?;
            let context = Context::new(&surface).map_err(|_| CssError::ContentError(""))?;

            context.set_source_rgba(
                style.background[0],
                style.background[1],
                style.background[2],
                style.background[3],
            );
            context.paint().map_err(|_| CssError::ContentError(""))?;

            if let Some(text) = &style.text {
                context.select_font_face(text.family.as_str(), text.slant, text.weight);
                context.set_font_size(text.size);
                context.set_source_rgb(text.color[0], text.color[1], text.color[2]);
                context.move_to(0.0, height as f64);
                _ = context.show_text(text.text.as_str());
            }

            let mut img = Vec::new();
            surface
                .write_to_png(&mut img)
                .map_err(|_| CssError::ContentError(""))?;

            Ok((style.name.clone(), img))
        })
        .collect();

    let result = result.map_err(|_| CssError::ContentError(""))?;

    Ok(result)
}
