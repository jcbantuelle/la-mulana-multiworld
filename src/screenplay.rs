use lazy_static::lazy_static;

lazy_static! {
    static ref FONT: Vec<char> = "!\"&'(),-./0123456789:?ABCDEFGHIJKLMNOPQRSTUVWXYZ　]^_abcdefghijklmnopqrstuvwxyz…♪、。々「」ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろわをんァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロワヲンヴ・ー一三上下不与世丘両中丸主乗乙乱乳予争事二人今介仕他付代以仮仲件会伝位低住体何作使供侵係保信俺倍倒値偉側偶備傷像僧元兄先光兜入全公具典内再冒冥出刀分切列初別利刻則前剣創力加助効勇勉動化匹十半協博印危去参双反取受叡口古召可台史右司合同名向否周呪味呼命品唯唱問喜営器噴四回囲図国土在地坂型域基堂報場塊塔墓増壁壇壊士声売壷変外多夜夢大天太央失奇契奥女好妊妖妻始姿娘婦子字存孤学宇守官宙定宝実客室宮家密寝対封専導小少尾屋屏属山岩崖崩嵐左巨己布帯帰常年幸幻幾広床底店度座庫廊廟弁引弟弱張強弾当形影役彼待後心必忍忘応念怒思急性怨恐息恵悔悟悪悲情惑想意愚愛感慈態憶我戦戻所扉手扱投抜押拝拡拳拾持指振探撃撮操支攻放敗教散数敵敷文料斧断新方旅族日早昇明昔星映時晩普晶智暗曲書最月有服望未末本杉村杖束来杯板析果架柱査格械棺検椿楼楽槍様槽模樹橋機欠次欲歓止正武歩歯歳歴死殊残段殺殿母毒毛気水氷永求汝池決治法波泥注洞洪流海消涙涯深済減湖満源溶滅滝火灯灼炎無然熱爆爪父版牛物特犬状狂独獄獅獣玄玉王珠現球理瓶生産用男画界略番発登白百的盤目直盾看真眠着知石研破碑示礼社祈祖神祠祭禁福私秘秤移種穴究空突窟立竜章竪端笛符第筒答箱範精系約納純紫細紹終経結続緑練罠罪罰義羽習翻翼老考者耐聖聞肉肩胸能脱腕自至船色若苦英荷華落葉蔵薇薔薬蛇血行術衛表裁装裏補製複要見覚親解言記訳証試話詳認誕誘語誠説読誰調論謁謎謝識議護谷貝財貧貯買貸資賢贄贖赤走起超足跡路踊蹴身車軽輝辞込辿近返迷追送逃通速造連進遊過道達違遠適選遺還郎部配重野量金針鉄銀銃銅録鍵鎖鏡長門閉開間関闇闘防限険陽階隠雄雑難雨霊青静面革靴音順領頭題顔願類風飛食館馬駄験骨高魂魔魚鳥鳴黄黒泉居転清成仏拠維視宿浮熟飾冷得集安割栄偽屍伸巻緒捨固届叩越激彫蘇狭浅Ⅱ［］：！？～／０１２３４５６７８９ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＲＳＴＵＶＷＸＹａｂｄｅｇｈｉｌｍｏｐｒｓｔｕｘ辺薄島異温復称狙豊穣虫絶ＱＺｃｆｊｋｎｑｖｗｙｚ＋－旧了設更横幅似確置整＞％香ü描園為渡象相聴比較掘酷艇原民雷絵南米平木秋田県湯環砂漠角運湿円背負構授輪圏隙草植快埋寺院妙該式判（）警告収首腰芸酒美組各演点勝観編丈夫姫救’，．霧節幽技師柄期瞬電購任販Á;û+→↓←↑⓪①②③④⑤⑥⑦⑧⑨<”挑朝痛魅鍛戒飲憂照磨射互降沈醜触煮疲素競際易堅豪屈潔削除替Ü♡*$街極ＵＤＦ▲✖■●✕七並久五亜亡交仰余依便修個借倣働儀償優免六共冑冠冶凄凍凶刃制刺労勢勿包医卑単厄及吐含吸吹咆和員哮哺商善喰噂噛嚇因団困圧垂執塗塞境奪威婆嫌完害容寄寒寛察尋尽峙巡巧差幼建弄彩往徊従徘御微徳徴忌怖怪恨悠慢慮憑憧扇才払抱担拶拷挙挨捕排掛掟接揃揮故敏敢旋既旺昂昆春是暑暮暴朽材枚枝染柔株根案棒森業権歌油泳活派浴液測準潜烈烏焼燃爵片牽狩狼猛猟猫献猿獲率珍甦由甲病症痩療癒皮益盛監眼睡矛短砕硬磁礁禽秀程穏筋管築簡粉粘糞級給統継綿総線縁縛縦織羅羊群耳職肌股肢肪育脂脅脈脚腐膚膜臭致興舞般良花荒葬蛮被裂襲覆討託訪詰諸貢質赦趣距跳軍軟迂迎迫逆透途這遅遥避邪都酸銭鋭錬鎌鑑闊阻陥陰陸障離震露非預頼額養騙驚骸髪鱗鶏鹿鼻龍".chars().collect();
}

pub unsafe fn decode(address: *const u16, letter_count: i32) -> String {
    std::slice::from_raw_parts(address, letter_count as usize)
        .iter()
        .map(|letter| {
            if *letter == 0x000A {
                "<break>".to_string()
            } else if *letter == 0x0020 {
                " ".to_string()
            } else {
                FONT[(letter - 0x100) as usize].to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

pub unsafe fn encode(word: String) -> Vec<u16> {
    word.chars()
        .map(|letter| {
            if letter == ' ' {
                0x0020
            } else {
                (FONT.iter().position(|&i| i == letter).unwrap() + 0x100) as u16
            }
        })
        .collect::<Vec<u16>>()
}