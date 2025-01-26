use crate::{models::Step, snapshot_handler::SnapshotHandler};

pub fn quick_sort<S>(arr: &mut Vec<i32>, mut snapshot_handler: S) -> Vec<Step>
where
    S: SnapshotHandler,
{
    if arr.is_empty() {
        return snapshot_handler.finish();
    }

    let len = arr.len();

    quick_sort_helper(arr, 0, len - 1, &mut snapshot_handler);

    snapshot_handler.finish()
}

fn quick_sort_helper<S>(arr: &mut Vec<i32>, low: usize, high: usize, snapshot_handler: &mut S)
where
    S: SnapshotHandler,
{
    if low < high {
        let pivot_index = partition(arr, low, high, snapshot_handler);
        if pivot_index > 0 {
            quick_sort_helper(arr, low, pivot_index - 1, snapshot_handler);
        }
        quick_sort_helper(arr, pivot_index + 1, high, snapshot_handler);
    }
}

fn partition<S>(arr: &mut Vec<i32>, low: usize, high: usize, snapshot_handler: &mut S) -> usize
where
    S: SnapshotHandler,
{
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        snapshot_handler.record(arr, j, high);

        if arr[j] <= pivot {
            arr.swap(i, j);
            snapshot_handler.record(arr, i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    snapshot_handler.record(arr, i, high);

    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_handler::FullSnapshot;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_quick_sort_unsorted_array_returns_steps() {
        let mut arr = vec![3, 1, 2];
        let steps = quick_sort(&mut arr, FullSnapshot::new());

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
