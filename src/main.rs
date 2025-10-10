fn main() {
    println!("Rust, funtions!");
    println!("{}", is_even(15));
    println!("{}", fib(15));
    let my_string = String::from("Hello");
    let count: usize = str_char_len(my_string);
    println!("The string count for hello {}", count);
    let user:User = get_user_details();
    println!("The user details {:?}", user);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: i32) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return 1;
    }

    for _ in 0..(num - 1) {
        let temp: i32 = second;
        second = second + first;
        first = temp;

    }
    return second
}

fn str_char_len(s: String) -> usize {
    s.chars().count()
}

// Structs
#[derive(Debug)]
struct User {
    is_active: bool,
    name: String,
    email: String,
    age: i32
}

fn get_user_details() -> User {
    let user1:User = User {
        is_active: true,
        name: String::from("Bassey Goodluck"),
        email: String::from("bassygoodluck@gmail.com"),
        age: 29
    };
    user1
}