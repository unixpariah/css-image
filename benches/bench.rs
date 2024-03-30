use criterion::{criterion_group, criterion_main, Criterion};
use css::parse;

fn bench_resize(c: &mut Criterion) {
    let css = r#"one { background-color: #FFFFFF; width: 100; height: 100; }"#.to_string();

    c.bench_function("single_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"one { background-color: #FFFFFF; width: 100; height: 100; } 
        two { background-color: #FFFFFF; width: 100; height: 100; }"#
        .to_string();

    c.bench_function("double_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"one { background-color: #FFFFFF; width: 100; height: 100; } 
        two { background-color: #FFFFFF; width: 100; height: 100; } 
        three { background-color: #FFFFFF; width: 100; height: 100; }"#
        .to_string();

    c.bench_function("triple_element", |b| b.iter(|| parse(css.clone())));

    let css = r#"one { background-color: #FFFFFF; width: 100; height: 100; } 
        two { background-color: #FFFFFF; width: 100; height: 100; } 
        three { background-color: #FFFFFF; width: 100; height: 100; } 
        four { background-color: #FFFFFF; width: 100; height: 100; } 
        five { background-color: #FFFFFF; width: 100; height: 100; } 
        six { background-color: #FFFFFF; width: 100; height: 100; } 
        seven { background-color: #FFFFFF; width: 100; height: 100; } 
        eight { background-color: #FFFFFF; width: 100; height: 100; } 
        nine { background-color: #FFFFFF; width: 100; height: 100; content: "aaaa"; }"#
        .to_string();

    c.bench_function("complex_style", |b| b.iter(|| parse(css.clone())));
}

criterion_group!(benches, bench_resize);
criterion_main!(benches);
