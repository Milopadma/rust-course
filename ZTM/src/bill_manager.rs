pub mod bill_manager_project {

    pub struct User {
        pub bills: Vec<Bill>,
        pub add_bill: fn(&mut User, Bill),
        pub print_bills: fn(&User),
    }

    pub struct Bill {
        pub name: String,
        pub amount: f64,
        pub description: String,
    }

    // impl both add_bill and print_bills for User struct
    impl User {
        pub fn add_bill(&self) {
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
                Err(_) => 0.0,
            };
            println!("Enter bill description: ");
            let mut bill_description = String::new();
            std::io::stdin()
                .read_line(&mut bill_description)
                .expect("Failed to read line");

            // add to the user
            self.bills.push(Bill {
                name: bill_name,
                amount: bill_amount,
                description: bill_description,
            });

            // print the successfully added bill by fetching the newest
            // bill from the User bill vector
            println!("Bill added successfully");
            println!("Bill name: {}", self.bills.last().unwrap().name);
            println!("Bill amount: {}", self.bills.last().unwrap().amount);
            println!(
                "Bill description: {}",
                self.bills.last().unwrap().description
            );

            // return to the main menu
            println!("Press enter to return to the main menu");
            let mut enter = String::new();
            std::io::stdin()
                .read_line(&mut enter)
                .expect("Failed to read line");

            // clear the screen
            print!("{}[2J", 27 as char);
        }
        pub fn print_bills(&self) {
            // check if there are any
            if self.bills.len() == 0 {
                println!("No bills to print");
                return;
            } else {
                for bill in &self.bills {
                    println!("Bill name: {}", bill.name);
                    println!("Bill amount: {}", bill.amount);
                    println!("Bill description: {}", bill.description);
                }
            }
        }
    }

    pub fn run() {
        loop {
            // generate the user
            let mut user = User {
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
                1 => user.add_bill(),
                2 => user.print_bills(),
                3 => break,
                _ => continue,
            }
        }
    }
}

pub fn main() {
    bill_manager_project::run();
}
