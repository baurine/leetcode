fn beverge(n: u32) -> String {
  match n {
    0 => "No more bottles".to_string(),
    1 => "1 bottle".to_string(),
    _ => format!("{} bottles", n),
  }
}

fn remain_beverge(n: u32) -> String {
  match n {
    0 => beverge(99),
    _ => beverge(n - 1),
  }
}

fn action(n: u32) -> String {
  match n {
    0 => "Go to the store and buy some more",
    1 => "Take it down and pass it around",
    _ => "Take one down and pass it around",
  }
  .to_string()
}

pub fn verse(n: u32) -> String {
  format!(
    "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
    beverge(n),
    beverge(n).to_lowercase(),
    action(n),
    remain_beverge(n).to_lowercase()
  )
}

pub fn sing(start: u32, end: u32) -> String {
  (end..=start)
    .rev()
    .map(verse)
    .collect::<Vec<_>>()
    .join("\n")
}
