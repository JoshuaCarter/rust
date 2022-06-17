// mylib is the name of our lib as defined by Cargo.toml
use mylib::{
    math2d::*,
    render::*,
    math2d::utils::hello_world::hello_world,
};
use futures::executor::block_on;
use std::{thread, ops::Add, str::FromStr};

// macro example
macro_rules! join {
    ($threads:ident) => {
        for t in $threads {
            let _ = t.join();
        }
    };
}

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```no_run
/// use self::add;
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
#[allow(dead_code)]
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn print_a<F>(f: F) where F: Print {
    f.print();
}
fn print_b<F: Print>(f: F) {
    f.print();
}
fn print_c<F: Print>(f: F) -> impl Fn() {
    move || f.print()
}
fn print_d(f: impl Print) {
    f.print();
}

fn print_os() {
    if cfg!(target_os = "linux") {
        println!("You are running linux!");
    }
    else if cfg!(target_os = "windows") {
        println!("You are running windows!");
    }
    else if cfg!(target_os = "macos") {
        println!("You are running macos!");
    }
    else {
        println!("Youre OS is unknown!");
    }
}

async fn async_nested() {
    println!("async nested");
}
async fn async_main(print_me: String) {
    async_nested().await; // doesn't block main thread
    println!("async {}", print_me);
}

fn main() {
    hello_world();
    print_os();

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
    print_d(printable);

    for (i, x) in ["a", "b", "c"].iter().enumerate() {
        print!("{}", x);
        if i == 2 { println!(); }
    }

    let nested: Point3 = Point3::new(1, 2, 3);
    nested.print();

    // threading
    let mut threads = Vec::new();
    for i in 0..10 {
        threads.push(thread::spawn(move || {
            println!("Thread {}", i);
        }));
    }

    // our custom macro expands into
    // for t in threads {
    //     let _ = t.join();
    // }
    join!(threads);

    // async/await
    let txt = String::from_str("yoyo").unwrap();
    let promise = async_main(txt);
    block_on(promise); // blocks current thread
}
