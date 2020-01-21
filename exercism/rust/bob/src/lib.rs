const REPLY_QUESTION: &str = "Sure.";
const REPLY_YELL: &str = "Whoa, chill out!";
const REPLY_YELL_QUESTION: &str = "Calm down, I know what I'm doing!";
const REPLY_EMPTY: &str = "Fine. Be that way!";
const REPLY_OTHERS: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
  let msg = message.trim();

  let empty = msg.is_empty();
  let question = msg.ends_with('?');
  let yell = msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic());

  match (yell, question) {
    _ if empty => REPLY_EMPTY,
    (true, true) => REPLY_YELL_QUESTION,
    (true, _) => REPLY_YELL,
    (_, true) => REPLY_QUESTION,
    _ => REPLY_OTHERS,
  }
}
