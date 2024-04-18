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
        let all_selector = css.get("*");
        let styles = css
            .iter()
            .map(|(selector, properties)| {
                let style = Style::new(properties, all_selector);
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
    fn new(css: &HashMap<String, String>, all_selector: Option<&HashMap<String, String>>) -> Self {
        let get_property = |property: &str| {
            css.get(property)
                .and_then(|s| {
                    if s.ends_with("px") {
                        Some(s.replace("px", ""))
                    } else {
                        None
                    }
                })
                .and_then(|s| s.parse::<i32>().ok())
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
                        .and_then(|s| s.parse::<i32>().ok())
                })
        };

        let width = get_property("width");
        let height = get_property("height");
        let border_radius = get_property("border-radius").unwrap_or(0) as f64;

        let background_color = css
            .get("background-color")
            .or_else(|| all_selector.as_ref()?.get("background-color"))
            .map(|color| get_color(color))
            .unwrap_or([0., 0., 0., 1.]);

        let get_padding_or_margin = |property: &str| match css
            .get(property)
            .or_else(|| all_selector.as_ref()?.get(property))
        {
            Some(value) => {
                let values: Vec<i32> = value.split(' ').map(|s| s.parse().unwrap_or(0)).collect();
                match values.len() {
                    1 => [values[0]; 4],
                    2 => [values[0], values[1], values[0], values[1]],
                    3 => [values[0], values[1], values[2], values[1]],
                    4 => [values[0], values[1], values[2], values[3]],
                    _ => [0; 4],
                }
            }
            None => [0; 4],
        };

        let mut padding = get_padding_or_margin("padding");
        let mut margin = get_padding_or_margin("margin");

        [
            "padding-top",
            "padding-right",
            "padding-bottom",
            "padding-left",
        ]
        .into_iter()
        .for_each(|direction| {
            if let Some(value) = get_property(direction) {
                match direction {
                    "padding-top" => padding[0] = value,
                    "padding-right" => padding[1] = value,
                    "padding-bottom" => padding[2] = value,
                    "padding-left" => padding[3] = value,
                    _ => {}
                }
            }
        });

        ["margin-top", "margin-right", "margin-bottom", "margin-left"]
            .into_iter()
            .for_each(|direction| {
                if let Some(value) = get_property(direction) {
                    match direction {
                        "margin-top" => margin[0] = value,
                        "margin-right" => margin[1] = value,
                        "margin-bottom" => margin[2] = value,
                        "margin-left" => margin[3] = value,
                        _ => {}
                    }
                }
            });

        let content = css
            .get("content")
            .map(|s| s.trim().replace("\"", "").to_string());

        let font = Font::new(&css, all_selector);

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
