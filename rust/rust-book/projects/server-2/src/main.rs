use std::{
    io::{stdout, Write},
    {thread, time},
};

use curl::easy::Easy;
const CURL_URL: &str = "127.0.0.1";
const PORT: &str = "7878";

fn main() {
    let mut easy = Easy::new();
    for i in 0..=10 {
        let url = format!("{}:{}", CURL_URL, PORT);
        easy.url(&url).unwrap();

        easy.write_function(|data| {
            stdout().write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        easy.perform().unwrap();
        println!("-----{i}----");
        thread::sleep(time::Duration::from_secs(1));
    }
}
