use std::collections::HashMap;
use std::env;
use std::sync::{LazyLock, Mutex};

use crate::ap_connection::APData;

pub static AP_DATA: Mutex<APData> = Mutex::new(APData { games: Vec::new(), active_game: None });
pub static AP_DATA_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}ap_data.json", AP_PATH) });
pub static AP_PATH: &str = "ap/";

pub static LAMULANA_EXECUTABLE_NAME: &str = "LaMulanaWin";
pub static LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION: LazyLock<String> = LazyLock::new(|| { format!("{}.exe", LAMULANA_EXECUTABLE_NAME) });
pub static LAMULANA_MW_DLL_NAME: &str = "LaMulanaMW.dll";

pub static ORIGINAL_RCD_PATH: &str = "data/mapdata/script.rcd";
pub static ORIGINAL_DAT_PATH: &str = "data/language/en/script_code.dat";
pub static ORIGINAL_EFFECTS_PATH: &str = "data/graphics/00/01effect.png";

pub static SOURCE_FILES_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}source/", AP_PATH) });
pub static SOURCE_RCD_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}script.rcd", SOURCE_FILES_PATH.to_string()) });
pub static SOURCE_DAT_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}script_code.rcd", SOURCE_FILES_PATH.to_string()) });
pub static SOURCE_EFFECTS_PATH: LazyLock<String> = LazyLock::new(|| { format!("{}01effect.png", SOURCE_FILES_PATH.to_string()) });

pub static STEAM_APP_ID_PATH: &str = "steam_appid";

pub static VALID_EXE_DIGESTS: LazyLock<HashMap<String, LaMulanaConfig>> = LazyLock::new(|| {
    let home_dir = env::var("USERPROFILE").unwrap_or(String::new());

    HashMap::from([
        (
            "390E26B6A0C1F14BCAC521D2F8E410C4DEAD0B3E2693B2192BD6CA7832CB5B17".to_string(),
            LaMulanaConfig {
                version: "1.0.0.1",
                save_path: "save/".to_string(),
                rcd_digest: "87437780618A3ABDE22BC7200B793FB900169E4F018D1F37D323AC6B5B2F120E",
                dat_digest: "E9F34854D82EBA1E72DD80C573DB1202AA15524FAAC3FC82C8D9F9943BD9F31C",
                effects_digest: "7CB3D2755ECE2E90BC88A81BCEA6C05350E4695182C3798F86F967A5D4BAC466"
            }
        ),
        (
            "94228016FFFF8A0BA6325140F0CFF6896E2BD0579BB2099D234508DEDE65923F".to_string(),
            LaMulanaConfig {
                version: "1.6.6.2",
                save_path: format!("{}/Documents/nigoro/la-mulana/save/", home_dir),
                rcd_digest: "583DCE2B2BB41E7A1927C6052F7A6AEFEE3F021A792E1AC587E2103C8B5D4CAC",
                dat_digest: "89A2AA21E2CB2DAD6DB5F2EEA474903927980384DE4BC868A9494B1DA3DFED2B",
                effects_digest: "7CB3D2755ECE2E90BC88A81BCEA6C05350E4695182C3798F86F967A5D4BAC466"
            }
        )
    ])
});

#[derive(Clone, Debug)]
pub struct LaMulanaConfig {
    pub version: &'static str,
    pub save_path: String,
    pub rcd_digest: &'static str,
    pub dat_digest: &'static str,
    pub effects_digest: &'static str,
}





