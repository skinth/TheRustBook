fn main() {
    let mut x = 12;

    println!("Value of x: {}", x);

    x = 44;

    println!("Value of x: {}", x);

    //destructuring compund variable
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The same value y: {}", tup.1);

    let a = [1, 2, 3, 4, 5];
    let mut ix = 0;
    loop {
        if (ix < 5) {
            println!("a[{}]: {}", ix, a[ix]);
            ix = ix +1;
        } else {
            break;
        }
    }

}
