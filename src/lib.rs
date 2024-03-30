mod error;
mod font;

use crate::font::Font;
use cairo::{Context, ImageSurface};
use error::CssError;
use std::{collections::HashMap, error::Error};

struct Style {
    name: String,
    dimensions: (f64, f64),
    styling: Vec<String>,
    text: Option<Font>,
    background: [f64; 4],
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

        let background = styling
            .iter()
            .find_map(|s| {
                if s.contains("background-color:") {
                    let parts: Vec<&str> = s.split(':').collect();
                    let color = parts.get(1)?.trim().to_string();
                    let color = match color.as_str() {
                        "black" => [0.0, 0.0, 0.0, 1.0],
                        "white" => [1.0, 1.0, 1.0, 1.0],
                        "red" => [1.0, 0.0, 0.0, 1.0],
                        "green" => [0.0, 1.0, 0.0, 1.0],
                        "blue" => [0.0, 0.0, 1.0, 1.0],
                        hex if hex.starts_with('#') => {
                            let hex = hex.trim_start_matches('#');
                            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f64 / 255.0;
                            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f64 / 255.0;
                            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f64 / 255.0;
                            [r, g, b, 1.0]
                        }
                        rgb if rgb.starts_with("rgb(") => {
                            let rgb = rgb.trim_start_matches("rgb(").trim_end_matches(')');
                            let mut parts = rgb.split(',');
                            let r = parts.next()?;
                            let g = parts.next()?;
                            let b = parts.next()?;
                            let r = r.trim().parse::<f64>().ok()? / 255.0;
                            let g = g.trim().parse::<f64>().ok()? / 255.0;
                            let b = b.trim().parse::<f64>().ok()? / 255.0;
                            [r, g, b, 1.0]
                        }
                        rgba if rgba.starts_with("rgba(") => {
                            let rgba = rgba.trim_start_matches("rgba(").trim_end_matches(')');
                            let mut parts = rgba.split(',');
                            let r = parts.next()?;
                            let g = parts.next()?;
                            let b = parts.next()?;
                            let a = parts.next()?;
                            let r = r.trim().parse::<f64>().ok()? / 255.0;
                            let g = g.trim().parse::<f64>().ok()? / 255.0;
                            let b = b.trim().parse::<f64>().ok()? / 255.0;
                            let a = a.trim().parse::<f64>().ok()? / 255.0;
                            [r, g, b, a]
                        }
                        _ => return None,
                    };
                    Some(color)
                } else {
                    None
                }
            })
            .unwrap_or([0.0, 0.0, 0.0, 1.0]);

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

        let slant = styling
            .iter()
            .find(|s| s.contains("font-style:"))
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].trim().to_string());

        let text = text
            .map(|text| Font::new(family, size, color, weight, slant, text))
            .transpose()?;

        Ok(Self {
            name,
            text,
            dimensions: (width, height),
            styling,
            background,
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

            context.set_source_rgba(
                style.background[0],
                style.background[1],
                style.background[2],
                style.background[3],
            );
            context.paint()?;

            if let Some(text) = &style.text {
                context.select_font_face(text.family.as_str(), text.slant, text.weight);
                context.set_font_size(text.size);
                context.set_source_rgb(text.color[0], text.color[1], text.color[2]);
                context.move_to(0.0, style.dimensions.1);
                _ = context.show_text(text.text.as_str());
            }

            style.styling.iter().for_each(|_| {});

            let mut img = Vec::new();
            surface.write_to_png(&mut img)?;

            /*
                        let mut file = std::fs::File::create("test.png")?;
                        surface.write_to_png(&mut file)?;
            */

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
            r#"body { color: #FF0000; font-size: 20; width: 100; height: 100; content: "aaaa"; font-weight: bold; }"#
                .to_string();
        let result = parse(css);
        assert!(result.is_ok());

        let css =
            r#"body { color: red; font-size: 20; width: 100; height: 100; content: "aaaa"; font-weight: bold; }"#
                .to_string();
        let result = parse(css);
        assert!(result.is_ok());

        let css =
            r#"body { color: rgb(0, 0, 0); font-size: 20; width: 100; height: 100; content: "aaaa"; font-weight: bold; background-color: rgb(0, 255, 0, 150); }"#
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
