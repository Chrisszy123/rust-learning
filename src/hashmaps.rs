use std::collections::HashMap;
pub fn handle_hasmaps(users: HashMap<String, i32>, key: String) {
    let age: Option<&i32> = users.get(&key);

    match age {
        Some(val) => println!("{}", val),
        None => println!("Null")
    }
}