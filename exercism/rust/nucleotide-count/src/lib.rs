use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide == 'X' || dna.contains('X') {
        return Err('X');
    }
    let count = dna.chars().filter(|c| c == &nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if dna.contains('X') {
        return Err('X');
    }
    let mut char_count = HashMap::new();
    "ATGC".chars().for_each(|c| {
        char_count.insert(c, 0);
    });
    dna.chars().for_each(|c| {
        // or_insert() 方法事实上会返回这个键的值的一个可变引用（&mut V）
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    });
    Ok(char_count)
}
