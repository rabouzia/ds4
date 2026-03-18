enum PacketIndex {
    AnalogSticklx = 13,
    AnalogStickly = 14,
    AnalogStickrx = 15,
    AnalogStickry = 16,

    ButtonStandard = 17,
    ButtonExtra = 18,
    ButtonPS = 19,

    Analogl2 = 20,
    Analogr2 = 21,

    Status = 42,
}

enum ButtonMask {
    MaskUp = 0,
    MaskRight = 0b0000_0010,
    MaskDown = 0b0000_00100,
    MaskLeft = 0b0000_0110,

    MaskUpRight = 0b0000_0001,
    MaskDownRight = 0b0000_0011,
    MaskUpLeft = 0b0000_00111,
    MaskDownLeft = 0b0000_0101,
}
