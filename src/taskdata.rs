type EventWithBool = extern "C" fn(&TaskData) -> bool;
type Event = extern "C" fn(&TaskData) -> ();
type Void = extern "C" fn();

pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct VectorColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub struct Clip {
    pub tx: f32,
    pub ty: f32,
    pub bx: f32,
    pub by: f32
}

pub struct Image {
    pub width: f32,
    pub height: f32,
    pub vcol: [VectorColor;4],
    pub vc: u32,
    pub pc: u32,
    pub clip: Clip,
    pub tex_addr: u32,
    pub base_obj: [u32:6],
    pub text_f: u32,
    pub rInvWidth: f32,
    pub rInvHeight: f32
}

pub struct TaskId {
    pub no: i16,
    pub undefined1: u8,
    pub undefined2: u8,
    pub uid: u32,
    pub live: u8,
    pub undefined3: u8,
    pub undefined4: u8,
    pub undefined5: u8
}

pub struct MyPro {
    pub load_f_time: u32,
    pub load_s_time: u32,
    pub load_d_time: u32,
    pub load_h_time: u32,
    pub load_r_time: u32,
    pub load_final_time: u32,
    pub function: Void
}

pub struct TaskData {
    pub init: Event,
    pub ffunc: EventWithBool,
    pub sfunc: Event,
    pub rfunc: EventWithBool,
    pub dfunc: Event,
    pub finalfunc: Event,
    pub hfunc: Event,
    pub sta: u32,
    pub sbuff: [i32;32],
    pub addr: [extern "C" fn();16],
    pub buff: [i32;32],
    pub fbuff: [f32;32],
    pub pos: Point3d,
    pub pos_bk: Point3d,
    pub base: Point3d,
    pub hp: i32,
    pub image: Image,
    pub anime: u16,
    pub ptrn: u16,
    pub draw_f: u16,
    pub undefined1: u8,
    pub undefined2: u8,
    pub sort_z: i32,
    pub st_flag_num: u8,
    pub undefined3: u8,
    pub undefined4: u8,
    pub undefined5: u8,
    pub st_flags: Void,
    pub stop_flag: u32,
    pub resetflag: u32,
    pub sysfunc: Event,
    pub farstfunc: EventWithBool,
    pub backfunc: EventWithBool,
    pub hit_si: f32,
    pub system_buff: [i32;17],
    pub event_addr: Void,
    pub hit_data: i16,
    pub next: i16,
    pub back: i16,
    pub my_layer: i16,
    pub undefined6: u8,
    pub undefined7: u8,
    pub id: TaskId,
    pub nexttask: Event,
    pub backtask: Event,
    pub field_pri: u8,
    pub room_pri: u8,
    pub view_pri: u8,
    pub field_no: u8,
    pub room_no: u8,
    pub view_no: u8,
    pub undefined8: u8,
    pub undefined9: u8,
    pub cash_no: i32,
    pub end_flag_num: u8,
    pub undefined10: u8,
    pub undefined11: u8,
    pub undefined12: u8,
    pub end_flags: Event,
    pub my_draw_z: i32,
    pub sub_draw_z: i32,
    pub fx: f32,
    pub fy: f32,
    pub my_pro: MyPro
}
