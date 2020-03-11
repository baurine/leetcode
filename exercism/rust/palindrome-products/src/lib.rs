use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    products: Vec<(u64, u64)>,
    val: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            products: vec![(a, b)],
            val: a * b,
        }
    }

    pub fn value(&self) -> u64 {
        self.val
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        if a * b == self.val {
            self.products.push((a, b))
        }
    }
}

fn is_palindrome(n: u64) -> bool {
    n.to_string().chars().rev().collect::<String>() == n.to_string()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    // cache
    let mut num_palindrome: HashMap<u64, bool> = HashMap::new();

    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;
    for i in min..=max {
        for j in i..=max {
            let v = i * j;

            let palindrome = match num_palindrome.get(&v) {
                None => {
                    let b = is_palindrome(v);
                    num_palindrome.insert(v, b);
                    b
                }
                Some(b) => *b,
            };
            if !palindrome {
                continue;
            }

            match &mut min_palindrome {
                None => min_palindrome = Some(Palindrome::new(i, j)),
                Some(ref p) if v < p.value() => min_palindrome = Some(Palindrome::new(i, j)),
                Some(ref mut p) if v == p.value() => p.insert(i, j),
                _ => (),
            }
            match &mut max_palindrome {
                None => max_palindrome = Some(Palindrome::new(i, j)),
                Some(ref p) if v > p.value() => max_palindrome = Some(Palindrome::new(i, j)),
                Some(ref mut p) if v == p.value() => p.insert(i, j),
                _ => (),
            }
        }
    }

    match (min_palindrome, max_palindrome) {
        (Some(min), Some(max)) => Some((min, max)),
        (_, _) => None,
    }
}
