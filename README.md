# ds4-driver
 
A DualShock 4 driver written in Rust, built to run everywhere.
 
## Goal
 
Most controller libraries lock you into one platform or hide the raw data behind abstractions. This driver works directly with HID reports, giving you full control over every button, stick, trigger, gyroscope, and LED — the same way on every target.
 
## Platform Support
 
| Platform | Transport | Status |
|----------|-----------|--------|
| macOS/Windows/Linux | Bluetooth / USB via `hidapi` | In progress |
| ESP (embedded) | Bare metal | Planned |
 
## Architecture
 
The driver is split into two clean layers:
 
- **Transport layer** — handles how bytes arrive (hidapi on desktop, bare metal on embedded)
- **Parser layer** — converts raw HID report bytes into a typed `ControllerState` — identical on all platforms
 
This means the parsing logic is written once and runs everywhere.
 
## What You Can Read
 
- All digital buttons (✕, ○, △, □, L1, R1, L2, R2, Share, Options, PS, touchpad, L3, R3)
- D-pad (8 directions)
- Analog sticks (left and right, X/Y axes)
- L2 / R2 analog triggers
- Gyroscope
- Accelerometer
- Battery status
 
## What You Can Control
 
- Rumble motors (small and large, independently)
- Lightbar color (RGB)
- Lightbar flash pattern (on/off timing)
 
## Built With
 
- [hidapi](https://crates.io/crates/hidapi) — cross-platform HID device access
 
 