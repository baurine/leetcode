pub fn build_proverb(list: &[&str]) -> String {
  if list.len() == 0 {
    return "".to_string();
  }
  let mut v = vec![];
  for i in 0..list.len() - 1 {
    v.push(format!(
      "For want of a {} the {} was lost.",
      list[i],
      list[i + 1]
    ));
  }
  v.push(format!("And all for the want of a {}.", list[0]));
  v.join("\n")
}
