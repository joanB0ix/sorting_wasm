use crate::{models::Step, snapshot_handler::SnapshotHandler};

pub fn insertion_sort<S>(arr: &mut Vec<i32>, mut snapshot_handler: S) -> Vec<Step>
where
    S: SnapshotHandler,
{
    if arr.is_empty() {
        return snapshot_handler.finish();
    }

    let len = arr.len();

    for i in 0..len {
        let mut j = i;
        while j > 0 {
            snapshot_handler.record(arr, j, j - 1);

            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                snapshot_handler.record(arr, j - 1, j);
                j -= 1;
            } else {
                break;
            }
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
    fn test_insertion_sort_empty_array_returns_no_steps() {
        let mut arr: Vec<i32> = vec![];
        let steps = insertion_sort(&mut arr, FullSnapshot::new());

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_insertion_sort_unsorted_array_returns_steps() {
        let mut arr = vec![3, 1, 2];
        let steps = insertion_sort(&mut arr, FullSnapshot::new());

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
