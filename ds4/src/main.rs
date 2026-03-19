// use modules;
use std::{
    mem,
    
    sync::atomic::{AtomicBool, Ordering},
    vec,
};
use utils::{inc,int, int::HidCmd,int::ControlPacketIndex,};

static IS_ACTIVE: AtomicBool = AtomicBool::new(false);
static HID_CMD_PAYLOAD_PS4_ENABLE: &[usize] = &[0x43, 0x02];
static HID_CMD_CODE_SET_REPORT: u8 = 0x50;
static HID_CMD_CODE_TYPE_OUTPUT: u8 = 0x02;
static HID_CMD_CODE_TYPE_FEATURE: u8 = 0x02;
static HID_CMD_IDENTIFIER_CONTROL: u8 = 0x11;
/*
static ps4_connection_callback_t ps4_connection_cb = NULL;
static ps4_connection_object_callback_t ps4_connection_object_cb = NULL;
static void* ps4_connection_object = NULL;

static ps4_event_callback_t ps4_event_cb = NULL;
static ps4_event_object_callback_t ps4_event_object_cb = NULL;
static void* ps4_event_object = NULL;
*/

fn ps4_l2cap_send_hid(hidCommand : int::HidCmd, length: i32){
    todo!()
    // did understand whatsup here
}
fn ps4_cmd(cmd: inc::Command) {
    let mut data = [0; 77];
    data[0] = 0x80;
    data[1] = 0x00;
    data[2] = 0xFF;
    data[ControlPacketIndex::SmallRumble as usize] = cmd.small_rumble;
    data[ControlPacketIndex::LargeRumble as usize] = cmd.large_rumble;
    data[ControlPacketIndex::LargeRumble as usize] = cmd.large_rumble;

    data[ControlPacketIndex::Red as usize] = cmd.r;
    data[ControlPacketIndex::Green as usize] = cmd.g;
    data[ControlPacketIndex::Blue as usize] = cmd.b;

    data[ControlPacketIndex::FlashOnTime as usize] = cmd.flash_on;
    data[ControlPacketIndex::FlashOffTime as usize] = cmd.flash_off;
    let length = 0;
    let code = HID_CMD_CODE_SET_REPORT | HID_CMD_CODE_TYPE_OUTPUT;
    let identifier = HID_CMD_IDENTIFIER_CONTROL;
    let hidCommand = int::HidCmd {
        code,
        identifier,
        data,
    };
    ps4_l2cap_send_hid(hidCommand, length);
}

fn ps4_set_led(r: u8, g: u8, b: u8) {
    let mut cmd = inc::Command::default();

    cmd.r = r;
    cmd.g = g;
    cmd.b = b;

    ps4_cmd(cmd);
}
fn ps4_enable() {
    let hidCommand : HidCmd = int::HidCmd::default;
    hidCommand.code = HID_CMD_CODE_SET_REPORT| HID_CMD_CODE_TYPE_FEATURE;
    // let length: usize = mem::size_of::<HID_CMD_PAYLOAD_PS4_ENABLE>();
    ps4_set_led(32, 32, 200);
    todo!();
}
fn ps4IsConnected(){
    todo!()
}
fn ps4_connect_event(is_connected: bool) {
    if is_connected {
        ps4_enable();
    } else {
        IS_ACTIVE.store(false, Ordering::Relaxed);
    }
}
fn spp_init(){
    todo!()
}
fn ps4_l2cap_init_services(){
    todo!()
}

fn ps4_init(){
    spp_init();
    ps4_l2cap_init_services();

}
fn ps4_set_connection_callback() {}
fn ps4_set_connection_object_callback() {}
fn ps4_set_event_callback() {}
fn ps4_set_event_object_callback() {}
fn ps4_set_output() {}
fn ps4_set_bluetooth_macadress() {}

fn main() {
    println!("Hello, world!");
}
