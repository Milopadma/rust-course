mod advanced_closures {
    fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
        op(a, b)
    }

    pub fn run() {
        let add = Box::new(|a, b| a + b);
        let sub = Box::new(|a, b| a - b);
        let divide = Box::new(|a, b| if a <= 0 || b <= 0 { 0 } else { a / b });
        println!("{}", math(2, 2, add));
        println!("{}", math(2, 2, sub));
        println!("{}", math(2, 2, divide.clone()));
        println!("{}", math(2, 0, divide));
    }
}

mod threads {
    use std::{
        thread::{self, JoinHandle},
        time::Duration,
    };

    pub fn run() {
        let iterations = 10;
        let data = vec!['a', 'b', 'c'];
        let caps = thread::spawn(move || {
            let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
            data
        });
        let a = thread::spawn(move || {
            for i in 1..=iterations {
                println!("A:{}", i);
            }
        });
        let b = thread::spawn(move || {
            for i in 1..=iterations {
                println!("B:{}", i);
            }
        });

        // thread that returns a value
        let value: JoinHandle<i32> = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            42
        });

        // joins returns a Result type
        a.join();
        b.join();
        println!("{:?}", value.join());
        println!("{:?}", caps.join());
    }
}

fn main() {
    advanced_closures::run();
    threads::run();
}
