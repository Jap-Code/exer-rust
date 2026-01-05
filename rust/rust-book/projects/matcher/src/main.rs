use std::io;

fn main() {
    println!("please enter a number: ");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("invalid input");


    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,

    };
    compare(number)
}

fn compare(number: u32) {
    match number {
        1 => println!("booya"),
        2 => println!("yeehaa"),
        _ => {
            println!("error.error.error");
        }
    }
}

