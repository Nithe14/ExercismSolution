use regex::Regex as re;

pub fn reply(message: &str) -> &str {
    let question = re::new(r".+\?$").unwrap();
    let yelling = re::new(r"^[A-Z|0-9]+[^a-z]*[A-Z]+[^a-z]*.*$").unwrap();
    let silence = re::new(r"^\s+$").unwrap();

    match (
        question.is_match(message.trim()),
        yelling.is_match(message.trim()),
        silence.is_match(message.trim()) || message.trim() == "",
    ) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (false, true, false) => "Whoa, chill out!",
        (true, false, false) => "Sure.",
        (_, _, true) => "Fine. Be that way!",
        (_, _, _) => "Whatever.",
    }
}
