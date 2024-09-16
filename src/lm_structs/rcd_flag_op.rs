#[repr(C)]
pub struct RcdFlagOp {
    pub flag_id: u16,
    pub unknown1: u8,
    pub unknown2: u8,
    pub flag_op: i32,
    pub flag_value: u8,
    pub unknown3: u8,
    pub unknown4: u8,
    pub unknown5: u8
}

impl std::fmt::Display for RcdFlagOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RcdFlagOp Object
    flag_id: {},
    unknown1: {},
    unknown2: {},
    flag_op: {},
    flag_value: {},
    unknown3: {},
    unknown4: {},
    unknown5: {}",
               self.flag_id,
               self.unknown1,
               self.unknown2,
               self.flag_op,
               self.flag_value,
               self.unknown3,
               self.unknown4,
               self.unknown5)
    }
}