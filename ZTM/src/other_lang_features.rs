mod turbofish {
    // identifier::<type>
}

mod loop_labels {
    // 'ident: loop {}

    pub fn matrix_loop() {
        let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        'rows: for row in matrix.iter() {
            // row column label
            'columns: for column in row.iter() {
                //'column loop label
                if column == &2 {
                    break 'columns;
                }
                println!("{}", column);
            }
        }
    }
}

mod loop_expressions {
    // values can be returned from loops

    pub fn run() {
        let value = 5;
        let result: usize = 'val: loop {
            break value;
            break 'val *2;
        };
        println!("The result is {}", result);
    }

    pub fn loop_return_label() {
        let nums = vec![1, 2, 3];
        let div_by_three: Option<usize> = 'outer: loop {
            for n in nums {
                if n % 3 == 0 {
                    break 'outer Some(n);
                }
            }
        };
    }

    // break can return value to a loop expression but only valid
    // inside a loop expression, not while or for
}

mod struct_update_syntax {

    // for when there are many fields to set during creation of a struct
    // but one field needs to be updated
    struct Particle {
        color: (u8, u8, u8),
        position: (u8, u8),
        radius: u8,
    }

    impl Default for Particle {
        fn default() -> Self {
            Self {
                color: (0, 0, 0),
                position: (0, 0),
                radius: 0,
            }
        }
    }

    pub fn run() {
        let mut particle = Particle::default();
        particle.color = (255, 0, 0);
        let particle = particle; // make it un mutable

        // or we can  ...

        let particle_immut = Particle {
            color: (255, 0, 0),
            //  where its immutable, but we changed only one val directly
            ..Particle::default()
        };
    }
}

mod escape_seqs_raw_strings {
    // not always convenient to include certain characters in a string.
    // escape sequences are used to make it easier to include certain characters

    pub fn run() {
        // escape seqs with raw strings
        let msg = r#"Hello, world!"#;
        let msg_backslash_n = r#"Hello, world!\n"#;
        let msg_backslash_t = r#"Hello, world!\t"#;

        // escape seqs only
        let msg_backslash_backslash = "right \\ left";
        let smiley = "\u{1f642}";

        println!("{}", msg);
    }
}
