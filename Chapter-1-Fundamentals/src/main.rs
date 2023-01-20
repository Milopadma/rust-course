// i32 type addition
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// generic types addition
fn add_generic<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// if statements
fn if_statements(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// loop
fn loop_print_5_times(s: &str) {
    let mut counter = 5;
    loop {
        println!("{} at {counter}", s);
        counter -= 1;
        if counter == 0 {
            break;
        }
    }
}

// for loop
fn for_loop_print_5_times(s: &str) {
    for i in 0..5 {
        println!("{} in {}", s, i);
    }
}

// while looop
fn while_loop_print_n_times(s: &str, n: i32) {
    let mut counter = n;
    while counter > 0 {
        println!("{} at {}", s, counter);
        counter -= 1;
    }
}

// match expressions
fn match_expr(s: &str) {
    match s {
        "hello" => println!("hello"),
        "world" => println!("world"),
        _ => println!("nothing"),
    }
}

// enums, q: what is an enum? a: an enum is a type that can be one of a few different variants, constnt
enum Color {
    Red,
    Green,
    Blue,
}

// using the enum
fn color_match(c: Color) {
    match c {
        Color::Red => println!("red"), // a variant
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}

// structs, a data type that contains multiple pieces of data
struct Point {
    x: i32, // a field
    y: i32,
}

struct Similar {
    a: u32,
    b: u32,
}

// using the struct
fn struct_use() {
    let p = Point { x: 1, y: 2 };
    let s = Similar { a: 1, b: 2 };

    println!("x: {}, y: {}", p.x, p.y);
    println!("a: {}, b: {}", s.a, s.b);
}

fn main() {
    println!("wow!");
    println!("this chapter is mostly the basics about data types, variables and functions...");

    // Variables
    let x = 5;
    let y = 2;

    // Data types
    let a: i32 = 5;
    let b: i32 = 2;
    let c: f64 = 5.0;
    let d: f64 = 2.0;

    // Functions
    println!("{} + {} = {}", x, y, add(x, y));
    println!("{} + {} = {}", c, d, add_generic(c, d));

    // if statements
    let e = if_statements(a, b);
    println!("{} is the biggest number", e);

    //loops, for loops
    loop_print_5_times("loop");
    for_loop_print_5_times("for loop");
    while_loop_print_n_times("while loop", 2);

    // match expressions
    match_expr("hello");

    // enums
    color_match(Color::Red);

    // structs
    struct_use();
}