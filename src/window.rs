use hyprland::shared::Address;

struct WindowAddress(usize);
impl WindowAddress {
    fn get(&self) -> usize {
        self.0
    }
}

impl From<Address> for WindowAddress {
    fn from(address: Address) -> Self {
        if let Ok(address) = usize::from_str_radix(address.to_string().trim_start_matches("0x"), 16)
        {
            Self(address)
        } else {
            Self(0)
        }
    }
}

impl From<WindowAddress> for Address {
    fn from(address: WindowAddress) -> Self {
        Address::new(format!("0x{:x}", address.0))
    }
}

struct WindowPos(i16, i16);
impl WindowPos {
    fn set(&mut self, x: i16, y: i16) {
        (self.0, self.1) = (x, y);
    }
    fn get(&self) -> (i16, i16) {
        (self.0, self.1)
    }
}

struct WindowSize(i16, i16);
impl WindowSize {
    fn set(&mut self, w: i16, h: i16) {
        (self.0, self.1) = (w, h);
    }
    fn get(&self) -> (i16, i16) {
        (self.0, self.1)
    }
}

pub struct Window {
    addr: WindowAddress,
    pos: WindowPos,
    size: WindowSize,
    owner: u8,
}

impl Window {
    pub fn new(addr: usize, pos: (i16, i16), size: (i16, i16), owner: u8) -> Self {
        Self {
            addr: WindowAddress(addr),
            pos: WindowPos(pos.0, pos.1),
            size: WindowSize(size.0, size.1),
            owner,
        }
    }
    pub fn set(&mut self, pos: Option<(i16, i16)>, size: Option<(i16, i16)>, owner: Option<u8>) {
        if let Some((x, y)) = pos {
            self.pos.set(x, y);
        }
        if let Some((w, h)) = size {
            self.size.set(w, h);
        }
        if let Some(o) = owner {
            self.owner = o;
        }
    }
    pub fn get(&self) -> ((i16, i16), (i16, i16)) {
        (self.pos.get(), self.size.get())
    }
}

impl core::fmt::Debug for Window {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            r#"

-------- Window -------
    Address: {:#x}
    Position: {:?},
    Size: {:?},
    Owner: {:?}
-----------------------

            "#,
            self.addr.get(),
            self.pos.get(),
            self.size.get(),
            self.owner
        )
    }
}
