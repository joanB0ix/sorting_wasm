use wasm_bindgen::prelude::wasm_bindgen;

use crate::models::Step;

#[wasm_bindgen]
pub fn bubble_sort(arr: Vec<i32>) -> Vec<Step> {
    let mut arr = arr.clone();
    let mut steps = vec![];
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                steps.push(Step::new(arr.clone(), j, j + 1));
            }
        }
    }
    steps
}
