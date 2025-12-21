use std::collections::HashMap;
pub fn handle_hasmaps(users: HashMap<String, i32>, key: String) {
    let age: Option<&i32> = users.get(&key);

    match age {
        Some(val) => println!("{}", val),
        None => println!("Null")
    }
}

pub fn group_values_by_keys(vec: Vec<(String, i32)>) ->  HashMap<String, i32> { // passing in a vector of turples
    let mut hm = HashMap::new();

    for (key, val) in vec {
        hm.insert(key, val);
    }
    return hm;
}