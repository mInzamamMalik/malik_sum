use std::ops::Add;

pub fn add(num1: i64, num2: i64) -> i64 {
    num1 + num2
}


pub fn subtract(num1: i64, num2: i64) -> i64 {
    num1 - num2
}


pub fn multiply(num1: i64, num2: i64) -> i64 {
    num1 * num2
}


pub fn divide(num1: i64, num2: i64) -> i64 {
    if (num2 == 0) {
        println!("can not divide with zero");
        0
    } else {
        num1 / num2
    }
}


pub fn power(num1: i64, num2: u32) -> i64 {
    num1.pow(num2)
}
