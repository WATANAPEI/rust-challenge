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
    print!("echo: {}", &line);
    println!("to: ");
    let ip_from = IPv4::new(&trimmed_line);
    //line.clear();
    //let _ = stdio_buffer.read_line(&mut line)?;
    //let trimmed_line = &line.trim();
    let ip_to = IPv4::new(&trimmed_line);
    print!("echo: {}", &line);
    let a = ip_from.unwrap();
    //ipv4::calc_dist(ip_from, ip_to);
    Ok(())
}
