use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input your number");
        let mut guess = String::new();
        //&mut = mutable reference
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //Shadowing variable
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            //_ is a "catchall" value
            Err(_) => continue,
        };

        println!("You wrote: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
