#[derive(Debug)]
struct Point {x: i32, y: i32}

#[derive(Debug)]
struct Rect {p0: Point, p1: Point}

fn main() {
    let (x, y) = (0, 11);
    let origin = Point { x, y};

    let p1 = Point {x: 10, ..origin};

    let rectangle = Rect {p0: origin, p1};
    println!("{:#?}", rectangle);

    let (a, _b, _c, d) = (1, 2, 3, 4);
    println!("{} {}", a, d);
    circler();
}

#[derive(Debug)]
struct Circle {r: i32, c: i32}

#[derive(Debug)]
struct Eight {one: Circle, two: Circle}

fn circler() {
    let c1 = Circle{r: 1, c: 2};
    let c2 = Circle{r: 3, c: 4};

    let e8 = Eight{one: c1, two: c2};

    println!("{:#?}", e8);
    enumerate();
}

fn enumerate() {
    #[derive(Debug)]
    enum Ampel { Rot, Gelb, Grün}

    let ampel = Ampel::Grün;

    println!("{:?}", ampel);
    example();
}


fn example() {
    #[derive(Debug)]
    enum Wert {
        Ganzzahl(i32), 
        Float(f64),
        Tubel((i32, f64)),
    }

    let value = Wert::Tubel((3, 4.0));

    println!("{:?}", value);
}