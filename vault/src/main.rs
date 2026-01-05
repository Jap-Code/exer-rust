use rand::Rng;
use std::io;

#[derive(Debug)]
enum Dice {
    Three,
    Four,
    Six,
    Eight,
}

impl Dice{
    fn sides(&self) -> u32 {
        match self {
            Dice::Three => 3,
            Dice::Four  => 4,
            Dice::Six   => 6,
            Dice::Eight => 8,
        }
    } 
}

impl Dice{
    fn roll(&self) -> u32 {
        let rng = rand::rng().random_range(1..=self.sides());
        rng
    }
}

fn main() {
    println!("How many D shall your Dice have?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    let input: u32  = match input
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

    match input {
        3       => println!("Rolling.. {:?}", Dice::Three.roll()),
        4       => println!("Rolling.. {:?}", Dice::Four.roll()),
        6       => println!("Rolling.. {:?}", Dice::Six.roll()),
        8       => println!("Rolling.. {:?}", Dice::Eight.roll()),
        _       => println!("Available are D3, D4, D6 and D8."),
    };

}