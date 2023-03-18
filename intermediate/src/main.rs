mod closures {
    pub fn run() {
        let add = |x: i32, y: i32| x + y;
    }
}

mod closures_iterators_activity {
    // Yes, yes, we know. It's an exercise, compiler, we want it that way!
    #[allow(unused_mut)]

    pub fn run() {
        // 1. Uncomment the code below. Create a closure that returns the square of an integer (the
        // number multiplied by itself), and assign the closure to the "square" variable. Then run the
        // code and make sure it works.

        let square = |num| num * num;
        println!("5 squared is {}", square(5));

        // 2. Uncomment the code below.  Finish the .map() iterator adaptor call by passing it a closure
        // which takes a tuple of two integers as a parameter, and returns a tuple with the first
        // integer incremented by 1, and the second integer left alone.  For example, if given the input
        // (0, 1), it should return (1, 1). Run the code and make sure it works.

        let pairs = vec![(0, 1), (2, 3), (4, 5)];
        pairs
            .into_iter()
            .map(|(x, y)| (x + 1, y))
            .for_each(|t| println!("{:?}", t));

        let pairs2 = vec![(0, 1), (2, 3), (4, 5)];
        for (x, y) in pairs2 {
            println!("{:?}", (x + 1, y));
        }

        // 3. Uncomment the code below. There is a mutable vector named `numbers`. Use an iterator over
        // mutable references to multiply each of the values in `numbers` by 3.
        // Hint 1: You'll need .iter_mut() -- bonus points if you use the shorter, syntactic sugar form!
        // Hint 2: `x` will be a mutable reference, so remember to dereference it to use it

        let mut numbers = vec![1, 2, 3, 4];
        for x in &mut numbers {
            *x *= 3;
        }
        println!("{:?}", numbers); // should print [3, 6, 9, 12]

        let mut numbers_2 = vec![1, 2, 3, 4];
        numbers_2.iter_mut().for_each(|x| *x *= 3);
        println!("{:?}", numbers_2); // should print [3, 6, 9, 12]

        // 4. Uncomment the code below.  Take the vector of words and
        // - Convert the vector into an iterator with .into_iter()
        // - Use .filter() to remove any word that contains the letter "h" -- use .contains()
        // - Use .map() to convert all the words to uppercase -- use .to_uppercase()
        // - Use .collect() to put the transformed words back into a vector
        //
        // Hint: .to_uppercase() is a method on `str` which returns a String

        let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
        let transformed = words
            .into_iter()
            .filter(|word| !word.contains("h"))
            .map(|word| word.to_uppercase())
            .collect::<Vec<_>>();
        println!("Transformed: {:?}", transformed);

        // Challenge:
        //
        // - Rewrite the code in #2 as a for loop
        // - Rewrite the code in #3 in functional style (without a for loop).  Hint: There are multiple
        // ways to accomplish this, but they all end with an iterator consumer.
    }
}

mod traits {
    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
    pub enum Cake {
        Chocolate,
        MapleBacon,
        Spice,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Party {
        pub at_restaurant: bool,
        pub num_people: u8,
        pub cake: Cake,
    }

    pub fn run() {
        // 1. The code below doesn't work because Cake doesn't implement Debug.
        // - Derive the Debug trait for the Cake enum above so this code will work. Then, run the code.

        let cake = Cake::Spice;
        admire_cake(cake);

        // 2. Uncomment the code below. It doesn't work since `cake` was *moved*
        // into the admire_cake() function. Let's fix the Cake enum so the code
        // below works without any changes.
        // - Derive the Copy trait for the Cake enum so that `cake` gets copied
        // into the admire_cake() function instead of moved.
        // - Hint: You may need to derive another trait in order to be able to
        //   derive the Copy trait

        match cake {
            Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate."),
            Cake::MapleBacon => println!("Dreams do come true!"),
            Cake::Spice => println!("Great, let's spice it up!"),
        }

        // 3. Uncomment the println below. It doesn't work since the Party
        // struct doesn't implement the Debug or Default traits.
        // - Derive the Debug trait for the Party struct
        // - Manually implement the Default trait for the Party struct. Use the
        // value below as the default value that you return from the `default`
        // method:
        //

        impl Default for Party {
            fn default() -> Self {
                Party {
                    at_restaurant: true,
                    num_people: 8,
                    cake: Cake::Chocolate,
                }
            }
        }
        //
        // Hint: If you get stuck, there is an example at
        // https://doc.rust-lang.org/std/default/trait.Default.html#how-can-i-implement-default

        println!("The default Party is\n{:#?}", Party::default());

        // 4. You prefer Maple Bacon cake. Use "struct update syntax" to create
        // a Party with `cake` set to `Cake::MapleBacon`, but the rest of the
        // values are default.
        //
        // Hint: The trick to struct update syntax is specifying the value(s)
        // you want to customize first and then ending the struct with
        // `..Default::default()` -- but no comma after that!

        let party = Party {
            cake: Cake::MapleBacon,
            ..Default::default()
        };
        println!("Yes! My party has my favorite {:?} cake!", party.cake);

        // 5. Parties are "equal" if they have the same cake.
        // - Derive the PartialEq trait for the Cake enum so Cakes can be
        //   compared.
        // - Manually implement the PartialEq trait for Party. If different
        // parties have the same cake, then they are equal, no matter the
        // location or number of attendees at the party.
        // - Uncomment and run the code below.

        impl PartialEq for Party {
            fn eq(&self, other: &Self) -> bool {
                self.cake == other.cake
            }
        }

        let other_party = Party {
            at_restaurant: false,
            num_people: 235,
            cake: Cake::MapleBacon,
        };
        if party == other_party {
            println!("Your party is just like mine!");
        }

        // Challenge: You would like to be able to pass a Party struct into the
        // smell_cake() function which takes a type T which implements the
        // Into<Cake> trait.
        // - Uncomment the code below AND uncomment the smell_cake() function at
        //   the bottom of this file
        // - Implement `From<Party> for Cake` so that the function call below
        //   works.
        //

        impl From<Party> for Cake {
            fn from(party: Party) -> Self {
                party.cake
            }
        }

        smell_cake(party);
        smell_cake(&party);

        // Challenge 2: Implement `From<&Party> for Cake` so that you can smell
        // your cake without consuming it. Change the code above to pass in a
        // &party. Then uncomment and run the code below. After all, you want to
        // smell your cake and eat it, too!

        impl From<&Party> for Cake {
            fn from(&party: &Party) -> Self {
                party.cake
            }
        }

        println!(
            "Yum! I'm eating this cake: {:?}. Oops, I dropped it on the
        floor.",
            party.cake
        );
        drop(cake);
    }

    pub fn admire_cake(cake: Cake) {
        println!("What a nice {:?} cake! 🎂", cake);
    }

    pub fn smell_cake<T: Into<Cake>>(something: T) {
        println!("Hmm...something smells like a {:?} cake!", something.into());
    }
}

fn main() {
    println!("Hello, world!");
    closures_iterators_activity::run();
    traits::run();
}
