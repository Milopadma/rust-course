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
