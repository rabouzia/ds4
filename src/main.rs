mod button;
mod controller;
mod event;
mod sensor;
mod sound;
mod stick;
use button::Button;
use hidapi::HidApi;
use stick::Stick;

fn main() {
    let api = HidApi::new().unwrap();

    let device = api.open(1356, 1476).unwrap();
    let mut buf: [u8; 10] = [0u8; 10];
    let manette = Stick::new(&buf);

    // Read: [1, 158, 130, 129, 129, 8, 0, 0, 0, 0]

    loop {
        let res: usize = device.read(&mut buf[..]).unwrap();
        println!("Read: {:?}", &buf[..res]);

        manette.use_as_mouse(buf, 50.0);
        let touches = Button::from_bits_retain(buf[5]);

        // let stick =
        // print!("pressed:");
        touches.print();
    }
}

// buf[1] -> l3 horizontal
// buf[2] -> l3 vertical
// buf[3] -> r3 horiz
// buf[4] -> r3 vert

// let x = (buf[1] as f32 / 255.0 * h as f32) as i32;
// let y = (buf[2] as f32 / 255.0 * l as f32) as i32;
// let stick = Stick::new(&buf);

// // Write data to device
// // let buf = [0u8, 1, 2, 3, 4];
// let res = device.write(&buf).unwrap();
// println!("Wrote: {:?} byte(s)", res);
// mouse.move_to(x, y).expect("Unable to move mouse");

// std::thread::sleep(std::time::Duration::from_millis(5));
// 24  0001 1000 square
// 40  0010 1000 cross
// 72  0100 1000 circle
// 136 1000 1000 triangle

// joystick
// battery
// couleur
// gachette
