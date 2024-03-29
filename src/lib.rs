mod error;

use cairo::{Context, ImageSurface};
use error::CssError;
use std::{collections::HashMap, error::Error};

struct Font {
    family: String,
    size: f64,
    color: String,
    weight: String,
    text: String,
}

impl Font {
    fn new(
        family: Option<String>,
        size: Option<f64>,
        color: Option<String>,
        weight: Option<String>,
        text: String,
    ) -> Self {
        let family = family.unwrap_or("sans-serif".to_string());
        let size = size.unwrap_or(16.0);
        let color = color.unwrap_or("black".to_string());
        let weight = weight.unwrap_or("normal".to_string());

        Self {
            family,
            size,
            color,
            weight,
            text,
        }
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

        let text = styling
            .iter()
            .find(|s| s.contains("content:"))
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].trim().to_string());

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

        let text = text.map(|text| Font::new(family, size, color, weight, text));

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
                _ = context.show_text(text.text.as_str());
            }

            style.styling.iter().for_each(|_| {});

            let mut img = Vec::new();
            surface.write_to_png(&mut img)?;

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
        let css = "body { color: red; font-size: 2; width: 4.0; height: 2.0; } aaa { color: red; width: 1.0; height: 2.0; }".to_string();
        let result = parse(css);
        //println!("{:?}", result);
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
