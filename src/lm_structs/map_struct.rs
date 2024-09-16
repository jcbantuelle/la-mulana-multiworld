#[repr(C)]
pub struct MapStruct {
    pub next: usize,
    pub prev: usize,
    pub child_object: usize,
    pub in_use: u8,
    pub unknown1: u8,
    pub unknown2: u16,
    pub scoped_db_idx: i32,
    pub uid_of_child: i32,
    pub rcd_object: usize
}

impl std::fmt::Display for MapStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MapStruct Object
    next: {},
    prev: {},
    child_object: {},
    in_use: {},
    unknown1: {},
    unknown2: {},
    scoped_db_idx: {},
    uid_of_child: {},
    rcd_object: {}",
               self.next,
               self.prev,
               self.child_object,
               self.in_use,
               self.unknown1,
               self.unknown2,
               self.scoped_db_idx,
               self.uid_of_child,
               self.rcd_object)
    }
}