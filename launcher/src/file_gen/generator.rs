use thiserror::Error;

use crate::archipelago::api::{ItemData, SlotData};
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
    #[error("Archipelago Slot Data was malformed, please check software versions")]
    MalformedSlotData,
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

impl Default for ItemData {
    fn default() -> Self {
        ItemData {
            category: "Unknown".to_string(),
            code: 0,
            progression: false,
            useful: false,
            trap: false,
            number: 0,
            game_code: 0,
            cost: Some(10),
            quantity: 1,
            obtain_flag: None,
            obtain_value: None
        }
    }
}

pub fn generate_files(mut app_config: AppConfig, slot_data: SlotData) -> Result<(), FileGenerationError>{
    // let rcd_bytes = rcd::generate(&slot_data);

    let mut dat_file = Dat::new()?;
    dat_file.apply_mods()?;

    let mut sav_file = Sav::new();
    sav_file.apply_mods(&slot_data)?;

    // let effect_bytes = effects::generate();

    for slot_data_location in slot_data.locations.iter() {
        match &slot_data_location.address {
            None => continue,
            _ => ()
        }

        let ap_item = match &slot_data_location.item {
            Some(item) => item,
            None => continue
        };

        let lm_item = match slot_data.item_table.get(&ap_item.name) {
            Some(item) => item.clone(),
            None => {
                if ap_item.player == app_config.local_player_id {
                    continue
                }
                Default::default()
            }
        };

        let item_id = if lm_item.game_code == 0 { 83 } else { lm_item.game_code };
        let item_flag = app_config.add_item(lm_item, item_id, &slot_data_location)?;

        match &slot_data_location.file_type {
            Some(file_type) => {
                if file_type == "dat" {
                    dat_file.place_item(item_id, &slot_data_location, item_flag);
                }
            },
            None => ()
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
