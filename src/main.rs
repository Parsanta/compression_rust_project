use rand::Rng;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self, Seek, Write};
mod lib;
use crate::lib::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("data.txt")?;
    let length = 1000;
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let character = rng.gen_range(b'A'..=b'B') as char;
        file.write_all(&[character as u8])?;
    }

    file.seek(io::SeekFrom::Start(0))?;
    let content = read_file_contents(&mut file)?;
    println!("Original Content: {}", content.len());

    let compressed_content = compress_text(&content);

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open("data.txt")?;

    file.write_all(compressed_content.as_bytes())?;

    file.seek(io::SeekFrom::Start(0))?;
    let content1 = read_file_contents(&mut file)?;
    println!("Compressed Content: {}", content1.len());

    Ok(())
}
