const PS4_SEND_BUFFER_SIZE: usize = 77;
const PS4_HID_BUFFER_SIZE: usize = 50;

enum HidCmdCode {
    SetReport = 0x50,
    TypeOutput = 0x02,
    TypeFeature = 0x03,
}

enum HidCmdIdentifier {
    Ps4Enable = 0xF4,
    Ps4Control = 0x11,
}

#[derive(Debug)]
pub struct HidCmd {
    pub code: u8,
    pub identifier: u8,
    pub data: [u8; PS4_SEND_BUFFER_SIZE],
}

impl Default for HidCmd {
    fn default() -> Self {
        Self {
            code: Default::default(),
            identifier: Default::default(),
            data: [0; PS4_SEND_BUFFER_SIZE],
        }
    }
}

pub enum ControlPacketIndex {
    SmallRumble = 5,
    LargeRumble = 6,

    Red = 7,
    Green = 8,
    Blue = 9,

    FlashOnTime = 10,
    FlashOffTime = 11,
}

/*
void ps4ConnectEvent(uint8_t isConnected);
void ps4PacketEvent(ps4_t ps4, ps4_event_t event);
void parsePacket(uint8_t* packet);
void sppInit();
void ps4_l2cap_init_services();
void ps4_l2cap_deinit_services();
void ps4_l2cap_send_hid(hid_cmd_t *hid_cmd, uint8_t len);
*/
