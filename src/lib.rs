mod error;
mod style;

use cairo::Context;
use error::CssError;
use std::collections::HashMap;
pub use style::Stylings;

pub fn parse(css: &str) -> Result<HashMap<String, Vec<u8>>, CssError<'static>> {
    /*
        styles.iter().for_each(|style| {
            let all_selector = styles.get("*");

            let selector = style.0;
            let style = style.1;

            let height = style
                .get("height")
                .or_else(|| all_selector.map(|height| height.get("height")).flatten())
                .map_or_else(|| "5px", |height| height)
                .replace("px", "")
                .parse::<i32>()
                .unwrap_or(5);

            println!("{height} {width}");
        });
    */
    Ok(HashMap::new())
}

fn draw_rectangle(context: &Context, x: f64, y: f64, width: f64, height: f64, border_radius: f64) {
    let border_radius = match border_radius > 20. {
        true => 20. / 3.33,
        false => border_radius / 3.33,
    };
    let degrees = std::f64::consts::PI / 180.0;

    context.new_sub_path();
    context.arc(
        x + width - border_radius,
        y + border_radius,
        border_radius,
        -90.0 * degrees,
        0.0 * degrees,
    );
    context.arc(
        x + width - border_radius,
        y + height - border_radius,
        border_radius,
        0.0 * degrees,
        90.0 * degrees,
    );
    context.arc(
        x + border_radius,
        y + height - border_radius,
        border_radius,
        90.0 * degrees,
        180.0 * degrees,
    );
    context.arc(
        x + border_radius,
        y + border_radius,
        border_radius,
        180.0 * degrees,
        270.0 * degrees,
    );
    context.close_path();
}
