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

mod threads_mutex {
    // threads can execute at random times, and multiple threads can execute on
    // the same data, causing the data to become corrupted easily

    // to avoid this, we can use a mutex by data syncronization
    use std::sync::Mutex; //mutually exclusive lock

    // this ensures that only one thread can access the data at a time
    // mutexes wrap the data and make the data mutually exclusive
    // only allow one thread at a time

    // to allow mutexes to be shared among theads, an Arc (atomic reference
    // count) pointers
    // is used
    use std::sync::Arc;

    use parking_lot::Mutex as parking_lot_mutex; // using the parking_lot crate for their mutex
    use std::thread;

    struct Counter(usize);

    pub fn run() {
        let counter = Counter(0);
        // wrap the Mutex wrapped Counter data in an Arc
        let shared_counter = Arc::new(parking_lot_mutex::new(counter));
        // two copies of the counter
        let thread_1_counter = Arc::clone(&shared_counter);
        let thread_2_counter = shared_counter.clone();

        // utilize them in threads
        // thread_1  locks the counter
        let thread_1 = thread::spawn(move || {
            let mut counter = thread_1_counter.lock();
            counter.0 += 1;
        });

        // thread_2 locks the counter
        let thread_2 = thread::spawn(move || {
            let mut counter = thread_2_counter.lock();
            counter.0 += 1;
        });

        // rejoin thread 1 into main thread and join thread 2
        thread_1.join().and_then(|_| thread_2.join());
        println!("{}", shared_counter.lock().0); // this will print out "2"
    }
}

mod threading_deadlocks {
    // a situation where threads are waiting for one another and get stuck
    // this can happen when :
    // - using multiple locks
    // - recursing when taking a lock
    // - locking the same lock twice

    use parking_lot::Mutex;
    use std::rc::Rc;

    // recursive function that locks the data it receives,
    // then keeps recursing until remaining is 0
    // this is a deadlock
    fn recurse(data: Rc<Mutex<i32>>, remaining: usize) -> usize {
        let mut locked = data.lock();
        match remaining {
            rem if rem == 0 => 0,
            rem => recurse(Rc::clone(&data), remaining - 1),
        }
    }
    // can replace mutex with a ReentrantMutex to fix this deadlock
    // this works because the function is called in the same thread
}

mod thread_deadlock_example_2 {
    use parking_lot::Mutex;
    use std::sync::Arc;
    use std::thread;

    struct Account {
        balance: i64,
    }

    type ArcAccount = Arc<Mutex<Account>>;

    pub fn transfer(from: ArcAccount, to: ArcAccount, amount: i64) {
        // exclusive access to both accounts data
        let mut from = from.lock();
        let mut to = to.lock();
        from.balance -= amount;
        to.balance += amount;
    }

    pub fn run() {
        // init the ArcAccounts
        let a = Arc::new(Mutex::new(Account { balance: 0 }));
        let b = Arc::new(Mutex::new(Account { balance: 0 }));

        // perform transfers using threads
        let t1 = thread::spawn(move || transfer(a, b, 500));
        let t2 = thread::spawn(move || transfer(b, a, 500));
        // this is a deadlock where  both threads can lock at the same time
        // can fix by retry on failure using Some and try_locks(), and waits
        // this can cause thread contention/backoff problems though

        // to truly fix this problem, we can use a backoff::ExponentialBackoff
        // basically using a variable wait times
    }
}

mod mutex_demo {
    use parking_lot::Mutex;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    type SharedSignData = Arc<Mutex<String>>;

    struct DigitalSignBoard {
        display: SharedSignData,
    }

    impl DigitalSignBoard {
        fn refresh(&self) {
            let mut locked = self.display.lock();

            // print
            println!("{}", locked);
        }
    }

    fn spawn_display_thread(display_data: SharedSignData) {
        thread::spawn(move || {
            let board = DigitalSignBoard {
                display: display_data,
            };
            loop {
                board.refresh();
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

    fn change_data(display_data: SharedSignData, new_data: &str) {
        let mut data = display_data.lock();
        *data = new_data.to_string();

        // update self display text
        display_data = Arc::new(Mutex::new(new_data.to_string()));
    }
}

mod mutex_activity {
    use crossbeam_channel::{unbounded, Receiver, Sender};
    use parking_lot::Mutex;
    use std::collections::VecDeque;
    use std::sync::Arc;
    // data structure that holds a queue of messages
    use std::thread::{self, JoinHandle}; // threading library
    use std::time::Duration;

    /// Job given to workers.
    #[derive(Clone)]
    enum Job {
        Print(String),
        Sum(isize, isize),
    }

    /// Message sent to workers.
    #[derive(Clone)]
    enum Message {
        AddJob(Job),
        Quit,
    }

    struct Worker<M> {
        tx: Sender<M>,
        _rx: Receiver<M>,
        handle: JoinHandle<()>,
    }

    impl Worker<Message> {
        fn add_job(&self, job: Job) {
            self.tx
                .send(Message::AddJob(job))
                .expect("failed to add job");
        }
        fn join(self) {
            self.handle.join().expect("failed to join thread");
        }
        fn send_msg(&self, msg: Message) {
            self.tx.send(msg).expect("failed to send message");
        }
    }

    /// Create a new worker to receive jobs.
    fn spawn_worker(counter: Arc<Mutex<usize>>) -> Worker<Message> {
        let (tx, rx) = unbounded();
        // We clone the receiving end here so we have a copy to give to the
        // thread. This allows us to save the `tx` and `rx` into the Worker struct.
        let rx_thread = rx.clone();
        // Spawn a new thread.
        let handle = thread::spawn(move || {
            // VecDeque allows us to get jobs in the order they arrive.
            let mut jobs = VecDeque::new();
            // Outer loop is so we can have a brief delay when no
            // jobs are available.
            loop {
                // Inner loop continuously processes jobs until
                // no more are available.
                loop {
                    // Get the next job.
                    for job in jobs.pop_front() {
                        match job {
                            Job::Print(msg) => println!("{}", msg),
                            Job::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, lhs + rhs),
                        }

                        let mut counter = counter.lock();
                        *counter += 1;
                    }
                    // Check for messages on the channel.
                    if let Ok(msg) = rx_thread.try_recv() {
                        match msg {
                            Message::AddJob(job) => {
                                // When we receive a new job, add it
                                // to the jobs list.
                                jobs.push_back(job);
                                // Continue processing jobs.
                                continue;
                            }
                            Message::Quit => return,
                        }
                    } else {
                        // No messages on the channel, break from inner loop
                        // and thread will wait momentarily for more messages.
                        break;
                    }
                }
                // Pause to wait for more messages to arrive on channel.
                thread::sleep(Duration::from_millis(100));
            }
        });

        Worker {
            tx,
            _rx: rx,
            handle,
        }
    }

    pub fn run() {
        let jobs = vec![
            Job::Print("hello".to_owned()),
            Job::Sum(2, 2),
            Job::Print("world".to_owned()),
            Job::Sum(4, 4),
            Job::Print("two words".to_owned()),
            Job::Sum(1, 1),
            Job::Print("a print job".to_owned()),
            Job::Sum(10, 10),
            Job::Print("message".to_owned()),
            Job::Sum(3, 4),
            Job::Print("thread".to_owned()),
            Job::Sum(9, 8),
            Job::Print("rust".to_owned()),
            Job::Sum(1, 2),
            Job::Print("compiler".to_owned()),
            Job::Sum(9, 1),
        ];

        let jobs_sent = jobs.len();

        let jobs_counter = Arc::new(Mutex::new(0));

        let mut workers = vec![];
        // Spawn 4 workers to process jobs.
        for _ in 0..4 {
            let worker = spawn_worker(Arc::clone(&jobs_counter));
            workers.push(worker);
        }

        // Create an iterator that cycles through each worker endlessly.
        let mut worker_ring = workers.iter().cycle();
        for job in jobs.into_iter() {
            // Get next worker
            let worker = worker_ring.next().expect("failed to get worker");
            // Add the job
            worker.add_job(job);
        }

        // Ask all workers to quit.
        for worker in &workers {
            worker.send_msg(Message::Quit);
        }

        // Wait for workers to terminate.
        for worker in workers {
            worker.join();
        }

        println!("Jobs sent: {}", jobs_sent);

        // print out the number of jobs completed here.
        let jobs_completed = jobs_counter.lock().clone(); // lock it to get
                                                          // a copy
        println!("Jobs completed: {}", jobs_completed);
    }
}

fn main() {
    // shared_mutability::run();
    // shared_mutability_ref_cells::run();
    smart_pointers_interior_mutability_demo::run();
}
