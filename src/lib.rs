use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw_smiley_at(canvas: web_sys::HtmlCanvasElement, x: f64, y: f64, scale: f64) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(x, y, 50.0 * scale, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(x + 35.0 * scale, y);
    context
        .arc(x, y, 35.0 * scale, 0.0, f64::consts::PI)
        .unwrap();

    // Draw the left eye.
    context.move_to(x - 5.0 * scale, y - 10.0 * scale);
    context
        .arc(
            x - 10.0 * scale,
            y - 10.0 * scale,
            5.0 * scale,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    // Draw the right eye.
    context.move_to(x + 5.0 * scale, y - 10.0 * scale);
    context
        .arc(
            x + 10.0 * scale,
            y - 10.0 * scale,
            5.0 * scale,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    context.stroke();
}

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    // make the canvas same res as screen
    canvas.set_width(document.body().unwrap().client_width() as u32);
    canvas.set_height(document.body().unwrap().client_height() as u32);

    // draw smileys in a grid 5x5
    for i in 0..5 {
        for j in 0..5 {
            draw_smiley_at(
                canvas.clone(),
                50.0 + i as f64 * 100.0,
                50.0 + j as f64 * 100.0,
                1.0,
            );
        }
    }
}
