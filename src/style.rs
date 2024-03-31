use crate::error::CssError;
use crate::font::Font;

#[derive(Debug)]
pub(crate) struct Style {
    pub(crate) name: String,
    pub(crate) dimensions: (Option<i32>, Option<i32>),
    pub(crate) text: Option<Font>,
    pub(crate) background: [f64; 4],
}

impl Style {
    pub(crate) fn new(name: &str, styling: Vec<String>) -> Result<Self, CssError<'static>> {
        let mut width = None;
        let mut height = None;
        let mut background = [0.0, 0.0, 0.0, 1.0];
        let mut text = None;
        let mut family = None;
        let mut size = None;
        let mut color = None;
        let mut weight = None;
        let mut slant = None;

        styling.iter().try_for_each(|style| {
            match style {
                s if s.contains("width:") => {
                    width = match s.split(':').collect::<Vec<&str>>()[1] {
                        s if s.ends_with("px") => s.replace("px", "").trim().parse::<i32>().ok(),
                        _ => None,
                    }
                }
                s if s.contains("height:") => {
                    height = match s.split(':').collect::<Vec<&str>>()[1] {
                        s if s.ends_with("px") => s.replace("px", "").trim().parse::<i32>().ok(),
                        _ => None,
                    }
                }
                s if s.contains("background-color:") => {
                    background = match s.split(':').collect::<Vec<&str>>()[1].trim() {
                        "black" => [0.0, 0.0, 0.0, 1.0],
                        "white" => [1.0, 1.0, 1.0, 1.0],
                        "red" => [1.0, 0.0, 0.0, 1.0],
                        "green" => [0.0, 1.0, 0.0, 1.0],
                        "blue" => [0.0, 0.0, 1.0, 1.0],
                        s if s.starts_with('#') => {
                            let hex = s.trim_start_matches('#');
                            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f64 / 255.0;
                            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f64 / 255.0;
                            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f64 / 255.0;
                            [r, g, b, 1.0]
                        }
                        s if s.starts_with("rgb(") => {
                            let rgb = s.trim_start_matches("rgb(").trim_end_matches(')');
                            let mut parts = rgb.split(',');
                            let r = parts.next()?;
                            let g = parts.next()?;
                            let b = parts.next()?;
                            let r = r.trim().parse::<f64>().ok()? / 255.0;
                            let g = g.trim().parse::<f64>().ok()? / 255.0;
                            let b = b.trim().parse::<f64>().ok()? / 255.0;
                            [r, g, b, 1.0]
                        }
                        s if s.starts_with("rgba(") => {
                            let rgba = s.trim_start_matches("rgba(").trim_end_matches(')');
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
                        _ => background,
                    }
                }
                s if s.contains("content:") => {
                    let parts: Vec<&str> = s.split('"').collect();
                    text = Some(parts.get(1)?.trim().replace(['"', ';'], ""));
                }
                s if s.contains("font-family:") => {
                    family = Some(s.split(':').collect::<Vec<&str>>()[1].trim().to_string());
                }
                s if s.contains("font-size:") => {
                    size = s.split(':').collect::<Vec<&str>>()[1]
                        .trim()
                        .parse::<f64>()
                        .ok();
                }
                s if s.contains("color:") => {
                    let a = Some(s.split(':').collect::<Vec<&str>>()[1].trim().to_string());
                    color = a;
                }
                s if s.contains("font-weight:") => {
                    weight = Some(s.split(':').collect::<Vec<&str>>()[1].trim().to_string());
                }
                s if s.contains("font-style:") => {
                    slant = Some(s.split(':').collect::<Vec<&str>>()[1].trim().to_string());
                }
                _ => {}
            }
            Some(())
        });

        let text = text
            .map(|text| Font::new(family, size, color, weight, slant, text))
            .transpose()?;

        Ok(Self {
            name: name.to_string(),
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
            "body",
            vec![
                "width: 100px".to_string(),
                "height: 100px".to_string(),
                "background-color: #FFFFFF".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(style.name, "body");
        assert_eq!(style.dimensions, (Some(100), Some(100)));
        assert_eq!(style.background, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(style.text, None);

        let style = Style::new(
            "body",
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
        assert_eq!(style.dimensions, (Some(100), Some(100)));
        assert_eq!(style.background, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(
            style.text,
            Some(Font {
                family: "Arial".to_string(),
                size: 16.0,
                color: [0.0, 0.0, 0.0],
                weight: cairo::FontWeight::Bold,
                text: "Hello".to_string(),
                slant: cairo::FontSlant::Italic,
            })
        );
    }
}
