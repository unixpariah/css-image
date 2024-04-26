use super::get_color;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Font {
    pub color: [f64; 4],
    pub size: f64,
    pub family: Box<str>,
    pub style: cairo::FontSlant,
    pub weight: cairo::FontWeight,
    pub text_align: Box<str>,
    pub letter_spacing: f64,
}
impl Font {
    pub fn new(
        css: &HashMap<Box<str>, String>,
        all_selector: Option<&HashMap<Box<str>, String>>,
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
            .map(|s| s.trim().replace('\"', ""))
            .unwrap_or_else(|| "Arial".to_string())
            .into();

        let letter_spacing = css
            .get("letter-spacing")
            .or_else(|| all_selector.as_ref()?.get("font-family"))
            .map(|s| s.trim().replace("px", "").parse::<f64>().unwrap_or(0.0))
            .unwrap_or_else(|| 0.0);

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
            .map(|s| s.as_str())
            .unwrap_or_else(|| "left")
            .into();

        Self {
            letter_spacing,
            text_align,
            color,
            size,
            family,
            style,
            weight,
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_style() {
        let css = [
            ("color", "#ffffff"),
            ("font-size", "12px"),
            ("font-family", "Arial"),
            ("font-style", "normal"),
            ("font-weight", "normal"),
            ("text-align", "left"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        let result = Font::new(&css, None);
        assert_eq!(result.color, [1., 1., 1., 1.]);
        assert_eq!(result.size, 12.0);
        assert_eq!(result.family, "Arial");
        assert_eq!(result.style, cairo::FontSlant::Normal);
        assert_eq!(result.weight, cairo::FontWeight::Normal);
        assert_eq!(result.text_align, "left");
    }

    #[test]
    fn all_selector() {
        let css = HashMap::new();
        let all_selector = Some(
            [
                ("color", "#ffffff"),
                ("font-size", "12px"),
                ("font-family", "Arial"),
                ("font-style", "normal"),
                ("font-weight", "normal"),
                ("text-align", "left"),
            ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        );

        let result = Font::new(&css, all_selector.as_ref());
        assert_eq!(result.color, [1., 1., 1., 1.]);
        assert_eq!(result.size, 12.0);
        assert_eq!(result.family, "Arial");
        assert_eq!(result.style, cairo::FontSlant::Normal);
        assert_eq!(result.weight, cairo::FontWeight::Normal);
        assert_eq!(result.text_align, "left");
    }
}
*/
