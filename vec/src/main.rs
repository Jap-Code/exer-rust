

fn main() {
    let vector: Vec<u32> = vec![1, 2, 3, 4, 5, 0];

    println!("chosse an index number to fetch data from the vector: ");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error while reading");

    let input: usize = match input
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_)  => return eprintln!("Please insert an integer!")
        };

    let result: Option<&u32> = vector.get(input);
    let output = match result {
        Some(val) => val,
        None      => return eprintln!("Index out of range. Lenght of vector is {}!", vector.len()),
    };

    println!("{:?}", output);
}
