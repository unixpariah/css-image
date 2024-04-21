use criterion::{criterion_group, criterion_main, Criterion};
use css_image::{parse, render};

fn bench_resize(c: &mut Criterion) {
    let css = r#"
    one { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    "#;

    c.bench_function("single_element_parse", |b| b.iter(|| parse(css)));
    c.bench_function("single_element_parse_render", |b| b.iter(|| render(css)));

    let css = r#"
    one { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    two { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    "#;

    c.bench_function("double_element_parse", |b| b.iter(|| parse(css)));
    c.bench_function("double_element_parse_render", |b| b.iter(|| render(css)));

    let css = r#"
    one { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    two { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    three { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; color: red; font-size: 20; font-weight: bold; font-style: italic; }
    "#;

    c.bench_function("triple_element_parse", |b| b.iter(|| parse(css)));
    c.bench_function("triple_element_parse_render", |b| b.iter(|| render(css)));

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
    "#;

    c.bench_function("complex_element_parse", |b| b.iter(|| parse(css)));
    c.bench_function("complex_element_parse_render", |b| b.iter(|| render(css)));
}

criterion_group!(benches, bench_resize);
criterion_main!(benches);
