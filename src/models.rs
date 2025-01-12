use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Step {
    index_a: usize,
    index_b: usize,

    snapshot: Vec<i32>,
}

#[wasm_bindgen]
impl Step {
    #[wasm_bindgen(constructor)]
    pub fn new(index_a: usize, index_b: usize, snapshot: Vec<i32>) -> Self {
        Step {
            index_a,
            index_b,
            snapshot,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn snapshot(&self) -> Vec<i32> {
        self.snapshot.clone()
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
