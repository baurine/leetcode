pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut start: i32 = 0;
    let mut end: i32 = (array.len() - 1) as i32;
    while start <= end {
        let middle_idx = (start + end) / 2;
        let value = array.get(middle_idx as usize).unwrap();
        if value == &key {
            return Some(middle_idx as usize);
        }
        if value < &key {
            start = middle_idx + 1;
        } else {
            end = middle_idx - 1;
        }
    }
    None
}
