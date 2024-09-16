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

lazy_static! {
    pub static ref ARCHIPELO_ITEM_LOOKUP: HashMap<u64, u32> = HashMap::from([
        (2359001, 1),
        (2359002, 2),
        (2359004, 3),
        (2359005, 4),
        (2359006, 5),
        (2359007, 6),
        (2359008, 8),
        (2359009, 9),
        (2359010, 10),
        (2359011, 11),
        (2359012, 12),
        (2359013, 13),
        (2359014, 14),
        (2359015, 15),
        (2359016, 16),
        (2359017, 75),
        (2359018, 17),
        (2359019, 18),
        (2359020, 19),
        (2359021, 19),
        (2359022, 19),
        (2359023, 19),
        (2359024, 19),
        (2359025, 19),
        (2359026, 19),
        (2359027, 19),
        (2359028, 19),
        (2359029, 19),
        (2359030, 20),
        (2359031, 21),
        (2359032, 22),
        (2359033, 23),
        (2359034, 24),
        (2359035, 25),
        (2359036, 26),
        (2359037, 27),
        (2359038, 28),
        (2359039, 29),
        (2359040, 77),
        (2359041, 30),
        (2359042, 31),
        (2359043, 81),
        (2359044, 32),
        (2359045, 33),
        (2359046, 34),
        (2359047, 72),
        (2359048, 73),
        (2359049, 36),
        (2359050, 37),
        (2359051, 38),
        (2359052, 39),
        (2359053, 40),
        (2359054, 41),
        (2359055, 42),
        (2359056, 43),
        (2359057, 44),
        (2359058, 45),
        (2359059, 46),
        (2359060, 47),
        (2359061, 48),
        (2359062, 49),
        (2359063, 50),
        (2359064, 51),
        (2359065, 52),
        (2359066, 53),
        (2359067, 54),
        (2359068, 55),
        (2359069, 56),
        (2359070, 57),
        (2359071, 58),
        (2359072, 59),
        (2359073, 60),
        (2359074, 61),
        (2359075, 62),
        (2359076, 63),
        (2359077, 64),
        (2359078, 65),
        (2359079, 66),
        (2359080, 67),
        (2359081, 68),
        (2359082, 69),
        (2359083, 71),
        (2359084, 76),
        (2359085, 74),
        (2359086, 85),
        (2359087, 86),
        (2359088, 87),
        (2359089, 88),
        (2359090, 89),
        (2359091, 90),
        (2359092, 91),
        (2359093, 92),
        (2359094, 93),
        (2359095, 94),
        (2359096, 95),
        (2359097, 96),
        (2359098, 97),
        (2359099, 98),
        (2359100, 99),
        (2359101, 100),
        (2359102, 101),
        (2359103, 102),
        (2359104, 103),
        (2359105, 104),
        (2359106, 70),
        (2359107, 70),
        (2359108, 70),
        (2359109, 70),
        (2359110, 70),
        (2359111, 70),
        (2359112, 70),
        (2359113, 70),
        (2359114, 70),
        (2359115, 70),
        (2359116, 70),
        (2359117, 70),
        (2359118, 70),
        (2359119, 70),
        (2359120, 70),
        (2359121, 70),
        (2359122, 70),
        (2359123, 107),
        (2359124, 108),
        (2359125, 109),
        (2359126, 110),
        (2359127, 111),
        (2359128, 112),
        (2359129, 113),
        (2359130, 114)
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
