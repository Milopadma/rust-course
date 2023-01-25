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

enum Drinks { // enumerations, variants of this data type
    Cola,
    Sprite,
    Fanta,
    Milo,
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
// all fields must be filled in when creating an instance of a struct
struct Point {
    x: i32, // a field
    y: i32,
}

struct Similar {
    a: u32,
    b: u32,
}

struct ShippingBox {
    width: u32,
    height: u32,
    depth: u32,
}

struct Drink { // drink struct, shows what types of data this struct can hold
    price: f64,
    size: u32,
    drink_type: Drinks,
}

// tuples, they are a type of record, store data anonymously and no names are required
// useful for returning multiple values from a function
fn to_tuple(vector: &Vec<i32>) -> (i32, i32) {
    let x = vector[0];
    let y = vector[1];
    (x, y)
}

fn cartesian_check(x: i32, y: i32) -> (i32, i32) {
    if y > 5 {
        println!("greater than 5");
        return (x, y);
    } else if y < 5 {
        println!("less than 5");
        return (x, y);
    } else {
        println!("equal to 5");
        return (x, y);
    }
}

// print out the Drink
fn print_drink(d: Drink) {
    println!("{}, {}, {}", d.price, d.size, match d.drink_type {
        // match returns a string  implicitly
        Drinks::Cola => "Cola",
        Drinks::Sprite => "Sprite",
        Drinks::Fanta => "Fanta",
        Drinks::Milo => "Milo",
    });
}

// using the struct
fn struct_use() {
    let p = Point { x: 1, y: 2 };
    let s = Similar { a: 1, b: 2 };
    let shipping_box = ShippingBox {
        width: 1,
        height: 2,
        depth: 3,
    };

    println!("x: {}, y: {}", p.x, p.y);
    println!("a: {}, b: {}", s.a, s.b);
    println!(
        "width: {}, height: {}, depth: {}",
        shipping_box.width,
        shipping_box.height,
        shipping_box.depth
    );
}
mod expressions {
    // expressions
    pub fn expressions_example() {
        let num = 3;
        let new_num = if num == 3 { 5 } else { 6 };

        // match example
        let x = 5;
        let y = match x {
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            _ => 6,
        };

        println!("new_num: {}, y: {}", new_num, y);
    }

    // simulation of a access level protocol
    pub enum Access {
        Admin,
        User,
        Guest,
    }

    pub fn access_level(a: Access) -> bool {
        let access_level = a;
        let can_access_file = match access_level {
            Access::Admin => true,
            _ => false,
        };

        can_access_file
    }

    // expressions activity
    pub fn expressions_activity(value: i32) {
        match value {
            // return true if value is above 100
            x if x > 100 => println!("true"),
            x if x < 100 => println!("false"),
            _ => println!("nothing"),
        }
    }
}

// memory and allocation - ownership
mod ownership_example {
    enum Light {
        Bright,
        Dull,
    }

    fn display_light(light: &Light) {
        // use borrows &
        match light {
            Light::Bright => println!("bright"),
            Light::Dull => println!("dull"),
        }
    }

    pub fn run() {
        let light = Light::Bright;
        // display_light(light);
        // display_light(light); // error, light has been moved

        // use borrows
        let light = Light::Bright; // light is owned by this fn
        display_light(&light); // borrow
        display_light(&light); // borrow
    }
}

mod ownership_example_2 {
    struct Book {
        pages: u32,
        rating: u32,
    }

    fn display_page_count(book: &Book) {
        println!("page count: {}", book.pages);
    }

    fn display_rating(book: &Book) {
        println!("rating: {}", book.rating);
    }

    pub fn run() {
        let book = Book {
            pages: 100,
            rating: 5,
        };

        display_page_count(&book);
        display_rating(&book);
    }
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

    // drinks activity
    let a_drink = Drink {
        price: 1.0,
        size: 500,
        drink_type: Drinks::Cola,
    };
    let b_drink = Drink {
        price: 1.0,
        size: 500,
        drink_type: Drinks::Milo,
    };

    print_drink(a_drink);
    print_drink(b_drink);

    // tuples
    let vector = vec![1, 2];
    let tuple = to_tuple(&vector);
    let (x, y) = to_tuple(&vector);
    println!("x: {}, y: {}", tuple.0, tuple.1); // one way
    println!("x: {}, y: {}", x, y); // better way

    // cartesian check
    let (x, y) = cartesian_check(1, 5);
    println!("x: {}, y: {}", x, y);

    // access level
    let access = expressions::Access::Admin;
    let can_access = expressions::access_level(access);
    println!("can access: {}", can_access);

    // expressions
    expressions::expressions_example();
    expressions::expressions_activity(32);

    // memory and allocation
    ownership_example::run();
    ownership_example_2::run();
}