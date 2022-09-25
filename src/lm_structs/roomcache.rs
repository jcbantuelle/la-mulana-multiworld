#[repr(C)]
pub struct Camera {
    pub x: f32,
    pub y: f32
}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Camera Object
        x: {},
        y: {}",
               self.x,
               self.y)
    }
}

#[repr(C)]
pub struct RoomCache {
    pub room_no: u8, // 0x0 (0) - 1 byte
    pub undefined1: u8, // 0x1 (1) - 1 byte
    pub view_w: u16, // 0x2 (2) - 2 bytes
    pub view_h: u16, // 0x4 (4) - 2 bytes
    pub undefined2: u8, // 0x6 (6) - 1 byte
    pub undefined3: u8, // 0x7 (7) - 1 byte
    pub room_w: f32, // 0x8 (8) - 4 bytes
    pub room_h: f32, // 0xC (12) - 4 bytes
    pub fogmask: [i32;2], // 0x10 (16) - 8 bytes
    pub cam: Camera // 0x18 (24) - 8 bytes
}

impl std::fmt::Display for RoomCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RoomCache Object
    room_no: {},
    undefined1: {},
    view_w: {},
    view_h: {},
    undefined2: {},
    undefined3: {},
    room_w: {},
    room_h: {},
    fogmask: {:?},
    cam: {}",
               self.room_no,
               self.undefined1,
               self.view_w,
               self.view_h,
               self.undefined2,
               self.undefined3,
               self.room_w,
               self.room_h,
               self.fogmask,
               self.cam)
    }
}