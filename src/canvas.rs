use hyprland::{
    dispatch::{Dispatch, DispatchType, Position, WindowIdentifier},
    shared::Address,
};

#[derive(Debug)]
struct Window {
    address: Address,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
}

impl Window {
    pub fn new(address: Address, x: i16, y: i16, width: i16, height: i16) -> Self {
        Self {
            address,
            x,
            y,
            width,
            height,
        }
    }
    pub fn set_pos(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }
    pub fn resize(&mut self, w: i16, h: i16) {
        self.width = w;
        self.height = h;
    }
    pub fn set_hyprland_window(&self) {
        let address = self.address.clone();
        let window_identifier = Some(WindowIdentifier::Address(address));
        // Dispatch::call(DispatchType::ToggleFloating(window_identifier.clone()));
        Dispatch::call(DispatchType::MoveWindowPixel(
            Position::Exact(self.x, self.y),
            window_identifier.clone().unwrap(),
        ));
        Dispatch::call(DispatchType::ResizeWindowPixel(
            Position::Exact(self.width, self.height),
            window_identifier.clone().unwrap(),
        ));
    }
}

pub struct Canvas {
    windows: Vec<Window>,
    gap: i16,
    width: i16,
    height: i16,
    active_address: Option<Address>,
}

impl Canvas {
    pub fn new(gap: i16, width: i16, height: i16) -> Self {
        Self {
            windows: vec![],
            gap,
            width,
            height,
            active_address: None,
        }
    }
    pub fn change_active(&mut self, address: Option<Address>) {
        self.active_address = address;
        self.arrange_windows();
    }
    pub fn add_window(&mut self, address: Address) {
        let window = Window::new(address, 0, 0, self.width / 3, self.height - self.gap);
        // println!("Adding window: {:#?}", window);
        self.windows.push(window);
        self.arrange_windows();
    }
    pub fn remove_window(&mut self, address: Address) {
        self.windows.retain(|w| w.address != address);
        self.arrange_windows();
    }
    fn arrange_windows(&mut self) {
        if let None = self.active_address {
            return;
        }
        let active_address = self.active_address.clone().unwrap();
        let (x, y) = (10 as i16, 10 as i16);
        let active_index = self
            .windows
            .iter()
            .position(|w| w.address == active_address);
        if let Some(index) = active_index {
            (self.windows[index].x, self.windows[index].y) = (x, y);

            let gap = self.gap;
            let mut current_x = x;
            for window in self.windows[..index].iter_mut().rev() {
                current_x -= window.width + 2 * gap;
                window.x = current_x;
            }

            let mut current_x = x + self.windows[index].width + 2 * gap;
            for window in self.windows[index + 1..].iter_mut().rev() {
                window.x = current_x;
                current_x += window.width + 2 * gap;
            }
        }
        println!("Windows: {:#?}", self.windows);
        self.windows.iter().for_each(|w| w.set_hyprland_window());
    }
}
