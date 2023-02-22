mod typestates {
    // the multiple states for luggage airport process
    struct CheckIn(LuggageID);
    struct OnLoading(LuggageID);
    struct Offloading(LuggageID);
    struct AwaitingPickup(LuggageID);
    struct EndCustody(LuggageID);

    // the luggage struct
    #[derive(Debug, Copy, Clone)]
    struct LuggageID(usize);
    #[derive(Debug)]
    struct Luggage(LuggageID);

    // the luggage struct with a method to change state
    impl Luggage {
        fn new(id: LuggageID) -> Self {
            Luggage(id)
        }

        fn check_in(self) -> CheckIn {
            CheckIn(self.0)
        }
    }

    // as these types will only be able to run said functions
    impl CheckIn {
        fn onload(self) -> OnLoading {
            OnLoading(self.0)
        }
    }

    impl OnLoading {
        fn offload(self) -> Offloading {
            Offloading(self.0)
        }
    }

    impl Offloading {
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
            .awaiting_pickup();
        let (luggage, _) = luggage.pickup();
        println!("Luggage: {:?}", luggage);
    }
}

mod match_guards_and_binding {
    #[derive(Debug)]
    enum TreasureItem {
        Gold,
        SuperPower,
    }

    // treasure chest contains treasure item type variants
    #[derive(Debug)]
    struct TreasureChest {
        content: TreasureItem,
        amount: usize,
    }

    // water struct has a pressure field
    #[derive(Debug)]
    struct Pressure(u16);

    // different types of bricks
    #[derive(Debug)]
    enum BrickStyle {
        Dungeon,
        Gray,
        Red,
    }

    #[derive(Debug)]
    enum Tile {
        Brick(BrickStyle),
        Dirt,
        Grass,
        Sand,
        Treasure(TreasureChest),
        Water(Pressure),
        Wood,
    }

    pub fn run() {
        let mut tile = Tile::Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: 100,
        });

        // match guard
        match tile {
            // if its a colored brick
            Tile::Brick(BrickStyle::Dungeon) => println!("Dungeon brick"),
            Tile::Brick(_) => println!("Brick"),
        }
    }
}

// main
fn main() {
    typestates::run();
}
