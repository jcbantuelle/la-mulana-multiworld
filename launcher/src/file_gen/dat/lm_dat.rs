use binrw::helpers::until_eof;
use binrw::io::TakeSeekExt;
use binrw::{BinRead, binrw, BinWrite};
use std::io::Cursor;

use crate::file_gen::generator::FileGenerationError;

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct LaMulanaDat {
    pub num_cards: i16,
    #[br(count = num_cards)]
    pub cards: Vec<Card>
}

impl LaMulanaDat {
    pub fn load_file(raw_file: Vec<u8>) -> Result<LaMulanaDat, FileGenerationError> {
        let mut reader = Cursor::new(raw_file);
        LaMulanaDat::read_be(&mut reader).map_err(|_| FileGenerationError::DatFileParseFailure)
    }

    pub fn write_file(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        self.write_be(&mut writer).map_err(|_| FileGenerationError::DatFileWriteFailure)?;
        Ok(writer.into_inner())
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct Card {
    #[bw(calc = calculate_contents_size(contents))]
    pub len_contents: u16,
    #[br(map_stream = |s| s.take_seek(len_contents as u64), parse_with = until_eof)]
    pub contents: Vec<Entry>
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Entry {
    pub header: u16,
    #[br(args(header))]
    pub contents: EntryContents
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
    pub address: i16,
    pub value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Flag2 {
    pub address: i16,
    pub value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Item {
    pub value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Pose {
    pub value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Mantra {
    pub value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Color {
    pub red: i16,
    pub green: i16,
    pub blue: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct ItemName {
    pub value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Data {
    pub num_values: i16,
    #[br(count = num_values)]
    pub values: Vec<i16>
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Anime {
    pub value: i16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct Noop {}

fn calculate_contents_size(contents: &Vec<Entry>) -> u16 {
    contents.iter().fold(0, |bytes, entry| {
        let entry_bytes = match &entry.contents {
            EntryContents::Flag(_) => 4,
            EntryContents::Flag2(_) => 4,
            EntryContents::Item(_) => 2,
            EntryContents::Pose(_) => 2,
            EntryContents::Mantra(_) => 2,
            EntryContents::Color(_) => 6,
            EntryContents::ItemName(_) => 2,
            EntryContents::Data(data) => 2 + (data.num_values * 2) as u16 ,
            EntryContents::Anime(_) => 2,
            EntryContents::Noop(_) => 0
        };
        bytes + entry_bytes + 2
    })
}
