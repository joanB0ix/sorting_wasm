use crate::{models::Step, snapshot_handler::SnapshotHandler};

pub fn selection_sort<S>(arr: &mut Vec<i32>, mut snapshot_handler: S) -> Vec<Step>
where
    S: SnapshotHandler,
{
    if arr.is_empty() {
        return snapshot_handler.finish();
    }

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
            snapshot_handler.record_swap(arr, i, min_index);
        }
    }

    snapshot_handler.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_handler::FullSnapshot;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_selection_sort_empty_array_returns_no_steps() {
        let mut arr: Vec<i32> = vec![];
        let steps = selection_sort(&mut arr, FullSnapshot::new());

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_selection_sort_sorted_array_returns_no_steps() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let steps = selection_sort(&mut arr, FullSnapshot::new());

        assert_eq!(
            steps.len(),
            0,
            "Expected 0 steps for an already sorted array"
        );
    }

    #[test]
    fn test_selection_sort_unsorted_array_returns_steps() {
        let mut arr = vec![3, 1, 2];
        let steps = selection_sort(&mut arr, FullSnapshot::new());

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
