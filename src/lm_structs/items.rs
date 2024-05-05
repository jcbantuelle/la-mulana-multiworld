use std::collections::HashMap;

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
