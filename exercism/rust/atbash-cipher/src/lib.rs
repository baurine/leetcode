use std::iter;

fn convert_char(ori_char: char) -> char {
  // if ori_char.is_numeric() {
  //   ori_char
  // } else if ori_char >= 'a' {
  //   // lowercase, 'a' = 97, convert to 'z'
  //   std::char::from_u32('a' as u32 + 'z' as u32 - ori_char as u32).unwrap()
  // } else {
  //   // uppercase, 'A' = 65, convert to 'z'
  //   std::char::from_u32('A' as u32 + 'z' as u32 - ori_char as u32).unwrap()
  // }
  match ori_char {
    'a'..='z' => std::char::from_u32('a' as u32 + 'z' as u32 - ori_char as u32).unwrap(),
    'A'..='Z' => std::char::from_u32('A' as u32 + 'z' as u32 - ori_char as u32).unwrap(),
    _ => ori_char,
  }
}

fn convert_str(src: &str) -> impl Iterator<Item = char> + '_ {
  src
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| convert_char(c))
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  convert_str(plain)
    .enumerate()
    .flat_map(|(i, c)| {
      iter::once(' ')
        .filter(move |_| i > 0 && i % 5 == 0)
        .chain(iter::once(c))
    })
    .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  convert_str(cipher).collect()
}
