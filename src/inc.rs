pub struct Analogstick {
    lx: u8,
    ly: u8,
    rx: u8,
    ry: u8,
}

struct Analogbutton {
    l2: u8,
    r2: u8,
}

struct Analog {
    button: Analogbutton,
    stick: Analogstick,
}

struct Button {
    up: u8,
    down: u8,
    right: u8,
    left: u8,

    square: u8,
    cross: u8,
    circle: u8,
    triangle: u8,

    upright: u8,
    upleft: u8,
    downright: u8,
    downleft: u8,

    l1: u8,
    l2: u8,
    r1: u8,
    r2: u8,

    share: u8,
    option: u8,
    l3: u8,
    r3: u8,

    ps: u8,
    touchpad: u8,
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

struct Command {
    small_rumble: u8,
    large_rumble: u8,
    r: u8,
    g: u8,
    b: u8,
    flash_off: u8,
    flash_on: u8,
}

struct Event {
    button_down: Button,
    button_up: Button,
    analog_move: Button,
}

struct Ps4 {
    analog: Analog,
    button: Button,
    status: Status,
    sensor: Sensor,
    last_packet: u8,
}

struct Controller {
    data: Ps4,
    event: Event,
    output: Command,
}
