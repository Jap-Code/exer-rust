use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 15);

    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    dbg!(&score);
    
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
    
    scores.insert(String::from("Blue"), 15);
    
    scores.entry(String::from("Yellow")).or_insert(50);
    dbg!(&scores);

    
    let text = "hello world hello wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
    }
    println!("{map:?}");

}
