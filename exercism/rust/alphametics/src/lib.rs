use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // isize represents the factor
    // bool represents whether this char is leading so it can't be 0
    let mut chars_map: HashMap<char, (isize, bool)> = HashMap::new();
    let mut chars_vec: Vec<char> = vec![];

    let equation_vec: Vec<&str> = input.split(" == ").collect();
    let left_statement_items: Vec<&str> = equation_vec[0].split(" + ").collect();
    let right_statement = equation_vec[1];

    // 将原始表达式转换成计算表达式
    // 比如: I + BB == ILL
    // 转换成: 1I + 10B + 1B == 100I + 10L + 1L
    // 将右边的移到左边，系数乘以 -1，得到: -99I + 11B + 11L = 0
    // 此时 chars_map 的值：{I: (-99, true), B: (11, true), L: (11, false)}
    left_statement_items.iter().for_each(|item| {
        let char_count = item.chars().count();
        for (i, c) in item.chars().enumerate() {
            let digit = 10usize.pow((char_count - i - 1) as u32);
            let char_digits = chars_map.entry(c).or_insert((0, false));
            char_digits.0 += digit as isize;
            if i == 0 {
                char_digits.1 = true;
            }
            if !chars_vec.contains(&c) {
                chars_vec.push(c);
            }
        }
    });

    // TODO: polish
    {
        let item = right_statement;
        let char_count = item.chars().count();
        for (i, c) in item.chars().enumerate() {
            let digit = 10usize.pow((char_count - i - 1) as u32);
            let char_digits = chars_map.entry(c).or_insert((0, false));
            char_digits.0 -= digit as isize;
            if i == 0 {
                char_digits.1 = true;
            }
            if !chars_vec.contains(&c) {
                chars_vec.push(c);
            }
        }
    }
    println!("{:?}", chars_map);
    println!("{:?}", chars_vec);

    // 第一次尝试，当 chars_vec.len() == 10 时，耗时太长
    // for i in (10usize.pow((chars_vec.len() - 1) as u32))..10usize.pow(chars_vec.len() as u32) {
    //     let mut bits = vec![0; chars_map.len()];
    // }

    // 方案 2: permutation (排列)
    // 上面得到数学表达式: -99I + 11B + 11L = 0
    // 将 0~9 进行排列组合，然后看哪一个组合可以让上面的表达式成立
    for perm in (0u8..10).permutations(chars_vec.len()) {
        let mut sum = 0;
        for i in 0..chars_vec.len() {
            sum += 1; // trick
            let value = perm[i];
            let c = chars_vec[i];
            let (factor, leading) = chars_map.get(&c).unwrap();
            // 首位字母不能是 0
            if *leading && value == 0 {
                break;
            }
            sum -= 1;
            sum += (*factor) * (value as isize);
        }
        if sum == 0 {
            let mut ret_map = HashMap::new();
            for i in 0..chars_vec.len() {
                let value = perm[i];
                let c = chars_vec[i];
                ret_map.insert(c, value);
            }
            return Some(ret_map);
        }
    }

    None
}
