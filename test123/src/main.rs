use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    let reader = io::BufReader::new(File::open("input")?);

    let mut processed = Vec::new();

    for line in reader.lines() {
        let numbered: Result<Vec<u8>, String> = line?
            .bytes()
            .map(|piece| match piece {
                b'@' => Ok(1),
                b'.' => Ok(0),
                _    => Err(format!("Unexpected char {}!", piece as char)),
            })
            .collect();

        match numbered {
            Ok(numbered) => processed.push(numbered),
            Err(e)   => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        };
    }

    Ok(())
}


