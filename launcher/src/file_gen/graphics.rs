use image::{GenericImage, ImageBuffer, ImageFormat, Rgba};
use log::debug;
use std::io::Cursor;
use std::path::Path;

use crate::consts::SOURCE_EFFECTS_PATH;
use crate::file_gen::generator::FileGenerationError;

const CUSTOM_EFFECTS: &[u8] = include_bytes!("../images/01effect-custom.png");

pub fn generate_effects() -> Result<Vec<u8>, FileGenerationError> {
    let original_path = Path::new(SOURCE_EFFECTS_PATH.as_str());
    let original = image::open(original_path).map_err(|e| {
        debug!("Error {} while attempting to open {:?}", e, original_path);
        FileGenerationError::EffectsFileOpenFailure
    })?;

    let custom = image::load_from_memory(CUSTOM_EFFECTS).map_err(|e| {
        debug!("Error {} while attempting to open custom effects file", e);
        FileGenerationError::EffectsFileOpenFailure
    })?;

    let width = original.width();
    let height = original.height() + custom.height();

    let mut generated = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    generated.copy_from(&original, 0, 0).map_err(|e| {
        debug!("Error {} while attempting to copy original 01effect.png into generated image", e);
        FileGenerationError::EffectsFileCopyFailure
    })?;

    generated.copy_from(&custom, 0, original.height()).map_err(|e| {
        debug!("Error {} while attempting to copy custom 01effect.png into generated image", e);
        FileGenerationError::EffectsFileCopyFailure
    })?;

    let mut writer = Cursor::new(Vec::new());
    generated.write_to(&mut writer, ImageFormat::Png).map_err(|_| FileGenerationError::DatFileWriteFailure)?;
    Ok(writer.into_inner())
}
