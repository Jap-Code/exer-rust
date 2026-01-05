use std::env;

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        3.14159265359 * self.radius.powi(2)
    }
    fn circumference(&self) -> f32 {
        2.0 * 3.14159265359 * self.radius
    }
    fn sph_vol(&self) -> f32 {
        (4.0/3.0) * 3.14159265359 * self.radius.powi(3)
    }
    fn sph_area(&self) -> f32 {
        4.0 * 3.14159265359 * self.radius.powi(2)
    }
}

fn main() {

    let arg = env::args().nth(1).expect("usage: <prog> <radius>");
    let r: f32 = arg.parse().expect("radius must be a number");

    let circ1 = Circle {
        radius: r
    };

    println!("radius:\t\t{} cm", r);
    println!("area:\t\t{} cm2", circ1.area());
    println!("circumference:\t{} cm", circ1.circumference());
    println!("volume:\t\t{} cm3", circ1.sph_vol());
    println!("area:\t\t{} cm2", circ1.sph_area());
}