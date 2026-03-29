use bitflags::bitflags;

bitflags! {

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct Button: u8 {

    const square    = 0b0001_0000;
    const cross     = 0b0010_0000;
    const circle    = 0b0100_0000;
    const triangle  = 0b1000_0000;

    const up        = 0b0000_0000;
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
    pub fn print(&self) {
        if self.contains(Button::square) {
            print!("⏹");
        }
        if self.contains(Button::cross) {
            print!("X");
        }
        if self.contains(Button::circle) {
            print!("⭕️");
        }
        if self.contains(Button::triangle) {
            print!("🔼");
        }
        if self.contains(Button::up) {
            print!("⬆️");
        }
        if self.contains(Button::down) {
            print!("⬇️");
        }
        if self.contains(Button::right) {
            print!("➡️");
        }
        if self.contains(Button::left) {
            print!("⬅️");
        }
        if self.contains(Button::upright) {
            print!("↗️");
        }
        if self.contains(Button::upleft) {
            print!("↖️");
        }
        if self.contains(Button::downright) {
            print!("↘️");
        }
        if self.contains(Button::downleft) {
            print!("↙️");
        }
        if self.contains(Button::l1) {
            print!("L1");
        }
        if self.contains(Button::r1) {
            print!("R1");
        }
        if self.contains(Button::l3) {
            print!("L3");
        }
        if self.contains(Button::r3) {
            print!("R3");
        }
        if self.contains(Button::share) {
            print!("SHARE");
        }
        if self.contains(Button::option) {
            print!("OPTION");
        }
        if self.contains(Button::ps) {
            print!("PS");
        }
        if self.contains(Button::touchpad) {
            print!("PAD");
        }

        println!();
        // todo!()
    }
}
