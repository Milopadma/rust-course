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

    // a show case of mono morphism
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

mod generic_structures_activity {
    trait Body {
        fn get_body(&self) -> String;
    }

    trait Color {
        fn get_color(&self) -> String;
    }

    #[derive(Debug)]
    struct Vehicle<T: Body, U: Color> {
        body: T,
        color: U,
    }

    // generic implementation for Vehicle struct
    impl<T: Body, U: Color> Vehicle<T, U> {
        fn new(body: T, color: U) -> Self {
            Vehicle { body, color }
        }
    }
    // trait implementation for body struct
    #[derive(Debug)]
    struct Car;
    impl Body for Car {
        fn get_body(&self) -> String {
            "car".to_string()
        }
    }

    #[derive(Debug)]
    struct Truck;
    impl Body for Truck {
        fn get_body(&self) -> String {
            "truck".to_string()
        }
    }

    // trait implementation for color struct
    #[derive(Debug)]
    struct Red;
    impl Color for Red {
        fn get_color(&self) -> String {
            "red".to_string()
        }
    }

    #[derive(Debug)]
    struct Blue;
    impl Color for Blue {
        fn get_color(&self) -> String {
            "blue".to_string()
        }
    }

    // trait implementation for Vehicle struct

    pub fn run() {
        let car = Vehicle::new(Car, Red);
        let truck = Vehicle::new(Truck, Blue);

        println!("car: {:?}", car);
        println!("truck: {:?}", truck);
    }
}

fn main() {
    traits_activity::run();
    generic_functions_activity::run();
    generic_structures_activity::run();
}
