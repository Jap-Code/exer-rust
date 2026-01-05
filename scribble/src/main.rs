struct Test {
    number: u32,
    number_two: u32,
}

fn main() {
    let trying = Test {
        number: 3,
        number_two: 2,
    };
    println!("{}, {}", trying.number, trying.number_two);
}