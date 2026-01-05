use std::io;
use rand::Rng;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("{:?}", generator(input));
}

fn generator(input: u32) -> Vec<u32> {
    let mut rng = rand::rng();
    let mut vector = Vec::new();

    for _ in 0..input {
        let dice: u8 = rng.random_range(1..10);
        vector.push(dice.into());
    };
    vector
}
