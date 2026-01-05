fn main() {
    let mut temperature = 14;

    while temperature < 25 {
        temperature += 1;
        println!("turn on da fuckin heater!");
        if temperature > 16 {
            continue;
        }
        if temperature > 21 {
            break;
        }
        println!("it is a really colda!");
    }
    println!("just turn it off!");
}
