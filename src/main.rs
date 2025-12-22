use std::{collections::btree_map::Values, fs::read_to_string, collections::HashMap}; // rust imports
use chrono::{Local, Utc};
mod borrows;
mod hashmaps;
mod vectors;
mod strings;
mod traits;
use crate::borrows::*;
use crate::hashmaps::*;
use crate::vectors::*;
use crate::strings::*;
use crate::traits::*;
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
    // using extenal packages
    let now = Local::now();
    println!("The time now is {}", now);

    let a = String::from("Hi");
    let b = a;
    println!("New String {}", b);

    let mut s1 = String::from("Goodluck");
    handle_borrows(&mut s1);
    println!("{}", s1);

    // hashmaps
    let mut users = HashMap::new();
    users.insert(String::from("Goodluck"), 29);
    users.insert(String::from("Bassey"), 66);

    println!("Printing age of Goodluck");
    handle_hasmaps(users, String::from("Goodluck"));
    let vec = vec![(String::from("Goodluck"), 29), (String::from("Bassey"), 66)];
    let grouped_vec = group_values_by_keys(vec);
    println!("The grouped vec {:?}", grouped_vec);
    vectors_handler();
    // strings
    let word: String = String::from("Goodluck Bassey");
    let first_word: &str = get_first_word(&word); // we take a string slice, which is a pointer of the string "word", hence ownership of "Goodluck Bassey" stays with word
    println!("{}, {}", first_word, word);
    // traits
    let user_trait: User = User{
        is_active: false,
        name: String::from("Goodluck Bassey"),
        email: String::from("bassygoodluck@gmail.com"),
        age: 29
    };
    handle_traits(user_trait); // the reason why this is working because, user trait, impl summary
    //println!("This should not work, as user_trait is out memory {:?}", user_trait);
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
impl Summary for User { // implementing the summary trait for the user
    fn summarize(&self) {
        println!("Hi there");
    }
    fn get_age(&self) -> i32{
        self.age
    }
    fn debug(&self){
        println!("User is {}, email {}, age {}, and active status is {}", self.name, self.email, self.age, self.is_active);
    }
}
fn get_user_age() -> i32 {
    let user1:User = User {
        is_active: true,
        name: String::from("Bassey Goodluck"),
        email: String::from("bassygoodluck@gmail.com"),
        age: 29
    };
    println!("debug is {:?}", User::debug(&user1)); // debug is a static method, and as such cannot be called on the object of a class, but from the class itself
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