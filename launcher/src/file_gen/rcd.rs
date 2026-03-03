use binrw::{BinRead, BinWrite, binrw};
use binrw::helpers::args_iter;
use log::debug;
use modular_bitfield::prelude::*;
use std::io::Cursor;

use crate::archipelago::api::{ItemData, Location};
use crate::consts::SOURCE_RCD_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_gen::lm_consts::{GLOBAL_FLAGS, ITEM_CODES, RCD_OBJECT_PARAMS, RCD_OBJECTS, WRITE_OPERATIONS, ZONES};
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
    id: i16,
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
    id: i16,
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

impl Rcd {
    pub fn new(starting_inventory: Vec<String>, cursed_chests: Vec<String>) -> Result<Self, FileGenerationError> {
        let raw_file = file_utils::read_file(&SOURCE_RCD_PATH).map_err(|_| FileGenerationError::RcdFileReadFailure)?;
        let mut reader = Cursor::new(raw_file);
        let rcd_file = LaMulanaRcd::read_be(&mut reader).map_err(|_| FileGenerationError::RcdFileParseFailure)?;
        Ok(Rcd { rcd_file, starting_inventory, cursed_chests })
    }

    pub fn place_item(&mut self, location: &Location, item: ItemData, item_id: i16, new_item_flag: i16) -> Result<(), FileGenerationError> {
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
        // Endless Corridor Keysword Exists Twice, Once as Regular and Once as Empowered
        if old_item_id == ITEM_CODES["Key Sword"] { old_ids.push(7) };

        let old_item_flag = location.obtain_flag.ok_or_else(|| {
            debug!("Item Flag is missing for Rcd Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

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
            let item_screen = &mut self.rcd_file.zones[zone].rooms[room].screens[screen];

            if item_type == RCD_OBJECTS["scan"] {
                for screen_object in item_screen.objects_without_position.iter_mut() {
                    if screen_object.id == item_type {
                        let target_item_id = screen_object.parameters[item_params.param_index];
                        if old_ids.contains(&target_item_id) {
                            Self::update_operations(&mut screen_object.test_operations, old_item_flag, new_item_flag, None, None, None, None);
                            Self::update_operations(&mut screen_object.write_operations, old_item_flag, new_item_flag, None, None, None, Some(2));

                            screen_object.parameters[item_params.param_index] = item_id;
                        }
                    }
                }
            } else {
                for screen_object in item_screen.objects_with_position.iter_mut() {
                    // The item we're randomizing
                    if screen_object.id == item_type {
                        let target_item_id = screen_object.parameters[item_params.param_index] - item_params.item_mod;
                        if old_ids.contains(&target_item_id) {
                            if item_type == RCD_OBJECTS["chest"] {
                                if self.cursed_chests.contains(&location.name) {
                                    screen_object.parameters[3] = 1;
                                    screen_object.parameters[4] = 1;
                                    screen_object.parameters[5] = 50;
                                } else {
                                    screen_object.parameters[3] = 0;
                                }
                            }

                            Self::update_operations(&mut screen_object.test_operations, old_item_flag, new_item_flag, None, None, None, None);

                            let write_op_value = if item_type == RCD_OBJECTS["naked_item"] || item_type == RCD_OBJECTS["instant_item"] {
                                Some(2)
                            } else {
                                None
                            };
                            Self::update_operations(&mut screen_object.write_operations, old_item_flag, new_item_flag, None, None, None, write_op_value);

                            screen_object.parameters[item_params.param_index] = item_id + item_params.item_mod;

                            // Additional customization is necessary for the Surface Map location
                            if old_item_flag == GLOBAL_FLAGS["surface_map"] {
                                screen_object.test_operations[0].id = GLOBAL_FLAGS["replacement_surface_map_scan"];
                                screen_object.write_operations.push(Operation {
                                    id: GLOBAL_FLAGS["replacement_surface_map_scan"],
                                    operation: WRITE_OPERATIONS["add"],
                                    op_value: 1
                                });
                            }
                        }
                    }

                    // Same Screen Object Customizations

                    // Destructible Cover customization
                    if screen_object.id == RCD_OBJECTS["hitbox_generator"] || screen_object.id == RCD_OBJECTS["room_spawner"] {
                        Self::update_operations(&mut screen_object.test_operations, old_item_flag, new_item_flag, None, None, None, None);
                        Self::update_operations(&mut screen_object.write_operations, old_item_flag, new_item_flag, None, None, None, None);
                    }

                    // Surface Map customization
                    if old_item_flag == GLOBAL_FLAGS["surface_map"] {
                        if screen_object.id == RCD_OBJECTS["scannable"] {
                            if screen_object.test_operations.iter().any(|op| { op.id == old_item_flag }) {
                                screen_object.test_operations[0].id = GLOBAL_FLAGS["replacement_surface_map_scan"];
                                screen_object.write_operations[0].id = GLOBAL_FLAGS["replacement_surface_map_scan"];
                            }
                        }
                    }

                    // Shrine of the Mother Map Crusher customization
                    if old_item_flag == GLOBAL_FLAGS["shrine_map"] {
                        if screen_object.id == RCD_OBJECTS["crusher"] {
                            Self::update_operations(&mut screen_object.write_operations, old_item_flag, new_item_flag, None, None, None, Some(2));
                        }
                    }

                    // Mausoleum Ankh Jewel Trap customization
                    if old_item_flag == GLOBAL_FLAGS["ankh_jewel_mausoleum"] {
                        if screen_object.id == RCD_OBJECTS["moving_texture"] {
                            Self::update_operations(&mut screen_object.write_operations, old_item_flag, new_item_flag, None, None, None, Some(2));
                        }
                    }

                    // Yagostr Dais customization
                    if old_item_flag == GLOBAL_FLAGS["yagostr_found"] {
                        if screen_object.id == RCD_OBJECTS["trigger_dais"] {
                            Self::update_operations(&mut screen_object.test_operations, old_item_flag, new_item_flag, None, None, None, None);
                        }
                    }
                }
            }
        }

        // Separate Screen Object Customizations

        // Vimana customization
        if old_item_flag == GLOBAL_FLAGS["plane_found"] {
            let vimana_screen = &mut self.rcd_file.zones[13].rooms[6].screens[1];
            for vimana_object in vimana_screen.objects_with_position.iter_mut() {
                if vimana_object.id == RCD_OBJECTS["vimana"] {
                    Self::update_operations(&mut vimana_object.test_operations, old_item_flag, new_item_flag, None, None, None, None);
                }
            }
        }

        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        self.rcd_file.write_be(&mut writer).map_err(|_| FileGenerationError::RcdFileWriteFailure)?;
        Ok(writer.into_inner())
    }

    fn update_operations(operations: &mut Vec<Operation>, old_flag: i16, new_flag: i16, old_operation: Option<i8>, new_operation: Option<i8>, old_op_value: Option<i8>, new_op_value: Option<i8>) {
        for op in operations.iter_mut() {
            let flag_match = op.id == old_flag;

            let op_match = match old_operation {
                Some(o) => { op.operation == o },
                None => true
            };

            let value_match = match old_op_value {
                Some(v) => { op.op_value == v },
                None => true
            };

            if flag_match && op_match && value_match {
                op.id = new_flag;

                if let Some(o) = new_operation {
                    op.operation = o;
                }

                if let Some(v) = new_op_value {
                    op.op_value = v;
                }
            }
        }
    }
}

