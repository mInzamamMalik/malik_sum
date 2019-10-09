use std::ops::Add;

pub fn add<T>(num1: u32, num2: u32) -> u32 {
    num1 + num2
}


pub fn subtract(num1: u32, num2: u32) -> u32 {
    num1 - num2
}


pub fn multiply(num1: u32, num2: u32) -> u32 {
    num1 * num2
}


pub fn divide(num1: u32, num2: u32) -> u32 {
    if (num2 == 0) {
        println!("can not divide with zero");
        0
    } else {
        num1 / num2
    }
}


pub fn power(num1: u32, num2: u32) -> u32 {
    num1.pow(num2)
}
