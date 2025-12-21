
pub fn get_first_word(s: &String) -> &str { // this is not returning a string, but a string slice
    let mut index = 0;

    for (_, i) in s.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index+1;
    }
    return &s[0..index];
}