use std::io;
use std::cmp::Ordering;

fn main() {
    println!("please enter your first number: ");

    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("error");

    let first: u32 = match first.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("please enter your second number: ");

    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("error");

    let second = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("your inputs are {first} and {second}");

    match first.cmp(&second) {
        Ordering::Less => println!("{first} is smaller than {second}"),
        Ordering::Greater => println!("{first} is greater than {second}"),
        Ordering::Equal => println!("{first} is equal to {second}"),
    }
}