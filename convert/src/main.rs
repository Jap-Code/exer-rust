use std::io;

fn main() {
    println!("Scale to convert to (options are C and K): ");
    let mut scale = String::new();
    io::stdin()
        .read_line(&mut scale)
        .expect("Error");

    println!("Input K/C to convert from: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Error");

    let temp: i32 = match temp
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => error()
        };

    match scale
        .trim()
        .to_lowercase()
        .chars()
        .next() { //gibt ein Option<T> zurück -> Some(Wert) oder null
            Some('c')   => println!("{} K is {}° C", temp,  celsius(temp)), // um damit arbeiten zu können muss man ihn erst mit match z.B. entpacken
            Some('k')   => println!("{}° C is {} K", temp, kelvin(temp)),
            _           => println!("Error: {}", error())
        };
}   

fn celsius(temp: i32) -> i32 {
    temp - 273
}

fn kelvin(temp: i32) -> i32 {
    temp + 273
}

fn error() -> i32 {
    0
}