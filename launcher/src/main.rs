#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod archipelago;

use dll_syringe::{process::OwnedProcess, Syringe};
use log::debug;
use sha2::{Sha256, Digest};
use slint::ComponentHandle;
use std::collections::HashMap;
use std::ffi::OsString;
use std::{env, fs, process};
use std::error::Error;
use std::sync::LazyLock;
use sysinfo::System;

use crate::archipelago::api::*;
use crate::archipelago::client::APClient;

#[derive(Clone, Debug)]
struct LaMulanaConfig {
    pub version: String,
    pub save_path: String,
    pub rcd_digest: String,
    pub dat_digest: String,
    pub effects_digest: String,
}

#[derive(Clone, Debug)]
struct LaMulanaFileVerification {
    pub file_path: String,
    pub digest: String
}

impl LaMulanaFileVerification {
    pub fn verify(&self) -> Result<(), String> {
        match fs::read(self.file_path.as_str()) {
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

static LAMULANA_EXECUTABLE_NAME: &str = "LaMulanaWin";
static LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION: LazyLock<String> = LazyLock::new(|| { format!("{}.exe", LAMULANA_EXECUTABLE_NAME) });
static LAMULANA_MW_DLL_NAME: &str = "LaMulanaMW.dll";
static SEEDS_PATH: &str = "seeds/";
static STEAM_APP_ID_PATH: &str = "steam_appid";
static VANILLA_SEED_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}vanilla/", SEEDS_PATH) });
static BASE_RCD_PATH: &str = "data/mapdata/script.rcd";
static BASE_DAT_PATH: &str = "data/language/en/script_code.dat";
static BASE_EFFECTS_PATH: &str = "data/graphics/00/01effect.png";
static VALID_EXE_DIGESTS: LazyLock<HashMap<String, LaMulanaConfig>> = LazyLock::new(|| {
    let home_dir = env::var_os("USERPROFILE").unwrap_or(OsString::new()).into_string().unwrap_or(String::new());

    HashMap::from([
        ("390E26B6A0C1F14BCAC521D2F8E410C4DEAD0B3E2693B2192BD6CA7832CB5B17".to_string(), LaMulanaConfig {
            version: "1.0.0.1".to_string(),
            save_path: "save/".to_string(),
            rcd_digest: "87437780618A3ABDE22BC7200B793FB900169E4F018D1F37D323AC6B5B2F120E".to_string(),
            dat_digest: "E9F34854D82EBA1E72DD80C573DB1202AA15524FAAC3FC82C8D9F9943BD9F31C".to_string(),
            effects_digest: "7CB3D2755ECE2E90BC88A81BCEA6C05350E4695182C3798F86F967A5D4BAC466".to_string()
        }),
        ("94228016FFFF8A0BA6325140F0CFF6896E2BD0579BB2099D234508DEDE65923F".to_string(), LaMulanaConfig {
            version: "1.6.6.2".to_string(),
            save_path: format!("{}/Documents/nigoro/la-mulana/save/", home_dir),
            rcd_digest: "583DCE2B2BB41E7A1927C6052F7A6AEFEE3F021A792E1AC587E2103C8B5D4CAC".to_string(),
            dat_digest: "89A2AA21E2CB2DAD6DB5F2EEA474903927980384DE4BC868A9494B1DA3DFED2B".to_string(),
            effects_digest: "7CB3D2755ECE2E90BC88A81BCEA6C05350E4695182C3798F86F967A5D4BAC466".to_string()
        })
    ])
});

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    match verify_install() {
        Ok(_) => {
            // If no seeds/ap.json file
            //     generate AP struct and serialize to seeds/ap.json
            // Else
            //     Deserialize seeds/ap.json to AP struct

            let launcher = Launcher::new().unwrap();
            let launcher_handle = launcher.as_weak();

            launcher.on_launch_game(move || {
                let _ = slint::spawn_local(async move {
                    let _ = tokio::spawn(async move { launch_game().await }).await.unwrap();
                });
            });

            launcher.on_connect_to_archipelago(move || {
                let _ = slint::spawn_local(async move {
                    let _ = tokio::spawn(async move { connect_to_archipelago().await }).await.unwrap();
                });
            });

            launcher.on_close(move || {
                let launcher = launcher_handle.unwrap();
                let _ = launcher.hide();
            });

            launcher.run()?;
        },
        Err(error_message) => {
            let error_message_window = ErrorMessage::new()?;
            error_message_window.set_error_message(error_message.into());
            let error_message_window_handle = error_message_window.as_weak();

            error_message_window.on_close(move || {
                let error_message_window = error_message_window_handle.unwrap();
                let _ = error_message_window.hide();
            });

            error_message_window.run()?;
        }
    }

    Ok(())
}

fn verify_install() -> Result<(), String> {
    let lm_config = verify_exe()?;
    if lm_config.version == "1.6.6.2" {
        verify_steam()?;
    }
    verify_game_files(lm_config)?;
    return Ok(());
}

fn verify_exe() -> Result<LaMulanaConfig, String> {
    // Confirm that the LM executable actually exists (mostly protection against the launcher being run from the wrong location)
    let _ = path_exists(LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION.as_str(), true)?;

    // Confirm that the LM executable is an unmodded copy of a supported version
    let lm_file = read_file(LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION.as_str())?;
    let lm_digest = hex::encode(Sha256::digest(lm_file)).to_uppercase();

    match VALID_EXE_DIGESTS.get(&lm_digest) {
        Some(lm_config) => { Ok(lm_config.clone()) },
        None => { Err(format!("Your version of {} appears to be an unsupported version or modded. Please ensure it's an unaltered copy and that it's either version 1.0.0.1 or 1.6.6.2", *LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)) }
    }
}

// If it's a Steam version of the game, ensure that Steam is running and create steam_appid.txt if it doesn't exist
fn verify_steam() -> Result<(), String> {
    // Without the correct app id in a file in the root install, Steam will fail to launch the game
    if !path_exists(STEAM_APP_ID_PATH, false)? {
        let _ = write_file(STEAM_APP_ID_PATH, "230700")?;
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

fn verify_game_files(lm_config: LaMulanaConfig) -> Result<(), String> {
    // Confirm the Save directory is discoverable
    let _ = path_exists(lm_config.save_path.as_str(), true)?;

    let seeds_dir_exists = path_exists(VANILLA_SEED_PATH.as_str(), false)?;
    let files_to_verify = if seeds_dir_exists {
        [
            LaMulanaFileVerification { file_path: format!("{}script.rcd", VANILLA_SEED_PATH.to_string()), digest: lm_config.rcd_digest },
            LaMulanaFileVerification { file_path: format!("{}script_code.dat", VANILLA_SEED_PATH.to_string()), digest: lm_config.dat_digest },
            LaMulanaFileVerification { file_path: format!("{}effects01.png", VANILLA_SEED_PATH.to_string()), digest: lm_config.effects_digest}
        ]
    } else {
        [
            LaMulanaFileVerification { file_path: BASE_RCD_PATH.to_string(), digest: lm_config.rcd_digest },
            LaMulanaFileVerification { file_path: BASE_DAT_PATH.to_string(), digest: lm_config.dat_digest },
            LaMulanaFileVerification { file_path: BASE_EFFECTS_PATH.to_string(), digest: lm_config.effects_digest}
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

     Ok(())
}

fn path_exists(file_path: &str, error_if_missing: bool) -> Result<bool, String> {
    match fs::exists(file_path) {
        Ok(exists) => {
            if error_if_missing {
                if exists {
                    Ok(exists)
                } else {
                    Err(format!("{} does not appear to exist. Please make sure the launcher is in the base of your La-Mulana install and that the file structure is correct.", file_path))
                }
            } else {
                Ok(exists)
            }
        },
        Err(e) => Err(format!("File system error {} attempting to check if {} exists, please correct and try again.", e, file_path))
    }
}

fn read_file(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).or_else(|e| {
        Err(format!("File system error {} attempting to read {} exists, please correct and try again.", e, file_path))
    })
}

fn write_file(file_path: &str, file_contents: &str) -> Result<(), String> {
    fs::write(file_path, file_contents).or_else(|e| {
        Err(format!("File system error {} attempting to write {} exists, please correct and try again.", e, file_path))
    })
}

async fn launch_game() {
    match process::Command::new(LAMULANA_EXECUTABLE_NAME).spawn() {
        Ok(mut p) => {
            let process_id = p.id();
            let target_process = OwnedProcess::from_pid(process_id).unwrap();
            let syringe = Syringe::for_process(target_process);

            println!("Injecting into {} of PID {} with {}.", LAMULANA_EXECUTABLE_NAME, process_id, LAMULANA_MW_DLL_NAME);
            match syringe.inject(LAMULANA_MW_DLL_NAME) {
                Ok(_) => {
                    println!("Injected and now waiting on process exit.");
                    p.wait().unwrap();
                },
                Err(e) => println!("Could not inject: {}", e)
            }
        },
        Err(e) => {
            println!("Could not launch LaMulanaWin: {:?}", e)
        }
    }
}

async fn connect_to_archipelago() {
    let mut randomizer = APClient::new("localhost:6969").await;
    match randomizer.as_mut() {
        Ok(ap_client) => {
            let player_id = 1;
            let player_name = "Justin";
            let password = "";
            match ap_client.connect(password, "La-Mulana", &player_name, player_id, ItemHandling::OtherWorldsOnly, vec![], false).await {
                Ok(_) => {},
                Err(e) => {
                    debug!("Connect Failure with error {:?}", e);
                }
            }
        },
        Err(e) => {
            debug!("AP Client Not Connected with Error {}", e);
        }
    };
}
