use rand::Rng;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};


// fn main() {
//     let dice = dice_roll();
//     println!("{}", dice);

//     match dice {
//         3 => println!("you stink"),
//         9 => println!("it's me"),
//         _ => println!("something else"),
//     };
// }

// fn dice_roll() -> u32 {
//     let dice = rand::rng().random_range(1..=10);
//     dice
// }

fn main() {
    let mut rng = rand::rng();
    let mut count: u32 = 0;
    
    for idx in 0..1000 {
        let dice: u32 = rng.random_range(1..10);
        print!("\ridx:\t{}\troll:\t{}\tcount:\t{}", idx, dice, count);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
        count += dice;
    }
}

// fn main() {
    
//     for idx in 0..=100 {
//         println!("idx:\t{}", idx);
//         let dice: u32 = rand::rng().random_range(1..=16);
//         println!("roll:\t{}", dice);
//     }
// }