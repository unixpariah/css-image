mod font;

use crate::error::CssError;
use font::Font;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Stylings {
    pub styles: HashMap<String, Style>,
}

impl Stylings {
    pub fn new(css: &str) -> Result<Self, CssError<'static>> {
        let css = Styles::from_str(css)?.0;
        let styles = css
            .iter()
            .map(|(selector, properties)| {
                let style = Style::new(properties);
                (selector.to_string(), style)
            })
            .collect();

        Ok(Self { styles })
    }
}

#[derive(Debug)]
pub struct Style {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub background_color: [f64; 4],
    pub font: Font,
    pub content: Option<String>,
    pub border_radius: f64,
    pub margin: [i32; 4],
    pub padding: [i32; 4],
}

pub(super) fn get_color(color: &str) -> [f64; 4] {
    if color.starts_with("#") {
        let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0) as f64 / 255.;
        let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0) as f64 / 255.;
        let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0) as f64 / 255.;
        [r, g, b, 1.]
    } else if color.starts_with("rgb") {
        let rgb: Vec<f64> = color[4..color.len() - 1]
            .split(',')
            .map(|s| s.trim().parse().unwrap_or(0) as f64 / 255.)
            .collect();
        if rgb.len() == 3 {
            [
                rgb[0] as f64 / 255.,
                rgb[1] as f64 / 255.,
                rgb[2] as f64 / 255.,
                1.,
            ]
        } else {
            [0., 0., 0., 1.]
        }
    } else {
        match color {
            "red" => [1., 0., 0., 1.],
            "green" => [0., 1., 0., 1.],
            "blue" => [0., 0., 1., 1.],
            "white" => [1., 1., 1., 1.],
            _ => [0., 0., 0., 1.],
        }
    }
}

impl Style {
    fn new(css: &HashMap<String, String>) -> Self {
        let width = match css.get("width").unwrap_or(&"".to_string()).ends_with("px") {
            true => css
                .get("width")
                .unwrap_or(&"".to_string())
                .replace("px", "")
                .parse::<i32>()
                .ok(),
            false => None,
        };
        let height = match css.get("height").unwrap_or(&"".to_string()).ends_with("px") {
            true => css
                .get("height")
                .unwrap_or(&"".to_string())
                .replace("px", "")
                .parse::<i32>()
                .ok(),
            false => None,
        };

        let border_radius = match css
            .get("border-radius")
            .unwrap_or(&"".to_string())
            .ends_with("px")
        {
            true => css
                .get("border-radius")
                .unwrap_or(&"".to_string())
                .replace("px", "")
                .parse::<f64>()
                .ok(),
            false => None,
        }
        .unwrap_or(0.);

        let background_color = match css.get("background-color") {
            Some(color) => get_color(color),
            None => [0., 0., 0., 1.],
        };

        let padding = match css.get("padding") {
            Some(padding) => {
                let padding: Vec<i32> =
                    padding.split(' ').map(|s| s.parse().unwrap_or(0)).collect();
                match padding.len() {
                    1 => [padding[0]; 4],
                    2 => [padding[0], padding[1], padding[0], padding[1]],
                    3 => [padding[0], padding[1], padding[2], padding[1]],
                    4 => [padding[0], padding[1], padding[2], padding[3]],
                    _ => [0; 4],
                }
            }
            None => [0; 4],
        };

        let margin = match css.get("margin") {
            Some(margin) => {
                let margin: Vec<i32> = margin.split(' ').map(|s| s.parse().unwrap_or(0)).collect();
                match margin.len() {
                    1 => [margin[0]; 4],
                    2 => [margin[0], margin[1], margin[0], margin[1]],
                    3 => [margin[0], margin[1], margin[2], margin[1]],
                    4 => [margin[0], margin[1], margin[2], margin[3]],
                    _ => [0; 4],
                }
            }
            None => [0; 4],
        };

        let content = css.get("content").map(|s| s.to_string());
        let font = Font::new(&css);

        Self {
            padding,
            margin,
            border_radius,
            content,
            font,
            width,
            height,
            background_color,
        }
    }
}

#[derive(Debug)]
struct Styles(pub HashMap<String, HashMap<String, String>>);

impl FromStr for Styles {
    type Err = CssError<'static>;

    fn from_str(css: &str) -> Result<Self, CssError<'static>> {
        let re = Regex::new(r"(?P<selector>\S+)\s*\{\s*(?P<properties>[^}]+)\s*\}").unwrap();
        let property_re = Regex::new(r"(?P<property>[\w-]+):\s*(?P<value>[^;]+);").unwrap();

        let split = css
            .split_inclusive('}')
            .filter_map(|a| {
                if a.trim().is_empty() {
                    return None;
                }
                Some(a.trim())
            })
            .collect::<Vec<&str>>();

        let styles = split
            .iter()
            .filter_map(|s| {
                let mut properties = HashMap::new();

                for cap in re.captures_iter(s) {
                    let selector = cap["selector"].to_string();

                    for property_cap in property_re.captures_iter(&cap["properties"]) {
                        properties.insert(
                            property_cap["property"].to_string(),
                            property_cap["value"].to_string(),
                        );
                    }

                    return Some((selector, properties));
                }

                None
            })
            .collect::<HashMap<String, HashMap<String, String>>>();

        Ok(Styles(styles))
    }
}
