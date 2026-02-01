use thiserror::Error;

use crate::archipelago::api::SlotData;
use crate::consts::AP_PATH;
use crate::file_utils;

use super::sav;

#[derive(Clone, Error, Debug)]
pub enum FileGenerationError {
    #[error("Invalid Starting Weapon option from Archipelago")]
    InvalidStartingWeapon,
    #[error("Failed to write Seed Directory")]
    SeedDirWriteFailure,
    #[error("Failed to apply Mods to Save File")]
    SaveFileModFailure,
    #[error("Failed to write Save File")]
    SaveFileWriteFailure
}

pub fn generate_files(slot_data: SlotData) -> Result<(), FileGenerationError>{
    // let rcd_bytes = rcd::generate(&slot_data);
    // let dat_bytes = dat::generate(&slot_data);
    let sav_bytes = sav::generate(&slot_data)?;
    // let effect_bytes = effects::generate();

    // Write files to disk
    let new_seed_path = format!("{}{}", AP_PATH, slot_data.seed);
    file_utils::create_dir(&new_seed_path).map_err(|_| FileGenerationError::SeedDirWriteFailure)?;

    let save_file_path = format!("{}/{}", new_seed_path, "lm00.sav");
    file_utils::write_file(&save_file_path, sav_bytes).map_err(|_| FileGenerationError::SaveFileWriteFailure)?;
    Ok(())
}
