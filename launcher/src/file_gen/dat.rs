use binrw::helpers::until_eof;
use binrw::io::TakeSeekExt;
use binrw::{BinRead, BinWrite};
use log::debug;
use std::io::Cursor;

use crate::archipelago::api::SlotData;
use crate::consts::SOURCE_DAT_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_utils;

use super::lm_flags::GLOBAL_FLAGS;

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct LaMulanaDat {
    num_cards: u16,
    #[br(count = num_cards)]
    cards: Vec<Card>
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Card {
    len_contents: u16,
    #[br(map_stream = |s| s.take_seek(len_contents as u64), parse_with = until_eof)]
    contents: Vec<Entry>
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Entry {
    header: u16,
    #[br(args(header))]
    contents: EntryContents
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[br(import(header: u16))]
pub enum EntryContents {
    #[br(pre_assert(header == 0x0040))]
    Flag(Flag),
    #[br(pre_assert(header == 0x0041))]
    Flag2(Flag2),
    #[br(pre_assert(header == 0x0042))]
    Item(Item),
    #[br(pre_assert(header == 0x0046))]
    Pose(Pose),
    #[br(pre_assert(header == 0x0047))]
    Mantra(Mantra),
    #[br(pre_assert(header == 0x004a))]
    Color(Color),
    #[br(pre_assert(header == 0x004d))]
    ItemName(ItemName),
    #[br(pre_assert(header == 0x004e))]
    Data(Data),
    #[br(pre_assert(header == 0x004f))]
    Anime(Anime),
    #[br(pre_assert(true))]
    Noop(Noop)
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Flag {
    address: i16,
    value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Flag2 {
    address: i16,
    value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Item {
    value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Pose {
    value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Mantra {
    value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Color {
    red: i16,
    green: i16,
    blue: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct ItemName {
    value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Data {
    num_values: i16,
    #[br(count = num_values)]
    values: Vec<i16>
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Anime {
    value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Noop {}

pub fn generate(slot_data: &SlotData) -> Result<Vec<u8>, FileGenerationError> {
    let raw_file = file_utils::read_file(&SOURCE_DAT_PATH).map_err(|_| FileGenerationError::DatFileReadFailure)?;
    let mut reader = Cursor::new(raw_file);
    let dat_file = LaMulanaDat::read_be(&mut reader).map_err(|_| FileGenerationError::DatFileParseFailure)?;

    let mut writer = Cursor::new(Vec::new());
    let _ = dat_file.write_be(&mut writer).map_err(|_| FileGenerationError::DatFileWriteFailure)?;
    Ok(writer.into_inner())
}
