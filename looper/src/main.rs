fn main() {
    let mut temperatur: i8 = 16;

    let text = loop {
        if temperatur > 21 {
            break "turn it off!"
        }
        println!("turn it on!");
        temperatur += 1;
        if temperatur > 16 {
            continue;
        }
        println!("fucking cold man!");
    };
    println!("{}", text);
}
