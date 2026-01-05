use std::fs::read_to_string;

fn main() {
    let txt_vec: Vec<String> = read_lines("./src/input.txt");
    let vec_int_vec: Vec<Vec<i32>> = integer(txt_vec);
    println!("{}", checker(vec_int_vec));
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn integer(vector: Vec<String>) -> Vec<Vec<i32>> {
    vector
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn checker(vec_int_vec: Vec<Vec<i32>>) -> u32 {
    let mut count: u32 = 0;

    for row in vec_int_vec {
        let diffs: Vec<i32> = row.windows(2).map(|w| w[0] - w[1]).collect();

        let all_valid =
            !diffs.iter().any(|&x| x == 0 || x.abs() > 3) &&
            !(diffs.iter().any(|&x| x < 0) && diffs.iter().any(|&x| x > 0));
        
        if all_valid {
            count += 1;
        }
    }
    count
}