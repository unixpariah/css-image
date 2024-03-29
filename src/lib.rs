use std::error::Error;

struct Style {
    name: String,
    content: String,
}

impl Style {
    fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}

pub fn parse(mut css: String) -> Result<(), Box<dyn Error>> {
    let mut styles = vec![];
    while !css.is_empty() {
        let opening_brace_pos = css.find('{');
        let closing_brace_pos = css.find('}');

        let style = match (opening_brace_pos, closing_brace_pos) {
            (Some(open), Some(close)) => {
                let name = css.drain(..open).collect::<String>().trim().to_string();
                let style = css
                    .drain(open + 1..=close)
                    .collect::<String>()
                    .trim()
                    .to_string();

                Style::new(name, style)
            }
            _ => return Err("Invalid CSS".into()),
        };

        styles.push(style);
    }

    for style in styles {
        println!("Name: {}", style.name);
        println!("Content: {}", style.content);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        _ = parse(
            "body { color: red; font-size: 2rem } aaa { color: red; font-size: 2rem }".to_string(),
        );
    }
}
