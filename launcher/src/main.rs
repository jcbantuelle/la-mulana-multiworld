#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod archipelago;

use dll_syringe::{process::OwnedProcess, Syringe};
use log::debug;
use sha2::{Sha256, Digest};
use slint::ComponentHandle;
use std::{fs, process};
use std::error::Error;
use std::path::Path;
use sysinfo::System;

use crate::archipelago::api::*;
use crate::archipelago::client::APClient;

const LAMULANA_EXECUTABLE_NAME: &str = "LaMulanaWin";
const LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION: &str = "LaMulanaWin.exe";
const LAMULANA_MW_DLL_NAME: &str = "LaMulanaMW.dll";
const VALID_EXE_DIGESTS: [&str;2] = [
    "94228016FFFF8A0BA6325140F0CFF6896E2BD0579BB2099D234508DEDE65923F", // 1.6.6.2 (Steam)
    "390E26B6A0C1F14BCAC521D2F8E410C4DEAD0B3E2693B2192BD6CA7832CB5B17" // 1.0.0.1
];

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut error_message: String = "".to_string();

    // Confirm that the LM executable actually exists (mostly protection against the launcher being run from the wrong location)
    let lm_exe_exists = fs::exists(LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)?;
    if lm_exe_exists {

        // Confirm that the LM executable is an unmodded copy of a supported version
        let lm_file = fs::read(LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)?;
        let lm_digest = hex::encode(Sha256::digest(lm_file)).to_uppercase();
        let valid_lm_digest = VALID_EXE_DIGESTS.contains(&lm_digest.as_str());
        if valid_lm_digest {

            // If it's a Steam version of the game, ensure that Steam is running and create steam_appid.txt if it doesn't existexists
            let valid_version_specific_configuration = if lm_digest == VALID_EXE_DIGESTS[0] {
                // Without the correct app id in a file in the root install, Steam will fail to launch the game
                if !Path::new("steam_appid.txt").exists() {
                    let app_id = "230700";
                    fs::write("steam_appid.txt", app_id).expect("Failed to generate steam_appid.txt, check permissions");
                }

                // If Steam isn't running, the game will fail to launch
                let mut sys = System::new_all();
                sys.refresh_all();
                sys.processes().iter().any(|(_, process)| {
                    process.name().to_ascii_lowercase() == "steam.exe"
                })
            } else { true };

            if valid_version_specific_configuration {
                // Verify location of save directory

                // If there is no seeds directory or no seeds/vanilla directory:
                //     verify script.rcd, script_code.dat, and 01effect.png in original locations are unmodified for matching version
                //     Generate seeds/vanilla and copy clean files and save directory
                // Else
                //     verify script.rcd, script_code.dat, and 01effect.png in seeds/vanilla are unmodified for matching version

                // If no seeds/ap.json file
                //     generate AP struct and serialize to seeds/ap.json
                // Else
                //     Deserialize seeds/ap.json to AP struct
                {};
            } else {
                error_message = "Steam does not appear to be running, which is a requirement for your version of the game to run. Please launch Steam and then re-run this launcher".to_string()
            }
        } else {
            error_message = format!("Your version of {} appears to be an unsupported version or modded. Please ensure it's an unaltered copy and that it's either version 1.0.0.1 or 1.6.6.2", LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION);
        }
    } else {
        error_message = format!("{} could not be found in the folder where this launcher is running. Please make sure you are running it from your La-Mulana install directory in the same place as {}", LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION, LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION);
    }

    if !error_message.is_empty() {
        let error_message_window = ErrorMessage::new()?;
        error_message_window.set_error_message(error_message.into());
        let error_message_window_handle = error_message_window.as_weak();

        error_message_window.on_close(move || {
            let error_message_window = error_message_window_handle.unwrap();
            let _ = error_message_window.hide();
        });

        error_message_window.run()?;

        return Ok(());
    }

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

    Ok(())
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
