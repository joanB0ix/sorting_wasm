use wasm_bindgen::prelude::wasm_bindgen;

use crate::models::Step;

#[wasm_bindgen]
pub fn heap_sort(arr: Vec<i32>) -> Vec<Step> {
    let mut arr = arr.clone();
    let mut steps = vec![];
    let len = arr.len();

    for i in (0..len / 2).rev() {
        heapify(&mut arr, len, i, &mut steps);
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        steps.push(Step::new(arr.clone(), 0, i));
        heapify(&mut arr, i, 0, &mut steps);
    }

    steps
}

fn heapify(arr: &mut Vec<i32>, heap_size: usize, root: usize, steps: &mut Vec<Step>) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }

    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != root {
        arr.swap(root, largest);
        steps.push(Step::new(arr.clone(), root, largest));
        heapify(arr, heap_size, largest, steps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_heap_sort_empty_array_returns_no_steps() {
        let arr = vec![];
        let steps = heap_sort(arr);

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_heap_sort_unsorted_array_returns_steps() {
        let arr = vec![3, 1, 2];
        let steps = heap_sort(arr.clone());

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
