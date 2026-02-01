use std::collections::HashMap;
use std::sync::LazyLock;
use crate::consts::LAMULANA_EXECUTABLE_NAME;

pub static LAMULANA_EXECUTABLE_NAME_WITH_EXTENSION: LazyLock<String> = LazyLock::new(|| { format!("{}.exe", LAMULANA_EXECUTABLE_NAME) });
const GLOBAL_FLAGS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
    HashMap::from([
        ("screen_flag_00", 0x00),
        ("screen_flag_01", 0x01),
        ("screen_flag_02", 0x02),
        ("screen_flag_0c", 0x0c),
        ("screen_flag_0d", 0x0d),
        ("screen_flag_2e", 0x2e),
        ("screen_flag_2f", 0x2f),
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
        ("rcd_filler_items", 0x9f6),
        ("dat_filler_items", 0xa8)
    ])
});

const INVENTORY: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
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

const HEADERS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
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
        ("anime", 0x004),
    ])
});

const CARDS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
    HashMap::from([
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
        ("xelpud_howling_wind", 104),
    ])
});

const RCD_OBJECTS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
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
        ("mother_ankh", 0xc0),
        ("scan", 0xc),
        ])
});

const TEST_OPERATIONS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
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
        ("nz", 0x4),
    ])
});

const WRITE_OPERATIONS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
    HashMap::from([
        ("assign", 0x0),
        ("add", 0x1),
        ("sub", 0x2),
        ("mult", 0x3),
        ("div", 0x4),
        ("and", 0x5),
        ("or", 0x6),
        ("xor", 0x),
    ])
});


def grail_flag_by_zone(zone, frontside):
    match zone:
        case 0:
            return GLOBAL_FLAGS["grail_tablet_guidance"]
        case 1:
            return GLOBAL_FLAGS["grail_tablet_surface"]
        case 2:
            return GLOBAL_FLAGS["grail_tablet_mausoleum"]
        case 3:
            return GLOBAL_FLAGS["grail_tablet_sun"]
        case 4:
            return GLOBAL_FLAGS["grail_tablet_spring"]
        case 5:
            return GLOBAL_FLAGS["grail_tablet_inferno"]
        case 6:
            return GLOBAL_FLAGS["grail_tablet_extinction"]
        case 7:
            if frontside:
                return GLOBAL_FLAGS["grail_tablet_twin_front"]
            else:
                return GLOBAL_FLAGS["grail_tablet_twin_back"]
        case 8:
            return GLOBAL_FLAGS["grail_tablet_endless"]
        case 9:
            return GLOBAL_FLAGS["grail_tablet_shrine_front"]
        case 10:
            return GLOBAL_FLAGS["grail_tablet_illusion"]
        case 11:
            return GLOBAL_FLAGS["grail_tablet_graveyard"]
        case 12:
            return GLOBAL_FLAGS["grail_tablet_moonlight"]
        case 13:
            return GLOBAL_FLAGS["grail_tablet_goddess"]
        case 14:
            return GLOBAL_FLAGS["grail_tablet_ruin"]
        case 15|16:
            return GLOBAL_FLAGS["grail_tablet_birth"]
        case 17:
            return GLOBAL_FLAGS["grail_tablet_dimensional"]
        case 18:
            return GLOBAL_FLAGS["grail_tablet_shrine_back"]
        case _:
            return 0xacf
