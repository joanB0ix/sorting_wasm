use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Step {
    array: Vec<i32>,
    index_a: usize,
    index_b: usize,
}

#[wasm_bindgen]
impl Step {
    #[wasm_bindgen(constructor)]
    pub fn new(array: Vec<i32>, index_a: usize, index_b: usize) -> Self {
        Step {
            array,
            index_a,
            index_b,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn array(&self) -> Vec<i32> {
        self.array.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn index_a(&self) -> usize {
        self.index_a
    }

    #[wasm_bindgen(getter)]
    pub fn index_b(&self) -> usize {
        self.index_b
    }
}
