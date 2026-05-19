#![no_std]
#![no_main]

use ds4_core::controller::Controller;
use defmt::info;
use panic_defmt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut ctrl = Controller::new();
    
    // Initialize HAL, UART/SPI, hardware timer for dt
    
    loop {
        // Read bytes into buffer
        // let dt = timer.read_elapsed_seconds();
        // ctrl.handle_report(&buf, dt);
        // info!("Pitch: {}, Roll: {}", ctrl.pitch, ctrl.roll);
    }
}