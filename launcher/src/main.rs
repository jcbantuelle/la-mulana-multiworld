use std::fs;
use std::process;
use std::path::Path;
use dll_syringe::{Syringe, process::OwnedProcess};

const LAMULANA_EXECUTABLE_NAME: &str = "LaMulanaWin";
const LAMULANA_MW_DLL_NAME: &str = "LaMulanaMW.dll";

fn main() {
    if !Path::new("steam_appid.txt").exists() {
        let app_id = "230700";
        fs::write("steam_appid.txt", app_id).expect("Failed to generate steam_appid.txt, check permissions");
    }

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
