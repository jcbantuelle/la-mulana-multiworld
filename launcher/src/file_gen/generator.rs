use thiserror::Error;

use crate::archipelago::api::SlotData;
use crate::consts::AP_PATH;
use crate::file_gen::dat::Dat;
use crate::file_gen::sav::Sav;
use crate::file_utils;

#[derive(Clone, Error, Debug)]
pub enum FileGenerationError {
    #[error("Invalid Starting Weapon option from Archipelago")]
    InvalidStartingWeapon,
    #[error("Failed to write Seed Directory")]
    SeedDirWriteFailure,
    #[error("Failed to read Original Dat File")]
    DatFileReadFailure,
    #[error("Failed to parse Original Dat File")]
    DatFileParseFailure,
    #[error("Failed to apply Mods to Dat File")]
    DatFileModFailure,
    #[error("Failed to write Dat File")]
    DatFileWriteFailure,
    #[error("Failed to apply Mods to Save File")]
    SaveFileModFailure,
    #[error("Failed to write Save File")]
    SaveFileWriteFailure
}

pub fn generate_files(slot_data: SlotData) -> Result<(), FileGenerationError>{
    // let rcd_bytes = rcd::generate(&slot_data);

    let mut dat_file = Dat::new()?;
    dat_file.apply_mods()?;

    let mut sav_file = Sav::new();
    sav_file.apply_mods(&slot_data)?;

    // let effect_bytes = effects::generate();

    // Write files to disk
    let new_seed_path = format!("{}{}", AP_PATH, slot_data.seed);
    file_utils::create_dir(&new_seed_path).map_err(|_| FileGenerationError::SeedDirWriteFailure)?;

    let dat_file_path = format!("{}/{}", new_seed_path, "script_code.dat");
    file_utils::write_file(&dat_file_path, dat_file.to_bytes()?).map_err(|_| FileGenerationError::DatFileWriteFailure)?;

    let save_file_path = format!("{}/{}", new_seed_path, "lm00.sav");
    file_utils::write_file(&save_file_path, sav_file.to_bytes()?).map_err(|_| FileGenerationError::SaveFileWriteFailure)?;
    Ok(())
}
