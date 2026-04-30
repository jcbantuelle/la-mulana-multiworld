use std::fs;

use log::debug;

use crate::consts::{AP_PATH, ORIGINAL_DAT_PATH, ORIGINAL_EFFECTS_PATH, ORIGINAL_RCD_PATH};

pub fn path_exists(file_path: &str, error_if_missing: bool) -> Result<bool, String> {
    match fs::exists(file_path) {
        Ok(exists) => {
            if error_if_missing && !exists {
                let error_message = format!("{} does not appear to exist. Please make sure the launcher is in the base of your La-Mulana install and that the file structure is correct.", file_path);
                debug!("{}", error_message);
                Err(error_message)
            } else {
                Ok(exists)
            }
        },
        Err(e) => Err(format!("File system error {} attempting to check if {} exists, please correct and try again.", e, file_path))
    }
}

pub fn read_file(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).or_else(|e| {
        let error_message = format!("File system error {} attempting to read {}, please correct and try again.", e, file_path);
        debug!("{}", error_message);
        Err(error_message)
    })
}

pub fn read_file_as_string(file_path: &str) -> Result<String, String> {
    fs::read_to_string(file_path).or_else(|e| {
        let error_message = format!("File system error {} attempting to read {}, please correct and try again.", e, file_path);
        debug!("{}", error_message);
        Err(error_message)
    })
}

pub fn write_file<T>(file_path: &str, file_contents: T) -> Result<(), String>
where
    T: AsRef<[u8]>
{
    fs::write(file_path, file_contents).or_else(|e| {
        let error_message = format!("File system error {} attempting to write {}, please correct and try again.", e, file_path);
        debug!("{}", error_message);
        Err(error_message)
    })
}

pub fn create_dir(file_path: &str) -> Result<(), String> {
    fs::create_dir_all(file_path).or_else(|e| {
        let error_message = format!("File system error {} attempting to create {}, please correct and try again.", e, file_path);
        debug!("{}", error_message);
        Err(error_message)
    })
}

pub fn read_dir(file_path: &str) -> Result<fs::ReadDir, String> {
    fs::read_dir(file_path).or_else(|e| {
        let error_message = format!("File system error {} attempting to read {}, please correct and try again.", e, file_path);
        debug!("{}", error_message);
        Err(error_message)
    })
}

pub fn copy_file(file_src: &str, file_dest: &str) -> Result<u64, String> {
    fs::copy(file_src, file_dest).or_else(|e| {
        let error_message = format!("File system error {} attempting to copy {} to {}, please correct and try again.", e, file_src, file_dest);
        debug!("{}", error_message);
        Err(error_message)
    })
}

pub fn move_saves(save_source: String, save_destination: String) -> Result<(), String> {
    let save_dir = read_dir(&save_source)?;
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
        let _ = copy_file(&save_file_path, &save_dest);
    }
    Ok(())
}

pub fn delete_seed(seed: String) -> Result<(), String> {
    let seed_file_path = format!("{}{}/", AP_PATH, seed);
    fs::remove_dir_all(seed_file_path.clone()).or_else(|e| {
        let error_message = format!("File system error {} attempting to delete {}, please correct and try again.", e, seed_file_path.clone());
        debug!("{}", error_message);
        Err(error_message)
    })
}

pub fn update_game_files(seed: String, save_destination: String) -> Result<(), String> {
    let seed_file_path = format!("{}{}/", AP_PATH, seed);

    let rcd_path = format!("{}script.rcd", seed_file_path);
    let dat_path = format!("{}script_code.dat", seed_file_path);
    let effects_path = format!("{}01effect.png", seed_file_path);
    let app_config_path = format!("{}lamulana-config.toml", seed_file_path);

    let _ = copy_file(&rcd_path, ORIGINAL_RCD_PATH)?;
    let _ = copy_file(&dat_path, ORIGINAL_DAT_PATH)?;
    let _ = copy_file(&effects_path, ORIGINAL_EFFECTS_PATH)?;
    let _ = copy_file(&app_config_path, "./lamulana-config.toml")?;

    let save_path = format!("{}save/", seed_file_path);
    move_saves(save_path, save_destination)?;

    Ok(())
}
