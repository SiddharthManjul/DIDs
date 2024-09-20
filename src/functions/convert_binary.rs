pub fn string_to_binary(msg: &str) -> Vec<String> {
    msg
    .chars()
    .map(|c| format!("{:08b}", c as u8))
    .collect::<Vec<String>>()
}