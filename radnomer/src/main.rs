use rand::Rng;
use std::io;

fn main() {

    println!("Input the number of randoms: ");
    let mut count = String::new();
    io::stdin()
        .read_line(&mut count)
        .expect("error");

    let count: u32 = match count
        .trim()
        .parse() {
            Ok(num) => num, 
            Err(_) => 1
        };
    
    println!("Input the upper Range limit: ");

    let mut rng = String::new();
    io::stdin()
        .read_line(&mut rng)
        .expect("error");

    let rng: u32 = match rng
        .trim()
        .parse() {
            Ok(num) => num, 
            Err(_) => 1
        };

    let mut storage: Vec<u32> = Vec::new();

    for _ in 1..=count {

        let rand = rand::rng().random_range(1..=rng);
        storage.push(rand);
    }
    println!("{:#?}", storage);
}