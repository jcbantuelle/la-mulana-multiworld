use binrw::{BinRead, BinWrite, binrw};
use binrw::helpers::args_iter;
use log::debug;
use modular_bitfield::prelude::*;
use std::io::Cursor;

use crate::archipelago::api::{ItemData, Location};
use crate::consts::SOURCE_RCD_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_gen::lm_consts::{GLOBAL_FLAGS, ITEM_CODES, RCD_OBJECT_PARAMS, RCD_OBJECTS, ZONES};
use crate::file_utils;

#[derive(Debug, BinRead, BinWrite)]
#[br(big)]
pub struct LaMulanaRcd {
    id: u16,
    #[br(parse_with = args_iter(ZONES.to_vec()))]
    zones: Vec<Zone>
}

#[binrw]
#[derive(Debug)]
#[br(big, import_raw(room_sizes: Vec<i32>))]
pub struct Zone {
    zone_name_length: u8,
    #[bw(calc = objects.len() as u16)]
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

#[binrw]
#[derive(Debug)]
pub struct ObjectWithoutPosition {
    id: u16,
    #[br(map = ObjectHeader::from_bytes)]
    #[bw(map = |obj| {
        let mut o = obj.clone();
        o.set_test_operations_length(test_operations.len() as u8);
        o.set_write_operations_length(write_operations.len() as u8);
        o.into_bytes()
    })]
    header: ObjectHeader,
    #[bw(calc = parameters.len() as u8)]
    parameters_length: u8,
    #[br(count = header.test_operations_length())]
    test_operations: Vec<Operation>,
    #[br(count = header.write_operations_length())]
    write_operations: Vec<Operation>,
    #[br(count = parameters_length)]
    parameters: Vec<i16>
}

#[binrw]
#[derive(Debug)]
pub struct ObjectWithPosition{
    id: u16,
    #[br(map = ObjectHeader::from_bytes)]
    #[bw(map = |obj| {
        let mut o = obj.clone();
        o.set_test_operations_length(test_operations.len() as u8);
        o.set_write_operations_length(write_operations.len() as u8);
        o.into_bytes()
    })]
    header: ObjectHeader,
    #[bw(calc = parameters.len() as u8)]
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

#[binrw]
#[derive(Debug)]
#[br(big, import_raw(screen_count: i32))]
pub struct Screen {
    screen_name_length: i8,
    #[bw(calc = (objects_with_position.len() + objects_without_position.len()) as i16)]
    objects_length: i16,
    #[bw(calc = objects_without_position.len() as i8)]
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

#[binrw]
#[derive(Debug)]
#[br(big, import_raw(screen_count: i32))]
pub struct Room {
    #[bw(calc = objects.len() as i16)]
    objects_length: i16,
    #[br(count = objects_length)]
    objects: Vec<ObjectWithoutPosition>,
    #[br(count = screen_count)]
    screens: Vec<Screen>,
}

pub struct Rcd {
    rcd_file: LaMulanaRcd,
    starting_inventory: Vec<String>,
    cursed_chests: Vec<String>
}

pub trait Object {
    fn id(&self) -> u16;
    fn test_operations(&self) -> Vec<Operation>;
    fn write_operations(&self) -> Vec<Operation>;
    fn parameters(&self) -> Vec<i16>;
}

macro_rules! impl_object {
    ($($t:ty),+) => {
        $(
            impl Object for $t {
                fn id(&self) -> u16 {
                    self.id
                }

                fn test_operations(&self) -> Vec<Operation> {
                    self.test_operations
                }

                fn write_operations(&self) -> Vec<Operation> {
                    self.write_operations
                }

                fn parameters(&self) -> Vec<i16> {
                    self.parameters
                }
            }
        )+
    };
}
impl_object!(ObjectWithPosition, ObjectWithoutPosition);

impl Rcd {
    pub fn new(starting_inventory: Vec<String>, cursed_chests: Vec<String>) -> Result<Self, FileGenerationError> {
        let raw_file = file_utils::read_file(&SOURCE_RCD_PATH).map_err(|_| FileGenerationError::RcdFileReadFailure)?;
        let mut reader = Cursor::new(raw_file);
        let rcd_file = LaMulanaRcd::read_be(&mut reader).map_err(|_| FileGenerationError::RcdFileParseFailure)?;
        Ok(Rcd { rcd_file, starting_inventory, cursed_chests })
    }

    pub fn place_item(&mut self, location: &Location, item: ItemData, item_id: i16) -> Result<(), FileGenerationError> {
        let item_type = location.object_type.ok_or_else(|| {
            debug!("Object Type Missing for Rcd Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

        let item_params = RCD_OBJECT_PARAMS.get(&item_type).ok_or_else(|| {
            debug!("Invalid Object Type for Rcd Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?.clone();

        let old_item_id = location.item_id.ok_or_else(|| {
            debug!("Item ID is missing for Rcd Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;
        let mut old_ids = vec![old_item_id];

        let zones = location.zones.clone().ok_or_else(|| {
            debug!("Zones are missing for Rcd Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

        let room = location.room.ok_or_else(|| {
            debug!("Room is missing for Rcd Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

        let screen = location.screen.ok_or_else(|| {
            debug!("Screen is missing for Rcd Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

        for zone in zones {
            let screen = &mut self.rcd_file.zones[zone].rooms[room].screens[screen];

            // Endless Corridor Twin Statue Chest Exists Twice
            let mut iterations = item_params.iterations;
            if old_item_id == ITEM_CODES["Twin Statue"] { iterations = 2}

            // Endless Corridor Keysword Exists Twice, Once as Regular and Once as Empowered
            if old_item_id == ITEM_CODES["Key Sword"] { old_ids.push(7) };
            for old_id in &old_ids {
                for _ in 0..iterations {
                    let scan_flag = GLOBAL_FLAGS["scan"];
                    let screen_objects: Vec<Box<dyn Object>> = match item_type {
                        scan_flag => screen.objects_without_position.iter().map(|obj| { Box::new(*obj)}).collect::<Vec<Box<dyn Object>>>(),
                        _ => screen.objects_with_position.iter().map(|obj| { Box::new(*obj)}).collect::<Vec<Box<dyn Object>>>()
                    };
        //             if item_type == GLOBAL_FLAGS["scan"] {
        //                 let screen_object = screen.objects_without_position.iter_mut().find(|object| {
        //                     object.id == item_type && object.parameters[item_params.param_index] == old_item_id && object.parameters.len() < item_params.param_length
        //                 }).ok_or_else(|| {
        //                     debug!("Object Type {} is missing for Item {} in Rcd Location: {:?}", item_type, old_item_id, location);
        //                     FileGenerationError::MalformedRcdFile
        //                 })?;
        //             } else {
        //                 let screen_object = screen.objects_with_position.iter_mut().find(|object| {
        //                     object.id == item_type && object.parameters[item_params.param_index] == old_item_id && object.parameters.len() < item_params.param_length
        //                 }).ok_or_else(|| {
        //                     debug!("Object Type {} is missing for Item {} in Rcd Location: {:?}", item_type, old_item_id, location);
        //                     FileGenerationError::MalformedRcdFile
        //                 })?;

        //                 if item_type == RCD_OBJECTS["chest"] {
        //                     if self.cursed_chests.contains(&location.name) {
        //                         screen_object.parameters[3] = 1;
        //                         screen_object.parameters[4] = 1;
        //                         screen_object.parameters[5] = 50;
        //                     } else {
        //                         screen_object.parameters[3] = 0;
        //                     }
        //                 }

        //                 for test_op in screen_object.test_operations.iter_mut() {
        //                     if test_op.id == item.obtain_flag
        //                 }
        // //         if test_op.flag == original_obtain_flag:
        // //             test_op.flag = new_obtain_flag
        // //     for write_op in item_location.write_operations:
        // //         if write_op.flag == original_obtain_flag:
        // //             write_op.flag = new_obtain_flag
        // //             if object_type in (RCD_OBJECTS["naked_item"], RCD_OBJECTS["instant_item"], RCD_OBJECTS["scan"]):
        // //                 write_op.op_value = obtain_value
        //             }
                }
            }
        }

        // def __place_item(self, objects, object_type, param_index, param_len, location, location_id, item_id, original_obtain_flag, new_obtain_flag, obtain_value, item_mod, iterations, item):
    
        //     for test_op in item_location.test_operations:
        //         if test_op.flag == original_obtain_flag:
        //             test_op.flag = new_obtain_flag
        //     for write_op in item_location.write_operations:
        //         if write_op.flag == original_obtain_flag:
        //             write_op.flag = new_obtain_flag
        //             if object_type in (RCD_OBJECTS["naked_item"], RCD_OBJECTS["instant_item"], RCD_OBJECTS["scan"]):
        //                 write_op.op_value = obtain_value

        //     # Destructible Cover customization
        //     for operation in ["test", "write"]:
        //         self.__update_operation(operation, objects, [RCD_OBJECTS["hitbox_generator"], RCD_OBJECTS["room_spawner"]], original_obtain_flag, new_obtain_flag)

        //     # Surface Map customization
        //     if original_obtain_flag == GLOBAL_FLAGS["surface_map"]:
        //         self.__fix_surface_map_scan(objects, item_location, original_obtain_flag)
            
        //     # Shrine of the Mother Map Crusher customization
        //     if original_obtain_flag == GLOBAL_FLAGS["shrine_map"]:
        //         self.__update_operation("write", objects, [RCD_OBJECTS["crusher"]], original_obtain_flag, new_obtain_flag, new_op_value=obtain_value)

        //     # Mausoleum Ankh Jewel Trap customization
        //     if original_obtain_flag == GLOBAL_FLAGS["ankh_jewel_mausoleum"]:
        //         self.__update_operation("write", objects, [RCD_OBJECTS["moving_texture"]], original_obtain_flag, new_obtain_flag, new_op_value=obtain_value)

        //     # Yagostr Dais customization
        //     if original_obtain_flag == GLOBAL_FLAGS["yagostr_found"]:
        //         self.__update_operation("test", objects, [RCD_OBJECTS["trigger_dais"]], original_obtain_flag, new_obtain_flag)

        //     # Vimana customization
        //     if original_obtain_flag == GLOBAL_FLAGS["plane_found"]:
        //         vimana_objects = self.file_contents.zones[13].rooms[6].screens[1].objects_with_position
        //         self.__update_operation("test", vimana_objects, [RCD_OBJECTS["vimana"]], original_obtain_flag, new_obtain_flag)

        //     item_location.parameters[param_index] = item_id+item_mod
        //     item_location.parameters.append(1)
        //     item_location.parameters_length += 1
        //     self.file_size += 2

        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        self.rcd_file.write_be(&mut writer).map_err(|_| FileGenerationError::RcdFileWriteFailure)?;
        Ok(writer.into_inner())
    }
}

