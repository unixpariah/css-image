use crate::error::CssError;
use crate::font::Font;

#[derive(Debug)]
pub(crate) struct Style {
    pub(crate) name: String,
    pub(crate) dimensions: (Option<i32>, Option<i32>),
    pub(crate) text: Option<Font>,
    pub(crate) background: [f64; 4],
    pub(crate) margin: [i32; 4],
    pub(crate) padding: [i32; 4],
}

impl Style {
    pub(crate) fn new(name: &str, styling: Vec<(&str, String)>) -> Result<Self, CssError<'static>> {
        let mut background = [0.0, 0.0, 0.0, 1.0];
        let mut margin = [0, 0, 0, 0];
        let mut padding = [0, 0, 0, 0];
        let mut width = None;
        let mut height = None;
        let mut text = None;
        let mut family = None;
        let mut size = None;
        let mut color = None;
        let mut weight = None;
        let mut slant = None;
        let mut text_align = None;

        styling.iter().try_for_each(|style| {
            match (style.0, style.1.as_str()) {
                ("text-align", value) => text_align = Some(value),
                ("paddint-top", value) => {
                    padding[0] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    }
                }
                ("padding-right", value) => {
                    padding[1] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    }
                }
                ("padding-bottom", value) => {
                    padding[2] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    }
                }
                ("padding-left", value) => {
                    padding[3] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    }
                }
                ("padding", value) => {
                    let value = value.trim();
                    let mut parts = value.split_whitespace();
                    let top = parts.next().unwrap_or("0px");
                    let right = parts.next().unwrap_or("0px");
                    let bottom = parts.next().unwrap_or("0px");
                    let left = parts.next().unwrap_or("0px");

                    padding[0] = match top {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                    padding[1] = match right {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                    padding[2] = match bottom {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                    padding[3] = match left {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                }
                ("width", value) => {
                    width = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()
                        }
                        _ => None,
                    }
                }
                ("height", value) => {
                    height = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()
                        }
                        _ => None,
                    }
                }
                ("margin-top", value) => {
                    margin[0] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    }
                }
                ("margin-right", value) => {
                    margin[1] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    }
                }
                ("margin-bottom", value) => {
                    margin[2] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    }
                }
                ("margin-left", value) => {
                    margin[3] = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }

                        _ => 0,
                    }
                }
                ("margin", value) => {
                    let value = value.trim();
                    let mut parts = value.split_whitespace();
                    let top = parts.next().unwrap_or("0px");
                    let right = parts.next().unwrap_or("0px");
                    let bottom = parts.next().unwrap_or("0px");
                    let left = parts.next().unwrap_or("0px");

                    margin[0] = match top {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                    margin[1] = match right {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                    margin[2] = match bottom {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                    margin[3] = match left {
                        value if value.ends_with("px") => {
                            value.replace("px", "").trim().parse::<i32>().ok()?
                        }
                        _ => 0,
                    };
                }
                ("background-color", value) => {
                    background = match value {
                        "black" => [0.0, 0.0, 0.0, 1.0],
                        "white" => [1.0, 1.0, 1.0, 1.0],
                        "red" => [1.0, 0.0, 0.0, 1.0],
                        "green" => [0.0, 1.0, 0.0, 1.0],
                        "blue" => [0.0, 0.0, 1.0, 1.0],
                        value if value.starts_with('#') => {
                            let hex = value.trim_start_matches('#');
                            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f64 / 255.0;
                            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f64 / 255.0;
                            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f64 / 255.0;
                            [r, g, b, 1.0]
                        }
                        value if value.starts_with("rgb(") => {
                            let rgb = value.trim_start_matches("rgb(").trim_end_matches(')');
                            let mut parts = rgb.split(',');
                            let r = parts.next()?;
                            let g = parts.next()?;
                            let b = parts.next()?;
                            let r = r.trim().parse::<f64>().ok()? / 255.0;
                            let g = g.trim().parse::<f64>().ok()? / 255.0;
                            let b = b.trim().parse::<f64>().ok()? / 255.0;
                            [r, g, b, 1.0]
                        }
                        value if value.starts_with("rgba(") => {
                            let rgba = value.trim_start_matches("rgba(").trim_end_matches(')');
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
                ("content", value) => text = Some(value),
                ("font-family", value) => family = Some(value),
                ("font-size", value) => {
                    size = match value {
                        value if value.ends_with("px") => {
                            value.replace("px", "").parse::<f64>().ok()
                        }
                        _ => None,
                    }
                }
                ("color", value) => color = Some(value),
                ("font-weight", value) => weight = Some(value),
                ("font-style", value) => slant = Some(value),
                _ => {}
            }
            Some(())
        });

        let text = text
            .map(|text| Font::new(family, size, color, weight, slant, text_align, text))
            .transpose()?;

        Ok(Self {
            name: name.to_string(),
            text,
            dimensions: (width, height),
            background,
            margin,
            padding,
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
                ("width", "100px".to_string()),
                ("height", "100px".to_string()),
                ("background-color", "#FFFFFF".to_string()),
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
                ("width", "100px".to_string()),
                ("height", "100px".to_string()),
                ("background-color", "#FFFFFF".to_string()),
                ("content", "Hello".to_string()),
                ("font-family", "Arial".to_string()),
                ("font-size", "16px".to_string()),
                ("color", "black".to_string()),
                ("font-weight", "bold".to_string()),
                ("font-style", "italic".to_string()),
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
                text_align: "left".to_string(),
            })
        );
    }
}
