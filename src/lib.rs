mod error;

use cairo::{Context, ImageSurface};
use error::CssError;
use std::{collections::HashMap, error::Error};

struct Font {
    family: String,
    size: f64,
    color: [f64; 3],
    weight: cairo::FontWeight,
    text: String,
}

impl Font {
    fn new(
        family: Option<String>,
        size: Option<f64>,
        color: Option<String>,
        weight: Option<String>,
        text: String,
    ) -> Result<Self, CssError<'static>> {
        let family = family.unwrap_or("Arial".to_string());
        let size = size.unwrap_or(16.0);
        let color = match color.as_deref() {
            Some("black") => [0.0, 0.0, 0.0],
            Some("white") => [1.0, 1.0, 1.0],
            Some("red") => [1.0, 0.0, 0.0],
            Some("green") => [0.0, 1.0, 0.0],
            Some("blue") => [0.0, 0.0, 1.0],
            Some(hex) if hex.starts_with('#') => {
                let hex = hex.trim_start_matches('#');
                let r = u8::from_str_radix(&hex[0..2], 16)? as f64 / 255.0;
                let g = u8::from_str_radix(&hex[2..4], 16)? as f64 / 255.0;
                let b = u8::from_str_radix(&hex[4..6], 16)? as f64 / 255.0;
                [r, g, b]
            }
            Some(rgb) if rgb.starts_with("rgb(") => {
                let rgb = rgb.trim_start_matches("rgb(").trim_end_matches(')');
                let mut parts = rgb.split(',');
                let r = parts
                    .next()
                    .ok_or(CssError::FontError("Invalid font color"))?;
                let g = parts
                    .next()
                    .ok_or(CssError::FontError("Invalid font color"))?;
                let b = parts
                    .next()
                    .ok_or(CssError::FontError("Invalid font color"))?;
                let r = r.trim().parse::<f64>()? / 255.0;
                let g = g.trim().parse::<f64>()? / 255.0;
                let b = b.trim().parse::<f64>()? / 255.0;
                [r, g, b]
            }
            Some(&_) => return Err(CssError::FontError("Invalid font color")),
            None => [0.0, 0.0, 0.0],
        };
        let weight = match weight.as_deref() {
            Some("bold") => cairo::FontWeight::Bold,
            Some("normal") => cairo::FontWeight::Normal,
            Some(&_) => return Err(CssError::FontError("Invalid font weight")),
            None => cairo::FontWeight::Normal,
        };

        Ok(Self {
            family,
            size,
            color,
            weight,
            text,
        })
    }
}

struct Style {
    name: String,
    dimensions: (f64, f64),
    styling: Vec<String>,
    text: Option<Font>,
}

impl Style {
    fn new(name: String, styling: Vec<String>) -> Result<Self, CssError<'static>> {
        let width = styling
            .iter()
            .find(|s| s.contains("width:"))
            .ok_or(CssError::SizeError("SizeError: Width not found"))?
            .split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse::<f64>()?;

        let height = styling
            .iter()
            .find(|s| s.contains("height:"))
            .ok_or(CssError::SizeError("SizeError: Height not found"))?
            .split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse::<f64>()?;

        let text: Option<String> = styling.iter().find_map(|s| {
            if s.contains("content:") {
                let parts: Vec<&str> = s.split(':').collect();
                Some(parts.get(1)?.trim().replace(['"', ';'], ""))
            } else {
                None
            }
        });

        let family = styling
            .iter()
            .find(|s| s.contains("font-family:"))
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].trim().to_string());

        let size = styling
            .iter()
            .find(|s| s.contains("font-size:"))
            .and_then(|s| {
                s.split(':').collect::<Vec<&str>>()[1]
                    .trim()
                    .parse::<f64>()
                    .ok()
            });

        let color = styling
            .iter()
            .find(|s| s.contains("color:"))
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].trim().to_string());

        let weight = styling
            .iter()
            .find(|s| s.contains("font-weight:"))
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].trim().to_string());

        let text = text
            .map(|text| Font::new(family, size, color, weight, text))
            .transpose()?;

        Ok(Self {
            name,
            text,
            dimensions: (width, height),
            styling,
        })
    }
}

pub fn parse(mut css: String) -> Result<HashMap<String, Vec<u8>>, CssError<'static>> {
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

    let mut output = HashMap::new();

    styles
        .iter()
        .try_for_each(|style| {
            let surface = ImageSurface::create(
                cairo::Format::ARgb32,
                style.dimensions.0 as i32,
                style.dimensions.1 as i32,
            )?;
            let context = Context::new(&surface)?;
            if let Some(text) = &style.text {
                context.select_font_face(
                    text.family.as_str(),
                    cairo::FontSlant::Normal,
                    text.weight,
                );
                context.set_font_size(text.size);
                context.set_source_rgb(text.color[0], text.color[1], text.color[2]);
                context.move_to(0.0, style.dimensions.1);
                _ = context.show_text(text.text.as_str());
            }

            style.styling.iter().for_each(|_| {});

            let mut img = Vec::new();
            surface.write_to_png(&mut img)?;

            let mut file = std::fs::File::create("test.png")?;
            surface.write_to_png(&mut file)?;

            output.insert(style.name.clone(), img);

            Ok::<(), Box<dyn Error>>(())
        })
        .map_err(|_| CssError::ParseError)?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let css =
            r#"body { color: rgb(255, 0, 0); font-size: 20; width: 100; height: 100; content: "aaaa"; font-weight: bold; }"#
                .to_string();
        let result = parse(css);
        assert!(result.is_ok());

        let css = "body { color: red; font-size: 2rem; }".to_string();
        let result = parse(css);
        println!("{:?}", result);
        assert!(result.is_err());

        let css = "body { color: red; font-size: 2rem; width: four; height: two; }".to_string();
        let result = parse(css);
        println!("{:?}", result);
        assert!(result.is_err());
    }
}
