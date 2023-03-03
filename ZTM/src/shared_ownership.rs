mod shared_mutability {
    // interior mutability is a feature of Rust that allows you to
    // * mutate data in a thread-safe way
    use std::cell::Cell;

    #[derive(Debug)]
    struct Book {
        signed: Cell<bool>,
    }

    impl Book {
        fn sign(&self) {
            self.signed.set(true);
        }

        fn signed(&self) -> bool {
            self.signed.get()
        }
    }

    pub fn run() {
        let my_book = Book {
            signed: Cell::new(false),
        };

        println!("{:?}", my_book.signed()); // this returns false
        my_book.sign();
        println!("{:?}", my_book.signed()); // this returns true
    }
}

mod shared_mutability_ref_cells {
    use std::cell::RefCell; // a refcell is a cell but with a mutable borrow instead of a
                            // normal mutate

    struct Person {
        name: RefCell<String>,
    }

    pub fn run() {
        let name = String::from("John");
        let person = Person {
            name: RefCell::new(name),
        };

        // using scopes to make these two scopes work
        {
            // mutably borrow the persons name
            let mut name = person.name.borrow_mut();
            *name = String::from("Jane"); // dereference
        }
        // drop first scope
        {
            // OR use replace()
            person.name.replace("peepoo".to_owned());
        }


        // OR use do a checked borrow using Result
        let new_name: Result<_, _> = person.name.try_borrow();
        match new_name {
            Ok(name) => {
                println!("{}", name);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        // 
        let new_name_mut: Result<_, _> = person.name.try_borrow_mut();
        match new_name_mut {
            Ok(name) => {
                println!("{}", name);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

    }
}

fn main() {
    shared_mutability::run();
}
