use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("should have been able to read");

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let num1: i32 = parts[0].trim().parse().expect("NaN");
            vec1.push(num1);
            let num2: i32 = parts[1].trim().parse().expect("NaN");
            vec2.push(num2);
        }
    }
    
    vec1.sort();
    vec2.sort();

    println!("Distance: {}", calculation(vec1, vec2));
}

fn calculation(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}