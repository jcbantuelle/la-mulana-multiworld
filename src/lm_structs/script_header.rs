#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScriptSubHeader {
    pub pointer: usize,
    pub data_num: i32,
    pub font_num: i32
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScriptHeader {
    pub scr_addr: usize,
    pub sub_num: i32,
    pub all_font_num: i32,
    pub data: usize
}