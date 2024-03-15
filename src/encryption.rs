use std::fs;
use std::io;
use crate::files;

pub fn encryption_hello()
{
    println!("Hello, From encryption.rs")
}

fn encrypt_char(ch: char) -> char {
    ((ch as u8) + 1) as char
}

fn encrypt_string(s: &str) -> String {
    s.chars().map(encrypt_char).collect()
}


fn main() -> std::io::Result<()> {
    let file_contents = files::read_files("D://SPZProject//src//test.txt")?;

    Ok(())
}

