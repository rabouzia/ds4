use bitflags::bitflags;
use derive_more::Constructor;
use hidapi::HidApi;
use mouse_rs::{Mouse, types::keys::Keys};

// #[derive(Constructor)]
#[derive(Constructor)]
struct Analog {
    button: u8,
    x: u8,
    y: u8,
}
struct Stick {
    l3: Analog,
    r3: Analog,
}

impl Stick {
    fn new(buf: &[u8; 10]) -> Self {
        Self {
            l3: Analog::new(0, buf[1], buf[2]),
            r3: Analog::new(0, buf[3], buf[4]),
        }
    }
    fn print(&self) {
        if self.l3.x > 0 {}
        if self.l3.y > 0 {}
        if self.r3.x > 0 {}
        if self.r3.y > 0 {}
    }
}
#[derive(Constructor)]
struct Controller {
    // data: Ps4,
    // event: Event,
    // output: Command,
    stick: Stick,
    flags: Button,
}

enum PacketIndex {
    AnalogSticklx = 13,
    AnalogStickly = 14,
    AnalogStickrx = 15,
    AnalogStickry = 16,

    ButtonStandard = 17,
    ButtonExtra = 18,
    ButtonPS = 19,

    Analogl2 = 20,
    Analogr2 = 21,

    Status = 42,
}

struct Status {
    battery: u8,
    charging: u8,
    audio: u8,
    mic: u8,
}

struct Gyroscope {
    z: u16,
}

struct Accelerometer {
    x: u16,
    y: u16,
    z: u16,
}

struct Sensor {
    accelerometer: Accelerometer,
    gyroscope: Gyroscope,
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

bitflags! {

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Button: u8 {

        const up        = 0b0000_0000;
        const down      = 0b0000_0100;
        const right     = 0b0000_0010;
        const left      = 0b0000_0110;

        const square    = 0b0001_0000;
        const cross     = 0b0010_0000;
        const circle    = 0b0100_0000;
        const triangle  = 0b1000_0000;

        // const upright   = 0b0000_0001;
        // const upleft    = 0b0000_0111;
        // const downright = 0b0000_0011;
        // const downleft  = 0b0000_0101;

        // const l1        = 0b0000_0001;
        // const l2        = 0b0000_0100;
        // const r1        = 0b0000_0010;
        // const r2        = 0b0000_1000;

        // const share     = 0b0001_0000;
        // const option    = 0b0010_0000;
        // const l3        = 0b0100_0000; 64
        // const r3        = 0b1000_0000; 128

        // const ps        = 0b0001_0000;
        // const touchpad  = 0b0010_0000;

    }
}

impl Button {
    fn print(&self) {
        // if self.contains(Button::up) {
        //     print!("⬆️");
        // }
        if self.contains(Button::down) {
            print!("⬇️");
        }
        if self.contains(Button::right) {
            print!("➡️");
        }
        if self.contains(Button::left) {
            print!("⬅️");
        }
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

        println!();
        // todo!()
    }
}

fn main() {
    let api = HidApi::new().unwrap();
    let manette: Controller;
    // manette.flags.is_
    let device = api.open(1356, 1476).unwrap();
    let mut buf: [u8; 10] = [0u8; 10];
    // let Vec
    let mouse = Mouse::new();
    // Read: [1, 158, 130, 129, 129, 8, 0, 0, 0, 0]
    let h = 2056;
    let l = 1329;

    loop {
        let res: usize = device.read(&mut buf[..]).unwrap();
        println!("Read: {:?}", &buf[..res]);

        let x = buf[1];
        let y = buf[2];
        let stick = Stick::new(&buf);

        // Write data to device
        let buf = [0u8, 1, 2, 3, 4];
        let res = device.write(&buf).unwrap();
        println!("Wrote: {:?} byte(s)", res);
        // mouse.move_to(x as i32, y as i32).expect("Unable to move mouse");

        // mouse.press(&Keys::RIGHT).expect("Unable to press button");
        // mouse
        //     .release(&Keys::RIGHT)
        //     .expect("Unable to release button");

        let touches = Button::from_bits_retain(buf[5]);
        let stick =
        // buf[1] -> l3 horizontal
        // buf[2] -> l3 vertical
        // buf[3] -> r3 horiz
        // buf[4] -> r3 vert

        print!("pressed:");
        touches.print();
    }
}
// 24  0001 1000 square
// 40  0010 1000 cross
// 72  0100 1000 circle
// 136 1000 1000 triangle

// joystick
// battery
// couleur
// gachette
