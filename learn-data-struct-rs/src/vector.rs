

#[cfg(test)]
mod tests {

    #[test]
    fn test_vec_ptr() {
        let temp = vec![1, 2, 3, 4];
        for i in 0..temp.len() {
            unsafe {
                println!("temp: [{}] PTR = {:?}", i, temp.as_ptr().add(i));
            }
        }
    }
}