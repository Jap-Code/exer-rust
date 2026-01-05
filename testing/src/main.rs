use std::io;

fn main() {
    let mut text = String::new();
    
    println!("Please input some Text: ");

    io::stdin()
        .read_line(&mut text)
        .expect("Error while reading input!");

    dbg!(&text);
    
    println!("{}", word_count(&text));
    println!("{}", index(&text));
}

fn word_count(text: &str) -> usize {
    let count: Vec<&str> = text.split_whitespace().collect();
    count.len()
}

fn index(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return &text[..i]
        }
    }

    &text[..]
}