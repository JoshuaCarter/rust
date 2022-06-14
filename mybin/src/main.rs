// mylib is the name of our lib as defined by Cargo.toml
use mylib::{
    math2d::*,
    render::*,
    math2d::utils::hello_world::hello_world,
};

// funcs with type restriction
fn print_a<F>(f: F) where F: Print {
    f.print();
}
fn print_b<F: Print>(f: F) {
    f.print();
}
fn print_c<F: Print>(f: F) -> impl Fn() {
    move || f.print()
}

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
    print_a(printable);
    print_b(printable);
    print_c(printable)();

    for (i, x) in ["a", "b", "c"].iter().enumerate() {
        print!("{}", x);
        if i == 2 { println!(); }
    }

    let nested: Point3 = Point3::new(1, 2, 3);
    nested.print();
}
