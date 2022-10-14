
pub fn check_msg(tokens: &Vec<&str>) -> bool {
    for token in tokens {
        let word = token.to_lowercase();
        if word == "js" || word == "javascript" {
            return false;
        }
    }
    true
}
