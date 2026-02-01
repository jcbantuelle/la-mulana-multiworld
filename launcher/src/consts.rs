use std::sync::LazyLock;

pub static AP_PATH: &str = "ap/";

pub static LAMULANA_EXECUTABLE_NAME: &str = "LaMulanaWin";
pub static LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION: LazyLock<String> = LazyLock::new(|| { format!("{}.exe", LAMULANA_EXECUTABLE_NAME) });

pub static ORIGINAL_RCD_PATH: &str = "data/mapdata/script.rcd";
pub static ORIGINAL_DAT_PATH: &str = "data/language/en/script_code.dat";
pub static ORIGINAL_EFFECTS_PATH: &str = "data/graphics/00/01effect.png";

pub static SOURCE_FILES_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}source/", AP_PATH) });
pub static SOURCE_RCD_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}script.rcd", SOURCE_FILES_PATH.to_string()) });
pub static SOURCE_DAT_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}script_code.dat", SOURCE_FILES_PATH.to_string()) });
pub static SOURCE_EFFECTS_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}01effect.png", SOURCE_FILES_PATH.to_string()) });
