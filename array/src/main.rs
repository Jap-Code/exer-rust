fn main() {
    let rep = [3.1464; 10];
    println!("{:?}", rep);
    let whole = 3;
    let mut array = [whole, 4, 5];
    println!("{:?}", array);
    array[0] = 1;
    array[1] = 10;
    let [a, _, b] = array;
    println!("{:#?}, {}", array, a);
    naim();
}

fn naim() {
    let double = [[1, 2, 3], [2, 3, 4], [3, 4, 5]];
    println!{"{:#?}", double};
}