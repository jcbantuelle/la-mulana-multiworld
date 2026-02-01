use binrw::{BinRead, BinWrite};

use crate::archipelago::api::SlotData;

use super::lm_flags::GLOBAL_FLAGS;

const NUM_EMAILS: u16 = 46;

#[derive(Debug, BinRead, BinWrite)]
#[br(big)]
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
    invetory: [u16; 255],
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

pub fn generate(slot_data: &SlotData) {
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
        flags: default_flags(),
        invetory: [0; 255],
        held_main_weapon: 0,
        held_sub_weapon: 0xff,
        held_use_item: 0xff,
        held_main_weapon_slot: 0,
        held_sub_weapon_slot: 0,
        held_use_item_slot: 0,
        num_emails: NUM_EMAILS,
        received_emails: 0,
        emails: default_emails(),
        equipped_software: [0; 20],
        rosettas_read: [0,0,0],
        bunemon_records: default_bunemon_records(),
        mantras_learned: [0; 10],
        maps_owned_bit_array: 0
    };

    // Open the binary file
    // let file = File::open("script.rcd")?;
    // let mut reader = BufReader::new(file);

    // return LaMulanaSave::new()
}

fn default_flags() -> [u8; 4096] {
    let mut flags = [0; 4096];

    flags[GLOBAL_FLAGS["end_start_animation"]] = 1;
    flags[GLOBAL_FLAGS["hell_dlc"]] = 1;
    flags[GLOBAL_FLAGS["randomizer_save_loaded"]] = 1;
    flags[GLOBAL_FLAGS["received_items_index_2"]] = 1;

    flags
}

fn default_emails() -> Vec<Email> {
    vec![default_email(); NUM_EMAILS.into()]
}

fn default_email() -> Email {
    Email {
        screenplay_card: 0,
        game_time_received: 0,
        mail_number: 0xffff
    }
}

fn default_bunemon_records() -> Vec<BunemonRecord> {
    vec![default_bunemon_record(); 20]
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
