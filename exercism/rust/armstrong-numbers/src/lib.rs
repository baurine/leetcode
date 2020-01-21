pub fn is_armstrong_number(num: u32) -> bool {
  let mut leaving = num;
  let mut bits = vec![];
  while leaving > 0 {
    bits.push(leaving % 10);
    leaving /= 10;
  }
  num
    == bits
      .iter()
      .map(|bit| bit.pow(bits.len() as u32))
      .sum::<u32>()
}
