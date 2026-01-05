use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("should have been able to read");

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let num1: i32 = parts[0].trim().parse().unwrap();
            vec1.push(num1);
            let num2: i32 = parts[1].trim().parse().unwrap();
            vec2.push(num2);
        }
    }
    println!("{}", double_count(vec1, vec2));
}

fn double_count(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut big_count: i32 = 0;
    for num1 in &vec1 {
        for num2 in &vec2 {
            let mut count: i32 = 0;
            if num1 == num2 {
                count += 1;
            }
            big_count += num1 * count;
        }
    }
    big_count
}