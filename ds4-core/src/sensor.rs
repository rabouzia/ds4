use std::f32::consts::PI;

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
    pub fn to_dps(&self) -> (f32, f32, f32) {
        const SCALE: f32 = 131.0;
        (
            self.x as f32 / SCALE,
            self.y as f32 / SCALE,
            self.z as f32 / SCALE,
        )
    }
    pub fn print(&self) {
        let (x, y, z) = self.to_dps();
        println!("Gyro  X:{:.2}°/s  Y:{:.2}°/s  Z:{:.2}°/s", x, y, z);
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
            x: (buf[20] as i16) << 8 | buf[19] as i16,
            y: (buf[22] as i16) << 8 | buf[21] as i16,
            z: (buf[24] as i16) << 8 | buf[23] as i16,
        }
    }
    pub fn to_g(&self) -> (f32, f32, f32) {
        const SCALE: f32 = 16384.0;
        (
            self.x as f32 / SCALE,
            self.y as f32 / SCALE,
            self.z as f32 / SCALE,
        )
    }
    pub fn print(&self) {
        let (x, y, z) = self.to_g();
        println!("Accel X:{:.3}g  Y:{:.3}g  Z:{:.3}g", x, y, z);
    }
    pub fn pitch(&self) -> f32 {
        let (ax, _, az) = self.to_g();
        ax.atan2(az) * (180.0 / PI)
    }
    pub fn roll(&self) -> f32 {
        let (ax, _, az) = self.to_g();
        ax.atan2(az) * (180.0 / PI)
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
        let accelerometer = Accelerometer::new(buf);
        let gyroscope = Gyroscope::new(buf);
        let pitch = accelerometer.pitch();
        let roll = accelerometer.roll();
        Self {
            accelerometer,
            gyroscope,
            pitch,
            roll,
        }
    }
    pub fn update(&mut self, buf: [u8; 78], dt: f32) {
        self.gyroscope = Gyroscope::new(buf);
        self.accelerometer = Accelerometer::new(buf);
        
        let (gx, gy, _gz) = self.gyroscope.to_dps();
        let accel_pitch = self.accelerometer.pitch();
        let accel_roll = self.accelerometer.roll();

        self.pitch = 0.98 * (self.pitch + gx * dt) + 0.02 * accel_pitch;
        self.roll = 0.98 * (self.roll + gy * dt) + 0.02 * accel_roll;
    }

    pub fn print(&mut self, buf: [u8; 78], dt: f32) {
        self.update(buf, dt);
        self.gyroscope.print();
        self.accelerometer.print();
        
        println!("Pitch:{:.1}° Roll:{:.1}°", self.pitch, self.roll);
    }
}
