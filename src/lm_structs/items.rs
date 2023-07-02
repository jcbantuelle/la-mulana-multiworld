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


/*

00 pub main_weapon: [u8; 5], 0
01 pub main_weapon: [u8; 5], 1
02 pub main_weapon: [u8; 5], 2
03 pub main_weapon: [u8; 5], 3
04 pub main_weapon: [u8; 5], 4
05 pub main_select: u8,
06 pub main_box: u8,
07 pub sub: [u8; 10], 0
08 pub sub: [u8; 10], 1
09 pub sub: [u8; 10], 2
10 pub sub: [u8; 10], 3
11 pub sub: [u8; 10], 4
12 pub sub: [u8; 10], 5
13 pub sub: [u8; 10], 6
14 pub sub: [u8; 10], 7
15 pub sub: [u8; 10], 8
16 pub sub: [u8; 10], 9
17 pub sub_num: [u8; 10], 0
18 pub sub_num: [u8; 10], 1
19 pub sub_num: [u8; 10], 2
20 pub sub_num: [u8; 10], 3
21 pub sub_num: [u8; 10], 4
22 pub sub_num: [u8; 10], 5
23 pub sub_num: [u8; 10], 6
24 pub sub_num: [u8; 10], 7
25 pub sub_num: [u8; 10], 8
26 pub sub_num: [u8; 10], 9
27 pub sub_select: u8,
28 pub sub_box: u8,
29 pub uses: [u8; 15], 0
30 pub uses: [u8; 15], 1
31 pub uses: [u8; 15], 2
32 pub uses: [u8; 15], 3
33 pub uses: [u8; 15], 4
34 pub uses: [u8; 15], 5
35 pub uses: [u8; 15], 6
36 pub uses: [u8; 15], 7
37 pub uses: [u8; 15], 8
38 pub uses: [u8; 15], 9
39 pub uses: [u8; 15], 10
40 pub uses: [u8; 15], 11
41 pub uses: [u8; 15], 12
42 pub uses: [u8; 15], 13
43 pub uses: [u8; 15], 14
44 pub use_select: u8,
45 pub use_box: u8,
46 pub lamp_num: u8,
47 pub items: [u8; 30], 0
48 pub items: [u8; 30], 1
49 pub items: [u8; 30], 2
50 pub items: [u8; 30], 3
51 pub items: [u8; 30], 4
52 pub items: [u8; 30], 5
53 pub items: [u8; 30], 6
54 pub items: [u8; 30], 7
55 pub items: [u8; 30], 8
56 pub items: [u8; 30], 9
57 pub items: [u8; 30], 10
58 pub items: [u8; 30], 11
59 pub items: [u8; 30], 12
60 pub items: [u8; 30], 13
61 pub items: [u8; 30], 14
62 pub items: [u8; 30], 15
63 pub items: [u8; 30], 16
64 pub items: [u8; 30], 17
65 pub items: [u8; 30], 18
66 pub items: [u8; 30], 19
67 pub items: [u8; 30], 20
68 pub items: [u8; 30], 21
69 pub items: [u8; 30], 22
70 pub items: [u8; 30], 23
71 pub items: [u8; 30], 24
72 pub items: [u8; 30], 25
73 pub items: [u8; 30], 26
74 pub items: [u8; 30], 27
75 pub items: [u8; 30], 28
76 pub items: [u8; 30], 29
77 pub key: [u8; 5] 0
78 pub key: [u8; 5] 1
79 pub key: [u8; 5] 2
80 pub key: [u8; 5] 3
81 pub key: [u8; 5] 4

*/

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
