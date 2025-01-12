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

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_bubble_sort_empty_array_returns_no_steps() {
        let arr = vec![];
        let steps = bubble_sort(arr);

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_bubble_sort_sorted_array_returns_no_steps() {
        let arr = vec![1, 2, 3, 4, 5];
        let steps = bubble_sort(arr.clone());

        assert_eq!(
            steps.len(),
            0,
            "Expected 0 steps for an already sorted array"
        );
    }

    #[test]
    fn test_bubble_sort_unsorted_array_returns_steps() {
        let arr = vec![3, 1, 2];
        let steps = bubble_sort(arr.clone());

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
