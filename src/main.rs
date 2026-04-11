mod button;
mod controller;
mod event;
mod sensor;
mod sound;
mod stick;
mod test;
use controller::Controller;
use hidapi::{HidApi, HidDevice};

fn set_buf(device: &HidDevice) {
    let mut enable_buf: [u8; 78] = [0u8; 78];
    enable_buf[0] = 0x14;
    enable_buf[1] = 0x02;
    enable_buf[2] = 0xF4;
    let _ = device.send_feature_report(&enable_buf);
}

/*
    buf = [0] * 78
    buf[0]  = 0x11    # report ID
    buf[1]  = 0xC0    # bluetooth header
    buf[2]  = 0x00    # reserved
    buf[3]  = 0x07    # enable rumble + LED flags



    if bluetooth:
        # bluetooth CRC must include 0xA2 prefix (not sent, only for checksum)
        crc_data = [0xA2] + buf[:74]
        checksum = crc32(crc_data)
        buf[74] = checksum & 0xFF
        buf[75] = (checksum >> 8) & 0xFF
        buf[76] = (checksum >> 16) & 0xFF
        buf[77] = (checksum >> 24) & 0xFF
    # USB: no CRC needed, controller accepts report as-is

    device.write(bytes(buf))

*/

// bt :1476 , usb : 0x05C4
fn main() {
    let api = HidApi::new().unwrap();
    let device: HidDevice = api.open(1356, 0x05C4).unwrap();
    let mut buf: [u8; 78] = [0u8; 78];
    set_buf(&device);
    // let mut frame = 0;
    let dt = 1.0 / 250.0;
    loop {
        // frame += 1;

        let _: usize = device.read(&mut buf[..]).unwrap();
        let manette = Controller::new(buf);
        manette.send_data(&device, "red", "low", "slow");

        // println!("Read: {:?}", &buf[..]);
        // manette.stick.use_as_mouse(buf, 50.0);
        // manette.trigger.print_trigger();
        // if frame % 20 == 0 {
        // manette.sensor.update_and_print(buf, dt);
        // manette.sensor.accelerometer.print();
        // manette.sensor.gyroscope.print();

        // }
    }
}
