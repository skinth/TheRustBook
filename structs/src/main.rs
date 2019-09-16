/*struct Point {
    x: u32,
    y: u32,
}

fn print_point(p: &Point) {
    println!("point ({}, {})", p.x, p.y);
}*/

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

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
