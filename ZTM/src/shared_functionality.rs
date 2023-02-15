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
    trait Body {}

    trait Color {}

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
    impl Body for Car {}

    #[derive(Debug)]
    struct Truck;
    impl Body for Truck {}

    // trait implementation for color struct
    #[derive(Debug)]
    struct Red;
    impl Color for Red {}

    #[derive(Debug)]
    struct Blue;
    impl Color for Blue {}

    // trait implementation for Vehicle struct

    pub fn run() {
        let car = Vehicle::new(Car, Red);
        let truck = Vehicle::new(Truck, Blue);

        println!("car: {:?}", car);
        println!("truck: {:?}", truck);
    }
}

mod trait_objects {

    trait MaterialCost {
        fn cost(&self, sqm: i32) -> u32;
    }

    struct Carpet(f64);

    impl MaterialCost for Carpet {
        fn cost(&self, sqm: i32) -> u32 {
            10 * sqm as u32
        }
    }

    struct Tile(f64);

    impl MaterialCost for Tile {
        fn cost(&self, sqm: i32) -> u32 {
            15 * sqm as u32
        }
    }

    struct Wood(f64);

    impl MaterialCost for Wood {
        fn cost(&self, sqm: i32) -> u32 {
            20 * sqm as u32
        }
    }

    pub fn run() {
        let carpet = Carpet(10.0);
        let tile = Tile(10.0);
        let wood = Wood(10.0);

        // a vector of dynamic trait objects Boxed up.
        let materials: Vec<Box<dyn MaterialCost>> =
            vec![Box::new(carpet), Box::new(tile), Box::new(wood)];

        for material in materials {
            println!("Cost: {}", material.cost(10));
        }
    }
}

mod lifetimes_activity {

    const MOCK_DATA: &'static str = include_str!("mock_data.txt");

    // the mock data is never copied
    #[derive(Debug)]
    pub struct Names<'a> {
        inner: Vec<&'a str>,
    }

    #[derive(Debug)]
    pub struct Titles<'a> {
        inner: Vec<&'a str>,
    }

    pub fn read() -> (Names<'static>, Titles<'static>) {
        let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect::<Vec<&str>>();
        let names: Vec<_> = data
            .iter()
            .filter_map(|line| line.split(',').nth(1))
            .collect();

        let names = Names { inner: names };

        let titles: Vec<_> = data
            .iter()
            .filter_map(|line| line.split(',').nth(4))
            .collect();

        let titles = Titles { inner: titles };

        (names, titles)
    }
}

fn main() {
    traits_activity::run();
    generic_functions_activity::run();
    generic_structures_activity::run();
    trait_objects::run();
    let (names, titles) = lifetimes_activity::read();
    println!("names: {:?}, titles: {:?}", names, titles);
}
