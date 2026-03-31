pub struct Gyroscope {
    x: i16,
    y: i16,
    z: i16,
}

impl Gyroscope {
    pub fn new(buf: [u8; 78]) -> Self {
        Self {
            x: ((buf[14] as i16) << 8 | buf[13] as i16) as i16,
            y: ((buf[16] as i16) << 8 | buf[15] as i16) as i16,
            z: ((buf[18] as i16) << 8 | buf[17] as i16) as i16,
        }
    }
    pub fn print(&self) {
        println!("Accel X:{} Y:{} Z:{}", self.x, self.y, self.z);
    }
}

pub struct Accelerometer {
    x: i16,
    y: i16,
    z: i16,
}

impl Accelerometer {
    pub fn new(buf: [u8; 78]) -> Self {
        Self {
            x: ((buf[20] as i16) << 8 | buf[19] as i16) as i16,
            y: ((buf[22] as i16) << 8 | buf[21] as i16) as i16,
            z: ((buf[24] as i16) << 8 | buf[23] as i16) as i16,
        }
    }
    pub fn print(&self) {
        println!("Gyro X:{} Y:{} Z:{}", self.x, self.y, self.z);
    }
}

pub struct Sensor {
    pub accelerometer: Accelerometer,
    pub gyroscope: Gyroscope,
    pub pitch: f32,
    pub roll: f32,
}

impl Sensor {
    pub fn new(buf: [u8; 78]) -> Self {
        Self {
            accelerometer: Accelerometer::new(buf),
            gyroscope: Gyroscope::new(buf),
            pitch: 0.0,
            roll: 0.0,
        }
    }
    pub fn update_and_print(&mut self, buf: [u8; 78], dt: f32) {
        self.gyroscope = Gyroscope::new(buf);
        self.accelerometer = Accelerometer::new(buf);

        let x_dps = self.gyroscope.x as f32 / 16.0;
        let y_dps = self.gyroscope.y as f32 / 16.0;

        let accel_pitch = (self.accelerometer.x as f32).atan2(self.accelerometer.z as f32).to_degrees();
        let accel_roll  = (self.accelerometer.y as f32).atan2(self.accelerometer.z as f32).to_degrees();

        self.pitch = 0.98 * (self.pitch + x_dps * dt) + 0.02 * accel_pitch;
        self.roll  = 0.98 * (self.roll  + y_dps * dt) + 0.02 * accel_roll;

        self.gyroscope.print();
        self.accelerometer.print();
        println!("Pitch:{:.1}° Roll:{:.1}°", self.pitch, self.roll);
    }
}
