mod typestates {
    // the multiple states for luggage airport process
    struct CheckIn(LuggageID);
    struct OnLoading(LuggageID);
    struct Offloadin(LuggageID);
    struct AwaitingPickup(LuggageID);
    struct EndCustody(LuggageID);

    // the luggage struct
    struct LuggageID(usize);
    struct Luggage(LuggageID);

    // the luggage struct with a method to change state
    impl Luggage {
        fn new(id: LuggageID) -> Self {
            Luggage(id)
        }

        fn check_in(self) -> Luggage<CheckIn> {
            Luggage(CheckIn(self.0))
        }
    }

    // as these types will only be able to run said functions
    impl CheckIn {
        fn onload(self) -> OnLoad {
            OnLoad(self.0)
        }
    }

    impl OnLoading {
        fn offload(self) -> Offload {
            Offload(self.0)
        }
    }

    impl Offload {
        fn awaiting_pickup(self) -> AwaitingPickup {
            AwaitingPickup(self.0)
        }
    }

    impl AwaitingPickup {
        fn pickup(self) -> (Luggage, EndCustody) {
            (Luggage(self.0), EndCustody(self.0))
        }
    }

    // run
    pub fn run() {
        let id = LuggageID(1);
        // we did all that so we can do this daisy chain
        let luggage = Luggage::new(id)
            .check_in()
            .onload()
            .offload()
            .awaiting_pickup()
            .pickup();
        println!("Luggage state: {:?}", luggage.state);
    }
}

// main
fn main() {
    typestates::run();
}
