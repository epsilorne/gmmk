#[allow(dead_code)]
pub enum KeyCode {
    Esc = 0x36,
    K1 = 0x39,
    K2 = 0x3c,
    K3 = 0x3f,
    K4 = 0x42,
    K5 = 0x45,
    K6 = 0x48,
    K7 = 0x4b,
    K8 = 0x4e,
    K9 = 0x51,
    K0 = 0x54,
    Minus = 0x57,
    Equals = 0x5a,
    Backspace = 0x26,
    Tab = 0x69,
    Q = 0x6c,
    W = 0x6f,
    E = 0x72,
    R = 0x75,
    T = 0x78,
    Y = 0x7b,
    U = 0x7e,
    I = 0x81,
    O = 0x84,
    P = 0x87,
    OpenSquareBracket = 0x8a,
    ClosedSquareBracket = 0x8d,
    BackSlash = 0x90,
    CapsLock = 0x9c,
    A = 0x9f,
    S = 0xa2,
    D = 0xa5,
    F = 0xa8,
    G = 0xab,
    H = 0xae,
    J = 0xb1,
    K = 0xb4,
    L = 0xb7,
    Semicolon = 0xba,
    Quote = 0xbd,
    Enter = 0xf3,
    LShift = 0xcf,
    Z = 0xd2,
    X = 0xd5,
    C = 0xd8,
    V = 0xdb,
    B = 0xde,
    N = 0xe1,
    M = 0xe4,
    Comma = 0xe7,
    Dot = 0xea,
    ForwardSlash = 0xed,
    RShift = 0xf0,
    LCtrl = 0x02,
    Win = 0x05,
    LAlt = 0x08,
    Space = 0x0b,
    RAlt = 0x0e,
    Fn = 0x11,
    App = 0x14,
    RCtrl = 0x17,
}

impl KeyCode {
    pub fn sets_other_bit(&self) -> bool {
        matches!(
            self,
            Self::Backspace
                | Self::LCtrl
                | Self::Win
                | Self::LAlt
                | Self::Space
                | Self::RAlt
                | Self::Fn
                | Self::App
                | Self::RCtrl
        )
    }
}

pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Colour { r, g, b }
    }

    pub fn sizeof(&self) -> u8 {
        (if self.r == 0 { 1 } else { 0 })
            + (if self.g == 0 { 1 } else { 0 })
            + (if self.b == 0 { 1 } else { 0 })
    }
}
