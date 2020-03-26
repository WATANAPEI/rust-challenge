extern crate chap015;
use std::env;
use chap015::ipv4::IPv4;


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

    match IPv4::new(&args[1]) {
        Ok(v) => {
            println!("{:?}", v);
            for e in &v.address {
                println!("{}", e);
            }
        },
        Err(e) => {
            println!("{}",e);
        }
    }
}
