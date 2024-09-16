pub type EventWithBool = *mut usize;
pub type Event = *mut usize;
pub type Void = *mut usize;

#[repr(C)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl std::fmt::Display for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point3d Object
        x: {},
        y: {},
        z: {}",
        self.x,
        self.y,
        self.z)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct VectorColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl std::fmt::Display for VectorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VectorColor Object
        r: {},
        g: {},
        b: {},
        a: {}",
               self.r,
               self.g,
               self.b,
               self.a
        )
    }
}

#[repr(C)]
pub struct Clip {
    pub tx: f32,
    pub ty: f32,
    pub bx: f32,
    pub by: f32
}

impl std::fmt::Display for Clip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Clip Object
        tx: {},
        ty: {},
        bx: {},
        by: {}",
               self.tx,
               self.ty,
               self.bx,
               self.by
        )
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct Image {
    pub width: f32,
    pub height: f32,
    pub vcol: [VectorColor;4],
    pub vc: u32,
    pub pc: u32,
    pub clip: Clip,
    pub tex_addr: u32,
    pub base_obj: [u32;6],
    pub text_f: u32,
    pub rInvWidth: f32,
    pub rInvHeight: f32
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Image Object
        width: {},
        height: {},
        vcol: {:?},
        vc: {},
        pc: {},
        clip: {},
        tex_addr: {},
        base_obj: {:?},
        text_f: {},
        rInvWidth: {},
        rInvHeight: {}",
               self.width,
               self.height,
               self.vcol,
               self.vc,
            self.pc,
            self.clip,
            self.tex_addr,
            self.base_obj,
            self.text_f,
            self.rInvWidth,
            self.rInvHeight
        )
    }
}

#[repr(C)]
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

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TaskId Object
        no: {},
        undefined1: {},
        undefined2: {},
        uid: {},
        live: {},
        undefined3: {},
        undefined4: {},
        undefined5: {}",
               self.no,
               self.undefined1,
               self.undefined2,
               self.uid,
               self.live,
               self.undefined3,
               self.undefined4,
               self.undefined5
        )
    }
}

#[repr(C)]
pub struct MyPro {
    pub load_f_time: u32,
    pub load_s_time: u32,
    pub load_d_time: u32,
    pub load_h_time: u32,
    pub load_r_time: u32,
    pub load_final_time: u32,
    pub function: Void
}

impl std::fmt::Display for MyPro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyPro Object
        load_f_time: {},
        load_s_time: {},
        load_d_time: {},
        load_h_time: {},
        load_r_time: {},
        load_final_time: {},
        function: {:p}",
               self.load_f_time,
               self.load_s_time,
               self.load_d_time,
               self.load_h_time,
               self.load_r_time,
               self.load_final_time,
               self.function
        )
    }
}

#[repr(C)]
pub struct TaskData {
    pub init: Event, // 0x0 (0) - 4 bytes
    pub ffunc: EventWithBool, // 0x4 (4) - 4 bytes
    pub sfunc: Event, // 0x8 (8) - 4 bytes
    pub rfunc: EventWithBool, // 0xc (12) - 4 bytes
    pub dfunc: Event, // 0x10 (16) - 4 bytes
    pub finalfunc: Event, // 0x14 (20) - 4 bytes
    pub hfunc: Event, // 0x18 (24) - 4 bytes
    pub sta: u32, // 0x1C (28) - 4 bytes
    pub sbuff: [i32;32], // 0x20 (32) - 128 bytes
    pub addr: [usize;16], // 0xA0 (160) - 64 bytes
    pub buff: [i32;32], // 0xE0 (224) - 128 bytes
    pub fbuff: [f32;32], // 0x160 (352) - 128 bytes
    pub pos: Point3d, // 0x1E0 (480) - 12 bytes
    pub pos_bk: Point3d, // 0x1EC (492) - 12 bytes
    pub base: Point3d, // 0x1F8 (504) - 12 bytes
    pub hp: i32, // 0x204 (516) - 4 bytes
    pub image: Image, // 0x25C (604) - 88 bytes
    pub anime: u16, // 0x25E (606) - 2 bytes
    pub ptrn: u16, // 0x260 (608) - 2 bytes
    pub draw_f: u16, // 0x262 (610) - 2 bytes
    pub undefined1: u8, // 0x263 (611) - 1 byte
    pub undefined2: u8, // 0x264 (612) - 1 byte
    pub sort_z: i32, // 0x268 (616) - 4 bytes
    pub st_flag_num: u8, // 0x269 (617) - 1 byte
    pub undefined3: u8, // 0x26A (618) - 1 byte
    pub undefined4: u8, // 0x26B (619) - 1 byte
    pub undefined5: u8, // 0x26C (620) - 1 byte
    pub st_flags: Void, // 0x270 (624) - 4 bytes
    pub stop_flag: u32, // 0x274 (628) - 4 bytes
    pub resetflag: u32, // 0x278 (632) - 4 bytes
    pub sysfunc: Event, // 0x27C (636) - 4 bytes
    pub farstfunc: EventWithBool, // 0x280 (640) - 4 bytes
    pub backfunc: EventWithBool, // 0x284 (644) - 4 bytes
    pub hit_si: f32, // 0x288 (648) - 4 bytes
    pub system_buff: [i32;17], // 0x28C (652) - 68 bytes
    pub event_addr: usize, // 0x2D0 (720) - 4 bytes
    pub hit_data: i32, // 0x2D4 (724) - 4 bytes
    pub next: i16, // 0x2D8 (728) - 2 bytes
    pub back: i16, // 0x2DA (730) - 2 bytes
    pub my_layer: i16, // 0x2DC (732) - 2 bytes
    pub undefined6: u8, // 0x2DE (734) - 1 byte
    pub undefined7: u8, // 0x2DF (735) - 1 byte
    pub id: TaskId, // 0x2E0 (736) - 12 bytes
    pub nexttask: Event, // 0x2EC (748) - 4 bytes
    pub backtask: Event, // 0x2F0 (752) - 4 bytes
    pub field_pri: u8, // 0x2F4 (756) - 1 byte
    pub room_pri: u8, // 0x2F5 (757) - 1 byte
    pub view_pri: u8, // 0x2F6 (758) - 1 byte
    pub field_no: u8, // 0x2F7 (759) - 1 byte
    pub room_no: u8, // 0x2F8 (760) - 1 byte
    pub view_no: u8, // 0x2F9 (761) - 1 byte
    pub undefined8: u8, // 0x2FA (762) - 1 byte
    pub undefined9: u8, // 0x2FB (763) - 1 byte
    pub cash_no: i32, // 0x2FC (764) - 4 bytes
    pub end_flag_num: u8, // 0x300 (768) - 1 byte
    pub undefined10: u8, // 0x301 (769) - 1 byte
    pub undefined11: u8, // 0x302 (770) - 1 byte
    pub undefined12: u8, // 0x303 (771) - 1 byte
    pub end_flags: Event, // 0x304 (772) - 4 bytes
    pub my_draw_z: i32, // 0x308 (776) - 4 bytes
    pub sub_draw_z: i32, // 0x30C (780) - 4 bytes
    pub fx: f32, // 0x310 (784) - 4 bytes
    pub fy: f32, // 0x314 (788) - 4 bytes
    pub my_pro: MyPro // 0x318 (792) - 28 bytes
}

impl std::fmt::Display for TaskData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TaskData Object
    init: {:p},
    ffunc: {:p},
    sfunc: {:p},
    rfunc: {:p},
    dfunc: {:p},
    finalfunc: {:p},
    hfunc: {:p},
    sta: {},
    sbuff: {:?},
    addr: {:?},
    buff: {:?},
    fbuff: {:?},
    pos: {},
    pos_bk: {},
    base: {},
    hp: {},
    image: {},
    anime: {},
    ptrn: {},
    draw_f: {},
    undefined1: {},
    undefined2: {},
    sort_z: {},
    st_flag_num: {},
    undefined3: {},
    undefined4: {},
    undefined5: {},
    st_flags: {:p},
    stop_flag: {},
    resetflag: {},
    sysfunc: {:p},
    farstfunc: {:p},
    backfunc: {:p},
    hit_si: {},
    system_buff: {:?},
    event_addr: {},
    hit_data: {},
    next: {},
    back: {},
    my_layer: {},
    undefined6: {},
    undefined7: {},
    id: {},
    nexttask: {:p},
    backtask: {:p},
    field_pri: {},
    room_pri: {},
    view_pri: {},
    field_no: {},
    room_no: {},
    view_no: {},
    undefined8: {},
    undefined9: {},
    cash_no: {},
    end_flag_num: {},
    undefined10: {},
    undefined11: {},
    undefined12: {},
    end_flags: {:p},
    my_draw_z: {},
    sub_draw_z: {},
    fx: {},
    fy: {},
    my_pro: {}",
            self.init,
            self.ffunc,
            self.sfunc,
            self.rfunc,
            self.dfunc,
            self.finalfunc,
            self.hfunc,
            self.sta,
            self.sbuff,
            self.addr,
            self.buff,
            self.fbuff,
            self.pos,
            self.pos_bk,
            self.base,
            self.hp,
            self.image,
            self.anime,
            self.ptrn,
            self.draw_f,
            self.undefined1,
            self.undefined2,
            self.sort_z,
            self.st_flag_num,
            self.undefined3,
            self.undefined4,
            self.undefined5,
            self.st_flags,
            self.stop_flag,
            self.resetflag,
            self.sysfunc,
            self.farstfunc,
            self.backfunc,
            self.hit_si,
            self.system_buff,
            self.event_addr,
            self.hit_data,
            self.next,
            self.back,
            self.my_layer,
            self.undefined6,
            self.undefined7,
            self.id,
            self.nexttask,
            self.backtask,
            self.field_pri,
            self.room_pri,
            self.view_pri,
            self.field_no,
            self.room_no,
            self.view_no,
            self.undefined8,
            self.undefined9,
            self.cash_no,
            self.end_flag_num,
            self.undefined10,
            self.undefined11,
            self.undefined12,
            self.end_flags,
            self.my_draw_z,
            self.sub_draw_z,
            self.fx,
            self.fy,
            self.my_pro)
    }
}