pub fn raindrops(n: u32) -> String {
  let sounds = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];

  let s = sounds
    .iter()
    .filter(|item| n % item.0 == 0)
    .map(|item| item.1.to_string())
    .collect::<String>();

  if s.len() == 0 {
    n.to_string()
  } else {
    s
  }
}
