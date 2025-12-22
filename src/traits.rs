pub trait Summary {
    fn summarize(&self) {
        println!("Hello from Summary");
    }
    fn get_age(&self) -> i32;
    fn debug(&self);
}
pub trait Calc {
    fn sum(&self, x: i32, y: i32) -> i32 {
        println!("sum");
        x + y // return without the key word, and no semi colon
    }
}

pub fn handle_traits<T: Summary>(u: T) {
    // the input params has to have summary and calc traits
    println!("{:?}", u.summarize());
}
