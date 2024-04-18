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
    pub fn new(
        css: &HashMap<String, String>,
        all_selector: Option<&HashMap<String, String>>,
    ) -> Self {
        let get_property = |property: &str, default: f64| {
            css.get(property)
                .and_then(|s| {
                    if s.ends_with("px") {
                        Some(s.replace("px", ""))
                    } else {
                        None
                    }
                })
                .and_then(|s| s.parse::<f64>().ok())
                .or_else(|| {
                    all_selector
                        .as_ref()?
                        .get(property)
                        .and_then(|s| {
                            if s.ends_with("px") {
                                Some(s.replace("px", ""))
                            } else {
                                None
                            }
                        })
                        .and_then(|s| s.parse::<f64>().ok())
                })
                .unwrap_or(default)
        };

        let size = get_property("font-size", 12.0);

        let color = css
            .get("color")
            .or_else(|| all_selector.as_ref()?.get("color"))
            .map(|color| get_color(color))
            .unwrap_or([0., 0., 0., 1.]);

        let family = css
            .get("font-family")
            .or_else(|| all_selector.as_ref()?.get("font-family"))
            .map(|s| s.trim().replace("\"", ""))
            .unwrap_or_else(|| "Arial".to_string());

        let style = css
            .get("font-style")
            .or_else(|| all_selector.as_ref()?.get("font-style"))
            .map(|s| match s.as_str() {
                "italic" => cairo::FontSlant::Italic,
                "oblique" => cairo::FontSlant::Oblique,
                _ => cairo::FontSlant::Normal,
            })
            .unwrap_or(cairo::FontSlant::Normal);

        let weight = css
            .get("font-weight")
            .or_else(|| all_selector.as_ref()?.get("font-weight"))
            .map(|s| match s.as_str() {
                "bold" => cairo::FontWeight::Bold,
                _ => cairo::FontWeight::Normal,
            })
            .unwrap_or(cairo::FontWeight::Normal);

        let text_align = css
            .get("text-align")
            .or_else(|| all_selector.as_ref()?.get("text-align"))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "left".to_string());

        Self {
            text_align,
            color,
            size,
            family,
            style,
            weight,
        }
    }
}
