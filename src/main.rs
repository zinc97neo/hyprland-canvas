#![allow(dead_code)]
#![allow(unused)]
mod canvas;
use canvas::Canvas;
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

lazy_static! {
    static ref CANVAS: Mutex<Canvas> = Mutex::new(Canvas::new(8, 1440, 900));
}

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
        let address = if let Some(data) = data.clone() {
            Some(data.window_address)
        } else {
            None
        };
        let canvas = CANVAS.lock();
        if let Ok(mut canvas) = canvas {
            canvas.change_active(address);
        } else if let Err(e) = canvas {
            panic!("Failed to lock canvas: {e}");
        }
    });
    listener.add_window_open_handler(|data| {
        let address = data.window_address.clone();
        let canvas = CANVAS.lock();
        if let Ok(mut canvas) = canvas {
            canvas.add_window(address);
        } else if let Err(e) = canvas {
            panic!("Failed to lock canvas: {e}");
        }
    });
    listener.add_window_close_handler(|data| {
        let address = data.clone();
        let canvas = CANVAS.lock();
        if let Ok(mut canvas) = canvas {
            canvas.remove_window(address);
        } else if let Err(e) = canvas {
            panic!("Failed to lock canvas: {e}");
        }
    });
    listener.start_listener()?;
    Ok(())
}
