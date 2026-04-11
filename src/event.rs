#[derive(Default, Debug)]
pub struct Flash {
    pub off: u8,
    pub on: u8,
}
pub struct Rumble {
    pub small: u8,
    pub large: u8,
}
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Event {
    pub color: Color,
    pub flash: Flash,
    pub rumble: Rumble,
}

//  buf[7]  = r       # LED red   (0-255)
//  buf[8]  = g       # LED green (0-255)
//  buf[9]  = b       # LED blue  (0-255)

impl Event {
    pub fn rumble(&self, level: &str) -> (u8, u8) {
        match level {
            "low" => (20, 0),
            "heavy" => (80, 180),
            "sharp" => (5, 60),
            _ => (0, 0),
        }
    }
    pub fn flash(&self, level: &str) -> (u8, u8) {
        match level {
            "low" => (40, 40),
            "heavy" => (10, 10),
            "sharp" => (200, 40),
            _ => (0, 0),
        }
    }
    pub fn change_color(&self, choose: &str) {
        match choose {
            "blue" => Color { r: 0, g: 0, b: 255 },
            "red" => Color { r: 255, g: 0, b: 0 },
            "green" => Color { r: 0, g: 255, b: 0 },
            "yellow" => Color {
                r: 255,
                g: 255,
                b: 0,
            },
            "pink" => Color {
                r: 255,
                g: 128,
                b: 255,
            },
            "violet" => Color {
                r: 128,
                g: 0,
                b: 255,
            },
            "cyan" => Color {
                r: 0,
                g: 255,
                b: 255,
            },
            "white" => Color {
                r: 255,
                g: 255,
                b: 255,
            },
            _ => Color { r: 0, g: 0, b: 0 },
        };
    }
    pub fn rgb(&self, choose: &str) -> (u8, u8, u8) {
        self.change_color(choose);
        (self.color.r, self.color.g, self.color.b)
    }
}
