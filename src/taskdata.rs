type EventWithBool = *mut usize;
type Event = *mut *const usize;
type Void = *mut *const usize;

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
pub struct Image {
    pub width: f32, // +4 = 4
    pub height: f32, // +4 = 8
    pub vcol: [VectorColor;4], // +[4*4] = 24
    pub vc: u32, // +4 = 28
    pub pc: u32, // +4 = 32
    pub clip: Clip, // +16 = 48
    pub tex_addr: u32, // +4 = 52
    pub base_obj: [u32;6], // +[4*6] = 76
    pub text_f: u32, // +4 = 80
    pub rInvWidth: f32, // +4 = 84
    pub rInvHeight: f32 // +4 = 88
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
    pub no: i16, // +2 = 2
    pub undefined1: u8, // +1 = 3
    pub undefined2: u8, // +1 = 4
    pub uid: u32, // +4 = 8
    pub live: u8, // +1 = 9
    pub undefined3: u8, // +1 = 10
    pub undefined4: u8, // +1 = 11
    pub undefined5: u8 // +1 = 12
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
    pub init: Event, // +4 = 4
    pub ffunc: EventWithBool, // +4 = 8
    pub sfunc: Event, // +4 = 12
    pub rfunc: EventWithBool, // +4 = 16
    pub dfunc: Event, // +4 = 20
    pub finalfunc: Event, // +4 = 24
    pub hfunc: Event, // +4 = 28
    pub sta: u32, // +4 = 32
    pub sbuff: [i32;32], // +[4*32](128) = 160
    pub addr: [*mut *const usize;16], // +[4*16](64) = 224
    pub buff: [i32;32], // +[4*32](128) = 352
    pub fbuff: [f32;32], // +[4*32](128) = 480
    pub pos: Point3d, // +12 = 492
    pub pos_bk: Point3d, // +12 = 504
    pub base: Point3d, // +12 = 516
    pub hp: i32, // +4 = 520
    pub image: Image, // +88 = 608
    pub anime: u16, // +2 = 610
    pub ptrn: u16, // +2 = 612 <- This should be 612
    pub draw_f: u16, // +2 = 614
    pub undefined1: u8, // +1 = 615
    pub undefined2: u8, // +1 = 616
    pub sort_z: i32, // +4 = 620
    pub st_flag_num: u8, // +1 = 621
    pub undefined3: u8, // +1 = 622
    pub undefined4: u8, // +1 = 623
    pub undefined5: u8, // +1 = 624
    pub st_flags: Void, // +4 = 628
    pub stop_flag: u32, // +4 = 632
    pub resetflag: u32, // +4 = 636
    pub sysfunc: Event, // +4 = 640
    pub farstfunc: EventWithBool, // +4 = 644
    pub backfunc: EventWithBool, // +4 = 648
    pub hit_si: f32, // +4 = 652
    pub system_buff: [i32;17], // +[4*17](68) = 720
    pub event_addr: Void, // +4 = 724
    pub hit_data: i16, // +2 = 726
    pub next: i16, // +2 = 728
    pub back: i16, // +2 = 730
    pub my_layer: i16, // +2 = 732
    pub undefined6: u8, // +1 = 733
    pub undefined7: u8, // +1 = 734
    pub id: TaskId, // +12 = 746
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

impl std::fmt::Display for TaskData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let addr = self.addr.map(|address| format!("{:p}", address));
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
    event_addr: {:p},
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
            addr,
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