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
}

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