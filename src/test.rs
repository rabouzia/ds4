
#[cfg(test)]
mod tests {
    // use super::*;
    use hidapi::HidApi;
    use crate::Controller;

    fn open_controller() -> Option<hidapi::HidDevice> {
        let api = HidApi::new().ok()?;
        api.open(1356, 0x05C4).ok()
    }

    #[test]
    fn test_controller_connection() {
        let dev = open_controller();
        assert!(dev.is_some(), "Controller not connected");
    }

    #[test]
    fn test_read_report() {
        let device = match open_controller() {
            Some(d) => d,
            None => return,
        };

        let mut buf = [0u8; 78];
        let result = device.read_timeout(&mut buf, 500);

        assert!(result.is_ok());
    }

    #[test]
    fn test_led_packet_write() {
        let device = match open_controller() {
            Some(d) => d,
            None => return,
        };

        let mut controller = Controller::new([0u8; 78]);

        controller.packet[7] = 255;
        controller.packet[8] = 0;
        controller.packet[9] = 0;

        let result = device.write(&controller.packet);

        assert!(result.is_ok());
    }

    #[test]
    fn test_rumble_packet_write() {
        let device = match open_controller() {
            Some(d) => d,
            None => return,
        };

        let mut controller = Controller::new([0u8; 78]);

        controller.packet[5] = 50;
        controller.packet[6] = 120;

        let result = device.write(&controller.packet);

        assert!(result.is_ok());
    }
}