use wasm_bindgen::prelude::wasm_bindgen;

use crate::models::Step;

#[wasm_bindgen]
pub fn quick_sort(arr: Vec<i32>) -> Vec<Step> {
    let mut arr = arr.clone();
    let mut steps = vec![];
    let len = arr.len();
    quick_sort_helper(&mut arr, 0, len - 1, &mut steps);
    steps
}

fn quick_sort_helper(arr: &mut Vec<i32>, low: usize, high: usize, steps: &mut Vec<Step>) {
    if low < high {
        let pivot_index = partition(arr, low, high, steps);
        if pivot_index > 0 {
            quick_sort_helper(arr, low, pivot_index - 1, steps);
        }
        quick_sort_helper(arr, pivot_index + 1, high, steps);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize, steps: &mut Vec<Step>) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            steps.push(Step::new(arr.clone(), i, j));
            i += 1;
        }
    }

    arr.swap(i, high);
    steps.push(Step::new(arr.clone(), i, high));
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_quick_sort_unsorted_array_returns_steps() {
        let arr = vec![3, 1, 2];
        let steps = quick_sort(arr.clone());

        assert!(
            !steps.is_empty(),
            "Expected some steps for an unsorted array"
        );

        let last_step = steps.last().unwrap();
        assert!(
            is_sorted(&last_step.array()),
            "Expected final array to be sorted"
        );
    }
}
