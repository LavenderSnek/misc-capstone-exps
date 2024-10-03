use rust_exps_capstone::rects::{Area, Point, Rectangle};

fn main() {
    let p = Point { x: 20, y: 40 };
    println!("{:?}", p); // debug printing

    let r = Rectangle {
        x: 10,
        y: 30,
        w: 200,
        h: 400,
    };
    println!("{:?}", r);

    r.bottom_border().dy(); // using Area trait of rectangle
}
