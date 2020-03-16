use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let num: i32 = args[1].parse().unwrap();
    let mut count: i32 = 0;

    for i in 1..num+1 {
        //println!("{}", i);
        if i % 3 == 0 || i % 5 == 0 {
            count += i;
        }
    }
    println!("count: {}", count);

}
