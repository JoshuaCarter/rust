// rusty is the name of our lib (and bin) as defined by Cargo.toml
use rusty::{
    math2d::*,
    render::*,
    math2d::utils::hello_world::hello_world,
};

fn main() {
    hello_world();

    // const
    let tmp: bool = true;
    println!("foo = {tmp}");

    // mutable
    let mut bar: bool = tmp;
    println!("bar = {bar}");
    bar = false;
    println!("bar = {bar}");

    // use module via full path (package dependency in Cargo.toml)
    println!("rand {}", rand::random::<u8>());

    let mut printable: Point2 = Point2::new(1, 2);
    printable.x = 2;
    // all things can debug print
    println!("{debug:?}", debug=printable);
    // all things can pretty-debug print
    println!("{pretty:#?}", pretty=printable);
    // things that impl the Display trait can display print
    println!("{display}", display=printable);
    printable.add(1, 2).add(1, 2);
    // custom trait to shothand display printing
    printable.print();

    for (i, x) in ["a", "b", "c"].iter().enumerate() {
        print!("{}", x);
        if i == 2 { println!(); }
    }

    let nested: Point3 = Point3::new(1, 2, 3);
    nested.print();
}
