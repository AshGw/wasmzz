use img_watermarker::watermark;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_watermarker(){
    watermark();
}
