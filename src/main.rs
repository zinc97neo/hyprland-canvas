use core::panic;
use std::thread;

use hyprland::{event_listener::EventListener, Result};

fn main() {
    loop {
        let handle = thread::spawn(move || {
            if let Err(e) = register_and_listen() {
                panic!("Failed to register event listener: {e}");
            }
        });
        match handle.join() {
            Ok(_) => {
                println!("unknown condition");
            }
            Err(e) => {
                println!("Failed to join thread: {:?}", e);
            }
        }
    }
}

fn register_and_listen() -> Result<()> {
    let mut listener = EventListener::new();
    listener.add_active_window_change_handler(|data| {
        println!("{:#?}", data);
    });
    listener.add_window_open_handler(|data| {
        println!("{:?}", data);
    });
    listener.add_window_close_handler(|data| {
        println!("{:?}", data);
    });
    listener.start_listener()?;
    Ok(())
}
