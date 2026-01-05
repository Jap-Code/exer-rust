//temp conversion exercise
//user input
//conversion
//println! result
use std::io;

fn main() {
    println!("Celcius and Fahrenheit converter.");

    let mut usecase = String::new();

    io::stdin()
        .read_line(&mut usecase)
        .expect("please enter only 'f' or 'c'");

    println!("please enter your temperature to convert from");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("please enter a number");

    let temp: u32 = temp.trim().parse()
        .expect("please enter a number");

    match usecase.trim() {
        "c" => println!("temp is {} °C", temp * 10),
        "f" => println!("temp is {} °F", temp * 100),
        _ => println!("invalid input {:?}", usecase),
    }
}