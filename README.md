# [WORK IN PROGRES] Idk how to name it yet

Its a rust library for creating images from css

[![Build Status](https://github.com/unixpariah/css-something/actions/workflows/test.yml/badge.svg)](https://github.com/unixpariah/css-something/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/unixpariah/css-something/graph/badge.svg?token=49LRWZ9D1K)](https://codecov.io/gh/unixpariah/css-something)

## Usage

```rust
use css::parse;

fn main() {
    let css = r#"
        body {
            background-color: red;
        }
    "#;

    let parsed = parse(css).unwrap();
    println!("{:?}", parsed); // Output: HashMap<String, Vec<u8>>
}
```
