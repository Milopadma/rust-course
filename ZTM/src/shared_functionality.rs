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

    pub fn print_area<T: Calculate>(shape: T) {
        println!("The area is {}", shape.calculate());
    }
}

fn main() {
    let square = traits_activity::Square { side: 5 };
    let triangle = traits_activity::Triangle {
        length: 5,
        width: 5,
    };

    traits_activity::print_area(square);
    traits_activity::print_area(triangle);
}
