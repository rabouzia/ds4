use std::{fs::TryLockError, str::from_utf8};

use crate::{
    button::Button,
    event::Event,
    sensor::{Accelerometer, Sensor},
    sound::Sound,
    stick::Stick,
};
// use derive_more::Constructor;

pub struct Trigger {
    l2: u8,
    r2: u8,
}
pub struct Controller {
    pub sensor: Sensor,
    //pub sound: Sound,
    //pub event: Event,
    //pub button: Button,
    pub stick: Stick,
    pub trigger: Trigger,
}

impl Trigger {
    pub fn print_trigger(&self) {
        self.show_level(self.l2 as usize, "L2");
        self.show_level(self.r2 as usize, "R2");
    }

    fn show_level(&self, trigger: usize, name: &str) {
        let mut bar = [b'-'; 17];
        let res: usize = trigger / 15;
        bar[0..res].fill(b'*');
        println!("{} -> [{}]", name, from_utf8(&bar).unwrap())
    }
}

impl Controller {
    pub fn new(buf: [u8; 78]) -> Self {
        // Button::new(&buf);
        Self {
            stick: Stick::new(&buf),
            // Trigger::new(buf[8], buf[9]);
            trigger: Trigger {
                l2: buf[8],
                r2: buf[9],
            },
            sensor: Sensor::new(buf),
            // button: Button::new(),
        }
    }
}
