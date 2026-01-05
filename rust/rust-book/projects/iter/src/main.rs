
fn main() {
    let vector: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    //let iterated: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    //let iterated: Vec<i32> = vector.windows(3).map(|x| x[0] + x[2]).collect();


    // let iterated: Vec<i32> = vector
    //     .windows(vector.len()) //windows iteriert nur über &[T] und consumiert nicht Vec<T>
    //     .map(|x| x.iter().sum())
    //     .collect();

    // let iterated: Vec<i32> = vector
    //     .windows(2)
    //     .map(|x| x[0].pow(x[1] as u32))
    //     .collect();
    
    let iterated: Vec<i32> = vector
        .windows(vector.len())
        .map(|x| x.iter().sum())
        .collect();

    println!("{:?}", iterated);
}