use thiserror::Error;

use crate::archipelago::api::SlotData;
use crate::consts::AP_PATH;
use crate::file_gen::app_config::AppConfig;
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

pub fn generate_files(mut app_config: AppConfig, slot_data: SlotData) -> Result<(), FileGenerationError>{
    // let rcd_bytes = rcd::generate(&slot_data);

    let mut dat_file = Dat::new()?;
    dat_file.apply_mods()?;

    let mut sav_file = Sav::new();
    sav_file.apply_mods(&slot_data)?;

    // let effect_bytes = effects::generate();

    let fallback_item_id: i64 = 83;
    for slot_data_location in slot_data.locations.iter() {
        let location = slot_data_location.clone();
        if location.item.is_none() || location.address.is_none() {
            continue;
        }
        let ap_item = location.item.as_ref().unwrap();
        let for_self = ap_item.player == app_config.local_player_id as u64;

        let lm_item = slot_data.item_table.get(&ap_item.name);
        if lm_item.is_none() && for_self {
            continue
        }

        let item_id = if lm_item.is_some() && for_self {
            lm_item.unwrap().game_code
        } else {
            fallback_item_id
        };

        let item_flag = app_config.add_item(lm_item, item_id, &location);

        if location.file_type == "dat" {
            dat_file.place_item(item_id, &location, item_flag);
        }
    }

    // Write files to disk
    let new_seed_path = format!("{}{}", AP_PATH, slot_data.seed);
    file_utils::create_dir(&new_seed_path).map_err(|_| FileGenerationError::SeedDirWriteFailure)?;

    let dat_file_path = format!("{}/{}", new_seed_path, "script_code.dat");
    file_utils::write_file(&dat_file_path, dat_file.to_bytes()?).map_err(|_| FileGenerationError::DatFileWriteFailure)?;

    let save_file_path = format!("{}/{}", new_seed_path, "lm00.sav");
    file_utils::write_file(&save_file_path, sav_file.to_bytes()?).map_err(|_| FileGenerationError::SaveFileWriteFailure)?;
    Ok(())
}
