use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ItemPossessionBuffer {
    pub main_weapon: [u8; 5],
    pub main_select: u8,
    pub main_box: u8,
    pub sub: [u8; 10],
    pub sub_num: [u8; 10],
    pub sub_select: u8,
    pub sub_box: u8,
    pub uses: [u8; 15],
    pub use_select: u8,
    pub use_box: u8,
    pub lamp_num: u8,
    pub items: [u8; 30],
    pub key: [u8; 5]
}

pub struct Value {
    pub index: u8,
    pub value: u8
}

pub struct Item {
    pub item_id: usize,
    pub item_name: String,
    pub flag: usize,
    pub value: u8
}

lazy_static! {
    pub static ref ARCHIPELAGO_ITEM_LOOKUP: HashMap<u64, Item> = HashMap::from([
        (2359001, Item{item_id: 1, item_name: "Chain Whip", flag: 0x7d, value: 1}),
        (2359002, Item{item_id: 2, item_name: "Flail Whip", flag: 0x7e, value: 1}),
        (2359004, Item{item_id: 3, item_name: "Knife", flag: 0x7f, value: 1}),
        (2359005, Item{item_id: 4, item_name: "Key Sword", flag: 0x80, value: 1}),
        (2359006, Item{item_id: 5, item_name: "Axe", flag: 0x81, value: 1}),
        (2359007, Item{item_id: 6, item_name: "Katana", flag: 0x82, value: 1}),
        (2359008, Item{item_id: 8, item_name: "Shuriken", flag: 0x83, value: 1}),
        (2359009, Item{item_id: 9, item_name: "Rolling Shuriken", flag: 0x84, value: 1}),
        (2359010, Item{item_id: 10, item_name: "Earth Spear", flag: 0x85, value: 1}),
        (2359011, Item{item_id: 11, item_name: "Flare Gun", flag: 0x86, value: 1}),
        (2359012, Item{item_id: 12, item_name: "Bomb", flag: 0x87, value: 1}),
        (2359013, Item{item_id: 13, item_name: "Chakram", flag: 0x88, value: 1}),
        (2359014, Item{item_id: 14, item_name: "Caltrops", flag: 0x89, value: 1}),
        (2359015, Item{item_id: 15, item_name: "Pistol", flag: 0x8a, value: 1}),
        (2359016, Item{item_id: 16, item_name: "Buckler", flag: 0x8b, value: 1}),
        (2359017, Item{item_id: 75, item_name: "Fake Silver Shield", flag: 0x82e, value: 1}),
        (2359018, Item{item_id: 17, item_name: "Silver Shield", flag: 0x8c, value: 1}),
        (2359019, Item{item_id: 18, item_name: "Angel Shield", flag: 0x8d, value: 2}),
        (2359020, Item{item_id: 19, item_name: "Ankh Jewel", flag: 0x852, value: 1}),
        (2359021, Item{item_id: 19, item_name: "Ankh Jewel (Amphisbaena)", flag: 0x853, value: 1}),
        (2359022, Item{item_id: 19, item_name: "Ankh Jewel (Sakit)", flag: 0x854, value: 1}),
        (2359023, Item{item_id: 19, item_name: "Ankh Jewel (Elmac)", flag: 0x855, value: 1}),
        (2359024, Item{item_id: 19, item_name: "Ankh Jewel (Bahamut)", flag: 0x856, value: 1}),
        (2359025, Item{item_id: 19, item_name: "Ankh Jewel (Viy)", flag: 0x857, value: 1}),
        (2359026, Item{item_id: 19, item_name: "Ankh Jewel (Palenque)", flag: 0x858, value: 1}),
        (2359027, Item{item_id: 19, item_name: "Ankh Jewel (Baphomet)", flag: 0x859, value: 1}),
        (2359028, Item{item_id: 19, item_name: "Ankh Jewel (Tiamat)", flag: 0x85a, value: 1}),
        (2359029, Item{item_id: 19, item_name: "Ankh Jewel (Mother)", flag: 0x85b, value: 1}),
        (2359030, Item{item_id: 20, item_name: "Hand Scanner", flag: 0x96, value: 2}),
        (2359031, Item{item_id: 21, item_name: "Djed Pillar", flag: 0x97, value: 2}),
        (2359032, Item{item_id: 22, item_name: "Mini Doll", flag: 0x98, value: 2}),
        (2359033, Item{item_id: 23, item_name: "Magatama Jewel", flag: 0x99, value: 2}),
        (2359034, Item{item_id: 24, item_name: "Cog of the Soul", flag: 0x9a, value: 1}),
        (2359035, Item{item_id: 25, item_name: "Lamp of Time", flag: 0x9b, value: 2}),
        (2359036, Item{item_id: 26, item_name: "Pochette Key", flag: 0x9c, value: 2}),
        (2359037, Item{item_id: 27, item_name: "Dragon Bone", flag: 0x9d, value: 2}),
        (2359038, Item{item_id: 28, item_name: "Crystal Skull", flag: 0x9e, value: 2}),
        (2359039, Item{item_id: 29, item_name: "Vessel", flag: 0x9f, value: 2}),
        (2359040, Item{item_id: 77, item_name: "Medicine of the Mind", flag: 0x85c, value: 1}),
        (2359041, Item{item_id: 30, item_name: "Pepper", flag: 0x228, value: 1}),
        (2359042, Item{item_id: 31, item_name: "Woman Statue", flag: 0xa1, value: 2}),
        (2359043, Item{item_id: 81, item_name: "Maternity Statue", flag: 0x10b, value: 2}),
        (2359044, Item{item_id: 32, item_name: "Key of Eternity", flag: 0xa2, value: 2}),
        (2359045, Item{item_id: 33, item_name: "Serpent Staff", flag: 0xa3, value: 2}),
        (2359046, Item{item_id: 34, item_name: "Talisman", flag: 0xa4, value: 2}),
        (2359047, Item{item_id: 72, item_name: "Diary", flag: 0x104, value: 2}),
        (2359048, Item{item_id: 73, item_name: "Mulana Talisman", flag: 0x105, value: 1}),
        (2359049, Item{item_id: 36, item_name: "Waterproof Case", flag: 0xa5, value: 2}),
        (2359050, Item{item_id: 37, item_name: "Heatproof Case", flag: 0xa6, value: 2}),
        (2359051, Item{item_id: 38, item_name: "Shell Horn", flag: 0xa7, value: 2}),
        (2359052, Item{item_id: 39, item_name: "Glove", flag: 0xa8, value: 2}),
        (2359053, Item{item_id: 40, item_name: "Holy Grail", flag: 0xa9, value: 2}),
        (2359054, Item{item_id: 41, item_name: "Isis' Pendant", flag: 0xaa, value: 2}),
        (2359055, Item{item_id: 42, item_name: "Crucifix", flag: 0xab, value: 2}),
        (2359056, Item{item_id: 43, item_name: "Helmet", flag: 0xac, value: 1}),
        (2359057, Item{item_id: 44, item_name: "Grapple Claw", flag: 0xad, value: 2}),
        (2359058, Item{item_id: 45, item_name: "Bronze Mirror", flag: 0xae, value: 2}),
        (2359059, Item{item_id: 46, item_name: "Eye of Truth", flag: 0xaf, value: 2}),
        (2359060, Item{item_id: 47, item_name: "Ring", flag: 0xb0, value: 1}),
        (2359061, Item{item_id: 48, item_name: "Scalesphere", flag: 0xb1, value: 2}),
        (2359062, Item{item_id: 49, item_name: "Gauntlet", flag: 0xb2, value: 2}),
        (2359063, Item{item_id: 50, item_name: "Anchor", flag: 0x228, value: 2}),
        (2359064, Item{item_id: 51, item_name: "Plane Model", flag: 0xb4, value: 2}),
        (2359065, Item{item_id: 52, item_name: "Philosopher's Ocarina", flag: 0xb5, value: 2}),
        (2359066, Item{item_id: 53, item_name: "Feather", flag: 0xb6, value: 2}),
        (2359067, Item{item_id: 54, item_name: "Book of the Dead", flag: 0x32a, value: 2}),
        (2359068, Item{item_id: 55, item_name: "Fairy Clothes", flag: 0xb8, value: 2}),
        (2359069, Item{item_id: 56, item_name: "Scriptures", flag: 0xb9, value: 2}),
        (2359070, Item{item_id: 57, item_name: "Hermes' Boots", flag: 0xba, value: 2}),
        (2359071, Item{item_id: 58, item_name: "Fruit of Eden", flag: 0xbb, value: 2}),
        (2359072, Item{item_id: 59, item_name: "Twin Statue", flag: 0xbc, value: 2}),
        (2359073, Item{item_id: 60, item_name: "Bracelet", flag: 0xbd, value: 2}),
        (2359074, Item{item_id: 61, item_name: "Perfume", flag: 0xbe, value: 2}),
        (2359075, Item{item_id: 62, item_name: "Spaulder", flag: 0xbf, value: 2}),
        (2359076, Item{item_id: 63, item_name: "Dimensional Key", flag: 0xc0, value: 2}),
        (2359077, Item{item_id: 64, item_name: "Ice Cape", flag: 0xc1, value: 2}),
        (2359078, Item{item_id: 65, item_name: "Origin Seal", flag: 0xc2, value: 2}),
        (2359079, Item{item_id: 66, item_name: "Birth Seal", flag: 0xc3, value: 2}),
        (2359080, Item{item_id: 67, item_name: "Life Seal", flag: 0xc4, value: 2}),
        (2359081, Item{item_id: 68, item_name: "Death Seal", flag: 0xc5, value: 2}),
        (2359082, Item{item_id: 69, item_name: "Sacred Orb", flag: 0x85d, value: 1}),
        (2359083, Item{item_id: 71, item_name: "Treasures", flag: 0x103, value: 2}),
        (2359084, Item{item_id: 76, item_name: "Mobile Super X2", flag: 0x2e6, value: 2}),
        (2359085, Item{item_id: 74, item_name: "Provocative Bathing Suit", flag: 0x106, value: 2}),
        (2359086, Item{item_id: 85, item_name: "", flag: 0x, value: }),
        (2359087, Item{item_id: 86, item_name: "", flag: 0x, value: }),
        (2359088, Item{item_id: 87, item_name: "", flag: 0x, value: }),
        (2359089, Item{item_id: 88, item_name: "", flag: 0x, value: }),
        (2359090, Item{item_id: 89, item_name: "", flag: 0x, value: }),
        (2359091, Item{item_id: 90, item_name: "", flag: 0x, value: }),
        (2359092, Item{item_id: 91, item_name: "", flag: 0x, value: }),
        (2359093, Item{item_id: 92, item_name: "", flag: 0x, value: }),
        (2359094, Item{item_id: 93, item_name: "", flag: 0x, value: }),
        (2359095, Item{item_id: 94, item_name: "", flag: 0x, value: }),
        (2359096, Item{item_id: 95, item_name: "", flag: 0x, value: }),
        (2359097, Item{item_id: 96, item_name: "", flag: 0x, value: }),
        (2359098, Item{item_id: 97, item_name: "", flag: 0x, value: }),
        (2359099, Item{item_id: 98, item_name: "", flag: 0x, value: }),
        (2359100, Item{item_id: 99, item_name: "", flag: 0x, value: }),
        (2359101, Item{item_id: 100, item_name: "", flag: 0x, value: }),
        (2359102, Item{item_id: 101, item_name: "", flag: 0x, value: }),
        (2359103, Item{item_id: 102, item_name: "", flag: 0x, value: }),
        (2359104, Item{item_id: 103, item_name: "", flag: 0x, value: }),
        (2359105, Item{item_id: 104, item_name: "", flag: 0x, value: }),
        (2359106, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359107, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359108, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359109, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359110, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359111, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359112, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359113, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359114, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359115, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359116, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359117, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359118, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359119, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359120, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359121, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359122, Item{item_id: 70, item_name: "", flag: 0x, value: }),
        (2359123, Item{item_id: 107, item_name: "", flag: 0x, value: }),
        (2359124, Item{item_id: 108, item_name: "", flag: 0x, value: }),
        (2359125, Item{item_id: 109, item_name: "", flag: 0x, value: }),
        (2359126, Item{item_id: 110, item_name: "", flag: 0x, value: }),
        (2359127, Item{item_id: 111, item_name: "", flag: 0x, value: }),
        (2359128, Item{item_id: 112, item_name: "", flag: 0x, value: }),
        (2359129, Item{item_id: 113, item_name: "", flag: 0x, value: }),
        (2359130, Item{item_id: 114, item_name: "", flag: 0x, value: })
    ]);
}

pub fn generate_item_translator() -> HashMap<u8, Value> {
    /*
    NOTES:
    56 (ITEM_BOOTS): There is some logic here but still assigns to the right place
    68 (ITEM_LIFEJEM): Does not assign to any thing
    72 (ITEM_FALSE_SILVER_SHIELD): Is weird where it assigns sub[8] multiple values depending on the logic
    73 (ITEM_MSX2): Sets item[0] to '#' if zero, else 'L'
    74 (ITEM_YELLOW_LIQUID): Has some weird logic that I don't understand
    75 (ITEM_GREEN_LIQUID): Same as yellow liquid
    76 (ITEM_RED_LIQUID): Same as yellow liquid
    79 (ITEM_GRAIL_COLLAPSE): Sets some other variables to 0 in the other struct
    80 (ITEM_GRAIL_AWAKING): Same as 79
    81 (ITEM_LIFEJEM_2): There is some weird logic after setting the inventory (key[4])
    82 (ITEM_SOFT_KOMONJOREADER): Sets game flags
    83 (ITEM_SOFT_MAILER): Sets game flags
     */
    let mut hash = HashMap::from([
        (0, Value { index: 0, value: 0 }),
        (1, Value { index: 0, value: 1 }),
        (2, Value { index: 0, value: 2 }),
        (3, Value { index: 1, value: 3 }),
        (4, Value { index: 2, value: 4 }),
        (5, Value { index: 3, value: 5 }),
        (6, Value { index: 4, value: 6 }),
        (7, Value { index: 2, value: 7 }),
        (77, Value { index: 34, value: 80 }),
        (78, Value { index: 40, value: 81 }),
        (79, Value { index: 52, value: 82 }),
        (80, Value { index: 52, value: 83 }),
        (81, Value { index: 81, value: 84 })
    ]);

    // map `use` member
    for i in 20..34 {
        if !hash.contains_key(&i) {
            hash.insert(i, Value { index: i + 9, value: i });
        }
    }

    // map `item` member
    for i in 35..63 {
        if !hash.contains_key(&i) {
            hash.insert(i, Value { index: i + 13, value: i });
        }
    }

    // map `key` member
    for i in 64..67 {
        if !hash.contains_key(&i) {
            hash.insert(i, Value { index: i + 13, value: i });
        }
    }
    return hash;
}
