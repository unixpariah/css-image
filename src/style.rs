use crate::error::CssError;
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
    pub width: u64,
    pub height: u64,
}

impl Style {
    fn new(css: &HashMap<String, String>) -> Self {
        let width = match css.get("width").unwrap_or(&"".to_string()).ends_with("px") {
            true => css
                .get("width")
                .unwrap_or(&"".to_string())
                .replace("px", "")
                .parse(),
            false => Ok(5),
        }
        .unwrap_or(5);
        let height = match css.get("height").unwrap_or(&"".to_string()).ends_with("px") {
            true => css
                .get("height")
                .unwrap_or(&"".to_string())
                .replace("px", "")
                .parse(),
            false => Ok(5),
        }
        .unwrap_or(5);

        Self { width, height }
    }
}

impl Into<Styles> for String {
    fn into(self) -> Styles {
        Styles::from_str(&self).unwrap()
    }
}

impl From<&str> for Styles {
    fn from(value: &str) -> Self {
        Styles::from_str(value).unwrap()
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
