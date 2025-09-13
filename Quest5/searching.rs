pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i, &value) in array.iter().enumerate().rev() {
        if value == key {
            return Some(i);
        }
    }
    None
}
