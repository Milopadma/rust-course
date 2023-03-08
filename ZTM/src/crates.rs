mod read_env {
    use dotenv::dotenv;
    use std::env;

    pub fn run() {
        // load the .env file
        dotenv().ok();

        let port = match env::var("PORT") {
            Ok(port) => port,
            Err(_) => "8080".to_string(),
        };
    }
}

mod serde_crate {
    use serde::{Deserialize, Serialize};
    // serialize the data structure from memory to a flat representation (file)
    // deserialize the data structure from a flat representation (file) to memory

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Person {
        name: String,
    }

    pub fn run() {
        let person = Person {
            name: "John Doe".to_string(),
        };

        let serialized = save_to_file(person);
        let person_from_file = read_from_file();
        println!("{:?}", person_from_file);
    }

    fn save_to_file(person: Person) -> Serialize {
        let serialized = serde_json::to_string(&person).expect("Failed to serialize");
        println!("{}", serialized);
    }

    fn read_from_file() -> Person {
        let deserialize: Result<Form, _> = serde_json::from_str(&serialized);

        deserialize
    }
}

mod rand_crate {
    use rand::{
        distributions::{Distribution, Uniform},
        seq::{IteratorRandom, SliceRandom},
        thread_rng, Rng,
    };

    pub fn run() {
        let mut rng = thread_rng();
        let number = rng.gen_range(0..10);
        let mut array = vec![];
        for _ in 0..number {
            array.push(rng.gen_range(0..10));
        }

        println!("{:?}", array.iter().choose(&mut rng));

        let mut derefList = *array;
        println!("{:?}", derefList.shuffle(&mut rng));

        let range = Uniform::from(5.500);
        let mut rng = thread_rng();
        println!("{:?}", range.sample(&mut rng));
    }
}

mod cached_crate {
    use cached::proc_macro::cached;
    use std::thread;
    use std::time::Duration;

    #[cached(size = 10, time = 30)]
    fn expensive(n: usize) -> usize {
        thread::sleep(Duration::from_millis(500));
        match n {
            1 => 1,
            _ => n,
        }
    }
}

mod regex_crate {
    use cached::proc_macro::cached;
    use regex::Regex;

    #[cached]
    pub fn run() -> Regex {
        // match ISO 8601 DATEs
        const re: &'static str = r"\d{4}-\d{2}-\d{2}";

        Regex::new(re).expect("Failed to compile regex")
    }
}

mod chrono_crate {
    use chrono::prelude::*;

    pub fn run() {
        let now = Utc::from(Utc::now());
        println!("{:?}", now);
    }
}

mod strum_crate {
    use strum::{EnumCount, IntoEnumIterator}; //string enum

    #[derive(Debug, EnumCount, EnumIter)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    impl Color {
        fn print_mul(&self) {
            for variant in Color::iter() {
                println!("{:?}", self::COUNT);
            }
        }
    }
}

mod derive_more {

    use derive_more::Display;
    use derive_more::From;

    #[derive(Display, fmt = "Item: {}, Quantity: {}", item, qty)]
    struct Order {
        item: String,
        qty: usize,
    }

    #[derive(From)]
    struct Person {
        name: String,
    }

    pub fn run() {
        let order = Order {
            item: "item".to_string(),
            qty: 1,
        };
    }
}

mod rayon_crate {
    use rayon::prelude::*; // ez parallel exec

    pub fn run() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let numbers = numbers.par_iter().map(|x| x * 2).collect::<Vec<_>>();
    }
}
