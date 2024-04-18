#[cfg(test)]
mod tests {
    use css_image::Stylings;

    #[test]
    fn integration() {
        single_style();
        all_selector();
    }

    fn single_style() {
        let css = r#"
        body { color: #ffffff; width: 100px; height: 100px; background-color: #ffffff; }
        "#;

        let result = Stylings::new(css);
        assert!(result.is_ok());
        let result = result.unwrap().styles;
        assert_eq!(result.len(), 1);
        assert!(result.get("body").is_some());
        let body = result.get("body").unwrap();
        assert_eq!(body.width.unwrap(), 100);
        assert_eq!(body.height.unwrap(), 100);
        assert_eq!(body.background_color, [1., 1., 1., 1.]);
        assert_eq!(body.font.color, [1., 1., 1., 1.]);
    }

    fn all_selector() {
        let css = r#"
        body { }

        * { color: #ffffff; width: 100px; height: 100px; background-color: #ffffff; }
        "#;

        let result = Stylings::new(css);
        assert!(result.is_ok());
        let result = result.unwrap().styles;
        assert_eq!(result.len(), 2);
        assert!(result.get("body").is_some());
        let body = result.get("body").unwrap();
        assert_eq!(body.width.unwrap(), 100);
        assert_eq!(body.height.unwrap(), 100);
        assert_eq!(body.background_color, [1., 1., 1., 1.]);
        assert_eq!(body.font.color, [1., 1., 1., 1.]);
    }
}
