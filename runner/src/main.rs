static ARRAY: [[i32; 3]; 3] = [[1, 2, 3], [21, 22, 23], [31, 32, 33]];

fn main() {
    for i in ARRAY.iter() {
        println!("{:?}", i );
    }
    
    let x: u32 = get_rn_num();
    println!("{:?}", ARRAY[x as usize])
    
}

fn get_rn_num() -> u32 {
    let x: u32 = rand::random::<u32>() % 3;
    x
}
