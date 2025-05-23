use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
    HtmlInputElement,
    HtmlImageElement,
};
use crate::*;

pub const ROUNDED_RECT_RADIUS: f64 = 10f64;
pub const DEFAULT_STROKE_WIDTH: f64 = 2f64;

pub fn get_canvas_dimension() -> (f64, f64) {
    let canvas = get_canvas();
    (canvas.width() as f64, canvas.height() as f64)
}

pub fn get_canvas_position() -> (f64, f64) {
    let canvas = get_canvas().get_bounding_client_rect();
    (canvas.left(), canvas.top())
}

pub fn get_canvas_rect() -> Rect {
    let (w, h) = get_canvas_dimension();
    Rect {
        x: 0f64,
        y: 0f64,
        w,
        h,
    }
}

pub fn get_canvas() -> HtmlCanvasElement {
    gloo::utils::body()
        .query_selector("#heart7-canvas").unwrap_throw()
        .ok_or("cannot find canvas element").unwrap_throw()
        .dyn_into::<HtmlCanvasElement>().unwrap_throw()
}

pub fn get_hidden_input() -> HtmlInputElement {
    gloo::utils::body()
        .query_selector("#hidden-input").unwrap_throw()
        .ok_or("cannot find hidden input element").unwrap_throw()
        .dyn_into::<HtmlInputElement>().unwrap_throw()
}

pub fn get_image(img: &str) -> HtmlImageElement {
    let img = format!("#img-{}", img);
    gloo::utils::body()
        .query_selector(&img).unwrap_throw()
        .ok_or(&format!("cannot find image element: {}", img)).unwrap_throw()
        .dyn_into::<HtmlImageElement>().unwrap_throw()
}

pub fn get_canvas_ctx() -> CanvasRenderingContext2d {
    get_canvas()
        .get_context("2d").unwrap_throw()
        .ok_or("Cannot get Canvas2DContext").unwrap_throw()
        .dyn_into().unwrap_throw()
}

pub fn clear_rect(rect: &Rect) {
    get_canvas_ctx().clear_rect(rect.x, rect.y, rect.w, rect.h);
}

pub fn clear_canvas() {
    clear_rect(&get_canvas_rect())
}

#[derive(Clone, Debug, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Clone)]
pub enum Slice1D<T> where T: Into<f64> {
    Percent(T),
    Fixed(T),
}

#[allow(unused)]
impl Rect {
    // cut out the center of `org` with v and h
    pub fn center_cut<W, H>(&self, w: Slice1D<W>, h: Slice1D<H>) -> Rect
    where
        W: Copy + Into<f64>,
        H: Copy + Into<f64>,
    {
        self.center_cut_width(w).center_cut_height(h)
    }

    pub fn center_cut_width<T: Copy + Into<f64>>(&self, w: Slice1D<T>) -> Rect {
        let (xoff, w) = match w {
            Percent(p) => Self::center_cut_1d(self.w, p.into()),
            Fixed(f) => Self::center_cut_1d_fixed(self.w, f.into()),
        };
        Rect {
            x: self.x + xoff,
            y: self.y,
            w,
            h: self.h,
        }
    }

    pub fn center_cut_height<T: Copy + Into<f64>>(&self, h: Slice1D<T>) -> Rect {
        let (yoff, h) = match h {
            Percent(p) => Self::center_cut_1d(self.h, p.into()),
            Fixed(f) => Self::center_cut_1d_fixed(self.h, f.into()),
        };
        Rect {
            x: self.x,
            y: self.y + yoff,
            w: self.w,
            h,
        }
    }

    fn center_cut_1d(len: f64, center: f64) -> (f64, f64) {
        Self::center_cut_1d_fixed(len, len * center / 100f64)
    }

    fn center_cut_1d_fixed(len: f64, center: f64) -> (f64, f64) {
        assert!(len >= center);
        ((len - center) / 2f64 , center)
    }

    pub fn get_x_center(&self) -> f64 {
        self.x + self.w / 2f64
    }

    pub fn get_y_center(&self) -> f64 {
        self.y + self.h / 2f64
    }

    // cut `self` by width into slices according to a list of `Slice1D`
    // if any space left, will be appended as the last slice
    pub fn cut_width<T: Into<f64>>(&self, s: impl IntoIterator<Item = Slice1D<T>>) -> Vec<Rect> {
        let mut sum = 0f64;
        let mut ans = vec![];
        for each in s {
            let w = self.width_slice(each);
            if sum + w > self.w {
                warn!("Rect cut_width oveflow: need {}, left {}", w, self.w - sum);
                break;
            }
            ans.push(Rect {
                x: self.x + sum,
                y: self.y,
                w,
                h: self.h,
            });
            sum += w;
        }
        if sum < self.w {
            ans.push(Rect {
                x: self.x + sum,
                y: self.y,
                w: self.w - sum,
                h: self.h,
            });
        }
        ans
    }

    // cut `self` by height into slices according to a list of `Slice1D`
    // if any space left, will be appended as the last slice
    pub fn cut_height<T: Into<f64>>(&self, s: impl IntoIterator<Item = Slice1D<T>>) -> Vec<Rect> {
        let mut sum = 0f64;
        let mut ans = vec![];
        for each in s.into_iter() {
            let h = self.height_slice(each);
            if sum + h > self.h {
                warn!("Rect cut_height oveflow: need {}, left {}", h, self.h - sum);
                break;
            }
            ans.push(Rect {
                x: self.x,
                y: self.y + sum,
                w: self.w,
                h,
            });
            sum += h;
        }
        if sum < self.h {
            ans.push(Rect {
                x: self.x,
                y: self.y + sum,
                w: self.w,
                h: self.h - sum,
            });
        }
        ans
    }

    pub fn width_slice<T: Into<f64>>(&self, s: Slice1D<T>) -> f64 {
        Self::slice_1d(self.w, s)
    }

    pub fn height_slice<T: Into<f64>>(&self, s: Slice1D<T>) -> f64 {
        Self::slice_1d(self.h, s)
    }

    fn slice_1d<T: Into<f64>>(di: f64, s: Slice1D<T>) -> f64 {
        match s {
            Percent(p) => di * p.into() / 100f64,
            Fixed(f) => f.into(),
        }
    }

    pub fn cut_border<W, H>(&self, w: Slice1D<W>, h: Slice1D<H>) -> Rect
    where
        W: Copy + Into<f64>,
        H: Copy + Into<f64>,
    {
        self.cut_border_fixed(self.width_slice(w), self.height_slice(h))
    }

    pub fn cut_border_equal<T: Copy + Into<f64>>(&self, b: Slice1D<T>) -> Rect {
        self.cut_border(b.clone(), b)
    }

    pub fn cut_border_fixed<T: Copy + Into<f64>>(&self, w: T, h: T) -> Rect {
        let wcut = 2f64 * w.into();
        let neww = if self.w <= wcut {
            warn!("Cut a border of {} out of width {} is not enough", w.into(), self.w);
            0f64
        } else {
            self.w - wcut
        };

        let hcut = 2f64 * h.into();
        let newh = if self.h <= hcut {
            warn!("Cut a border of {} out of height {} is not enough", h.into(), self.h);
            0f64
        } else {
            self.h - hcut
        };

        Rect {
            x: self.x + w.into(),
            y: self.y + h.into(),
            w: neww,
            h: newh,
        }
    }

    pub fn cut_border_fixed_equal<T: Copy + Into<f64>>(&self, b: T) -> Rect {
        self.cut_border_fixed(b, b)
    }

    pub fn is_clicked_in<W, H>(&self, x: W, y: H) -> bool
    where
        W: Copy + Into<f64>,
        H: Copy + Into<f64>,
    {
        let x = x.into();
        let y = y.into();
        self.x <= x && x <= self.x + self.w && self.y <= y && y <= self.y + self.h
    }

    pub fn shift<X, Y>(&mut self, dx: X, dy: Y)
    where
        X: Copy + Into<f64>,
        Y: Copy + Into<f64>,
    {
        let x = self.x + dx.into();
        if x < 0f64 {
            warn!("Move rect results a negative x {}", x);
        }
        let y = self.y + dy.into();
        if y < 0f64 {
            warn!("Move rect results a negative y {}", y);
        }
        self.x = x;
        self.y = y;
    }
}

pub fn draw_rect(rect: &Rect, color: &str) {
    let ctx = get_canvas_ctx();
    ctx.set_stroke_style_str(color);
    ctx.set_line_width(DEFAULT_STROKE_WIDTH);
    ctx.stroke_rect(rect.x, rect.y, rect.w, rect.h);
}

pub fn draw_rounded_rect(rect: &Rect, color: &str) {
    draw_rounded_rect_with_r(rect, ROUNDED_RECT_RADIUS, color);
}

pub fn draw_rounded_rect_with_r(rect: &Rect, r: f64, color: &str) {
    let ctx = get_canvas_ctx();
    let x = rect.x;
    let y = rect.y;
    let w = rect.w;
    let h = rect.h;
    ctx.set_stroke_style_str(color);
    ctx.set_line_width(DEFAULT_STROKE_WIDTH);
    ctx.begin_path();
    ctx.move_to(x + r, y);
    ctx.arc_to(x + w, y, x + w, y + r, r).unwrap_throw();
    ctx.arc_to(x + w, y + h, x + w - r, y + h, r).unwrap_throw();
    ctx.arc_to(x, y + h, x, y + h - r, r).unwrap_throw();
    ctx.arc_to(x, y, x + r, y, r).unwrap_throw();
    ctx.close_path();
    ctx.stroke();
}

pub fn draw_rounded_rect_with_title(rect: &Rect, msg: &str, color: &str) {
    draw_rounded_rect(&rect, color);
    draw_text_oneline_center(&rect, msg);
}

pub fn get_font_normal() -> String {
    format!("{}px Arial", 16)
}

pub fn get_font_small() -> String {
    format!("{}px Arial", 14)
}

pub fn set_font_normal() {
    get_canvas_ctx().set_font(&get_font_normal());
}

pub fn set_font_small() {
    get_canvas_ctx().set_font(&get_font_small());
}

pub fn get_text_metric(t: &str) -> (f64, f64) {
    let ctx = get_canvas_ctx();
    let metrics = ctx.measure_text(t).unwrap_throw();
    let h = metrics.actual_bounding_box_ascent() + metrics.actual_bounding_box_descent();
    (metrics.width(), h)
}

pub fn get_text_ascent(t: &str) -> f64 {
    get_canvas_ctx().measure_text(t).unwrap_throw().actual_bounding_box_ascent()
}

pub fn get_text_descent(t: &str) -> f64 {
    get_canvas_ctx().measure_text(t).unwrap_throw().actual_bounding_box_descent()
}

// draw multiline text in center, with respect to the top line
pub fn draw_paragraph_vec(rect: &Rect, p: Vec<String>) {
    draw_paragraph(rect, &p.join("\n"));
}

pub fn draw_paragraph(rect: &Rect, t: &str) {
    let ctx = get_canvas_ctx();
    ctx.set_font(&get_font_normal());
    let org_text_align = ctx.text_align();
    ctx.set_text_align("center");

    let tm: Vec<_> = t.lines().clone().map(
        |line| get_text_metric(line)
    ).collect();

    let maxw = tm.iter().map(|(w, _)| *w).fold(f64::NAN, f64::max);
    if maxw > rect.w {
        warn!("Try to draw text with width {} inside rect with width {}", maxw, rect.w);
    }

    let h = tm.iter().map(|(_, h)| *h).collect::<Vec<_>>();

    let mut yoff = get_text_ascent(t.lines().next().unwrap_throw());
    for (line, h) in t.lines().zip(h) {
        if rect.h < yoff {
            warn!("Try to draw text with height {} inside rect with height {}", yoff, rect.h);
        }
        ctx.fill_text(line, rect.get_x_center(), rect.y + yoff).unwrap_throw();
        yoff += h;
        yoff += 3f64;
    }

    ctx.set_text_align(&org_text_align);
}

pub fn draw_text_oneline_center(rect: &Rect, t: &str) {
    let (w, h) = get_text_metric(t);
    if w > rect.w {
        warn!("draw text oneline center of width {} within {}", w, rect.w);
    }
    if h > rect.h {
        warn!("draw text oneline center of height {} within {}", h, rect.h);
    }
    draw_text_oneline(&rect.center_cut(Fixed(w), Fixed(h)), t);
}

// draw text oneline with respect to the bottom line
pub fn draw_text_oneline(rect: &Rect, t: &str) {
    draw_text_oneline_with_descent(rect, t, get_text_descent(t));
}

pub fn draw_text_oneline_center_color(rect: &Rect, t: &str, color: &str) {
    let ctx = get_canvas_ctx();
    let old = ctx.fill_style();
    ctx.set_fill_style_str(color);
    draw_text_oneline_center(rect, t);
    #[allow(deprecated)]
    ctx.set_fill_style(&old);
}

pub fn draw_text_oneline_with_descent(rect: &Rect, t: &str, descent: f64) {
    // draw_rect(rect, BORDER_NORMAL);
    let w = get_text_metric(t).0;
    if w > rect.w {
        warn!("Try to draw text with width {} inside rect with width {}", w, rect.w);
    }
    get_canvas_ctx().fill_text(t, rect.x, rect.y + rect.h - descent).unwrap_throw();
}

pub fn get_ascii_max_descent() -> f64 {
    get_text_descent("1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
}

pub fn draw_image(rect: &Rect, img: &str) {
    get_canvas_ctx().draw_image_with_html_image_element_and_dw_and_dh(
        &get_image(img),
        rect.x,
        rect.y,
        rect.w,
        rect.h,
    ).unwrap_throw();
}
