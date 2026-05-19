use crate::{
    button::Button,
    event::{Color, Event, Flash, Rumble},
    sensor::{Accelerometer, Sensor},
    sound::Sound,
    stick::Stick,
};
use crc32fast::hash;
use hidapi::{HidApi, HidDevice};
use std::str::from_utf8;
// use derive_more::Constructor;
struct Status {
    battery: u8,
    charging: u8,
}

impl Status {
    pub fn new() -> Self {
        Self {
            battery: 0,
            charging: 0,
        }
    }
    pub fn battery_percentage(&self, buf: u8) {
        let raw = buf & 0x0F; // 0-15
        println!("battery lvl is {}", ((raw as f32 / 15.0) * 100.0) as u8);
    }
    pub fn is_charging(&self, buf: u8) {
        println!("is charging {}", (buf & 0xF0) >> 4 == 1);
    }
}

pub struct Trigger {
    l2: u8,
    r2: u8,
}
pub struct Controller {
    pub sensor: Sensor,
    // pub sound: Sound,
    pub event: Event,
    pub button: Button,
    pub stick: Stick,
    pub trigger: Trigger,
    pub status: Status,
    pub packet: [u8; 78],
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
            event: Event {
                color: Color { r: 0, g: 0, b: 0 },
                rumble: Rumble { small: 0, large: 0 },
                flash: Flash { off: 0, on: 0 },
            },
            packet: Self::build_packet(),
            button: Button::empty(),
            status: Status::new(),
        }
    }
    pub fn battery(&self) {
        // buf[12] → battery level
        // buf[30] → charging
        self.status.battery_percentage(self.packet[12]);
        self.status.is_charging(self.packet[30]);
    }
    fn build_packet() -> [u8; 78] {
        let mut buf = [0u8; 78];
        let mut crc_input = [0u8; 75];

        buf[0] = 0x11;
        buf[1] = 0xC0;
        buf[2] = 0x00;
        buf[3] = 0x07;

        crc_input[0] = 0xA2;
        crc_input[1..].copy_from_slice(&buf[..74]);

        let checksum = hash(&crc_input);
        buf[74] = (checksum & 0xFF) as u8;
        buf[75] = ((checksum >> 8) & 0xFF) as u8;
        buf[76] = ((checksum >> 16) & 0xFF) as u8;
        buf[77] = ((checksum >> 24) & 0xFF) as u8;
        buf
    }

    pub fn send_data(mut self, device: &HidDevice, colored: &str, rumble: &str, flash: &str) {
        let (r, g, b) = self.event.rgb(colored);
        let (s, l) = self.event.rumble(rumble);
        let (f1, f2) = self.event.flash(flash);
        self.packet[5] = s;
        self.packet[6] = l;

        self.packet[7] = r;
        self.packet[8] = g;
        self.packet[9] = b;

        self.packet[10] = f1;
        self.packet[11] = f2;

        let _: usize = device.write(&mut self.packet[..]).unwrap();
    }
}
