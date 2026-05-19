use derive_more::Constructor;
use mouse_rs::{Mouse, types::keys::Keys};

#[derive(Constructor)]
pub struct Analog {
    x: u8,
    y: u8,
}
pub struct Stick {
    l3: Analog,
    r3: Analog,
}

impl Stick {
    pub fn new(buf: &[u8; 78]) -> Self {
        Self {
            l3: Analog::new(buf[1], buf[2]),
            r3: Analog::new(buf[3], buf[4]),
        }
    }
    pub fn print(&self) {
        if self.l3.x > 0 {}
        if self.l3.y > 0 {}
        if self.r3.x > 0 {}
        if self.r3.y > 0 {}
    }
    pub fn use_click(&self, mouse: &Mouse, buf: [u8; 78]) {
        mouse.press(&Keys::RIGHT).expect("Unable to press button");
        mouse
            .release(&Keys::RIGHT)
            .expect("Unable to release button");
    }
    pub fn use_mouse(&self, mouse: &Mouse, buf: [u8; 78], sensitivity: f32) {
        let mut dx = (buf[1] as f32 - 127.0) / 127.0;
        let mut dy = (buf[2] as f32 - 127.0) / 127.0;

        let len2: f32 = dx * dx + dy * dy;
        let dist: f32 = len2.sqrt();
        if dist < 0.25 {
            dx = 0.0;
            dy = 0.0;
        }

        let move_x = (dx * sensitivity) as i32;
        let move_y = (dy * sensitivity) as i32;

        let pos = mouse.get_position().unwrap();
        let x = pos.x;
        let y = pos.y;

        mouse.move_to(x + move_x, y + move_y).unwrap();
    }
    fn ds4_is_mouse(&self, buf: [u8; 78], sensitivity: f32) {
        let mouse: Mouse = Mouse::new();
        self.use_mouse(&mouse, buf, sensitivity);
        self.use_click(&mouse, buf)
    }
}
