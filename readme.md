# A demo rust library

this is a demo rust library published on crates.io

you may have a look of source code in this repo how a minimal rust library could look like

to use this library

add `malik_sum = "0.1.4"` in your cargo.toml file in dependency section

you cargo.toml file should look like this now:
```
[package]
name = "project1"
version = "0.1.0"
authors = ["Mohammad Inzamam Malik <malikasinger1@gmail.com>"]
edition = "2018"

[dependencies]
malik_sum = "0.1.4"
```


and then in `src/main.rs` you may do something like this:


```
extern crate malik_sum;

fn main() {
    println!("Hello, world!");

    println!("add: {}", malik_sum::add(2, 6));
    println!("subtract: {}", malik_sum::subtract(2, 6));
    println!("multiply: {}", malik_sum::multiply(2, 6));
    println!("divide: {}", malik_sum::divide(2, 6));
    println!("power: {}", malik_sum::power(2, 6));
}
```

now do `cargo run` to see the results. 


you may contact me on malikasinger@gmail.com incase you need rust consultancy or you have a project to be done in rust, 
you may also check my upwork profile here: https://www.upwork.com/freelancers/~014998370bf4b28c01
you may become my fan on facebook: https://web.facebook.com/malikasinger

best regards,  
M. Inzamam Malik
