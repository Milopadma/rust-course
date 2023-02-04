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
    if a > b {
        a
    } else {
        b
    }
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

enum Drinks {
    // enumerations, variants of this data type
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

struct Drink {
    // drink struct, shows what types of data this struct can hold
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
    println!(
        "{}, {}, {}",
        d.price,
        d.size,
        match d.drink_type {
            // match returns a string  implicitly
            Drinks::Cola => "Cola",
            Drinks::Sprite => "Sprite",
            Drinks::Fanta => "Fanta",
            Drinks::Milo => "Milo",
        }
    );
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
        shipping_box.width, shipping_box.height, shipping_box.depth
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

mod ownership_activity {
    pub struct GroceryItem {
        pub quantity: u32,
        pub id: u32,
    }

    pub fn show_quantity(item: &GroceryItem) {
        println!("quantity: {}", item.quantity);
    }

    pub fn show_id(item: &GroceryItem) {
        println!("id: {}", item.id);
    }
}

mod data_collections {
    #[derive(Debug)]
    pub enum Drinks {
        Cola,
        Sprite,
        Fanta,
        Milo,
    }

    #[derive(Debug)]
    pub struct Drink {
        // drink struct, shows what types of data this struct can hold
        price: f64,
        size: u32,
        drink_type: Drinks,
    }

    // implements the Drink struct with the new function
    impl Drink {
        pub fn new(price: f64, size: u32) -> Drink {
            Drink {
                price,
                size,
                drink_type: Drinks::Cola,
            }
        }

        // self is a reference to the struct that this impl is for
        pub fn set_drink_type(&mut self, drink_type: Drinks) {
            self.drink_type = drink_type;
        }

        pub fn get_drink_type(&self) -> &Drinks {
            &self.drink_type
        }
    }

    // vectors
    pub fn vectors() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(6);
        v.push(7);
        v.push(8);
        v.push(9);
        v.push(10);

        println!("v: {:?}", v);
        v.pop(); // remove last element
        println!("v: {:?}", v);
        v.remove(0); // remove element at index 0
        println!("v: {:?}", v);
    }

    struct Test {
        score: i32,
    }

    pub fn show_tests() {
        let Scores = vec![Test { score: 1 }, Test { score: 2 }];

        for tests in Scores {
            println!("score: {}", tests.score);
        }
    }
}

mod data_collections_activity {
    #[derive(Debug)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(Debug)]
    pub struct Dimensions {
        pub width: u32,
        pub height: u32,
        pub weight: u32,
    }

    #[derive(Debug)]
    pub struct ShippingBox {
        pub dimensions: Dimensions,
        pub color: Color,
    }

    // implements functions for the Shipping_Box struct, the problem domain is shipping boxes
    impl ShippingBox {
        // create a new shipping box
        pub fn new(width: u32, height: u32, weight: u32, color: Color) -> Self {
            Self {
                dimensions: Dimensions {
                    width,
                    height,
                    weight,
                },
                color,
            }
        }

        // getters and setters lmao
        pub fn set_color(&mut self, color: Color) {
            self.color = color;
        }

        pub fn get_color(&self) -> &Color {
            &self.color
        }

        // the print method implementation
        pub fn print_box(&self) {
            println!(
                "width: {}, height: {}, weight: {}, color: {:?}",
                self.dimensions.width, self.dimensions.height, self.dimensions.weight, self.color
            );
        }
    }
}

mod vectors_activity {
    pub fn run() {
        let vect = vec![10, 20, 30, 40];
        for i in &vect {
            match i {
                10 => println!("10"),
                20 => println!("20"),
                30 => println!("thirty"),
                40 => println!("40"),
                _ => println!("not found"),
            }
        }
        println!("vect length: {:?}", vect.len());
    }
}

// strings and &str
// String is a heap allocated data structure,
// &str is a slice of a string that is stored on the stack

mod strings_activity {
    struct Person {
        name: String,
        age: u32,
        color: String,
    }

    pub fn run() {
        let person_1 = Person {
            name: String::from("John"),
            age: 12,
            color: String::from("blue"),
        };

        let person_2 = Person {
            name: String::from("Jane"),
            age: 9,
            color: String::from("blue"),
        };

        let person_3 = Person {
            name: String::from("Janni"),
            age: 10,
            color: String::from("red"),
        };

        let vect = vec![person_1, person_2, person_3];

        for person in &vect {
            if person.age <= 10 {
                println!("{} {} {}", person.name, person.age, person.color);
            }
        }
    }
}

mod advanced_match_activity {
    // for when some variants have different types of data
    #[derive(Debug)]
    enum Ticket {
        Standard(f64),
        Vip(f64, String),
        Backstage(f64, String),
    }

    pub fn run() {
        let vect = vec![
            Ticket::Standard(10.0),
            Ticket::Vip(20.0, String::from("John")),
            Ticket::Backstage(30.0, String::from("Jonnie")),
        ];
        for ticket in vect {
            match ticket {
                Ticket::Standard(price) => println!("Standard ticket, price: {}", price),
                Ticket::Vip(price, name) => {
                    println!("VIP ticket, price: {}, name: {}", price, name);
                }
                Ticket::Backstage(price, name) => {
                    println!("Backstage ticket, price: {}, name: {}", price, name);
                }
            }
        }
    }
}

mod options_activity {
    struct Student {
        name: String,
        locker: Option<u32>, // some students dont have a locker
    }

    /// this function shows how to use the Option enum
    pub fn run() {
        let vect = vec![
            Student {
                name: String::from("John"),
                /// turns the string slice into a String
                locker: Some(1),
            },
            Student {
                name: String::from("Jane"),
                locker: None,
            },
            Student {
                name: String::from("Janni"),
                locker: Some(2),
            },
        ];

        for student in vect {
            match student.locker {
                Some(locker) => println!("{} has a locker at {}", student.name, locker),
                None => println!("{} has no locker", student.name),
            }
        }
    }
}

mod standard_library_activity {
    pub fn print_string(str: &str) {
        println!("{}", str.to_lowercase());
        println!("{}", str.to_uppercase());
    }
}

mod result_enum_activity {
    struct Customer {
        name: String,
        age: u32,
    }

    fn can_purchase(customer: &Customer) -> Result<(String), String> {
        if customer.age >= 18 {
            Ok(String::from("Customer is old enough"))
        } else {
            Err(String::from("Customer is not old enough"))
        }
    }

    pub fn run() {
        let customer_1 = Customer {
            name: String::from("John"),
            age: 12,
        };

        let customer_2 = Customer {
            name: String::from("Jane"),
            age: 20,
        };

        println! {"{:?}, {:?}", customer_1.name, can_purchase(&customer_1).unwrap_err()
        }
        println! {"{:?}, {:?}", customer_2.name, can_purchase(&customer_2).unwrap()
        }
    }
}

mod results_question_mark_activity {
    enum EmployeeType {
        MaintainanceCrew,
        MarketingDepartment,
        Manager,
        LineSupervisor,
        KitchenStaff,
        AssemblyTechnician,
    }

    enum EmployeeStatus {
        Employed,
        Unemployed,
    }

    struct Employee {
        name: String,
        is_employed: EmployeeStatus,
        employee_type: EmployeeType,
    }

    fn can_enter_building(employee: &Employee) -> Result<&Employee, String> {
        // early return if the employee is not employed
        match employee.is_employed {
            EmployeeStatus::Unemployed => return Err(String::from("Employee cannot enter")),
            _ => (), // if the employee is employed, continue
        };
        // if the employee is employed, check if they are of the correct type
        match employee.employee_type {
            EmployeeType::MaintainanceCrew => Ok(employee),
            EmployeeType::MarketingDepartment => Ok(employee),
            EmployeeType::Manager => Ok(employee),
            _ => Err(String::from("Employee cannot enter")),
        }
    }

    fn try_access(employee: &Employee) -> Result<(), String> {
        let result: &Employee = can_enter_building(employee)?; // this is the same as can_enter_building(employee).unwrap(), as it may return an error, it will return the error, otherwise it will return the value of the Ok variant
                                                               // returns the value of the Ok variant
        println!("Employee {:?} can enter", result.name);
        Ok(())

        // in this case, the Err variant is never printed
    }
    pub fn run() {
        let employee_1 = Employee {
            name: String::from("John"),
            employee_type: EmployeeType::MaintainanceCrew,
            is_employed: EmployeeStatus::Employed,
        };

        let employee_2 = Employee {
            name: String::from("Jane"),
            employee_type: EmployeeType::MarketingDepartment,
            is_employed: EmployeeStatus::Unemployed,
        };

        let employee_3 = Employee {
            name: String::from("Janni"),
            employee_type: EmployeeType::KitchenStaff,
            is_employed: EmployeeStatus::Employed,
        };

        try_access(&employee_1).unwrap();
        try_access(&employee_2).unwrap_err();
        try_access(&employee_3).unwrap_err();
    }
}

mod hash_maps_activity {
    enum FurnitureType {
        Chair,
        Table,
        Couch,
        Bed,
    }
    struct Furniture {
        name: String,
        amount: i32,
        price: f64,
        furniture_type: FurnitureType,
    }

    pub fn run() {
        let mut stock: std::collections::HashMap<i32, Furniture> = std::collections::HashMap::new();

        stock.insert(
            1,
            Furniture {
                name: String::from("Chair"),
                amount: 5,
                price: 10.0,
                furniture_type: FurnitureType::Chair,
            },
        );

        stock.insert(
            2,
            Furniture {
                name: String::from("Table"),
                amount: 2,
                price: 20.0,
                furniture_type: FurnitureType::Table,
            },
        );

        stock.insert(
            3,
            Furniture {
                name: String::from("Couch"),
                amount: 0,
                price: 50.0,
                furniture_type: FurnitureType::Couch,
            },
        );

        stock.insert(
            4,
            Furniture {
                name: String::from("Bed"),
                amount: 3,
                price: 100.0,
                furniture_type: FurnitureType::Bed,
            },
        );

        // print the amount of each furniture
        for (key, value) in &stock {
            if value.amount == 0 {
                println!("{} is out of stock", value.name);
            } else {
                println!("key-{} {}: {}", key, value.name, value.amount);
            }
        }

        // prints the total furniture stock
        println!(
            "total stock: {}",
            // takes the values of the hashmap, and for each value, adds the amount to the accumulator
            stock
                .values()
                .fold(0, |accumulator, furniture| accumulator + furniture.amount)
        );
    }
}

mod map_combinator_activity {
    #[derive(Debug)]
    struct User {
        user_id: i32,
        name: String,
    }

    fn find_user(name: &str) -> Option<i32> {
        let name = name.to_lowercase();
        match name.as_str() {
            "sam" => Some(1),
            "matt" => Some(5),
            "katie" => Some(9),
            _ => None,
        }
    }

    pub fn run() {
        // if the user is found, create a user struct
        let user = find_user("sam").map(|user_id| User {
            user_id,
            name: String::from("sam"),
        });

        // print the user if the struct exists
        match user {
            Some(user) => println!("user: {:?}", user),
            None => println!("user not found"),
        }
    }
}

mod option_combinator_activity {
    #[derive(Debug)]
    pub enum Access {
        Admin,
        User,
        Guest,
    }

    fn maybe_access(name: &str) -> Option<Access> {
        let name = name.to_lowercase();
        match name.as_str() {
            "sam" => Some(Access::Admin),
            "matt" => Some(Access::User),
            "katie" => Some(Access::Guest),
            _ => None,
        }
    }

    fn root() -> Option<Access> {
        Some(Access::Admin)
    }

    pub fn part_1() -> bool {
        maybe_access("sam").is_some()
    }

    pub fn part_2() -> Option<Access> {
        // returns the Option enum if the value is Some, otherwise it returns the value of the or_else function
        maybe_access("sam").or_else(|| maybe_access("matt"))
    }

    pub fn part_3() -> Access {
        // unwraps the Option enum and returns the value of the Some variant,
        // or just return Guest if the value is None
        maybe_access("alice").unwrap_or(Access::Guest)
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

    // ownership activity
    let item = ownership_activity::GroceryItem { quantity: 5, id: 2 };
    ownership_activity::show_quantity(&item);
    ownership_activity::show_id(&item);

    // data collections
    let mut drink = data_collections::Drink::new(1.0, 500);
    drink.set_drink_type(data_collections::Drinks::Milo);
    println!("drink type: {:?}", drink.get_drink_type());

    // data collections activity
    let mut box1 =
        data_collections_activity::ShippingBox::new(1, 2, 3, data_collections_activity::Color::Red);
    box1.print_box();
    box1.set_color(data_collections_activity::Color::Green);
    box1.print_box();

    // vectors
    data_collections::vectors();
    data_collections::show_tests();

    // vectors activity
    vectors_activity::run();

    // strings and &str
    strings_activity::run();

    // advanced match activity
    advanced_match_activity::run();

    // options activity
    options_activity::run();

    // standard library activity
    standard_library_activity::print_string("Hello World!");

    // result enum activity
    result_enum_activity::run();

    // results question mark activity
    results_question_mark_activity::run();

    // hash maps activity
    hash_maps_activity::run();

    // map combinator activity
    map_combinator_activity::run();

    // option combinator activity
    println!("part 1: {}", option_combinator_activity::part_1());
    println!("part 2: {:?}", option_combinator_activity::part_2());
    println!("part 3: {:?}", option_combinator_activity::part_3());
}
