pub mod search {
    pub fn binary_search<T: core::cmp::PartialOrd>(arrays: &[T], key: T) -> Option<usize> {
        // assert arrays is sort
        let mut lo = 0usize;
        let mut hi = arrays.len() - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if key < arrays[mid] {
                hi = mid - 1;
            } else if key > arrays[mid] {
                lo = mid + 1;
            } else {
                return Some(mid);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::search::binary_search;

    #[test]
    fn test_binary_search() {
        let arrays = vec![1, 2, 3, 6, 8, 9, 12, 45, 45, 67];
        let key = 8;
        assert_eq!(Some(4usize), binary_search(&arrays, key));
    }
}
