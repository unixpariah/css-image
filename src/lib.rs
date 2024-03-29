mod error;

use cairo::{Context, ImageSurface};
use error::CssError;
use std::collections::HashMap;

struct Style {
    name: String,
    dimensions: (usize, usize),
    styling: Vec<String>,
    text: Option<String>,
}

impl Style {
    fn new(name: String, styling: Vec<String>) -> Result<Self, CssError> {
        let width = styling
            .iter()
            .find(|s| s.contains("width:"))
            .ok_or(CssError::SizeError("Width not found"))?
            .split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse::<usize>()?;

        let height = styling
            .iter()
            .find(|s| s.contains("height:"))
            .ok_or(CssError::SizeError("Height not found"))?
            .split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse::<usize>()?;

        let text = styling
            .iter()
            .find(|s| s.contains("content:"))
            .map(|s| s.split(':').collect::<Vec<&str>>()[1].trim().to_string());

        Ok(Self {
            name,
            text,
            dimensions: (width, height),
            styling,
        })
    }
}

pub fn parse(mut css: String) -> Result<HashMap<String, Vec<u8>>, CssError> {
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

    styles.iter().for_each(|style| {
        style.styling.iter().for_each(|_styling| {});

        let surface = ImageSurface::create(
            cairo::Format::ARgb32,
            style.dimensions.0 as i32,
            style.dimensions.1 as i32,
        )
        .expect("Failed to create ImageSurface");
        let _context = Context::new(&surface);

        let mut img = Vec::new();
        surface.write_to_png(&mut img).unwrap();

        output.insert(style.name.clone(), img);
    });

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let css = "body { color: red; font-size: 2rem; width: 4; height: 2; } aaa { coalor: red; width: 1; height: 2; }".to_string();
        assert!(parse(css).is_ok());

        let css = "body { color: red; font-size: 2rem; }".to_string();
        assert!(parse(css).is_err());

        let css = "body { color: red; font-size: 2rem; width: four; height: two; }".to_string();
        assert!(parse(css).is_err());
    }
}
