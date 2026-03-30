mod button;
mod controller;
mod event;
mod sensor;
mod sound;
mod stick;
use controller::Controller;
use hidapi::HidApi;

fn main() {
    let api = HidApi::new().unwrap();
    let device = api.open(1356, 1476).unwrap();
    let mut buf: [u8; 78] = [0u8; 78];
    loop {
        let res: usize = device.read(&mut buf[..]).unwrap();
        let manette = Controller::new(buf);
        println!("Read: {:?}", &buf[..res]);
        manette.stick.use_as_mouse(buf, 50.0);
        manette.trigger.print_trigger();
    }
}