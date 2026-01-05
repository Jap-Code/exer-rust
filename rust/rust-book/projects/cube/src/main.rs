use std::io;

#[derive(Debug)]
struct Cube {
    width: f32,
    depth: f32,
    height: f32,
}

impl Cube {
    
    fn volume(&self) -> f32 {
        self.width * self.depth * self.height
    }
}

fn main() {

    println!("input your params: ");

    let mut width = String::new();
    let mut depth = String::new();
    let mut height = String::new();

    println!("width:");
    io::stdin()
        .read_line(&mut width)
        .expect("error");

    println!("depth:");
    io::stdin()
        .read_line(&mut depth)
        .expect("error");

    println!("height:");
    io::stdin()
        .read_line(&mut height)
        .expect("error");

    let cube1 = Cube {
        width: width.trim().parse().unwrap(),
        depth: depth.trim().parse().unwrap(),
        height: height.trim().parse().unwrap(),
    };

    println!("volume: {}", cube1.volume());

}