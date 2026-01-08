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
    "94228016FFFF8A0BA6325140F0CFF6896E2BD0579BB2099D234508DEDE65923F",
    "390E26B6A0C1F14BCAC521D2F8E410C4DEAD0B3E2693B2192BD6CA7832CB5B17"
];

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !Path::new("steam_appid.txt").exists() {
        let app_id = "230700";
        fs::write("steam_appid.txt", app_id).expect("Failed to generate steam_appid.txt, check permissions");
    }

    let mut sys = System::new_all();
    sys.refresh_all();
    let steam_running = sys.processes().iter().any(|(_, process)| {
        process.name().to_ascii_lowercase() == "steam.exe"
    });

    let lm_exe_exists = fs::exists(LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)?;

    let lm_file = fs::read(LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION)?;
    let lm_digest = hex::encode(Sha256::digest(lm_file)).to_uppercase();
    let valid_lm_digest = VALID_EXE_DIGESTS.contains(&lm_digest.as_str());

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

    let checker_output = if valid_lm_digest { "Valid EXE Digest" } else { "Invalid EXE Digest" };

    let setup = Setup::new()?;
    setup.set_checker_output(checker_output.into());
    let setup_handle = setup.as_weak();

    let launcher = Launcher::new().unwrap();
    let launcher_handle = launcher.as_weak();

    setup.on_complete_setup(move || {
        let launcher = launcher_handle.unwrap();
        let setup = setup_handle.unwrap();

        let _ = launcher.show();
        let _ = setup.hide();
    });

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

    setup.run()?;

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
