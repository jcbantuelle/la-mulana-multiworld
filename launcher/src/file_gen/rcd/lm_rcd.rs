use binrw::{BinRead, BinWrite, binrw};
use binrw::helpers::args_iter;
use modular_bitfield::prelude::*;
use std::io::Cursor;

use crate::file_gen::lm_consts::ZONES;
use crate::file_gen::generator::FileGenerationError;

#[derive(Debug, BinRead, BinWrite)]
#[br(big)]
pub struct LaMulanaRcd {
    pub id: u16,
    #[br(parse_with = args_iter(ZONES.to_vec()))]
    pub zones: Vec<Zone>
}

impl LaMulanaRcd {
    pub fn load_file(raw_file: Vec<u8>) -> Result<LaMulanaRcd, FileGenerationError> {
        let mut reader = Cursor::new(raw_file);
        LaMulanaRcd::read_be(&mut reader).map_err(|_| FileGenerationError::RcdFileParseFailure)
    }

    pub fn write_file(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        self.write_be(&mut writer).map_err(|_| FileGenerationError::RcdFileWriteFailure)?;
        Ok(writer.into_inner())
    }
}

#[binrw]
#[derive(Debug)]
#[br(big, import_raw(room_sizes: Vec<i32>))]
pub struct Zone {
    pub zone_name_length: u8,
    #[bw(calc = objects.len() as u16)]
    pub objects_length: u16,
    #[br(count = zone_name_length)]
    pub zone_name: Vec<u8>,
    #[br(count = objects_length)]
    pub objects: Vec<ObjectWithoutPosition>,
    #[br(parse_with = args_iter(room_sizes))]
    pub rooms: Vec<Room>
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ObjectHeader {
    pub write_operations_length: B4,
    pub test_operations_length: B4
}

#[binrw]
#[derive(Debug)]
pub struct ObjectWithoutPosition {
    pub id: i16,
    #[br(map = ObjectHeader::from_bytes)]
    #[bw(map = |obj| {
        let mut o = obj.clone();
        o.set_test_operations_length(test_operations.len() as u8);
        o.set_write_operations_length(write_operations.len() as u8);
        o.into_bytes()
    })]
    pub header: ObjectHeader,
    #[bw(calc = parameters.len() as u8)]
    pub parameters_length: u8,
    #[br(count = header.test_operations_length())]
    pub test_operations: Vec<Operation>,
    #[br(count = header.write_operations_length())]
    pub write_operations: Vec<Operation>,
    #[br(count = parameters_length)]
    pub parameters: Vec<i16>
}

#[binrw]
#[derive(Debug)]
pub struct ObjectWithPosition{
    pub id: i16,
    #[br(map = ObjectHeader::from_bytes)]
    #[bw(map = |obj| {
        let mut o = obj.clone();
        o.set_test_operations_length(test_operations.len() as u8);
        o.set_write_operations_length(write_operations.len() as u8);
        o.into_bytes()
    })]
    pub header: ObjectHeader,
    #[bw(calc = parameters.len() as u8)]
    pub parameters_length: u8,
    pub x_pos: i16,
    pub y_pos: i16,
    #[br(count = header.test_operations_length())]
    pub test_operations: Vec<Operation>,
    #[br(count = header.write_operations_length())]
    pub write_operations: Vec<Operation>,
    #[br(count = parameters_length)]
    pub parameters: Vec<i16>
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Exit{
    pub id: i8,
    pub room_id: i8,
    pub screen_id: i8,
}

#[derive(Clone, Debug, BinRead, BinWrite)]
pub struct Operation {
    pub id: i16,
    pub op_value: i8,
    pub operation: i8,
}

#[binrw]
#[derive(Debug)]
#[br(big, import_raw(screen_count: i32))]
pub struct Screen {
    pub screen_name_length: i8,
    #[bw(calc = (objects_with_position.len() + objects_without_position.len()) as i16)]
    pub objects_length: i16,
    #[bw(calc = objects_without_position.len() as i8)]
    pub objects_without_position_length: i8,
    #[br(count = objects_without_position_length)]
    pub objects_without_position: Vec<ObjectWithoutPosition>,
    #[br(count = objects_length - objects_without_position_length as i16)]
    pub objects_with_position: Vec<ObjectWithPosition>,
    #[br(count = screen_name_length)]
    pub screen_name: Vec<i8>,
    #[br(count = 4)]
    pub exits: Vec<Exit>,
}

#[binrw]
#[derive(Debug)]
#[br(big, import_raw(screen_count: i32))]
pub struct Room {
    #[bw(calc = objects.len() as i16)]
    pub objects_length: i16,
    #[br(count = objects_length)]
    pub objects: Vec<ObjectWithoutPosition>,
    #[br(count = screen_count)]
    pub screens: Vec<Screen>,
}
