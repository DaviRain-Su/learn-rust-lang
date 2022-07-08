use std::fmt::{Debug, Display};

/// binary search algorithm
pub fn binary_search<T>(array: &[T], item: T) -> Option<usize>
where
    T: Debug + Display + PartialOrd,
{
    let mut low = 0usize;
    let mut hight = array.len() - 1;

    while low <= hight {
        let mid = low + (hight - low) / 2;
        if item == array[mid] {
            return Some(mid);
        } else if item < array[mid] {
            hight = mid - 1;
        } else if item > array[mid] {
            low = mid + 1;
        }
    }

    None
}

#[test]
fn test_binary_search() {
    use std::fmt::Formatter;

    // binary_search vector i32
    let temp_list = vec![1, 3, 5, 7, 9];

    assert_eq!(binary_search(&temp_list, 3), Some(1));

    // binary_search vector char
    let temp_list = vec!['a', 'b', 'c', 'd', 'e', 'f'];

    assert_eq!(binary_search(&temp_list, 'c'), Some(2));

    // binary_search vector &str
    let temp_list = vec!["a", "b", "c", "d", "e", "f"];

    assert_eq!(binary_search(&temp_list, "c"), Some(2));

    #[derive(PartialOrd, PartialEq, Debug)]
    struct Test {
        value: i64,
    }

    impl Display for Test {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Test({})", self.value)
        }
    }

    // binary_search custom struct Test`
    let temp_list = vec![
        Test { value: 1 },
        Test { value: 3 },
        Test { value: 5 },
        Test { value: 7 },
        Test { value: 9 },
    ];

    assert_eq!(binary_search(&temp_list, Test { value: 3 }), Some(1));
}
