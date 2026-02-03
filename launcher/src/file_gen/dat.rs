use binrw::helpers::until_eof;
use binrw::io::TakeSeekExt;
use binrw::{BinRead, BinWrite};
use log::debug;
use std::collections::HashMap;
use std::io::Cursor;

use crate::archipelago::api::SlotData;
use crate::consts::SOURCE_DAT_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_utils;

use super::lm_consts::{CARDS, GLOBAL_FLAGS};

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

pub struct Dat {
    next_filler_item_flag: usize,
    shop_placements: HashMap<u64,[Option<String>;3]>,
    dat_file: LaMulanaDat
}

impl Dat {
    pub fn new() -> Result<Self, FileGenerationError> {
        let raw_file = file_utils::read_file(&SOURCE_DAT_PATH).map_err(|_| FileGenerationError::DatFileReadFailure)?;
        let mut reader = Cursor::new(raw_file);
        let dat_file = LaMulanaDat::read_be(&mut reader).map_err(|_| FileGenerationError::DatFileParseFailure)?;
        Ok(Dat {
            next_filler_item_flag: GLOBAL_FLAGS["dat_filler_items"],
            shop_placements: HashMap::new(),
            dat_file
        })
    }

    pub fn apply_mods(&mut self) -> Result<(), FileGenerationError> {
        self.rewrite_xelpud_flag_checks();
        // self.rewrite_xelpud_xmailer_conversation();
        // self.rewrite_xelpud_talisman_conversation();
        // self.rewrite_xelpud_pillar_conversation();
        // self.rewrite_xelpud_mulana_talisman_conversation();
        // self.rewrite_mulbruk_book_of_the_dead_conversation();
        // self.update_slushfund_flags();
        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        let _ = self.dat_file.write_be(&mut writer).map_err(|_| FileGenerationError::DatFileWriteFailure)?;
        Ok(writer.into_inner())
    }

    fn rewrite_xelpud_flag_checks(&mut self) {
        let entries_to_remove = [
            CARDS["xelpud_howling_wind"],
            CARDS["xelpud_mulana_talisman"],
            CARDS["xelpud_pillar"]
        ];
        for entry_value in entries_to_remove {
            self.remove_data_entry_by_value(CARDS["xelpud_conversation_tree"], entry_value as i16);
        }

        // data_values_to_add = [
        //     [GLOBAL_FLAGS["xelpud_conversation_diary_found"], 1, CARDS["xelpud_mulana_talisman"], 0],
        //     [GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 2, CARDS["xelpud_pillar"], 0],
        //     [GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 1, CARDS["xelpud_talisman"], 0]
        // ]
        // for data_values in data_values_to_add:
        //     self.__add_data_entry(card, data_values)
    }

    fn remove_data_entry_by_value(&mut self, card_index: usize, value: i16) {
        let card = &mut self.dat_file.cards[card_index];
        let entries = &mut card.contents;
        let mut removed_bytes: u16 = 6;
        let mut start_delete_index = entries.iter().position(|entry| {
            match &entry.contents {
                EntryContents::Data(data) => {
                    if data.values[2] == value {
                        removed_bytes += (data.num_values as u16) * 2;
                        return true
                    }
                    false
                },
                _ => false
            }
        }).unwrap();
        let mut end_delete_index = start_delete_index + 2;

        // If it's the final Entry, we want to remove the preceding break instead of the trailing one
        if start_delete_index == (entries.len() - 1) {
            start_delete_index -= 1;
            end_delete_index -= 1;
        }

        entries.drain(start_delete_index..end_delete_index);
        card.len_contents -= removed_bytes;
    }
}
