use css_image::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration() {
        let css = r#"
        body { background-color: #FFFFFF; width: 100px; height: 100px; }
        "#
        .to_string();

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

        let css = r#"
        * {
            width: 5px;
        }

        body {
            width: auto;
            height: auto;
            font-size: 20;
            font-style: italic;
            font-weight: bold;
            color: red;
            content: "aaa";
            margin-top: 10px;
            margin-right: 20px;
            margin-bottom: 30px;
            margin-left: 40px;
        }

        body2 {
            width: auto;
            height: auto;
            font-size: 20;
            font-style: italic;
            font-weight: bold;
            color: red;
            content: "aaa";
            background-color: #FF0000;
            margin: 10px 20px 30px 40px;
        }
        "#
        .to_string();
        let result = parse(css);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
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
        assert!(result.is_ok());

        let css = r#"body { width: four; height: 100px; }"#.to_string();
        let result = parse(css);
        assert!(result.is_ok());

        let css = r#"aha { content: "aaa"; }"#.to_string();
        let result = parse(css);
        assert!(result.is_ok());
    }
}
