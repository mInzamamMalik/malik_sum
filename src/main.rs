extern crate malik_sum; // optional in same project

fn main() {
    println!("Hello, world!");

    println!("add: {}", malik_sum::add(2, 6));
    println!("subtract: {}", malik_sum::subtract(2, 6));
    println!("multiply: {}", malik_sum::multiply(2, 6));
    println!("divide: {}", malik_sum::divide(2, 6));
    println!("power: {}", malik_sum::power(2, 6));
}
