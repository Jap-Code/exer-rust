use std::io;

fn main() {
    println!("Please enter a number");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("invalid input {n}");
    
    let n: u32 = match n.trim().parse() {
        Ok(num) => num, 
        Err(_) => 1,
    };

    println!("{}", fib_calc(n));
}

fn fib_calc(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}