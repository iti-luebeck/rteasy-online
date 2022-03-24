use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Signals {
    pub(crate) condition_signals: Vec<String>,
    pub(crate) control_signals: Vec<String>,
}

#[wasm_bindgen]
impl Signals {
    pub fn condition_signals(&self) -> Vec<JsValue> {
        self.condition_signals.iter().map(Into::into).collect()
    }

    pub fn control_signals(&self) -> Vec<JsValue> {
        self.control_signals.iter().map(Into::into).collect()
    }
}
