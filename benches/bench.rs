use criterion::{criterion_group, criterion_main, Criterion};
use css_image::parse;

fn bench_resize(c: &mut Criterion) {
    let css = r#"
    one { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    "#
        .to_string();

    c.bench_function("single_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"
    one { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    two { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    "#
        .to_string();

    c.bench_function("double_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"
    one { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    two { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    three { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    "#
        .to_string();

    c.bench_function("triple_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"
    one { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    two { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    three { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    four { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    five { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    six { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    seven { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    eight { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    nine { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    ten { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    "#
        .to_string();

    c.bench_function("complex_style", |b| b.iter(|| parse(css.clone())));
}

criterion_group!(benches, bench_resize);
criterion_main!(benches);
