use std::vec;

use crate::models::Step;

pub fn merge_sort(arr: &mut Vec<i32>) -> Vec<Step> {
    let mut steps = vec![];
    let len = arr.len();
    merge_sort_helper(arr, 0, len, &mut steps);
    steps
}

fn merge_sort_helper(arr: &mut Vec<i32>, left: usize, right: usize, steps: &mut Vec<Step>) {
    if right - left > 1 {
        let mid = left + (right - left) / 2;

        merge_sort_helper(arr, left, mid, steps);
        merge_sort_helper(arr, mid, right, steps);

        merge(arr, left, mid, right, steps);
    }
}

fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize, steps: &mut Vec<Step>) {
    let mut temp = vec![];
    let (mut i, mut j) = (left, mid);

    while i < mid && j < right {
        if arr[i] <= arr[j] {
            temp.push(arr[i]);
            i += 1;
        } else {
            temp.push(arr[j]);
            j += 1;
        }
    }

    while i < mid {
        temp.push(arr[i]);
        i += 1;
    }

    while j < right {
        temp.push(arr[j]);
        j += 1;
    }

    for (k, &val) in temp.iter().enumerate() {
        arr[left + k] = val;
    }

    steps.push(Step::new(arr.clone(), left, right));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_merge_sort_empty_array_returns_no_steps() {
        let mut arr: Vec<i32> = vec![];
        let steps = merge_sort(&mut arr);

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_merge_sort_unsorted_array_returns_steps() {
        let mut arr: Vec<i32> = vec![3, 1, 2];
        let steps = merge_sort(&mut arr);

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
