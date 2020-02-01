use std::collections::BTreeMap;

// fn lowercase_char(src: char) -> char {
//   match src {
//     'A'..='Z' => std::char::from_u32(src as u32 + 'a' as u32 - 'A' as u32).unwrap(),
//     _ => src,
//   }
// }

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
  let mut char_point_map = BTreeMap::new();
  for (point, char_vec) in h {
    for s_char in char_vec {
      // char_point_map.insert(lowercase_char(*s_char), *point);
      char_point_map.insert(s_char.to_ascii_lowercase(), *point);
    }
  }
  char_point_map
}
