use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(false)
            .open("/home/jle/file.txt")
            .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);

    file.write_all(b"suh duh")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);

    Ok(())
}
