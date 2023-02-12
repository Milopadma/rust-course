mod traits_activity {
    pub trait Calculate {
        fn calculate(&self) -> u32;
    }

    pub struct Square {
        pub side: u32,
    }

    impl Calculate for Square {
        fn calculate(&self) -> u32 {
            self.side * self.side
        }
    }

    pub struct Triangle {
        pub length: u32,
        pub width: u32,
    }

    impl Calculate for Triangle {
        fn calculate(&self) -> u32 {
            (self.length * self.width) / 2
        }
    }

    #[allow(dead_code)]
    fn print_area<T: Calculate>(shape: T) {
        println!("The area is {}", shape.calculate());
    }

    pub fn run() {
        let square = Square { side: 5 };
        let triangle = Triangle {
            length: 5,
            width: 5,
        };

        print_area(square);
        print_area(triangle);
    }
}

mod generic_functions_activity {
    #[derive(Debug)]
    enum ServicePriority {
        High,
        Standard,
    }

    trait Priority {
        fn get_priority(&self) -> ServicePriority;
    }

    #[derive(Debug)]
    struct ImportantGuest;
    impl Priority for ImportantGuest {
        fn get_priority(&self) -> ServicePriority {
            ServicePriority::High
        }
    }

    struct Guest;
    impl Priority for Guest {
        fn get_priority(&self) -> ServicePriority {
            ServicePriority::Standard
        }
    }

    fn print_guest_priority<T: Priority>(guest: T) {
        println!("Guest priority: {:?}", guest.get_priority());
    }

    pub fn run() {
        let guest = ImportantGuest;
        let guest2 = Guest;

        print_guest_priority(guest);
    }
}

fn main() {
    traits_activity::run(); 
    generic_functions_activity::run();
}
