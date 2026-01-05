use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn dice_count() -> u8 {
    println!("Input how many dice to roll?");

    let mut dice_count = String::new();
    io::stdin()
        .read_line(&mut dice_count)
        .expect("failed to read");
    
    let dice_count: u8 = match dice_count.trim().parse() {
        Ok(num) => num, 
        Err(_) => 0,
    };
    dice_count
}

fn hit(dice: u8) -> u8 {
    println!("\nOn what to hit?");

    let mut hit = String::new();

    io::stdin()
        .read_line(&mut hit)
        .expect("failed to read");

    let hit: u8 = match hit.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match dice.cmp(&hit) {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        Ordering::Greater => 1,
    }
}

fn main() {
    let dice_count: u8 = dice_count();
    let mut rng = rand::rng();

    for _ in 0..dice_count {
        let dice: u8 = rng.random_range(1..=12);
        let hit = hit(dice);
        print!("roll: {} hit: {}\t", dice, hit);
        println!("\n----------");
    }
}