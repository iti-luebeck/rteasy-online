use rt_easy::{memory_file::MemoryFile, vhdl::Ident};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vhdl(pub(crate) rt_easy::vhdl::Vhdl);

type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
impl Vhdl {
    pub fn memories(&self) -> Vec<JsValue> {
        let mut memories =
            self.0.declarations.memories.iter().map(|(ident, _, _)| &ident.0).collect::<Vec<_>>();
        memories.sort();

        memories.into_iter().map(Into::into).collect()
    }

    pub fn render(&self, module_name: String, is_debug: bool, memories_arg: Vec<JsValue>) -> Result<String> {
        let mut memories = HashMap::new();

        let mut memories_arg = memories_arg.into_iter();
        while let Some(name) = memories_arg.next() {
            let name = name.as_string().ok_or_else(|| JsValue::from_str("invalid args"))?;

            let file = memories_arg.next().ok_or_else(|| JsValue::from_str("invalid args"))?;
            let file = file.as_string().ok_or_else(|| JsValue::from_str("invalid args"))?;
            let file = MemoryFile::parse(&file)
                .map_err(|()| JsValue::from_str(&format!("Invalid memory file for `{}`", name)))?;

            memories.insert(Ident(name), file);
        }

        self.0.render(&module_name, is_debug, memories).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
