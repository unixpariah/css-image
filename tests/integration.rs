use css::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration() {
        let css = r#"body { background-color: #FFFFFF; width: 100px; height: 100px; }"#.to_string();
        let result = parse(css);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let css =
            r#"body { background-color: rgba(255, 255, 255, 255); width: 100px; height: 100px; }"#
                .to_string();
        let result = parse(css);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let css = r#"body { background-color: rgb(255, 255, 255); width: 100px; height: 100px; }"#
            .to_string();
        let result = parse(css);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let css = r#"body { background-color: red; width: 100px; height: 100px; }"#.to_string();
        let result = parse(css);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let css = r#"body { background-color: error; width: 100px; height: 100px; }"#.to_string();
        let result = parse(css);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let css = r#"body { width: auto; height: auto; font-size: 20; font-style: italic; font-weight: bold; color: red; content: "aaa"; }"#.to_string();
        let result = parse(css);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn empty() {
        let css = "".to_string();
        let result = parse(css);
        assert!(result.is_err());

        let css = "body { }".to_string();
        let result = parse(css);
        assert!(result.is_err());
    }

    #[test]
    fn wrong_size() {
        let css = r#"body { width: 100px; height: two; }"#.to_string();
        let result = parse(css);
        assert!(result.is_err());

        let css = r#"body { width: four; height: 100px; }"#.to_string();
        let result = parse(css);
        assert!(result.is_err());
    }
}
