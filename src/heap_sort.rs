use crate::{models::Step, snapshot_handler::SnapshotHandler};

pub fn heap_sort<S>(arr: &mut Vec<i32>, mut snapshot_handler: S) -> Vec<Step>
where
    S: SnapshotHandler,
{
    if arr.is_empty() {
        return snapshot_handler.finish();
    }

    let len = arr.len();

    for i in (0..len / 2).rev() {
        heapify(arr, len, i, &mut snapshot_handler);
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        snapshot_handler.record_swap(arr, 0, i);
        heapify(arr, i, 0, &mut snapshot_handler);
    }

    snapshot_handler.finish()
}

fn heapify<S>(arr: &mut Vec<i32>, heap_size: usize, root: usize, snapshot_handler: &mut S)
where
    S: SnapshotHandler,
{
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
        snapshot_handler.record_swap(arr, root, largest);
        heapify(arr, heap_size, largest, snapshot_handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_handler::FullSnapshot;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_heap_sort_empty_array_returns_no_steps() {
        let mut arr: Vec<i32> = vec![];
        let steps = heap_sort(&mut arr, FullSnapshot::new());

        assert_eq!(steps.len(), 0, "Expected 0 steps for an empty array");
    }

    #[test]
    fn test_heap_sort_unsorted_array_returns_steps() {
        let mut arr = vec![3, 1, 2];
        let steps = heap_sort(&mut arr, FullSnapshot::new());

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
