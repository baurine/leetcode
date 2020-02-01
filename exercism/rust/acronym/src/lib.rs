pub fn abbreviate(phrase: &str) -> String {
  phrase
    .split(" ")
    .filter(|word| word.chars().any(|c| c.is_alphabetic())) // 过滤掉不含字母的连接符
    .flat_map(|word| word.split("-")) // 对含有 `-` 连接符的单词再次进行 split
    .map(|word| word.trim_matches('_')) // 清除下划线
    .map(|word| {
      if word == word.to_uppercase() || word == word.to_lowercase() {
        // 如果全是大写或小写，取首字母
        // 比如：GNU Image Manipulation Program -> GIMP
        // 比如：Ruby on Rails -> ROR
        word
          .chars()
          .next()
          .unwrap()
          .to_ascii_uppercase()
          .to_string()
      } else {
        // 否则取单词中大写的字母
        // 比如：HyperText Markup Language -> HTML
        word
          .chars()
          .filter(|c| c.is_uppercase())
          .collect::<String>()
      }
    })
    .collect::<String>()
}
