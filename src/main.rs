use rusty::{
    math2d::*,
    render::*,
};

fn main() {
    // Print text to the console
    println!("Hello World!");

    // const
    let tmp: bool = true;
    println!("foo = {tmp}");

    // mutable
    let mut bar: bool = tmp;
    println!("bar = {bar}");
    bar = false;
    println!("bar = {bar}");

    let rand: u8 = rand::random();
    println!("{}", rand);

    let mut printable: Point2 = Point2::new(1, 2);
    printable.x = 2;
    println!("{debug:?}", debug=printable);
    println!("{pretty:#?}", pretty=printable);
    println!("{display}", display=printable);
    printable.add(1, 2).add(1, 2);
    printable.print();

    for (i, x) in ["a", "b", "c"].iter().enumerate() {
        print!("{}", x);
        if i == 2 { println!(); }
    }

    let nested: Point3 = Point3::new(1, 2, 3);
    nested.print();
}
