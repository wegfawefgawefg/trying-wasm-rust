use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};

fn canvas_context(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("2D context is unavailable"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| JsValue::from_str("Failed to cast to CanvasRenderingContext2d"))
}

fn draw_smiley(
    context: &web_sys::CanvasRenderingContext2d,
    x: f64,
    y: f64,
    scale: f64,
) -> Result<(), JsValue> {
    context.begin_path();

    context.arc(x, y, 50.0 * scale, 0.0, std::f64::consts::PI * 2.0)?;

    context.move_to(x + 35.0 * scale, y);
    context.arc(x, y, 35.0 * scale, 0.0, std::f64::consts::PI)?;

    context.move_to(x - 5.0 * scale, y - 10.0 * scale);
    context.arc(
        x - 10.0 * scale,
        y - 10.0 * scale,
        5.0 * scale,
        0.0,
        std::f64::consts::PI * 2.0,
    )?;

    context.move_to(x + 5.0 * scale, y - 10.0 * scale);
    context.arc(
        x + 10.0 * scale,
        y - 10.0 * scale,
        5.0 * scale,
        0.0,
        std::f64::consts::PI * 2.0,
    )?;

    context.stroke();
    Ok(())
}

fn viewport_size(window: &web_sys::Window) -> Result<(f64, f64), JsValue> {
    let width = window
        .inner_width()?
        .as_f64()
        .ok_or_else(|| JsValue::from_str("Failed to read window.innerWidth"))?;
    let height = window
        .inner_height()?
        .as_f64()
        .ok_or_else(|| JsValue::from_str("Failed to read window.innerHeight"))?;

    Ok((width, height))
}

fn redraw_grid(canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window not available"))?;
    let (width, height) = viewport_size(&window)?;
    let dpr = window.device_pixel_ratio().max(1.0);

    canvas
        .style()
        .set_property("width", &format!("{}px", width.round()))?;
    canvas
        .style()
        .set_property("height", &format!("{}px", height.round()))?;
    canvas.set_width((width * dpr).round() as u32);
    canvas.set_height((height * dpr).round() as u32);

    let context = canvas_context(canvas)?;
    context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;
    context.clear_rect(
        0.0,
        0.0,
        f64::from(canvas.width()),
        f64::from(canvas.height()),
    );
    context.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0)?;

    let cols = ((width / 120.0).floor() as u32).clamp(1, 10);
    let rows = ((height / 120.0).floor() as u32).clamp(1, 10);

    let x_step = width / (f64::from(cols) + 1.0);
    let y_step = height / (f64::from(rows) + 1.0);

    for i in 0..cols {
        for j in 0..rows {
            let x = x_step * (f64::from(i) + 1.0);
            let y = y_step * (f64::from(j) + 1.0);
            draw_smiley(&context, x, y, 0.9)?;
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub fn draw_smiley_at(
    canvas: web_sys::HtmlCanvasElement,
    x: f64,
    y: f64,
    scale: f64,
) -> Result<(), JsValue> {
    let context = canvas_context(&canvas)?;
    draw_smiley(&context, x, y, scale)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window not available"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document not available"))?;

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| JsValue::from_str("Missing #canvas element"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("#canvas is not a HtmlCanvasElement"))?;

    redraw_grid(&canvas)?;

    let canvas_for_resize = canvas.clone();
    let on_resize = Closure::<dyn FnMut()>::new(move || {
        if let Err(error) = redraw_grid(&canvas_for_resize) {
            web_sys::console::error_1(&error);
        }
    });
    window.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())?;
    on_resize.forget();

    let canvas_for_click = canvas.clone();
    let on_click =
        Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            if let Err(error) = draw_smiley_at(
                canvas_for_click.clone(),
                f64::from(event.offset_x()),
                f64::from(event.offset_y()),
                0.75,
            ) {
                web_sys::console::error_1(&error);
            }
        });
    canvas.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())?;
    on_click.forget();

    Ok(())
}
