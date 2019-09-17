/*struct Point {
    x: u32,
    y: u32,
}

fn print_point(p: &Point) {
    println!("point ({}, {})", p.x, p.y);
}*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }

    //associated function 
    //(this is not a method! A method must have the self parameter)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    /*let p: Point = Point {
        x: 4,
        y: 5,
    };
    let q = Point {
        x: 2,
        ..p
    };
    print_point(&p);
    print_point(&q);*/

    let r = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        r.area()
    );
    println!("r: {:?}", r);
    println!("square: {:?}", Rectangle::square(12));
}