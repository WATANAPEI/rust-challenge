extern crate chap015;
use std::io::{self, BufRead, BufReader};
use chap015::ipv4::IPv4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("input ip v4 address:");
    println!("from: ");
    let mut lines_iter = BufReader::new(std::io::stdin())
        .lines().map(|l| l.unwrap());
    match lines_iter.next() {
        Some(v) => {
            println!("echo: {}", v);
        },
        None => {
            println!("Error occured: No input");
        },
    }

    /*
    for result in BufReader::new(std::io::stdin()).lines() {
        let l = result?;
        println!("{}", l);
    }
    */
    Ok(())
}
