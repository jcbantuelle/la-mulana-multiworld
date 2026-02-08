use binrw::helpers::until_eof;
use binrw::io::TakeSeekExt;
use binrw::{BinRead, binread, BinWrite, binwrite};
use log::debug;
use std::collections::HashMap;
use std::io::Cursor;

use crate::archipelago::api::Location;
use crate::consts::SOURCE_DAT_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_utils;

use super::lm_consts::{CARDS, GLOBAL_FLAGS, HEADERS};

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct LaMulanaDat {
    num_cards: i16,
    #[br(count = num_cards)]
    cards: Vec<Card>
}

#[binread]
#[binwrite]
#[derive(Clone, Debug)]
pub struct Card {
    #[bw(calc = calculate_contents_size(contents))]
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

pub struct Dat {
    shop_placements: HashMap<usize,[Option<String>;3]>,
    dat_file: LaMulanaDat,
    card_lookup: HashMap<&'static str, usize>
}

impl Dat {
    pub fn new() -> Result<Self, FileGenerationError> {
        let raw_file = file_utils::read_file(&SOURCE_DAT_PATH).map_err(|_| FileGenerationError::DatFileReadFailure)?;
        let mut reader = Cursor::new(raw_file);
        let dat_file = LaMulanaDat::read_be(&mut reader).map_err(|_| FileGenerationError::DatFileParseFailure)?;
        let card_lookup = CARDS.iter().map(|(k,v)| (*k, *v as usize)).collect::<HashMap<&str, usize>>();
        Ok(Dat {
            shop_placements: HashMap::new(),
            dat_file,
            card_lookup
        })
    }

    pub fn apply_mods(&mut self) -> Result<(), FileGenerationError> {
        self.rewrite_xelpud_flag_checks();
        self.rewrite_xelpud_xmailer_conversation();
        self.rewrite_xelpud_talisman_conversation();
        self.rewrite_xelpud_pillar_conversation();
        self.rewrite_xelpud_mulana_talisman_conversation();
        self.rewrite_mulbruk_book_of_the_dead_conversation();
        self.rewrite_slushfund_flags();
        Ok(())
    }

    pub fn place_item(&mut self, item_id: i16, location: &Location, flag: i16) {
        for card_index in location.cards.clone().unwrap() {
            let old_flag = match location.original_obtain_flag {
                Some(obtain_flag) => obtain_flag,
                None => location.obtain_flag.unwrap()
            };
            if location.slot.is_none() {
                self.place_conversation_item(card_index, location.item_id.unwrap(), item_id, old_flag, flag);
                if card_index == self.card_lookup["xelpud_xmailer"] {
                    self.update_xelpud_xmailer_flag(flag);
                }
            } else {
                self.place_shop_item(card_index);
            }
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        let _ = self.dat_file.write_be(&mut writer).map_err(|_| FileGenerationError::DatFileWriteFailure)?;
        Ok(writer.into_inner())
    }

    fn place_conversation_item(&mut self, card_index: usize, old_item_id: i16, new_item_id: i16, original_flag: i16, new_flag: i16) {
        let card = &mut self.dat_file.cards[card_index];
        for entry in card.contents.iter_mut() {
            match entry.contents {
                EntryContents::Item(ref mut item) => {
                    if item.value == old_item_id {
                        item.value = new_item_id;
                    }
                },
                EntryContents::Flag(ref mut flag) => {
                    if flag.address == original_flag {
                        flag.address = new_flag;
                        flag.value = 2;
                    }
                },
                _ => ()
            }
        }
    }

    fn place_shop_item(&mut self, card_index: usize) {
        if !self.shop_placements.contains_key(&card_index) {
            self.shop_placements.insert(card_index, [None, None, None]);
        }
    }

    fn rewrite_xelpud_flag_checks(&mut self) {
        let entries_to_remove = [
            CARDS["xelpud_howling_wind"],
            CARDS["xelpud_mulana_talisman"],
            CARDS["xelpud_pillar"]
        ];
        for entry_value in entries_to_remove {
            self.remove_data_entry_by_value(self.card_lookup["xelpud_conversation_tree"], entry_value);
        }

        let entries_to_add: Vec<Vec<i16>> = vec![
            vec![GLOBAL_FLAGS["xelpud_conversation_diary_found"], 1, CARDS["xelpud_mulana_talisman"], 0],
            vec![GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 2, CARDS["xelpud_pillar"], 0],
            vec![GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 1, CARDS["xelpud_talisman"], 0]
        ];
        for entry in entries_to_add {
            self.add_data_entry(self.card_lookup["xelpud_conversation_tree"], entry);
        }
    }

    fn rewrite_xelpud_xmailer_conversation(&mut self) {
        self.update_flag_entry(self.card_lookup["xelpud_xmailer"], GLOBAL_FLAGS["xmailer"], None, Some(2));
    }

    fn rewrite_xelpud_talisman_conversation(&mut self) {
        let card_index = self.card_lookup["xelpud_talisman"];
        let insert_at_index = self.cant_leave_index(card_index);

        self.add_flag_entry(card_index, insert_at_index, GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 2);
        self.add_flag_entry(card_index, insert_at_index, GLOBAL_FLAGS["xelpud_talisman"], 1);
    }

    fn rewrite_xelpud_pillar_conversation(&mut self) {
        self.update_flag_entry(self.card_lookup["xelpud_pillar"], GLOBAL_FLAGS["shrine_diary_chest"], Some(GLOBAL_FLAGS["xelpud_conversation_talisman_found"]), Some(3));
    }

    fn rewrite_xelpud_mulana_talisman_conversation(&mut self) {
        self.update_flag_entry(self.card_lookup["xelpud_mulana_talisman"], GLOBAL_FLAGS["diary_chest_puzzle"], Some(GLOBAL_FLAGS["xelpud_conversation_diary_found"]), Some(2));
    }

    fn rewrite_mulbruk_book_of_the_dead_conversation(&mut self) {
        self.update_data_entry(self.card_lookup["mulbruk_conversation_tree"], 0, GLOBAL_FLAGS["mulbruk_book_of_the_dead"], GLOBAL_FLAGS["replacement_mulbruk_book_of_the_dead"]);

        let mulbruk_book_of_the_dead_index = self.card_lookup["mulbruk_book_of_the_dead_conversation"];
        let insert_at_index = self.cant_leave_index(mulbruk_book_of_the_dead_index);

        self.add_flag_entry(mulbruk_book_of_the_dead_index, insert_at_index , GLOBAL_FLAGS["replacement_mulbruk_book_of_the_dead"], 2);
    }

    fn rewrite_slushfund_flags(&mut self) {
        let slushfund_pepper_index = self.card_lookup["slushfund_give_pepper"];
        let slushfund_pepper_card = &self.dat_file.cards[slushfund_pepper_index];
        self.add_flag_entry(slushfund_pepper_index, slushfund_pepper_card.contents.len(), GLOBAL_FLAGS["replacement_slushfund_conversation"], 1);

        let slushfund_anchor_index = self.card_lookup["slushfund_give_anchor"];
        let slushfund_anchor_card = &self.dat_file.cards[slushfund_anchor_index];
        self.add_flag_entry(slushfund_anchor_index, slushfund_anchor_card.contents.len(), GLOBAL_FLAGS["replacement_slushfund_conversation"], 2);
    }

    fn update_xelpud_xmailer_flag(&mut self, flag: i16) {
        self.update_data_entry(self.card_lookup["xelpud_conversation_tree"], 0, GLOBAL_FLAGS["xmailer"], flag);
    }

    // Utility Functions

    fn update_data_entry(&mut self, card_index: usize, data_index: usize, old_value: i16, new_value: i16) {
        let card = &mut self.dat_file.cards[card_index];
        for entry in card.contents.iter_mut() {
            match entry.contents {
                EntryContents::Data(ref mut data) => {
                    if data.values[data_index] == old_value {
                        data.values[data_index] = new_value;
                        break;
                    }
                },
                _ => ()
            }
        }
    }

    fn update_flag_entry(&mut self, card_index: usize, old_address: i16, new_address: Option<i16>, value: Option<i16>) {
        let card = &mut self.dat_file.cards[card_index];
        for entry in card.contents.iter_mut() {
            match entry.contents {
                EntryContents::Flag(ref mut flag) => {
                    if flag.address == old_address {
                        match new_address {
                            Some(flag_address) => flag.address = flag_address,
                            None => ()
                        }
                        match value {
                            Some(flag_value) => flag.value = flag_value,
                            None => ()
                        }
                        break;
                    }
                },
                _ => ()
            }
        }
    }

    fn remove_data_entry_by_value(&mut self, card_index: usize, value: i16) {
        let card = &mut self.dat_file.cards[card_index];
        let entries = &mut card.contents;
        let mut start_delete_index = entries.iter().position(|entry| {
            match &entry.contents {
                EntryContents::Data(data) => data.values[2] == value,
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
    }

    fn add_data_entry(&mut self, card_index: usize, entry: Vec<i16>) {
        let card = &mut self.dat_file.cards[card_index];
        let entries = &mut card.contents;

        let break_entry = Entry {
            header: HEADERS["break"],
            contents: EntryContents::Noop(Noop{})
        };

        let data_entry = Entry {
            header: HEADERS["data"],
            contents: EntryContents::Data(Data {
                num_values: entry.len() as i16,
                values: entry
            })
        };

        entries.insert(0, break_entry);
        entries.insert(0, data_entry);
    }

    fn add_flag_entry(&mut self, card_index: usize, index: usize, address: i16, value: i16) {
        let card = &mut self.dat_file.cards[card_index];
        let entries = &mut card.contents;

        let flag = Entry {
            header: HEADERS["flag"],
            contents: EntryContents::Flag(Flag {
                address: address,
                value
            })
        };

        entries.insert(index, flag);
    }

    fn cant_leave_index(&mut self, card_index: usize) -> usize {
        let card = &mut self.dat_file.cards[card_index];
        card.contents
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                match &entry.contents {
                    EntryContents::Flag(flag) => {
                        flag.address == GLOBAL_FLAGS["cant_leave_conversation"]
                    },
                    _ => false
                }
            })
            .map(|(index, _)| index)
            .max()
            .unwrap()
    }
}
