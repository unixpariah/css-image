use criterion::{criterion_group, criterion_main, Criterion};
use css::parse;

fn bench_resize(c: &mut Criterion) {
    let css = r#"one { background-color: #FFFFFF; width: 100px; height: 100px; }"#.to_string();

    c.bench_function("single_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"one { background-color: #FFFFFF; width: 100px; height: 100px; } 
        two { background-color: #FFFFFF; width: 100px; height: 100px; }"#
        .to_string();

    c.bench_function("double_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"one { background-color: #FFFFFF; width: 100px; height: 100px; } 
        two { background-color: #FFFFFF; width: 100px; height: 100px; } 
        three { background-color: #FFFFFF; width: 100px; height: 100px; }"#
        .to_string();

    c.bench_function("triple_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"one { background-color: #FFFFFF; width: 100px; height: 100px; } 
        two { background-color: #FFFFFF; width: 100px; height: 100px; } 
        three { background-color: #FFFFFF; width: 100px; height: 100px; } 
        four { background-color: #FFFFFF; width: 100px; height: 100px; } 
        five { background-color: #FFFFFF; width: 100px; height: 100px; } 
        six { background-color: #FFFFFF; width: 100px; height: 100px; } 
        seven { background-color: #FFFFFF; width: 100px; height: 100px; } 
        eight { background-color: #FFFFFF; width: 100px; height: 100px; } 
        nine { background-color: #FFFFFF; width: 100px; height: 100px; content: "aaaa"; }"#
        .to_string();

    c.bench_function("complex_style", |b| b.iter(|| parse(css.clone())));
}

criterion_group!(benches, bench_resize);
criterion_main!(benches);
