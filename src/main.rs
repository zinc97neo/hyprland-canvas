#![allow(dead_code)]
#![allow(unused)]
use core::panic;
use hyprland::{
    data::{Clients, Monitors},
    dispatch::{Dispatch, DispatchType, Position, WindowIdentifier},
    event_listener::EventListener,
    shared::{Address, HyprData, HyprDataVec},
    Result,
};
use std::sync::Mutex;
use std::thread;
#[macro_use]
extern crate lazy_static;

// lazy_static! {
//     static ref CANVAS: Mutex<Canvas> = Mutex::new(Canvas::new(8, 1440, 900));
// }

fn main() {
    let clients = Clients::get();
    for client in clients.iter() {
        println!("{:#?}", client);
    }
    // loop {
    //     let handle = thread::spawn(move || {
    //         if let Err(e) = register_and_listen() {
    //             panic!("Failed to register event listener: {e}");
    //         }
    //     });
    //     match handle.join() {
    //         Ok(_) => {
    //             println!("unknown condition");
    //         }
    //         Err(e) => {
    //             println!("Failed to join thread: {:?}", e);
    //         }
    //     }
    // }
}

fn register_and_listen() -> Result<()> {
    let mut listener = EventListener::new();
    listener.add_active_window_change_handler(|data| {});
    listener.add_window_open_handler(|data| {});
    listener.add_window_close_handler(|data| {});
    listener.start_listener()?;
    Ok(())
}
