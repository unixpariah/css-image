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
pub fn parse(mut css: String) -> Result<HashMap<String, Vec<u8>>, CssError<'static>> {
    if css.is_empty() {
        return Err(CssError::ContentError("Empty CSS"));
    }

    let mut styles = vec![];
    while !css.is_empty() {
        let opening_brace_pos = css.find('{');
        let closing_brace_pos = css.find('}');

        let style = match (opening_brace_pos, closing_brace_pos) {
            (Some(open), Some(close)) => {
                let styling = css
                    .drain(open..=close)
                    .collect::<String>()
                    .trim()
                    .replace(['{', '}'], "")
                    .trim()
                    .to_string();
                let name = css.drain(..open).collect::<String>().trim().to_string();

                let mut styling = styling
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();

                styling.remove(styling.len() - 1);

                Style::new(name, styling)?
            }
            _ => return Err("Invalid CSS".into()),
        };

        styles.push(style);
    }

    Ok(styles
        .par_iter()
        .filter_map(|style| {
            let mut width = style.dimensions.0 as i32;
            let mut height = style.dimensions.1 as i32;

            if (width <= 0 || height <= 0) && style.text.is_some() {
                let surface = ImageSurface::create(cairo::Format::ARgb32, 0, 0).ok()?;
                let context = Context::new(&surface).ok()?;
                context.select_font_face(
                    style.text.as_ref()?.family.as_str(),
                    style.text.as_ref()?.slant,
                    style.text.as_ref()?.weight,
                );
                context.set_font_size(style.text.as_ref()?.size);
                let extents = context
                    .text_extents(style.text.as_ref()?.text.as_str())
                    .ok()?;

                width = extents.width() as i32;
                height = extents.height() as i32;
            }

            let surface = ImageSurface::create(cairo::Format::ARgb32, width, height).ok()?;
            let context = Context::new(&surface).ok()?;

            context.set_source_rgba(
                style.background[0],
                style.background[1],
                style.background[2],
                style.background[3],
            );
            context.paint().ok()?;

            if let Some(text) = &style.text {
                context.select_font_face(text.family.as_str(), text.slant, text.weight);
                context.set_font_size(text.size);
                context.set_source_rgb(text.color[0], text.color[1], text.color[2]);
                context.move_to(0.0, style.dimensions.1);
                _ = context.show_text(text.text.as_str());
            }

            let mut img = Vec::new();
            surface.write_to_png(&mut img).ok()?;

            Some((style.name.clone(), img))
        })
        .collect::<HashMap<String, Vec<u8>>>())
}
