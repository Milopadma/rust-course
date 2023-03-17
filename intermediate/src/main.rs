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

fn main() {
    println!("Hello, world!");
    closures_iterators_activity::run();
}
