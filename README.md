# css-image

Rust crate for rendering images from css

[![Build Status](https://github.com/unixpariah/css-image/actions/workflows/test.yml/badge.svg)](https://github.com/unixpariah/css-image/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/unixpariah/css-image/graph/badge.svg?token=49LRWZ9D1K)](https://codecov.io/gh/unixpariah/css-image)

## Features

Only px units are supported for now.

- [x] width/height
- [x] background-color, color
- [x] font-size, font-family, font-weight, font-style, content, text-align
- [x] margin
- [x] padding
- [x] * selector
- [ ] border, border-radius, border-color, border-width, border-style
- [ ] multiple selectors

## Dependencies

- cairo

## Usage

```rust
use css_image::parse;

fn main() {
    let css = r#"
        body {
            background-color: red;
            width: 100px;
            height: 100px;
        }
    "#;

    let parsed = parse(css).unwrap();
    println!("{:?}", parsed); // Returns a hashmap of css selector -> Image
}
```
