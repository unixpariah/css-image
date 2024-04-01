# css-image

Rust crate for rendering images from css

[![Build Status](https://github.com/unixpariah/css-image/actions/workflows/test.yml/badge.svg)](https://github.com/unixpariah/css-image/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/unixpariah/css-image/graph/badge.svg?token=49LRWZ9D1K)](https://codecov.io/gh/unixpariah/css-image)

## Features

- [x] width/height (Only px)
- [x] background-color, color 
- [x] font-size (Only px), font-family, font-weight, font-style
- [x] margin (Only px)
- [ ] text-align
- [ ] border, border-radius, border-color, border-width, border-style 
- [ ] padding
- [ ] * (all)
- [ ] multiple selectors

## Dependencies

- cairo

## Usage

```rust
use css::parse;

fn main() {
    let css = r#"
        body {
            background-color: red;
            width: 100px;
            height: 100px;
        }
    "#;

    let parsed = parse(css).unwrap();
    println!("{:?}", parsed); // Output: HashMap<String, Vec<u8>>
}
```
