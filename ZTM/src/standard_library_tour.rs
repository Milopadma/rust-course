mod comparing_enums {
    // comaparing enums

    #[derive(PartialEq, PartialOrd)]
    enum Floor {
        ClientServices,
        Marketing, // ensure same variant to compare values
        Ops,
    }

    pub fn run() {
        let floor1 = Floor::ClientServices;
        let floor2 = Floor::Marketing;
        // check if floor1 and floor2 are the same
        println!("{:?}", floor1 == floor2); //so we can compare the enums
        println!("{:?}", is_below(floor1, floor2)); //comparing enums
    }

    fn is_below(this: Floor, that: Floor) -> bool {
        this < that // partialOrd lets us do this
    }
}

mod comparing_structs {

    // manually implementing PartialOrd for struct Point
    // since derives only uses the first field
    impl PartialOrd for Point {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.x.cmp(&other.x))
        }
    }

    #[derive(PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    pub fn run() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 0, y: 0 };
        // check if p1 and p2 are the same
        println!("{:?}", p1 == p2); //so we can compare the structs
        println!("{:?}", is_below(p1, p2)); //comparing structs
    }

    fn is_below(this: Point, that: Point) -> bool {
        this.x < that.x && this.y < that.y
    }
}

mod operator_overloading {
    use std::ops::Add; // Add trait from ops module

    struct Speed(u32);

    impl Add<Self> for Speed {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Speed(self.0 + rhs.0) // new struct with the same fields as self
        }
    }

    pub fn run() {
        let fast = Speed(50) + Speed(30);
    }

    // the Index can also be overloaded
}

mod iterator_implementation {

    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Odd {
        number: isize,
        max: isize,
    }

    impl Iterator for Odd {
        type Item = isize;

        fn next(&mut self) -> Option<Self::Item> {
            self.number += 2;
            if self.number <= self.max {
                Some(self.number)
            } else {
                None
            }
        }
    }

    impl Odd {
        fn new(max: isize) -> Self {
            Odd { number: -1, max }
        }
    }

    pub fn run() {
        let mut odd = Odd::new(10);
        while let Some(num) = odd.next() {
            println!("{}", num); // this prints the odd numbers
        }
    }
}

fn main() {
    // comparing_enums::run();
    // comparing_structs::run();
    // operator_overloading::run();
    iterator_implementation::run();
}
