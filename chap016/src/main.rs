use chap015::ipv4;
use chap015::ipv4::IPv4;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("input ip v4 address:");
    println!("from: ");
    let mut line = String::new();
    let mut stdio_buffer = BufReader::new(std::io::stdin());
    let _ = stdio_buffer.read_line(&mut line)?;
    let mut trimmed_line = &line.trim();
    let mut ip_from = IPv4::new(&trimmed_line);
    print!("echo: {}", &line);

    println!("to: ");
    //line.clear();
    let mut line_to = String::new();
    let mut stdio_buffer_to = BufReader::new(std::io::stdin());
    let _ = stdio_buffer_to.read_line(&mut line_to)?;
    let trimmed_line_to = &line_to.trim();
    let mut ip_to = IPv4::new(&trimmed_line_to);
    print!("echo: {}", &line_to);
    ipv4::calc_dist(ip_from.unwrap(), ip_to.unwrap());
    Ok(())
}
