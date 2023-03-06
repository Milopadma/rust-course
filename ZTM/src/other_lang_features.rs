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
