use wasm_bindgen::prelude::wasm_bindgen;

use crate::models::Step;

#[wasm_bindgen]
pub fn selection_sort(arr: Vec<i32>) -> Vec<Step> {
    let mut arr = arr.clone();
    let mut steps = vec![];
    let len = arr.len();

    for i in 0..len {
        let mut min_index = i;

        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
            steps.push(Step::new(arr.clone(), i, min_index));
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_selection_sort_empty_array_returns_no_steps() {
        let arr = vec![];
        let steps = selection_sort(arr);

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_selection_sort_sorted_array_returns_no_steps() {
        let arr = vec![1, 2, 3, 4, 5];
        let steps = selection_sort(arr.clone());

        assert_eq!(
            steps.len(),
            0,
            "Expected 0 steps for an already sorted array"
        );
    }

    #[test]
    fn test_selection_sort_unsorted_array_returns_steps() {
        let arr = vec![3, 1, 2];
        let steps = selection_sort(arr.clone());

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