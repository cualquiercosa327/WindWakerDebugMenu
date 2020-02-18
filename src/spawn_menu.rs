use core::fmt::Write;
use libtww::game::actor::ActorTemplate;
use libtww::game::Console;
// use libtww::game::actor;
use libtww::Link;
// use libtww::system;

use crate::controller;
use crate::utils::*;

static mut cursor: usize = 0;

static mut category_index: usize = 0;
static mut actor_index: usize = 0;
static mut param_1: u8 = 0;
static mut param_2: u8 = 0;
static mut param_3: u8 = 0;
static mut param_4: u8 = 0;
static mut flag: u8 = 0;
static mut flag_bit: u8 = 0;
static mut y_offset: f32 = 0.0;

type Actor = (&'static str, &'static str);

static CATEGORIES: [&str; 18] = [
    "Breakables",
    "Doors",
    "Dungeon Bosses",
    "Enemy NPC",
    "Exits / Entrances",
    "Foliage",
    "Friendly NPC",
    "Gameplay",
    "LOD Models",
    "Large Objects",
    "Mechanics",
    "Obstacle",
    "Storyline",
    "Switches",
    "TG Doors",
    "Treasure Chests",
    "Triggers",
    "Uncategorized",
];

static BREAKABLES: &[Actor; 9] = &[
    ("Sign", "Kanban"),
    ("Breakable Cup", "MKoppu"),
    ("Breakable Plate", "MOsara"),
    ("Breakable Jug", "MPot"),
    ("Skull", "Odokuro"),
    ("Nut", "VigaH"),
    ("Pile of Leaves", "Vochi"),
    ("Small Pot", "kotubo"),
    ("Large Pot", "ootubo1"),
];

static DOORS: &[Actor; 2] = &[("KNOB00D", "KNOB00D"), ("KNOB01D", "KNOB01D")];

static DUNGEON_BOSSES: &[Actor; 6] = &[
    ("Kalle Demos", "Bkm"),
    ("Gohdan", "Bst"),
    ("Gohma", "Btd"),
    ("Molgera", "Bwd"),
    ("Ganondorf", "Gnd"),
    ("Jalhalla", "big_pow"),
];

static ENEMY_NPC: &[Actor; 32] = &[
    ("Kargaroc", "Bb"),
    ("Bokoblin", "Bk"),
    ("Quill", "Bm1"),
    ("Canon", "Canon"),
    ("Big Octo", "Daiocta"),
    ("Phantom Ganon", "Fganon"),
    ("Fire Keese", "Fkeeth"),
    ("Floor Master (2)", "Fmastr2"),
    ("Gyorg", "GyCtrl"),
    ("Redead", "Rdead1"),
    ("Dexivine", "Sss"),
    ("Stalfos", "Stal"),
    ("Darknut", "Tn"),
    ("Blade trap", "Trap"),
    ("Armos", "amos"),
    ("Armos 2", "amos2"),
    ("Bubble", "bable"),
    ("Boko Baba", "bbaba"),
    ("Black Chuchu", "c_black"),
    ("Blue Chuchu", "c_blue"),
    ("Green Chuchu", "c_green"),
    ("Yellow Chuchu", "c_kiiro"),
    ("Red Chuchu", "c_red"),
    ("Keese", "keeth"),
    ("Magtail", "magtail"),
    ("Moblin", "mo2"),
    ("Moblin Statue", "moZOU"),
    ("Mouse", "nezumi"),
    ("Peahat", "p_hat"),
    ("Poe", "pow"),
    ("Redead (1)", "rdead1"),
    ("Regular Wizzrobe", "wiz_r"),
];

static EXITS_ENTRANCES: &[Actor; 3] = &[
    ("Door (0)", "KNOB00"),
    ("Door (1)", "KNOB01"),
    ("Grotto Entrance", "Pitfall"),
];

static FOLIAGE: &[Actor; 13] = &[
    ("Palm Tree", "Oyashi"),
    ("Flower", "flower"),
    ("flwr17", "flwr17"),
    ("flwr7", "flwr7"),
    ("Small Rock (1)", "koisi1"),
    ("kusax1", "kusax1"),
    ("kusax21", "kusax21"),
    ("kusax7", "kusax7"),
    ("Large Tree", "lwood"),
    ("Large Tree", "lwood"),
    ("pflwrx7", "pflwrx7"),
    ("Small Tree (3)", "swood3"),
    ("Small Tree (5)", "swood5"),
];

static FRIENDLY_NPC: &[Actor; 13] = &[
    ("Sturgeon", "Aj1"),
    ("Grandma", "Ba1"),
    ("Great Fairy", "BigElf"),
    ("Rito Postman (2)", "Bm2"),
    ("Rito Postman (4)", "Bm4"),
    ("Rito Postman (5)", "Bm5"),
    ("Makar", "Cb1"),
    ("Seagull", "Kamome"),
    ("Aryll", "Ls1"),
    ("Medli", "Md1"),
    ("Pig", "Pig"),
    ("Tetra", "Zl1"),
    ("Crab", "kani"),
];

static GAMEPLAY: &[Actor; 22] = &[
    ("Attention Grabber", "AttTag"),
    ("Bomb Flower", "BFlower"),
    ("Heart Container (Dungeon Boss Item Drop)", "Bitem"),
    ("Valoo's Tail", "Dr2"),
    ("Hookshot Target", "Hfuck1"),
    ("Breakable Floor Tile", "Hhyu1"),
    ("Spring on a Block (2)", "Hjump2"),
    ("Wind Column Generator", "Hsen1"),
    ("Grapple Point", "Kui"),
    ("Grapple Point", "Kui"),
    ("Solidified Magma Platform", "Magrock"),
    ("Wooden Box with Black Frame", "Ospbox"),
    ("Dangling Rope with Lantern", "RopeR"),
    ("Postbox", "Tpost"),
    ("Warp Jar (2)", "Warpts2"),
    ("Jet of Magma", "Yfire00"),
    ("Ring of Fire", "Zenfire"),
    ("Bridge", "bridge"),
    ("Collectible Item", "item"),
    ("Baba Bud", "jbaba"),
    ("Pushable Block (0)", "osiBLK0"),
    ("Pushable Block (1)", "osiBLK1"),
];

static LOD_MODELS: &[Actor; 49] = &[
    ("Forsaken Fortress", "LOD01"),
    ("Star Island", "LOD02"),
    ("Northern Fairy Isle", "LOD03"),
    ("Gale Island", "LOD04"),
    ("Crescent Moon Isle", "LOD05"),
    ("Seven-Star Isles", "LOD06"),
    ("Overlook Island", "LOD07"),
    ("Four-Eye Reef", "LOD08"),
    ("Mother and Child Isles", "LOD09"),
    ("Spectacle Island", "LOD10"),
    ("Windfall Island", "LOD11"),
    ("Pawprint Isle", "LOD12"),
    ("Dragon Roost Island", "LOD13"),
    ("Flight Control Platform", "LOD14"),
    ("Western Fairy Isle", "LOD15"),
    ("Rock Spire Isle", "LOD16"),
    ("Tingle Island", "LOD17"),
    ("Northern Triangle Island", "LOD18"),
    ("Eastern Fairy Isle", "LOD19"),
    ("Fire Mountain", "LOD20"),
    ("Star Belt Archipelago", "LOD21"),
    ("Three-Eye Reef", "LOD22"),
    ("Greatfish Isle", "LOD23"),
    ("Cyclops Reef", "LOD24"),
    ("Six-Eye Reef", "LOD25"),
    ("Tower of the Gods", "LOD26"),
    ("Eastern Triangle Island", "LOD27"),
    ("Thorned Fairy Isle", "LOD28"),
    ("Needlepoint Island", "LOD29"),
    ("Islet of Steel", "LOD30"),
    ("Stone Watcher Island", "LOD31"),
    ("Southern Triangle Island", "LOD32"),
    ("Private Oasis", "LOD33"),
    ("Bomb Island", "LOD34"),
    ("Bird's Peak Island", "LOD35"),
    ("Diamond Steppe Island", "LOD36"),
    ("Five-Eye Reef", "LOD37"),
    ("Shark Island", "LOD38"),
    ("Southern Fairy Isle", "LOD39"),
    ("Ice Ring Isle", "LOD40"),
    ("Forest Haven", "LOD41"),
    ("Cliff Plateau Isles", "LOD42"),
    ("Horseshoe Island", "LOD43"),
    ("Outset Island", "LOD44"),
    ("Headstone Island", "LOD45"),
    ("Two-Eye Reef", "LOD46"),
    ("Angular Isles", "LOD47"),
    ("Boat Race Island", "LOD48"),
    ("Five-Star Isles", "LOD49"),
];

static LARGE_OBJECTS: &[Actor; 5] = &[
    ("Stall A", "RotenA"),
    ("Stall B", "RotenB"),
    ("Stall C", "RotenC"),
    ("Tower of the Gods Exterior", "X_tower"),
    ("Link Statue (Inside Hyrule Castle)", "YLzou"),
];

static MECHANICS: &[Actor; 1] = &[("Seed planting spot for Makar", "VmcBS")];

static OBSTACLE: &[Actor; 5] = &[
    ("Iron Bars", "Ashut"),
    ("Large Rock", "Ebrock"),
    ("Spike", "Htoge1"),
    ("Eye-Vine Blocker", "Ss"),
    ("Tingle", "Tc"),
];

static STORYLINE: &[Actor; 6] = &[
    ("Triangle Island Statue", "Doguu"),
    ("Zephos / Cyclos", "Hr"),
    ("Din Statue", "MegamiD"),
    ("Farore Statue", "MegamiF"),
    ("Nayru Statue", "MegamiN"),
    ("Ganon's Tower 4-Boss Door", "VgnFD"),
];

static SWITCHES: &[Actor; 9] = &[
    ("\"All Enemies Killed\" Switch", "ALLdie"),
    ("Switch Buffer (0)", "AND_SW0"),
    ("Switch Buffer (2)", "AND_SW2"),
    ("Wind Switch", "Hpbot1"),
    ("Floor Switch (A)", "Kbota_A"),
    ("Proximity Switch", "SW_C00"),
    ("Crystal Switch", "SW_HIT0"),
    ("Wind Waker Song Switch (B)", "SWtactB"),
    ("Tingle C Switch", "agbCSW"),
];

static TG_DOORS: &[Actor; 12] = &[
    ("KNOB00D", "KNOB00D"),
    ("KNOB01D", "KNOB01D"),
    ("KNOB03D", "KNOB03D"),
    ("ZenS12", "ZenS12"),
    ("Dungeon Barred Door", "Zenshut"),
    ("Normal Dungeon Door", "door10"),
    ("Normal Earth/Wind Temple Door", "door12"),
    ("Boss Dungeon Door", "door20"),
    ("Forbidden Woods Boss Door", "doorKD"),
    ("Barred Earth/Wind Temple Door", "doorSH"),
    ("Locked Earth/Wind Temple Door", "keyS12"),
    ("Dungeon Locked Door", "keyshut"),
];

static TREASURE_CHESTS: &[Actor; 20] = &[
    ("Treasure Chest", "takara"),
    ("Treasure Chest (2)", "takara2"),
    ("takara3", "takara3"),
    ("Treasure Chest (3)", "takara3"),
    ("Treasure Chest (4)", "takara4"),
    ("Treasure Chest (5)", "takara5"),
    ("Treasure Chest (6)", "takara6"),
    ("Treasure Chest (7)", "takara7"),
    ("Treasure Chest (8)", "takara8"),
    ("Treasure I", "takaraI"),
    ("Treasure K", "takaraK"),
    ("Treasure M", "takaraM"),
    ("Treasure AGc", "tkrAGc"),
    ("Treasure AIk", "tkrAIk"),
    ("Treasure AKd", "tkrAKd"),
    ("Treasure AOc", "tkrAOc"),
    ("Treasure AOs", "tkrAOs"),
    ("Treasure A Switch", "tkrASw"),
    ("Treasure Chest Unlocked by Light Beam", "tkrBMs"),
    ("Treasure CTf", "tkrCTf"),
];

static TRIGGERS: &[Actor; 10] = &[
    ("Event Trigger", "TagEv"),
    ("Hint Trigger", "TagHt"),
    ("Hint Trigger (2)", "TagHt2"),
    ("Text Event Trigger", "TagMsg"),
    ("Weather Trigger (0)", "ky_tag0"),
    ("Weather Trigger (1)", "ky_tag1"),
    ("Weather Trigger (2)", "ky_tag2"),
    ("Weather Trigger (3)", "ky_tag3"),
    ("Weather Trigger (4)", "kytag4"),
    ("Weather Trigger (6)", "kytag6"),
];

static UNCATEGORIZED: &[Actor; 452] = &[
    ("Prison Door", "ATdoor"),
    ("Ac1", "Ac1"),
    ("Old Man Ho Ho", "Ah"),
    ("Invisible Wall", "Akabe"),
    ("Akabe10", "Akabe10"),
    ("Cabana Puzzle", "Apzl"),
    ("Astop", "Astop"),
    ("Attention Grabber (B)", "AttTagB"),
    ("Bokoblin Sea Platform", "Aygr"),
    ("Ayush", "Ayush"),
    ("BLK_CR", "BLK_CR"),
    ("Helmaroc King Object Gibs", "Bdkobj"),
    ("Wooden Platform", "Bita"),
    ("Bj1", "Bj1"),
    ("Bj2", "Bj2"),
    ("Bj3", "Bj3"),
    ("Bj4", "Bj4"),
    ("Bj5", "Bj5"),
    ("Bj6", "Bj6"),
    ("Bj7", "Bj7"),
    ("Bj8", "Bj8"),
    ("Bj9", "Bj9"),
    ("Blift", "Blift"),
    ("Rito Basht", "Bm3"),
    ("Bmcon1", "Bmcon1"),
    ("Bmcon2", "Bmcon2"),
    ("Rito Koboli", "Bmsw"),
    ("Bs1", "Bs1"),
    ("Bs2", "Bs2"),
    ("Btsw2", "Btsw2"),
    ("Cafe Lamp", "Cafelmp"),
    ("CmTrap", "CmTrap"),
    ("Comali in his Bed", "Co1"),
    ("Com_A", "Com_A"),
    ("Com_C", "Com_C"),
    ("CrTrM1", "CrTrM1"),
    ("CrTrM2", "CrTrM2"),
    ("CrTrS3", "CrTrS3"),
    ("CrTrS4", "CrTrS4"),
    ("CrTrS5", "CrTrS5"),
    ("DBLK0", "DBLK0"),
    ("DKkiba", "DKkiba"),
    ("Demo Helmaroc", "Demo_Dk"),
    ("Dk", "Dk"),
    ("Doc Bandam", "Ds1"),
    ("Breakable Wooden Gate", "Dsaku"),
    ("Eayogn", "Eayogn"),
    ("Bombable Statue", "Ebomzo"),
    ("Ebrock2", "Ebrock2"),
    ("Ecube", "Ecube"),
    ("Ecube", "Ecube"),
    ("Ekao", "Ekao"),
    ("Wind Temple Entrance Stone", "Ekskz"),
    ("Wind's Requiem Stone", "Esekh"),
    ("Wind's Requiem Stone (2)", "Esekh2"),
    ("Huge Rock", "Eskban"),
    ("Evsw", "Evsw"),
    ("FTree", "FTree"),
    ("F Platform (Flight Platform?)", "Fdai"),
    ("Figurine Pillar", "Figure"),
    ("Fire Ring", "Fire"),
    ("Floor Master", "Fmaster"),
    ("Floor Master (1)", "Fmastr1"),
    ("Sploosh Kaboom Board", "GBoard"),
    ("Gaship1", "Gaship1"),
    ("Forsaken Fortress 3", "Gaship2"),
    ("Gbrg00", "Gbrg00"),
    ("Gdemo20", "Gdemo20"),
    ("Red Flag", "Gflag"),
    ("Yellow Ocean Warp", "Ghrwp"),
    ("Huge Tornado", "GiceL"),
    ("Gk1", "Gk1"),
    ("Magic Staircase", "Gkai00"),
    ("Gnbtaki", "Gnbtaki"),
    ("Gntakie", "Gntakie"),
    ("Gntakis", "Gntakis"),
    ("Gp1", "Gp1"),
    ("Gryw00", "Gryw00"),
    ("Water pouring down", "Gtaki"),
    ("GyCtrlB", "GyCtrlB"),
    ("Wind Platform", "Hami1"),
    ("Rotatable Wind Platform", "Hami2"),
    ("Vertical Rotatable Wind Platform", "Hami3"),
    ("Large Wind Platform", "Hami4"),
    ("Yet another Wind Platform", "HamiY"),
    ("Wooden Box with Pirate Flag", "Hbox1"),
    ("More Hungry Box", "Hbox2"),
    ("Hbox2S", "Hbox2S"),
    ("Wind Temple Escalator", "Hbrf1"),
    ("Tower of the Gods Statue", "Hcbh"),
    ("Tower of the Gods Platform (1)", "Hdai1"),
    ("Tower of the Gods Platform (2)", "Hdai2"),
    ("Tower of the Gods Platform (3)", "Hdai3"),
    ("Tower of the Gods Magic Switch (A)", "Hfbot1A"),
    ("Tower of the Gods Magic Switch (B)", "Hfbot1B"),
    ("Tower of the Gods Magic Switch (C)", "Hfbot1C"),
    ("Tower of the Gods Waterfall", "Hha"),
    ("Iron Boots Switch", "Hhbot1"),
    ("Iron Boots Switch (N)", "Hhbot1N"),
    ("Spring on a Block (1)", "Hjump1"),
    ("Wind Temple Moving Spikes", "Hkikai1"),
    ("Hmlif", "Hmlif"),
    ("Tower of the Gods Magical Chess Piece (1)", "Hmon1"),
    ("Tower of the Gods Magical Chess Piece (1d)", "Hmon1d"),
    ("Tower of the Gods Magical Chess Piece (2)", "Hmon2"),
    ("Tower of the Gods Magical Chess Piece (2d)", "Hmon2d"),
    ("Beamos (1)", "Hmos1"),
    ("Beamos (2)", "Hmos2"),
    ("Beamos (3)", "Hmos3"),
    ("Mrs. Marie", "Ho"),
    ("Iron Boots + Hookshot Statue (1)", "Homen1"),
    ("Iron Boots + Hookshot Statue (2)", "Homen2"),
    ("Wind Temple Propellor (1)", "Hpu1"),
    ("Wind Temple Propellor (2)", "Hpu2"),
    ("Hr2", "Hr2"),
    ("Tower of the Gods Talking Magical Chess Piece", "Hseki1"),
    ("Hseki2", "Hseki2"),
    ("Hseki3", "Hseki3"),
    ("Hseki4", "Hseki4"),
    ("Hseki5", "Hseki5"),
    ("Hseki6", "Hseki6"),
    ("Hseki7", "Hseki7"),
    ("Wind Temple Fan (2)", "Hsen2"),
    ("Wind Temple Fan (3)", "Hsen3"),
    ("Hsh", "Hsh"),
    ("Stone with Text", "Hsh2"),
    ("Tower of the Gods Golden Gate", "Htetu1"),
    ("Tower of the Gods Face Wall (1)", "Htobi1"),
    ("Tower of the Gods Face Wall (2)", "Htobi2"),
    ("Wind Temple Checkerboard Wall", "Htobi3"),
    ("Ocean Water (0)", "Humi0z"),
    ("Ocean Water (2)", "Humi2z"),
    ("Ocean Water (3)", "Humi3z"),
    ("Ocean Water (4)", "Humi4z"),
    ("Ocean Water (5)", "Humi5z"),
    ("HyoiKam", "HyoiKam"),
    ("Shootable Eye", "Hys"),
    ("Shootable Eye (2)", "Hys2"),
    ("Hyuf1", "Hyuf1"),
    ("Hyuf2", "Hyuf2"),
    ("ITat00", "ITat00"),
    ("Ocean Bokoblin Platform", "Ikada"),
    ("Anchor", "Ikari"),
    ("Ice", "Ikori"),
    ("Ji1", "Ji1"),
    ("Forsaken Fortress Gate", "KGBdor"),
    ("Outset Door (2)", "KNOB02"),
    ("Outset Door (3)", "KNOB03"),
    ("Leafes and Branches", "Kanat"),
    ("Switch (C)", "KbotaC"),
    ("Switch (B)", "Kbota_B"),
    ("Mila's Father", "Kf1"),
    ("Salvatore (1)", "Kg1"),
    ("Salvatore (2)", "Kg2"),
    ("Red Swimming Flower Platform", "Kita"),
    ("Kk1", "Kk1"),
    ("Kkiba", "Kkiba"),
    ("KkibaB", "KkibaB"),
    ("Forbbiden Woods Lift", "Klft"),
    ("Mila", "Km1"),
    ("Kmi00", "Kmi00"),
    ("Kmi02", "Kmi02"),
    ("Kmtub", "Kmtub"),
    ("Ko1", "Ko1"),
    ("Joel", "Ko2"),
    ("Cyan Swimming Flower Platform", "Kokiie"),
    ("Kp1", "Kp1"),
    ("Krock00", "Krock00"),
    ("Rotatable Wind Cavern Platform", "Kryu00"),
    ("Breakable Wooden Gate", "Ksaku"),
    ("Ktaru", "Ktaru"),
    ("Barrel without Collision", "Ktaruo"),
    ("Pirate Barrel", "Ktarur"),
    ("Barrel", "Ktarux"),
    ("Reflectable Light Beam (0)", "LTag0"),
    ("Reflectable Light Beam (1)", "LTag1"),
    ("LTagR0", "LTagR0"),
    ("Lantern", "Lamp"),
    ("Grave (2)", "MKanok2"),
    ("Grave (e)", "MKanoke"),
    ("Red Skull Flag", "Mcrtn"),
    ("Mcube", "Mcube"),
    ("Mcube10", "Mcube10"),
    ("Mcyln", "Mcyln"),
    ("DRC Rope Platform", "Mflft"),
    ("Skull Hammer Switch", "MhmrSW0"),
    ("Yellow Ladder (12)", "Mhsg12"),
    ("Yellow Ladder (15)", "Mhsg15"),
    ("Yellow Ladder (4h)", "Mhsg4h"),
    ("Yellow Ladder (6)", "Mhsg6"),
    ("Yellow Ladder (9)", "Mhsg9"),
    ("Forsaken Fortress Outer Gate", "MjDoor"),
    ("Ivan", "Mk"),
    ("Falling down Stair", "Mkdan1"),
    ("Earth Temple Light Statue", "MkieBA"),
    ("Earth Temple Light Statue (B)", "MkieBAB"),
    ("MkieBB", "MkieBB"),
    ("Earth Temple Light Wall", "MkieK"),
    ("Earth Gods Lyric Song Stone", "MknjD"),
    ("Mmrr", "Mmrr"),
    ("Mmusic", "Mmusic"),
    ("Mn", "Mn"),
    ("Small Wooden Fence", "Mori1"),
    ("MpwrB", "MpwrB"),
    ("Earth Temple Movable Stairs", "Msdan"),
    ("Earth Temple Movable Stairs (2)", "Msdan2"),
    ("Sun and Moon Mask", "MsuSW"),
    ("Sund and Moon Mask (B)", "MsuSWB"),
    ("Swinging Wooden Platform", "Mswing"),
    ("Mt", "Mt"),
    ("White Flag", "MtFlag"),
    ("Bird Nest", "MtoriSU"),
    ("Triangular Prism Block", "MtryB"),
    ("Triangular Prism Block Target Location", "MtryBCr"),
    ("MwtrSB", "MwtrSB"),
    ("MygnSB", "MygnSB"),
    ("NBOX", "NBOX"),
    ("NBOX10", "NBOX10"),
    ("Forest Firefly", "Nh"),
    ("NpcSo", "NpcSo"),
    ("Rat Trap", "Nzfall"),
    ("Outset Rose", "Ob1"),
    ("Timer", "ObjTime"),
    ("Side of Canon", "Ocanon"),
    ("Huge Cloud Ring", "Ocloud"),
    ("Hatch", "Ohatch"),
    ("Deku Tree's Bulb", "Ojtree"),
    ("Okioke", "Okioke"),
    ("Deku Tree Lift", "Olift"),
    ("Octorok", "Oq"),
    ("Octorok (w)", "Oqw"),
    ("Tower of the Gods Magic Statue", "Os"),
    ("Tower of the Gods Magic Statue (1)", "Os1"),
    ("Tower of the Gods Magic Statue (2)", "Os2"),
    ("Oship", "Oship"),
    ("Ostool", "Ostool"),
    ("Small Wooden Platform", "Otana"),
    ("Table", "Otble"),
    ("Table (L)", "OtbleL"),
    ("Owater", "Owater"),
    ("P1a", "P1a"),
    ("P1b", "P1b"),
    ("Pirate Zuko (a)", "P2a"),
    ("Pirate Zuko (b)", "P2b"),
    ("Pirate Zuko (c)", "P2c"),
    ("PScnChg", "PScnChg"),
    ("Paper", "Paper"),
    ("Flat Wooden Platform", "Pbco"),
    ("Ceiling Fan", "Pbka"),
    ("Maggie's Father", "Pf1"),
    ("Pirates", "Pirates"),
    ("Paper (Piwa)", "Piwa"),
    ("Plant", "Plant"),
    ("Maggie", "Pm1"),
    ("Po", "Po"),
    ("Paper (Ppos)", "Ppos"),
    ("Ptco", "Ptco"),
    ("Ptcu", "Ptcu"),
    ("Ptubo", "Ptubo"),
    ("Puti", "Puti"),
    ("Qdghd", "Qdghd"),
    ("Tingle's Tower Head", "Qtkhd"),
    ("Quake", "Quake"),
    ("Cloud Ring", "Rcloud"),
    ("Rdead2", "Rdead2"),
    ("Void when Hitting Water Tag", "ReTag0"),
    ("Luxurious Flowers", "Rflw"),
    ("Rforce", "Rforce"),
    ("Roten2", "Roten2"),
    ("Roten3", "Roten3"),
    ("Roten4", "Roten4"),
    ("Forsaken Fortress Gate", "SMBdor"),
    ("Helmaroc Spikes", "SMtoge"),
    ("SPitem", "SPitem"),
    ("Iron Doors", "SWTdoor"),
    ("SWat00", "SWat00"),
    ("Wind Waker Song Switch", "SWtact"),
    ("Chandelier", "SYAN"),
    ("Windfall Guy 1", "Sa1"),
    ("Windfall Guy 2", "Sa2"),
    ("Windfall Guy 3", "Sa3"),
    ("Windfall Guy 4", "Sa4"),
    ("Windfall Guy 5", "Sa5"),
    ("SalvFM", "SalvFM"),
    ("Salvag2", "Salvag2"),
    ("SalvagE", "SalvagE"),
    ("SalvagN", "SalvagN"),
    ("Salvage", "Salvage"),
    ("Sarace", "Sarace"),
    ("Forsaken Fortress Bokoblin Light Tower", "Search"),
    ("Fairy", "Sfairy"),
    ("King of Red Lions (Ship Form) (Prop?)", "Ship"),
    ("Spiked Skull Hammer Switch", "Shmrgrd"),
    ("White Flag", "SieFlag"),
    ("Nut Baba", "Sitem"),
    ("Windmill", "Skanran"),
    ("Stdoorl", "Stdoorl"),
    ("Stdoorr", "Stdoorr"),
    ("Stgate", "Stgate"),
    ("Lighthouse Light", "Stoudai"),
    ("Sttoge", "Sttoge"),
    ("Diver (0)", "Sv0"),
    ("Diver (1)", "Sv1"),
    ("Diver (2)", "Sv2"),
    ("Diver (3)", "Sv3"),
    ("Bokoblin Ocean Platform", "Svsp"),
    ("Salvage Switch", "SwSlvg"),
    ("Round Table", "Table"),
    ("TagCb1", "TagCb1"),
    ("TagCb11", "TagCb11"),
    ("TagCb12", "TagCb12"),
    ("TagCb13", "TagCb13"),
    ("TagCb14", "TagCb14"),
    ("TagD1", "TagD1"),
    ("TagD2", "TagD2"),
    ("TagD3", "TagD3"),
    ("TagD4", "TagD4"),
    ("TagDM", "TagDM"),
    ("TagIsl", "TagIsl"),
    ("TagKb", "TagKb"),
    ("TagMSo", "TagMSo"),
    ("TagMd", "TagMd"),
    ("TagMd1", "TagMd1"),
    ("TagMd11", "TagMd11"),
    ("TagMd12", "TagMd12"),
    ("TagMd13", "TagMd13"),
    ("TagMd14", "TagMd14"),
    ("TagMd15", "TagMd15"),
    ("TagMd16", "TagMd16"),
    ("TagMk", "TagMk"),
    ("TagPo", "TagPo"),
    ("TagSo", "TagSo"),
    ("TagWp", "TagWp"),
    ("Grey Platform without Collision", "Tenmado"),
    ("TestPo", "TestPo"),
    ("Laser Barrier (Iniside Hyrule Castle)", "TnTrap"),
    ("Tpota", "Tpota"),
    ("Red Flag", "TrFlag"),
    ("Kalle Demos Trials Spikes", "Turu"),
    ("Kalle Demos Trials Platform 1", "Turu2"),
    ("Kalle Demos Trials Platform 2", "Turu3"),
    ("Pompie", "Ub1"),
    ("Vera", "Ub2"),
    ("Missy", "Ub3"),
    ("Minenco", "Ub4"),
    ("Joanna", "Ug1"),
    ("Potova", "Ug2"),
    ("Um1", "Um1"),
    ("Anton", "Um2"),
    ("Um3", "Um3"),
    ("Sam", "Uo1"),
    ("Gossack", "Uo2"),
    ("Garrickson", "Uo3"),
    ("Makar Seed Plant", "Usovmc"),
    ("Gillian", "Uw1"),
    ("Linda", "Uw2"),
    ("Bomb Flower", "VbakH"),
    ("Pirate Bell", "Vdora"),
    ("Earth Temple Light Face", "Vds"),
    ("Phantom Ganon Maze Door", "Vfan"),
    ("Pedastal of Time", "VmsDZ"),
    ("Master Sword (Model from Hyrule Castle Basement)", "VmsMS"),
    ("VolTag", "VolTag"),
    ("Wind Switch", "Vpbot"),
    ("Puppet Ganon Platform", "Vteng"),
    ("Vtil1", "Vtil1"),
    ("Vtil2", "Vtil2"),
    ("Vtil3", "Vtil3"),
    ("Vtil4", "Vtil4"),
    ("Vtil5", "Vtil5"),
    ("Bent Palm Tree", "Vyasi"),
    ("WLvTag", "WLvTag"),
    ("Breakable Wall", "Wall"),
    ("Dungeon Warp Exit", "Warpf"),
    ("Warpfo", "Warpfo"),
    ("Warpgn", "Warpgn"),
    ("Warpnt", "Warpnt"),
    ("Warpt", "Warpt"),
    ("Warp Jar (1)", "Warpts1"),
    ("Warp Jar (3)", "Warpts3"),
    ("Tornado", "WindTag"),
    ("Tornado (Ybgaf)", "Ybgaf00"),
    ("Boiling Bubbles", "Yboil00"),
    ("Magical Barrier", "Ycage00"),
    ("Light Shaft", "Yfrlt00"),
    ("Magical Floor Ring", "Ygcwp"),
    ("Water Puddle (Ygstp00)", "Ygstp00"),
    ("Water Puddle (Ygush00)", "Ygush00"),
    ("Water Puddle (Ygush01)", "Ygush01"),
    ("Water Puddle (Ygush02)", "Ygush02"),
    ("YkgrOFF", "YkgrOFF"),
    ("YkgrON", "YkgrON"),
    ("Fire Mountain", "Ykzyg"),
    ("Ice Platform (Ylkic)", "Ylkic"),
    ("Ice Platform (Yllic)", "Yllic"),
    ("Ice Platform (Yllic)", "Yllic"),
    ("Ice Platform (Ylsic)", "Ylsic"),
    ("Mesa", "Ym1"),
    ("Abe", "Ym2"),
    ("Shaft of Light Warp", "Ysdls00"),
    ("Huge Tornado", "Ytrnd00"),
    ("Sue-Belle", "Yw1"),
    ("Ywarp00", "Ywarp00"),
    ("Zk1", "Zk1"),
    ("agbA", "agbA"),
    ("agbA2", "agbA2"),
    ("agbAT", "agbAT"),
    ("agbB", "agbB"),
    ("agbD", "agbD"),
    ("agbF", "agbF"),
    ("agbF2", "agbF2"),
    ("agbFA", "agbFA"),
    ("agbMARK", "agbMARK"),
    ("agbMW", "agbMW"),
    ("agbR", "agbR"),
    ("agbTBOX", "agbTBOX"),
    ("Golden Torch", "bonbori"),
    ("Burnt Ganondorf's House", "dmgroom"),
    ("Valoo", "dragon"),
    ("frock", "frock"),
    ("gmos", "gmos"),
    ("Environmental Firefly", "ho"),
    ("Ocean Bokoblin Platform", "ikadaS"),
    ("Beedle's Shopship", "ikada_h"),
    ("ikada_u", "ikada_u"),
    ("Bird", "kt"),
    ("Morth (s)", "kuro_s"),
    ("Morth (t)", "kuro_t"),
    ("ky00you", "ky00you"),
    ("kytag00", "kytag00"),
    ("Snowfall", "kytag5"),
    ("Mouse Hole", "nezuana"),
    ("pflower", "pflower"),
    ("pflower", "pflower"),
    ("Spiky Tentacle", "s_turu"),
    ("sea", "sea"),
    ("speakun", "speakun"),
    ("spotbx1", "spotbx1"),
    ("swood", "swood"),
    ("swood3", "swood3"),
    ("Bush", "woodb"),
    ("Bush (x)", "woodbx"),
    ("Knight Statue", "zouK"),
    ("Knight Statue (1)", "zouK1"),
    ("Knight Statue (2)", "zouK2"),
    ("Knight Statue (3)", "zouK3"),
    ("Knight Statue (4)", "zouK4"),
    ("Triforce Flag", "HcFlag"),
    ("Forsaken Fortress Flag", "MjFlag"),
    ("Jet of Steam (0)", "Ystm0"),
    ("Jet of Steam (1)", "Ystm1"),
    ("Magma", "magma"),
];

fn get_actors_for_category(index: usize) -> &'static [(&'static str, &'static str)] {
    match index {
        0 => BREAKABLES,
        1 => DOORS,
        2 => DUNGEON_BOSSES,
        3 => ENEMY_NPC,
        4 => EXITS_ENTRANCES,
        5 => FOLIAGE,
        6 => FRIENDLY_NPC,
        7 => GAMEPLAY,
        8 => LOD_MODELS,
        9 => LARGE_OBJECTS,
        10 => MECHANICS,
        11 => OBSTACLE,
        12 => STORYLINE,
        13 => SWITCHES,
        14 => TG_DOORS,
        15 => TREASURE_CHESTS,
        16 => TRIGGERS,
        17 => UNCATEGORIZED,
        _ => unreachable!(),
    }
}

pub fn transition_into() {}

pub fn render() {
    const MENU_ITEM_CATEGORY: usize = 0;
    const MENU_ITEM_ACTOR: usize = 1;
    const MENU_ITEM_PARAM_1: usize = 2;
    const MENU_ITEM_PARAM_2: usize = 3;
    const MENU_ITEM_PARAM_3: usize = 4;
    const MENU_ITEM_PARAM_4: usize = 5;
    const MENU_ITEM_FLAG: usize = 6;
    const MENU_ITEM_FLAG_BIT: usize = 7;
    const MENU_ITEM_Y_OFFSET: usize = 8;
    const MENU_ITEM_SPAWN: usize = 10;
    const MENU_ITEM_X: usize = 14;
    const MENU_ITEM_Y: usize = 15;
    const MENU_ITEM_Z: usize = 16;
    const MENU_ITEM_ROTATION: usize = 17;

    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Spawn Menu");
    let _ = write!(lines[1].begin(), "==========");

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    let contents = [
        "Category: ",
        "Actor:    ",
        "Param 1:  ",
        "Param 2:  ",
        "Param 3:  ",
        "Param 4:  ",
        "Flag:     ",
        "Flag Bit: ",
        "Y Offset: ",
        "",
        "Spawn",
        "",
        "",
        "",
        "X: ",
        "Y: ",
        "Z: ",
        "R: ",
    ];

    move_cursor(MENU_ITEM_SPAWN + 1, unsafe { &mut cursor });

    let actors: &[Actor] = get_actors_for_category(unsafe { category_index });
    let category_name = CATEGORIES[unsafe { category_index }];
    let (visible_actor_name, actual_actor_name) = actors[unsafe { actor_index }];

    let mut coord = Link::position().clone();
    coord.y += unsafe { y_offset };

    match unsafe { cursor } {
        MENU_ITEM_CATEGORY => unsafe {
            if dpad_left && category_index > 0 {
                category_index -= 1;
                actor_index = 0;
            } else if dpad_right && category_index + 1 < CATEGORIES.len() {
                category_index += 1;
                actor_index = 0;
            }
        },
        MENU_ITEM_ACTOR => unsafe {
            if dpad_left && actor_index > 0 {
                actor_index -= 1;
            } else if dpad_right && actor_index + 1 < actors.len() {
                actor_index += 1;
            }
        },
        MENU_ITEM_PARAM_1 => unsafe {
            if dpad_left {
                param_1 -= 1;
            } else if dpad_right {
                param_1 += 1;
            }
        },
        MENU_ITEM_PARAM_2 => unsafe {
            if dpad_left {
                param_2 -= 1;
            } else if dpad_right {
                param_2 += 1;
            }
        },
        MENU_ITEM_PARAM_3 => unsafe {
            if dpad_left {
                param_3 -= 1;
            } else if dpad_right {
                param_3 += 1;
            }
        },
        MENU_ITEM_PARAM_4 => unsafe {
            if dpad_left {
                param_4 -= 1;
            } else if dpad_right {
                param_4 += 1;
            }
        },
        MENU_ITEM_FLAG => unsafe {
            if dpad_left {
                flag -= 1;
            } else if dpad_right {
                flag += 1;
            }
        },
        MENU_ITEM_FLAG_BIT => unsafe {
            if dpad_left {
                flag_bit -= 1;
            } else if dpad_right {
                flag_bit += 1;
            }
        },
        MENU_ITEM_Y_OFFSET => unsafe {
            if dpad_left {
                y_offset -= 10.0;
            } else if dpad_right {
                y_offset += 10.0;
            }
        },
        MENU_ITEM_SPAWN => {
            if pressed_a {
                let rotation = Link::horizontal_movement_direction();
                let params = unsafe {
                    ((param_1 as u32) << 24)
                        | ((param_2 as u32) << 16)
                        | ((param_3 as u32) << 8)
                        | (param_4 as u32)
                };
                let flag_to_set = unsafe { (flag as u16) << 8 | flag_bit as u16 };

                ActorTemplate::new(actual_actor_name, coord.clone(), [0, rotation])
                    .with_params(params)
                    .with_flag(flag_to_set)
                    .spawn();

                // let mut code = format!(
                //     "ActorTemplate::new(\"{}\", Coord {{ x: {:.1}, y: {:.1}, z: {:.1} }}, \
                //      [0, {:#06X}])",
                //     actual_actor_name, coord.x, coord.y, coord.z, rotation
                // );
                // if params != actor::DEFAULT_PARAMS {
                //     let _ = write!(code, ".with_params({:#010X})", params);
                // }
                // if flag_to_set != actor::DEFAULT_FLAG {
                //     let _ = write!(code, ".with_flag({:#06X})", flag_to_set);
                // }
                // let _ = write!(code, ".spawn();");
                // println!("{}", code);
            }
        }
        _ => {}
    }

    for (index, (line, &content)) in lines.into_iter().skip(3).zip(&contents).enumerate() {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let _ = write!(line.append(), "{}", content);
        match index {
            MENU_ITEM_CATEGORY => {
                let _ = write!(line.append(), "{}", category_name);
            }
            MENU_ITEM_ACTOR => {
                let _ = write!(line.append(), "{}", visible_actor_name);
            }
            MENU_ITEM_PARAM_1 => {
                let _ = write!(line.append(), "{}", unsafe { param_1 });
            }
            MENU_ITEM_PARAM_2 => {
                let _ = write!(line.append(), "{}", unsafe { param_2 });
            }
            MENU_ITEM_PARAM_3 => {
                let _ = write!(line.append(), "{}", unsafe { param_3 });
            }
            MENU_ITEM_PARAM_4 => {
                let _ = write!(line.append(), "{}", unsafe { param_4 });
            }
            MENU_ITEM_FLAG => {
                let _ = write!(line.append(), "{}", unsafe { flag });
            }
            MENU_ITEM_FLAG_BIT => {
                let _ = write!(line.append(), "{}", unsafe { flag_bit });
            }
            MENU_ITEM_Y_OFFSET => {
                let _ = write!(line.append(), "{}", unsafe { y_offset });
            }
            MENU_ITEM_X => {
                let _ = write!(line.append(), "{:10.02}", coord.x);
            }
            MENU_ITEM_Y => {
                let _ = write!(line.append(), "{:10.02}", coord.y);
            }
            MENU_ITEM_Z => {
                let _ = write!(line.append(), "{:10.02}", coord.z);
            }
            MENU_ITEM_ROTATION => {
                let _ = write!(line.append(), "{:7}", Link::horizontal_movement_direction());
            }
            _ => {}
        }
    }
}
