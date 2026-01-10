use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // vecxtract();
    // pig_latin();
    add_employee();
}


fn vecxtract() {
    let mut list: Vec<i32> = vec![2 ,4 ,3 ,18 ,6 ,45 ,2 ,98 ,0, 4, 109, 255, 4, 1, 76, 2];
    list.sort();
    println!("{:?}", list);

    let median: i32 = list[list.len()/2];
    dbg!(&median);

    let mut map = HashMap::new();

    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    dbg!(&map);

    let mode = map
        .iter()
        .max_by_key(|&(_k, v)| v);

    if let Some((k, v)) = mode {
        println!("Most frequent value is: {} with amount: {}", k, v);

    }
}


fn pig_latin() {
    println!("Give a word to convert: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("should be able to read");

    let word = input.trim();
    let first_char = word.chars().next().unwrap_or(' ');

    let is_vowel = "aeiouAEIOU".contains(first_char);

    let result = if is_vowel {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", &word[first_char.len_utf8()..], first_char)
    };

    dbg!(&result);
    

    // let mut input: String = match input
    //     .trim()
    //     .parse() {
    //         Ok(word)    => word,
    //         Err(_)              => return eprintln!("Wrong input!"),
    //     };

    // match input.starts_with(['a', 'e', 'i', 'o', 'u']) {
    //     true    => {
    //         input.push_str("-hay");
    //     }
    //     false   => {
    //         input.push('-');
    //         let new = input.remove(0);
    //         input.push(new);
    //         input.push_str("ay");
    //     },
    // };
    // dbg!(&input);
    
}


