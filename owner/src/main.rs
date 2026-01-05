fn main() {
    let s: &str = "hello";
    dbg!(s);
    println!("{:p}", &s);
    let p: String = String::from("hello");
    println!("p:  {:p}", &p);
    let p1: String = p;
    println!("p1: {:p}", &p1);
    
}
