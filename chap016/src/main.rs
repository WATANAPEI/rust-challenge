extern crate chap015;
use std::io::{self, BufRead, BufReader};
use chap015::ipv4::IPv4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("input ip v4 address:");
    println!("from: ");
    let mut line = String::new();
    let mut stdio_buffer = BufReader::new(std::io::stdin());
    let _ = stdio_buffer.read_line(&mut line)?;
    print!("echo: {}", line);
    println!("to: ");
    let _ = stdio_buffer.read_line(&mut line)?;
    print!("echo: {}", line);
    Ok(())
}
