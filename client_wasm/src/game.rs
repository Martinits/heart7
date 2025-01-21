use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::*;

const RED: &str = "#AA0000";

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn draw_rounded_rect(
    ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64, radius: f64
) -> JsResult<()> {
    ctx.begin_path();
    ctx.set_stroke_style_str(RED);
    ctx.move_to(x + radius, y);
    ctx.arc_to(x + width, y, x + width, y + radius, radius)?;
    ctx.arc_to(x + width, y + height, x + width - radius, y + height, radius)?;
    ctx.arc_to(x, y + height, x, y + height - radius, radius)?;
    ctx.arc_to(x, y, x + radius, y, radius)?;
    ctx.close_path();
    ctx.stroke();
    Ok(())
}

pub fn start_game() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let canvas: HtmlCanvasElement = window
        .document()
        .ok_or(JsError::new("Cannot get Canvas element"))?
        .get_element_by_id("heart7_canvas")
        .ok_or(JsError::new("Cannot get heart7_canvas"))?
        .dyn_into()?;

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .ok_or(JsError::new("Cannot get Canvas2DContext"))?
        .dyn_into()?;

    draw_rounded_rect(&context, 400f64, 300f64, 80f64, 60f64, 5f64)?;

    Ok(())
}
