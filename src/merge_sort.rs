use std::vec;

use crate::{models::Step, snapshot_handler::SnapshotHandler};

pub fn merge_sort<S>(arr: &mut Vec<i32>, mut snapshot_handler: S) -> Vec<Step>
where
    S: SnapshotHandler,
{
    if arr.is_empty() {
        return snapshot_handler.finish();
    }

    let len = arr.len();
    merge_sort_helper(arr, 0, len, &mut snapshot_handler);

    snapshot_handler.finish()
}

fn merge_sort_helper<S>(arr: &mut Vec<i32>, left: usize, right: usize, snapshot_handler: &mut S)
where
    S: SnapshotHandler,
{
    if right - left > 1 {
        let mid = left + (right - left) / 2;

        merge_sort_helper(arr, left, mid, snapshot_handler);
        merge_sort_helper(arr, mid, right, snapshot_handler);

        merge(arr, left, mid, right, snapshot_handler);
    }
}

fn merge<S>(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize, snapshot_handler: &mut S)
where
    S: SnapshotHandler,
{
    let mut temp = vec![];
    let (mut i, mut j) = (left, mid);

    while i < mid && j < right {
        snapshot_handler.record(arr, i, j);

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
        snapshot_handler.record(arr, left, left + k);
    }

    snapshot_handler.record(arr, left, right);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_handler::FullSnapshot;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_merge_sort_empty_array_returns_no_steps() {
        let mut arr: Vec<i32> = vec![];
        let steps = merge_sort(&mut arr, FullSnapshot::new());

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_merge_sort_unsorted_array_returns_steps() {
        let mut arr: Vec<i32> = vec![3, 1, 2];
        let steps = merge_sort(&mut arr, FullSnapshot::new());

        assert!(
            !steps.is_empty(),
            "Expected some steps for an unsorted array"
        );

        let last_step = steps.last().unwrap();
        assert!(
            is_sorted(&last_step.snapshot()),
            "Expected final array to be sorted"
        );
    }
}
