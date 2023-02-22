mod typestates {
    // the multiple states for luggage airport process
    struct CheckIn;
    struct OnLoading;
    struct Offloading;
    struct AwaitingPickup;
    struct EndCustody;

    // the luggage struct
    struct Luggage<T> {
        state: T,
    }

    // the luggage struct with a method to change state
    impl<T> Luggage<T> {
        fn new(state: T) -> Self {
            Luggage { state }
        }

        fn check_in(self) -> Luggage<CheckIn> {
            Luggage { state: CheckIn }
        }

        fn on_loading(self) -> Luggage<OnLoading> {
            Luggage { state: OnLoading }
        }

        fn offloading(self) -> Luggage<Offloading> {
            Luggage { state: Offloading }
        }

        fn awaiting_pickup(self) -> Luggage<AwaitingPickup> {
            Luggage {
                state: AwaitingPickup,
            }
        }

        fn end_custody(self) -> Luggage<EndCustody> {
            Luggage { state: EndCustody }
        }
    }

    // run
    pub fn run() {
        let luggage = Luggage::new(CheckIn);
        let luggage = luggage.check_in();
        let luggage = luggage.on_loading();
        let luggage = luggage.offloading();
        let luggage = luggage.awaiting_pickup();
        let luggage = luggage.end_custody();

        println!("Luggage state: {:?}", luggage.state);
    }
}
