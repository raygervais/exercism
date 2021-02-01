// https://stackoverflow.com/questions/26915472/how-do-i-return-a-reversed-string-from-a-function
pub fn reverse(input: &str) -> String {
    return input
        .chars()
        .rev()
        .collect();
}
