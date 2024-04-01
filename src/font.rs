use crate::error::CssError;

#[derive(Debug, PartialEq)]
pub(crate) struct Font {
    pub(crate) family: String,
    pub(crate) size: f64,
    pub(crate) color: [f64; 3],
    pub(crate) weight: cairo::FontWeight,
    pub(crate) text: String,
    pub(crate) slant: cairo::FontSlant,
}

impl Font {
    pub(crate) fn new(
        family: Option<&str>,
        size: Option<f64>,
        color: Option<&str>,
        weight: Option<&str>,
        slant: Option<&str>,
        text: &str,
    ) -> Result<Self, CssError<'static>> {
        let family = family.unwrap_or("Arial").to_string();
        let size = size.unwrap_or(16.0);
        let color = match color {
            Some("black") => [0.0, 0.0, 0.0],
            Some("white") => [1.0, 1.0, 1.0],
            Some("red") => [1.0, 0.0, 0.0],
            Some("green") => [0.0, 1.0, 0.0],
            Some("blue") => [0.0, 0.0, 1.0],
            Some(hex) if hex.starts_with('#') => {
                let hex = hex.trim_start_matches('#');
                let r = u8::from_str_radix(&hex[0..2], 16)? as f64 / 255.0;
                let g = u8::from_str_radix(&hex[2..4], 16)? as f64 / 255.0;
                let b = u8::from_str_radix(&hex[4..6], 16)? as f64 / 255.0;
                [r, g, b]
            }
            Some(rgb) if rgb.starts_with("rgb(") => {
                let rgb = rgb.trim_start_matches("rgb(").trim_end_matches(')');
                let mut parts = rgb.split(',');
                let r = parts
                    .next()
                    .ok_or(CssError::FontError("Invalid font color"))?;
                let g = parts
                    .next()
                    .ok_or(CssError::FontError("Invalid font color"))?;
                let b = parts
                    .next()
                    .ok_or(CssError::FontError("Invalid font color"))?;
                let r = r.trim().parse::<f64>()? / 255.0;
                let g = g.trim().parse::<f64>()? / 255.0;
                let b = b.trim().parse::<f64>()? / 255.0;
                [r, g, b]
            }
            Some(&_) => return Err(CssError::FontError("Invalid font color")),
            None => [0.0, 0.0, 0.0],
        };
        let weight = match weight {
            Some("bold") => cairo::FontWeight::Bold,
            Some("normal") => cairo::FontWeight::Normal,
            Some(&_) => return Err(CssError::FontError("Invalid font weight")),
            None => cairo::FontWeight::Normal,
        };

        let slant = match slant {
            Some("italic") => cairo::FontSlant::Italic,
            Some("normal") => cairo::FontSlant::Normal,
            Some(&_) => return Err(CssError::FontError("Invalid font style")),
            None => cairo::FontSlant::Normal,
        };

        Ok(Self {
            family,
            size,
            color,
            weight,
            text: text.to_string(),
            slant,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font() {
        let font = Font::new(None, None, None, None, None, "Test").unwrap();
        assert_eq!(font.family, "Arial");
        assert_eq!(font.size, 16.0);
        assert_eq!(font.color, [0.0, 0.0, 0.0]);
        assert_eq!(font.weight, cairo::FontWeight::Normal);
        assert_eq!(font.text, "Test");

        let font = Font::new(
            Some("Arial"),
            Some(16.0),
            Some("black"),
            Some("bold"),
            Some("italic"),
            "Test",
        )
        .unwrap();
        assert_eq!(font.family, "Arial");
        assert_eq!(font.size, 16.0);
        assert_eq!(font.color, [0.0, 0.0, 0.0]);
        assert_eq!(font.weight, cairo::FontWeight::Bold);
        assert_eq!(font.text, "Test");

        let font = Font::new(
            Some("Arial"),
            Some(16.0),
            Some("#000000"),
            Some("normal"),
            Some("normal"),
            "Test",
        )
        .unwrap();
        assert_eq!(font.family, "Arial");
        assert_eq!(font.size, 16.0);
        assert_eq!(font.color, [0.0, 0.0, 0.0]);
        assert_eq!(font.weight, cairo::FontWeight::Normal);
        assert_eq!(font.text, "Test");

        let font = Font::new(
            Some("Arial"),
            Some(16.0),
            Some("rgb(0,0,0)"),
            Some("bold"),
            Some("italic"),
            "Test",
        )
        .unwrap();
        assert_eq!(font.family, "Arial");
        assert_eq!(font.size, 16.0);
        assert_eq!(font.color, [0.0, 0.0, 0.0]);
        assert_eq!(font.weight, cairo::FontWeight::Bold);
        assert_eq!(font.text, "Test");

        let font = Font::new(
            Some("Arial"),
            Some(16.0),
            Some("rgb(0,0,0)"),
            Some("bold"),
            Some(""),
            "Test",
        );
        assert!(font.is_err());

        let font = Font::new(
            Some("Arial"),
            Some(16.0),
            Some(""),
            Some("bold"),
            Some("italic"),
            "Test",
        );
        assert!(font.is_err());

        let font = Font::new(
            Some("Arial"),
            Some(16.0),
            Some("rgb(0,0,0)"),
            Some(""),
            Some("italic"),
            "Test",
        );
        assert!(font.is_err());
    }
}
