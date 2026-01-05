

fn main() {
    let a = [1 , 2, 3, 4, 5, 10];

    for i in 0..a.len() {
        match a[i] {
            1 | 2 => println!("1 or 2"),
            3     => println!("3"),
            3..=5 => println!("3 to five"),
            _     => println!("catching other cases"),
        }
    }
}