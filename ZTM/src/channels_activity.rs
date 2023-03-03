// Topic: Channels
//
mod channels_activity {

    // Summary:
    //   Using the existing code, create a program that simulates an internet-of-things
    //   remote control light bulb. The color of the light can be changed remotely.
    //   Use threads and channels to communicate what color the light bulb should display.
    //
    // Requirements:
    // * Create a separate thread representing the light bulb
    // * Use a channel to communicate with the thread
    // * Display a color change message using the println! macro
    // * The light bulb must also be able to turn on and off
    //   * Display whether the light is on or off on each color change
    // * Turn off the light when disconnecting from it
    //
    // Notes:
    // * Remember to add `crossbeam-channel` to your Cargo.toml file
    // * Use the `colored` crate if you want to get fancy and display actual colors
    // * The docs.rs site can be used to read documentation for third-party crates
    // * Disconnection can be accomplished by dropping the sender, or
    //   by telling the thread to self-terminate
    // * Use `cargo test --bin a39` to test your program to ensure all cases are covered

    use crossbeam_channel::{unbounded, Receiver};
    use std::thread::{self, JoinHandle};

    enum LightMsg {
        // Add additional variants needed to complete the exercise
        ChangeColor(u8, u8, u8),
        Disconnect,
        Off,
        On,
    }

    enum LightStatus {
        Off,
        On,
    }

    fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
        // Add code here to spawn a thread to control the light bulb
        thread::spawn(move || {
            // Add code here to control the light bulb
            let mut status = LightStatus::Off;
            while let Ok(msg) = receiver.recv() {
                match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        status = LightStatus::On;
                    }
                    LightMsg::Disconnect => {
                        status = LightStatus::Off;
                        println!("Light is off");
                    }
                    LightMsg::Off => {
                        status = LightStatus::Off;
                    }
                    LightMsg::On => {
                        status = LightStatus::On;
                    }
                }
            }
            status
        })
    }

    fn main() {
        let (sender, receiver) = unbounded();
        // send the receiver into the spawnlightthread
        let light_thread = spawn_light_thread(receiver);

        sender.send(LightMsg::On).unwrap();
        sender.send(LightMsg::ChangeColor(255, 0, 0)).unwrap();
        sender.send(LightMsg::ChangeColor(0, 255, 0)).unwrap();
        sender.send(LightMsg::ChangeColor(0, 0, 255)).unwrap();
        sender.send(LightMsg::Disconnect).unwrap();
        sender.send(LightMsg::Off).unwrap();
        sender.send(LightMsg::On).unwrap();
        sender.send(LightMsg::ChangeColor(255, 0, 0)).unwrap();
        sender.send(LightMsg::ChangeColor(0, 255, 0)).unwrap();
        sender.send(LightMsg::ChangeColor(0, 0, 255)).unwrap();
        sender.send(LightMsg::Disconnect).unwrap();

        let status = light_thread.join().unwrap();
        println!("{:?}", status);
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crossbeam_channel::unbounded;

        #[test]
        fn light_off_when_disconnect() {
            let (s, r) = unbounded();

            let light = spawn_light_thread(r);
            s.send(LightMsg::Disconnect).expect("channel disconnected");

            let light_status = light.join().expect("failed to join light thread");

            if let LightStatus::On = light_status {
                panic!("light should be off after disconnection");
            }
        }

        #[test]
        fn light_off_when_dropped() {
            let (s, r) = unbounded();

            let light = spawn_light_thread(r);
            drop(s);

            let light_status = light.join().expect("failed to join light thread");

            if let LightStatus::On = light_status {
                panic!("light should be off after dropping sender");
            }
        }
    }
}
