use super::get_color;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Font {
    pub color: [f64; 4],
    pub size: f64,
    pub family: String,
    pub style: cairo::FontSlant,
    pub weight: cairo::FontWeight,
    pub text_align: String,
}

impl Font {
    pub fn new(css: &HashMap<String, String>) -> Self {
        let size = match css
            .get("font-size")
            .unwrap_or(&"".to_string())
            .ends_with("px")
        {
            true => css
                .get("font-size")
                .unwrap_or(&"".to_string())
                .replace("px", "")
                .parse(),
            false => Ok(12.0),
        };

        let color = match css.get("color") {
            Some(color) => get_color(color),
            None => [0., 0., 0., 1.],
        };

        let family = match css.get("font-family") {
            Some(family) => family.to_string(),
            None => "Arial".to_string(),
        };

        let style = match css.get("font-style") {
            Some(style) => match style.as_str() {
                "italic" => cairo::FontSlant::Italic,
                "oblique" => cairo::FontSlant::Oblique,
                _ => cairo::FontSlant::Normal,
            },
            None => cairo::FontSlant::Normal,
        };

        let weight = match css.get("font-weight") {
            Some(weight) => match weight.as_str() {
                "bold" => cairo::FontWeight::Bold,
                _ => cairo::FontWeight::Normal,
            },
            None => cairo::FontWeight::Normal,
        };

        let text_align = match css.get("text-align") {
            Some(align) => align.to_string(),
            None => "left".to_string(),
        };

        Self {
            text_align,
            color,
            size: size.unwrap_or(12.0),
            family,
            style,
            weight,
        }
    }
}
