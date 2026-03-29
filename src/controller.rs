use crate::button::Button;
use crate::event::Event;
use crate::sensor::Sensor;
use crate::sound::Sound;
use crate::stick::Stick;
// use derive_more::Constructor;

pub struct Trigger {
    r2: u8,
    l2: u8,
}

impl Trigger {
    fn print_trigger(&self) {
        todo!()
    }
}

pub struct Controller {
    // data: Ps4,
    sensor: Sensor,
    sound: Sound,
    event: Event,
    stick: Stick,
    flags: Button,
    trigger: Trigger,
}

impl Controller {
    pub fn new(&self, buf: [u8; 10]) {
        todo!()
    }
}

// pub enum PacketIndex {
//     AnalogSticklx = 13,
//     AnalogStickly = 14,
//     AnalogStickrx = 15,
//     AnalogStickry = 16,

//     ButtonStandard = 17,
//     ButtonExtra = 18,
//     ButtonPS = 19,

//     Analogl2 = 20,
//     Analogr2 = 21,

//     Status = 42,
// }
