#[cfg(test)]
mod tests {
    use css_image::{parse, Stylings};

    #[test]
    fn integration() {
        all_selector();
    }

    fn all_selector() {
        let css = r#"
        body { background-color: #ffffff; width: 100px; height: 100px; }
        "#;

        let result = Stylings::new(css);
        assert!(result.is_ok());
        let result = result.unwrap().styles;
        assert!(result.get("body").is_some());
        let body = result.get("body").unwrap();
        assert_eq!(body.width, 100);
        assert_eq!(body.height, 100);
    }

    /*
    fn single_style() {
        let css = r#"
        body { background-color: #ffffff; width: 100px; height: 100px; }
        "#;

        let result = Styles::from_str(css);
        assert!(result.is_ok());
        let result = result.unwrap().0;
        assert_eq!(result.len(), 1);
        assert!(result.get("body").is_some());
        let body = result.get("body").unwrap();
        assert!(body.get("width").is_some());
        assert!(body.get("height").is_some());
        assert!(body.get("background-color").is_some());
    }

    fn two_styles() {
        let css = r#"
        body { background-color: #ffffff; width: 100px; height: 100px; }

        body2 {
        background-color: #ffffff;
        width: 100px;
        height: 100px;
        }
        "#;

        let result = Styles::from_str(css);
        assert!(result.is_ok());
        let result = result.unwrap().0;
        assert_eq!(result.len(), 2);
        assert!(result.get("body").is_some());
        let body = result.get("body").unwrap();
        assert!(body.get("width").is_some());
        assert!(body.get("height").is_some());
        assert!(body.get("background-color").is_some());

        assert!(result.get("body2").is_some());
        let body = result.get("body2").unwrap();
        assert!(body.get("width").is_some());
        assert!(body.get("height").is_some());
        assert!(body.get("background-color").is_some());
    }
    */
}
