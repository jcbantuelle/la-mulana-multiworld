use std::collections::HashMap;
use std::sync::LazyLock;

pub const GLOBAL_FLAGS: LazyLock<HashMap<&'static str, i16>> = LazyLock::new(|| {
    HashMap::from([
        ("screen_flag_00", 0x00),
        ("screen_flag_01", 0x01),
        ("screen_flag_02", 0x02),
        ("screen_flag_0c", 0x0c),
        ("screen_flag_0d", 0x0d),
        ("screen_flag_2e", 0x2e),
        ("screen_flag_2f", 0x2f),
        ("coin_chests", 0x77),
        ("knife_found", 0x7f),
        ("keysword_found", 0x80),
        ("axe_found", 0x81),
        ("katana_found", 0x82),
        ("shurikens_found", 0x83),
        ("rolling_shurikens_found", 0x84),
        ("earth_spears_found", 0x85),
        ("flare_gun_found", 0x86),
        ("bombs_found", 0x87),
        ("chakrams_found", 0x88),
        ("caltrops_found", 0x89),
        ("pistol_found", 0x8a),
        ("amphisbaena_ankh_jewel_found", 0x8e),
        ("sakit_ankh_jewel_found", 0x8f),
        ("ellmac_ankh_jewel_found", 0x90),
        ("bahamut_ankh_jewel_found", 0x91),
        ("viy_ankh_jewel_found", 0x92),
        ("palenque_ankh_jewel_found", 0x93),
        ("baphomet_ankh_jewel_found", 0x94),
        ("tiamat_ankh_jewel_found", 0x95),
        ("talisman_found", 0xa4),
        ("crucifix_found", 0xab),
        ("plane_found", 0xb4),
        ("guidance_orb_found", 0xc7),
        ("surface_map", 0xd1),
        ("shrine_map", 0xda),
        ("xmailer", 0xe3),
        ("mekuri", 0xf1),
        ("grail_tablet_guidance", 0x64),
        ("grail_tablet_mausoleum", 0x65),
        ("grail_tablet_sun", 0x66),
        ("grail_tablet_spring", 0x67),
        ("grail_tablet_inferno", 0x68),
        ("grail_tablet_extinction", 0x69),
        ("grail_tablet_twin_front", 0x6a),
        ("grail_tablet_endless", 0x6b),
        ("grail_tablet_shrine_front", 0x6c),
        ("grail_tablet_illusion", 0x6d),
        ("grail_tablet_graveyard", 0x6e),
        ("grail_tablet_moonlight", 0x6f),
        ("grail_tablet_goddess", 0x70),
        ("grail_tablet_ruin", 0x71),
        ("grail_tablet_birth", 0x72),
        ("grail_tablet_twin_back", 0x73),
        ("grail_tablet_dimensional", 0x74),
        ("grail_tablet_shrine_back", 0x75),
        ("score", 0x7b),
        ("xelpud_conversation_general", 0x7c),
        ("ankh_jewel_mausoleum", 0x8f),
        ("ankh_jewel_sun", 0x90),
        ("yagostr_found", 0xe5),
        ("amphisbaena_state", 0xf6),
        ("sakit_state", 0xf7),
        ("ellmac_state", 0xf8),
        ("bahamut_state", 0xf9),
        ("viy_state", 0xfa),
        ("palenque_state", 0xfb),
        ("baphomet_state", 0xfc),
        ("tiamat_state", 0xfd),
        ("mother_state", 0xfe),
        ("guardians_killed", 0x102),
        ("diary_found", 0x104),
        ("mulana_talisman", 0x105),
        ("swimsuit_found", 0x106),
        ("amphisbaena_ankh_puzzle", 0x133),
        ("sakit_ankh_puzzle", 0x164),
        ("hardmode", 0x16a),
        ("ellmac_ankh_puzzle", 0x178),
        ("sun_map_chest_ladder_despawned", 0x183),
        ("sun_map_chest_ladder_restored", 0x188),
        ("fishman_shop_puzzle", 0x197),
        ("bahamut_room_flooded", 0x199),
        ("bahamut_ankh_puzzle", 0x19f),
        ("chain_whip_dais_left", 0x1b1),
        ("chain_whip_dais_right", 0x1b2),
        ("viy_ankh_puzzle", 0x1b4),
        ("palenque_ankh_puzzle", 0x1c3),
        ("palenque_screen_mural", 0x1ca),
        ("baphomet_ankh_puzzle", 0x1e0),
        ("little_brother_purchase_counter", 0x1ea),
        ("endless_fairyqueen", 0x1f5),
        ("diary_chest_puzzle", 0x212),
        ("shrine_dragon_bone", 0x218),
        ("shrine_diary_chest", 0x219),
        ("shrine_shawn", 0x21b),
        ("xelpud_msx2", 0x21d),
        ("slushfund_conversation", 0x228),
        ("cog_puzzle", 0x23a),
        ("moonlight_to_twin_breakable_floor", 0x25e),
        ("flail_whip_puzzle", 0x27b),
        ("ushumgallu_state", 0x2cc),
        ("dimensional_angel_shield_dais_left", 0x2d2),
        ("dimensional_angel_shield_dais_right", 0x2d3),
        ("cant_leave_conversation", 0x2e4),
        ("translation_tablets_read", 0x2e5),
        ("msx2_found", 0x2e6),
        ("ancient_lamulanese_learned", 0x2ea),
        ("dimensional_children_dead", 0x2ec),
        ("tiamat_ankh_puzzle", 0x2ed),
        ("mother_ankh_puzzle", 0x2e0),
        ("xelpud_talisman", 0x327),
        ("mulbruk_book_of_the_dead", 0x32a),
        ("end_start_animation", 0x338),
        ("hell_dlc", 0x34a),
        ("mulbruk_father", 0x34c),
        ("sacred_orb_count", 0x354),
        ("orb_count_incremented_guidance", 0x355),
        ("mulbruk_conversation_unknown", 0x36a),
        ("escape", 0x382),
        ("kill_flag", 0x3e9),
        ("grail_tablet_surface", 0x54e),
        ("starting_items", 0x84f),
        ("mother_ankh_jewel_found", 0x853),
        ("replacement_surface_map_scan", 0x85f),
        ("replacement_slushfund_conversation", 0x860),
        ("replacement_cog_puzzle", 0x861),
        ("mother_ankh_jewel_recovery", 0x862),
        ("randomizer_save_loaded", 0x863),
        ("replacement_mulbruk_book_of_the_dead", 0x864),
        ("xelpud_conversation_talisman_found", 0x865),
        ("xelpud_conversation_diary_found", 0x866),
        ("received_items_index_1", 0x867),
        ("received_items_index_2", 0x868),
        ("filler_items", 0x9f6)
    ])
});

pub const INVENTORY: LazyLock<HashMap<&'static str, usize>> = LazyLock::new(|| {
        HashMap::from([
            ("knife", 0x3),
            ("keysword", 0x4),
            ("axe", 0x5),
            ("katana", 0x6),
            ("axe", 0x5),
            ("shurikens", 0x8),
            ("rolling_shurikens", 0x9),
            ("earth_spears", 0xa),
            ("flare_gun", 0xb),
            ("bombs", 0xc),
            ("chakrams", 0xd),
            ("caltrops", 0xe),
            ("pistol", 0xf),
            ("shuriken_ammo", 0x6b),
            ("rolling_shuriken_ammo", 0x6c),
            ("earth_spear_ammo", 0x6d),
            ("flare_gun_ammo", 0x6e),
            ("bomb_ammo", 0x6f),
            ("chakram_ammo", 0x70),
            ("caltrop_ammo", 0x71),
            ("pistol_clip_ammo", 0x72),
            ("pistol_bullet_ammo", 0x74),
        ])
    }
);

pub const HEADERS: LazyLock<HashMap<&'static str, u16>> = LazyLock::new(|| {
    HashMap::from([
        ("break", 0x000a),
        ("white_space", 0x0020),
        ("flag", 0x0040),
        ("flag2", 0x0041),
        ("item", 0x0042),
        ("newline", 0x0045),
        ("pose", 0x0046),
        ("mantra", 0x0047),
        ("color", 0x004a),
        ("item_name", 0x004d),
        ("data", 0x004e),
        ("anime", 0x004f)
    ])
});

pub const CARDS: LazyLock<HashMap<&'static str, i16>> = LazyLock::new(|| {
    HashMap::from([
        ("mekuri_conversation", 37),
        ("little_brother_shop", 185),
        ("slushfund_give_pepper", 245),
        ("slushfund_give_anchor", 247),
        ("xelpud_xmailer", 364),
        ("xelpud_talisman", 369),
        ("xelpud_pillar", 370),
        ("xelpud_mulana_talisman", 371),
        ("xelpud_score_howling_wind", 373),
        ("mulbruk_book_of_the_dead_conversation", 397),
        ("xelpud_conversation_tree", 480),
        ("xelpud_score_tree", 482),
        ("mulbruk_conversation_tree", 486),
        ("nebur_guardian", 490),
        ("xelpud_howling_wind", 1049),
    ])
});

pub const RCD_OBJECTS: LazyLock<HashMap<&'static str, i16>> = LazyLock::new(|| {
    HashMap::from([
        ("ladder", 0x7),
        ("trigger_dais", 0x8),
        ("moving_texture", 0xa),
        ("flag_timer", 0xb),
        ("room_spawner", 0xe),
        ("crusher", 0x11),
        ("hitbox_generator", 0x12),
        ("lemeza_detector", 0x14),
        ("counterweight_platform", 0x33),
        ("chest", 0x2c),
        ("ankh", 0x2e),
        ("naked_item", 0x2f),
        ("trigger_seal", 0x34),
        ("big_anubis", 0x6b),
        ("vimana", 0x71),
        ("texture_draw_animation", 0x93),
        ("warp_door", 0x98),
        ("use_item", 0x9c),
        ("scannable", 0x9e),
        ("grail_point", 0x9f),
        ("language_conversation", 0xa0),
        ("animation", 0xa3),
        ("fairy_keyspot", 0xa7),
        ("explosion", 0xb4),
        ("instant_item", 0xb5),
        ("xelpud_pillar", 0xbb),
        ("mother_ankh", 0xc0),
        ("scan", 0xc3),
    ])
});

pub const TEST_OPERATIONS: LazyLock<HashMap<&'static str, i8>> = LazyLock::new(|| {
    HashMap::from([
        ("eq", 0x0),
        ("lteq", 0x1),
        ("gteq", 0x2),
        ("andnz", 0x3),
        ("ornz", 0x4),
        ("xornz", 0x5),
        ("zero", 0x6),
        ("neq", 0x40),
        ("gt", 0x41),
        ("lt", 0x42),
        ("andz", 0x43),
        ("orz", 0x44),
        ("xorz", 0x45),
        ("nz", 0x46),
    ])
});

pub const WRITE_OPERATIONS: LazyLock<HashMap<&'static str, i8>> = LazyLock::new(|| {
    HashMap::from([
        ("assign", 0x0),
        ("add", 0x1),
        ("sub", 0x2),
        ("mult", 0x3),
        ("div", 0x4),
        ("and", 0x5),
        ("or", 0x6),
        ("xor", 0x7),
    ])
});

pub const STARTING_WEAPONS: LazyLock<HashMap<u64, &'static str>> = LazyLock::new(|| {
	HashMap::from([
        (0, "Leather Whip"),
        (1, "Knife"),
        (2, "Key Sword"),
        (3, "Axe"),
        (4, "Katana"),
        (5, "Shuriken"),
        (6, "Rolling Shuriken"),
        (7, "Earth Spear"),
        (8, "Flare Gun"),
        (9, "Bomb"),
        (10, "Chakram"),
        (11, "Caltrops"),
        (12, "Pistol")
    ])
});

pub const SUBWEAPON_AMMO: LazyLock<HashMap<&str, i16>> = LazyLock::new(|| {
    HashMap::from([
        ("Shuriken Ammo", 150),
        ("Rolling Shuriken Ammo", 100),
        ("Earth Spear Ammo", 80),
        ("Flare Gun Ammo", 80),
        ("Bomb Ammo", 30),
        ("Chakram Ammo", 10),
        ("Caltrops Ammo", 80),
        ("Pistol Ammo", 3)
    ])
});

pub const ITEM_CODES: LazyLock<HashMap<&'static str, i16>> = LazyLock::new(|| {
	HashMap::from([
        ("Key Sword", 4),
        ("Shell Horn", 38),
        ("Twin Statue", 59),
        ("Map", 70),
        ("Holy Grail (Full)", 83),
        ("Weights", 105)
    ])
});

#[derive(Clone, Debug)]
pub struct RcdParams {
    pub param_index: usize,
    pub item_mod: i16
}

pub const RCD_OBJECT_PARAMS: LazyLock<HashMap<i16, RcdParams>> = LazyLock::new(|| {
    HashMap::from([
        (RCD_OBJECTS["chest"], RcdParams { param_index: 0, item_mod: 11 }),
        (RCD_OBJECTS["naked_item"], RcdParams { param_index: 1, item_mod: 0 }),
        (RCD_OBJECTS["instant_item"], RcdParams { param_index: 0, item_mod: 0 }),
        (RCD_OBJECTS["scan"], RcdParams { param_index: 3, item_mod: 0 }),
    ])
});

pub const DOUBLE_CHEST_ADDRESSES: LazyLock<HashMap<i64, i16>> = LazyLock::new(|| {
    HashMap::from([
        (2359125, 45),  // Gate of Guidance Left Coin Chest [00-02-01]
        (2359126, 55),  // Gate of Guidance Right Coin Chest [00-02-01]
        (2359106, 29),  // Gate of Illusion Coin Chest [10-06-00]
        (2359056, 2),   // Gate of Illusion Fairy Clothes Chest [10-06-00]
        (2359040, 10),  // Chamber of Extinction Map Chest [06-03-00]
        (2359101, 19)   // Chamber of Extinction Coin Chest [06-03-00]
    ])
});

pub const ZONES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| {
    vec![
        vec![2,2,2,2,3,1,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![3,2,2,1,3,3,2,2,2,2,4,2,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,1,1,3,2,3,3,1,0,0,0,0,0,0,0,0,0,0],
        vec![2,3,2,1,6,1,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![3,1,2,5,1,2,2,2,2,0,1,1,1,1,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0],
        vec![2,2,3,3,1,2,1,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,2,2,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,3,3,2,2,2,2,2,2,2,3,3,2,2,3,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,4,4,4,4,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,2,2,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,3,1,2,2,1,3,2,2,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![3,2,2,1,4,2,1,2,1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,1,4,2,2,1,1,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,3,2,2,2,4,3,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0],
        vec![3,2,2,2,2,2,2,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,3,0,0,0,0,0,0,0,0],
        vec![2,2,2,1,2,2,1,2,2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,2,2,2,2,2,2,2,2,2,0,0,0,0,0],
        vec![2,3,2,2,2],
        vec![2,3,2,2,2],
        vec![2,0],
        vec![3,2,2,1,3,3,2,2,2,2,4,2,0,0,0,0,0,0,0,0,0,0,0],
        vec![1,2,2,1,2,2,2,1,2,2,2,1,2,1,2,2,1,1,2,1,1,1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![2,1,2,0,0],
        vec![5,5,5,5,0]
    ]
});

pub const FONT: &'static str = "!\"&'(),-./0123456789:?ABCDEFGHIJKLMNOPQRSTUVWXYZ　]^_\
abcdefghijklmnopqrstuvwxyz…♪、。々「」ぁあぃいぅうぇえぉおか\
がきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほ\
ぼぽまみむめもゃやゅゆょよらりるれろわをんァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセ\
ゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリル\
レロワヲンヴ・ー一三上下不与世丘両中丸主乗乙乱乳予争事二人今介仕他付代以仮仲件会伝位低住体何作使\
供侵係保信俺倍倒値偉側偶備傷像僧元兄先光兜入全公具典内再冒冥出刀分切列初別利刻則前剣創力加助効勇\
勉動化匹十半協博印危去参双反取受叡口古召可台史右司合同名向否周呪味呼命品唯唱問喜営器噴四回囲図国\
土在地坂型域基堂報場塊塔墓増壁壇壊士声売壷変外多夜夢大天太央失奇契奥女好妊妖妻始姿娘婦子字存孤学\
宇守官宙定宝実客室宮家密寝対封専導小少尾屋屏属山岩崖崩嵐左巨己布帯帰常年幸幻幾広床底店度座庫廊廟\
弁引弟弱張強弾当形影役彼待後心必忍忘応念怒思急性怨恐息恵悔悟悪悲情惑想意愚愛感慈態憶我戦戻所扉手\
扱投抜押拝拡拳拾持指振探撃撮操支攻放敗教散数敵敷文料斧断新方旅族日早昇明昔星映時晩普晶智暗曲書最\
月有服望未末本杉村杖束来杯板析果架柱査格械棺検椿楼楽槍様槽模樹橋機欠次欲歓止正武歩歯歳歴死殊残段\
殺殿母毒毛気水氷永求汝池決治法波泥注洞洪流海消涙涯深済減湖満源溶滅滝火灯灼炎無然熱爆爪父版牛物特\
犬状狂独獄獅獣玄玉王珠現球理瓶生産用男画界略番発登白百的盤目直盾看真眠着知石研破碑示礼社祈祖神祠\
祭禁福私秘秤移種穴究空突窟立竜章竪端笛符第筒答箱範精系約納純紫細紹終経結続緑練罠罪罰義羽習翻翼老\
考者耐聖聞肉肩胸能脱腕自至船色若苦英荷華落葉蔵薇薔薬蛇血行術衛表裁装裏補製複要見覚親解言記訳証試\
話詳認誕誘語誠説読誰調論謁謎謝識議護谷貝財貧貯買貸資賢贄贖赤走起超足跡路踊蹴身車軽輝辞込辿近返迷\
追送逃通速造連進遊過道達違遠適選遺還郎部配重野量金針鉄銀銃銅録鍵鎖鏡長門閉開間関闇闘防限険陽階隠\
雄雑難雨霊青静面革靴音順領頭題顔願類風飛食館馬駄験骨高魂魔魚鳥鳴黄黒泉居転清成仏拠維視宿浮熟飾冷\
得集安割栄偽屍伸巻緒捨固届叩越激彫蘇狭浅Ⅱ［］：！？～／０１２３４５６７８９ＡＢＣＤＥＦＧＨＩＪ\
ＫＬＭＮＯＰＲＳＴＵＶＷＸＹａｂｄｅｇｈｉｌｍｏｐｒｓｔｕｘ辺薄島異温復称狙豊穣虫絶ＱＺｃｆｊｋ\
ｎｑｖｗｙｚ＋－旧了設更横幅似確置整＞％香ü描園為渡象相聴比較掘酷艇原民雷絵南米平木秋田県湯環砂\
漠角運湿円背負構授輪圏隙草植快埋寺院妙該式判（）警告収首腰芸酒美組各演点勝観編丈夫姫救’，．霧節\
幽技師柄期瞬電購任販Á;û+→↓←↑⓪①②③④⑤⑥⑦⑧⑨<”挑朝痛魅鍛戒飲憂照磨射互降沈醜触煮疲\
素競際易堅豪屈潔削除替Ü♡*$街極ＵＤＦ▲✖■●✕七並久五亜亡交仰余依便修個借倣働儀償優免六共冑\
冠冶凄凍凶刃制刺労勢勿包医卑単厄及吐含吸吹咆和員哮哺商善喰噂噛嚇因団困圧垂執塗塞境奪威婆嫌完害容\
寄寒寛察尋尽峙巡巧差幼建弄彩往徊従徘御微徳徴忌怖怪恨悠慢慮憑憧扇才払抱担拶拷挙挨捕排掛掟接揃揮故\
敏敢旋既旺昂昆春是暑暮暴朽材枚枝染柔株根案棒森業権歌油泳活派浴液測準潜烈烏焼燃爵片牽狩狼猛猟猫献\
猿獲率珍甦由甲病症痩療癒皮益盛監眼睡矛短砕硬磁礁禽秀程穏筋管築簡粉粘糞級給統継綿総線縁縛縦織羅羊\
群耳職肌股肢肪育脂脅脈脚腐膚膜臭致興舞般良花荒葬蛮被裂襲覆討託訪詰諸貢質赦趣距跳軍軟迂迎迫逆透途\
這遅遥避邪都酸銭鋭錬鎌鑑闊阻陥陰陸障離震露非預頼額養騙驚骸髪鱗鶏鹿鼻龍";

pub fn grail_flag_by_zone(zone: usize, frontside: bool) -> i16 {
    match zone {
        0 => GLOBAL_FLAGS["grail_tablet_guidance"],
        1 => GLOBAL_FLAGS["grail_tablet_surface"],
        2 => GLOBAL_FLAGS["grail_tablet_mausoleum"],
        3 => GLOBAL_FLAGS["grail_tablet_sun"],
        4 => GLOBAL_FLAGS["grail_tablet_spring"],
        5 => GLOBAL_FLAGS["grail_tablet_inferno"],
        6 => GLOBAL_FLAGS["grail_tablet_extinction"],
        7 => {
            if frontside {
                GLOBAL_FLAGS["grail_tablet_twin_front"]
            } else {
                GLOBAL_FLAGS["grail_tablet_twin_back"]
            }
        },
        8 => GLOBAL_FLAGS["grail_tablet_endless"],
        9 => GLOBAL_FLAGS["grail_tablet_shrine_front"],
        10 => GLOBAL_FLAGS["grail_tablet_illusion"],
        11 => GLOBAL_FLAGS["grail_tablet_graveyard"],
        12 => GLOBAL_FLAGS["grail_tablet_moonlight"],
        13 => GLOBAL_FLAGS["grail_tablet_goddess"],
        14 => GLOBAL_FLAGS["grail_tablet_ruin"],
        15 | 16 => GLOBAL_FLAGS["grail_tablet_birth"],
        17 => GLOBAL_FLAGS["grail_tablet_dimensional"],
        18 => GLOBAL_FLAGS["grail_tablet_shrine_back"],
        _ => 0xacf
    }
}
