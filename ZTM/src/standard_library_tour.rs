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

mod into_interator_implementation {
    // since we need mutable access to structures to implement iteration over them,
    // we can instead turn the structure into an iterator so we dont have to
    // turn into mutable
    use std::iter::FromIterator;
    use std::iter::IntoIterator;
    use std::iter::Iterator;

    // the default IntoIterator trait
    // trait IntoIterator {
    //     type Item;
    //     type IntoIter;
    //     fn into_iter(self) -> Self::IntoIter;
    // }

    struct Friends {
        friends: Vec<String>,
    }

    // standard imlpementation for intoiter, consumes the struct
    // impl IntoIterator for Friends {
    //     type Item = String;
    //     type IntoIter = std::vec::IntoIter<Self::Item>;
    //     fn into_iter(self) -> Self::IntoIter {
    //         self.friends.into_iter() // turn the collection to a iterator without mutating
    //     }
    // }

    // intoiterator impl but using borrows and lifetimes to allow multiple
    // iterations in scope
    impl<'a> IntoIterator for &'a Friends {
        type Item = &'a String;
        type IntoIter = std::slice::Iter<'a, String>;
        fn into_iter(self) -> Self::IntoIter {
            self.friends.iter()
        }
    }

    pub fn run() {
        let friends = Friends {
            friends: vec!["John".to_string(), "Jane".to_string(), "Janni".to_string()],
        };
        for f in &friends {
            // now we can borrow
            println!("{}", f);
        }
    }
}

mod implementing_iterator_activity {

    struct Multiplier {
        factor: isize,
        per_iter: isize,
    }

    impl Multiplier {
        fn new() -> Self {
            Multiplier {
                factor: 0,
                per_iter: 1,
            }
        }
    }

    // a custom Iterator Implementation for Multiplier
    impl Iterator for Multiplier {
        type Item = isize;
        fn next(&mut self) -> Option<Self::Item> {
            self.factor += self.per_iter; // increase by 1 per iteration
            Some(self.factor)
        }
    }

    struct Game {
        score: u32,
        multiplier: Multiplier,
    }

    pub fn run() {
        let mut multiplier = Multiplier::new();

        println!("{:?}", multiplier.next());
        println!("{:?}", multiplier.next());
        println!("{:?}", multiplier.next());
        println!("{:?}", multiplier.next());
        println!("{:?}", multiplier.next());
    }
}

mod intermediary_struct_iterator {

    // the data structure that needs an intermediary/proxy
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    // act as a intermediary/proxy
    struct ColorIntoIter {
        color: Color,
        position: u8,
    }

    struct ColorIter<'a> {
        color: &'a Color, // borrow color
        position: u8,
    }

    // impl iter for ColorIntoIter
    impl Iterator for ColorIntoIter {
        type Item = u8; // since all the items are u8
        fn next(&mut self) -> Option<Self::Item> {
            let next = match self.position {
                0 => Some(self.color.r),
                1 => Some(self.color.g),
                2 => Some(self.color.b),
                _ => None,
            };

            self.position += 1;
            next // return the value wrapped in Option
        }
    }

    // impl into-iter for Color to use the into-iter to
    //turn the Color struct into a intermediary iterator struct
    impl IntoIterator for Color {
        type Item = u8;
        type IntoIter = ColorIntoIter; // the intermediary iterator struct

        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter {
                color: self,
                position: 0,
            }
        }
    }

    // // borrow
    // // impl the iter for mutable Color struct
    // impl<'a> Iterator for ColorIter<'a> {
    //     type Item = u8;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         let next = match self.position {
    //             0 => Some(self.color.r),
    //             1 => Some(self.color.g),
    //             2 => Some(self.color.b),
    //             _ => None,
    //         };
    //         self.position += 1;
    //         next
    //     }
    // }

    pub fn run() {
        let color = Color { r: 255, g: 0, b: 0 };
        for c in color {
            // implicitly call intoiter() on color
            println!("{}", c); // prints 255, 0, 0

        }
    }
}

fn main() {
    // comparing_enums::run();
    // comparing_structs::run();
    // operator_overloading::run();
    iterator_implementation::run();
    into_interator_implementation::run();
    implementing_iterator_activity::run();
    intermediary_struct_iterator::run();
}
