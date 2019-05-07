mod utils;
mod swirlr;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use swirlr::*;
use web_sys::*;
use utils::set_panic_hook;
use image;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}

fn log(value: &str) {
    console::log_1(&JsValue::from_str(value));
}

#[wasm_bindgen]
pub fn render() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let svg_path = document.get_element_by_id("outputpath").unwrap();

    let data = context.get_image_data(0.0, 0.0, 512.0, 512.0).unwrap().data();

    let source = image::RgbaImage::from_raw(512, 512, data.to_vec()).unwrap();

    let path_data = swirl(&source);

    svg_path.set_attribute("d", path_data.as_str());
}

