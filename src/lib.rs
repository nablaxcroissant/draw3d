#![allow(dead_code)]

mod app;
use app::AppBuilder;
mod draw;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run(){
    AppBuilder::new().run().await;
}
