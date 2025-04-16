use log::debug;
use winapi::shared::minwindef::*;
use winapi::um::timeapi::timeGetTime;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::get_application;
use crate::application::ApplicationMemoryOps;
use crate::lm_structs::taskdata::TaskData;
use crate::lm_structs::script_header::{ScriptHeader, ScriptSubHeader};
use crate::screenplay;

pub type FnGameLoop = extern "C" fn();
pub type FnPopupDialogDrawIntercept = extern "C" fn(&mut TaskData);

pub enum EggType {
    SingleEgg {egg_id: i32},
    MultiEgg {flag: usize, egg_ids: Vec<i32>}
}

lazy_static! {
    pub static ref EGG_LOOKUP: HashMap<String, EggType> = HashMap::from([
        ("1-2-0".to_string(), EggType::SingleEgg{egg_id: 170}),
        ("1-1-1".to_string(), EggType::SingleEgg{egg_id: 170}),
        ("11-1-0".to_string(), EggType::SingleEgg{egg_id: 171}),
        ("7-1-0".to_string(), EggType::SingleEgg{egg_id: 172}),
        ("6-9-0".to_string(), EggType::SingleEgg{egg_id: 173}),
        ("3-0-1".to_string(), EggType::SingleEgg{egg_id: 208}),
        ("13-3-1".to_string(), EggType::SingleEgg{egg_id: 175}),
        ("5-9-0".to_string(), EggType::SingleEgg{egg_id: 176}),
        ("12-0-1".to_string(), EggType::SingleEgg{egg_id: 177}),
        ("12-0-1".to_string(), EggType::MultiEgg{flag: 0x00c, egg_ids: vec![177,193]}),
        ("23-9-0".to_string(), EggType::SingleEgg{egg_id: 178}),
        ("11-3-0".to_string(), EggType::SingleEgg{egg_id: 179}),
        ("0-0-0".to_string(), EggType::SingleEgg{egg_id: 180}),
        ("2-4-0".to_string(), EggType::SingleEgg{egg_id: 175}),
        ("4-2-0".to_string(), EggType::SingleEgg{egg_id: 207}),
        ("10-5-1".to_string(), EggType::SingleEgg{egg_id: 170}),
        ("14-5-0".to_string(), EggType::SingleEgg{egg_id: 218}),
        ("15-1-1".to_string(), EggType::SingleEgg{egg_id: 217}),
        ("17-1-0".to_string(), EggType::SingleEgg{egg_id: 208}),
        ("8-4-3".to_string(), EggType::SingleEgg{egg_id: 184}),
        ("8-1-1".to_string(), EggType::SingleEgg{egg_id: 185}),
        ("9-0-0".to_string(), EggType::SingleEgg{egg_id: 186}),
        ("18-0-0".to_string(), EggType::SingleEgg{egg_id: 187}),
        ("9-5-0".to_string(), EggType::SingleEgg{egg_id: 187}),
        ("16-0-0".to_string(), EggType::SingleEgg{egg_id: 183}),
        ("21-0-0".to_string(), EggType::MultiEgg{flag: 0x00c, egg_ids: vec![188,185]}),
        ("5-3-0".to_string(), EggType::SingleEgg{egg_id: 189}),
        ("2-1-1".to_string(), EggType::SingleEgg{egg_id: 186}),
        ("12-4-3".to_string(), EggType::SingleEgg{egg_id: 213}),
        ("8-5-2".to_string(), EggType::SingleEgg{egg_id: 199}),
        ("3-4-5".to_string(), EggType::SingleEgg{egg_id: 191}),
        ("1-6-1".to_string(), EggType::SingleEgg{egg_id: 192}),
        ("1-7-0".to_string(), EggType::SingleEgg{egg_id: 193}),
        ("2-2-0".to_string(), EggType::SingleEgg{egg_id: 190}),
        ("3-7-1".to_string(), EggType::SingleEgg{egg_id: 212}),
        ("4-7-0".to_string(), EggType::SingleEgg{egg_id: 181}),
        ("4-3-0".to_string(), EggType::SingleEgg{egg_id: 214}),
        ("5-6-0".to_string(), EggType::SingleEgg{egg_id: 194}),
        ("5-4-0".to_string(), EggType::SingleEgg{egg_id: 189}),
        ("6-7-1".to_string(), EggType::SingleEgg{egg_id: 195}),
        ("7-9-0".to_string(), EggType::SingleEgg{egg_id: 196}),
        ("7-8-1".to_string(), EggType::SingleEgg{egg_id: 196}),
        ("9-2-1".to_string(), EggType::SingleEgg{egg_id: 197}),
        ("10-9-0".to_string(), EggType::SingleEgg{egg_id: 198}),
        ("10-2-0".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("12-3-0".to_string(), EggType::SingleEgg{egg_id: 205}),
        ("13-4-0".to_string(), EggType::SingleEgg{egg_id: 207}),
        ("13-8-0".to_string(), EggType::SingleEgg{egg_id: 209}),
        ("12-9-1".to_string(), EggType::SingleEgg{egg_id: 177}),
        ("12-6-0".to_string(), EggType::SingleEgg{egg_id: 177}),
        ("14-1-1".to_string(), EggType::SingleEgg{egg_id: 182}),
        ("14-8-2".to_string(), EggType::SingleEgg{egg_id: 182}),
        ("17-8-0".to_string(), EggType::SingleEgg{egg_id: 200}),
        ("15-4-0".to_string(), EggType::SingleEgg{egg_id: 201}),
        ("17-10-1".to_string(), EggType::SingleEgg{egg_id: 200}),
        ("18-1-1".to_string(), EggType::SingleEgg{egg_id: 186}),
        ("18-3-1".to_string(), EggType::SingleEgg{egg_id: 187}),
        ("19-2-1".to_string(), EggType::SingleEgg{egg_id: 202}),
        ("20-1-0".to_string(), EggType::SingleEgg{egg_id: 202}),
        ("20-4-0".to_string(), EggType::SingleEgg{egg_id: 202}),
        ("25-0-0".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-0-1".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-0-2".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-0-3".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-0-4".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-1-0".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-1-1".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-1-2".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-1-3".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-1-4".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-2-0".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-2-1".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-2-2".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-2-3".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-2-4".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-3-0".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-3-1".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-3-2".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-3-3".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("25-3-4".to_string(), EggType::SingleEgg{egg_id: 203}),
        ("23-6-1".to_string(), EggType::SingleEgg{egg_id: 204}),
        ("23-6-0".to_string(), EggType::SingleEgg{egg_id: 178}),
        ("23-22-0".to_string(), EggType::SingleEgg{egg_id: 178}),
        ("23-8-0".to_string(), EggType::SingleEgg{egg_id: 204}),
        ("1-10-3".to_string(), EggType::SingleEgg{egg_id: 197}),
        ("6-3-0".to_string(), EggType::SingleEgg{egg_id: 206}),
        ("1-7-1".to_string(), EggType::SingleEgg{egg_id: 210}),
        ("6-0-0".to_string(), EggType::SingleEgg{egg_id: 211}),
        ("23-14-1".to_string(), EggType::SingleEgg{egg_id: 204}),
        ("3-1-0".to_string(), EggType::SingleEgg{egg_id: 174}),
        ("3-3-0".to_string(), EggType::SingleEgg{egg_id: 190}),
        ("10-6-0".to_string(), EggType::SingleEgg{egg_id: 215}),
        ("2-8-2".to_string(), EggType::SingleEgg{egg_id: 216}),
        ("16-3-1".to_string(), EggType::SingleEgg{egg_id: 183}),
        ("5-0-1".to_string(), EggType::SingleEgg{egg_id: 205}),
        ("5-3-2".to_string(), EggType::SingleEgg{egg_id: 190}),
        ("13-5-0".to_string(), EggType::SingleEgg{egg_id: 206}),
        ("13-0-1".to_string(), EggType::SingleEgg{egg_id: 209}),
        ("0-4-1".to_string(), EggType::SingleEgg{egg_id: 215}),
        ("14-2-1".to_string(), EggType::SingleEgg{egg_id: 182}),
        ("7-15-1".to_string(), EggType::SingleEgg{egg_id: 182})
    ]);
}

pub fn game_loop() {
    let application = get_application();
    let app_addresses = application.app_addresses();
        
    let game_init: &mut u32 = application.read_address(app_addresses.game_init_address);
    if *game_init != 0 {
        let global_flags: &[u8;4096] = application.read_address(app_addresses.global_flags_address);
        let egg_count = global_flags[0xadd];

        let script_header: &*const ScriptHeader = application.read_address(app_addresses.script_header_pointer_address);
        let inventory_descriptions_card = unsafe { (*script_header.add(2)).data as *mut ScriptSubHeader};
        let waterproof_case_line = unsafe { &mut *inventory_descriptions_card.add(36) };

        let eggs_found_text = format!("Easter Egg：You've found {egg_count} so far");
        let space_count = eggs_found_text.chars().filter(|c| *c == ' ').count();
        let mut encoded_eggs_found_text = screenplay::encode(eggs_found_text);
        encoded_eggs_found_text.push(0x000a);
        *waterproof_case_line = ScriptSubHeader {
            pointer: encoded_eggs_found_text.as_ptr() as usize,
            data_num: encoded_eggs_found_text.len() as i32,
            font_num: (encoded_eggs_found_text.len() - space_count - 1) as i32
        };
    }
    application.original_game_loop()
}

pub fn popup_dialog_draw_intercept(popup_dialog: &'static mut TaskData) {
    let application = get_application();
    let app_addresses = application.app_addresses();

    let global_flags: &[u8;4096] = application.read_address(app_addresses.global_flags_address);

    let item_id = popup_dialog.sbuff[0];
    let zone: &mut u8 = application.read_address(app_addresses.current_zone_address);
    let room: &mut u8 = application.read_address(app_addresses.current_room_address);
    let screen: &mut u8 = application.read_address(app_addresses.current_screen_address);

    let room_key = format!("{zone}-{room}-{screen}");

    if item_id == 36 {
        let egg_id_option = EGG_LOOKUP.get(&room_key);
        match egg_id_option {
            Some(EggType::SingleEgg{egg_id }) => {
                debug!("{} - SingleEgg. Displaying ID {}", room_key, egg_id);
                popup_dialog.sbuff[0] = *egg_id;
            },
            Some(EggType::MultiEgg{flag, egg_ids }) => {
                let flag_value = global_flags[*flag];
                match egg_ids.get((flag_value-1) as usize) {
                    Some(egg_id) => {
                        debug!("{} - MultiEgg for flag {} with value {}. Displaying ID {}", room_key, flag, flag_value, egg_id);
                        popup_dialog.sbuff[0] = *egg_id;
                    },
                    None => {
                        debug!("{} - MultiEgg but no matching index for flag {} with value {}. Default Egg fallback", room_key, flag, flag_value);
                    }
                }
                
            },
            None => {
                debug!("{} - No Egg matching room, Default Egg fallback", room_key);
            }
        }
    }
    application.popup_dialog_draw(popup_dialog);
}

pub fn get_time() -> DWORD {
    unsafe { timeGetTime() }
}
