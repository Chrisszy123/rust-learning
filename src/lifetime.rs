pub fn longest<'a> (str1: &'a str, str2: &'a str) -> &'a str { // the 'a is a lifetime identifier that says the shortest/intersection of the lifetimes of str1 and str2 should be taken
    if str1.len() > str2.len(){
        str1
    } else {
        str2
    }
}  