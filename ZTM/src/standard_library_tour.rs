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
    #[derive(PartialEq, PartialOrd)]
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
