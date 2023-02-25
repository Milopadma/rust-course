fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let add = Box::new(|a, b| a + b);
    let sub = Box::new(|a, b| a - b);
    let divide = Box::new(|a, b| if a <= 0 || b <= 0 { 0 } else { a / b });
    println!("{}", math(2, 2, add));
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 2, divide.clone()));
    println!("{}", math(2, 0, divide));
}
