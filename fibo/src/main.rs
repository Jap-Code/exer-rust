use std::io;

fn main() {

    let mut size = String::new();
    println!("Input the nth number to be calculated: ");
    io::stdin()
        .read_line(&mut size)
        .expect("Not readable!");

    let size: u64 = match size
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_)  => 1
        };

    fibonacci(size);
}

fn fibonacci(size: u64) -> u64 {
    if size == 0 { return 0; }
        if size == 1 { return 1; }

        let mut a = 0; // F(n-2)
        let mut b = 1; // F(n-1)
        
        for _ in 2..=size {
            let next = a + b;
            a = b;
            b = next;
        }
        
    println!("Final Fibonacci number: {}", b);
    b
}