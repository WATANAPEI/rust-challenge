use std::env;

struct IPv4 {
    address: [u8; 4],
}

impl IPv4 {
    pub fn new(address_str: &str) -> Result<IPv4, &str> {
        let v: Vec<u8> = address_str.split('.').into_iter().map(|x| x.parse().unwrap()).collect();
        if v.len() != 4 {
            return Err("address must contain 4 segment");
        }
        Ok(IPv4 {
            address: [v[0], v[1], v[2], v[3]]
        })
    }
}

fn print_usage(args: &Vec<String>) {
    println!("usage: {} address ", &args[0]);
    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage(&args);
        return;
    }

    let ipv4: IPv4 = IPv4::new(&args[1]).unwrap();
    for e in &ipv4.address {
        println!("{}", e);
    }
}
