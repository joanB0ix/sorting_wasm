use wasm_bindgen::prelude::wasm_bindgen;

use crate::bubble_sort::bubble_sort;
use crate::heap_sort::heap_sort;
use crate::insertion_sort::insertion_sort;
use crate::merge_sort::merge_sort;
use crate::quick_sort::quick_sort;
use crate::selection_sort::selection_sort;

use crate::models::Step;
use crate::snapshot_handler::{FullSnapshot, IndicesOnly};

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
pub enum Snapshot {
    Full,
    Indices,
}

#[wasm_bindgen]
pub fn sort(arr: Vec<i32>, algorithm: Algorithm, snapshot: Snapshot) -> Vec<Step> {
    let mut arr = arr.clone();

    match snapshot {
        Snapshot::Full => {
            let handler = FullSnapshot::new();
            match algorithm {
                Algorithm::BubbleSort => bubble_sort(&mut arr, handler),
                Algorithm::InsertionSort => insertion_sort(&mut arr, handler),
                Algorithm::SelectionSort => selection_sort(&mut arr, handler),
                Algorithm::MergeSort => merge_sort(&mut arr, handler),
                Algorithm::HeapSort => heap_sort(&mut arr, handler),
                Algorithm::QuickSort => quick_sort(&mut arr, handler),
            }
        }
        Snapshot::Indices => {
            let handler = IndicesOnly::new();
            match algorithm {
                Algorithm::BubbleSort => bubble_sort(&mut arr, handler),
                Algorithm::InsertionSort => insertion_sort(&mut arr, handler),
                Algorithm::SelectionSort => selection_sort(&mut arr, handler),
                Algorithm::MergeSort => merge_sort(&mut arr, handler),
                Algorithm::HeapSort => heap_sort(&mut arr, handler),
                Algorithm::QuickSort => quick_sort(&mut arr, handler),
            }
        }
    }
}
