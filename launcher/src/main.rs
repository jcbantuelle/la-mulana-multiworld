use std::process;
use dll_syringe::{Syringe, process::OwnedProcess};

const LAMULANA_EXECUTABLE_NAME: &str = "LaMulanaWin";
const LAMULANA_MW_DLL_NAME: &str = "LaMulanaMW.dll";

fn main() {
    match process::Command::new(LAMULANA_EXECUTABLE_NAME).spawn() {
        Ok(mut p) => {
            let target_process = OwnedProcess::find_first_by_name(LAMULANA_EXECUTABLE_NAME).unwrap();
            let syringe = Syringe::for_process(target_process);

            println!("Injecting into process...");
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
