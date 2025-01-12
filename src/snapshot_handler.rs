use crate::models::Step;

pub trait SnapshotHandler {
    fn record_swap(&mut self, arr: &[i32], i: usize, j: usize);

    fn finish(self) -> Vec<Step>;
}

pub struct IndicesOnly {
    steps: Vec<Step>,
}

impl IndicesOnly {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
}

impl SnapshotHandler for IndicesOnly {
    fn record_swap(&mut self, arr: &[i32], i: usize, j: usize) {
        let snapshot = vec![arr[i], arr[j]];
        self.steps.push(Step::new(i, j, snapshot));
    }

    fn finish(self) -> Vec<Step> {
        self.steps
    }
}

pub struct FullSnapshot {
    steps: Vec<Step>,
}

impl FullSnapshot {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
}

impl SnapshotHandler for FullSnapshot {
    fn record_swap(&mut self, arr: &[i32], i: usize, j: usize) {
        self.steps.push(Step::new(i, j, arr.to_vec()));
    }

    fn finish(self) -> Vec<Step> {
        self.steps
    }
}
