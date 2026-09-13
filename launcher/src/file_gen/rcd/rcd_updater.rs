use archipelago_api::api::{ItemData, Location, SlotData};
use log::debug;
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;
use std::collections::HashMap;

use crate::consts::SOURCE_RCD_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_gen::lm_consts::*;
use crate::file_gen::rcd::lm_rcd::*;
use crate::file_utils;

pub struct RcdUpdater {
    rcd_file: LaMulanaRcd,
    cursed_chests: Vec<String>
}

impl RcdUpdater {
    pub fn new(cursed_chests: Vec<String>) -> Result<Self, FileGenerationError> {
        let raw_file = file_utils::read_file(&SOURCE_RCD_PATH).map_err(|_| FileGenerationError::RcdFileReadFailure)?;
        let rcd_file = LaMulanaRcd::load_file(raw_file)?;
        Ok(RcdUpdater { rcd_file, cursed_chests })
    }

    pub fn place_item(&mut self, location: &Location, original_item_id: i16, new_item_flag: i16) -> Result<(), FileGenerationError> {
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

        let old_item_flag = location.original_obtain_flag.or_else(|| {
            location.obtain_flag
        }).ok_or_else(|| {
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

        // Convert filler chest items to coin chests
        let item_id = if item_type == RCD_OBJECTS["chest"] && original_item_id == ITEM_CODES["Shell Horn"] { -10 } else { original_item_id };

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
                                let address = location.address.unwrap_or(0);

                                // Screens with multiple chests need an additional position check
                                let skip = DOUBLE_CHEST_ADDRESSES.get(&address).map_or(false, |x_pos| {
                                    screen_object.x_pos != *x_pos
                                });

                                if skip {
                                    continue;
                                }
                                // Coin Chest
                                if item_id == -10 {
                                    let coin_chest_quantities = [(200, 1), (100, 2), (50, 1), (30, 4), (10, 6), (1, 2)];
                                    let distribution = WeightedIndex::new(coin_chest_quantities.iter().map(|quantity| quantity.1)).unwrap();
                                    let mut rng = rand::rng();
                                    let quantity = coin_chest_quantities[distribution.sample(&mut rng)].0;

                                    screen_object.parameters[1] = quantity;
                                    screen_object.parameters[2] = 0;
                                    screen_object.write_operations[0].id = new_item_flag;
                                    screen_object.write_operations[0].op_value = 2;
                                    screen_object.write_operations[0].operation = WRITE_OPERATIONS["assign"];
                                    screen_object.write_operations[2].id = new_item_flag;
                                    screen_object.write_operations[2].operation = WRITE_OPERATIONS["assign"];
                                    screen_object.write_operations[2].op_value = 2;
                                    screen_object.write_operations[3].id = GLOBAL_FLAGS["coin_chests"];
                                    screen_object.write_operations[3].operation = WRITE_OPERATIONS["add"];
                                    screen_object.write_operations[3].op_value = 1;
                                // Item Chest
                                } else {
                                    screen_object.parameters[1] = 1;
                                    screen_object.parameters[2] = 1;
                                    screen_object.write_operations[0].op_value = 2;
                                    screen_object.write_operations[3].id = new_item_flag;
                                    screen_object.write_operations[3].operation = WRITE_OPERATIONS["assign"];
                                    screen_object.write_operations[3].op_value = 2;
                                }
                                // Mark Chest as Cursed
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

                    // Removable Cover customization
                    if screen_object.id == RCD_OBJECTS["hitbox_generator"] || screen_object.id == RCD_OBJECTS["room_spawner"] || screen_object.id == RCD_OBJECTS["trigger_seal"] {
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

                    // Shrine of the Mother Diary Room Pillar
                    if old_item_flag == GLOBAL_FLAGS["diary_found"] {
                        if screen_object.id == RCD_OBJECTS["xelpud_pillar"] {
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

    pub fn give_starting_items(&mut self, starting_inventory: Vec<String>, starting_weapon_id: u64, item_table: HashMap<String, ItemData>) -> Result<(), FileGenerationError> {
        let start_screen = &mut self.rcd_file.zones[1].rooms[2].screens[1];
        let starting_weapon = STARTING_WEAPONS[&starting_weapon_id].to_string();

        let filtered_inventory = starting_inventory.iter().filter(|&item_name| item_name != &starting_weapon);

        for (flag_counter, item_name) in filtered_inventory.enumerate() {
            let item = item_table[item_name].clone();

            let obtain_flag = item.obtain_flag.ok_or_else(|| {
                debug!("Obtain Flag is missing for Item: {:?}", item_name);
                FileGenerationError::MalformedSlotData
            })?;

            let item_giver = ObjectWithPosition {
                id: RCD_OBJECTS["instant_item"],
                header: ObjectHeader::from_bytes([0b00010010]),
                x_pos: 0,
                y_pos: 0,
                test_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["starting_items"],
                        op_value: flag_counter as i8,
                        operation: TEST_OPERATIONS["eq"]
                    }
                ],
                write_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["starting_items"],
                        op_value: 1,
                        operation: WRITE_OPERATIONS["add"]
                    },
                    Operation {
                        id: obtain_flag,
                        op_value: 2,
                        operation: WRITE_OPERATIONS["assign"]
                    }
                ],
                parameters: vec![item.game_code, 160, 120, 39]
            };

            start_screen.objects_with_position.push(item_giver);
        }

        Ok(())
    }

    pub fn rewrite_four_guardian_shop_conditions(&mut self, four_guardian_item_flag: i16) {
        let nebur_screen = &mut self.rcd_file.zones[1].rooms[2].screens[0];
        for screen_object in nebur_screen.objects_with_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["language_conversation"] {
                for op in screen_object.test_operations.iter_mut() {
                    if op.id == GLOBAL_FLAGS["msx2_found"] {
                        op.id = four_guardian_item_flag;
                    } else if op.id == GLOBAL_FLAGS["xelpud_msx2"] {
                        if op.op_value == 0 {
                            op.id = GLOBAL_FLAGS["guardians_killed"];
                            op.op_value = 3;
                            op.operation = TEST_OPERATIONS["lteq"];
                        } else if op.op_value == 1 {
                            op.id = GLOBAL_FLAGS["guardians_killed"];
                            op.op_value = 4;
                        }
                    }
                }
            }
        }
    }

    pub fn rewrite_mekuri_door(&mut self, mekuri_flag: i16) {
        let mekuri_screen = &mut self.rcd_file.zones[1].rooms[7].screens[0];
        for screen_object in mekuri_screen.objects_with_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["language_conversation"] || screen_object.id == RCD_OBJECTS["texture_draw_animation"] {
                Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["mekuri"], mekuri_flag, None, None, None, None);
            }
        }
    }

    pub fn apply_mods(&mut self, slot_data: SlotData) -> Result<(), FileGenerationError> {
        let options = slot_data.options.clone();

        self.rewrite_diary_events();
        self.rewrite_mulbruk_doors();
        self.rewrite_sun_lights_hitbox();
        self.rewrite_slushfund_conversation_conditions();
        self.rewrite_stray_fairy_events();
        self.rewrite_fishman_alt_shop();
        self.rewrite_boss_ankhs(&options);
        self.rewrite_anubis_seen();

        self.add_dimensional_orb_ladder();
        self.add_true_shrine_doors();
        self.add_moonlight_to_twin_lockout_fix();
        self.add_chain_whip_lockout_fix();
        self.add_flail_whip_lockout_fix();
        self.add_angel_shield_lockout_fix();
        self.add_sun_map_lockout_fix();
        self.add_hardmode_toggle();
        self.add_sacred_orb_timers();
        self.add_new_game_kill_timer();

        self.clean_up_operations();

        if options.get("AutoScanGrailTablets").is_some_and(|option| *option > 0) {
            self.create_grail_autoscans();
        }

        if options.get("AncientLaMulaneseLearned").is_some_and(|option| *option > 0) {
            self.create_ancient_lamulanese_timer();
        }

        if options.get("AlternateMotherAnkh").is_some_and(|option| *option > 0) {
            self.create_alternate_mother_ankh();
        }

        if options.get("RandomizeBacksideDoors").is_some_and(|option| *option > 0) {
            let door_map = slot_data.door_map.clone().ok_or_else(|| {
                debug!("Door Map is missing from Slot Data: {:?}", slot_data);
                FileGenerationError::MalformedSlotData
            })?;
            self.randomize_doors(door_map)?;
        }

        Ok(())
    }

    pub fn write_file(&self) -> Result<Vec<u8>, FileGenerationError> {
        self.rcd_file.write_file()
    }

    fn rewrite_diary_events(&mut self) {
        {
            // Remove Diary conversation door from Xelpud conversations
            let xelpud_screen = &mut self.rcd_file.zones[1].rooms[2].screens[1];

            let _ = xelpud_screen.objects_with_position.extract_if(.., |object| {
                object.id == RCD_OBJECTS["language_conversation"] && object.parameters[4] == 913
            }).collect::<Vec<_>>();

            // Remove Diary Puzzle Timer
            let _ = xelpud_screen.objects_without_position.extract_if(.., |object| {
                object.id == RCD_OBJECTS["flag_timer"] && object.test_operations.len() > 1 && object.test_operations[1].id == GLOBAL_FLAGS["diary_found"]
            }).collect::<Vec<_>>();

            // Add new Talisman Xelpud Timer
            let talisman_flag_timer = ObjectWithoutPosition {
                id: RCD_OBJECTS["flag_timer"],
                header: ObjectHeader::from_bytes([0b00110001]),
                test_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["talisman_found"],
                        op_value: 2,
                        operation: TEST_OPERATIONS["eq"]
                    },
                    Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_talisman_found"],
                        op_value: 0,
                        operation: TEST_OPERATIONS["eq"]
                    },
                    Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_general"],
                        op_value: 1,
                        operation: TEST_OPERATIONS["gteq"]
                    }
                ],
                write_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_talisman_found"],
                        op_value: 1,
                        operation: WRITE_OPERATIONS["assign"]
                    }
                ],
                parameters: vec![0,0]
            };
            xelpud_screen.objects_without_position.push(talisman_flag_timer);

            // Add new Talisman Diary Timer
            let talisman_flag_timer = ObjectWithoutPosition {
                id: RCD_OBJECTS["flag_timer"],
                header: ObjectHeader::from_bytes([0b00110001]),
                test_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["diary_found"],
                        op_value: 2,
                        operation: TEST_OPERATIONS["eq"]
                    },
                    Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_diary_found"],
                        op_value: 0,
                        operation: TEST_OPERATIONS["eq"]
                    },
                    Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_general"],
                        op_value: 1,
                        operation: TEST_OPERATIONS["gteq"]
                    }
                ],
                write_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_diary_found"],
                        op_value: 1,
                        operation: WRITE_OPERATIONS["assign"]
                    }
                ],
                parameters: vec![0,0]
            };
            xelpud_screen.objects_without_position.push(talisman_flag_timer);
        }

        {
            // Update Diary Chest flags
            let diary_screen = &mut self.rcd_file.zones[9].rooms[2].screens[1];
            for screen_object in diary_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["chest"] && screen_object.write_operations.iter().any(|op| { op.id == GLOBAL_FLAGS["diary_chest_puzzle"] }) {
                    Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["shrine_shawn"], GLOBAL_FLAGS["shrine_dragon_bone"], None, None, None, None);
                    screen_object.test_operations.push(Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_talisman_found"],
                        op_value: 2,
                        operation: TEST_OPERATIONS["gteq"]
                    });
                }
            }
        }

        {
            let diary_puzzle_screen = &mut self.rcd_file.zones[9].rooms[2].screens[0];

            // Remove old Diary Puzzle Timer
            _ = diary_puzzle_screen.objects_without_position.extract_if(.., |object| {
                object.id == RCD_OBJECTS["flag_timer"] && object.write_operations.iter().any(|op| op.id == GLOBAL_FLAGS["diary_chest_puzzle"])
            }).collect::<Vec<_>>();

            let diary_puzzle_flag_timer = ObjectWithoutPosition {
                id: RCD_OBJECTS["flag_timer"],
                header: ObjectHeader::from_bytes([0b00100001]),
                test_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["xelpud_conversation_talisman_found"],
                        op_value: 3,
                        operation: TEST_OPERATIONS["gteq"]
                    },
                    Operation {
                        id: GLOBAL_FLAGS["shrine_dragon_bone"],
                        op_value: 1,
                        operation: TEST_OPERATIONS["gteq"]
                    }
                ],
                write_operations: vec![
                    Operation {
                        id: GLOBAL_FLAGS["shrine_diary_chest"],
                        op_value: 2,
                        operation: WRITE_OPERATIONS["assign"]
                    }
                ],
                parameters: vec![0,0]
            };
            diary_puzzle_screen.objects_without_position.push(diary_puzzle_flag_timer);
        }
    }

    fn rewrite_mulbruk_doors(&mut self) {
        let mulbruk_screen = &mut self.rcd_file.zones[3].rooms[3].screens[0];

        _ = mulbruk_screen.objects_with_position.extract_if(.., |object| {
            object.id == RCD_OBJECTS["language_conversation"] && (
                [926, 1014].contains(&object.parameters[4]) ||
                object.test_operations.iter().any(|op| op.id == GLOBAL_FLAGS["score"] && op.op_value == 55 && op.operation == TEST_OPERATIONS["lteq"])
            )
        }).collect::<Vec<_>>();

        for screen_object in mulbruk_screen.objects_with_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["language_conversation"] && screen_object.test_operations.iter().any(|op| { op.id == GLOBAL_FLAGS["swimsuit_found"] }) {
                screen_object.test_operations.push(Operation {
                    id: GLOBAL_FLAGS["mulbruk_father"],
                    op_value: 9,
                    operation: TEST_OPERATIONS["neq"]
                });
            }
        }
    }

    fn rewrite_sun_lights_hitbox(&mut self) {
        let lights_screen = &mut self.rcd_file.zones[3].rooms[0].screens[0];

        for screen_object in lights_screen.objects_with_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["hitbox_generator"] {
                screen_object.x_pos = 20;
            }
        }
    }

    fn rewrite_slushfund_conversation_conditions(&mut self) {
        let slushfund_screen = &mut self.rcd_file.zones[10].rooms[8].screens[0];

        for screen_object in slushfund_screen.objects_with_position.iter_mut()  {
            if screen_object.id == RCD_OBJECTS["language_conversation"] {
                Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["slushfund_conversation"], GLOBAL_FLAGS["replacement_slushfund_conversation"], None, None, None, None);
            }
        }
    }

    fn rewrite_stray_fairy_events(&mut self) {
        // Stray Fairy Screen Modifications
        {
            let stray_fairy_screen = &mut self.rcd_file.zones[10].rooms[0].screens[1];

            for screen_object in stray_fairy_screen.objects_with_position.iter_mut() {
                // Update Fairy Door to Use Custom Cog Puzzle Flag
                if screen_object.id == RCD_OBJECTS["room_spawner"] {
                    Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["cog_puzzle"], GLOBAL_FLAGS["replacement_cog_puzzle"], Some(TEST_OPERATIONS["lteq"]), None, Some(2), None);
                }

                // Update Test and Write Operations for the Chest and Stray Fairy Conversations to use Custom Cog Puzzle Flag
                if screen_object.id == RCD_OBJECTS["chest"] || screen_object.id == RCD_OBJECTS["language_conversation"] {
                    Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["cog_puzzle"], GLOBAL_FLAGS["replacement_cog_puzzle"], None, None, None, None);
                    Self::update_operations(&mut screen_object.write_operations, GLOBAL_FLAGS["cog_puzzle"], GLOBAL_FLAGS["replacement_cog_puzzle"], None, None, None, None);
                }

                if [RCD_OBJECTS["room_spawner"], RCD_OBJECTS["use_item"], RCD_OBJECTS["scannable"], RCD_OBJECTS["texture_draw_animation"]].contains(&screen_object.id) {
                    if screen_object.test_operations.iter().any(|op| op.id == GLOBAL_FLAGS["cog_puzzle"] && op.op_value == 3 && op.operation == TEST_OPERATIONS["eq"]) {
                        screen_object.x_pos -= 3;
                        screen_object.test_operations[0].operation = TEST_OPERATIONS["lteq"];
                    }
                }
            }

            for screen_object in stray_fairy_screen.objects_without_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["flag_timer"] {
                    Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["cog_puzzle"], GLOBAL_FLAGS["replacement_cog_puzzle"], None, None, None, None);
                    Self::update_operations(&mut screen_object.write_operations, GLOBAL_FLAGS["cog_puzzle"], GLOBAL_FLAGS["replacement_cog_puzzle"], None, None, None, None);
                }
            }
        }

        // Cog Puzzle Tablets Screen Modifications
        {
            let cog_tablets_screen = &mut self.rcd_file.zones[10].rooms[1].screens[0];

            for screen_object in cog_tablets_screen.objects_without_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["flag_timer"] {
                    Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["cog_puzzle"], GLOBAL_FLAGS["replacement_cog_puzzle"], None, None, None, None);
                }
            }

            for screen_object in cog_tablets_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["texture_draw_animation"] || screen_object.id == RCD_OBJECTS["scannable"] {
                    Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["cog_puzzle"], GLOBAL_FLAGS["replacement_cog_puzzle"], None, None, None, None);
                }
            }
        }
    }

    fn rewrite_fishman_alt_shop(&mut self) {
        let fishman_screen = &mut self.rcd_file.zones[4].rooms[3].screens[3];

        for screen_object in fishman_screen.objects_with_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["language_conversation"] {
                // Persist Main Shop after Alt is Opened
                Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["fishman_shop_puzzle"], GLOBAL_FLAGS["fishman_shop_puzzle"], None, Some(TEST_OPERATIONS["gteq"]), Some(2), None);

                // Relocate Alt Shop
                if screen_object.test_operations.iter().any(|op| op.id == GLOBAL_FLAGS["fishman_shop_puzzle"] && op.op_value == 3) {
                    screen_object.x_pos = 9;
                    screen_object.y_pos = 76;
                }
            }

            // Relocate Fairy Keyspot trigger
            if screen_object.id == RCD_OBJECTS["fairy_keyspot"] {
                if screen_object.test_operations.iter().any(|op| op.id == GLOBAL_FLAGS["fishman_shop_puzzle"]) {
                    screen_object.x_pos = 9;
                    screen_object.y_pos = 74;
                }
            }

            // Relocate Alt Shop Explosion
            if screen_object.id == RCD_OBJECTS["explosion"] {
                if screen_object.test_operations.iter().any(|op| op.id == GLOBAL_FLAGS["screen_flag_0d"]) {
                    screen_object.x_pos = 7;
                    screen_object.y_pos = 76;
                }
            }
        }

        // Add Alt Shop Door Graphic
        let fishman_alt_door = ObjectWithPosition {
            id: RCD_OBJECTS["texture_draw_animation"],
            header: ObjectHeader::from_bytes([0b00100000]),
            x_pos: 9,
            y_pos: 76,
            test_operations: vec![
                Operation {
                    id: GLOBAL_FLAGS["mother_state"],
                    op_value: 3,
                    operation: TEST_OPERATIONS["neq"]
                },
                Operation {
                    id: GLOBAL_FLAGS["fishman_shop_puzzle"],
                    op_value: 3,
                    operation: TEST_OPERATIONS["eq"]
                }
            ],
            write_operations: vec![],
            parameters: vec![-1, 0, 260, 0, 40, 40, 0, 1, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0]
        };
        fishman_screen.objects_with_position.push(fishman_alt_door);
    }

    fn rewrite_boss_ankhs(&mut self, options: &HashMap<String, u64>) {
        let boss_checkpoints = options.get("BossCheckpoints").is_some_and(|option| *option > 0);
        let guardian_specific_ankh_jewels = options.get("GuardianSpecificAnkhJewels").is_some_and(|option| *option > 0);
        let alternate_mother_ankh = options.get("AlternateMotherAnkh").is_some_and(|option| *option > 0);

        // Amphisbaena
        {
            let amphisbaena_screen = &mut self.rcd_file.zones[0].rooms[8].screens[1];
            if boss_checkpoints {
                let amphisbaena_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01000001]),
                    x_pos: 15,
                    y_pos: 44,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["amphisbaena_ankh_puzzle"], op_value: 5, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["amphisbaena_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![41, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                amphisbaena_screen.objects_with_position.push(amphisbaena_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in amphisbaena_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["amphisbaena_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Sakit
        {
            let sakit_screen = &mut self.rcd_file.zones[2].rooms[8].screens[1];
            if boss_checkpoints {
                let sakit_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01000001]),
                    x_pos: 45,
                    y_pos: 6,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["sakit_ankh_puzzle"], op_value: 1, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["sakit_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![75, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                sakit_screen.objects_with_position.push(sakit_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in sakit_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["sakit_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Ellmac
        {
            let ellmac_screen = &mut self.rcd_file.zones[3].rooms[8].screens[0];
            if boss_checkpoints {
                let ellmac_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01000001]),
                    x_pos: 20,
                    y_pos: 16,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["ellmac_ankh_puzzle"], op_value: 5, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["ellmac_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![104, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                ellmac_screen.objects_with_position.push(ellmac_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in ellmac_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["ellmac_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Bahamut
        {
            let bahamut_screen = &mut self.rcd_file.zones[4].rooms[4].screens[0];
            if boss_checkpoints {
                let bahamut_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01010001]),
                    x_pos: 19,
                    y_pos: 17,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["bahamut_ankh_puzzle"], op_value: 1, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["bahamut_room_flooded"], op_value: 1, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["bahamut_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![136, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                bahamut_screen.objects_with_position.push(bahamut_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in bahamut_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["bahamut_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Viy
        {
            let viy_screen = &mut self.rcd_file.zones[5].rooms[8].screens[1];
            if boss_checkpoints {
                let viy_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01000001]),
                    x_pos: 23,
                    y_pos: 28,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["viy_ankh_puzzle"], op_value: 4, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["viy_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![149, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                viy_screen.objects_with_position.push(viy_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in viy_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["viy_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Palenque
        {
            let palenque_screen = &mut self.rcd_file.zones[6].rooms[9].screens[1];
            if boss_checkpoints {
                let palenque_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01010001]),
                    x_pos: 47,
                    y_pos: 20,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["palenque_ankh_puzzle"], op_value: 3, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["palenque_screen_mural"], op_value: 3, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["palenque_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![170, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                palenque_screen.objects_with_position.push(palenque_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in palenque_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["palenque_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Baphomet
        {
            let baphomet_screen = &mut self.rcd_file.zones[7].rooms[4].screens[1];
            if boss_checkpoints {
                let baphomet_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01000001]),
                    x_pos: 47,
                    y_pos: 4,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["baphomet_ankh_puzzle"], op_value: 2, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["baphomet_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![188, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                baphomet_screen.objects_with_position.push(baphomet_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in baphomet_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["baphomet_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Tiamat
        {
            let tiamat_screen = &mut self.rcd_file.zones[17].rooms[9].screens[0];
            if boss_checkpoints {
                let tiamat_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01000001]),
                    x_pos: 15,
                    y_pos: 4,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["tiamat_ankh_puzzle"], op_value: 1, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["tiamat_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![368, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                tiamat_screen.objects_with_position.push(tiamat_grail_point);
            }

            if guardian_specific_ankh_jewels {
                for screen_object in tiamat_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["tiamat_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }

        // Mother
        {
            if boss_checkpoints {
                let mother_entrance_screen = &mut self.rcd_file.zones[18].rooms[3].screens[1];
                let mother_grail_point = ObjectWithPosition {
                    id: RCD_OBJECTS["grail_point"],
                    header: ObjectHeader::from_bytes([0b01000001]),
                    x_pos: 33,
                    y_pos: 20,
                    test_operations: vec![
                        Operation { id: GLOBAL_FLAGS["mother_ankh_puzzle"], op_value: 1, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["mother_state"], op_value: 2, operation: TEST_OPERATIONS["lt"] },
                        Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                        Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] }
                    ],
                    write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_02"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                    parameters: vec![231, 0, 0, 1, 1, 1, 1, 506, 280]
                };
                mother_entrance_screen.objects_with_position.push(mother_grail_point);
            }

            if guardian_specific_ankh_jewels && alternate_mother_ankh {
                let mother_screen = &mut self.rcd_file.zones[18].rooms[3].screens[0];
                for screen_object in mother_screen.objects_with_position.iter_mut() {
                    if screen_object.id == RCD_OBJECTS["ankh"] {
                        screen_object.test_operations.push(Operation {
                            id: GLOBAL_FLAGS["mother_ankh_jewel_found"],
                            op_value: 1,
                            operation: TEST_OPERATIONS["gteq"]
                        });
                    }
                }
            }
        }
    }

    fn rewrite_anubis_seen(&mut self) {
        let anubis_screen = &mut self.rcd_file.zones[12].rooms[10].screens[0];
        for screen_object in anubis_screen.objects_without_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["flag_timer"] {
                Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["mulbruk_book_of_the_dead"], GLOBAL_FLAGS["replacement_mulbruk_book_of_the_dead"], None, None, None, None);
                Self::update_operations(&mut screen_object.write_operations, GLOBAL_FLAGS["mulbruk_book_of_the_dead"], GLOBAL_FLAGS["replacement_mulbruk_book_of_the_dead"], None, None, None, None);
            }
        }
    }

    fn add_dimensional_orb_ladder(&mut self) {
        let ushumgallu_screen = &mut self.rcd_file.zones[17].rooms[10].screens[1];

        let ladder = ObjectWithPosition {
            id: RCD_OBJECTS["ladder"],
            header: ObjectHeader::from_bytes([0b00010000]),
            x_pos: 28,
            y_pos: 31,
            test_operations: vec![Operation { id: GLOBAL_FLAGS["ushumgallu_state"], op_value: 2, operation: TEST_OPERATIONS["eq"] }],
            write_operations: vec![],
            parameters: vec![0, 8, 2, 0, 660, 0, 0, 1]
        };
        ushumgallu_screen.objects_with_position.push(ladder);
    }

    fn add_true_shrine_doors(&mut self) {
        let doors: Vec<HashMap<&str, i16>> = vec![
            HashMap::from([("room", 0), ("screen", 0), ("x", 17), ("y", 4), ("dest_x", 340), ("dest_y", 92)]), // Upper Entrance
            HashMap::from([("room", 8), ("screen", 1), ("x", 13), ("y", 40), ("dest_x", 300), ("dest_y", 320)]), // Lower Entrance
            HashMap::from([("room", 7), ("screen", 0), ("x", 25), ("y", 4), ("dest_x", 500), ("dest_y", 80)]), // Grail Point
            HashMap::from([("room", 9), ("screen", 0), ("x", 25), ("y", 20), ("dest_x", 300), ("dest_y", 332)]) // Treasury
        ];

        for door in doors {
            let true_shrine_screen = &mut self.rcd_file.zones[18].rooms[door["room"] as usize].screens[door["screen"] as usize];
            let warp_door = ObjectWithPosition {
                id: RCD_OBJECTS["warp_door"],
                header: ObjectHeader::from_bytes([0b00000000]),
                x_pos: door["x"],
                y_pos: door["y"],
                test_operations: vec![],
                write_operations: vec![],
                parameters: vec![0, 9, door["room"], door["screen"], door["dest_x"], door["dest_y"]]
            };
            true_shrine_screen.objects_with_position.push(warp_door);

            let door_graphic = ObjectWithPosition {
                id: RCD_OBJECTS["texture_draw_animation"],
                header: ObjectHeader::from_bytes([0b00000000]),
                x_pos: door["x"]-1,
                y_pos: door["y"]-2,
                test_operations: vec![],
                write_operations: vec![],
                parameters: vec![-1, -1, 0, 512, 80, 80, 0, 0, 1, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0]
            };
            true_shrine_screen.objects_with_position.push(door_graphic);
        }
    }

    fn add_moonlight_to_twin_lockout_fix(&mut self) {
        let moonlight_screen = &mut self.rcd_file.zones[12].rooms[2].screens[0];

        let undo_breakable_floor_timer = ObjectWithoutPosition {
            id: RCD_OBJECTS["flag_timer"],
            header: ObjectHeader::from_bytes([0b00010001]),
            test_operations: vec![Operation { id: GLOBAL_FLAGS["moonlight_to_twin_breakable_floor"], op_value: 1, operation: TEST_OPERATIONS["eq"] }],
            write_operations: vec![Operation { id: GLOBAL_FLAGS["moonlight_to_twin_breakable_floor"], op_value: 0, operation: WRITE_OPERATIONS["assign"] }],
            parameters: vec![0, 0]
        };
        moonlight_screen.objects_without_position.push(undo_breakable_floor_timer);
    }

    fn add_chain_whip_lockout_fix(&mut self) {
        let chain_whip_screen = &mut self.rcd_file.zones[5].rooms[3].screens[0];

        // Swap permanent puzzle flags to screen flags so puzzle resets on lockout
        for screen_object in chain_whip_screen.objects_with_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["trigger_dais"] || screen_object.id == RCD_OBJECTS["crusher"] {
                Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["chain_whip_dais_left"], GLOBAL_FLAGS["screen_flag_2e"], None, None, None, None);
                Self::update_operations(&mut screen_object.write_operations, GLOBAL_FLAGS["chain_whip_dais_left"], GLOBAL_FLAGS["screen_flag_2e"], None, None, None, None);

                Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["chain_whip_dais_right"], GLOBAL_FLAGS["screen_flag_2f"], None, None, None, None);
                Self::update_operations(&mut screen_object.write_operations, GLOBAL_FLAGS["chain_whip_dais_right"], GLOBAL_FLAGS["screen_flag_2f"], None, None, None, None);
            }
        }
    }

    fn add_flail_whip_lockout_fix(&mut self) {
        let locations = vec![
            HashMap::from([("room", 5), ("screen", 1)]),
            HashMap::from([("room", 6), ("screen", 2)])
        ];

        for location in locations {
            let flail_whip_screen = &mut self.rcd_file.zones[13].rooms[location["room"]].screens[location["screen"]];

            let flail_whip_lockout_timer = ObjectWithoutPosition {
                id: RCD_OBJECTS["flag_timer"],
                header: ObjectHeader::from_bytes([0b00010001]),
                test_operations: vec![Operation { id: GLOBAL_FLAGS["flail_whip_puzzle"], op_value: 1, operation: TEST_OPERATIONS["eq"] }],
                write_operations: vec![Operation { id: GLOBAL_FLAGS["flail_whip_puzzle"], op_value: 0, operation: WRITE_OPERATIONS["assign"] }],
                parameters: vec![0, 0]
            };
            flail_whip_screen.objects_without_position.push(flail_whip_lockout_timer);
        }
    }

    fn add_angel_shield_lockout_fix(&mut self) {
        let angel_shield_screen = &mut self.rcd_file.zones[17].rooms[8].screens[0];

        let left_dais_timer = ObjectWithoutPosition {
            id: RCD_OBJECTS["flag_timer"],
            header: ObjectHeader::from_bytes([0b00100001]),
            test_operations: vec![
                Operation { id: GLOBAL_FLAGS["dimensional_angel_shield_dais_left"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                Operation { id: GLOBAL_FLAGS["dimensional_children_dead"], op_value: 11, operation: TEST_OPERATIONS["gteq"] },
            ],
            write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_00"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
            parameters: vec![0, 30]
        };
        angel_shield_screen.objects_without_position.push(left_dais_timer);

        let right_dais_timer = ObjectWithoutPosition {
            id: RCD_OBJECTS["flag_timer"],
            header: ObjectHeader::from_bytes([0b00100001]),
            test_operations: vec![
                Operation { id: GLOBAL_FLAGS["dimensional_angel_shield_dais_right"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                Operation { id: GLOBAL_FLAGS["dimensional_children_dead"], op_value: 11, operation: TEST_OPERATIONS["gteq"] },
            ],
            write_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_01"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
            parameters: vec![0, 30]
        };
        angel_shield_screen.objects_without_position.push(right_dais_timer);
    }

    fn add_sun_map_lockout_fix(&mut self) {
        let sun_map_screen = &mut self.rcd_file.zones[3].rooms[0].screens[1];

        for screen_object in sun_map_screen.objects_with_position.iter_mut() {
            if screen_object.id == RCD_OBJECTS["lemeza_detector"] {
                let _ = screen_object.write_operations.extract_if(.., |op| { op.id == GLOBAL_FLAGS["sun_map_chest_ladder_despawned"] }).collect::<Vec<_>>();

                if screen_object.write_operations.iter().any(|op| { op.id == GLOBAL_FLAGS["sun_map_chest_ladder_restored"] }) {
                    screen_object.write_operations.push(Operation {
                        id: GLOBAL_FLAGS["sun_map_chest_ladder_despawned"],
                        op_value: 1,
                        operation: WRITE_OPERATIONS["assign"]
                    });
                }
            } else if screen_object.id == RCD_OBJECTS["room_spawner"] {
                if screen_object.test_operations.iter().any(|op| { op.id == GLOBAL_FLAGS["sun_map_chest_ladder_despawned"] }) {
                    screen_object.test_operations.push(Operation {
                        id: GLOBAL_FLAGS["screen_flag_0c"],
                        op_value: 0,
                        operation: TEST_OPERATIONS["eq"]
                    });
                }
            }
        }
    }

    fn add_hardmode_toggle(&mut self) {
        let hardmode_screen = &mut self.rcd_file.zones[2].rooms[2].screens[0];

        struct HardmodeOp { test_op: String, write_val: i8 }

        let ops = [
            HardmodeOp { test_op: "eq".to_string(), write_val: 0 },
            HardmodeOp { test_op: "lt".to_string(), write_val: 2 }
        ];
        for op in ops {
            let dais = ObjectWithPosition {
                id: RCD_OBJECTS["trigger_dais"],
                header: ObjectHeader::from_bytes([0b00010001]),
                x_pos: 28,
                y_pos: 5,
                test_operations: vec![Operation { id: GLOBAL_FLAGS["hardmode"], op_value: 2, operation: TEST_OPERATIONS[op.test_op.as_str()] }],
                write_operations: vec![Operation { id: GLOBAL_FLAGS["hardmode"], op_value: op.write_val, operation: WRITE_OPERATIONS["assign"] }],
                parameters: vec![0, 60, -1, 2, 0, 860, 60, 1, 10, 60]
            };
            hardmode_screen.objects_with_position.push(dais);
        }
    }

    fn add_sacred_orb_timers(&mut self) {
        let sacred_orb_screen = &mut self.rcd_file.zones[1].rooms[1].screens[1];

        for i in 0..10 {
            let orb_timer = ObjectWithoutPosition {
                id: RCD_OBJECTS["flag_timer"],
                header: ObjectHeader::from_bytes([0b00100010]),
                test_operations: vec![
                    Operation { id: GLOBAL_FLAGS["orb_count_incremented_guidance"]+i, op_value: 0, operation: TEST_OPERATIONS["eq"] },
                    Operation { id: GLOBAL_FLAGS["guidance_orb_found"]+i, op_value: 2, operation: TEST_OPERATIONS["eq"] }
                ],
                write_operations: vec![
                    Operation { id: GLOBAL_FLAGS["orb_count_incremented_guidance"]+i, op_value: 1, operation: WRITE_OPERATIONS["assign"] },
                    Operation { id: GLOBAL_FLAGS["sacred_orb_count"], op_value: 1, operation: WRITE_OPERATIONS["add"] }
                ],
                parameters: vec![0, 0]
            };
            sacred_orb_screen.objects_without_position.push(orb_timer);
        }
    }

    fn add_new_game_kill_timer(&mut self) {
        let new_game_screen = &mut self.rcd_file.zones[1].rooms[2].screens[1];

        let kill_timer = ObjectWithoutPosition {
            id: RCD_OBJECTS["flag_timer"],
            header: ObjectHeader::from_bytes([0b00010001]),
            test_operations: vec![Operation { id: GLOBAL_FLAGS["randomizer_save_loaded"], op_value: 1, operation: TEST_OPERATIONS["neq"] }],
            write_operations: vec![Operation { id: GLOBAL_FLAGS["kill_flag"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
            parameters: vec![0, 0]
        };
        new_game_screen.objects_without_position.push(kill_timer);
    }

    fn  clean_up_operations(&mut self) {
        // Remove Fairy Conversation Requirement from Buer Room Ladder
        {
            let buer_screen = &mut self.rcd_file.zones[3].rooms[2].screens[1];
            for screen_object in buer_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["hitbox_generator"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["endless_fairyqueen"] }).collect::<Vec<_>>();
                }
            }
        }

        // Remove Slushfund Conversation Requirement from Pepper Puzzle
        {
            let pepper_puzzle_screen = &mut self.rcd_file.zones[0].rooms[0].screens[0];
            for screen_object in pepper_puzzle_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["use_item"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["slushfund_conversation"] }).collect::<Vec<_>>();
                }
            }
        }

        // Remove Crucifix Check from Crucifix Puzzle Torches
        {
            let crucifix_puzzle_screen = &mut self.rcd_file.zones[0].rooms[1].screens[1];
            for screen_object in crucifix_puzzle_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["texture_draw_animation"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["crucifix_found"] }).collect::<Vec<_>>();
                }
            }
        }

        // Remove Plane Missing Requirement from Plane Puzzle
        {
            let plane_platform_left_screen = &mut self.rcd_file.zones[13].rooms[7].screens[0];
            for screen_object in plane_platform_left_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["counterweight_platform"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["plane_found"] }).collect::<Vec<_>>();
                }
            }
        }
        {
            let plane_platform_right_screen = &mut self.rcd_file.zones[13].rooms[7].screens[2];
            for screen_object in plane_platform_right_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["counterweight_platform"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["plane_found"] }).collect::<Vec<_>>();
                }
            }
        }

        // Remove Dracuet Check From Guidance Elevator Block
        {
            let guidance_elevator_screen = &mut self.rcd_file.zones[0].rooms[6].screens[0];
            for screen_object in guidance_elevator_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["hitbox_generator"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["mulbruk_father"] }).collect::<Vec<_>>();
                }
            }
        }

        // Remove Shrine Chest Check from Xelpud Conversations
        {
            let xelpud_conversation_screen = &mut self.rcd_file.zones[1].rooms[2].screens[1];
            for screen_object in xelpud_conversation_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["language_conversation"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["shrine_diary_chest"] }).collect::<Vec<_>>();
                }
            }
        }

        // Remove Unknown Test from Mulbruk Conversations
        {
            let mulbruk_conversation_screen = &mut self.rcd_file.zones[3].rooms[3].screens[0];
            for screen_object in mulbruk_conversation_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["language_conversation"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["mulbruk_conversation_unknown"] }).collect::<Vec<_>>();
                    Self::update_operations(&mut screen_object.test_operations, GLOBAL_FLAGS["score"], GLOBAL_FLAGS["score"], Some(TEST_OPERATIONS["gteq"]), None, Some(56), Some(0));
                }
            }
        }

        // Remove Book of the Dead Write Flag from Anubis Kill
        {
            let anubis_screen = &mut self.rcd_file.zones[12].rooms[10].screens[0];
            for screen_object in anubis_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["big_anubis"] {
                    let _ = screen_object.write_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["mulbruk_book_of_the_dead"] }).collect::<Vec<_>>();
                }
            }
        }

        // Remove Ankh Jewel Check From Temple of the Sun Ankh Jewel Chest Puzzle
        {
            let sun_ankh_jewel_screen = &mut self.rcd_file.zones[3].rooms[7].screens[0];
            for screen_object in sun_ankh_jewel_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["trigger_dais"] {
                    let _ = screen_object.test_operations.extract_if(..,|op| { op.id == GLOBAL_FLAGS["ankh_jewel_sun"] }).collect::<Vec<_>>();
                }
            }
        }
    }

    fn create_grail_autoscans(&mut self) {
        for (zone_index, zone) in self.rcd_file.zones.iter_mut().enumerate() {
            for room in zone.rooms.iter_mut() {
                for screen in room.screens.iter_mut() {
                    let mut lemeza_detector = None;
                    for screen_object in screen.objects_with_position.iter_mut() {
                        if screen_object.id == RCD_OBJECTS["scannable"] {
                            let language_block = screen_object.parameters[0];
                            let frontside = language_block == 41 || language_block == 75 || language_block == 104 || language_block == 136 || language_block == 149 || language_block == 170 || language_block == 188 || language_block == 221 || (language_block == 231 && zone_index == 9);
                            let backside = language_block == 250 || language_block == 275 || language_block == 291 || language_block == 305 || language_block == 323 || language_block == 339 || language_block == 206 || language_block == 358 || (language_block == 231 && zone_index != 9);

                            if frontside || backside {
                                let grail_flag = grail_flag_by_zone(zone_index, frontside);

                                lemeza_detector = Some(ObjectWithPosition {
                                    id: RCD_OBJECTS["lemeza_detector"],
                                    header: ObjectHeader::from_bytes([0b00010001]),
                                    x_pos: screen_object.x_pos,
                                    y_pos: screen_object.y_pos-1,
                                    test_operations: vec![Operation {id: grail_flag, op_value: 0, operation: TEST_OPERATIONS["eq"]}],
                                    write_operations: vec![Operation {id: grail_flag, op_value: 1, operation: WRITE_OPERATIONS["assign"]}],
                                    parameters: vec![0, 0, 0, 0, 2, 3]
                                });
                            }
                        }
                    }
                    if let Some(autoscan) = lemeza_detector {
                        screen.objects_with_position.push(autoscan);
                    }
                }
            }
        }
    }

    fn create_ancient_lamulanese_timer(&mut self) {
        let start_screen = &mut self.rcd_file.zones[1].rooms[2].screens[1];

        let lamulanese_timer = ObjectWithoutPosition {
            id: RCD_OBJECTS["flag_timer"],
            header: ObjectHeader::from_bytes([0b00010010]),
            test_operations: vec![Operation {id: GLOBAL_FLAGS["ancient_lamulanese_learned"], op_value: 0, operation: TEST_OPERATIONS["eq"]}],
            write_operations: vec![
                Operation { id: GLOBAL_FLAGS["translation_tablets_read"], op_value: 3, operation: WRITE_OPERATIONS["assign"] },
                Operation { id: GLOBAL_FLAGS["ancient_lamulanese_learned"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }
            ],
            parameters: vec![0, 0]
        };
        start_screen.objects_without_position.push(lamulanese_timer);
    }

    fn create_alternate_mother_ankh(&mut self) {
        {
            let mother_screen = &mut self.rcd_file.zones[18].rooms[3].screens[0];

            // Remove Mother Animations
            let _ = mother_screen.objects_with_position.extract_if(.., |screen_object| {
                screen_object.id == RCD_OBJECTS["animation"]
            }).collect::<Vec<_>>();

            // Modify Mother Ankh
            for screen_object in mother_screen.objects_with_position.iter_mut() {
                if screen_object.id == RCD_OBJECTS["mother_ankh"] {
                    screen_object.id = RCD_OBJECTS["ankh"];
                    screen_object.parameters[0] = 8;
                    screen_object.write_operations[0].op_value = 1;
                    screen_object.write_operations[1].op_value = 2;
                    screen_object.y_pos += 3;
                }
            }
        }

        // Return Ankh Jewel if warped out of fight
        {
            let surface_screen = &mut self.rcd_file.zones[1].rooms[11].screens[0];

            let instant_item = ObjectWithPosition {
                id: RCD_OBJECTS["instant_item"],
                header: ObjectHeader::from_bytes([0b00010010]),
                x_pos: 5,
                y_pos: 3,
                test_operations: vec![Operation { id: GLOBAL_FLAGS["mother_ankh_jewel_recovery"], op_value: 1, operation: TEST_OPERATIONS["eq"] }],
                write_operations: vec![
                    Operation { id: GLOBAL_FLAGS["mother_ankh_jewel_recovery"], op_value: 0, operation: WRITE_OPERATIONS["assign"] },
                    Operation { id: GLOBAL_FLAGS["mother_state"], op_value: 1, operation: WRITE_OPERATIONS["assign"] },
                ],
                parameters: vec![19, 12, 16, 39]
            };
            surface_screen.objects_with_position.push(instant_item);

            let flag_timer = ObjectWithoutPosition {
                id: RCD_OBJECTS["flag_timer"],
                header: ObjectHeader::from_bytes([0b00110001]),
                test_operations: vec![
                    Operation { id: GLOBAL_FLAGS["mother_state"], op_value: 2, operation: TEST_OPERATIONS["eq"] },
                    Operation { id: GLOBAL_FLAGS["escape"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                    Operation { id: GLOBAL_FLAGS["mother_ankh_jewel_recovery"], op_value: 0, operation: TEST_OPERATIONS["eq"] },
                ],
                write_operations: vec![Operation { id: GLOBAL_FLAGS["mother_ankh_jewel_recovery"], op_value: 1, operation: WRITE_OPERATIONS["assign"]}],
                parameters: vec![0, 0]
            };
            surface_screen.objects_without_position.push(flag_timer);
        }
    }

    fn randomize_doors(&mut self, door_map: HashMap<String, [String;2]>) -> Result<(), FileGenerationError> {
        for (source_door_name, [destination_door_name, door_requirement_name]) in door_map.clone() {

            // The Endless Corridor Exit is tracked for mapping purposes, but doesn't have a Warp Door to update
            if source_door_name == "Endless One-way Exit" { continue; }

            let source_door = DOORS.get(source_door_name.as_str()).ok_or_else(|| {
                debug!("Source Door {} Doesn't Exist", source_door_name);
                FileGenerationError::MalformedSlotData
            })?.clone();

            let destination_door = DOORS.get(destination_door_name.as_str()).ok_or_else(|| {
                debug!("Destination Door {} Doesn't Exist", source_door_name);
                FileGenerationError::MalformedSlotData
            })?.clone();

            let door_screen = &mut self.rcd_file.zones[source_door.zone as usize].rooms[source_door.room as usize].screens[source_door.screen as usize];

            // Remove existing Door assets to ensure consistent application of new door
            let _ = door_screen.objects_with_position.extract_if(.., |object| {
                object.id == RCD_OBJECTS["warp_door"] ||
                source_door.remove_with_position.iter().any(|removal_object| {
                    removal_object.rcd_id == object.id &&
                    Self::flag_match(object.test_operations.clone(), removal_object.test_flags.clone()) &&
                    Self::flag_match(object.write_operations.clone(), removal_object.write_flags.clone())
                })
            }).collect::<Vec<_>>();

            let _ = door_screen.objects_without_position.extract_if(.., |object| {
                source_door.remove_without_position.iter().any(|removal_object| {
                    removal_object.rcd_id == object.id &&
                    Self::flag_match(object.test_operations.clone(), removal_object.test_flags.clone()) &&
                    Self::flag_match(object.write_operations.clone(), removal_object.write_flags.clone())
                })
            }).collect::<Vec<_>>();

            let mut door_test_ops: Vec<Operation> = Vec::new();
            let door_req = DOOR_REQUIREMENTS.get(door_requirement_name.as_str()).ok_or_else(|| {
                debug!("Door Requirement Missing for : {:?}", door_requirement_name);
                FileGenerationError::MalformedSlotData
            })?.clone();

            if let Some(primary_flag) = door_req.primary_flag {
                let op_value = if door_requirement_name == "Key" { 1 } else { 3 };
                door_test_ops.push(Operation { id: primary_flag, op_value, operation: TEST_OPERATIONS["eq"] });
            }

            if let Some(mirror_flag) = door_req.mirror_flag {
                door_test_ops.push(Operation { id: mirror_flag, op_value: 1, operation: TEST_OPERATIONS["eq"] });
            }

            let header_bytes = (door_test_ops.len() << 4) as u8;

            let door = ObjectWithPosition {
                id: RCD_OBJECTS["warp_door"],
                header: ObjectHeader::from_bytes([header_bytes]),
                x_pos: source_door.x_pos,
                y_pos: source_door.y_pos,
                test_operations: door_test_ops,
                write_operations: vec![],
                parameters: vec![
                    0,
                    destination_door.zone,
                    destination_door.room,
                    destination_door.screen,
                    destination_door.x_pos,
                    destination_door.y_pos
                ]
            };
            door_screen.objects_with_position.push(door);

            if source_door.zone != 19 {
                let image_x = match source_door.zone {
                    6 => 80,
                    10 => 160,
                    13 => 240,
                    _ => 0
                };

                let door_graphic = ObjectWithPosition {
                    id: RCD_OBJECTS["texture_draw_animation"],
                    header: ObjectHeader::from_bytes([0b0]),
                    x_pos: source_door.x_pos - 1,
                    y_pos: source_door.y_pos - 2,
                    test_operations: vec![],
                    write_operations: vec![],
                    parameters: vec![-1, -1, image_x, 512, 80, 80, 0, 0, 1, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1]
                };
                door_screen.objects_with_position.push(door_graphic);
            }

            if let Some(primary_flag) = door_req.primary_flag {
                let graphic_index = door_req.graphic_index.ok_or_else(|| {
                    debug!("Graphic Index Missing for : {:?}", door_requirement_name);
                    FileGenerationError::MalformedSlotData
                })?;

                if let Some(mirror_flag) = door_req.mirror_flag {
                    // Place Mirror Cover Effects
                    let door_cover_animation = ObjectWithPosition {
                        id: RCD_OBJECTS["texture_draw_animation"],
                        header: ObjectHeader::from_bytes([0b00100001]),
                        x_pos: source_door.x_pos,
                        y_pos: source_door.y_pos,
                        test_operations: vec![
                            Operation { id: primary_flag, op_value: 0, operation: TEST_OPERATIONS["eq"] },
                            Operation { id: GLOBAL_FLAGS["bronze_mirror_found"], op_value: 2, operation: TEST_OPERATIONS["eq"] }
                        ],
                        write_operations: vec![Operation { id: primary_flag, op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                        parameters: vec![0, -1, 0, 360, 40, 40, 1, 7, 6, 1, 0, 0, 5, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1]
                    };
                    door_screen.objects_with_position.push(door_cover_animation);

                    let door_req_graphic = ObjectWithPosition {
                        id: RCD_OBJECTS["texture_draw_animation"],
                        header: ObjectHeader::from_bytes([0b00010000]),
                        x_pos: source_door.x_pos - 1,
                        y_pos: source_door.y_pos - 2,
                        test_operations: vec![Operation { id: mirror_flag, op_value: 0, operation: TEST_OPERATIONS["gt"] }],
                        write_operations: vec![],
                        parameters: vec![0, -1, 50 * graphic_index, 592, 50, 36, 0, 0, 1, 0, 0, 0, 5, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1]
                    };
                    door_screen.objects_with_position.push(door_req_graphic);
                } else {
                    // Place Key Fairy Effects
                    let fairy_keyspot = ObjectWithPosition {
                        id: RCD_OBJECTS["fairy_keyspot"],
                        header: ObjectHeader::from_bytes([0b00010011]),
                        x_pos: source_door.x_pos,
                        y_pos: source_door.y_pos - 2,
                        test_operations: vec![Operation { id: primary_flag, op_value: 0, operation: TEST_OPERATIONS["eq"] }],
                        write_operations: vec![
                            Operation { id: primary_flag, op_value: 1, operation: WRITE_OPERATIONS["assign"] },
                            Operation { id: GLOBAL_FLAGS["screen_flag_29"], op_value: 1, operation: WRITE_OPERATIONS["assign"] },
                            Operation { id: GLOBAL_FLAGS["keyfairy_point_gate_of_time"], op_value: 1, operation: WRITE_OPERATIONS["assign"] }
                        ],
                        parameters: vec![0, 40, 40]
                    };
                    door_screen.objects_with_position.push(fairy_keyspot);

                    let door_cover_animation = ObjectWithPosition {
                        id: RCD_OBJECTS["texture_draw_animation"],
                        header: ObjectHeader::from_bytes([0b00010001]),
                        x_pos: source_door.x_pos,
                        y_pos: source_door.y_pos,
                        test_operations: vec![Operation { id: primary_flag, op_value: 0, operation: TEST_OPERATIONS["eq"] }],
                        write_operations: vec![Operation { id: primary_flag, op_value: 1, operation: WRITE_OPERATIONS["assign"] }],
                        parameters: vec![0, -1, 0, 360, 40, 40, 1, 7, 6, 1, 0, 0, 5, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1]
                    };
                    door_screen.objects_with_position.push(door_cover_animation);

                    let door_req_graphic = ObjectWithPosition {
                        id: RCD_OBJECTS["texture_draw_animation"],
                        header: ObjectHeader::from_bytes([0b00000000]),
                        x_pos: source_door.x_pos - 1,
                        y_pos: source_door.y_pos - 2,
                        test_operations: vec![],
                        write_operations: vec![],
                        parameters: vec![0, -1, 50 * graphic_index, 592, 50, 36, 0, 0, 1, 0, 0, 0, 5, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1]
                    };
                    door_screen.objects_with_position.push(door_req_graphic);

                    let keyfairy_timer = ObjectWithoutPosition {
                        id: RCD_OBJECTS["flag_timer"],
                        header: ObjectHeader::from_bytes([0b00010010]),
                        test_operations: vec![Operation { id: GLOBAL_FLAGS["keyfairy_point_gate_of_time"], op_value: 1, operation: TEST_OPERATIONS["eq"] }],
                        write_operations: vec![
                            Operation { id: GLOBAL_FLAGS["keyfairy_point_gate_of_time"], op_value: 2, operation: WRITE_OPERATIONS["assign"] },
                            Operation { id: GLOBAL_FLAGS["keyfairy_points"], op_value: 1, operation: WRITE_OPERATIONS["add"] }
                        ],
                        parameters: vec![]
                    };
                    door_screen.objects_without_position.push(keyfairy_timer);

                    let keyfairy_sound = ObjectWithoutPosition {
                        id: RCD_OBJECTS["sound_effect"],
                        header: ObjectHeader::from_bytes([0b00010000]),
                        test_operations: vec![Operation { id: GLOBAL_FLAGS["screen_flag_29"], op_value: 1, operation: TEST_OPERATIONS["eq"] }],
                        write_operations: vec![],
                        parameters: vec![0x7b, 127, 0, -1500, 115, 127, -1500, 19, 0, 0, 0, 10, 8, 2, 0]
                    };
                    door_screen.objects_without_position.push(keyfairy_sound);

                    let door_open_woodle = ObjectWithoutPosition {
                        id: RCD_OBJECTS["sound_effect"],
                        header: ObjectHeader::from_bytes([0b00110000]),
                        test_operations: vec![
                            Operation { id: GLOBAL_FLAGS["shell_horn_found"], op_value: 2, operation: TEST_OPERATIONS["eq"] },
                            Operation { id: primary_flag, op_value: 1, operation: TEST_OPERATIONS["eq"] },
                            Operation { id: GLOBAL_FLAGS["screen_flag_29"], op_value: 1, operation: TEST_OPERATIONS["eq"] }
                        ],
                        write_operations: vec![],
                        parameters: vec![0x1e, 120, 64, 0, 120, 64, 0, 25, 1, 5, 0, 10, 0, 0, 0]
                    };
                    door_screen.objects_without_position.push(door_open_woodle);
                }

                // Change Gate of Time Door Open flag to New Primary Flag for 0x93, 0x12, 0xe RCD object
                if &source_door_name == "Extinction Key Door" {
                    for object in door_screen.objects_with_position.iter_mut() {
                        let matching_object = [RCD_OBJECTS["texture_draw_animation"], RCD_OBJECTS["hitbox_generator"], RCD_OBJECTS["room_spawner"], RCD_OBJECTS["scannable"]].contains(&object.id);
                        if matching_object {
                            let matching_op = object.test_operations.iter().any(|op| { op.id == GLOBAL_FLAGS["gate_of_time_puzzle"] });
                            if matching_op {
                                let door_req_value = if door_requirement_name == "Key" { 1 } else { 3 };
                                Self::update_operations(&mut object.test_operations, GLOBAL_FLAGS["gate_of_time_puzzle"], primary_flag, None, Some(TEST_OPERATIONS["eq"]), None, Some(door_req_value));
                                if let Some(mirror_flag) = door_req.mirror_flag {
                                    object.test_operations.push(Operation { id: mirror_flag, op_value: 1, operation: TEST_OPERATIONS["eq"] });
                                }
                            }
                        }
                    }
                }
            } else {
                // Remove Gate of Time Closed Objects
                if &source_door_name == "Extinction Key Door" {
                    let _ = door_screen.objects_with_position.extract_if(.., |object| {
                        [RCD_OBJECTS["texture_draw_animation"], RCD_OBJECTS["hitbox_generator"], RCD_OBJECTS["room_spawner"], RCD_OBJECTS["scannable"]].contains(&object.id) &&
                        object.test_operations.iter().any(|op| { op.id == GLOBAL_FLAGS["gate_of_time_puzzle"] })
                    });
                }
            }
        }

        Ok(())
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

    fn flag_match(object_flags: Vec<Operation>, comparison_flags: Vec<RcdComparisonFlag>) -> bool {
        comparison_flags.is_empty() || comparison_flags.iter().all(|comparison_flag| {
            object_flags.iter().any(|object_flag| {
                comparison_flag.id == object_flag.id &&
                comparison_flag.op.is_none_or(|op| op == object_flag.operation) &&
                comparison_flag.value.is_none_or(|value| value == object_flag.op_value)
            })
        })
    }
}
