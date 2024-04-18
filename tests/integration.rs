#[cfg(test)]
mod tests {
    use css_image::{parse, render};

    #[test]
    fn test_auto_sizes() {
        let css = r#"
        body {
        width: 100px;
        content: "aaa";
        text-align: right;
        }
        "#;

        let result = render(css);
        assert!(result.is_ok());

        let css = r#"
        body {
        height: 100px;
        content: "aaa";
        text-align: center;
        }
        "#;

        let result = render(css);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_render() {
        let css = r#"
        body {
        color: #ffffff;
        width: 100px;
        height: 100px;
        background-color: #ffffff;
        margin: 10px 20px 30px 40px;
        padding: 10px 20px 30px 40px;
        content: "Hello, World!";
        border-radius: 10px;
        }
        "#;

        let result = render(css);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_then_render() {
        let css = r#"
        body {
        color: #ffffff;
        width: 100px;
        height: 100px;
        background-color: #ffffff;
        margin: 10px 20px 30px 40px;
        padding: 10px 20px 30px 40px;
        content: "hello, world!";
        border-radius: 10px;
        }
        "#;

        let result = parse(css);
        assert!(result.is_ok());

        let result = render(result.unwrap());
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.get("body").is_some());
    }
}
