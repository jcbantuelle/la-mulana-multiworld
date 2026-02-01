use std::fs::File;
use std::io::{BufReader, Result as IOResult};
use std::sync::LazyLock;
use binrw::{BinRead, BinWrite};
use binrw::helpers::args_iter;
use modular_bitfield::prelude::*;

static ZONE_SIZES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| {
    vec![
        vec![2,2,2,2,3,1,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![3,2,2,1,3,3,2,2,2,2,4,2,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,1,1,3,2,3,3,1,0,0,0,0,0,0,0,0,0,0],
        vec![2,3,2,1,6,1,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![3,1,2,5,1,2,2,2,2,0,1,1,1,1,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0],
        vec![2,2,3,3,1,2,1,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,2,2,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,3,3,2,2,2,2,2,2,2,3,3,2,2,3,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,4,4,4,4,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,2,2,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,3,1,2,2,1,3,2,2,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![3,2,2,1,4,2,1,2,1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,1,4,2,2,1,1,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,3,2,2,2,4,3,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0],
        vec![3,2,2,2,2,2,2,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,3,0,0,0,0,0,0,0,0],
        vec![2,2,2,1,2,2,1,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,2,2,2,2,2,2,0,0,0,0,0],
        vec![2,3,2,2,2],
        vec![2,3,2,2,2],
        vec![2,0],
        vec![3,2,2,1,3,3,2,2,2,2,4,2,0,0,0,0,0,0,0,0,0,0,0],
        vec![1,2,2,1,2,2,2,1,2,2,2,1,2,1,2,2,1,1,2,1,1,1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,1,2,0,0],
        vec![5,5,5,5,0]
    ]
});

#[derive(Debug, BinRead, BinWrite)]
#[br(big)]
pub struct LaMulanaRcd {
    id: u16,
    #[br(parse_with = args_iter(ZONE_SIZES.to_vec()))]
    zones: Vec<Zone>
}

#[derive(Debug, BinRead, BinWrite)]
#[br(big, import_raw(room_sizes: Vec<i32>))]
pub struct Zone {
    zone_name_length: u8,
    objects_length: u16,
    #[br(count = zone_name_length)]
    zone_name: Vec<u8>,
    #[br(count = objects_length)]
    objects: Vec<ObjectWithoutPosition>,
    #[br(parse_with = args_iter(room_sizes))]
    rooms: Vec<Room>
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ObjectHeader {
    test_operations_length: B4,
    write_operations_length: B4
}

#[derive(Debug, BinRead, BinWrite)]
pub struct ObjectWithoutPosition {
    id: u16,
    #[br(map = ObjectHeader::from_bytes)]
    #[bw(map = |obj| obj.into_bytes())]
    header: ObjectHeader,
    parameters_length: u8,
    #[br(count = header.test_operations_length())]
    test_operations: Vec<Operation>,
    #[br(count = header.write_operations_length())]
    write_operations: Vec<Operation>,
    #[br(count = parameters_length)]
    parameters: Vec<i16>
}

#[derive(Debug, BinRead, BinWrite)]
pub struct ObjectWithPosition{
    id: u16,
    #[br(map = ObjectHeader::from_bytes)]
    #[bw(map = |obj| obj.into_bytes())]
    header: ObjectHeader,
    parameters_length: u8,
    x_pos: i16,
    y_pos: i16,
    #[br(count = header.test_operations_length())]
    test_operations: Vec<Operation>,
    #[br(count = header.write_operations_length())]
    write_operations: Vec<Operation>,
    #[br(count = parameters_length)]
    parameters: Vec<i16>
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Exit{
    id: i8,
    room_id: i8,
    screen_id: i8,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Operation {
    id: i16,
    op_value: i8,
    operation: i8,
}

#[derive(Debug, BinRead, BinWrite)]
#[br(big, import_raw(screen_count: i32))]
pub struct Screen {
    screen_name_length: i8,
    objects_length: i16,
    objects_without_position_length: i8,
    #[br(count = objects_without_position_length)]
    objects_without_position: Vec<ObjectWithoutPosition>,
    #[br(count = objects_length - objects_without_position_length as i16)]
    objects_with_position: Vec<ObjectWithPosition>,
    #[br(count = screen_name_length)]
    screen_name: Vec<i8>,
    #[br(count = 4)]
    exits: Vec<Exit>,
}

#[derive(Debug, BinRead, BinWrite)]
#[br(big, import_raw(screen_count: i32))]
pub struct Room {
    objects_length: i16,
    #[br(count = objects_length)]
    objects: Vec<ObjectWithoutPosition>,
    #[br(count = screen_count)]
    screens: Vec<Screen>,
}

pub fn generate(slot_data: SlotData) -> IOResult<()> {
    // Open the binary file
    let file = File::open("script.rcd")?;
    let mut reader = BufReader::new(file);

    return LaMulanaRcd::read(&mut reader);
}

