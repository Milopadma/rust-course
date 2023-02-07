pub mod bill_manager_project {
    use std::collections::HashMap;

    pub struct User {
        pub bills: HashMap<i16, Bill>,
        pub add_bill: fn(User) -> User,
        pub edit_bills: fn(User) -> User,
        pub print_bills: fn(&User),
    }

    pub struct Bill {
        pub name: String,
        pub amount: f64,
        pub description: String,
    }

    // impl both add_bill and print_bills for User struct
    impl User {
        pub fn add_bill(mut self) -> User {
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
            self.bills.insert(
                self.bills.len() as i16,
                Bill {
                    name: bill_name,
                    amount: bill_amount,
                    description: bill_description,
                },
            );

            // print the successfully added bill by getting the last element
            // bill from the User bill hashmap
            println!("Bill added successfully");
            println!(
                "Bill name: {}",
                self.bills.get(&(self.bills.len() as i16 - 1)).unwrap().name
            );

            // return to the main menu
            println!("Press enter to return to the main menu");
            let mut enter = String::new();
            std::io::stdin()
                .read_line(&mut enter)
                .expect("Failed to read line");

            // clear the screen
            print!("{}[2J", 27 as char);

            // return the user
            return self;
        }

        pub fn edit_bills(mut self) -> User {
            // check if there are any bills
            if self.bills.len() == 0 {
                println!("No bills to edit");
                return self;
            }

            // print the bills by immutably borrowing the user
            (self.print_bills)(&self);

            println!("Enter bill number: ");
            let mut bill_number = String::new();
            std::io::stdin()
                .read_line(&mut bill_number)
                .expect("Failed to read line");
            let bill_number: f64 = match bill_number.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.0,
            };

            // then verify the bill number if it exists
            if !self.bills.contains_key(&(bill_number as i16)) {
                println!("Bill number does not exist");
                return self;
            }

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

            // then edit the bill
            self.bills.insert(
                bill_number as i16,
                Bill {
                    name: bill_name,
                    amount: bill_amount,
                    description: bill_description,
                },
            );

            // print the successfully edited bill
            println!("Bill edited successfully");
            println!("Bill name: {}", self.bills[&(bill_number as i16)].name);
            println!("Bill amount: {}", self.bills[&(bill_number as i16)].amount);

            println!(
                "Bill description: {}",
                self.bills[&(bill_number as i16)].description
            );

            // return to the main menu
            println!("Press enter to return to the main menu");
            let mut enter = String::new();
            std::io::stdin()
                .read_line(&mut enter)
                .expect("Failed to read line");

            // return the user
            return self;
        }

        pub fn print_bills(&self) {
            // check if there are any
            if self.bills.len() == 0 {
                println!("No bills to print");
                return;
            } else {
                // print the bills hashmap
                for (i, bills) in &self.bills {
                    println!("Bill number: {}", i);
                    println!("Bill name: {}", bills.name);
                    println!("Bill amount: {}", bills.amount);
                    println!("Bill description: {}", bills.description);
                }
            }
        }
    }

    pub fn run() {
        // generate the user
        let mut user = User {
            bills: HashMap::new(),
            add_bill: User::add_bill as fn(User) -> User,
            edit_bills: User::edit_bills as fn(User) -> User,
            print_bills: User::print_bills as fn(&User),
        };

        // ugh -> bug was creating new user objects everytime restarted
        loop {
            // the default welcome screen for ALL users
            println!("Welcome to Bill Manager");
            println!("1. Add Bill");
            println!("2. Print Bills");
            println!("3. Edit Bills");
            println!("4. Exit");

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
                1 => {
                    // add bill
                    user = (user.add_bill)(user);
                }
                2 => {
                    // print bills
                    (user.print_bills)(&user);
                }
                3 => {
                    // edit bills
                    user = (user.edit_bills)(user);
                }
                4 => {
                    // exit
                    println!("Exiting...");
                    break;
                }
                _ => {
                    // invalid choice
                    println!("Invalid choice");
                }
            }
        }
    }
}

pub fn main() {
    bill_manager_project::run();
}
