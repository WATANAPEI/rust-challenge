extern crate chap015;
use std::io::{self, BufRead, BufReader};
use chap015::ipv4::IPv4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for result in BufReader::new(std::io::stdin()).lines() {
        let l = result?;
        println!("{}", l);
    }
    Ok(())
    /*
    let mut buf = String::new();
    let ip = io::stdin().read_to_string(&mut buf);
    if let Ok(v) = ip {
        println!("{:?}", v);
    }
    */
}
