use std::io;
use rand::Rng;

fn main() {
    println!("How many dice to roll?: ");

    let mut input = String::new();
    println!("{:p}", &input);
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let input: u32 = match input
        .trim()
        .parse() {
            Ok(num) => num, 
            Err(_) => 1,
        };

    let mut one: u32 = 0;
    let mut two: u32 = 0;
    let mut three: u32 = 0;
    let mut four: u32 = 0;
    let mut five: u32 = 0;
    
    for _ in 1..=input {
        let rng: u32 = rand::rng().random_range(1..=5);
        match rng {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            4 => four += 1,
            5 => five += 1,
            _ => ()
        };
    }
    dbg!(one, two, three, four, five);
    println!("{:p}", &one);
    println!("{:p}", &two);
    println!("{:p}", &three);
    println!("{:p}", &four);
    println!("{:p}", &five);
    println!("{:p}", &input);
}

// fn main() {
//     let stack_var = 5;
//     let heap_var = Box::new(5);

//     println!("Address of stack_var: {:p}", &stack_var);
//     println!("Address of heap_var:  {:p} (pointer on stack)", &heap_var);
//     println!("Address of heap data: {:p} (actual data on heap)", heap_var);
// }