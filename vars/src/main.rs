fn main() {
    // let read_only: i32;
    // read_only = 3;

    let mut mutable = 5;
    mutable += 5;
    println!("{}", mutable);
    {
        let mutable = mutable *2;
        println!("{}", mutable);
    }
    mutable += 1;
    println!("{}", mutable);
}