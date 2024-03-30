use crate::error::CssError;
use crate::font::Font;

#[derive(Debug)]
pub(crate) struct Style {
    pub(crate) name: String,
    pub(crate) dimensions: (f64, f64),
    pub(crate) text: Option<Font>,
    pub(crate) background: [f64; 4],
}

impl Style {
    pub(crate) fn new(name: String, styling: Vec<String>) -> Result<Self, CssError<'static>> {
        let width = match styling
            .iter()
            .find(|s| s.contains("width:"))
            .ok_or(CssError::SizeError("SizeError: Width not found"))?
            .split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
        {
            size if size.ends_with("px") => size.trim_end_matches("px"),
            "auto" => "0",
            _ => return Err(CssError::SizeError("SizeError: Invalid width")),
        }
        .parse::<f64>()
        .unwrap_or(0.0);

        let height = match styling
            .iter()
            .find(|s| s.contains("height:"))
            .ok_or(CssError::SizeError("SizeError: Height not found"))?
            .split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
        {
            size if size.ends_with("px") => size.trim_end_matches("px"),
            "auto" => "0",
            _ => return Err(CssError::SizeError("SizeError: Invalid width")),
        }
        .parse::<f64>()
        .unwrap_or(0.0);

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
            background,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style() {
        let style = Style::new(
            "body".to_string(),
            vec![
                "width: 100px".to_string(),
                "height: 100px".to_string(),
                "background-color: #FFFFFF".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(style.name, "body");
        assert_eq!(style.dimensions, (100.0, 100.0));
        assert_eq!(style.background, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(style.text, None);

        let style = Style::new(
            "body".to_string(),
            vec![
                "width: 100px".to_string(),
                "height: 100px".to_string(),
                "background-color: #FFFFFF".to_string(),
                "content: \"Hello\"".to_string(),
                "font-family: Arial".to_string(),
                "font-size: 16".to_string(),
                "color: black".to_string(),
                "font-weight: bold".to_string(),
                "font-style: italic".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(style.name, "body");
        assert_eq!(style.dimensions, (100.0, 100.0));
        assert_eq!(style.background, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(
            style.text,
            Some(Font {
                family: "Arial".to_string(),
                size: 16.0,
                color: [1.0, 1.0, 1.0],
                weight: cairo::FontWeight::Bold,
                text: "Hello".to_string(),
                slant: cairo::FontSlant::Italic,
            })
        );
    }
}
