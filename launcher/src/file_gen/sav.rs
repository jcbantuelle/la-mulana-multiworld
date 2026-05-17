use archipelago_api::api::SlotData;
use binrw::{BinRead, BinWrite};
use std::collections::HashMap;
use std::io::Cursor;

use crate::file_gen::generator::FileGenerationError;

use super::lm_consts::{GLOBAL_FLAGS, INVENTORY, STARTING_WEAPONS};

const NUM_EMAILS: u16 = 46;

#[derive(Debug, BinRead, BinWrite)]
pub struct LaMulanaSav {
    valid: u8,
    game_time: u32,
    zone: u8,
    room: u8,
    screen: u8,
    x_postion: u16,
    y_postion: u16,
    max_hp: u8,
    current_hp: u16,
    current_exp: u16,
    flags: [u8; 4096],
    inventory: [u16; 255],
    held_main_weapon: u8,
    held_sub_weapon: u8,
    held_use_item: u8,
    held_main_weapon_slot: u8,
    held_sub_weapon_slot: u8,
    held_use_item_slot: u8,
    num_emails: u16,
    received_emails: u16,
    #[br(count = num_emails)]
    emails: Vec<Email>,
    equipped_software: [u8; 20],
    rosettas_read: [u16; 3],
    #[br(count = 20)]
    bunemon_records: Vec<BunemonRecord>,
    mantras_learned: [u8; 10],
    maps_owned_bit_array: u32
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Email {
    screenplay_card: u16,
    game_time_received: u32,
    mail_number: u16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct BunemonRecord {
    slot_number: u8,
    field_map_card: u16,
    field_map_record: u16,
    location_card: u16,
    location_record: u16,
    text_card: u16,
    text_record: u16,
    is_tablet: u8
}

pub struct Sav {
    save_file: LaMulanaSav,
    global_flag_lookup: HashMap<&'static str, usize>
}

impl Sav {
    pub fn new() -> Self {
        let global_flag_lookup = GLOBAL_FLAGS.iter().map(|(k,v)| (*k, *v as usize)).collect::<HashMap<&str, usize>>();
        let save_file = LaMulanaSav {
            valid: 1,
            game_time: 0,
            zone: 1,
            room: 2,
            screen: 1,
            x_postion: 940 % 640,
            y_postion: 160 % 480,
            max_hp: 1,
            current_hp: 32,
            current_exp: 0,
            flags: Self::default_flags(global_flag_lookup.clone()),
            inventory: [0; 255],
            held_main_weapon: 0,
            held_sub_weapon: 0xff,
            held_use_item: 0xff,
            held_main_weapon_slot: 0,
            held_sub_weapon_slot: 0,
            held_use_item_slot: 0,
            num_emails: NUM_EMAILS,
            received_emails: 0,
            emails: Self::default_emails(),
            equipped_software: [0; 20],
            rosettas_read: [0,0,0],
            bunemon_records: Self::default_bunemon_records(),
            mantras_learned: [0; 10],
            maps_owned_bit_array: 0
        };
        Sav { save_file, global_flag_lookup }
    }

    pub fn apply_mods(&mut self, slot_data: &SlotData) -> Result<(), FileGenerationError> {
        self.set_starting_weapon(slot_data.options["StartingWeapon"])?;
        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        self.save_file.write_be(&mut writer).map_err(|_| FileGenerationError::SaveFileModFailure)?;
        Ok(writer.into_inner())
    }

    fn set_starting_weapon(&mut self, weapon_id: u64) -> Result<(), FileGenerationError> {
        let starting_weapon = STARTING_WEAPONS[&weapon_id];
        if starting_weapon != "Leather Whip" {
            // Remove Default Leather Whip
            self.save_file.inventory[0] = 0xffff;

            match starting_weapon {
                "Knife" => {
                    self.save_file.flags[self.global_flag_lookup["knife_found"]] = 2;
                    self.save_file.inventory[INVENTORY["knife"]] = 1;
                    self.save_file.held_main_weapon = 3;
                    self.save_file.held_main_weapon_slot = 1;
                },
                "Key Sword" => {
                    self.save_file.flags[self.global_flag_lookup["keysword_found"]] = 2;
                    self.save_file.inventory[INVENTORY["keysword"]] = 1;
                    self.save_file.held_main_weapon = 4;
                    self.save_file.held_main_weapon_slot = 2;
                },
                "Axe" => {
                    self.save_file.flags[self.global_flag_lookup["axe_found"]] = 2;
                    self.save_file.inventory[INVENTORY["axe"]] = 1;
                    self.save_file.held_main_weapon = 5;
                    self.save_file.held_main_weapon_slot = 3;
                },
                "Katana" => {
                    self.save_file.flags[self.global_flag_lookup["katana_found"]] = 2;
                    self.save_file.inventory[INVENTORY["katana"]] = 1;
                    self.save_file.held_main_weapon = 6;
                    self.save_file.held_main_weapon_slot = 4;
                },
                subweapon => {
                    self.save_file.held_main_weapon = 0xff;
                    self.save_file.held_main_weapon_slot = 0xff;
                    match subweapon {
                        "Shuriken" => {
                            self.save_file.flags[self.global_flag_lookup["shurikens_found"]] = 2;
                            self.save_file.inventory[INVENTORY["shurikens"]] = 1;
                            self.save_file.inventory[INVENTORY["shuriken_ammo"]] = 150;
                            self.save_file.held_sub_weapon = 8;
                            self.save_file.held_sub_weapon_slot = 0;
                        },
                        "Rolling Shuriken" => {
                            self.save_file.flags[self.global_flag_lookup["rolling_shurikens_found"]] = 2;
                            self.save_file.inventory[INVENTORY["rolling_shurikens"]] = 1;
                            self.save_file.inventory[INVENTORY["rolling_shuriken_ammo"]] = 100;
                            self.save_file.held_sub_weapon = 9;
                            self.save_file.held_sub_weapon_slot = 1;
                        },
                        "Earth Spear" => {
                            self.save_file.flags[self.global_flag_lookup["earth_spears_found"]] = 2;
                            self.save_file.inventory[INVENTORY["earth_spears"]] = 1;
                            self.save_file.inventory[INVENTORY["earth_spear_ammo"]] = 80;
                            self.save_file.held_sub_weapon = 10;
                            self.save_file.held_sub_weapon_slot = 2;
                        },
                        "Flare Gun" => {
                            self.save_file.flags[self.global_flag_lookup["flare_gun_found"]] = 2;
                            self.save_file.inventory[INVENTORY["flare_gun"]] = 1;
                            self.save_file.inventory[INVENTORY["flare_gun_ammo"]] = 80;
                            self.save_file.held_sub_weapon = 11;
                            self.save_file.held_sub_weapon_slot = 3;
                        },
                        "Bomb" => {
                            self.save_file.flags[self.global_flag_lookup["bombs_found"]] = 2;
                            self.save_file.inventory[INVENTORY["bombs"]] = 1;
                            self.save_file.inventory[INVENTORY["bomb_ammo"]] = 30;
                            self.save_file.held_sub_weapon = 12;
                            self.save_file.held_sub_weapon_slot = 4;
                        },
                        "Chakram" => {
                            self.save_file.flags[self.global_flag_lookup["chakrams_found"]] = 2;
                            self.save_file.inventory[INVENTORY["chakrams"]] = 1;
                            self.save_file.inventory[INVENTORY["chakram_ammo"]] = 10;
                            self.save_file.held_sub_weapon = 13;
                            self.save_file.held_sub_weapon_slot = 5;
                        },
                        "Caltrops" => {
                            self.save_file.flags[self.global_flag_lookup["caltrops_found"]] = 2;
                            self.save_file.inventory[INVENTORY["caltrops"]] = 1;
                            self.save_file.inventory[INVENTORY["caltrop_ammo"]] = 80;
                            self.save_file.held_sub_weapon = 14;
                            self.save_file.held_sub_weapon_slot = 6;
                        },
                        "Pistol" => {
                            self.save_file.flags[self.global_flag_lookup["pistol_found"]] = 2;
                            self.save_file.inventory[INVENTORY["pistol"]] = 1;
                            self.save_file.inventory[INVENTORY["pistol_clip_ammo"]] = 3;
                            self.save_file.inventory[INVENTORY["pistol_bullet_ammo"]] = 6;
                            self.save_file.held_sub_weapon = 15;
                            self.save_file.held_sub_weapon_slot = 7;
                        },
                        &_ => {
                            return Err(FileGenerationError::InvalidStartingWeapon);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn default_flags(global_flag_lookup: HashMap<&'static str, usize>) -> [u8; 4096] {
        let mut flags = [0; 4096];

        flags[global_flag_lookup["end_start_animation"]] = 1;
        flags[global_flag_lookup["hell_dlc"]] = 1;
        flags[global_flag_lookup["randomizer_save_loaded"]] = 1;
        flags[global_flag_lookup["received_items_index_2"]] = 1;

        flags
    }

    fn default_emails() -> Vec<Email> {
        vec![Self::default_email(); NUM_EMAILS.into()]
    }

    fn default_email() -> Email {
        Email {
            screenplay_card: 0,
            game_time_received: 0,
            mail_number: 0xffff
        }
    }

    fn default_bunemon_records() -> Vec<BunemonRecord> {
        vec![Self::default_bunemon_record(); 20]
    }

    fn default_bunemon_record() -> BunemonRecord {
        BunemonRecord {
            slot_number: 0xff,
            field_map_card: 0,
            field_map_record: 0,
            location_card: 0,
            location_record: 0,
            text_card: 0,
            text_record: 0,
            is_tablet: 0
        }
    }
}
