// 秩
type Rank = usize;
// 默认的初始化容量
const DEFAULT_CAPACITY: usize = 3;

#[derive(Debug, Clone)]
pub struct Vector<T> {
    // 规模
    size: Rank,
    // 容量
    capacity: usize,
    // 数据区域
    elem: Vec<T>,
}

impl<T: Clone> Vector<T> {
    /// constructor function
    pub fn new(capacity: usize, size: usize, v: T) -> Self {
        Self { 
            size, 
            capacity,
            elem: vec![v; size],
        }
    }

    pub fn from_array_all_range(array: &[T], n: Rank) -> Self {
        todo!()
    }

    pub fn from_array_range(array: &[T], lo: Rank, hi: Rank) -> Self {
        todo!()
    }

    pub fn from_vector_all_range(array: &Vector<T>, n: Rank) -> Self {
        todo!()
    }

    pub fn from_vector_range(array: &Vector<T>, lo: Rank, hi: Rank) -> Self {
        todo!()
    }

    pub fn size(&self) -> Rank { 
        self.size
    }

    pub fn is_empty(&self) -> bool { 
        self.size == 0 
    }

    pub fn disordered(&self) -> usize { 
        todo!()
    }

    pub fn find(&self, e: &T) -> Rank { 
        self.find_of_range(e, 0, self.size)
    }

    pub fn find_of_range(&self, e: &T, lo: Rank, hi: Rank) -> Rank {
        todo!()
    }

    pub fn search(&self, e: &T) -> Option<Rank> { 
        if self.size <= 0 {
            None
        } else {
            Some(self.search_of_range(e, 0, self.size))
        }
    }


    pub fn remove(&mut self, r: Rank) -> T {
        todo!()
    }

    pub fn remove_of_range(&mut self, lo: Rank, hi: Rank) -> usize {
        todo!()
    }

    pub fn insert(&mut self, r: Rank, e: &T) -> Rank {
        todo!()
    }

    pub fn insert_last(&mut self, e: &T) -> Rank {
        todo!()
    }

    pub fn sort_of_range(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }

    pub fn sort(&mut self) {
        todo!()
    }


    pub fn unsort_of_range(&mut self,lo: Rank, hi: Rank) {
        todo!()
    }

    pub fn unsort(&mut self) {
        todo!()
    }

    pub fn deduplicate(&mut self) -> usize {
        todo!()
    }
    
    pub fn uniquify(&mut self) -> usize {
        todo!()
    }


    pub fn search_of_range(&self, e: &T, lo: Rank, hi: Rank) -> Rank {
        todo!()
    }
    
    fn copy_from(&mut self, array: &[T], lo: Rank, hi: Rank) {
        todo!()
    }

    fn expand(&mut self) {
        todo!()
    }

    fn shrink(&mut self) {
        todo!()
    }

    fn bubble(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }

    fn bubble_sort(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }

    fn max(&self, lo: Rank, hi: Rank) -> Rank {
        todo!()
    }

    fn selection_sort(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }

    fn merge(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }

    fn merge_sort(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }

    fn partition(&mut self, lo: Rank, hi: Rank) -> Rank { 
        todo!()
    }

    fn quick_sort(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }

    fn heap_sort(&mut self, lo: Rank, hi: Rank) {
        todo!()
    }


}



#[cfg(test)]
mod tests {

}