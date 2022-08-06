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
    /// 从向量中构造堆
    pub fn from_vector(array: &[T]) -> Self {
        Self {
            data: array.into(),
            size: array.len() - 1,
        }
    }

    /// 当前堆的数据的大小
    pub fn len(&self) -> usize {
        self.size
    }

    /// 大根堆调整
    pub fn max_heapify(&mut self, index: usize) {
        // setting largest is index
        let mut largest = index;
        let left = left(index);
        let right = right(index);

        // if left > largest then larget = left
        if left <= self.len() && self.data.get(largest) < self.data.get(left) {
            largest = left;
        }

        // if right > largest then largest = right
        if right <= self.len() && self.data.get(largest) < self.data.get(right) {
            largest = right;
        }

        if largest != index {
            // swap vector index , largest value
            self.data.swap(index, largest);
            // rec call max_heapify
            self.max_heapify(largest);
        }
    }

    /// 小根堆调整
    pub fn min_heapify(&mut self, index: usize) {
        // setting min is index
        let mut min = index;
        let left = left(index);
        let right = right(index);

        // if min > left then min = left
        if left <= self.len() && self.data.get(min) > self.data.get(left) {
            min = left;
        }

        // if min > right then min = right
        if right <= self.len() && self.data.get(min) > self.data.get(right) {
            min = right;
        }

        if min != index {
            // swap vector index, min value
            self.data.swap(index, min);
            // rec call min_heapify
            self.min_heapify(min);
        }
    }

    /// 构造大根堆
    pub fn build_max_heap(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.max_heapify(index);
        }
    }

    /// 构造小根堆
    pub fn build_min_heap(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.min_heapify(index);
        }
    }

    /// 升序排序，根据大根堆
    pub fn heap_sort(&mut self) {
        self.build_max_heap();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size = self.size - 1;
            self.max_heapify(0);
        }
    }

    /// 降序排序，根据小根堆
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
