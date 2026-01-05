use std::io;
use rand::Rng;

#[derive(Debug)]
struct Numbers {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

fn numbers(a: u32, b: u32, c: u32, d: u32) -> Numbers {
    Numbers {
        a,
        b, 
        c, 
        d,
    }
}

fn input(x: &str) -> u32 {
    println!("please input you values for {}: ", x);
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Error");

    let x: u32 = match x
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => rand::rng().random_range(1..100),
        };
    x
}

fn calculation(input: &Numbers) -> u32 {
    let product = input.a * input.b * input.c * input.d;
    product
}

fn main() {
    let (a, b, c, d) = (input("a"), input("b"), input("c"), input("d"));

    let mut construct = numbers(a, b, c, d);
    println!("{}", construct.a);
    println!("{}", construct.b);
    println!("{}", construct.c);
    println!("{}", construct.d);

    construct.a = rand::rng().random_range(1..10);
    dbg!(&construct);

    println!("{}", calculation(&construct));
}