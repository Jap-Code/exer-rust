
fn main() {
    let temp = 19;
    let text = if temp > 21 {
        "es ist warm"
    } else if temp > 24 {
        "es ist heiß"
    } else {
        "what da fuck!?"
    };
    println!("{}", text)
}