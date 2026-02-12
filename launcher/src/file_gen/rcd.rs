use binrw::{BinRead, BinWrite};
use binrw::helpers::args_iter;
use modular_bitfield::prelude::*;
use std::io::Cursor;

use crate::consts::SOURCE_RCD_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_gen::lm_consts::{ZONES};
use crate::file_utils;

#[derive(Debug, BinRead, BinWrite)]
#[br(big)]
pub struct LaMulanaRcd {
    id: u16,
    #[br(parse_with = args_iter(ZONES.to_vec()))]
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

pub struct Rcd {
    rcd_file: LaMulanaRcd
}

impl Rcd {
    pub fn new() -> Result<Self, FileGenerationError> {
        let raw_file = file_utils::read_file(&SOURCE_RCD_PATH).map_err(|_| FileGenerationError::RcdFileReadFailure)?;
        let mut reader = Cursor::new(raw_file);
        let rcd_file = LaMulanaRcd::read_be(&mut reader).map_err(|_| FileGenerationError::RcdFileParseFailure)?;
        Ok(Rcd { rcd_file })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        self.rcd_file.write_be(&mut writer).map_err(|_| FileGenerationError::RcdFileWriteFailure)?;
        Ok(writer.into_inner())
    }
}

