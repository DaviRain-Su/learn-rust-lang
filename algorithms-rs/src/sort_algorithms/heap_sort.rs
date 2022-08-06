use std::fmt::{Debug, Display};

fn parent(i: usize) -> usize {
    i / 2
}

fn left(i: usize) -> usize {
    ((i + 1) << 1) - 1
}

fn right(i: usize) -> usize {
    (i + 1) << 1
}

/// Heap 堆
#[derive(Debug)]
pub struct Heap<T> {
    /// heap data
    data: Vec<T>,
    /// heap size
    size: usize,
}

impl<T: Clone + PartialOrd + Default + Display + Debug> Heap<T> {
    pub fn from_vector(array: &[T]) -> Self {
        Self {
            data: array.into(),
            size: array.len() - 1,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn max_heapify(&mut self, index: usize) {
        let mut largest = index;
        let left = left(index);
        let right = right(index);

        if left <= self.len() && self.data.get(largest) < self.data.get(left) {
            largest = left;
        }

        if right <= self.len() && self.data.get(largest) < self.data.get(right) {
            largest = right;
        }

        if largest != index {
            self.data.swap(index, largest);
            self.max_heapify(largest);
        }
    }

    pub fn min_heapify(&mut self, index: usize) {
        let mut min = index;
        let left = left(index);
        let right = right(index);

        if left <= self.len() && self.data.get(min) > self.data.get(left) {
            min = left;
        }

        if right <= self.len() && self.data.get(min) > self.data.get(right) {
            min = right;
        }

        if min != index {
            self.data.swap(index, min);
            self.min_heapify(min);
        }
    }

    pub fn build_max_heap(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.max_heapify(index);
        }
    }

    pub fn build_min_heap(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.min_heapify(index);
        }
    }

    pub fn heap_sort(&mut self) {
        self.build_max_heap();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size = self.size - 1;
            self.max_heapify(0);
        }
    }

    pub fn heap_sort_by_min_heap(&mut self) {
        self.build_min_heap();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size = self.size - 1;
            self.min_heapify(0);
        }
    }
}

#[test]
fn test_replace() {
    let mut vec_temp = vec![1, 2, 3];

    vec_temp.swap(0, 1);
    println!("vector = {:?}", vec_temp);
}

#[test]
fn test_build_max_heap() {
    let mut temp_heap = Heap::from_vector(&vec![5, 3, 7, 9, 10, 23, 45, 23, 12, 23, 0, 12, 32]);
    println!("temp Heap = {:?}", temp_heap);

    temp_heap.heap_sort();

    println!("temp Heap = {:?}", temp_heap);
}

#[test]
fn test_build_min_heap() {
    let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 0, 23, 34, 56, 11, 230, 12]);

    println!("min_heap = {:?}", min_heap);

    min_heap.build_min_heap();

    min_heap.heap_sort_by_min_heap();

    println!("min_heap = {:?}", min_heap);
}
