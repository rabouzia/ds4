struct Status {
    battery: u8,
    charging: u8,
}

pub struct Event {}
impl Event {
    pub fn rumble(&self) {
        todo!()
    }
    pub fn change_color(&self) {
        todo!()
    }
    pub fn battery(&self) {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct Command {
    pub small_rumble: u8,
    pub large_rumble: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub flash_off: u8,
    pub flash_on: u8,
}
