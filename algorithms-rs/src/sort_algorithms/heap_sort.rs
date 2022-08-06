use std::fmt::Display;


fn parent(i: usize) -> usize {
    i  / 2
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
    pub data: Vec<T>,
}

impl<T: Clone + PartialOrd + Default + Display> Heap<T> {
    pub fn from_vector(array: &[T]) -> Self {
        Self { data: array.into() }
    }

    pub fn len(&self) -> usize { 
        self.data.len()
    }

    pub fn max_heapify(&mut self, index: usize) {
        let mut largest = 0usize;
        let left = left(index);
        let right = right(index);
        
        if left <= self.len() && self.data.get(index) < self.data.get(left) {
            largest = left;
        } else {
            largest = index;
        }

        if right <= self.len() && self.data.get(largest) < self.data.get(right) {
            largest = right;
        }
        
        if largest != index {
            self.data.swap(index, largest);
            self.max_heapify(largest);
        }
    }

    pub fn build_max_heap(&mut self) {
        for index in (0..(self.data.len()/2)).rev() {
            self.max_heapify(index);
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
    let mut temp_heap = Heap::from_vector(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1]);
    println!("temp Heap = {:?}", temp_heap);

    temp_heap.build_max_heap();

    println!("temp Heap = {:?}", temp_heap);
}