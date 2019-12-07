/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

pub fn solution() -> u32 {
  let mut abc_product = 1;
  'outer: for a in 1..=333 {
    for b in (a + 1)..=500 {
      let c = 1000 - a - b;
      if (c > b) && (a * a + b * b == c * c) {
        println!("a: {}, b: {}, c: {}", a, b, c); // a: 200, b: 375, c: 425
        abc_product = a * b * c;
        break 'outer;
      }
    }
  }
  abc_product
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_1() {
    assert_eq!(solution(), 31875000);
  }
}
