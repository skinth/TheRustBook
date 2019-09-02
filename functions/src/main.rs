fn main() {
    println!("Hello, world!");

    another_func(12345);

    let exp = {
        let a = 22;
        a + 44
    };

    println!("I am an expression: {}", exp);
}

fn another_func(x: i32) {
    println!("I am another function! {}", x);
}