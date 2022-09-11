use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null_mut};
use winapi::um::winuser::{MB_OK, MessageBoxW};
use log::debug;

pub static OPTION_SDATA_NUM_ADDRESS: usize = 0x00db6fb7;
pub static OPTION_SDATA_ADDRESS: usize = 0x00db7048;
pub static OPTION_POS_CX_ADDRESS: usize = 0x00db7168;
pub static OPTION_POS_CY_ADDRESS: usize = 0x00db714c;
pub static SET_VIEW_EVENT_NS_ADDRESS: usize = 0x00507160;
pub static ITEM_GET_AREA_INIT_ADDRESS: usize = 0x004b8950;
pub static ITEM_GET_AREA_BACK_ADDRESS: usize = 0x004b8a80;

pub struct Application {
    pub address: *mut *const u8
}

impl Application {
    pub unsafe fn write_address<T>(&self, offset: usize, f: T) {
        self.get_address_from_offset(offset).cast::<T>().write(f);
    }

    pub unsafe fn write_address_with_additional_offset<T>(&self, offset: usize, additional_offset: usize, f: T) {
        self.get_address_from_offset(offset).cast::<T>().add(additional_offset).write(f);
    }

    pub unsafe fn read_address<T>(&self, offset: usize) -> T {
        self.get_address_from_offset(offset).cast::<T>().read()
    }

    pub unsafe fn get_address_from_offset(&self, offset: usize) -> *mut *const u8 {
        let address = self.address.add(offset / 4);
        address
    }

    pub unsafe fn option_stuck(&self, option_num: u32) {
        let s_data_num: u8 = self.read_address(OPTION_SDATA_NUM_ADDRESS);
        if s_data_num < 32 {
            self.write_address_with_additional_offset(OPTION_SDATA_ADDRESS, s_data_num as usize, option_num);
            self.write_address(OPTION_SDATA_NUM_ADDRESS, s_data_num + 1);
        }
    }

    pub unsafe fn option_pos(&self, x: f32, y: f32) {
        self.write_address(OPTION_POS_CX_ADDRESS, x);
        self.write_address(OPTION_POS_CY_ADDRESS, y);
    }

    pub unsafe fn get_sdata_num(&self) -> u8 {
        return self.read_address(OPTION_SDATA_NUM_ADDRESS);
    }
}

fn create_wstring(str : &str) -> Vec<u16> {
    return OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
}

pub unsafe fn show_message_box(message: &str) {
    let converted_message = create_wstring(message).as_ptr();
    MessageBoxW(null_mut(), converted_message, null_mut(), MB_OK);
}
