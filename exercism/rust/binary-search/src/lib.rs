use std::cmp::Ordering;

fn find_1(array: &[i32], key: i32) -> Option<usize> {
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

fn find_2(array: &[i32], key: i32) -> Option<usize> {
    let mut s = &array[..];
    let mut idx_base = 0;
    while !s.is_empty() {
        let middle_idx = s.len() / 2;
        // let value = s.get(middle_idx).unwrap();
        let value = s[middle_idx];
        if value == key {
            return Some(idx_base + middle_idx);
        }
        if value < key {
            s = &s[middle_idx + 1..];
            idx_base += middle_idx + 1;
        } else {
            s = &s[..middle_idx];
        }
    }
    None
}

fn find_3<T: Ord>(array: &[T], key: T) -> Option<usize> {
    let mut s = &array[..];
    let mut idx_base = 0;
    while !s.is_empty() {
        let middle_idx = s.len() / 2;
        let value = s.get(middle_idx).unwrap();
        if value == &key {
            return Some(idx_base + middle_idx);
        }
        if value < &key {
            s = &s[middle_idx + 1..];
            idx_base += middle_idx + 1;
        } else {
            s = &s[..middle_idx];
        }
    }
    None
}

pub fn find_4(array: &[i32], key: i32) -> Option<usize> {
    find_1(array, key);
    find_2(array, key);
    find_3(array, key)
}

pub fn find<R: AsRef<[T]>, T: Ord>(array: R, key: T) -> Option<usize> {
    let s = array.as_ref();
    if s.is_empty() {
        return None;
    }
    let middle = s.len() / 2;
    // right include middle element
    let (left, right) = s.split_at(middle);
    match key.cmp(s.get(middle).unwrap()) {
        Ordering::Equal => Some(middle),
        Ordering::Less => find(left, key),
        Ordering::Greater => find(&right[1..], key).map(|x| x + middle + 1),
    }
}
