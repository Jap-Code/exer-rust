fn main() {
    for i in 0..=3 {
        for y in 0..=3 {
            for z in 0..=3 {
                println!("{}.{}.{}", i, y, z);
            }
        }
    }
    real_range();
}

// diese art der Schleife ist die von Rust empfohlene
fn real_range() {
    for i in 0.. {
        println!("{}", i);
        if i > 3 {
            break;
        }
    }
    var_range();
}

// funktioniert in bash z.B. so nicht. 
fn var_range() {
    let beginn = 1;
    let ende = 10;
    for i in beginn..ende {
        println!("loop it my boy {}", i);
    }
}