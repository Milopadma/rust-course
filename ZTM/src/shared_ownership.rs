mod shared_mutability {
    // interior mutability is a feature of Rust that allows you to
    // * mutate data in a thread-safe way
    use std::cell::Cell;

    #[derive(Debug)]
    struct Book {
        signed: Cell<bool>,
    }

    impl Book {
        fn sign(&self) {
            self.signed.set(true);
        }

        fn signed(&self) -> bool {
            self.signed.get()
        }
    }

    pub fn run() {
        let my_book = Book {
            signed: Cell::new(false),
        };

        println!("{:?}", my_book.signed()); // this returns false
        my_book.sign();
        println!("{:?}", my_book.signed()); // this returns true
    }
}

mod shared_mutability_ref_cells {
    use std::cell::RefCell; // a refcell is a cell but with a mutable borrow instead of a
                            // normal mutate

    struct Person {
        name: RefCell<String>,
    }

    pub fn run() {
        let name = String::from("John");
        let person = Person {
            name: RefCell::new(name),
        };

        // using scopes to make these two scopes work
        {
            // mutably borrow the persons name
            let mut name = person.name.borrow_mut();
            *name = String::from("Jane"); // dereference
        }
        // drop first scope
        {
            // OR use replace()
            person.name.replace("peepoo".to_owned());
        }

        // OR use do a checked borrow using Result
        let new_name: Result<_, _> = person.name.try_borrow();
        match new_name {
            Ok(name) => {
                println!("{}", name);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        //
        let new_name_mut: Result<_, _> = person.name.try_borrow_mut();
        match new_name_mut {
            Ok(name) => {
                println!("{}", name);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

mod smart_pointers_interior_mutability_demo {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum MenuItem {
        Drink,
        Salad,
    }

    #[derive(Debug)]
    struct ItemOrder {
        item: MenuItem,
        quantity: u32,
    }

    // all orders at a table
    struct TableOrder {
        items: Vec<ItemOrder>,
    }

    fn new_table_order() -> TableOrder {
        TableOrder {
            items: vec![ItemOrder {
                item: MenuItem::Drink,
                quantity: 1,
            }],
        }
    }

    // this type alias contains the Vec of TableOrder
    type Order = Rc<RefCell<Vec<TableOrder>>>;

    #[derive(Debug)]
    struct Chef(Order);

    #[derive(Debug)]
    struct WaitStaff(Order);

    #[derive(Debug)]
    struct Accounting(Order);

    pub fn run() {
        let orders = Rc::new(RefCell::new(vec![]));

        // all the staff has access to the orders
        let chef = Chef(Rc::clone(&orders));
        let wait_staff = WaitStaff(Rc::clone(&orders));
        let accounting = Accounting(Rc::clone(&orders));

        // new order
        let new_order = new_table_order();

        {
            orders.borrow_mut().push(new_order);
        }

        dbg!(chef.0.borrow());
        drop(chef); // chef go home and no longer has access to the orders
                    // but the wait staff still has access to the orders
        dbg!(wait_staff.0.borrow());
        dbg!(accounting.0.borrow());
    }
}

mod smart_points_ref_cell_activity {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum Vehicle {
        Car,
        Truck,
    }

    #[derive(Debug, Hash, PartialEq, PartialOrd)]
    enum Status {
        Avaliable,
        Unavailable,
        Maintenance,
        Rented,
    }

    #[derive(Debug)]
    struct Rentals {
        vehicle_type: Vehicle,
        vehicle_VIN: u32,
        vehicle_status: Status,
    }

    impl Rentals {
        fn new(vehicle_type: Vehicle, vehicle_VIN: u32, vehicle_status: Status) -> Self {
            Rentals {
                vehicle_type,
                vehicle_VIN,
                vehicle_status,
            }
        }
    }

    // wrap the Rentals struct in a Rc<RefCell<Rentals>>
    type RentalsRefCell = Rc<RefCell<Vec<Rentals>>>;
    // rc means data exists as long as one copy of it exists
    // refcell means we can mutate the data inside the struct
    // "anyone that has access can be modified with a copy of it, can make as many copy as we want"

    struct Corporate(RentalsRefCell);

    struct StoreFront(RentalsRefCell);

    pub fn run() {
        let new_rentals_main = vec![
            Rentals::new(Vehicle::Car, 1, Status::Avaliable),
            Rentals::new(Vehicle::Truck, 2, Status::Avaliable),
        ];

        // wrap it inside a refcell
        let vehicles_vec_ref_cell = Rc::new(RefCell::new(new_rentals_main));

        // clone the pointers for the vec ref cell
        let corporate = Corporate(Rc::clone(&vehicles_vec_ref_cell));
        let store_front = StoreFront(Rc::clone(&vehicles_vec_ref_cell));

        dbg!(new_corporate.0.borrow());
        drop(new_corporate);
        dbg!(new_storefront.0.borrow());
    }
}

fn main() {
    // shared_mutability::run();
    // shared_mutability_ref_cells::run();
    smart_pointers_interior_mutability_demo::run();
}
