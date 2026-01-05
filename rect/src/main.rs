use std::io;
use rand::Rng;

#[derive(Debug)]
enum Geometrics {
    Rect(Rect),
    Circle(Circle),
}
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn circumference(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        self.radius * 3.142
    } 
}

fn main() {
    println!("please insert your width: ");
    let w = input();
    println!("please insert your height: ");
    let h = input();

    let rect1 = Rect {
        width: w,
        height: h,
    };

    println!("choose operation: 1 for area, 2 for circumference: ");
    match input() {
        1 => println!("{}", rect1.area()),
        2 => println!("{}", rect1.circumference()),
        _ => println!("Error"),
    };
    let circ1 = Geometrics::Circle(Circle {radius: 10.0});
    println!("circle: {:?}", circ1);
}

fn input() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");

    let input: u32 = match input
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => rand::rng().random_range(1..100),
        };
    input
}