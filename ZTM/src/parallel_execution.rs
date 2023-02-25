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

mod threads_activity {
    use std::thread;

    fn msg_hello() -> &'static str {
        use std::time::Duration;
        std::thread::sleep(Duration::from_millis(1000));
        "Hello. "
    }

    fn msg_thread() -> &'static str {
        use std::time::Duration;
        std::thread::sleep(Duration::from_millis(1000));
        "threads, "
    }

    fn msg_excited() -> &'static str {
        use std::time::Duration;
        std::thread::sleep(Duration::from_millis(1000));
        "!! "
    }

    pub fn spawn_threads() -> String {
        // spawns the threads
        let value_1 = thread::spawn(move || msg_hello());
        let value_2 = thread::spawn(move || msg_excited());
        let value_3 = thread::spawn(move || msg_thread());

        // get the data
        let cat = value_1.join().expect("hu poh");
        let cat2 = value_2.join().expect("hu poh");
        let cat3 = value_3.join().expect("hu poh");

        // println
        // println!("{}{}{}", cat, cat2, cat3);

        // concat and return
        cat.to_owned() + cat2 + cat3
    }

    pub fn run() {
        println!("{:?}", spawn_threads());
    }
}

fn main() {
    advanced_closures::run();
    threads::run();
    threads_activity::run();
    // threads_activity::spawn_threads();
}
