use wasm_bindgen::prelude::wasm_bindgen;

use crate::bubble_sort::bubble_sort;
use crate::heap_sort::heap_sort;
use crate::insertion_sort::insertion_sort;
use crate::merge_sort::merge_sort;
use crate::quick_sort::quick_sort;
use crate::selection_sort::selection_sort;

use crate::models::Step;

#[wasm_bindgen]
pub enum Algorithm {
    BubbleSort,
    InsertionSort,
    SelectionSort,
    HeapSort,
    QuickSort,
    MergeSort,
}

#[wasm_bindgen]
pub fn sort(arr: Vec<i32>, algorithm: Algorithm) -> Vec<Step> {
    let mut arr = arr.clone();

    match algorithm {
        Algorithm::BubbleSort => bubble_sort(&mut arr),
        Algorithm::InsertionSort => insertion_sort(&mut arr),
        Algorithm::SelectionSort => selection_sort(&mut arr),
        Algorithm::MergeSort => merge_sort(&mut arr),
        Algorithm::HeapSort => heap_sort(&mut arr),
        Algorithm::QuickSort => quick_sort(&mut arr),
    }
}
