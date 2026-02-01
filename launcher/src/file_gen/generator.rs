use std::fmt::Error;
use super::sav;

use crate::archipelago::api::SlotData;

pub fn generate_files(slot_data: SlotData) -> Result<(), Error>{
    // let rcd_bytes = rcd::generate(&slot_data);
    // let dat_bytes = dat::generate(&slot_data);
    let sav_bytes = sav::generate(&slot_data);
    // let effect_bytes = effects::generate();

    // Write files to disk
    Ok(())
}
