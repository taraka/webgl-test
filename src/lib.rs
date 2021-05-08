extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello() {
    log("Hello from rust");
}

#[wasm_bindgen]
pub struct GameClient {

}

#[wasm_bindgen]
impl GameClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("Update");
        Ok(())
    }

    pub fn render(&self) {
        
    }
}
