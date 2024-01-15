// #![allow(dead_code)]

pub mod app;
pub mod draw;
pub mod vertex;
pub mod geometry;

use app::AppBuilder;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run(){
    AppBuilder::new().run();
}

pub fn app<M>(model: app::ModelFn<M>) -> app::AppBuilder<M> where M: 'static{
    AppBuilder::app(model)
}
