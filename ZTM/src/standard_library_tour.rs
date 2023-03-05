mod comparison_operators {
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

    pub fn is_below(this: Floor, that: Floor) -> bool {
        this < that // partialOrd lets us do this
    }
}
