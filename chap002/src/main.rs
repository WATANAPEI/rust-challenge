use std::env;

fn gcd(a: i32, b: i32) -> i32 {
    let mut gcd: i32 = 0;
    return gcd;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("argument number must be 3");
        return;
    }
    let a: i32 = args[1].parse().expect("arg1 must be integral");
    let b: i32 = args[2].parse().expect("arg2 must be integral");
    let gcd: i32 = gcd(a, b);
    println!("gcd: {:?}", gcd);
}
