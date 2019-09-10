pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with('?');

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_shouted =
        message.chars().any(char::is_uppercase) && !message.chars().any(char::is_lowercase);

    match (is_question, is_shouted) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
