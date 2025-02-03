mod color;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::*;
use color::*;

fn draw_rounded_rect(
    ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64, radius: f64
) -> JsResult<()> {
    ctx.begin_path();
    ctx.set_stroke_style_str(CARD_BORDER);
    ctx.move_to(x + radius, y);
    ctx.arc_to(x + width, y, x + width, y + radius, radius)?;
    ctx.arc_to(x + width, y + height, x + width - radius, y + height, radius)?;
    ctx.arc_to(x, y + height, x, y + height - radius, radius)?;
    ctx.arc_to(x, y, x + radius, y, radius)?;
    ctx.close_path();
    ctx.stroke();
    Ok(())
}

fn draw_normal(cs: ClientState) -> JsResult<()> {
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

fn draw_blocked(cs: ClientState) -> JsResult<()> {
    Ok(())
}

pub fn should_block() -> bool {
    // TODO: should_block
    false
}

pub fn draw(cs: ClientState) -> JsResult<()> {
    if should_block() {
        draw_blocked(cs)?;
    } else {
        draw_normal(cs)?;
    }
    Ok(())
}
