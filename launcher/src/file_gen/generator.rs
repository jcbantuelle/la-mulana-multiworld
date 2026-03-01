use thiserror::Error;

use crate::archipelago::api::{ItemData, SlotData};
use crate::consts::AP_PATH;
use crate::file_gen::app_config::AppConfig;
use crate::file_gen::dat::Dat;
use crate::file_gen::graphics;
use crate::file_gen::lm_consts::ITEM_CODES;
use crate::file_gen::rcd::Rcd;
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
    #[error("Original Dat File is missing expected data, please report this issue to the devs")]
    MalformedDatFile,
    #[error("Failed to apply Mods to Dat File")]
    DatFileModFailure,
    #[error("Failed to write Dat File")]
    DatFileWriteFailure,
    #[error("Failed to Encode Item data, please report this issue to the devs")]
    FontEncodingError,
    #[error("Failed to read Original Rcd File")]
    RcdFileReadFailure,
    #[error("Failed to parse Original Rcd File")]
    RcdFileParseFailure,
    #[error("Original Rcd File is missing expected data, please report this issue to the devs")]
    MalformedRcdFile,
    #[error("Failed to write Rcd File")]
    RcdFileWriteFailure,
    #[error("Failed to apply Mods to Save File")]
    SaveFileModFailure,
    #[error("Failed to write Save File")]
    SaveFileWriteFailure,
    #[error("Failed to read 01effect.png")]
    EffectsFileOpenFailure,
    #[error("Failed to copy 01effect.png")]
    EffectsFileCopyFailure,
    #[error("Failed to write 01effect.png")]
    EffectsFileWriteFailure
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
    let mut rcd_file = Rcd::new(slot_data.start_inventory.clone(), slot_data.cursed_chests.clone())?;

    let mut dat_file = Dat::new()?;
    dat_file.apply_mods()?;

    let mut sav_file = Sav::new();
    sav_file.apply_mods(&slot_data)?;

    for slot_data_location in slot_data.locations.iter() {
        match &slot_data_location.address {
            None => { continue; },
            _ => ()
        }

        let ap_item = match &slot_data_location.item {
            Some(item) => item,
            None => { continue; }
        };

        let lm_item = match slot_data.item_table.get(&ap_item.name) {
            Some(item) => item.clone(),
            None => {
                if ap_item.player == app_config.local_player_id {
                    continue;
                }
                Default::default()
            }
        };

        let item_id = if lm_item.game_code == 0 { ITEM_CODES["Holy Grail (Full)"] } else { lm_item.game_code };
        let item_flag = app_config.add_item(lm_item.clone(), item_id, &slot_data_location)?;

        match &slot_data_location.file_type {
            Some(file_type) => {
                if file_type == "dat" {
                    match &slot_data_location.slot {
                        Some(slot) => {
                            dat_file.place_shop_item(&slot_data_location, item_id, item_flag, *slot, lm_item.clone(), &slot_data.options)?;
                        },
                        None => {
                            dat_file.place_conversation_item(&slot_data_location, item_id, item_flag)?;
                        }
                    }
                } else if file_type == "rcd" {
                    rcd_file.place_item(&slot_data_location, lm_item.clone(), item_id, item_flag)?;
                }
            },
            None => ()
        }
    }

    dat_file.update_shop_bunemon_text()?;

    let effect_bytes = graphics::generate_effects()?;

    // Write files to disk
    let new_seed_path = format!("{}{}", AP_PATH, slot_data.seed);
    file_utils::create_dir(&new_seed_path).map_err(|_| FileGenerationError::SeedDirWriteFailure)?;

    let rcd_file_path = format!("{}/{}", new_seed_path, "script.rcd");
    file_utils::write_file(&rcd_file_path, rcd_file.to_bytes()?).map_err(|_| FileGenerationError::RcdFileWriteFailure)?;

    let dat_file_path = format!("{}/{}", new_seed_path, "script_code.dat");
    file_utils::write_file(&dat_file_path, dat_file.to_bytes()?).map_err(|_| FileGenerationError::DatFileWriteFailure)?;

    let save_file_path = format!("{}/{}", new_seed_path, "lm00.sav");
    file_utils::write_file(&save_file_path, sav_file.to_bytes()?).map_err(|_| FileGenerationError::SaveFileWriteFailure)?;

    let effects_file_path = format!("{}/{}", new_seed_path, "01effect.png");
    file_utils::write_file(&effects_file_path, effect_bytes).map_err(|_| FileGenerationError::EffectsFileWriteFailure)?;

    Ok(())
}
