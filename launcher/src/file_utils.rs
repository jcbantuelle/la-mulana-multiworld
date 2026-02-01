use std::fs;

use log::debug;

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
