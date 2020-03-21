use std::env;

fn lcm(v: &Vec<i32>) -> &i32 {
    match v.last() {
        Some(e) => e,
        None => { panic!("something went wrong"); },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Number of argument must be more than 2");
        return;
    }
    let nums: Vec<i32> = args[1..].into_iter().map(|x| x.parse().unwrap()).collect();
    let result: &i32 = lcm(&nums);
    println!("lcm is {}", result);
}
