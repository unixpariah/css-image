mod error;
pub mod style;

use cairo::{Context, ImageSurface};
use error::CssError;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use style::{Parseable, Style};

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<selector>\S+)\s*\{\s*(?P<properties>[^}]+)\s*\}").unwrap();
    static ref PROPERTY_RE: Regex =
        Regex::new(r"(?P<property>[\w-]+):\s*(?P<value>[^;]+);").unwrap();
}

/// Parse CSS into a HashMap of selector name -> Style for easier manipulation
///
/// # Examples
///
/// ```
/// use css_image::parse;
///
/// let css = r#"
/// body {
///    background-color: #FFFFFF;
///    width: 100px;
///    height: 100px;
/// }
/// "#;
///
/// let mut result = parse(css).unwrap(); // HashMap of selector name -> Style
///
/// let body = result.get_mut("body").unwrap(); // Get the body element
/// body.content = Some("Hello, World!".to_string()); // Change the content of the body element
/// ```
pub fn parse(css: &str) -> Result<HashMap<String, Style>, CssError<'static>> {
    let split = css
        .split_inclusive('}')
        .filter_map(|selector| {
            let selector = selector.trim();
            if selector.is_empty() {
                return None;
            }
            Some(selector)
        })
        .collect::<Vec<&str>>();

    let all_selector = split.iter().find_map(|s| {
        let mut properties = HashMap::new();

        for cap in RE.captures_iter(s) {
            if &cap["selector"] == "*" {
                for property_cap in PROPERTY_RE.captures_iter(&cap["properties"]) {
                    properties.insert(
                        property_cap["property"].to_string(),
                        property_cap["value"].to_string(),
                    );
                }
                return Some(properties);
            }
        }
        None
    });

    Ok(split
        .par_iter()
        .filter_map(|s| {
            let mut properties = HashMap::with_capacity(split.len() - 1);

            for cap in RE.captures_iter(s) {
                let selector = cap["selector"].to_string();

                PROPERTY_RE
                    .captures_iter(&cap["properties"])
                    .for_each(|property_cap| {
                        properties.insert(
                            property_cap["property"].to_string(),
                            property_cap["value"].to_string(),
                        );
                    });

                return Some((selector, properties));
            }

            None
        })
        .map(|(selector, properties)| {
            let style = Style::new(&properties, all_selector.as_ref());
            (selector, style)
        })
        .collect::<HashMap<String, Style>>())
}

/// Render images with CSS
///
/// # Examples
///
/// ```
/// use css_image::{render, parse};
///
/// let css = r#"
/// body {
///    background-color: #FFFFFF;
///    width: 100px;
///    height: 100px;
/// }
/// "#;
///
/// let result = render(css); // HashMap of selector name -> image data
/// assert!(result.is_ok());
///
/// let mut styles = parse(css).unwrap(); // Parse the CSS into a Styles struct for easier manipulation
/// styles.get_mut("body").unwrap().content = Some("Hello, World!".to_string()); // Change the content of the body element
///
/// let result = render(styles); // HashMap of selector name -> image data
/// assert!(result.is_ok());
/// ```
pub fn render<T>(css: T) -> Result<HashMap<String, Vec<u8>>, CssError<'static>>
where
    T: Parseable,
{
    let mut styles = css.parse()?;

    styles
        .par_iter_mut()
        .map(|style| {
            let name = style.0;
            let style = style.1;

            let mut width = style.width;
            let mut height = style.height;
            let mut position = 0;

            let mut text_width = 0;

            if let Some(content) = &style.content {
                if content.is_empty() {
                    style.content = None;
                }
            }
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
