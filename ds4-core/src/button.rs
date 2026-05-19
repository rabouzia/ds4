use bitflags::bitflags;

// use crate::button;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Button: u8 {

    const square    = 0b0001_0000;
    const cross     = 0b0010_0000;
    const circle    = 0b0100_0000;
    const triangle  = 0b1000_0000;

    const up        = 0b0000_0001;
    const down      = 0b0000_0100;
    const right     = 0b0000_0010;
    const left      = 0b0000_0110;


    const upright   = 0b0000_0001;
    const upleft    = 0b0000_0111;
    const downright = 0b0000_0011;
    const downleft  = 0b0000_0101;


    // buf 6
    const l1        = 0b0000_0001;
    const r1        = 0b0000_0010;
    const l3        = 0b0100_0000; //64
    const r3        = 0b1000_0000; //128
    const share     = 0b0001_0000; //32
    const option    = 0b0010_0000; //16


    // buf 7
    const ps        = 0b0001_0000; // 1
    const touchpad  = 0b0010_0000; // 2

    }
}

impl Button {
    const BUTTON_MAP: &'static [(Button, &'static str)] = &[
        (Button::square, "□"),
        (Button::cross, "✕"),
        (Button::circle, "○"),
        (Button::triangle, "🔼"),
        (Button::up, "⬆️"),
        (Button::down, "⬇️"),
        (Button::right, "➡️"),
        (Button::left, "⬅️"),
        (Button::upright, "↗️"),
        (Button::upleft, "↖️"),
        (Button::downright, "↘️"),
        (Button::downleft, "↙️"),
        (Button::l1, "L1"),
        (Button::r1, "R1"),
        (Button::l3, "L3"),
        (Button::r3, "R3"),
        (Button::share, "SHARE"),
        (Button::option, "OPTION"),
        (Button::ps, "PS"),
        (Button::touchpad, "PAD"),
    ];

    pub fn print(&self) {
        for (btn, label) in Self::BUTTON_MAP {
            if self.contains(*btn) {
                print!("{label} ");
            }
        }
        println!();
    }
}
