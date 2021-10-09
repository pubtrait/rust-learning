use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read a line into buffer
    loop {
        let size = reader.read_line(&mut buffer).unwrap_or_else(|err| {
            println!("{}", err);
            0
        });
        if size == 0 {
            return Ok(());
        }
        println!("got {}", buffer);
        buffer.clear();
    }
}
