use std::{collections::btree_map::Values, fs::read_to_string}; // rust imports

fn main() {
    println!("Rust, funtions!");
    println!("{}", is_even(15));
    println!("{}", fib(15));
    let my_string = String::from("Hello");
    let count: usize = str_char_len(my_string);
    println!("The string count for hello {}", count);
    let user:i32 = get_user_age();
    println!("The user age {:?}", user);

    let rect = Shape::Rectangle(1.0, 2.0);
    calculate_area(rect);
    let circle = Shape::Circle(4.0);
    calculate_area(circle);

    let index = find_first_a(String::from("James"));
    let result = read_to_string("text.txt");

    match index {
        Some(value) => println!("index is {}", value),
        None => println!("a not found"),
    }
    match result {
        Ok(val) => println!("Text file data {}", val),
        Err(err) => println!("There was an error, {}", err),
    }
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
impl User {
    fn get_age(&self) -> i32{
        self.age
    }
    fn debug() -> i32{
        1
    }
}
fn get_user_age() -> i32 {
    let user1:User = User {
        is_active: true,
        name: String::from("Bassey Goodluck"),
        email: String::from("bassygoodluck@gmail.com"),
        age: 29
    };
    println!("debug is {}", User::debug()); // debug is a static method, and as such cannot be called on the object of a class, but from the class itself
    user1.get_age()
}

enum Shape {
    Rectangle(f64, f64), // height and width
    Circle(f64) // radius
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(a, b) => a*b,
        Shape::Circle(a) =>  3.142 * a * a
    };
    area
}

fn find_first_a (s: String) -> Option<i32> { // options is used for Null values
    for (index, char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32)
        }
    }
    None
}

// use of Result Enum