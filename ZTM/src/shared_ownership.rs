mod shared_mutability {
    // interior mutability is a feature of Rust that allows you to
    // * mutate data in a thread-safe way
    use std::cell::Cell;

    #[derive(Debug)]
    struct Book {
        signed: Cell<bool>,
    }

    pub fn run() {
        let my_book = Book {
            signed: Cell::new(false),
        };

        println!("{:?}", my_book.signed()); // this returns false
        my_book.sign();
        println!("{:?}", my_book.signed()); // this returns true
    }

    impl Book {
        fn sign(&self) {
            self.signed.set(true);
        }

        fn signed(&self) -> bool {
            self.signed.get()
        }
    }
}

fn main() {
    shared_mutability::run();
}
