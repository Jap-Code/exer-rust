struct Point {
    x: i32,
    y: i32,
    z: i32,
}
fn main() {
    let origin = Point {
        x: 0,
        y: 0,
        z: 0,
    };
    println!("Point({}, {})", origin.x, origin.y);

    let original = Point {
        x: 12,
        y: 24,
        z: 36,
    };
    println!("{} {} {}", original.x * 2, original.y * 3, original.z * 4);
}

