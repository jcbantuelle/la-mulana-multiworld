use winapi::shared::minwindef::*;
use winapi::um::timeapi::timeGetTime;

use crate::get_application;
use crate::application::ApplicationMemoryOps;
use crate::lm_structs::taskdata::TaskData;
use crate::lm_structs::script_header::{ScriptHeader, ScriptSubHeader};
use crate::screenplay;

pub type FnGameLoop = extern "C" fn();
pub type FnPopupDialogDrawIntercept = extern "C" fn(&mut TaskData);

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

        let eggs_found_text = format!("Easter Egg：(Find as many as you can!) You've found {egg_count} so far");
        let mut encoded_eggs_found_text = screenplay::encode(eggs_found_text);
        encoded_eggs_found_text.push(0x000a);
        *waterproof_case_line = ScriptSubHeader {
            pointer: encoded_eggs_found_text.as_ptr() as usize,
            data_num: encoded_eggs_found_text.len() as i32,
            font_num: (encoded_eggs_found_text.len() - 12) as i32
        };
    }
    application.original_game_loop()
}

pub fn popup_dialog_draw_intercept(popup_dialog: &'static mut TaskData) {
    let application = get_application();

    // let item_id = popup_dialog.sbuff[0];
    // let field = popup_dialog.field_no;
    // let screen = popup_dialog.room_no;
    // let room = popup_dialog.view_no;

    // // If item is Waterproof Case
    // if item_id == 36 {
    //     // If Surface Skeleton Scan room
    //     if field == 1 && screen == 4 && room == 2 {
    //         // Replace item image with corresponding graphic from 01menu / text from card 1
    //         popup_dialog.sbuff[0] = 60;
    //     }
    // }
    application.popup_dialog_draw(popup_dialog);
}

pub fn get_time() -> DWORD {
    unsafe { timeGetTime() }
}
