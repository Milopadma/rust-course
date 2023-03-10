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
            Tile::Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
                println!("Colored brick: {:?}", brick)
            }
            // if its water tile
            Tile::Water(Pressure(p)) if p >= 10 => println!("High water pressure"),
            Tile::Water(Pressure(p)) if p <= 10 => println!("Water pressure: {:?}", p),
            // if its grass, dirt, sand,
            Tile::Grass | Tile::Dirt | Tile::Sand => println!("Ground tile"),
            // if its treasure
            Tile::Treasure(TreasureChest {
                content: TreasureItem::Gold,
                ..
            }) => {
                println!("Gold treasure")
            }
            _ => println!("Unknown tile"),
        }
    }
}

mod slices_activity {
    fn data() -> &'static [u64] {
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    }

    pub fn run() {
        let mut stream = data().chunks(2);
        for chunks in stream.clone() {
            // either using ifs
            if chunks.len() != 2 {
                println!("Odd chunk: {:?}", chunks);
            } else {
                // sum of the two elements
                println!("{:?}, ", chunks[0] + chunks[1]);
            }
            // or

            // utilizing the match
            match chunks {
                [a, b] => println!("{:?}, ", a + b),      // double
                [a] => println!("Odd chunk: {:?}", a),    // single
                _ => unreachable!("Should never be < 2"), // none
            }
        }
        println!("Done");
    }
}

// main
fn main() {
    typestates::run();
    slices_activity::run();
}
