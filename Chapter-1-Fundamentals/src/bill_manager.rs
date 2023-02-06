mod bill_manager_project {

    pub struct User {
        pub bills: Vec<Bill>,
        pub add_bill: fn(&mut User, Bill),
        pub print_bills: fn(&User),
    }

    pub struct Bill {
        pub name: String,
        pub amount: f64,
        pub description: String,
        pub owner: User,
    }

    // impl both add_bill and print_bills for User struct
    impl User {
        pub fn add_bill(&mut self, bill: Bill) {
            self.bills.push(bill);
        }

        pub fn print_bills(&self) {
            for bill in &self.bills {
                println!("Bill name: {}", bill.name);
                println!("Bill amount: {}", bill.amount);
                println!("Bill description: {}", bill.description);
                println!("Bill owner: {}", bill.owner.name);
            }
        }
    }

    pub fn run() {
        while true {
            // generate the user
            let user = User {
                bills: Vec::new(),
                add_bill: User::add_bill,
                print_bills: User::print_bills,
            };

            // the default welcome screen for ALL users
            println!("Welcome to Bill Manager");
            println!("1. Add Bill");
            println!("2. Print Bills");
            println!("3. Exit");

            // for user input
            let mut choice = String::new();
            // read line
            std::io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            // safely convert the string to a number for use in Matching
            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // match the choice
            match choice {
                1 => add_bill_flow(&user),
            }
        }
    }

    fn add_bill_flow(user: &User) {
        println!("Enter bill name: ");
        let mut bill_name = String::new();
        std::io::stdin()
            .read_line(&mut bill_name)
            .expect("Failed to read line");
        println!("Enter bill amount: ");
        let mut bill_amount = String::new();
        std::io::stdin()
            .read_line(&mut bill_amount)
            .expect("Failed to read line");
        let bill_amount: f64 = match bill_amount.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Enter bill description: ");
        let mut bill_description = String::new();
        std::io::stdin()
            .read_line(&mut bill_description)
            .expect("Failed to read line");

        // add to the user
        user.add_bill(Bill {
            name: bill_name,
            amount: bill_amount,
            description: bill_description,
            owner: user,
        });

        // print the successfully added bill by fetching the newest
        // bill from the User bill vector
        println!("Bill added successfully");
        println!("Bill name: {}", user.bills.last().unwrap().name);
        println!("Bill amount: {}", user.bills.last().unwrap().amount);
        println!(
            "Bill description: {}",
            user.bills.last().unwrap().description
        );
    }
}
