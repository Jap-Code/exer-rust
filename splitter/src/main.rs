use std::io;

fn main() {
    println!("Input you sentence to split: ");
    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("should have been able to read.");

    splitter(&s);
    println!("main: {}", s);
    println!("{}", first_word(&s));
}

fn splitter(s: &String) {
    let s: Vec<&str> = s.split_whitespace().collect();
    println!("{:#?}", &s);
    println!("{}", &s[0]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    dbg!(bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == 32 {
            dbg!(item);
            return i
        }
    }
    s.len()
}