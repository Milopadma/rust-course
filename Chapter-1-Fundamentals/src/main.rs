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
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}