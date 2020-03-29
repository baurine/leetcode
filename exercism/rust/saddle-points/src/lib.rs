pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    input.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, &value)| {
            if row.iter().all(|&x| value >= x) && input.iter().all(|x| value <= x[col_idx]) {
                result.push((row_idx, col_idx))
            }
        })
    });
    result
}
