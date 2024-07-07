pub mod diagnostics;
pub mod parse;
pub mod span;
pub mod symbol;

use wasm_bindgen::prelude::*;

/// Represents a compilation result of a single shader pack.
#[derive(Debug, Clone, Hash)]
#[wasm_bindgen]
pub struct Compiled {
    errors: Vec<String>,
}

#[wasm_bindgen]
impl Compiled {
    /// Returns the errors that occurred during compilation.
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}

/// Compiles a shader pack from source code.
#[wasm_bindgen]
pub fn compile_shader_pack(source: &str) -> Compiled {
    Compiled { errors: vec![] }
}
