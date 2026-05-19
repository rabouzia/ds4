use ds4_core::controller::Controller;
use ds4_core::event::ControllerEvent;

fn main() {
    env_logger::init();
    
    let mut ctrl = Controller::new();
    // Initialize hidapi, open DS4 device
    // loop: read report -> ctrl.handle_report(&buf, dt) -> print events
}