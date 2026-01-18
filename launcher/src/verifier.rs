use sha2::{Sha256, Digest};
use sysinfo::System;

use crate::consts::*;
use crate::file_utils;

#[derive(Clone, Debug)]
struct LaMulanaFileVerification {
    pub file_path: String,
    pub digest: &'static str
}

impl LaMulanaFileVerification {
    pub fn verify(&self) -> Result<(), String> {
        match file_utils::read_file(&self.file_path) {
            Ok(lm_file) => {
                let lm_digest = hex::encode(Sha256::digest(lm_file)).to_uppercase();
                if lm_digest == self.digest {
                    Ok(())
                } else {
                    Err(format!("{} appears to be modified from it's original format. Please restore it to the version from the base game", self.file_path))
                }
            },
            Err(_) => { Err(format!("Unable to locate {}", self.file_path)) }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Verifier {
}

impl Verifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn verify_install(&self) -> Result<(), String> {
        let lm_config = self.verify_exe()?;
        if lm_config.version == "1.6.6.2" {
            self.verify_steam()?;
        }
        self.verify_game_files(lm_config)?;
        return Ok(());
    }

    fn verify_exe(&self) -> Result<LaMulanaConfig, String> {
        // Confirm that the LM executable actually exists (mostly protection against the launcher being run from the wrong location)
        let _ = file_utils::path_exists(&LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION, true)?;

        // Confirm that the LM executable is an unmodded copy of a supported version
        let lm_file = file_utils::read_file(&LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)?;
        let lm_digest = hex::encode(Sha256::digest(lm_file)).to_uppercase();

        match VALID_EXE_DIGESTS.get(&lm_digest) {
            Some(lm_config) => { Ok(lm_config.clone()) },
            None => { Err(format!("Your version of {} appears to be an unsupported version or modded. Please ensure it's an unaltered copy and that it's either version 1.0.0.1 or 1.6.6.2", *LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)) }
        }
    }

    // If it's a Steam version of the game, ensure that Steam is running and create steam_appid.txt if it doesn't exist
    fn verify_steam(&self) -> Result<(), String> {
        // Without the correct app id in a file in the root install, Steam will fail to launch the game
        if !file_utils::path_exists(STEAM_APP_ID_PATH, false)? {
            let _ = file_utils::write_file(STEAM_APP_ID_PATH, "230700")?;
        }

        // If Steam isn't running, the game will fail to launch
        let mut sys = System::new_all();
        sys.refresh_all();
        let steam_running = sys.processes().iter().any(|(_, process)| {
            process.name().to_ascii_lowercase() == "steam.exe"
        });

        if steam_running {
            Ok(())
        } else {
            Err("Steam does not appear to be running, which is a requirement for your version of the game to run. Please launch Steam and then re-run this launcher".to_string())
        }
    }

    fn verify_game_files(&self, lm_config: LaMulanaConfig) -> Result<(), String> {
        // Confirm the Save directory is discoverable
        let _ = file_utils::path_exists(lm_config.save_path.as_str(), true)?;

        let seeds_dir_exists = file_utils::path_exists(SOURCE_FILES_PATH.as_str(), false)?;
        let files_to_verify = if seeds_dir_exists {
            [
                LaMulanaFileVerification { file_path: SOURCE_RCD_PATH.to_string(), digest: lm_config.rcd_digest },
                LaMulanaFileVerification { file_path: SOURCE_DAT_PATH.to_string(), digest: lm_config.dat_digest },
                LaMulanaFileVerification { file_path: SOURCE_EFFECTS_PATH.to_string(), digest: lm_config.effects_digest }
            ]
        } else {
            [
                LaMulanaFileVerification { file_path: ORIGINAL_RCD_PATH.to_string(), digest: lm_config.rcd_digest },
                LaMulanaFileVerification { file_path: ORIGINAL_DAT_PATH.to_string(), digest: lm_config.dat_digest },
                LaMulanaFileVerification { file_path: ORIGINAL_EFFECTS_PATH.to_string(), digest: lm_config.effects_digest }
            ]
        };

        let invalid_files = files_to_verify.iter().map(|file_verification| {
            file_verification.verify()
        }).filter_map(|verification| {
            match verification {
                Ok(_) => None,
                Err(error_message) => Some(error_message)
            }
        }).collect::<Vec<String>>();

        if !invalid_files.is_empty() {
            return Err(invalid_files.join("\n"))
        }

        if !seeds_dir_exists {
            let _ = file_utils::create_dir(&SOURCE_FILES_PATH)?;
            let _ = file_utils::copy_file(ORIGINAL_RCD_PATH, &SOURCE_RCD_PATH)?;
            let _ = file_utils::copy_file(ORIGINAL_DAT_PATH, &SOURCE_DAT_PATH)?;
            let _ = file_utils::copy_file(ORIGINAL_EFFECTS_PATH, &SOURCE_EFFECTS_PATH)?;

            let save_destination = format!("{}save/", SOURCE_FILES_PATH.to_string());
            let _ = file_utils::create_dir(&save_destination)?;
            let save_dir = file_utils::read_dir(&lm_config.save_path)?;
            let save_files = save_dir.filter_map(|save_file| {
                match save_file {
                    Ok(f) => {
                        if f.path().is_file() {
                            let file_name = f.path().file_name().unwrap().to_str().unwrap().to_string();
                            let file_path = f.path().as_os_str().to_str().unwrap().to_string();
                            Some((file_name, file_path))
                        } else {
                            None
                        }
                    },
                    Err(_) => None
                }
            });
            for (save_file_name, save_file_path) in save_files {
                let save_dest = format!("{}{}", save_destination, save_file_name);
                let _ = file_utils::copy_file(&save_file_path, &save_dest);
            }
        }

        Ok(())
    }
}
