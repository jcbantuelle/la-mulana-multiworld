use dirs_next;
use sha2::{Sha256, Digest};
use sysinfo::System;

use crate::ap_data::LaMulanaConfig;
use crate::consts::*;
use crate::file_utils;

pub fn verify_install() -> Result<LaMulanaConfig, String> {
    let lm_config = verify_exe()?;
    if lm_config.version == "1.6.6.2" {
        verify_steam()?;
    }
    verify_game_files(lm_config.clone())?;
    return Ok(lm_config.clone());
}

fn verify_exe() -> Result<LaMulanaConfig, String> {
    // Confirm that the LM executable actually exists (mostly protection against the launcher being run from the wrong location)
    let _ = file_utils::path_exists(&LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION, true)?;

    // Confirm that the LM executable is an unmodded copy of a supported version
    let lm_file = file_utils::read_file(&LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)?;
    let lm_digest = hex::encode(Sha256::digest(lm_file)).to_uppercase();

    fetch_config(lm_digest)
}

// If it's a Steam version of the game, ensure that Steam is running and create steam_appid.txt if it doesn't exist
fn verify_steam() -> Result<(), String> {
    let steam_app_id_path = "steam_appid.txt";

    // Without the correct app id in a file in the root install, Steam will fail to launch the game
    if !file_utils::path_exists(steam_app_id_path, false)? {
        let _ = file_utils::write_file(steam_app_id_path, "230700")?;
    }

    // If Steam isn't running, the game will fail to launch
    let mut sys = System::new_all();
    sys.refresh_all();
    let steam_running = sys.processes().iter().any(|(_, process)| {
        let process_name = process.name().to_ascii_lowercase();
        process_name == "steam.exe" || process_name == "steam"
    });

    if steam_running {
        Ok(())
    } else {
        Err("Steam does not appear to be running, which is a requirement for your version of the game to run. Please launch Steam and then re-run this launcher".to_string())
    }
}

fn verify_game_files(lm_config: LaMulanaConfig) -> Result<(), String> {
    // Confirm the Save directory is discoverable
    let _ = file_utils::path_exists(lm_config.save_path.as_str(), true)?;

    let seeds_dir_exists = file_utils::path_exists(SOURCE_FILES_PATH.as_str(), false)?;
    let files_to_verify = if seeds_dir_exists {
        [
            verify_lm_file(SOURCE_RCD_PATH.to_string(), lm_config.rcd_digest),
            verify_lm_file(SOURCE_DAT_PATH.to_string(), lm_config.dat_digest),
            verify_lm_file(SOURCE_EFFECTS_PATH.to_string(), lm_config.effects_digest)
        ]
    } else {
        [
            verify_lm_file(ORIGINAL_RCD_PATH.to_string(), lm_config.rcd_digest),
            verify_lm_file(ORIGINAL_DAT_PATH.to_string(), lm_config.dat_digest),
            verify_lm_file(ORIGINAL_EFFECTS_PATH.to_string(), lm_config.effects_digest)
        ]
    };

    let invalid_files = files_to_verify.iter().filter_map(|verification| {
        match verification {
            Ok(_) => None,
            Err(error_message) => Some(error_message.clone())
        }
    }).collect::<Vec<String>>();

    if !invalid_files.is_empty() {
        return Err(invalid_files.join("\n"))
    }

    if !seeds_dir_exists {
        let _ = file_utils::create_dir(&SOURCE_FILES_PATH)?;
        let _ = file_utils::create_dir(&format!("{}save/", &SOURCE_FILES_PATH.to_string()))?;
        let _ = file_utils::copy_file(ORIGINAL_RCD_PATH, &SOURCE_RCD_PATH)?;
        let _ = file_utils::copy_file(ORIGINAL_DAT_PATH, &SOURCE_DAT_PATH)?;
        let _ = file_utils::copy_file(ORIGINAL_EFFECTS_PATH, &SOURCE_EFFECTS_PATH)?;

        let save_destination = format!("{}save/", SOURCE_FILES_PATH.to_string());
        file_utils::move_saves(lm_config.save_path, save_destination)?;
    }

    Ok(())
}

fn verify_lm_file(file_path: String, digest: String) -> Result<(), String> {
    match file_utils::read_file(file_path.as_str()) {
        Ok(lm_file) => {
            let lm_digest = hex::encode(Sha256::digest(lm_file)).to_uppercase();
            if lm_digest == digest {
                Ok(())
            } else {
                Err(format!("{} appears to be modified from it's original format. Please restore it to the version from the base game", file_path))
            }
        },
        Err(_) => { Err(format!("Unable to locate {}", file_path)) }
    }
}

fn fetch_config(digest: String) -> Result<LaMulanaConfig, String> {
    if digest == "390E26B6A0C1F14BCAC521D2F8E410C4DEAD0B3E2693B2192BD6CA7832CB5B17" {
        Ok(LaMulanaConfig {
            version: "1.0.0.1".to_string(),
            save_path: "save/".to_string(),
            rcd_digest: "87437780618A3ABDE22BC7200B793FB900169E4F018D1F37D323AC6B5B2F120E".to_string(),
            dat_digest: "E9F34854D82EBA1E72DD80C573DB1202AA15524FAAC3FC82C8D9F9943BD9F31C".to_string(),
            effects_digest: "7CB3D2755ECE2E90BC88A81BCEA6C05350E4695182C3798F86F967A5D4BAC466".to_string()
        })
    } else {
        let documents_dir = match dirs_next::document_dir() {
            Some(doc_path) => doc_path.into_os_string().into_string().unwrap_or(String::new()),
            None => String::new()
        };
        let save_path = format!("{}/nigoro/la-mulana/save/", documents_dir);

        if digest == "E4B5EBE57017C5838DAB44D51C6330902B4FB333AD4714C0E8C8BD37FD354BC8" {
            Ok(LaMulanaConfig {
                version: "1.6.6.1".to_string(),
                save_path: save_path,
                rcd_digest: "583DCE2B2BB41E7A1927C6052F7A6AEFEE3F021A792E1AC587E2103C8B5D4CAC".to_string(),
                dat_digest: "89A2AA21E2CB2DAD6DB5F2EEA474903927980384DE4BC868A9494B1DA3DFED2B".to_string(),
                effects_digest: "7CB3D2755ECE2E90BC88A81BCEA6C05350E4695182C3798F86F967A5D4BAC466".to_string()
            })
        } else if digest == "94228016FFFF8A0BA6325140F0CFF6896E2BD0579BB2099D234508DEDE65923F" {
            Ok(LaMulanaConfig {
                version: "1.6.6.2".to_string(),
                save_path: save_path,
                rcd_digest: "583DCE2B2BB41E7A1927C6052F7A6AEFEE3F021A792E1AC587E2103C8B5D4CAC".to_string(),
                dat_digest: "89A2AA21E2CB2DAD6DB5F2EEA474903927980384DE4BC868A9494B1DA3DFED2B".to_string(),
                effects_digest: "7CB3D2755ECE2E90BC88A81BCEA6C05350E4695182C3798F86F967A5D4BAC466".to_string()
            })
        } else {
            Err(format!("Your version of {} appears to be an unsupported version or modded. Please ensure it's an unaltered copy and that it's either version 1.0.0.1, 1.6.6.1, or 1.6.6.2", *LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION))
        }
    }
}
