use binrw::helpers::until_eof;
use binrw::io::TakeSeekExt;
use binrw::{BinRead, binrw, BinWrite};
use log::debug;
use std::collections::HashMap;
use std::io::Cursor;
use unicode_segmentation::UnicodeSegmentation;

use crate::archipelago::api::{ItemData, Location};
use crate::consts::SOURCE_DAT_PATH;
use crate::file_gen::generator::FileGenerationError;
use crate::file_gen::lm_consts::{FONT, ITEM_CODES, STARTING_WEAPONS, SUBWEAPON_AMMO};
use crate::file_gen::rcd::Rcd;
use crate::file_utils;

use super::lm_consts::{CARDS, GLOBAL_FLAGS, HEADERS};

#[derive(BinRead, BinWrite, Clone, Debug)]
pub struct LaMulanaDat {
    num_cards: i16,
    #[br(count = num_cards)]
    cards: Vec<Card>
}

#[binrw]
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
    shop_placements: HashMap<usize,[String;3]>,
    dat_file: LaMulanaDat
}

impl Dat {
    pub fn new() -> Result<Self, FileGenerationError> {
        let raw_file = file_utils::read_file(&SOURCE_DAT_PATH).map_err(|_| FileGenerationError::DatFileReadFailure)?;
        let mut reader = Cursor::new(raw_file);
        let dat_file = LaMulanaDat::read_be(&mut reader).map_err(|_| FileGenerationError::DatFileParseFailure)?;
        Ok(Dat {
            shop_placements: HashMap::new(),
            dat_file
        })
    }

    pub fn apply_mods(&mut self) -> Result<(), FileGenerationError> {
        self.rewrite_xelpud_flag_checks()?;
        self.rewrite_xelpud_xmailer_conversation();
        self.rewrite_xelpud_talisman_conversation()?;
        self.rewrite_xelpud_pillar_conversation();
        self.rewrite_xelpud_mulana_talisman_conversation();
        self.rewrite_mulbruk_book_of_the_dead_conversation()?;
        self.rewrite_slushfund_flags();

        Ok(())
    }

    pub fn place_conversation_item(&mut self, rcd_file: &mut Rcd, location: &Location, new_item_id: i16, new_flag: i16) -> Result<(), FileGenerationError> {
        let cards = location.cards.clone().ok_or_else(|| {
            debug!("Cards were not set for Dat Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

        for card_index in cards {
            let old_flag = match location.original_obtain_flag {
                Some(obtain_flag) => obtain_flag,
                None => location.obtain_flag.ok_or_else(|| {
                    debug!("Obtain Flag is missing for Dat Location: {:?}", location);
                    FileGenerationError::MalformedSlotData
                })?
            };
            let old_item_id = location.item_id.ok_or_else(|| {
                debug!("Item ID is missing for Dat Location: {:?}", location);
                FileGenerationError::MalformedSlotData
            })?;

            let card = &mut self.dat_file.cards[card_index as usize];
            for entry in card.contents.iter_mut() {
                match entry.contents {
                    EntryContents::Item(ref mut item) => {
                        if item.value == old_item_id {
                            item.value = new_item_id;
                        }
                    },
                    EntryContents::Flag(ref mut flag) => {
                        if flag.address == old_flag {
                            flag.address = new_flag;
                            flag.value = 2;
                        }
                    },
                    _ => ()
                }
            }

            if card_index == CARDS["xelpud_xmailer"] {
                self.update_xelpud_xmailer_flag(new_flag);
            }

            if card_index == CARDS["mekuri_conversation"] {
                rcd_file.rewrite_mekuri_door(new_flag);
            }
        }

        Ok(())
    }

    pub fn place_shop_item(&mut self, rcd_file: &mut Rcd, location: &Location, item_id: i16, item_flag: i16, slot: usize, mut item: ItemData, options: &HashMap<String, u64>) -> Result<(), FileGenerationError> {
        let cards = location.cards.clone().ok_or_else(|| {
            debug!("Cards were not set for Dat Location: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

        for card_id in cards {
            let card_index = card_id as usize;
            if !self.shop_placements.contains_key(&card_index) {
                self.shop_placements.insert(card_index, ["".into(), "".into(), "".into()]);
            }

            // Override Other Player Item to Map if in a Shop to prevent quantity from selling out
            let shop_item_id = if item_id == ITEM_CODES["Holy Grail (Full)"] { ITEM_CODES["Map"] } else { item_id };

            let card = &mut self.dat_file.cards[card_index];
            let mut data_entries = card.contents.iter_mut().filter_map(|entry| {
                match entry.contents {
                    EntryContents::Data(ref mut data) => Some(data),
                    _ => None
                }
            }).collect::<Vec<&mut Data>>();

            // Update Shop Item ID
            data_entries[0].values[slot] = shop_item_id;

            // Set Name of Item for Updating Shop Text
            let item_name = self.shop_placements.get_mut(&card_index).ok_or_else(|| {
                debug!("Invalid Card Index for Dat Shop Placement: {:?}", card_index);
                FileGenerationError::MalformedSlotData
            })?.get_mut(slot).ok_or_else(|| {
                debug!("Shop slot was out of bounds for Dat Location: {:?}", location);
                FileGenerationError::MalformedSlotData
            })?;

            let new_name = match &location.item {
                Some(location_item) => location_item.name.clone(),
                None => "Unknown".to_string()
            };
            *item_name = new_name;

            // Subweapon Start - make ammo for starting subweapon free and max out in 1 purchase. Same behavior for subweapon only across all subweapons
            if *item_name == format!("{} Ammo", STARTING_WEAPONS[&options["StartingWeapon"]]) || (options["SubweaponOnly"] == 1 && SUBWEAPON_AMMO.contains_key(item_name.as_str())) {
                item.cost = Some(0);
                item.quantity = SUBWEAPON_AMMO[item_name.as_str()];
            }

            // Set Cost
            data_entries[1].values[slot] = item.cost.unwrap_or_else(|| 10);
            // Set Quantity
            data_entries[2].values[slot] = item.quantity;
            // Set Flag
            data_entries[3].values[slot] = item_flag;

            // Update Little Brother Weights Purchase Flag
            if card_id == CARDS["little_brother_shop"] && shop_item_id == ITEM_CODES["Weights"] {
                data_entries[6].values[slot] = GLOBAL_FLAGS["little_brother_purchase_counter"];
            } else {
                data_entries[6].values[slot] = item_flag;
            }

            // Set New Item Name In Shop Description
            let break_indices: Vec<usize> = card.contents.iter().enumerate().filter_map(|(index, entry)| {
                if entry.header == HEADERS["break"] {
                    Some(index)
                } else {
                    None
                }
            }).collect();

            // The item descriptions in a shop are always the 7th, 8th, and 9th lines, so we want to start from the 6th break
            let item_description_start_index = break_indices[6 + slot];

            // The item name always appears between color entries
            let color_indices: Vec<usize> = card.contents[item_description_start_index..].iter().enumerate().filter_map(|(index, entry)| {
                match entry.contents {
                    EntryContents::Color(_) => Some(index + item_description_start_index),
                    _ => None
                }
            }).collect();
            let item_name_start_index = color_indices[0] + 1;

            // Encode the new item name as Entries
            let item_name_entries = Self::encode(item_name.clone())?;

            // Remove the old item name
            let entries = &mut card.contents;
            entries.drain(item_name_start_index..color_indices[1]);

            // Add the new item name
            entries.splice(item_name_start_index..item_name_start_index, item_name_entries.into_iter());

            // Nebur's 4 boss item requires an RCD mod to her door
            if card_id == CARDS["nebur_guardian"] {
                rcd_file.rewrite_four_guardian_shop_conditions(item_flag);
            }
        }

        Ok(())
    }

    pub fn update_shop_bunemon_text(&mut self) -> Result<(), FileGenerationError> {
        for (card_index, items) in self.shop_placements.clone() {
            let entries = &mut self.dat_file.cards[card_index].contents;

            // The bunemon text is always from the final break to a newline
            let last_break = entries.iter().enumerate().filter_map(|(index, entry)| {
                if entry.header == HEADERS["break"] {
                    Some(index)
                } else {
                    None
                }
            }).collect::<Vec<usize>>().last().ok_or_else(|| {
                debug!("Shop is missing Breaks: {:?}", card_index);
                FileGenerationError::MalformedDatFile
            })? + 1;

            let last_newline = entries.iter().enumerate().filter_map(|(index, entry)| {
                if entry.header == HEADERS["newline"] {
                    Some(index)
                } else {
                    None
                }
            }).collect::<Vec<usize>>().last().ok_or_else(|| {
                debug!("Shop is missing Newlines: {:?}", card_index);
                FileGenerationError::MalformedDatFile
            })?.clone();

            entries.drain(last_break..last_newline);

            let bunemon_text = items.join(" , ");
            let bunemon_entries = Self::encode(bunemon_text)?;
            entries.splice(last_break..last_break, bunemon_entries.into_iter());
        }

        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let mut writer = Cursor::new(Vec::new());
        self.dat_file.write_be(&mut writer).map_err(|_| FileGenerationError::DatFileWriteFailure)?;
        Ok(writer.into_inner())
    }

    fn rewrite_xelpud_flag_checks(&mut self) -> Result<(), FileGenerationError> {
        let entries_to_remove = [
            CARDS["xelpud_howling_wind"],
            CARDS["xelpud_mulana_talisman"],
            CARDS["xelpud_pillar"]
        ];
        for entry_value in entries_to_remove {
            self.remove_data_entry_by_value(CARDS["xelpud_conversation_tree"], entry_value)?;
        }

        let entries_to_add: Vec<Vec<i16>> = vec![
            vec![GLOBAL_FLAGS["xelpud_conversation_diary_found"], 1, CARDS["xelpud_mulana_talisman"], 0],
            vec![GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 2, CARDS["xelpud_pillar"], 0],
            vec![GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 1, CARDS["xelpud_talisman"], 0]
        ];
        for entry in entries_to_add {
            self.add_data_entry(CARDS["xelpud_conversation_tree"], entry);
        }

        Ok(())
    }

    fn rewrite_xelpud_xmailer_conversation(&mut self) {
        self.update_flag_entry(CARDS["xelpud_xmailer"], GLOBAL_FLAGS["xmailer"], None, Some(2));
    }

    fn rewrite_xelpud_talisman_conversation(&mut self) -> Result<(), FileGenerationError> {
        let card_index = CARDS["xelpud_talisman"];
        let insert_at_index = self.cant_leave_index(card_index)?;

        self.add_flag_entry(card_index, insert_at_index, GLOBAL_FLAGS["xelpud_conversation_talisman_found"], 2);
        self.add_flag_entry(card_index, insert_at_index, GLOBAL_FLAGS["xelpud_talisman"], 1);

        Ok(())
    }

    fn rewrite_xelpud_pillar_conversation(&mut self) {
        self.update_flag_entry(CARDS["xelpud_pillar"], GLOBAL_FLAGS["shrine_diary_chest"], Some(GLOBAL_FLAGS["xelpud_conversation_talisman_found"]), Some(3));
    }

    fn rewrite_xelpud_mulana_talisman_conversation(&mut self) {
        self.update_flag_entry(CARDS["xelpud_mulana_talisman"], GLOBAL_FLAGS["diary_chest_puzzle"], Some(GLOBAL_FLAGS["xelpud_conversation_diary_found"]), Some(2));
    }

    fn rewrite_mulbruk_book_of_the_dead_conversation(&mut self) -> Result<(), FileGenerationError> {
        self.update_data_entry(CARDS["mulbruk_conversation_tree"], 0, GLOBAL_FLAGS["mulbruk_book_of_the_dead"], GLOBAL_FLAGS["replacement_mulbruk_book_of_the_dead"]);

        let mulbruk_book_of_the_dead_index = CARDS["mulbruk_book_of_the_dead_conversation"];
        let insert_at_index = self.cant_leave_index(mulbruk_book_of_the_dead_index)?;

        self.add_flag_entry(mulbruk_book_of_the_dead_index, insert_at_index , GLOBAL_FLAGS["replacement_mulbruk_book_of_the_dead"], 2);

        Ok(())
    }

    fn rewrite_slushfund_flags(&mut self) {
        let slushfund_pepper_index = CARDS["slushfund_give_pepper"];
        let slushfund_pepper_card = &self.dat_file.cards[slushfund_pepper_index as usize];
        self.add_flag_entry(slushfund_pepper_index, slushfund_pepper_card.contents.len(), GLOBAL_FLAGS["replacement_slushfund_conversation"], 1);

        let slushfund_anchor_index = CARDS["slushfund_give_anchor"];
        let slushfund_anchor_card = &self.dat_file.cards[slushfund_anchor_index as usize];
        self.add_flag_entry(slushfund_anchor_index, slushfund_anchor_card.contents.len(), GLOBAL_FLAGS["replacement_slushfund_conversation"], 2);
    }

    fn update_xelpud_xmailer_flag(&mut self, flag: i16) {
        self.update_data_entry(CARDS["xelpud_conversation_tree"], 0, GLOBAL_FLAGS["xmailer"], flag);
    }

    // Utility Functions

    fn update_data_entry(&mut self, card_id: i16, data_index: usize, old_value: i16, new_value: i16) {
        let card = &mut self.dat_file.cards[card_id as usize];
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

    fn update_flag_entry(&mut self, card_id: i16, old_address: i16, new_address: Option<i16>, value: Option<i16>) {
        let card = &mut self.dat_file.cards[card_id as usize];
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

    fn remove_data_entry_by_value(&mut self, card_id: i16, value: i16) -> Result<(), FileGenerationError> {
        let card = &mut self.dat_file.cards[card_id as usize];
        let entries = &mut card.contents;

        let mut start_delete_index = entries.iter().position(|entry| {
            match &entry.contents {
                EntryContents::Data(data) => data.values[2] == value,
                _ => false
            }
        }).ok_or_else(|| {
            debug!("Couldn't locate Data Entry containing {:?} in Card {:?}", value, card_id);
            FileGenerationError::MalformedDatFile
        })?;
        let mut end_delete_index = start_delete_index + 2;

        // If it's the final Entry, we want to remove the preceding break instead of the trailing one
        if start_delete_index == (entries.len() - 1) {
            start_delete_index -= 1;
            end_delete_index -= 1;
        }

        entries.drain(start_delete_index..end_delete_index);

        Ok(())
    }

    fn add_data_entry(&mut self, card_id: i16, entry: Vec<i16>) {
        let card = &mut self.dat_file.cards[card_id as usize];
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

    fn add_flag_entry(&mut self, card_id: i16, index: usize, address: i16, value: i16) {
        let card = &mut self.dat_file.cards[card_id as usize];
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

    fn cant_leave_index(&mut self, card_id: i16) -> Result<usize, FileGenerationError> {
        let card = &mut self.dat_file.cards[card_id as usize];
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
            .ok_or_else(|| {
                debug!("Can't Leave Conversation Flag is missing for Card {:?}", card_id);
                FileGenerationError::MalformedDatFile
            })
    }

    fn encode(text: String) -> Result<Vec<Entry>, FileGenerationError> {
        let font_graphemes = UnicodeSegmentation::graphemes(FONT, true).collect::<Vec<&str>>();
        let fallback_grapheme_position = font_graphemes.iter().position(|font_grapheme| { *font_grapheme == "?" }).ok_or_else(|| {
            debug!("Error Reading Font Encoding");
            FileGenerationError::FontEncodingError
        })?;

        let text_graphemes = UnicodeSegmentation::graphemes(text.as_str(), true).collect::<Vec<&str>>().into_iter();
        let encoded_text = text_graphemes.map(|text_grapheme| {
            let codepoint = if text_grapheme == " " {
                HEADERS["white_space"]
            } else {
                let grapheme_position = font_graphemes.iter().position(|font_grapheme| { *font_grapheme == text_grapheme }).unwrap_or_else(|| fallback_grapheme_position);
                (grapheme_position + 0x100) as u16
            };

            Entry {
                header: codepoint,
                contents: EntryContents::Noop(Noop{})
            }
        }).collect::<Vec<Entry>>();

        Ok(encoded_text)
    }
}
