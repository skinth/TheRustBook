use std::io;

fn main() {
    let mut nth = String::new();
    println!("Input a number");
    io::stdin().read_line(&mut nth)
        .expect("Failed to read the number!");
    let nth = match nth.trim().parse::<u64>() {
        Ok(num) => num,
        Err(_) => 1,
    };
    println!("Fib({}): {}",nth, fibonacci(nth));
}

fn fibonacci(n: u64) -> u64 {
    let mut a:u64 = 0;
    let mut b:u64 = 1;
    let mut res:u64 = a+b;

    for i in 0..n {
        a = b;
        b = res;
        res = a + b;
    }

    return res;
}