pub struct Gyroscope {
    z: u16,
}

pub struct Accelerometer {
    x: u16,
    y: u16,
    z: u16,
}

pub struct Sensor {
    accelerometer: Accelerometer,
    gyroscope: Gyroscope,
}
