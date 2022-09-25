#[repr(C)]
pub struct RectBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl std::fmt::Display for RectBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RectBox Object
        x: {},
        y: {},
        w: {},
        h: {}",
               self.x,
               self.y,
               self.w,
               self.h)
    }
}

#[repr(C)]
pub struct HitBox {
    pub rect: RectBox, // 0x0 (0) - 16 bytes
    pub dmg: i32, // 0x10 (16) - 4 bytes
    pub flag: u32, // 0x14 (20) - 4 bytes
    pub td: usize, // 0x18 (24) - 4 bytes
    pub w_id: i32, // 0x1C (28) - 4 bytes
    pub attack_element: u32, // 0x20 (32) - 4 bytes
    pub hari_houkou: u8, // 0x24 (36) - 1 byte
    pub undefined1: u8, // 0x25 (37) - 1 byte
    pub undefined2: u8, // 0x26 (38) - 1 byte
    pub undefined3: u8 // 0x27 (39) - 1 byte
}

impl std::fmt::Display for HitBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HitBox Object
    rect: {},
    dmg: {},
    flag: {},
    td: {},
    w_id: {},
    attack_element: {},
    hari_houkou: {},
    undefined1: {},
    undefined2: {},
    undefined3: {}",
               self.rect,
               self.dmg,
               self.flag,
               self.td,
               self.w_id,
               self.attack_element,
               self.hari_houkou,
               self.undefined1,
               self.undefined2,
               self.undefined3)
    }
}