use std::io;

fn main() {

    let mut name = String::new();

    println!("Hi! whats your name?");

    io::stdin().read_line(&mut name).expect("Failed to read line");

    let name = name.trim();

    println!("whats your age?");

    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");

    let age: u32 = age.trim().parse().expect("Please type a number!");

    println!("hi {}, you are {} years old!", name, age);


}
