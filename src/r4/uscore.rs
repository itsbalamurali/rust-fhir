///  The U.S. Centers for Disease Control and Prevention (CDC) has prepared a
///  code set for use in codingrace and ethnicity data. This code set is based on
///  current federal standards for classifying data onrace and ethnicity,
///  specifically the minimum race and ethnicity categories defined by the U.S.
///  Office ofManagement and Budget (OMB) and a more detailed set of race and
///  ethnicity categories maintainedby the U.S. Bureau of the Census (BC). The
///  main purpose of the code set is to facilitate use of federalstandards for
///  classifying data on race and ethnicity when these data are exchanged,
///  stored, retrieved,or analyzed in electronic form. At the same time, the code
///  set can be applied to paper-based recordsystems to the extent that these
///  systems are used to collect, maintain, and report data on race andethnicity
///  in accordance with current federal standards. Source: [Race and Ethnicity
///  Code Set
///  Version 1.0](<https://www.cdc.gov/phin/resources/vocabulary/documents/cdc-race--ethnicity-background-and-purpose.pdf>).
/// See urn:oid:2.16.840.1.113883.6.238
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct RaceAndEthnicityCdcCode {
}
/// Nested message and enum types in `RaceAndEthnicityCDCCode`.
pub mod race_and_ethnicity_cdc_code {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Race = 1,
        AmericanIndianOrAlaskaNative = 2,
        AmericanIndian = 3,
        AlaskaNative = 4,
        Abenaki = 5,
        Algonquian = 6,
        Apache = 7,
        Arapaho = 8,
        Arikara = 9,
        Assiniboine = 10,
        AssiniboineSioux = 11,
        Bannock = 12,
        Blackfeet = 13,
        Brotherton = 14,
        BurtLakeBand = 15,
        Caddo = 16,
        Cahuilla = 17,
        CaliforniaTribes = 18,
        CanadianAndLatinAmericanIndian = 19,
        Catawba = 20,
        Cayuse = 21,
        Chehalis = 22,
        Chemakuan = 23,
        Chemehuevi = 24,
        Cherokee = 25,
        CherokeeShawnee = 26,
        Cheyenne = 27,
        CheyenneArapaho = 28,
        Chickahominy = 29,
        Chickasaw = 30,
        Chinook = 31,
        Chippewa = 32,
        ChippewaCree = 33,
        Chitimacha = 34,
        Choctaw = 35,
        Chumash = 36,
        ClearLake = 37,
        CoeurDAlene = 38,
        Coharie = 39,
        ColoradoRiver = 40,
        Colville = 41,
        Comanche = 42,
        CoosLowerUmpquaSiuslaw = 43,
        Coos = 44,
        Coquilles = 45,
        Costanoan = 46,
        Coushatta = 47,
        Cowlitz = 48,
        Cree = 49,
        Creek = 50,
        Croatan = 51,
        Crow = 52,
        Cupeno = 53,
        Delaware = 54,
        Diegueno = 55,
        EasternTribes = 56,
        Esselen = 57,
        FortBelknap = 58,
        FortBerthold = 59,
        FortMcdowell = 60,
        FortHall = 61,
        Gabrieleno = 62,
        GrandRonde = 63,
        GrosVentres = 64,
        Haliwa = 65,
        Hidatsa = 66,
        Hoopa = 67,
        HoopaExtension = 68,
        Houma = 69,
        InajaCosmit = 70,
        Iowa = 71,
        Iroquois = 72,
        Juaneno = 73,
        Kalispel = 74,
        Karuk = 75,
        Kaw = 76,
        Kickapoo = 77,
        Kiowa = 78,
        Klallam = 79,
        Klamath = 80,
        Konkow = 81,
        Kootenai = 82,
        Lassik = 83,
        LongIsland = 84,
        Luiseno = 85,
        Lumbee = 86,
        Lummi = 87,
        Maidu = 88,
        Makah = 89,
        Maliseet = 90,
        Mandan = 91,
        Mattaponi = 92,
        Menominee = 93,
        Miami = 94,
        Miccosukee = 95,
        Micmac = 96,
        MissionIndians = 97,
        Miwok = 98,
        Modoc = 99,
        Mohegan = 100,
        Mono = 101,
        Nanticoke = 102,
        Narragansett = 103,
        Navajo = 104,
        NezPerce = 105,
        Nomalaki = 106,
        NorthwestTribes = 107,
        Omaha = 108,
        OregonAthabaskan = 109,
        Osage = 110,
        OtoeMissouria = 111,
        Ottawa = 112,
        Paiute = 113,
        Pamunkey = 114,
        Passamaquoddy = 115,
        Pawnee = 116,
        Penobscot = 117,
        Peoria = 118,
        Pequot = 119,
        Pima = 120,
        Piscataway = 121,
        PitRiver = 122,
        Pomo = 123,
        Ponca = 124,
        Potawatomi = 125,
        Powhatan = 126,
        Pueblo = 127,
        PugetSoundSalish = 128,
        Quapaw = 129,
        Quinault = 130,
        Rappahannock = 131,
        RenoSparks = 132,
        RoundValley = 133,
        SacAndFox = 134,
        Salinan = 135,
        Salish = 136,
        SalishAndKootenai = 137,
        Schaghticoke = 138,
        ScottValley = 139,
        Seminole = 140,
        Serrano = 141,
        Shasta = 142,
        Shawnee = 143,
        Shinnecock = 144,
        ShoalwaterBay = 145,
        Shoshone = 146,
        ShoshonePaiute = 147,
        Siletz = 148,
        Sioux = 149,
        Siuslaw = 150,
        Spokane = 151,
        Stewart = 152,
        Stockbridge = 153,
        Susanville = 154,
        TohonoOOdham = 155,
        Tolowa = 156,
        Tonkawa = 157,
        Tygh = 158,
        Umatilla = 159,
        Umpqua = 160,
        Ute = 161,
        Wailaki = 162,
        WallaWalla = 163,
        Wampanoag = 164,
        WarmSprings = 165,
        Wascopum = 166,
        Washoe = 167,
        Wichita = 168,
        WindRiver = 169,
        Winnebago = 170,
        Winnemucca = 171,
        Wintun = 172,
        Wiyot = 173,
        Yakama = 174,
        YakamaCowlitz = 175,
        Yaqui = 176,
        YavapaiApache = 177,
        Yokuts = 178,
        Yuchi = 179,
        Yuman = 180,
        Yurok = 181,
        Chiricahua = 182,
        FortSillApache = 183,
        JicarillaApache = 184,
        LipanApache = 185,
        MescaleroApache = 186,
        OklahomaApache = 187,
        PaysonApache = 188,
        SanCarlosApache = 189,
        WhiteMountainApache = 190,
        NorthernArapaho = 191,
        SouthernArapaho = 192,
        WindRiverArapaho = 193,
        FortPeckAssiniboineSioux = 194,
        OklahomaCado = 195,
        AguaCalienteCahuilla = 196,
        Augustine = 197,
        Cabazon = 198,
        LosCoyotes = 199,
        Morongo = 200,
        SantaRosaCahuilla = 201,
        TorresMartinez = 202,
        Cahto = 203,
        Chimariko = 204,
        CoastMiwok = 205,
        Digger = 206,
        Kawaiisu = 207,
        KernRiver = 208,
        Mattole = 209,
        RedWood = 210,
        SantaRosa = 211,
        Takelma = 212,
        Wappo = 213,
        Yana = 214,
        Yuki = 215,
        CanadianIndian = 216,
        CentralAmericanIndian10702 = 217,
        FrenchAmericanIndian = 218,
        MexicanAmericanIndian10728 = 219,
        SouthAmericanIndian10736 = 220,
        SpanishAmericanIndian = 221,
        Hoh = 222,
        Quileute = 223,
        CherokeeAlabama = 224,
        CherokeesOfNortheastAlabama = 225,
        CherokeesOfSoutheastAlabama = 226,
        EasternCherokee = 227,
        EchotaCherokee = 228,
        EtowahCherokee = 229,
        NorthernCherokee = 230,
        Tuscola = 231,
        UnitedKeetowahBandOfCherokee = 232,
        WesternCherokee = 233,
        NorthernCheyenne = 234,
        SouthernCheyenne = 235,
        EasternChickahominy = 236,
        WesternChickahominy = 237,
        Clatsop = 238,
        ColumbiaRiverChinook = 239,
        Kathlamet = 240,
        UpperChinook = 241,
        WakiakumChinook = 242,
        WillapaChinook = 243,
        Wishram = 244,
        BadRiver = 245,
        BayMillsChippewa = 246,
        BoisForte = 247,
        BurtLakeChippewa = 248,
        FondDuLac = 249,
        GrandPortage = 250,
        GrandTraverseBandOfOttawaChippewa = 251,
        Keweenaw = 252,
        LacCourteOreilles = 253,
        LacDuFlambeau = 254,
        LacVieuxDesertChippewa = 255,
        LakeSuperior = 256,
        LeechLake = 257,
        LittleShellChippewa = 258,
        MilleLacs = 259,
        MinnesotaChippewa = 260,
        Ontonagon = 261,
        RedCliffChippewa = 262,
        RedLakeChippewa = 263,
        SaginawChippewa = 264,
        StCroixChippewa = 265,
        SaultSteMarieChippewa = 266,
        SokoagonChippewa = 267,
        TurtleMountain = 268,
        WhiteEarth = 269,
        RockyBoysChippewaCree = 270,
        CliftonChoctaw = 271,
        JenaChoctaw = 272,
        MississippiChoctaw = 273,
        MowaBandOfChoctaw = 274,
        OklahomaChoctaw = 275,
        SantaYnez = 276,
        OklahomaComanche = 277,
        AlabamaCoushatta = 278,
        AlabamaCreek = 279,
        AlabamaQuassarte = 280,
        EasternCreek = 281,
        EasternMuscogee = 282,
        Kialegee = 283,
        LowerMuscogee = 284,
        MachisLowerCreekIndian = 285,
        PoarchBand = 286,
        PrincipalCreekIndianNation = 287,
        StarClanOfMuscogeeCreeks = 288,
        Thlopthlocco = 289,
        Tuckabachee = 290,
        AguaCaliente = 291,
        EasternDelaware = 292,
        LenniLenape = 293,
        Munsee = 294,
        OklahomaDelaware = 295,
        RampoughMountain = 296,
        SandHill = 297,
        Campo = 298,
        CapitanGrande = 299,
        Cuyapaipe = 300,
        LaPosta = 301,
        Manzanita = 302,
        MesaGrande = 303,
        SanPasqual = 304,
        SantaYsabel = 305,
        Sycuan = 306,
        Attacapa = 307,
        Biloxi = 308,
        GeorgetownEasternTribes = 309,
        Moor = 310,
        Nansemond = 311,
        Natchez = 312,
        NausuWaiwash = 313,
        Nipmuc = 314,
        Paugussett = 315,
        PocomokeAcohonock = 316,
        SoutheasternIndians = 317,
        Susquehanock = 318,
        TunicaBiloxi = 319,
        WaccamawSiousan = 320,
        Wicomico = 321,
        Atsina = 322,
        Trinity = 323,
        Whilkut = 324,
        IowaOfKansasNebraska = 325,
        IowaOfOklahoma = 326,
        Cayuga = 327,
        Mohawk = 328,
        Oneida = 329,
        Onondaga = 330,
        Seneca = 331,
        SenecaNation = 332,
        SenecaCayuga = 333,
        TonawandaSeneca = 334,
        Tuscarora = 335,
        Wyandotte = 336,
        OklahomaKickapoo = 337,
        TexasKickapoo = 338,
        OklahomaKiowa = 339,
        Jamestown = 340,
        LowerElwha = 341,
        PortGambleKlallam = 342,
        Matinecock = 343,
        Montauk = 344,
        Poospatuck = 345,
        Setauket = 346,
        LaJolla = 347,
        Pala = 348,
        Pauma = 349,
        Pechanga = 350,
        Soboba = 351,
        TwentyNinePalms = 352,
        Temecula = 353,
        MountainMaidu = 354,
        Nishinam = 355,
        IllinoisMiami = 356,
        IndianaMiami = 357,
        OklahomaMiami = 358,
        Aroostook = 359,
        AlamoNavajo = 360,
        CanoncitoNavajo = 361,
        RamahNavajo = 362,
        Alsea = 363,
        Celilo = 364,
        Columbia = 365,
        Kalapuya = 366,
        Molala = 367,
        Talakamish = 368,
        Tenino = 369,
        Tillamook = 370,
        Wenatchee = 371,
        Yahooskin = 372,
        BurtLakeOttawa = 373,
        MichiganOttawa = 374,
        OklahomaOttawa = 375,
        Bishop = 376,
        Bridgeport = 377,
        BurnsPaiute = 378,
        Cedarville = 379,
        FortBidwell = 380,
        FortIndependence = 381,
        Kaibab = 382,
        LasVegas = 383,
        LonePine = 384,
        Lovelock = 385,
        MalheurPaiute = 386,
        Moapa = 387,
        NorthernPaiute = 388,
        OwensValley = 389,
        PyramidLake = 390,
        SanJuanSouthernPaiute = 391,
        SouthernPaiute = 392,
        SummitLake = 393,
        UtuUtuGwaituPaiute = 394,
        WalkerRiver = 395,
        YeringtonPaiute = 396,
        IndianTownship = 397,
        PleasantPointPassamaquoddy = 398,
        OklahomaPawnee = 399,
        OklahomaPeoria = 400,
        MarshantucketPequot = 401,
        GilaRiverPimaMaricopa = 402,
        SaltRiverPimaMaricopa = 403,
        CentralPomo = 404,
        DryCreek = 405,
        EasternPomo = 406,
        Kashia = 407,
        NorthernPomo = 408,
        ScottsValley = 409,
        Stonyford = 410,
        SulphurBank = 411,
        NebraskaPonca = 412,
        OklahomaPonca = 413,
        CitizenBandPotawatomi = 414,
        ForestCounty = 415,
        Hannahville = 416,
        HuronPotawatomi = 417,
        PokagonPotawatomi = 418,
        PrairieBand = 419,
        WisconsinPotawatomi = 420,
        Acoma = 421,
        ArizonaTewa = 422,
        Cochiti = 423,
        Hopi = 424,
        Isleta = 425,
        Jemez = 426,
        Keres = 427,
        Laguna = 428,
        Nambe = 429,
        Picuris = 430,
        Piro = 431,
        Pojoaque = 432,
        SanFelipe = 433,
        SanIldefonso = 434,
        SanJuanPueblo = 435,
        SanJuanDe = 436,
        SanJuan = 437,
        Sandia = 438,
        SantaAna = 439,
        SantaClara = 440,
        SantoDomingo = 441,
        Taos = 442,
        Tesuque = 443,
        Tewa = 444,
        Tigua = 445,
        Zia = 446,
        Zuni = 447,
        Duwamish = 448,
        Kikiallus = 449,
        LowerSkagit = 450,
        Muckleshoot = 451,
        Nisqually = 452,
        Nooksack = 453,
        PortMadison = 454,
        Puyallup = 455,
        Samish = 456,
        SaukSuiattle = 457,
        Skokomish = 458,
        Skykomish = 459,
        Snohomish = 460,
        Snoqualmie = 461,
        SquaxinIsland = 462,
        Steilacoom = 463,
        Stillaguamish = 464,
        Suquamish = 465,
        Swinomish = 466,
        Tulalip = 467,
        UpperSkagit = 468,
        IowaSacAndFox = 469,
        MissouriSacAndFox = 470,
        OklahomaSacAndFox = 471,
        BigCypress = 472,
        Brighton = 473,
        FloridaSeminole = 474,
        HollywoodSeminole = 475,
        OklahomaSeminole = 476,
        SanManual = 477,
        AbsenteeShawnee = 478,
        EasternShawnee = 479,
        BattleMountain = 480,
        Duckwater = 481,
        Elko = 482,
        Ely = 483,
        Goshute = 484,
        Panamint = 485,
        RubyValley = 486,
        SkullValley = 487,
        SouthForkShoshone = 488,
        TeMoakWesternShoshone = 489,
        TimbiShaShoshone = 490,
        Washakie = 491,
        WindRiverShoshone = 492,
        Yomba = 493,
        DuckValley = 494,
        Fallon = 495,
        FortMcDermitt = 496,
        BlackfootSioux = 497,
        BruleSioux = 498,
        CheyenneRiverSioux = 499,
        CrowCreekSioux = 500,
        DakotaSioux = 501,
        FlandreauSantee = 502,
        FortPeck = 503,
        LakeTraverseSioux = 504,
        LowerBruleSioux = 505,
        LowerSioux = 506,
        MdewakantonSioux = 507,
        Miniconjou = 508,
        OglalaSioux = 509,
        PineRidgeSioux = 510,
        PipestoneSioux = 511,
        PrairieIslandSioux = 512,
        PriorLakeSioux = 513,
        RosebudSioux = 514,
        SansArcSioux = 515,
        SanteeSioux = 516,
        SissetonWahpeton = 517,
        SissetonSioux = 518,
        SpiritLakeSioux = 519,
        StandingRockSioux = 520,
        TetonSioux = 521,
        TwoKettleSioux = 522,
        UpperSioux = 523,
        WahpekuteSioux = 524,
        WahpetonSioux = 525,
        WazhazaSioux = 526,
        YanktonSioux = 527,
        YanktonaiSioux = 528,
        AkChin = 529,
        GilaBend = 530,
        SanXavier = 531,
        Sells = 532,
        CowCreekUmpqua = 533,
        AllenCanyon = 534,
        UintahUte = 535,
        UteMountainUte = 536,
        GayHeadWampanoag = 537,
        MashpeeWampanoag = 538,
        Alpine = 539,
        Carson = 540,
        Dresslerville = 541,
        HoChunk = 542,
        NebraskaWinnebago = 543,
        TableBluff = 544,
        BarrioLibre = 545,
        PascuaYaqui = 546,
        Chukchansi = 547,
        Tachi = 548,
        TuleRiver = 549,
        Cocopah = 550,
        Havasupai = 551,
        Hualapai = 552,
        Maricopa = 553,
        Mohave = 554,
        Quechan = 555,
        Yavapai = 556,
        CoastYurok = 557,
        AlaskaIndian = 558,
        Eskimo = 559,
        Aleut = 560,
        AlaskanAthabascan = 561,
        SoutheastAlaska = 562,
        Ahtna = 563,
        Alatna = 564,
        Alexander = 565,
        Allakaket = 566,
        Alanvik = 567,
        Anvik = 568,
        Arctic = 569,
        Beaver = 570,
        BirchCreek = 571,
        Cantwell = 572,
        Chalkyitsik = 573,
        Chickaloon = 574,
        Chistochina = 575,
        Chitina = 576,
        Circle = 577,
        CookInlet = 578,
        CopperCenter = 579,
        CopperRiver = 580,
        DotLake = 581,
        Doyon = 582,
        Eagle = 583,
        Eklutna = 584,
        Evansville = 585,
        FortYukon = 586,
        Gakona = 587,
        Galena = 588,
        Grayling = 589,
        Gulkana = 590,
        HealyLake = 591,
        HolyCross = 592,
        Hughes = 593,
        Huslia = 594,
        Iliamna = 595,
        Kaltag = 596,
        KlutiKaah = 597,
        Knik = 598,
        Koyukuk = 599,
        LakeMinchumina = 600,
        Lime = 601,
        Mcgrath = 602,
        ManleyHotSprings = 603,
        MentastaLake = 604,
        Minto = 605,
        Nenana = 606,
        Nikolai = 607,
        Ninilchik = 608,
        Nondalton = 609,
        Northway = 610,
        Nulato = 611,
        PedroBay = 612,
        Rampart = 613,
        Ruby = 614,
        Salamatof = 615,
        Seldovia = 616,
        Slana = 617,
        Shageluk = 618,
        Stevens = 619,
        StonyRiver = 620,
        Takotna = 621,
        Tanacross = 622,
        Tanaina = 623,
        Tanana = 624,
        TananaChiefs = 625,
        Tazlina = 626,
        Telida = 627,
        Tetlin = 628,
        Tok = 629,
        Tyonek = 630,
        Venetie = 631,
        Wiseman = 632,
        TlingitHaida = 633,
        Tsimshian = 634,
        Angoon = 635,
        CentralCouncilOfTlingitAndHaidaTribes = 636,
        Chilkat = 637,
        Chilkoot = 638,
        Craig = 639,
        Douglas = 640,
        Haida = 641,
        Hoonah = 642,
        Hydaburg = 643,
        Kake = 644,
        Kasaan = 645,
        Kenaitze = 646,
        Ketchikan = 647,
        Klawock = 648,
        Pelican = 649,
        Petersburg = 650,
        Saxman = 651,
        Sitka = 652,
        TenakeeSprings = 653,
        Tlingit = 654,
        Wrangell = 655,
        Yakutat = 656,
        Metlakatla = 657,
        GreenlandEskimo = 658,
        InupiatEskimo = 659,
        SiberianEskimo = 660,
        YupikEskimo = 661,
        Ambler = 662,
        Anaktuvuk = 663,
        AnaktuvukPass = 664,
        ArcticSlopeInupiat = 665,
        ArcticSlopeCorporation = 666,
        Atqasuk = 667,
        Barrow = 668,
        BeringStraitsInupiat = 669,
        BrevigMission = 670,
        Buckland = 671,
        Chinik = 672,
        Council = 673,
        Deering = 674,
        Elim = 675,
        Golovin = 676,
        InalikDiomede = 677,
        Inupiaq = 678,
        Kaktovik = 679,
        Kawerak = 680,
        Kiana = 681,
        Kivalina = 682,
        Kobuk = 683,
        Kotzebue = 684,
        Koyuk = 685,
        Kwiguk = 686,
        MaunelukInupiat = 687,
        NanaInupiat = 688,
        Noatak = 689,
        Nome = 690,
        Noorvik = 691,
        Nuiqsut = 692,
        PointHope = 693,
        PointLay = 694,
        Selawik = 695,
        Shaktoolik = 696,
        Shishmaref = 697,
        Shungnak = 698,
        Solomon = 699,
        Teller = 700,
        Unalakleet = 701,
        Wainwright = 702,
        Wales = 703,
        WhiteMountain = 704,
        WhiteMountainInupiat = 705,
        MarysIgloo = 706,
        Gambell = 707,
        Savoonga = 708,
        SiberianYupik = 709,
        Akiachak = 710,
        Akiak = 711,
        Alakanuk = 712,
        Aleknagik = 713,
        Andreafsky = 714,
        Aniak = 715,
        Atmautluak = 716,
        Bethel = 717,
        BillMooresSlough = 718,
        BristolBayYupik = 719,
        CalistaYupik = 720,
        Chefornak = 721,
        Chevak = 722,
        Chuathbaluk = 723,
        ClarksPoint = 724,
        CrookedCreek = 725,
        Dillingham = 726,
        Eek = 727,
        Ekuk = 728,
        Ekwok = 729,
        Emmonak = 730,
        GoodnewsBay = 731,
        HooperBay = 732,
        IqurmuitRussianMission = 733,
        Kalskag = 734,
        Kasigluk = 735,
        Kipnuk = 736,
        Koliganek = 737,
        Kongiganak = 738,
        Kotlik = 739,
        Kwethluk = 740,
        Kwigillingok = 741,
        Levelock = 742,
        LowerKalskag = 743,
        Manokotak = 744,
        Marshall = 745,
        Mekoryuk = 746,
        MountainVillage = 747,
        Naknek = 748,
        Napaumute = 749,
        Napakiak = 750,
        Napaskiak = 751,
        Newhalen = 752,
        NewStuyahok = 753,
        Newtok = 754,
        Nightmute = 755,
        Nunapitchukv = 756,
        Oscarville = 757,
        PilotStation = 758,
        PitkasPoint = 759,
        Platinum = 760,
        PortageCreek = 761,
        Quinhagak = 762,
        RedDevil = 763,
        StMichael = 764,
        ScammonBay = 765,
        SheldonsPoint = 766,
        Sleetmute = 767,
        Stebbins = 768,
        Togiak = 769,
        Toksook = 770,
        Tulukskak = 771,
        Tuntutuliak = 772,
        Tununak = 773,
        TwinHills = 774,
        GeorgetownYupikEskimo = 775,
        StMarys = 776,
        Umkumiate = 777,
        AlutiiqAleut = 778,
        BristolBayAleut = 779,
        ChugachAleut = 780,
        Eyak = 781,
        KoniagAleut = 782,
        Sugpiaq = 783,
        Suqpigaq = 784,
        UnanganAleut = 785,
        Tatitlek = 786,
        Ugashik = 787,
        Chignik = 788,
        ChignikLake = 789,
        Egegik = 790,
        Igiugig = 791,
        IvanofBay = 792,
        KingSalmon = 793,
        Kokhanok = 794,
        Perryville = 795,
        PilotPoint = 796,
        PortHeiden = 797,
        Chenega = 798,
        ChugachCorporation = 799,
        EnglishBay = 800,
        PortGraham = 801,
        Akhiok = 802,
        Agdaagux = 803,
        Karluk = 804,
        Kodiak = 805,
        LarsenBay = 806,
        OldHarbor = 807,
        Ouzinkie = 808,
        PortLions = 809,
        Akutan = 810,
        AleutCorporation = 811,
        Aleutian = 812,
        AleutianIslander = 813,
        Atka = 814,
        Belkofski = 815,
        ChignikLagoon = 816,
        KingCove = 817,
        FalsePass = 818,
        NelsonLagoon = 819,
        Nikolski = 820,
        PauloffHarbor = 821,
        QaganToyagungin = 822,
        Qawalangin = 823,
        StGeorge = 824,
        StPaul = 825,
        SandPoint = 826,
        SouthNaknek = 827,
        Unalaska = 828,
        Unga = 829,
        Asian = 830,
        AsianIndian = 831,
        Bangladeshi = 832,
        Bhutanese = 833,
        Burmese = 834,
        Cambodian = 835,
        Chinese = 836,
        Taiwanese = 837,
        Filipino = 838,
        Hmong = 839,
        Indonesian = 840,
        Japanese = 841,
        Korean = 842,
        Laotian = 843,
        Malaysian = 844,
        Okinawan = 845,
        Pakistani = 846,
        SriLankan = 847,
        Thai = 848,
        Vietnamese = 849,
        IwoJiman = 850,
        Maldivian = 851,
        Nepalese = 852,
        Singaporean = 853,
        Madagascar = 854,
        BlackOrAfricanAmerican = 855,
        Black = 856,
        AfricanAmerican = 857,
        African = 858,
        Bahamian = 859,
        Barbadian = 860,
        Dominican20693 = 861,
        DominicaIslander = 862,
        Haitian = 863,
        Jamaican = 864,
        Tobagoan = 865,
        Trinidadian = 866,
        WestIndian = 867,
        Botswanan = 868,
        Ethiopian = 869,
        Liberian = 870,
        Namibian = 871,
        Nigerian = 872,
        Zairean = 873,
        NativeHawaiianOrOtherPacificIslander = 874,
        Polynesian = 875,
        Micronesian = 876,
        Melanesian = 877,
        OtherPacificIslander = 878,
        NativeHawaiian = 879,
        Samoan = 880,
        Tahitian = 881,
        Tongan = 882,
        Tokelauan = 883,
        GuamanianOrChamorro = 884,
        Guamanian = 885,
        Chamorro = 886,
        MarianaIslander = 887,
        Marshallese = 888,
        Palauan = 889,
        Carolinian = 890,
        Kosraean = 891,
        Pohnpeian = 892,
        Saipanese = 893,
        Kiribati = 894,
        Chuukese = 895,
        Yapese = 896,
        Fijian = 897,
        PapuaNewGuinean = 898,
        SolomonIslander = 899,
        NewHebrides = 900,
        White = 901,
        European = 902,
        MiddleEasternOrNorthAfrican = 903,
        Arab = 904,
        Armenian = 905,
        English = 906,
        French = 907,
        German = 908,
        Irish = 909,
        Italian = 910,
        Polish = 911,
        Scottish = 912,
        Assyrian = 913,
        Egyptian = 914,
        Iranian = 915,
        Iraqi = 916,
        Lebanese = 917,
        Palestinian = 918,
        Syrian = 919,
        Afghanistani = 920,
        Israeili = 921,
        OtherRace = 922,
        Ethnicity = 923,
        HispanicOrLatino = 924,
        Spaniard = 925,
        Mexican = 926,
        CentralAmerican = 927,
        SouthAmerican = 928,
        LatinAmerican = 929,
        PuertoRican = 930,
        Cuban = 931,
        Dominican21840 = 932,
        Andalusian = 933,
        Asturian = 934,
        Castillian = 935,
        Catalonian = 936,
        BelearicIslander = 937,
        Gallego = 938,
        Valencian = 939,
        Canarian = 940,
        SpanishBasque = 941,
        MexicanAmerican = 942,
        Mexicano = 943,
        Chicano = 944,
        LaRaza = 945,
        MexicanAmericanIndian21535 = 946,
        CostaRican = 947,
        Guatemalan = 948,
        Honduran = 949,
        Nicaraguan = 950,
        Panamanian = 951,
        Salvadoran = 952,
        CentralAmericanIndian21626 = 953,
        CanalZone = 954,
        Argentinean = 955,
        Bolivian = 956,
        Chilean = 957,
        Colombian = 958,
        Ecuadorian = 959,
        Paraguayan = 960,
        Peruvian = 961,
        Uruguayan = 962,
        Venezuelan = 963,
        SouthAmericanIndian21758 = 964,
        Criollo = 965,
        NotHispanicOrLatino = 966,
    }
}
/// Set of codes that are needed for implementation of the US-Core profiles.
/// These codes are used as extensions to the FHIR and US Core value sets.
///
/// See <http://hl7.org/fhir/us/core/CodeSystem/careplan-category>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreCarePlanCategoryExtensionCode {
}
/// Nested message and enum types in `USCoreCarePlanCategoryExtensionCode`.
pub mod us_core_care_plan_category_extension_code {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        AssessPlan = 1,
    }
}
/// Set of codes that are needed for implementation of the US-Core profiles.
/// These codes are used as extensions to the FHIR and US Core value sets.
///
/// See <http://hl7.org/fhir/us/core/CodeSystem/condition-category>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreConditionCategoryExtensionCode {
}
/// Nested message and enum types in `USCoreConditionCategoryExtensionCode`.
pub mod us_core_condition_category_extension_code {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Problem = 1,
        HealthConcern = 2,
    }
}
/// The US Core DocumentReferences Type Code System is a 'starter set' of
/// categories supported for fetching and storing DocumentReference Resources.
/// See <http://hl7.org/fhir/us/core/CodeSystem/us-core-documentreference-category>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreDocumentReferencesCategoryCode {
}
/// Nested message and enum types in `USCoreDocumentReferencesCategoryCode`.
pub mod us_core_document_references_category_code {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        ClinicalNote = 1,
    }
}
/// Set of codes that are needed for implementation of the US-Core profiles.
/// These codes are used as extensions to the FHIR and US Core value sets.
///
/// See
/// <http://hl7.org/fhir/us/core/CodeSystem/us-core-provenance-participant-type>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreProvenancePaticipantTypeExtensionCode {
}
/// Nested message and enum types in `USCoreProvenancePaticipantTypeExtensionCode`.
pub mod us_core_provenance_paticipant_type_extension_code {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Transmitter = 1,
    }
}
/// Codes for assigning sex at birth as specified by the [Office of the National
/// Coordinator for Health IT (ONC)](<https://www.healthit.gov/newsroom/about-onc>)
/// See <http://hl7.org/fhir/us/core/ValueSet/birthsex>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct BirthSexValueSet {
}
/// Nested message and enum types in `BirthSexValueSet`.
pub mod birth_sex_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        F = 1,
        M = 2,
        Unk = 3,
    }
}
/// The 41 [CDC ethnicity
/// codes](<http://www.cdc.gov/phin/resources/vocabulary/index.html>) that are
/// grouped under one of the 2 OMB ethnicity category codes. See
/// <http://hl7.org/fhir/us/core/ValueSet/detailed-ethnicity>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct DetailedEthnicityValueSet {
}
/// Nested message and enum types in `DetailedEthnicityValueSet`.
pub mod detailed_ethnicity_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Ethnicity = 1,
        Spaniard = 2,
        Mexican = 3,
        CentralAmerican = 4,
        SouthAmerican = 5,
        LatinAmerican = 6,
        PuertoRican = 7,
        Cuban = 8,
        Dominican = 9,
        Andalusian = 10,
        Asturian = 11,
        Castillian = 12,
        Catalonian = 13,
        BelearicIslander = 14,
        Gallego = 15,
        Valencian = 16,
        Canarian = 17,
        SpanishBasque = 18,
        MexicanAmerican = 19,
        Mexicano = 20,
        Chicano = 21,
        LaRaza = 22,
        MexicanAmericanIndian = 23,
        CostaRican = 24,
        Guatemalan = 25,
        Honduran = 26,
        Nicaraguan = 27,
        Panamanian = 28,
        Salvadoran = 29,
        CentralAmericanIndian = 30,
        CanalZone = 31,
        Argentinean = 32,
        Bolivian = 33,
        Chilean = 34,
        Colombian = 35,
        Ecuadorian = 36,
        Paraguayan = 37,
        Peruvian = 38,
        Uruguayan = 39,
        Venezuelan = 40,
        SouthAmericanIndian = 41,
        Criollo = 42,
    }
}
/// The 900+ [CDC Race
/// codes](<http://www.cdc.gov/phin/resources/vocabulary/index.html>) that are
/// grouped under one of the 5 OMB race category codes. See
/// <http://hl7.org/fhir/us/core/ValueSet/detailed-race>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct DetailedRaceValueSet {
}
/// Nested message and enum types in `DetailedRaceValueSet`.
pub mod detailed_race_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Race = 1,
        AmericanIndian = 2,
        AlaskaNative = 3,
        Abenaki = 4,
        Algonquian = 5,
        Apache = 6,
        Arapaho = 7,
        Arikara = 8,
        Assiniboine = 9,
        AssiniboineSioux = 10,
        Bannock = 11,
        Blackfeet = 12,
        Brotherton = 13,
        BurtLakeBand = 14,
        Caddo = 15,
        Cahuilla = 16,
        CaliforniaTribes = 17,
        CanadianAndLatinAmericanIndian = 18,
        Catawba = 19,
        Cayuse = 20,
        Chehalis = 21,
        Chemakuan = 22,
        Chemehuevi = 23,
        Cherokee = 24,
        CherokeeShawnee = 25,
        Cheyenne = 26,
        CheyenneArapaho = 27,
        Chickahominy = 28,
        Chickasaw = 29,
        Chinook = 30,
        Chippewa = 31,
        ChippewaCree = 32,
        Chitimacha = 33,
        Choctaw = 34,
        Chumash = 35,
        ClearLake = 36,
        CoeurDAlene = 37,
        Coharie = 38,
        ColoradoRiver = 39,
        Colville = 40,
        Comanche = 41,
        CoosLowerUmpquaSiuslaw = 42,
        Coos = 43,
        Coquilles = 44,
        Costanoan = 45,
        Coushatta = 46,
        Cowlitz = 47,
        Cree = 48,
        Creek = 49,
        Croatan = 50,
        Crow = 51,
        Cupeno = 52,
        Delaware = 53,
        Diegueno = 54,
        EasternTribes = 55,
        Esselen = 56,
        FortBelknap = 57,
        FortBerthold = 58,
        FortMcdowell = 59,
        FortHall = 60,
        Gabrieleno = 61,
        GrandRonde = 62,
        GrosVentres = 63,
        Haliwa = 64,
        Hidatsa = 65,
        Hoopa = 66,
        HoopaExtension = 67,
        Houma = 68,
        InajaCosmit = 69,
        Iowa = 70,
        Iroquois = 71,
        Juaneno = 72,
        Kalispel = 73,
        Karuk = 74,
        Kaw = 75,
        Kickapoo = 76,
        Kiowa = 77,
        Klallam = 78,
        Klamath = 79,
        Konkow = 80,
        Kootenai = 81,
        Lassik = 82,
        LongIsland = 83,
        Luiseno = 84,
        Lumbee = 85,
        Lummi = 86,
        Maidu = 87,
        Makah = 88,
        Maliseet = 89,
        Mandan = 90,
        Mattaponi = 91,
        Menominee = 92,
        Miami = 93,
        Miccosukee = 94,
        Micmac = 95,
        MissionIndians = 96,
        Miwok = 97,
        Modoc = 98,
        Mohegan = 99,
        Mono = 100,
        Nanticoke = 101,
        Narragansett = 102,
        Navajo = 103,
        NezPerce = 104,
        Nomalaki = 105,
        NorthwestTribes = 106,
        Omaha = 107,
        OregonAthabaskan = 108,
        Osage = 109,
        OtoeMissouria = 110,
        Ottawa = 111,
        Paiute = 112,
        Pamunkey = 113,
        Passamaquoddy = 114,
        Pawnee = 115,
        Penobscot = 116,
        Peoria = 117,
        Pequot = 118,
        Pima = 119,
        Piscataway = 120,
        PitRiver = 121,
        Pomo = 122,
        Ponca = 123,
        Potawatomi = 124,
        Powhatan = 125,
        Pueblo = 126,
        PugetSoundSalish = 127,
        Quapaw = 128,
        Quinault = 129,
        Rappahannock = 130,
        RenoSparks = 131,
        RoundValley = 132,
        SacAndFox = 133,
        Salinan = 134,
        Salish = 135,
        SalishAndKootenai = 136,
        Schaghticoke = 137,
        ScottValley = 138,
        Seminole = 139,
        Serrano = 140,
        Shasta = 141,
        Shawnee = 142,
        Shinnecock = 143,
        ShoalwaterBay = 144,
        Shoshone = 145,
        ShoshonePaiute = 146,
        Siletz = 147,
        Sioux = 148,
        Siuslaw = 149,
        Spokane = 150,
        Stewart = 151,
        Stockbridge = 152,
        Susanville = 153,
        TohonoOOdham = 154,
        Tolowa = 155,
        Tonkawa = 156,
        Tygh = 157,
        Umatilla = 158,
        Umpqua = 159,
        Ute = 160,
        Wailaki = 161,
        WallaWalla = 162,
        Wampanoag = 163,
        WarmSprings = 164,
        Wascopum = 165,
        Washoe = 166,
        Wichita = 167,
        WindRiver = 168,
        Winnebago = 169,
        Winnemucca = 170,
        Wintun = 171,
        Wiyot = 172,
        Yakama = 173,
        YakamaCowlitz = 174,
        Yaqui = 175,
        YavapaiApache = 176,
        Yokuts = 177,
        Yuchi = 178,
        Yuman = 179,
        Yurok = 180,
        Chiricahua = 181,
        FortSillApache = 182,
        JicarillaApache = 183,
        LipanApache = 184,
        MescaleroApache = 185,
        OklahomaApache = 186,
        PaysonApache = 187,
        SanCarlosApache = 188,
        WhiteMountainApache = 189,
        NorthernArapaho = 190,
        SouthernArapaho = 191,
        WindRiverArapaho = 192,
        FortPeckAssiniboineSioux = 193,
        OklahomaCado = 194,
        AguaCalienteCahuilla = 195,
        Augustine = 196,
        Cabazon = 197,
        LosCoyotes = 198,
        Morongo = 199,
        SantaRosaCahuilla = 200,
        TorresMartinez = 201,
        Cahto = 202,
        Chimariko = 203,
        CoastMiwok = 204,
        Digger = 205,
        Kawaiisu = 206,
        KernRiver = 207,
        Mattole = 208,
        RedWood = 209,
        SantaRosa = 210,
        Takelma = 211,
        Wappo = 212,
        Yana = 213,
        Yuki = 214,
        CanadianIndian = 215,
        CentralAmericanIndian = 216,
        FrenchAmericanIndian = 217,
        MexicanAmericanIndian = 218,
        SouthAmericanIndian = 219,
        SpanishAmericanIndian = 220,
        Hoh = 221,
        Quileute = 222,
        CherokeeAlabama = 223,
        CherokeesOfNortheastAlabama = 224,
        CherokeesOfSoutheastAlabama = 225,
        EasternCherokee = 226,
        EchotaCherokee = 227,
        EtowahCherokee = 228,
        NorthernCherokee = 229,
        Tuscola = 230,
        UnitedKeetowahBandOfCherokee = 231,
        WesternCherokee = 232,
        NorthernCheyenne = 233,
        SouthernCheyenne = 234,
        EasternChickahominy = 235,
        WesternChickahominy = 236,
        Clatsop = 237,
        ColumbiaRiverChinook = 238,
        Kathlamet = 239,
        UpperChinook = 240,
        WakiakumChinook = 241,
        WillapaChinook = 242,
        Wishram = 243,
        BadRiver = 244,
        BayMillsChippewa = 245,
        BoisForte = 246,
        BurtLakeChippewa = 247,
        FondDuLac = 248,
        GrandPortage = 249,
        GrandTraverseBandOfOttawaChippewa = 250,
        Keweenaw = 251,
        LacCourteOreilles = 252,
        LacDuFlambeau = 253,
        LacVieuxDesertChippewa = 254,
        LakeSuperior = 255,
        LeechLake = 256,
        LittleShellChippewa = 257,
        MilleLacs = 258,
        MinnesotaChippewa = 259,
        Ontonagon = 260,
        RedCliffChippewa = 261,
        RedLakeChippewa = 262,
        SaginawChippewa = 263,
        StCroixChippewa = 264,
        SaultSteMarieChippewa = 265,
        SokoagonChippewa = 266,
        TurtleMountain = 267,
        WhiteEarth = 268,
        RockyBoysChippewaCree = 269,
        CliftonChoctaw = 270,
        JenaChoctaw = 271,
        MississippiChoctaw = 272,
        MowaBandOfChoctaw = 273,
        OklahomaChoctaw = 274,
        SantaYnez = 275,
        OklahomaComanche = 276,
        AlabamaCoushatta = 277,
        AlabamaCreek = 278,
        AlabamaQuassarte = 279,
        EasternCreek = 280,
        EasternMuscogee = 281,
        Kialegee = 282,
        LowerMuscogee = 283,
        MachisLowerCreekIndian = 284,
        PoarchBand = 285,
        PrincipalCreekIndianNation = 286,
        StarClanOfMuscogeeCreeks = 287,
        Thlopthlocco = 288,
        Tuckabachee = 289,
        AguaCaliente = 290,
        EasternDelaware = 291,
        LenniLenape = 292,
        Munsee = 293,
        OklahomaDelaware = 294,
        RampoughMountain = 295,
        SandHill = 296,
        Campo = 297,
        CapitanGrande = 298,
        Cuyapaipe = 299,
        LaPosta = 300,
        Manzanita = 301,
        MesaGrande = 302,
        SanPasqual = 303,
        SantaYsabel = 304,
        Sycuan = 305,
        Attacapa = 306,
        Biloxi = 307,
        GeorgetownEasternTribes = 308,
        Moor = 309,
        Nansemond = 310,
        Natchez = 311,
        NausuWaiwash = 312,
        Nipmuc = 313,
        Paugussett = 314,
        PocomokeAcohonock = 315,
        SoutheasternIndians = 316,
        Susquehanock = 317,
        TunicaBiloxi = 318,
        WaccamawSiousan = 319,
        Wicomico = 320,
        Atsina = 321,
        Trinity = 322,
        Whilkut = 323,
        IowaOfKansasNebraska = 324,
        IowaOfOklahoma = 325,
        Cayuga = 326,
        Mohawk = 327,
        Oneida = 328,
        Onondaga = 329,
        Seneca = 330,
        SenecaNation = 331,
        SenecaCayuga = 332,
        TonawandaSeneca = 333,
        Tuscarora = 334,
        Wyandotte = 335,
        OklahomaKickapoo = 336,
        TexasKickapoo = 337,
        OklahomaKiowa = 338,
        Jamestown = 339,
        LowerElwha = 340,
        PortGambleKlallam = 341,
        Matinecock = 342,
        Montauk = 343,
        Poospatuck = 344,
        Setauket = 345,
        LaJolla = 346,
        Pala = 347,
        Pauma = 348,
        Pechanga = 349,
        Soboba = 350,
        TwentyNinePalms = 351,
        Temecula = 352,
        MountainMaidu = 353,
        Nishinam = 354,
        IllinoisMiami = 355,
        IndianaMiami = 356,
        OklahomaMiami = 357,
        Aroostook = 358,
        AlamoNavajo = 359,
        CanoncitoNavajo = 360,
        RamahNavajo = 361,
        Alsea = 362,
        Celilo = 363,
        Columbia = 364,
        Kalapuya = 365,
        Molala = 366,
        Talakamish = 367,
        Tenino = 368,
        Tillamook = 369,
        Wenatchee = 370,
        Yahooskin = 371,
        BurtLakeOttawa = 372,
        MichiganOttawa = 373,
        OklahomaOttawa = 374,
        Bishop = 375,
        Bridgeport = 376,
        BurnsPaiute = 377,
        Cedarville = 378,
        FortBidwell = 379,
        FortIndependence = 380,
        Kaibab = 381,
        LasVegas = 382,
        LonePine = 383,
        Lovelock = 384,
        MalheurPaiute = 385,
        Moapa = 386,
        NorthernPaiute = 387,
        OwensValley = 388,
        PyramidLake = 389,
        SanJuanSouthernPaiute = 390,
        SouthernPaiute = 391,
        SummitLake = 392,
        UtuUtuGwaituPaiute = 393,
        WalkerRiver = 394,
        YeringtonPaiute = 395,
        IndianTownship = 396,
        PleasantPointPassamaquoddy = 397,
        OklahomaPawnee = 398,
        OklahomaPeoria = 399,
        MarshantucketPequot = 400,
        GilaRiverPimaMaricopa = 401,
        SaltRiverPimaMaricopa = 402,
        CentralPomo = 403,
        DryCreek = 404,
        EasternPomo = 405,
        Kashia = 406,
        NorthernPomo = 407,
        ScottsValley = 408,
        Stonyford = 409,
        SulphurBank = 410,
        NebraskaPonca = 411,
        OklahomaPonca = 412,
        CitizenBandPotawatomi = 413,
        ForestCounty = 414,
        Hannahville = 415,
        HuronPotawatomi = 416,
        PokagonPotawatomi = 417,
        PrairieBand = 418,
        WisconsinPotawatomi = 419,
        Acoma = 420,
        ArizonaTewa = 421,
        Cochiti = 422,
        Hopi = 423,
        Isleta = 424,
        Jemez = 425,
        Keres = 426,
        Laguna = 427,
        Nambe = 428,
        Picuris = 429,
        Piro = 430,
        Pojoaque = 431,
        SanFelipe = 432,
        SanIldefonso = 433,
        SanJuanPueblo = 434,
        SanJuanDe = 435,
        SanJuan = 436,
        Sandia = 437,
        SantaAna = 438,
        SantaClara = 439,
        SantoDomingo = 440,
        Taos = 441,
        Tesuque = 442,
        Tewa = 443,
        Tigua = 444,
        Zia = 445,
        Zuni = 446,
        Duwamish = 447,
        Kikiallus = 448,
        LowerSkagit = 449,
        Muckleshoot = 450,
        Nisqually = 451,
        Nooksack = 452,
        PortMadison = 453,
        Puyallup = 454,
        Samish = 455,
        SaukSuiattle = 456,
        Skokomish = 457,
        Skykomish = 458,
        Snohomish = 459,
        Snoqualmie = 460,
        SquaxinIsland = 461,
        Steilacoom = 462,
        Stillaguamish = 463,
        Suquamish = 464,
        Swinomish = 465,
        Tulalip = 466,
        UpperSkagit = 467,
        IowaSacAndFox = 468,
        MissouriSacAndFox = 469,
        OklahomaSacAndFox = 470,
        BigCypress = 471,
        Brighton = 472,
        FloridaSeminole = 473,
        HollywoodSeminole = 474,
        OklahomaSeminole = 475,
        SanManual = 476,
        AbsenteeShawnee = 477,
        EasternShawnee = 478,
        BattleMountain = 479,
        Duckwater = 480,
        Elko = 481,
        Ely = 482,
        Goshute = 483,
        Panamint = 484,
        RubyValley = 485,
        SkullValley = 486,
        SouthForkShoshone = 487,
        TeMoakWesternShoshone = 488,
        TimbiShaShoshone = 489,
        Washakie = 490,
        WindRiverShoshone = 491,
        Yomba = 492,
        DuckValley = 493,
        Fallon = 494,
        FortMcDermitt = 495,
        BlackfootSioux = 496,
        BruleSioux = 497,
        CheyenneRiverSioux = 498,
        CrowCreekSioux = 499,
        DakotaSioux = 500,
        FlandreauSantee = 501,
        FortPeck = 502,
        LakeTraverseSioux = 503,
        LowerBruleSioux = 504,
        LowerSioux = 505,
        MdewakantonSioux = 506,
        Miniconjou = 507,
        OglalaSioux = 508,
        PineRidgeSioux = 509,
        PipestoneSioux = 510,
        PrairieIslandSioux = 511,
        PriorLakeSioux = 512,
        RosebudSioux = 513,
        SansArcSioux = 514,
        SanteeSioux = 515,
        SissetonWahpeton = 516,
        SissetonSioux = 517,
        SpiritLakeSioux = 518,
        StandingRockSioux = 519,
        TetonSioux = 520,
        TwoKettleSioux = 521,
        UpperSioux = 522,
        WahpekuteSioux = 523,
        WahpetonSioux = 524,
        WazhazaSioux = 525,
        YanktonSioux = 526,
        YanktonaiSioux = 527,
        AkChin = 528,
        GilaBend = 529,
        SanXavier = 530,
        Sells = 531,
        CowCreekUmpqua = 532,
        AllenCanyon = 533,
        UintahUte = 534,
        UteMountainUte = 535,
        GayHeadWampanoag = 536,
        MashpeeWampanoag = 537,
        Alpine = 538,
        Carson = 539,
        Dresslerville = 540,
        HoChunk = 541,
        NebraskaWinnebago = 542,
        TableBluff = 543,
        BarrioLibre = 544,
        PascuaYaqui = 545,
        Chukchansi = 546,
        Tachi = 547,
        TuleRiver = 548,
        Cocopah = 549,
        Havasupai = 550,
        Hualapai = 551,
        Maricopa = 552,
        Mohave = 553,
        Quechan = 554,
        Yavapai = 555,
        CoastYurok = 556,
        AlaskaIndian = 557,
        Eskimo = 558,
        Aleut = 559,
        AlaskanAthabascan = 560,
        SoutheastAlaska = 561,
        Ahtna = 562,
        Alatna = 563,
        Alexander = 564,
        Allakaket = 565,
        Alanvik = 566,
        Anvik = 567,
        Arctic = 568,
        Beaver = 569,
        BirchCreek = 570,
        Cantwell = 571,
        Chalkyitsik = 572,
        Chickaloon = 573,
        Chistochina = 574,
        Chitina = 575,
        Circle = 576,
        CookInlet = 577,
        CopperCenter = 578,
        CopperRiver = 579,
        DotLake = 580,
        Doyon = 581,
        Eagle = 582,
        Eklutna = 583,
        Evansville = 584,
        FortYukon = 585,
        Gakona = 586,
        Galena = 587,
        Grayling = 588,
        Gulkana = 589,
        HealyLake = 590,
        HolyCross = 591,
        Hughes = 592,
        Huslia = 593,
        Iliamna = 594,
        Kaltag = 595,
        KlutiKaah = 596,
        Knik = 597,
        Koyukuk = 598,
        LakeMinchumina = 599,
        Lime = 600,
        Mcgrath = 601,
        ManleyHotSprings = 602,
        MentastaLake = 603,
        Minto = 604,
        Nenana = 605,
        Nikolai = 606,
        Ninilchik = 607,
        Nondalton = 608,
        Northway = 609,
        Nulato = 610,
        PedroBay = 611,
        Rampart = 612,
        Ruby = 613,
        Salamatof = 614,
        Seldovia = 615,
        Slana = 616,
        Shageluk = 617,
        Stevens = 618,
        StonyRiver = 619,
        Takotna = 620,
        Tanacross = 621,
        Tanaina = 622,
        Tanana = 623,
        TananaChiefs = 624,
        Tazlina = 625,
        Telida = 626,
        Tetlin = 627,
        Tok = 628,
        Tyonek = 629,
        Venetie = 630,
        Wiseman = 631,
        TlingitHaida = 632,
        Tsimshian = 633,
        Angoon = 634,
        CentralCouncilOfTlingitAndHaidaTribes = 635,
        Chilkat = 636,
        Chilkoot = 637,
        Craig = 638,
        Douglas = 639,
        Haida = 640,
        Hoonah = 641,
        Hydaburg = 642,
        Kake = 643,
        Kasaan = 644,
        Kenaitze = 645,
        Ketchikan = 646,
        Klawock = 647,
        Pelican = 648,
        Petersburg = 649,
        Saxman = 650,
        Sitka = 651,
        TenakeeSprings = 652,
        Tlingit = 653,
        Wrangell = 654,
        Yakutat = 655,
        Metlakatla = 656,
        GreenlandEskimo = 657,
        InupiatEskimo = 658,
        SiberianEskimo = 659,
        YupikEskimo = 660,
        Ambler = 661,
        Anaktuvuk = 662,
        AnaktuvukPass = 663,
        ArcticSlopeInupiat = 664,
        ArcticSlopeCorporation = 665,
        Atqasuk = 666,
        Barrow = 667,
        BeringStraitsInupiat = 668,
        BrevigMission = 669,
        Buckland = 670,
        Chinik = 671,
        Council = 672,
        Deering = 673,
        Elim = 674,
        Golovin = 675,
        InalikDiomede = 676,
        Inupiaq = 677,
        Kaktovik = 678,
        Kawerak = 679,
        Kiana = 680,
        Kivalina = 681,
        Kobuk = 682,
        Kotzebue = 683,
        Koyuk = 684,
        Kwiguk = 685,
        MaunelukInupiat = 686,
        NanaInupiat = 687,
        Noatak = 688,
        Nome = 689,
        Noorvik = 690,
        Nuiqsut = 691,
        PointHope = 692,
        PointLay = 693,
        Selawik = 694,
        Shaktoolik = 695,
        Shishmaref = 696,
        Shungnak = 697,
        Solomon = 698,
        Teller = 699,
        Unalakleet = 700,
        Wainwright = 701,
        Wales = 702,
        WhiteMountain = 703,
        WhiteMountainInupiat = 704,
        MarysIgloo = 705,
        Gambell = 706,
        Savoonga = 707,
        SiberianYupik = 708,
        Akiachak = 709,
        Akiak = 710,
        Alakanuk = 711,
        Aleknagik = 712,
        Andreafsky = 713,
        Aniak = 714,
        Atmautluak = 715,
        Bethel = 716,
        BillMooresSlough = 717,
        BristolBayYupik = 718,
        CalistaYupik = 719,
        Chefornak = 720,
        Chevak = 721,
        Chuathbaluk = 722,
        ClarksPoint = 723,
        CrookedCreek = 724,
        Dillingham = 725,
        Eek = 726,
        Ekuk = 727,
        Ekwok = 728,
        Emmonak = 729,
        GoodnewsBay = 730,
        HooperBay = 731,
        IqurmuitRussianMission = 732,
        Kalskag = 733,
        Kasigluk = 734,
        Kipnuk = 735,
        Koliganek = 736,
        Kongiganak = 737,
        Kotlik = 738,
        Kwethluk = 739,
        Kwigillingok = 740,
        Levelock = 741,
        LowerKalskag = 742,
        Manokotak = 743,
        Marshall = 744,
        Mekoryuk = 745,
        MountainVillage = 746,
        Naknek = 747,
        Napaumute = 748,
        Napakiak = 749,
        Napaskiak = 750,
        Newhalen = 751,
        NewStuyahok = 752,
        Newtok = 753,
        Nightmute = 754,
        Nunapitchukv = 755,
        Oscarville = 756,
        PilotStation = 757,
        PitkasPoint = 758,
        Platinum = 759,
        PortageCreek = 760,
        Quinhagak = 761,
        RedDevil = 762,
        StMichael = 763,
        ScammonBay = 764,
        SheldonsPoint = 765,
        Sleetmute = 766,
        Stebbins = 767,
        Togiak = 768,
        Toksook = 769,
        Tulukskak = 770,
        Tuntutuliak = 771,
        Tununak = 772,
        TwinHills = 773,
        GeorgetownYupikEskimo = 774,
        StMarys = 775,
        Umkumiate = 776,
        AlutiiqAleut = 777,
        BristolBayAleut = 778,
        ChugachAleut = 779,
        Eyak = 780,
        KoniagAleut = 781,
        Sugpiaq = 782,
        Suqpigaq = 783,
        UnanganAleut = 784,
        Tatitlek = 785,
        Ugashik = 786,
        Chignik = 787,
        ChignikLake = 788,
        Egegik = 789,
        Igiugig = 790,
        IvanofBay = 791,
        KingSalmon = 792,
        Kokhanok = 793,
        Perryville = 794,
        PilotPoint = 795,
        PortHeiden = 796,
        Chenega = 797,
        ChugachCorporation = 798,
        EnglishBay = 799,
        PortGraham = 800,
        Akhiok = 801,
        Agdaagux = 802,
        Karluk = 803,
        Kodiak = 804,
        LarsenBay = 805,
        OldHarbor = 806,
        Ouzinkie = 807,
        PortLions = 808,
        Akutan = 809,
        AleutCorporation = 810,
        Aleutian = 811,
        AleutianIslander = 812,
        Atka = 813,
        Belkofski = 814,
        ChignikLagoon = 815,
        KingCove = 816,
        FalsePass = 817,
        NelsonLagoon = 818,
        Nikolski = 819,
        PauloffHarbor = 820,
        QaganToyagungin = 821,
        Qawalangin = 822,
        StGeorge = 823,
        StPaul = 824,
        SandPoint = 825,
        SouthNaknek = 826,
        Unalaska = 827,
        Unga = 828,
        AsianIndian = 829,
        Bangladeshi = 830,
        Bhutanese = 831,
        Burmese = 832,
        Cambodian = 833,
        Chinese = 834,
        Taiwanese = 835,
        Filipino = 836,
        Hmong = 837,
        Indonesian = 838,
        Japanese = 839,
        Korean = 840,
        Laotian = 841,
        Malaysian = 842,
        Okinawan = 843,
        Pakistani = 844,
        SriLankan = 845,
        Thai = 846,
        Vietnamese = 847,
        IwoJiman = 848,
        Maldivian = 849,
        Nepalese = 850,
        Singaporean = 851,
        Madagascar = 852,
        Black = 853,
        AfricanAmerican = 854,
        African = 855,
        Bahamian = 856,
        Barbadian = 857,
        Dominican = 858,
        DominicaIslander = 859,
        Haitian = 860,
        Jamaican = 861,
        Tobagoan = 862,
        Trinidadian = 863,
        WestIndian = 864,
        Botswanan = 865,
        Ethiopian = 866,
        Liberian = 867,
        Namibian = 868,
        Nigerian = 869,
        Zairean = 870,
        Polynesian = 871,
        Micronesian = 872,
        Melanesian = 873,
        OtherPacificIslander = 874,
        NativeHawaiian = 875,
        Samoan = 876,
        Tahitian = 877,
        Tongan = 878,
        Tokelauan = 879,
        GuamanianOrChamorro = 880,
        Guamanian = 881,
        Chamorro = 882,
        MarianaIslander = 883,
        Marshallese = 884,
        Palauan = 885,
        Carolinian = 886,
        Kosraean = 887,
        Pohnpeian = 888,
        Saipanese = 889,
        Kiribati = 890,
        Chuukese = 891,
        Yapese = 892,
        Fijian = 893,
        PapuaNewGuinean = 894,
        SolomonIslander = 895,
        NewHebrides = 896,
        European = 897,
        MiddleEasternOrNorthAfrican = 898,
        Arab = 899,
        Armenian = 900,
        English = 901,
        French = 902,
        German = 903,
        Irish = 904,
        Italian = 905,
        Polish = 906,
        Scottish = 907,
        Assyrian = 908,
        Egyptian = 909,
        Iranian = 910,
        Iraqi = 911,
        Lebanese = 912,
        Palestinian = 913,
        Syrian = 914,
        Afghanistani = 915,
        Israeili = 916,
        OtherRace = 917,
    }
}
/// The US Core Narrative Status Value Set limits the text status for the
/// resource narrative. See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-narrative-status>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct NarrativeStatusValueSet {
}
/// Nested message and enum types in `NarrativeStatusValueSet`.
pub mod narrative_status_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Additional = 1,
        Generated = 2,
    }
}
/// The codes for the ethnicity categories - 'Hispanic or Latino' and 'Non
/// Hispanic or Latino' - as defined by the [OMB Standards for Maintaining,
/// Collecting, and Presenting Federal Data on Race and Ethnicity, Statistical
/// Policy Directive No. 15, as revised, October 30,
/// 1997](<https://www.whitehouse.gov/omb/fedreg_1997standards>). See
/// <http://hl7.org/fhir/us/core/ValueSet/omb-ethnicity-category>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct OmbEthnicityCategoriesValueSet {
}
/// Nested message and enum types in `OmbEthnicityCategoriesValueSet`.
pub mod omb_ethnicity_categories_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        HispanicOrLatino = 1,
        NotHispanicOrLatino = 2,
    }
}
/// The codes for the concepts 'Unknown' and  'Asked but no answer' and the the
/// codes for the five race categories - 'American Indian' or 'Alaska Native',
/// 'Asian', 'Black or African American', 'Native Hawaiian or Other Pacific
/// Islander', and 'White' - as defined by the [OMB Standards for Maintaining,
/// Collecting, and Presenting Federal Data on Race and Ethnicity, Statistical
/// Policy Directive No. 15, as revised, October 30,
/// 1997](<https://www.whitehouse.gov/omb/fedreg_1997standards>) . See
/// <http://hl7.org/fhir/us/core/ValueSet/omb-race-category>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct OmbRaceCategoriesValueSet {
}
/// Nested message and enum types in `OmbRaceCategoriesValueSet`.
pub mod omb_race_categories_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        AmericanIndianOrAlaskaNative = 1,
        Asian = 2,
        BlackOrAfricanAmerican = 3,
        NativeHawaiianOrOtherPacificIslander = 4,
        White = 5,
        Unk = 6,
        Asku = 7,
    }
}
/// Documentation of substances suspected of (or not suspected of) causing an
/// allergy or intolerance reaction in an individual. **Inclusion Criteria:**
/// specific or general substances to which a patient may be exposed and which
/// may be suspected of causing an adverse reaction; assertions refuting these
/// suspicions. This includes:  1. Common dietary substances for allergy and
/// intolerance documentation (SNOMEDCT) 2. Common drug classes for allergy and
/// intolerance documentation (SNOMEDCT) 3. Common drug substances for allergy
/// and intolerance documentation (RXNORM) 4. Common environmental substances for
/// allergy and intolerance documentation (SNOMEDCT) 5. Common refutations and
/// null values for substance causes for allergy and intolerance documentation
/// (SNOMEDCT)  **Exclusion Criteria:** actual conditions caused by exposure
/// (reactions, allergies) See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-allergy-substance>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreAllergySubstanceValueSet {
}
/// Nested message and enum types in `USCoreAllergySubstanceValueSet`.
pub mod us_core_allergy_substance_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        FormoterolMometasone = 1,
        LactaseRennet = 2,
        AcetaminophenCaffeineChlorpheniramineHydrocodonePhenylephrine = 3,
        GuaiacolsulfonateHydrocodone = 4,
        AmpicillinSulbactam = 5,
        Streptomycin = 6,
        Succinylcholine = 7,
        Sucralfate = 8,
        Sulfacetamide = 9,
        Sulfadiazine = 10,
        Sulfamethoxazole = 11,
        Sulfisoxazole = 12,
        Sulfur = 13,
        Sulindac = 14,
        Tamoxifen = 15,
        Temazepam = 16,
        Terbutaline = 17,
        DabigatranEtexilate = 18,
        Testosterone = 19,
        Tetracycline = 20,
        CarbidopaLevodopa = 21,
        Lurasidone = 22,
        Theophylline = 23,
        Thimerosal = 24,
        Thiopental = 25,
        Thioridazine = 26,
        Thiothixene = 27,
        Levothyroxine = 28,
        Ticlopidine = 29,
        Timolol = 30,
        Tobramycin = 31,
        Tolmetin = 32,
        Tramadol = 33,
        Trazodone = 34,
        Triamcinolone = 35,
        EpinephrineLidocaine = 36,
        Triamterene = 37,
        Triazolam = 38,
        Trifluoperazine = 39,
        Mometasone = 40,
        Trimethoprim = 41,
        SulfamethoxazoleTrimethoprim = 42,
        Vancomycin = 43,
        Rivaroxaban = 44,
        Ticagrelor = 45,
        Verapamil = 46,
        VitaminB12 = 47,
        VitaminD = 48,
        VitaminE = 49,
        Warfarin = 50,
        ErythromycinSulfisoxazole = 51,
        Zinc = 52,
        ZincOxide = 53,
        Levetiracetam = 54,
        Zafirlukast = 55,
        Rabeprazole = 56,
        AscorbicAcid = 57,
        Ibandronate = 58,
        Trovafloxacin = 59,
        Ziprasidone = 60,
        Aspirin = 61,
        Tolterodine = 62,
        Atenolol = 63,
        Rituximab = 64,
        Atropine = 65,
        Azathioprine = 66,
        Aztreonam = 67,
        Bacitracin = 68,
        Baclofen = 69,
        Gadolinium = 70,
        Xanthine = 71,
        Aspartame = 72,
        Nickel = 73,
        Latex = 74,
        BariumSulfate = 75,
        Brimonidine = 76,
        Beclomethasone = 77,
        Donepezil = 78,
        Zolmitriptan = 79,
        BelladonnaAlkaloids = 80,
        SulfurDioxide = 81,
        EthylEther = 82,
        Sildenafil = 83,
        Apixaban = 84,
        Gemifloxacin = 85,
        Moxifloxacin = 86,
        Benzocaine = 87,
        Celecoxib = 88,
        BenzoinResin = 89,
        Colesevelam = 90,
        BenzoylPeroxide = 91,
        Benztropine = 92,
        Betamethasone = 93,
        Hyoscyamine = 94,
        Gentamicin = 95,
        Mirtazapine = 96,
        Acetaminophen = 97,
        Acarbose = 98,
        Acetazolamide = 99,
        Lansoprazole = 100,
        TetanusImmuneGlobulin = 101,
        Alfuzosin = 102,
        Amlodipine = 103,
        Buspirone = 104,
        Rivastigmine = 105,
        Butorphanol = 106,
        Azithromycin = 107,
        Pregabalin = 108,
        Caffeine = 109,
        Benazepril = 110,
        Calcium = 111,
        CalciumCarbonate = 112,
        Benzonatate = 113,
        Linezolid = 114,
        Infliximab = 115,
        BismuthSubsalicylate = 116,
        Cefprozil = 117,
        AmoxicillinClavulanate = 118,
        Budesonide = 119,
        Captopril = 120,
        Carbamazepine = 121,
        Carvedilol = 122,
        Cefepime = 123,
        Cefpodoxime = 124,
        Cetirizine = 125,
        Carisoprodol = 126,
        Cilostazol = 127,
        CitricAcid = 128,
        Clarithromycin = 129,
        AcetaminophenButalbitalCaffeine = 130,
        AcetaminophenDichloralphenazoneIsometheptene = 131,
        AspirinButalbitalCaffeine = 132,
        AspirinButalbitalCaffeineCodeine = 133,
        AcetaminophenDiphenhydramine = 134,
        AcetaminophenHydrocodone = 135,
        AcetaminophenOxycodone = 136,
        AlbuterolIpratropium = 137,
        AmlodipineBenazepril = 138,
        AspirinCaffeine = 139,
        AspirinOxycodone = 140,
        AspirinPentazocine = 141,
        BisoprololHydrochlorothiazide = 142,
        CaffeineErgotamine = 143,
        Candesartan = 144,
        CarbinoxaminePseudoephedrine = 145,
        ChlorpheniramineHydrocodone = 146,
        CodeineGuaifenesin = 147,
        CodeinePseudoephedrine = 148,
        DextromethorphanGuaifenesin = 149,
        DiclofenacMisoprostol = 150,
        Etanercept = 151,
        EthinylEstradiolLevonorgestrel = 152,
        FexofenadinePseudoephedrine = 153,
        GuaifenesinPseudoephedrine = 154,
        HomatropineHydrocodone = 155,
        HydrochlorothiazideIrbesartan = 156,
        HydrochlorothiazideLisinopril = 157,
        HydrochlorothiazideLosartan = 158,
        HydrochlorothiazideValsartan = 159,
        HydrocodoneIbuprofen = 160,
        HydrocodonePseudoephedrine = 161,
        LoratadinePseudoephedrine = 162,
        NaloxonePentazocine = 163,
        PseudoephedrineTriprolidine = 164,
        Cefaclor = 165,
        HydrocortisoneNeomycinPolymyxinB = 166,
        Cefadroxil = 167,
        Cefazolin = 168,
        Cefoxitin = 169,
        Ceftazidime = 170,
        Ceftriaxone = 171,
        PolymyxinBTrimethoprim = 172,
        IronPolysaccharide = 173,
        Cefuroxime = 174,
        Cyclobenzaprine = 175,
        PolyethyleneGlycol3350 = 176,
        Daptomycin = 177,
        Cephalexin = 178,
        AspirinDipyridamole = 179,
        Gatifloxacin = 180,
        Dutasteride = 181,
        Rofecoxib = 182,
        Dronedarone = 183,
        Chloramphenicol = 184,
        Chlordiazepoxide = 185,
        Chlorhexidine = 186,
        Trospium = 187,
        Levalbuterol = 188,
        Chloroquine = 189,
        Chlorpheniramine = 190,
        Chlorpromazine = 191,
        Chlorthalidone = 192,
        Chlorzoxazone = 193,
        Cholecalciferol = 194,
        CholestyramineResin = 195,
        Etodolac = 196,
        FerrousSulfate = 197,
        Finasteride = 198,
        Cefixime = 199,
        Cefdinir = 200,
        Flunisolide = 201,
        Formoterol = 202,
        BeePollen = 203,
        Cimetidine = 204,
        Gabapentin = 205,
        Ciprofloxacin = 206,
        Citalopram = 207,
        Glimepiride = 208,
        Clindamycin = 209,
        HydrochlorothiazideTriamterene = 210,
        Clonazepam = 211,
        Clonidine = 212,
        Oseltamivir = 213,
        Ondansetron = 214,
        Clotrimazole = 215,
        Codeine = 216,
        Colchicine = 217,
        Colestipol = 218,
        Leflunomide = 219,
        InsulinGlargine = 220,
        Telithromycin = 221,
        IodinatedGlycerol = 222,
        Valdecoxib = 223,
        Itraconazole = 224,
        Acyclovir = 225,
        Esomeprazole = 226,
        Travoprost = 227,
        Lamotrigine = 228,
        FluticasoneSalmeterol = 229,
        Cortisone = 230,
        Loratadine = 231,
        Loracarbef = 232,
        Lisinopril = 233,
        MercuryAmmoniated = 234,
        Meropenem = 235,
        Adenosine = 236,
        Cyclosporine = 237,
        Rosuvastatin = 238,
        Vardenafil = 239,
        Dapsone = 240,
        Prasterone = 241,
        Nabumetone = 242,
        Nebivolol = 243,
        Nefazodone = 244,
        NickelSulfate = 245,
        AloeVeraPreparation = 246,
        Olmesartan = 247,
        Escitalopram = 248,
        Solifenacin = 249,
        Desipramine = 250,
        Ertapenem = 251,
        Oxaliplatin = 252,
        Oxaprozin = 253,
        Oxcarbazepine = 254,
        Dexamethasone = 255,
        Oxybutynin = 256,
        Adalimumab = 257,
        Dextromethorphan = 258,
        Paroxetine = 259,
        Clopidogrel = 260,
        Diazepam = 261,
        Phenyltoloxamine = 262,
        Diclofenac = 263,
        Dicloxacillin = 264,
        Dicyclomine = 265,
        Pioglitazone = 266,
        Diflunisal = 267,
        Digoxin = 268,
        Ezetimibe = 269,
        Dihydroergotamine = 270,
        Hydromorphone = 271,
        Diltiazem = 272,
        Dimenhydrinate = 273,
        Diphenhydramine = 274,
        Quinapril = 275,
        Dipyridamole = 276,
        AcetaminophenTramadol = 277,
        Ramipril = 278,
        Resorcinol = 279,
        Risperidone = 280,
        Tadalafil = 281,
        Ketorolac = 282,
        Ranolazine = 283,
        Salsalate = 284,
        Salmeterol = 285,
        Dobutamine = 286,
        Doxepin = 287,
        Doxycycline = 288,
        Sertraline = 289,
        Droperidol = 290,
        Simvastatin = 291,
        Sumatriptan = 292,
        Tazobactam = 293,
        Terazosin = 294,
        Terbinafine = 295,
        Enalapril = 296,
        Enalaprilat = 297,
        Atomoxetine = 298,
        Topiramate = 299,
        Torsemide = 300,
        Trichloroacetaldehyde = 301,
        Trimethobenzamide = 302,
        BudesonideFormoterol = 303,
        Ephedrine = 304,
        Venlafaxine = 305,
        Epinephrine = 306,
        Zolpidem = 307,
        Zonisamide = 308,
        Carboplatin = 309,
        DexbrompheniraminePseudoephedrine = 310,
        Ergotamine = 311,
        Valproate = 312,
        Erythromycin = 313,
        Zileuton = 314,
        Pantoprazole = 315,
        Estradiol = 316,
        EstrogensConjugatedUsp = 317,
        Fluticasone = 318,
        Fluvastatin = 319,
        EthinylEstradiol = 320,
        Lactase = 321,
        Meloxicam = 322,
        Terfenadine = 323,
        Misoprostol = 324,
        Bupropion = 325,
        LithiumCarbonate = 326,
        Mupirocin = 327,
        Pravastatin = 328,
        Famotidine = 329,
        Felodipine = 330,
        Fentanyl = 331,
        Albuterol = 332,
        Latanoprost = 333,
        FishOils = 334,
        Flecainide = 335,
        Fluconazole = 336,
        Ethanol = 337,
        Fluorouracil = 338,
        Fluoxetine = 339,
        Fluphenazine = 340,
        Flurandrenolide = 341,
        Formaldehyde = 342,
        Furosemide = 343,
        Alendronate = 344,
        Eszopiclone = 345,
        Galantamine = 346,
        CiprofloxacinDexamethasone = 347,
        DiphenhydramineZincAcetate = 348,
        NeomycinPolymyxinB = 349,
        AspirinCaffeineOrphenadrine = 350,
        PenicillinGBenzathinePenicillinGProcaine = 351,
        AcetaminophenDextromethorphanDiphenhydraminePseudoephedrine = 352,
        AcetaminophenAspirinCaffeine = 353,
        Gemfibrozil = 354,
        Liraglutide = 355,
        Glyburide = 356,
        Clavulanate = 357,
        Glipizide = 358,
        AcetaminophenPropoxyphene = 359,
        ChlorhexidineIsopropylAlcohol = 360,
        EzetimibeSimvastatin = 361,
        Glucose = 362,
        Nitroglycerin = 363,
        Doxazosin = 364,
        Fosinopril = 365,
        Griseofulvin = 366,
        Guaifenesin = 367,
        Haloperidol = 368,
        Quetiapine = 369,
        Allopurinol = 370,
        Losartan = 371,
        Heparin = 372,
        Mesalamine = 373,
        Hydralazine = 374,
        Hydrochlorothiazide = 375,
        Hydrocodone = 376,
        Hydrocortisone = 377,
        HydrogenPeroxide = 378,
        Hydroxychloroquine = 379,
        Hydroxyzine = 380,
        Ibuprofen = 381,
        Imipramine = 382,
        Paclitaxel = 383,
        Tizanidine = 384,
        Indapamide = 385,
        Indomethacin = 386,
        Milnacipran = 387,
        Metaxalone = 388,
        Varenicline = 389,
        Iodine = 390,
        Sitagliptin = 391,
        AtropineDiphenoxylate = 392,
        Iohexol = 393,
        Alprazolam = 394,
        Cerivastatin = 395,
        BrimonidineTimolol = 396,
        IronDextranComplex = 397,
        Dorzolamide = 398,
        Isoniazid = 399,
        Exenatide = 400,
        Isosorbide = 401,
        IsosorbideDinitrate = 402,
        ChlordiazepoxideClidinium = 403,
        Ketamine = 404,
        Ketoconazole = 405,
        Olanzapine = 406,
        Ketoprofen = 407,
        Labetalol = 408,
        Amantadine = 409,
        Lactulose = 410,
        Lanolin = 411,
        Lidocaine = 412,
        Lincomycin = 413,
        Lithium = 414,
        BacitracinPolymyxinB = 415,
        Loperamide = 416,
        Lorazepam = 417,
        Lovastatin = 418,
        Magnesium = 419,
        MagnesiumSulfate = 420,
        DorzolamideTimolol = 421,
        Meclizine = 422,
        Medroxyprogesterone = 423,
        Enoxaparin = 424,
        Melatonin = 425,
        Memantine = 426,
        Menthol = 427,
        Meperidine = 428,
        Metformin = 429,
        Methadone = 430,
        Methimazole = 431,
        Methocarbamol = 432,
        Methotrexate = 433,
        Methyldopa = 434,
        Aminophylline = 435,
        OxytetracyclinePolymyxinB = 436,
        AspirinCaffeinePropoxyphene = 437,
        AcetaminophenAspirinPhenylpropanolamine = 438,
        AcetaminophenBrompheniraminePseudoephedrine = 439,
        AcetaminophenButalbitalCaffeineCodeine = 440,
        AcetaminophenChlorpheniramineDextromethorphanPseudoephedrine = 441,
        AtropineHyoscyaminePhenobarbitalScopolamine = 442,
        BacitracinHydrocortisoneNeomycinPolymyxinB = 443,
        BenzalkoniumLidocaine = 444,
        Methylphenidate = 445,
        Methylprednisolone = 446,
        DiphenhydraminePhenylephrine = 447,
        BrompheniramineDextromethorphanPseudoephedrine = 448,
        Tiotropium = 449,
        Metoclopramide = 450,
        Metolazone = 451,
        Metoprolol = 452,
        Metronidazole = 453,
        BacitracinNeomycinPolymyxinB = 454,
        GramicidinNeomycinPolymyxinB = 455,
        Miconazole = 456,
        Midazolam = 457,
        Valsartan = 458,
        Minocycline = 459,
        Minoxidil = 460,
        Amiodarone = 461,
        Amitriptyline = 462,
        Morphine = 463,
        AcetaminophenDextromethorphanDoxylamine = 464,
        Ipratropium = 465,
        Raloxifene = 466,
        Fosphenytoin = 467,
        Amoxicillin = 468,
        Ropinirole = 469,
        Nafcillin = 470,
        Nalbuphine = 471,
        Naltrexone = 472,
        Amphetamine = 473,
        Naproxen = 474,
        Duloxetine = 475,
        Neomycin = 476,
        Risedronate = 477,
        Ampicillin = 478,
        Telmisartan = 479,
        Valacyclovir = 480,
        Niacin = 481,
        Nicotine = 482,
        PiperacillinTazobactam = 483,
        Nifedipine = 484,
        Nitrofurantoin = 485,
        Pramipexole = 486,
        NitrousOxide = 487,
        Norfloxacin = 488,
        Nortriptyline = 489,
        Nystatin = 490,
        Ofloxacin = 491,
        Omeprazole = 492,
        Opium = 493,
        Orphenadrine = 494,
        Tamsulosin = 495,
        Oxycodone = 496,
        Oxytetracycline = 497,
        Tapentadol = 498,
        Penicillamine = 499,
        IsopropylAlcohol = 500,
        PenicillinG = 501,
        PenicillinV = 502,
        Pentamidine = 503,
        Pentazocine = 504,
        Phenazopyridine = 505,
        Phenobarbital = 506,
        DextromethorphanDoxylamine = 507,
        Phenylephrine = 508,
        Dexlansoprazole = 509,
        Phenylpropanolamine = 510,
        AcetaminophenCodeine = 511,
        AspirinCalciumCarbonate = 512,
        Phenytoin = 513,
        Levofloxacin = 514,
        AmphetamineAspartateAmphetamineSulfateDextroamphetamineSaccharateDextroamphetamineSulfate = 515,
        Atorvastatin = 516,
        Piroxicam = 517,
        Irbesartan = 518,
        Rosiglitazone = 519,
        PolymyxinB = 520,
        Saxagliptin = 521,
        Potassium = 522,
        PotassiumChloride = 523,
        Povidone = 524,
        PovidoneIodine = 525,
        Pitavastatin = 526,
        Prazosin = 527,
        Prednisolone = 528,
        Prednisone = 529,
        Primaquine = 530,
        Primidone = 531,
        Probenecid = 532,
        Procainamide = 533,
        Procaine = 534,
        Fenofibrate = 535,
        Prochlorperazine = 536,
        Progesterone = 537,
        Promethazine = 538,
        Propafenone = 539,
        Fexofenadine = 540,
        Propofol = 541,
        Propoxyphene = 542,
        Propranolol = 543,
        Propylthiouracil = 544,
        Rizatriptan = 545,
        Montelukast = 546,
        DexamethasoneTobramycin = 547,
        Pseudoephedrine = 548,
        Aripiprazole = 549,
        Psyllium = 550,
        PurifiedProteinDerivativeOfTuberculin = 551,
        Iron = 552,
        Quinidine = 553,
        Quinine = 554,
        AloeExtract = 555,
        Ranitidine = 556,
        Rifampin = 557,
        Sulfasalazine = 558,
        Scopolamine = 559,
        Silicones = 560,
        SilverSulfadiazine = 561,
        Sotalol = 562,
        Spironolactone = 563,
        CitrusFruitSubstance = 564,
        StrawberrySubstance = 565,
        ChocolateSubstance = 566,
        EggsEdibleSubstance = 567,
        CheeseSubstance = 568,
        LatexSubstance = 569,
        AnabolicSteroidSubstance = 570,
        AspartameSubstance = 571,
        ArtificialSweetenerSubstance = 572,
        SteroidSubstance = 573,
        NutSubstance = 574,
        SubstanceWithAminoglycosideStructureAndAntibacterialMechanismOfActionSubstance = 575,
        BuckwheatCerealSubstance = 576,
        WheatgermSubstance = 577,
        DairyFoodsSubstance = 578,
        RedMeatSubstance = 579,
        BeefSubstance = 580,
        PorkSubstance = 581,
        ChickenMeatSubstance = 582,
        TurkeyMeatSubstance = 583,
        TunaFishSubstance = 584,
        PrawnsSubstance = 585,
        AbaloneCannedInBrineSubstance = 586,
        AubergineSubstance = 587,
        PulseVegetablesSubstance = 588,
        CinnamonSubstance = 589,
        GingerSubstance = 590,
        CranberriesSubstance = 591,
        RaspberriesSubstance = 592,
        CashewNutSubstance = 593,
        PistachioNutSubstance = 594,
        HoneySubstance = 595,
        SodiumNitrateSubstance = 596,
        AnticonvulsantSubstance = 597,
        SalicylateSubstance = 598,
        CaffeineSubstance = 599,
        PollenSubstance = 600,
        GrassPollenSubstance = 601,
        OrangeFruitSubstance = 602,
        BananaSubstance = 603,
        PineappleSubstance = 604,
        GrapefruitSubstance = 605,
        GrapesSubstance = 606,
        CarrotSubstance = 607,
        CelerySubstance = 608,
        SpinachSubstance = 609,
        AlmondSubstance = 610,
        BrazilNutSubstance = 611,
        WalnutNutSubstance = 612,
        HazelnutSubstance = 613,
        BeanSubstance = 614,
        HorseDanderSubstance = 615,
        WaspVenomSubstance = 616,
        VaricellaZosterVirusAntibodySubstance = 617,
        CatDanderSubstance = 618,
        DogDanderSubstance = 619,
        SesameSeedSubstance = 620,
        KiwiFruitSubstance = 621,
        MelonSubstance = 622,
        MangoFruitSubstance = 623,
        PeasSubstance = 624,
        PecanNutSubstance = 625,
        SunflowerSeedSubstance = 626,
        AnimalDanderSubstance = 627,
        SeedSubstance = 628,
        PoultrySubstance = 629,
        BeeVenomSubstance = 630,
        CoconutOilSubstance = 631,
        CoffeeSubstance = 632,
        CorticosteroidAndCorticosteroidDerivativeSubstance = 633,
        DustSubstance = 634,
        DiphtheriaPlusTetanusVaccineProduct = 635,
        WineSubstance = 636,
        NitrofuranDerivativeSubstance = 637,
        SodiumSulfiteSubstance = 638,
        SubstanceWithMacrolideStructureAndAntibacterialMechanismOfActionSubstance = 639,
        BenzodiazepineSubstance = 640,
        NonSteroidalAntiInflammatoryAgentSubstance = 641,
        SulfonylureaSubstance = 642,
        SubstanceWithQuinoloneStructureAndAntibacterialMechanismOfActionSubstance = 643,
        SubstanceWithAngiotensinConvertingEnzymeInhibitorMechanismOfActionSubstance = 644,
        ThiazideDiureticSubstance = 645,
        AntiparkinsonianAgentSubstance = 646,
        BarbiturateSubstance = 647,
        SubstanceWithHistamineReceptorAntagonistMechanismOfActionSubstance = 648,
        FirstGenerationCephalosporinSubstance = 649,
        SubstanceWith3Hydroxy3MethylglutarylCoenzymeAReductaseInhibitorMechanismOfActionSubstance = 650,
        SubstanceWithAngiotensinIiReceptorAntagonistMechanismOfActionSubstance = 651,
        SubstanceWithTetracyclineStructureAndAntibacterialMechanismOfActionSubstance = 652,
        TricyclicAntidepressantSubstance = 653,
        SubstanceWithBetaAdrenergicReceptorAntagonistMechanismOfActionSubstance = 654,
        SubstanceWithCephalosporinStructureAndAntibacterialMechanismOfActionSubstance = 655,
        SubstanceWithPenicillinStructureAndAntibacterialMechanismOfActionSubstance = 656,
        SubstanceWithBetaLactamStructureAndAntibacterialMechanismOfActionSubstance = 657,
        SubstanceWithCalciumChannelBlockerMechanismOfActionSubstance = 658,
        GelatinSubstance = 659,
        ContrastMediaSubstance = 660,
        FormulaMilkSubstance = 661,
        PlasmaProteinFractionSubstance = 662,
        SubstanceWithProstaglandinEndoperoxideSynthaseIsoform2InhibitorMechanismOfActionSubstance = 663,
        SulfonamideSubstance = 664,
        AlmondOilSubstance = 665,
        AloeSubstance = 666,
        CarbapenemSubstance = 667,
        AnthraxVaccineSubstance = 668,
        InfluenzaVirusVaccineSubstance = 669,
        PertussisVaccineSubstance = 670,
        SmallpoxVaccineSubstance = 671,
        TyphoidVaccineSubstance = 672,
        VaricellaVirusVaccineSubstance = 673,
        PneumococcalVaccineSubstance = 674,
        HydrocolloidSubstance = 675,
        SubstanceWithOpioidReceptorAgonistMechanismOfActionSubstance = 676,
        CarbamateSubstance = 677,
        NoKnownDrugAllergySituation = 678,
        BlueberriesSubstance = 679,
        CantaloupeSubstance = 680,
        PepperSubstance = 681,
        RyeSubstance = 682,
        WheatSubstance = 683,
        HorseSerumProteinSubstance = 684,
        CornSubstance = 685,
        DiphtheriaPlusPertussisPlusTetanusPlusHaemophilusInfluenzaeTypeBVaccineProduct = 686,
        TetanusVaccineSubstance = 687,
        WheatBranSubstance = 688,
        YeastSubstance = 689,
        BeePollenSubstance = 690,
        EstrogenSubstance = 691,
        ArachisOilSubstance = 692,
        MethadoneAnalogSubstance = 693,
        OatsSubstance = 694,
        AdhesiveAgentSubstance = 695,
        WatermelonSubstance = 696,
        GlucocorticoidSubstance = 697,
        DiphtheriaPlusPertussisPlusTetanusVaccineProduct = 698,
        ProductContainingBetaGalactosidaseMedicinalProduct = 699,
        IodinatedContrastMediaSubstance = 700,
        NoKnownEnvironmentalAllergySituation = 701,
        NoKnownFoodAllergySituation = 702,
        SulfurSubstance = 703,
        NickelCompoundSubstance = 704,
        SeafoodSubstance = 705,
        BlueFoodColoringSubstance = 706,
        TreeNutSubstance = 707,
        PepperoniSubstance = 708,
        IodineSubstance = 709,
        RedFoodColoringSubstance = 710,
        YellowFoodColoringSubstance = 711,
        LactoseSubstance = 712,
        FoodPreservativeSubstance = 713,
        MustardSubstance = 714,
        AlcoholSubstance = 715,
        DyeSubstance = 716,
        BerrySubstance = 717,
        RiceSubstance = 718,
        InsulinSubstance = 719,
        MilkSubstance = 720,
        LupineSeedSubstance = 721,
        NoKnownLatexAllergySituation = 722,
        NoKnownAllergySituation = 723,
        DustMiteProteinSubstance = 724,
        FruitSubstance = 725,
        YamSubstance = 726,
        TomatoSubstance = 727,
        SquidSubstance = 728,
        SalmonSubstance = 729,
        ShellfishSubstance = 730,
        GarlicSubstance = 731,
        MackerelSubstance = 732,
        MushroomSubstance = 733,
        OnionSubstance = 734,
        PeachSubstance = 735,
        PearSubstance = 736,
        PlumSubstance = 737,
        PotatoSubstance = 738,
        BroccoliSubstance = 739,
        BarleySubstance = 740,
        CoconutSubstance = 741,
        PapayaSubstance = 742,
        CucumberSubstance = 743,
        ApricotSubstance = 744,
        AppleSubstance = 745,
        CherrySubstance = 746,
        AvocadoSubstance = 747,
        LemonSubstance = 748,
        MarineMolluskSubstance = 749,
        FishSubstance = 750,
        MarineCrustaceanSubstance = 751,
        ScallopSubstance = 752,
        ClamSubstance = 753,
        OysterSubstance = 754,
        CrabSubstance = 755,
        LobsterSubstance = 756,
        SugarSubstance = 757,
        MonosodiumGlutamateSubstance = 758,
        PeanutSubstance = 759,
        SoyProteinSubstance = 760,
        FoodFlavoringAgentSubstance = 761,
        MoldOrganism = 762,
        NitrateSaltSubstance = 763,
        SesameOilSubstance = 764,
        GlutenSubstance = 765,
    }
}
/// The US Core Clinical Note Type Value Set is a 'starter set' of types
/// supported for fetching and storing clinical notes. See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-clinical-note-type>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreClinicalNoteTypeValueSet {
}
/// Nested message and enum types in `USCoreClinicalNoteTypeValueSet`.
pub mod us_core_clinical_note_type_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        V188425 = 1,
        V114884 = 2,
        V341172 = 3,
        V115063 = 4,
        V285700 = 5,
    }
}
/// The US Core Condition Category Codes support the separate concepts of
/// problems and health concerns in Condition.category in order for API consumers
/// to be able to separate health concerns and problems. However this is not
/// mandatory for 2015 certification See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-condition-category>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreConditionCategoryCodesValueSet {
}
/// Nested message and enum types in `USCoreConditionCategoryCodesValueSet`.
pub mod us_core_condition_category_codes_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        ProblemListItem = 1,
        EncounterDiagnosis = 2,
        HealthConcern = 3,
    }
}
/// This describes the problem. Diagnosis/Problem List is broadly defined as a
/// series of brief statements that catalog a patient's medical, nursing, dental,
/// social, preventative and psychiatric events and issues that are relevant to
/// that patient's healthcare (e.g., signs, symptoms, and defined conditions).
/// ICD-10 is appropriate for Diagnosis information, and ICD-9 for historical
/// information. See <http://hl7.org/fhir/us/core/ValueSet/us-core-condition-code>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreConditionCodeValueSet {
}
/// Nested message and enum types in `USCoreConditionCodeValueSet`.
pub mod us_core_condition_code_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        V160245001 = 1,
    }
}
/// The US Core Diagnostic Report Category Value Set is a 'starter set' of
/// categories supported for fetching and Diagnostic Reports and notes. See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-diagnosticreport-category>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreDiagnosticReportCategoryValueSet {
}
/// Nested message and enum types in `USCoreDiagnosticReportCategoryValueSet`.
pub mod us_core_diagnostic_report_category_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Lp296845 = 1,
        Lp297082 = 2,
        Lp78396 = 3,
    }
}
/// The US Core DocumentReference Type Value Set includes all LOINC  values whose
/// SCALE is DOC in the LOINC database and the HL7 v3 Code System NullFlavor
/// concept 'unknown' See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-documentreference-type>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreDocumentReferenceTypeValueSet {
}
/// Nested message and enum types in `USCoreDocumentReferenceTypeValueSet`.
pub mod us_core_document_reference_type_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Unk = 1,
    }
}
/// Codes providing the status of an observation for smoking status. Constrained
/// to `final`and `entered-in-error`. See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-observation-smoking-status-status>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreObservationSmokingStatusStatusValueSet {
}
/// Nested message and enum types in `USCoreObservationSmokingStatusStatusValueSet`.
pub mod us_core_observation_smoking_status_status_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Final = 1,
        EnteredInError = 2,
    }
}
/// The type of participation a provenance agent played for a given target.
/// See <http://hl7.org/fhir/us/core/ValueSet/us-core-provenance-participant-type>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreProvenancePaticipantTypeCodesValueSet {
}
/// Nested message and enum types in `USCoreProvenancePaticipantTypeCodesValueSet`.
pub mod us_core_provenance_paticipant_type_codes_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Transmitter = 1,
        Enterer = 2,
        Performer = 3,
        Author = 4,
        Verifier = 5,
        Legal = 6,
        Attester = 7,
        Informant = 8,
        Custodian = 9,
        Assembler = 10,
        Composer = 11,
    }
}
/// Provider roles codes which are composed of the NUCC Health Care Provider
/// Taxonomy Code Set classification codes for providers. Only concepts with a
/// classification and no specialization are included. See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-provider-role>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreProviderRoleNuccValueSet {
}
/// Nested message and enum types in `USCoreProviderRoleNuccValueSet`.
pub mod us_core_provider_role_nucc_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Counselor = 1,
        Psychoanalyst = 2,
        PoetryTherapist = 3,
        ClinicalNeuropsychologist = 4,
        BehaviorAnalyst = 5,
        Psychologist = 6,
        SocialWorker = 7,
        AssistantBehaviorAnalyst = 8,
        MarriageFamilyTherapist = 9,
        BehaviorTechnician = 10,
        Chiropractor = 11,
        Dentist = 12,
        Denturist = 13,
        DentalHygienist = 14,
        DentalTherapist = 15,
        AdvancedPracticeDentalTherapist = 16,
        OralMedicinist = 17,
        DentalAssistant = 18,
        DentalLaboratoryTechnician = 19,
        DietaryManager = 20,
        Nutritionist = 21,
        DietitianRegistered = 22,
        DieteticTechnicianRegistered = 23,
        PersonalEmergencyResponseAttendant = 24,
        EmergencyMedicalTechnicianParamedic = 25,
        EmergencyMedicalTechnicianIntermediate = 26,
        EmergencyMedicalTechnicianBasic = 27,
        Optometrist = 28,
        TechnicianTechnologist = 29,
        RegisteredNurse = 30,
        LicensedPracticalNurse = 31,
        LicensedVocationalNurse = 32,
        LicensedPsychiatricTechnician = 33,
        MedicalGeneticsPhDMedicalGenetics = 34,
        GeneticCounselorMs = 35,
        MilitaryHealthCareProvider = 36,
        Acupuncturist = 37,
        CaseManagerCareCoordinator = 38,
        Interpreter = 39,
        Contractor = 40,
        Driver = 41,
        Mechanotherapist = 42,
        Naprapath = 43,
        CommunityHealthWorker = 44,
        LegalMedicine173000000X = 45,
        Reflexologist = 46,
        SleepSpecialistPhD = 47,
        Meals = 48,
        Specialist = 49,
        HealthEducator = 50,
        Veterinarian = 51,
        LactationConsultantNonRn = 52,
        ClinicalEthicist = 53,
        Naturopath = 54,
        Homeopath = 55,
        MidwifeLay = 56,
        PeerSpecialist = 57,
        Midwife = 58,
        FuneralDirector = 59,
        Lodging = 60,
        Pharmacist = 61,
        PharmacyTechnician = 62,
        MultiSpecialty = 63,
        SingleSpecialty = 64,
        IndependentMedicalExaminer = 65,
        Phlebology = 66,
        NeuromusculoskeletalMedicineSportsMedicine = 67,
        NeuromusculoskeletalMedicineOmm = 68,
        OralMaxillofacialSurgery = 69,
        TransplantSurgery = 70,
        ElectrodiagnosticMedicine = 71,
        AllergyImmunology = 72,
        Anesthesiology = 73,
        Dermatology = 74,
        EmergencyMedicine = 75,
        FamilyMedicine = 76,
        InternalMedicine = 77,
        NeurologicalSurgery = 78,
        NuclearMedicine = 79,
        ObstetricsGynecology = 80,
        Ophthalmology = 81,
        OrthopaedicSurgery = 82,
        Otolaryngology = 83,
        Pediatrics = 84,
        PhysicalMedicineRehabilitation = 85,
        PlasticSurgery = 86,
        Surgery = 87,
        Urology = 88,
        ColonRectalSurgery = 89,
        GeneralPractice = 90,
        ThoracicSurgeryCardiothoracicVascularSurgery = 91,
        Hospitalist = 92,
        ClinicalPharmacology = 93,
        LegalMedicine209800000X = 94,
        AssistantPodiatric = 95,
        Podiatrist = 96,
        ArtTherapist = 97,
        DevelopmentalTherapist = 98,
        Orthotist = 99,
        MastectomyFitter = 100,
        Pedorthist = 101,
        Prosthetist = 102,
        ClinicalExercisePhysiologist = 103,
        OccupationalTherapyAssistant = 104,
        OrthoticFitter = 105,
        PhysicalTherapist = 106,
        PhysicalTherapyAssistant = 107,
        RehabilitationPractitioner = 108,
        SpecialistTechnologist225500000X = 109,
        DanceTherapist = 110,
        MassageTherapist = 111,
        RecreationTherapist = 112,
        MusicTherapist = 113,
        PulmonaryFunctionTechnologist = 114,
        RehabilitationCounselor = 115,
        OccupationalTherapist = 116,
        RecreationalTherapistAssistant = 117,
        Kinesiotherapist = 118,
        RespiratoryTherapistCertified = 119,
        RespiratoryTherapistRegistered = 120,
        Anaplastologist = 121,
        Audiologist = 122,
        SpecialistTechnologist235500000X = 123,
        SpeechLanguagePathologist = 124,
        AudiologistHearingAidFitter = 125,
        HearingInstrumentSpecialist = 126,
        Perfusionist = 127,
        RadiologyPractitionerAssistant = 128,
        SpecialistTechnologistPathology = 129,
        TechnicianPathology = 130,
        TechnicianCardiology = 131,
        SpecialistTechnologistCardiovascular = 132,
        SpecialistTechnologistHealthInformation = 133,
        SpecialistTechnologistOther = 134,
        TechnicianHealthInformation = 135,
        RadiologicTechnologist = 136,
        TechnicianOther = 137,
        LocalEducationAgencyLea = 138,
        CaseManagement = 139,
        DayTrainingDevelopmentallyDisabledServices = 140,
        HomeHealth = 141,
        HomeInfusion = 142,
        HospiceCareCommunityBased = 143,
        NursingCare = 144,
        PublicHealthOrWelfare = 145,
        CommunityBehavioralHealth = 146,
        ProgramOfAllInclusiveCareForTheElderlyPaceProviderOrganization = 147,
        VoluntaryOrCharitable = 148,
        SupportsBrokerage = 149,
        EarlyInterventionProviderAgency = 150,
        FosterCareAgency = 151,
        InHomeSupportiveCare = 152,
        ClinicCenter = 153,
        EpilepsyUnit = 154,
        PsychiatricUnit = 155,
        RehabilitationUnit = 156,
        MedicareDefinedSwingBedUnit = 157,
        RehabilitationSubstanceUseDisorderUnit = 158,
        ChronicDiseaseHospital = 159,
        LongTermCareHospital = 160,
        ReligiousNonmedicalHealthCareInstitution = 161,
        GeneralAcuteCareHospital = 162,
        PsychiatricHospital = 163,
        RehabilitationHospital = 164,
        SpecialHospital = 165,
        MilitaryHospital = 166,
        ChristianScienceSanitorium = 167,
        MilitaryClinicalMedicalLaboratory = 168,
        ClinicalMedicalLaboratory = 169,
        DentalLaboratory = 170,
        PhysiologicalLaboratory = 171,
        ExclusiveProviderOrganization = 172,
        HealthMaintenanceOrganization = 173,
        PreferredProviderOrganization = 174,
        PointOfService = 175,
        AssistedLivingFacility = 176,
        IntermediateCareFacilityMentalIllness = 177,
        AlzheimerCenterDementiaCenter = 178,
        CustodialCareFacility = 179,
        NursingFacilityIntermediateCareFacility = 180,
        SkilledNursingFacility = 181,
        HospiceInpatient = 182,
        IntermediateCareFacilityMentallyRetarded = 183,
        ChristianScienceFacility = 184,
        ResidentialTreatmentFacilityMentalRetardationAndOrDevelopmentalDisabilities = 185,
        ResidentialTreatmentFacilityPhysicalDisabilities = 186,
        CommunityBasedResidentialTreatmentFacilityMentalIllness = 187,
        CommunityBasedResidentialTreatmentFacilityMentalRetardationAndOrDevelopmentalDisabilities = 188,
        ResidentialTreatmentFacilityEmotionallyDisturbedChildren = 189,
        PsychiatricResidentialTreatmentFacility = 190,
        SubstanceAbuseRehabilitationFacility = 191,
        BloodBank = 192,
        MilitaryUSCoastGuardPharmacy = 193,
        DepartmentOfVeteransAffairsVaPharmacy = 194,
        IndianHealthServiceTribalUrbanIndianHealthITUPharmacy = 195,
        NonPharmacyDispensingSite = 196,
        DurableMedicalEquipmentMedicalSupplies = 197,
        EyeBank = 198,
        EyewearSupplier = 199,
        HearingAidEquipment = 200,
        HomeDeliveredMeals = 201,
        EmergencyResponseSystemCompanies = 202,
        Pharmacy = 203,
        ProstheticOrthoticSupplier = 204,
        MedicalFoodsSupplier = 205,
        OrganProcurementOrganization = 206,
        PortableXRayAndOrOtherPortableDiagnosticImagingSupplier = 207,
        Ambulance = 208,
        MilitaryUSCoastGuardTransport = 209,
        SecuredMedicalTransportVan = 210,
        NonEmergencyMedicalTransportVan = 211,
        Taxi = 212,
        AirCarrier = 213,
        Bus = 214,
        PrivateVehicle = 215,
        Train = 216,
        TransportationBroker = 217,
        PhysicianAssistant = 218,
        NursePractitioner = 219,
        ClinicalNurseSpecialist = 220,
        NurseAnesthetistCertifiedRegistered = 221,
        AdvancedPracticeMidwife = 222,
        AnesthesiologistAssistant = 223,
        ChoreProvider = 224,
        AdultCompanion = 225,
        DayTrainingHabilitationSpecialist = 226,
        Technician = 227,
        Doula = 228,
        ReligiousNonmedicalPractitioner = 229,
        ReligiousNonmedicalNursingPersonnel = 230,
        HomeHealthAide = 231,
        NursingHomeAdministrator = 232,
        Homemaker = 233,
        NursesAide = 234,
        RespiteCare = 235,
        StudentInAnOrganizedHealthCareEducationTrainingProgram = 236,
        PreventionProfessional = 237,
    }
}
/// The US Core Smoking Status Observation Codes Value Set is a 'starter set' of
/// concepts to capture smoking status. See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-smoking-status-observation-codes>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreSmokingStatusObservationCodesValueSet {
}
/// Nested message and enum types in `USCoreSmokingStatusObservationCodesValueSet`.
pub mod us_core_smoking_status_observation_codes_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        TobaccoSmokingStatusNhis = 1,
    }
}
/// This value set indicates the current smoking status of a patient.
/// See <http://hl7.org/fhir/us/core/ValueSet/us-core-observation-smokingstatus>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreSmokingStatusValueSet {
}
/// Nested message and enum types in `USCoreSmokingStatusValueSet`.
pub mod us_core_smoking_status_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        CurrentEveryDaySmoker = 1,
        CurrentSomeDaySmoker = 2,
        FormerSmoker = 3,
        NeverSmoker = 4,
        SmokerCurrentStatusUnknown = 5,
        UnknownIfEverSmoked = 6,
        CurrentHeavyTobaccoSmoker = 7,
        CurrentLightTobaccoSmoker = 8,
    }
}
/// This identifies the vaccine substance administered - CVX codes. **Inclusion
/// Criteria:**  Any CVX code with CVX  'status' (VSAC Property) = `Active`,`
/// Inactive`, `Non-US` except  those noted in exclusions.  **Exclusion
/// Criteria:**  CVX codes that have a CVX  'status' of either `Pending` or
/// `Never Active` AND CVX  codes with CVX 'Nonvaccine' property = True.
/// Available at
/// <http://www2a.cdc.gov/vaccines/iis/iisstandards/vaccines.asp?rpt=cvx> See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-vaccines-cvx>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreVaccineAdministeredValueSetCvxValueSet {
}
/// Nested message and enum types in `USCoreVaccineAdministeredValueSetCvxValueSet`.
pub mod us_core_vaccine_administered_value_set_cvx_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        DiphtheriaTetanusToxoidsAndPertussisVaccine = 1,
        TrivalentPoliovirusVaccineLiveOral = 2,
        MeaslesMumpsAndRubellaVirusVaccine = 3,
        MeaslesAndRubellaVirusVaccine = 4,
        MeaslesVirusVaccine = 5,
        RubellaVirusVaccine = 6,
        MumpsVirusVaccine = 7,
        HepatitisBVaccinePediatricOrPediatricAdolescentDosage = 8,
        TetanusAndDiphtheriaToxoidsAdsorbedPreservativeFreeForAdultUse2LfOfTetanusToxoidAnd2LfOfDiphtheriaToxoid = 9,
        PoliovirusVaccineInactivated = 10,
        PneumococcalConjugateVaccine7Valent = 11,
        TyphoidViCapsularPolysaccharideVaccine = 12,
        DtpHaemophilusInfluenzaeTypeBConjugateAndHepatitisBVaccine = 13,
        MeningococcalCConjugateVaccine = 14,
        HepatitisAAndHepatitisBVaccine = 15,
        VacciniaSmallpoxVaccineDiluted = 16,
        DiphtheriaTetanusToxoidsAndAcellularPertussisVaccine5PertussisAntigens = 17,
        DiphtheriaTetanusToxoidsAndAcellularPertussisVaccineUnspecifiedFormulation = 18,
        MeningococcalAcwyVaccineUnspecifiedFormulation = 19,
        PneumococcalVaccineUnspecifiedFormulation = 20,
        PertussisVaccine = 21,
        DTaPHepatitisBAndPoliovirusVaccine = 22,
        InfluenzaVirusVaccineLiveAttenuatedForIntranasalUse = 23,
        TetanusToxoidUnspecifiedFormulation = 24,
        TetanusAndDiphtheriaToxoidsAdsorbedPreservativeFreeForAdultUse5LfOfTetanusToxoidAnd2LfOfDiphtheriaToxoid = 25,
        MeningococcalPolysaccharideGroupsACYAndW135DiphtheriaToxoidConjugateVaccineMcv4P = 26,
        TetanusToxoidReducedDiphtheriaToxoidAndAcellularPertussisVaccineAdsorbed = 27,
        RotavirusLivePentavalentVaccine = 28,
        VaricellaZosterImmuneGlobulinInvestigationalNewDrug = 29,
        HumanPapillomaVirusVaccineBivalent = 30,
        RotavirusLiveMonovalentVaccine = 31,
        DiphtheriaAntitoxin = 32,
        DiphtheriaTetanusToxoidsAndAcellularPertussisVaccineHaemophilusInfluenzaeTypeBConjugateAndPoliovirusVaccineInactivatedDTaPHibIpv = 33,
        ZosterVaccineLive = 34,
        RotavirusVaccineUnspecifiedFormulation = 35,
        InfluenzaVirusVaccineH5N1AVietnam12032004NationalStockpile = 36,
        NovelInfluenzaH1N109LiveVirusForNasalAdministration = 37,
        NovelInfluenzaH1N109PreservativeFreeInjectable = 38,
        NovelInfluenzaH1N109Injectable = 39,
        NovelInfluenzaH1N109AllFormulations = 40,
        JapaneseEncephalitisVaccineUnspecifiedFormulation = 41,
        TetanusImmuneGlobulin13 = 42,
        DiphtheriaTetanusToxoidsAndAcellularPertussisVaccineAndPoliovirusVaccineInactivated = 43,
        HistoricalRecordOfATyphusVaccination = 44,
        HistoricalDiphtheriaAndTetanusToxoidsAndAcellularPertussisPoliovirusHaemophilusBConjugateAndHepatitisBRecombinantVaccine = 45,
        PneumococcalConjugateVaccine13Valent = 46,
        JapaneseEncephalitisVaccineForIntramuscularAdministration = 47,
        InfluenzaHighDoseSeasonalPreservativeFree = 48,
        MeningococcalOligosaccharideGroupsACYAndW135DiphtheriaToxoidConjugateVaccineMcv4O = 49,
        HpvUnspecifiedFormulation = 50,
        TetanusAndDiphtheriaToxoidsNotAdsorbedForAdultUse = 51,
        TdAdultUnspecifiedFormulation = 52,
        ImmuneGlobulinUnspecifiedFormulation = 53,
        InfluenzaSeasonalInjectablePreservativeFree = 54,
        InfluenzaSeasonalInjectable = 55,
        TetanusToxoidNotAdsorbed = 56,
        AdenovirusType4AndType7LiveOral = 57,
        SeasonalInfluenzaIntradermalPreservativeFree = 58,
        MeningococcalMcv4UnspecifiedConjugateFormulationGroupsACYAndW135 = 59,
        MeningococcalGroupsCAndYAndHaemophilusBTetanusToxoidConjugateVaccine = 60,
        InfluenzaLiveIntranasalQuadrivalent = 61,
        InfluenzaVirusVaccineSplitVirusInclPurifiedSurfaceAntigenRetiredCode = 62,
        InfluenzaInjectableQuadrivalentPreservativeFree = 63,
        InfluenzaNasalUnspecifiedFormulation = 64,
        PneumococcalConjugateUnspecifiedFormulation = 65,
        InfluenzaInjectableMadinDarbyCanineKidneyPreservativeFree = 66,
        SeasonalTrivalentRecombinantInjectableInfluenzaVaccinePreservativeFree = 67,
        RhoDImmuneGlobulinIvOrIm = 68,
        RhoDImmuneGlobulinIm = 69,
        InfluenzaInjectableQuadrivalentContainsPreservative = 70,
        RhoDUnspecifiedFormulation = 71,
        InfluenzaVirusVaccineWholeVirus = 72,
        InfluenzaAMonovalentH5N1AdjuvantedNationalStockpile2013 = 73,
        InfluenzaInjectablequadrivalentPreservativeFreePediatric = 74,
        MeningococcalBVaccineFullyRecombinant = 75,
        MeningococcalBVaccineRecombinantOmvAdjuvanted = 76,
        MeningococcalBUnspecifiedFormulation = 77,
        HumanPapillomavirus9ValentVaccine = 78,
        InfluenzaIntradermalQuadrivalentPreservativeFreeInjectable = 79,
        MeningococcalVaccineOfUnknownFormulationAndUnknownSerogroups = 80,
        SeasonalTrivalentInfluenzaVaccineAdjuvantedPreservativeFree = 81,
        HepALiveAttenuatedIm = 82,
        HaemophilusInfluenzaeTypeBVaccineConjugateUnspecifiedFormulation = 83,
        NonUsDiphtheriaTetanusToxoidsAndAcellularPertussisVaccineHaemophilusInfluenzaeTypeBConjugateAndPoliovirusVaccineInactivatedDTaPHibIpv = 84,
        InfluenzaInjectableMadinDarbyCanineKidneyPreservativeFreeQuadrivalent = 85,
        CholeraWcRBs = 86,
        CholeraBivWc = 87,
        CholeraLiveAttenuated = 88,
        HumanRabiesVaccineFromHumanDiploidCellCulture = 89,
        HumanRabiesVaccineFromChickenFibroblastCulture = 90,
        PneumococcalConjugateVaccine10Valent = 91,
        NonUsBivalentOralPolioVaccineTypes1And3 = 92,
        NonUsMonovalentOralPolioVaccineUnspecifiedFormulation = 93,
        RabiesVaccineForIntramuscularInjectionRetiredCode = 94,
        TetanusImmuneGlobulin180 = 95,
        AnthraxImmuneGlobulin = 96,
        OralPolioVaccineUnspecifiedFormulation = 97,
        YellowFeverVaccineAlternativeFormulation = 98,
        YellowFeverVaccineUnspecifiedFormulation = 99,
        SeasonalQuadrivalentRecombinantInjectableInfluenzaVaccinePreservativeFree = 100,
        InfluenzaInjectableMadinDarbyCanineKidneyQuadrivalentWithPreservative = 101,
        ZosterVaccineRecombinant = 102,
        ZosterVaccineUnspecifiedFormulation = 103,
        HepatitisBVaccineRecombinantCpGAdjuvanted = 104,
        BacillusCalmetteGuerinVaccine = 105,
        DiphtheriaTetanusToxoidsAndAcellularPertussisVaccine = 106,
        VaricellaVirusVaccine = 107,
        DtpHaemophilusInfluenzaeTypeBConjugateVaccine = 108,
        PlagueVaccine = 109,
        AnthraxVaccine = 110,
        TyphoidVaccineLiveOral = 111,
        CholeraVaccineUnspecifiedFormulation = 112,
        BotulinumAntitoxin = 113,
        DiphtheriaAndTetanusToxoidsAdsorbedForPediatricUse = 114,
        CytomegalovirusImmuneGlobulinIntravenous = 115,
        HepatitisBImmuneGlobulin = 116,
        HepatitisAVaccinePediatricDosageUnspecifiedFormulation = 117,
        MeningococcalPolysaccharideVaccineMpsv4 = 118,
        PneumococcalPolysaccharideVaccine23Valent = 119,
        RabiesImmuneGlobulin = 120,
        TetanusToxoidAdsorbed = 121,
        VaricellaZosterImmuneGlobulin = 122,
        YellowFeverVaccine = 123,
        RubellaAndMumpsVirusVaccine = 124,
        JapaneseEncephalitisVaccineSc = 125,
        RabiesVaccineForIntradermalInjection = 126,
        TyphoidVaccineParenteralOtherThanAcetoneKilledDried = 127,
        HepatitisBVaccineAdolescentHighRiskInfantDosage = 128,
        HepatitisBVaccineAdultDosage = 129,
        HepatitisBVaccineDialysisPatientDosage = 130,
        HepatitisBVaccineUnspecifiedFormulation = 131,
        HaemophilusInfluenzaeTypeBVaccinePrpDConjugate = 132,
        HaemophilusInfluenzaeTypeBVaccineHbOcConjugate = 133,
        HaemophilusInfluenzaeTypeBVaccinePrpTConjugate = 134,
        HaemophilusInfluenzaeTypeBVaccinePrpOmpConjugate = 135,
        DTaPHaemophilusInfluenzaeTypeBConjugateVaccine = 136,
        HaemophilusInfluenzaeTypeBConjugateAndHepatitisBVaccine = 137,
        HepatitisAVaccineAdultDosage = 138,
        TyphoidVaccineParenteralAcetoneKilledDriedUSMilitary = 139,
        AdenovirusVaccineType4LiveOral = 140,
        AdenovirusVaccineType7LiveOral = 141,
        HumanPapillomaVirusVaccineQuadrivalent = 142,
        LymeDiseaseVaccine = 143,
        Parainfluenza3VirusVaccine = 144,
        RespiratorySyncytialVirusImmuneGlobulinIntravenous = 145,
        RotavirusLiveTetravalentVaccine = 146,
        VacciniaSmallpoxVaccine = 147,
        StaphylococcusBacteriophageLysate = 148,
        TickBorneEncephalitisVaccine = 149,
        TularemiaVaccine = 150,
        VacciniaImmuneGlobulin = 151,
        VenezuelanEquineEncephalitisLiveAttenuated = 152,
        As03Adjuvant = 153,
        VenezuelanEquineEncephalitisInactivated = 154,
        AdenovirusVaccineUnspecifiedFormulation = 155,
        HepatitisAVaccinePediatricAdolescentDosage2DoseSchedule = 156,
        HepatitisAVaccinePediatricAdolescentDosage3DoseSchedule = 157,
        HepatitisAVaccineUnspecifiedFormulation = 158,
        ImmuneGlobulinIntramuscular = 159,
        ImmuneGlobulinIntravenous = 160,
        InfluenzaVirusVaccineUnspecifiedFormulation = 161,
        PoliovirusVaccineUnspecifiedFormulation = 162,
        RabiesVaccineUnspecifiedFormulation = 163,
        TyphoidVaccineUnspecifiedFormulation = 164,
        VenezuelanEquineEncephalitisVaccineUnspecifiedFormulation = 165,
        RespiratorySyncytialVirusMonoclonalAntibodyPalivizumabIntramuscular = 166,
        MeaslesMumpsRubellaAndVaricellaVirusVaccine = 167,
        TuberculinSkinTestOldTuberculinMultipunctureDevice = 168,
        TuberculinSkinTestPurifiedProteinDerivativeSolutionIntradermal = 169,
        TuberculinSkinTestPurifiedProteinDerivativeMultipunctureDevice = 170,
        TuberculinSkinTestUnspecifiedFormulation = 171,
        NoVaccineAdministered = 172,
    }
}
/// This value set includes all the Vaccine National Drug Codes (NDC).  This
/// source of this data is provided by the
/// \[CDC\](<https://www2a.cdc.gov/vaccines/iis/iisstandards/ndc_crosswalk.asp>) See
/// <http://hl7.org/fhir/us/core/ValueSet/us-core-ndc-vaccine-codes>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreVaccineNationalDrugCodeValueSet {
}
/// Nested message and enum types in `USCoreVaccineNationalDrugCodeValueSet`.
pub mod us_core_vaccine_national_drug_code_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        FluzoneIntradermal49281070355 = 1,
        TyphimVi49281079020 = 2,
        AfluriaQuadrivalent33332031601 = 3,
        FluzoneIntradermalQuadrivalent49281071240 = 4,
        Fluvirin66521011202 = 5,
        Hiberix58160081605 = 6,
        Pneumovax2300006483702 = 7,
        InfluenzaAH5N1MonovalentVaccineAdjuvanted = 8,
        Boostrix58160084251 = 9,
        FluzoneQuadrivalent49281041850 = 10,
        Pneumovax2300006494300 = 11,
        FluzoneIntradermalQuadrivalent49281070840 = 12,
        EngerixB54868073400 = 13,
        Shingrix58160081912 = 14,
        FluzoneQuadrivalent49281051725 = 15,
        TetanusAndDiphtheriaToxoidsAdsorbed00006413341 = 16,
        RabAvert50090309600 = 17,
        Afluria33332011810 = 18,
        FlulavalQuadrivalent19515090952 = 19,
        InfluenzaAH1n12009MonovalentVaccine49281065010 = 20,
        EngerixB58160082011 = 21,
        Ixiaro42515000101 = 22,
        FluzoneQuadrivalent49281062515 = 23,
        FluzoneQuadrivalent49281051625 = 24,
        Fluvirin66521011402 = 25,
        FlulavalQuadrivalent19515089611 = 26,
        Afluria33332011710 = 27,
        AfluriaQuadrivalent33332041610 = 28,
        Fluvirin66521011802 = 29,
        EngerixB58160082111 = 30,
        Prevnar1300005197105 = 31,
        Flumist66019010910 = 32,
        DiphtheriaAndTetanusToxoidsAdsorbed49281027810 = 33,
        Fluzone49281001110 = 34,
        RecombivaxHb54868221900 = 35,
        FluzoneQuadrivalent49281041510 = 36,
        Afluria33332001601 = 37,
        Fluzone49281070555 = 38,
        FluzoneQuadrivalent49281062115 = 39,
        Boostrix58160084234 = 40,
        Fluzone49281001010 = 41,
        Fluvirin66521011302 = 42,
        FluzoneQuadrivalent49281051425 = 43,
        Boostrix58160084252 = 44,
        FlulavalQuadrivalent19515090152 = 45,
        Flucelvax62577061301 = 46,
        FluMistQuadrivalent66019030310 = 47,
        Fluzone49281038815 = 48,
        Vaqta00006484141 = 49,
        FluarixQuadrivalent58160090052 = 50,
        FlucelvaxQuadrivalent = 51,
        YfVax49281091505 = 52,
        InfluenzaAH1n12009MonovalentVaccine49281065050 = 53,
        Afluria33332011610 = 54,
        Pneumovax2354868333901 = 55,
        FluzoneQuadrivalent49281041810 = 56,
        Kinrix58160081252 = 57,
        Daptacel49281028605 = 58,
        Flucelvax63851061201 = 59,
        FlulavalQuadrivalent19515090852 = 60,
        MMRIi54868098000 = 61,
        Cervarix58160083052 = 62,
        Fluzone49281011325 = 63,
        InfluenzaAH1n12009MonovalentVaccine49281065070 = 64,
        Fluvirin66521011510 = 65,
        FluzoneQuadrivalent49281041750 = 66,
        Afluria33332011310 = 67,
        FluzoneQuadrivalent49281062915 = 68,
        Shingrix58160082311 = 69,
        PedvaxHib = 70,
        EngerixB58160082134 = 71,
        Fluvirin66521011502 = 72,
        Afluria33332001401 = 73,
        Quadracel = 74,
        Flublok42874001410 = 75,
        Flublok42874001310 = 76,
        Afluria33332011510 = 77,
        Zostavax00006496300 = 78,
        Fluzone49281001025 = 79,
        InfluenzaA33332051901 = 80,
        FluarixQuadrivalent58160089852 = 81,
        Fluzone49281011225 = 82,
        FlulavalQuadrivalent19515089811 = 83,
        Gardasil00006410909 = 84,
        FluzoneQuadrivalent49281041450 = 85,
        Vaqta00006409509 = 86,
        Gardasil00006404500 = 87,
        FlulavalQuadrivalent19515091252 = 88,
        Menhibrix58160080111 = 89,
        MenomuneACYW135Combined49281048901 = 90,
        Flublok42874001710 = 91,
        Fluvirin66521011602 = 92,
        Menveo46028020801 = 93,
        FluzoneQuadrivalent49281062715 = 94,
        Trumenba00005010002 = 95,
        Vivotif69401000001 = 96,
        TetanusAndDiphtheriaToxoidsAdsorbed21695041301 = 97,
        FluzoneQuadrivalent49281041610 = 98,
        InfluenzaAH1n12009MonovalentVaccine49281065025 = 99,
        TetanusToxoidAdsorbed49281080083 = 100,
        Decavac49281029183 = 101,
        Vaqta00006409502 = 102,
        Rotarix = 103,
        Flulaval19515088907 = 104,
        Fluzone49281039215 = 105,
        FlulavalQuadrivalent19515089111 = 106,
        Adacel49281040005 = 107,
        Stamaril = 108,
        InfluenzaAH1n12009MonovalentVaccine49281064015 = 109,
        FluzoneQuadrivalent49281051325 = 110,
        ProQuad00006417100 = 111,
        Vaqta00006409609 = 112,
        Cervarix58160083034 = 113,
        RecombivaxHb00006498000 = 114,
        TetanusAndDiphtheriaToxoidsAdsorbed17478013101 = 115,
        FluzoneQuadrivalent49281041410 = 116,
        HeplisavB43528000205 = 117,
        InfluenzaAH1N12009MonovalentVaccine66521020002 = 118,
        Fluzone49281001150 = 119,
        Fluvirin70461012010 = 120,
        FluMistQuadrivalent66019030410 = 121,
        Bexsero58160097620 = 122,
        Varivax00006482600 = 123,
        Fluvirin66521011610 = 124,
        Zostavax00006496341 = 125,
        Pentacel = 126,
        Flublok42874001210 = 127,
        Menveo58160095509 = 128,
        Trumenba00005010005 = 129,
        Fluzone49281070755 = 130,
        TetanusAndDiphtheriaToxoidsAdsorbed14362011104 = 131,
        Fluvirin66521011210 = 132,
        Fluvirin66521011710 = 133,
        Gardasil00006404541 = 134,
        FluzoneHighDose49281038965 = 135,
        Vivotif69401000002 = 136,
        YfVax49281091501 = 137,
        RecombivaxHb00006409302 = 138,
        Twinrix58160081548 = 139,
        Vaxchora = 140,
        Havrix58160082611 = 141,
        RecombivaxHb00006499200 = 142,
        Fluzone49281011125 = 143,
        RecombivaxHb00006409309 = 144,
        HeplisavB50090346900 = 145,
        FluzoneHighDose49281040365 = 146,
        Fluvirin70461011910 = 147,
        RecombivaxHb00006499500 = 148,
        Twinrix58160081534 = 149,
        FluzoneHighDose49281039365 = 150,
        Prevnar = 151,
        Afluria33332001701 = 152,
        RabAvert63851050101 = 153,
        Fluarix58160088152 = 154,
        BioThrax = 155,
        Fluzone49281039415 = 156,
        Varivax00006482700 = 157,
        Hiberix58160080605 = 158,
        FluzoneQuadrivalent49281051825 = 159,
        Ixiaro62195005110 = 160,
        Vaxelis = 161,
        FluzoneIntradermal49281070955 = 162,
        FluMistQuadrivalent66019030010 = 163,
        Tenivac49281021515 = 164,
        Havrix58160082552 = 165,
        Trumenba00005010010 = 166,
        Fluvirin66521011702 = 167,
        InfluenzaAH1n12009MonovalentVaccine49281065090 = 168,
        Flublok42874001510 = 169,
        Afluria33332001801 = 170,
        ProQuad00006499900 = 171,
        Prevnar1300005197104 = 172,
        Flulaval19515085052 = 173,
        Prevnar1300005197102 = 174,
        RecombivaxHb00006409402 = 175,
        Vaqta00006409602 = 176,
        Havrix58160082511 = 177,
        Pediarix58160081152 = 178,
        Ixiaro42515000201 = 179,
        Fluzone49281001350 = 180,
        MedicalProviderSingleUseEzFluShot2013201476420048301 = 181,
        Fluvirin66521011810 = 182,
        FluzoneHighDose49281039965 = 183,
        Fluzone49281039615 = 184,
        Flumist66019010701 = 185,
        Flulaval19515089007 = 186,
        MedicalProviderSingleUseEzFluShot2013201476420048201 = 187,
        Afluria33332001501 = 188,
        FluMistQuadrivalent66019030210 = 189,
        Fluzone49281001210 = 190,
        FluzoneIntradermalQuadrivalent49281071040 = 191,
        RabAvert63851050102 = 192,
        Fluarix58160087952 = 193,
        FluzoneHighDose49281039765 = 194,
        Vaqta00006483141 = 195,
        Twinrix58160081546 = 196,
        Afluria33332011010 = 197,
        Pneumovax2354868432000 = 198,
        Flublok42874001610 = 199,
        Fluzone49281001250 = 200,
        Hiberix58160081811 = 201,
        Fluzone49281038615 = 202,
        Bexsero46028011401 = 203,
        Comvax = 204,
        Havrix58160082652 = 205,
        ActHib49281054505 = 206,
        Flumist66019010810 = 207,
        FlucelvaxQuadrivalentMultiDoseVial70461041810 = 208,
        RecombivaxHb00006409409 = 209,
        Tripedia = 210,
        InfluenzaA33332062910 = 211,
        Fluarix58160088052 = 212,
        RotaTeq00006404720 = 213,
        Gardasil900006411902 = 214,
        Boostrix58160084211 = 215,
        FlulavalQuadrivalent19515090311 = 216,
        RecombivaxHb00006498100 = 217,
        FluarixQuadrivalent58160090552 = 218,
        FluzoneHighDose49281040165 = 219,
        Afluria33332011410 = 220,
        Ipol49281086010 = 221,
        FlucelvaxQuadrivalentPrefilledSyringe70461031803 = 222,
        RecombivaxHb54868221901 = 223,
        FlublokQuadrivalent49281071810 = 224,
        Adacel49281040015 = 225,
        Fluvirin70461012002 = 226,
        FluzoneQuadrivalent49281041650 = 227,
        FluzoneQuadrivalent49281041350 = 228,
        Fluarix58160088352 = 229,
        TyphimVi49281079051 = 230,
        Daptacel49281028610 = 231,
        FluMist = 232,
        Bexsero46028011402 = 233,
        EngerixB58160082152 = 234,
        Fluzone49281001310 = 235,
        FlulavalQuadrivalent19515089452 = 236,
        FluMistQuadrivalent66019030510 = 237,
        Adacel49281040010 = 238,
        Fluzone49281039015 = 239,
        BcgVaccine = 240,
        AdenovirusType4AndType7VaccineLive = 241,
        AfluriaQuadrivalent33332041710 = 242,
        FluzoneHighDose49281039565 = 243,
        FluMistQuadrivalent66019030110 = 244,
        Tenivac49281021510 = 245,
        FlulavalQuadrivalent19515089511 = 246,
        FlucelvaxQuadrivalentPrefilledSyringe70461020101 = 247,
        FluarixQuadrivalent58160090752 = 248,
        Havrix55045384101 = 249,
        Infanrix50090288300 = 250,
        TetanusToxoidAdsorbed49281082010 = 251,
        FluzoneQuadrivalent49281041710 = 252,
        Afluria33332001001 = 253,
        Afluria33332001301 = 254,
        InfluenzaAH1N12009MonovalentVaccine66521020010 = 255,
        Bexsero58160097606 = 256,
        Menhibrix58160080905 = 257,
        Pneumovax2300006473900 = 258,
        Fluad70461001803 = 259,
        FluzoneQuadrivalent49281041310 = 260,
        TetanusAndDiphtheriaToxoidsAdsorbed13533013101 = 261,
        Kinrix58160081211 = 262,
        FluzoneHighDose49281039165 = 263,
        Flulaval19515084511 = 264,
        Pediarix58160081151 = 265,
        Twinrix58160081552 = 266,
        Fluvirin70461011902 = 267,
        Infanrix58160081052 = 268,
        Flucelvax62577061401 = 269,
        FlublokQuadrivalent42874011710 = 270,
        MenomuneACYW135Combined49281048991 = 271,
        RabAvert58160096412 = 272,
        Fluzone49281001450 = 273,
        Gardasil00006410902 = 274,
        Fluad70461000201 = 275,
        Daptacel49281028601 = 276,
        Infanrix58160081011 = 277,
        FlulavalQuadrivalent19515090011 = 278,
        Pneumovax2300006483703 = 279,
        Fluvirin66521011310 = 280,
        Havrix58160082634 = 281,
        FluarixQuadrivalent58160090352 = 282,
        Vaqta00006484100 = 283,
        Fluzone54868618000 = 284,
        MMRIi00006468100 = 285,
        AfluriaQuadrivalent33332031701 = 286,
        Fluad70461000101 = 287,
        Menactra = 288,
        Fluzone49281038765 = 289,
        Ipol49281086055 = 290,
        Flulaval19515089307 = 291,
        InfluenzaA33332051925 = 292,
        FlucelvaxQuadrivalentMultiDoseVial70461030110 = 293,
        InfluenzaAH1N1Intranasal = 294,
        HeplisavB43528000305 = 295,
        EngerixB58160082052 = 296,
        Fluad66521000001 = 297,
        ImovaxRabies = 298,
        Decavac49281029110 = 299,
        AfluriaQuadrivalent33332041810 = 300,
        Gardasil900006412102 = 301,
        Flucelvax63851061301 = 302,
        Fluvirin66521011410 = 303,
        RotaTeq00006404741 = 304,
        FluarixQuadrivalent58160090152 = 305,
        AfluriaQuadrivalent33332031801 = 306,
        Gardasil900006411903 = 307,
        DiphtheriaAndTetanusToxoidsAdsorbed49281022510 = 308,
        Twinrix58160081511 = 309,
        Fluzone54868617700 = 310,
        Fluzone49281001050 = 311,
        Adacel49281040020 = 312,
        ActHib49281054503 = 313,
        Ipol50090169309 = 314,
        RecombivaxHb00006499541 = 315,
    }
}
/// This value set defines two letter USPS alphabetic codes.
/// See <http://hl7.org/fhir/us/core/ValueSet/us-core-usps-state>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UspsTwoLetterAlphabeticCodesValueSet {
}
/// Nested message and enum types in `UspsTwoLetterAlphabeticCodesValueSet`.
pub mod usps_two_letter_alphabetic_codes_value_set {
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    pub enum Value {
        InvalidUninitialized = 0,
        Ak = 1,
        Al = 2,
        Ar = 3,
        As = 4,
        Az = 5,
        Ca = 6,
        Co = 7,
        Ct = 8,
        Dc = 9,
        De = 10,
        Fl = 11,
        Fm = 12,
        Ga = 13,
        Gu = 14,
        Hi = 15,
        Ia = 16,
        Id = 17,
        Il = 18,
        In = 19,
        Ks = 20,
        Ky = 21,
        La = 22,
        Ma = 23,
        Md = 24,
        Me = 25,
        Mh = 26,
        Mi = 27,
        Mn = 28,
        Mo = 29,
        Mp = 30,
        Ms = 31,
        Mt = 32,
        Nc = 33,
        Nd = 34,
        Ne = 35,
        Nh = 36,
        Nj = 37,
        Nm = 38,
        Nv = 39,
        Ny = 40,
        Oh = 41,
        Ok = 42,
        Or = 43,
        Pa = 44,
        Pr = 45,
        Pw = 46,
        Ri = 47,
        Sc = 48,
        Sd = 49,
        Tn = 50,
        Tx = 51,
        Um = 52,
        Ut = 53,
        Va = 54,
        Vi = 55,
        Vt = 56,
        Wa = 57,
        Wi = 58,
        Wv = 59,
        Wy = 60,
    }
}
/// Auto-generated from StructureDefinition for USCoreProcedureProfile.
/// An action that is being or was performed on a patient.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-procedure>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreProcedureProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Identifiers for this procedure
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag="11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag="12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// A request for this procedure
    #[prost(message, repeated, tag="13")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="14")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="15")]
    pub status: ::core::option::Option<us_core_procedure_profile::StatusCode>,
    /// Reason for current status
    #[prost(message, optional, tag="16")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// Classification of the procedure
    #[prost(message, optional, tag="17")]
    pub category: ::core::option::Option<super::core::CodeableConcept>,
    /// Procedure codes from SNOMED CT, CPT, or HCPCS II
    #[prost(message, optional, tag="18")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who the procedure was performed on
    #[prost(message, optional, tag="19")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag="20")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="21")]
    pub performed: ::core::option::Option<us_core_procedure_profile::PerformedX>,
    /// Who recorded the procedure
    #[prost(message, optional, tag="22")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Person who asserts this procedure
    #[prost(message, optional, tag="23")]
    pub asserter: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="24")]
    pub performer: prost::alloc::vec::Vec<us_core_procedure_profile::Performer>,
    /// Where the procedure happened
    #[prost(message, optional, tag="25")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Coded reason procedure performed
    #[prost(message, repeated, tag="26")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The justification that the procedure was performed
    #[prost(message, repeated, tag="27")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Target body sites
    #[prost(message, repeated, tag="28")]
    pub body_site: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The result of procedure
    #[prost(message, optional, tag="29")]
    pub outcome: ::core::option::Option<super::core::CodeableConcept>,
    /// Any report resulting from the procedure
    #[prost(message, repeated, tag="30")]
    pub report: prost::alloc::vec::Vec<super::core::Reference>,
    /// Complication following the procedure
    #[prost(message, repeated, tag="31")]
    pub complication: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// A condition that is a result of the procedure
    #[prost(message, repeated, tag="32")]
    pub complication_detail: prost::alloc::vec::Vec<super::core::Reference>,
    /// Instructions for follow up
    #[prost(message, repeated, tag="33")]
    pub follow_up: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Additional information about the procedure
    #[prost(message, repeated, tag="34")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    #[prost(message, repeated, tag="35")]
    pub focal_device: prost::alloc::vec::Vec<us_core_procedure_profile::FocalDevice>,
    /// Items used during procedure
    #[prost(message, repeated, tag="36")]
    pub used_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Coded items used during the procedure
    #[prost(message, repeated, tag="37")]
    pub used_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
}
/// Nested message and enum types in `USCoreProcedureProfile`.
pub mod us_core_procedure_profile {
    /// preparation | in-progress | not-done | on-hold | stopped | completed |
    /// entered-in-error | unknown
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::event_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// When the procedure was performed
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PerformedX {
        #[prost(oneof="performed_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<performed_x::Choice>,
    }
    /// Nested message and enum types in `PerformedX`.
    pub mod performed_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
        }
    }
    /// The people who performed the procedure
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Performer {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of performance
        #[prost(message, optional, tag="4")]
        pub function: ::core::option::Option<super::super::core::CodeableConcept>,
        /// The reference to the practitioner
        #[prost(message, optional, tag="5")]
        pub actor: ::core::option::Option<super::super::core::Reference>,
        /// Organization the device or practitioner was acting for
        #[prost(message, optional, tag="6")]
        pub on_behalf_of: ::core::option::Option<super::super::core::Reference>,
    }
    /// Manipulated, implanted, or removed device
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct FocalDevice {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Kind of change to device
        #[prost(message, optional, tag="4")]
        pub action: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Device that was changed
        #[prost(message, optional, tag="5")]
        pub manipulated: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for USCorePractitionerProfile.
/// A person with a  formal responsibility in the provisioning of healthcare or
/// related services. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitioner>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCorePractitionerProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// An identifier for the person as this agent
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether this practitioner's record is in active use
    #[prost(message, optional, tag="11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// The name(s) associated with the practitioner
    #[prost(message, repeated, tag="12")]
    pub name: prost::alloc::vec::Vec<super::core::HumanName>,
    /// A contact detail for the practitioner (that apply to all roles)
    #[prost(message, repeated, tag="13")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Address(es) of the practitioner that are not role specific (typically home
    /// address)
    #[prost(message, repeated, tag="14")]
    pub address: prost::alloc::vec::Vec<super::core::Address>,
    #[prost(message, optional, tag="15")]
    pub gender: ::core::option::Option<us_core_practitioner_profile::GenderCode>,
    /// The date  on which the practitioner was born
    #[prost(message, optional, tag="16")]
    pub birth_date: ::core::option::Option<super::core::Date>,
    /// Image of the person
    #[prost(message, repeated, tag="17")]
    pub photo: prost::alloc::vec::Vec<super::core::Attachment>,
    #[prost(message, repeated, tag="18")]
    pub qualification: prost::alloc::vec::Vec<us_core_practitioner_profile::Qualification>,
    /// A language the practitioner can use in patient communication
    #[prost(message, repeated, tag="19")]
    pub communication: prost::alloc::vec::Vec<super::core::CodeableConcept>,
}
/// Nested message and enum types in `USCorePractitionerProfile`.
pub mod us_core_practitioner_profile {
    /// male | female | other | unknown
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct GenderCode {
        #[prost(enumeration="super::super::core::administrative_gender_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Certification, licenses, or training pertaining to the provision of care
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Qualification {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// An identifier for this qualification for the practitioner
        #[prost(message, repeated, tag="4")]
        pub identifier: prost::alloc::vec::Vec<super::super::core::Identifier>,
        /// Coded representation of the qualification
        #[prost(message, optional, tag="5")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Period during which the qualification is valid
        #[prost(message, optional, tag="6")]
        pub period: ::core::option::Option<super::super::core::Period>,
        /// Organization that regulates and issues the qualification
        #[prost(message, optional, tag="7")]
        pub issuer: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for
/// USCoreDiagnosticReportProfileLaboratoryReporting. A Diagnostic report - a
/// combination of request information, atomic results, images, interpretation,
/// as well as formatted reports. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-diagnosticreport-lab>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreDiagnosticReportProfileLaboratoryReporting {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier for report
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// What was requested
    #[prost(message, repeated, tag="11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="12")]
    pub status: ::core::option::Option<us_core_diagnostic_report_profile_laboratory_reporting::StatusCode>,
    /// Service category
    #[prost(message, repeated, tag="13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// US Core Laboratory Report Order Code
    #[prost(message, optional, tag="14")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// The subject of the report - usually, but not always, the patient
    #[prost(message, optional, tag="15")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Health care event when test ordered
    #[prost(message, optional, tag="16")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="17")]
    pub effective: ::core::option::Option<us_core_diagnostic_report_profile_laboratory_reporting::EffectiveX>,
    /// DateTime this version was made
    #[prost(message, optional, tag="18")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Responsible Diagnostic Service
    #[prost(message, repeated, tag="19")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    /// Primary result interpreter
    #[prost(message, repeated, tag="20")]
    pub results_interpreter: prost::alloc::vec::Vec<super::core::Reference>,
    /// Specimens this report is based on
    #[prost(message, repeated, tag="21")]
    pub specimen: prost::alloc::vec::Vec<super::core::Reference>,
    /// Observations
    #[prost(message, repeated, tag="22")]
    pub result: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reference to full details of imaging associated with the diagnostic report
    #[prost(message, repeated, tag="23")]
    pub imaging_study: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="24")]
    pub media: prost::alloc::vec::Vec<us_core_diagnostic_report_profile_laboratory_reporting::Media>,
    /// Clinical conclusion (interpretation) of test results
    #[prost(message, optional, tag="25")]
    pub conclusion: ::core::option::Option<super::core::String>,
    /// Codes for the clinical conclusion of test results
    #[prost(message, repeated, tag="26")]
    pub conclusion_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Entire report as issued
    #[prost(message, repeated, tag="27")]
    pub presented_form: prost::alloc::vec::Vec<super::core::Attachment>,
}
/// Nested message and enum types in `USCoreDiagnosticReportProfileLaboratoryReporting`.
pub mod us_core_diagnostic_report_profile_laboratory_reporting {
    /// registered | partial | preliminary | final +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::diagnostic_report_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Specimen Collection Datetime or Period
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof="effective_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Key images associated with this report
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Media {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Comment about the image (e.g. explanation)
        #[prost(message, optional, tag="4")]
        pub comment: ::core::option::Option<super::super::core::String>,
        /// Reference to the image source
        #[prost(message, optional, tag="5")]
        pub link: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for
/// USCorePediatricWeightForHeightObservationProfile. FHIR Vital Signs Profile.
/// See
/// <http://hl7.org/fhir/us/core/StructureDefinition/pediatric-weight-for-height>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCorePediatricWeightForHeightObservationProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag="11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<us_core_pediatric_weight_for_height_observation_profile::StatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag="14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Weight-for-length per age and gender
    #[prost(message, optional, tag="15")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who and/or what the observation is about
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// What the observation is about, when it is not about the subject of record
    #[prost(message, repeated, tag="17")]
    pub focus: prost::alloc::vec::Vec<super::core::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag="18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="19")]
    pub effective: ::core::option::Option<us_core_pediatric_weight_for_height_observation_profile::EffectiveX>,
    /// Date/Time this version was made available
    #[prost(message, optional, tag="20")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag="21")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="22")]
    pub value: ::core::option::Option<us_core_pediatric_weight_for_height_observation_profile::ValueX>,
    /// Why the result is missing
    #[prost(message, optional, tag="23")]
    pub data_absent_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, repeated, tag="24")]
    pub interpretation: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments about the observation
    #[prost(message, repeated, tag="25")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Observed body part
    #[prost(message, optional, tag="26")]
    pub body_site: ::core::option::Option<super::core::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag="27")]
    pub method: ::core::option::Option<super::core::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag="28")]
    pub specimen: ::core::option::Option<super::core::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag="29")]
    pub device: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="30")]
    pub reference_range: prost::alloc::vec::Vec<us_core_pediatric_weight_for_height_observation_profile::ReferenceRange>,
    /// Used when reporting vital signs panel components
    #[prost(message, repeated, tag="31")]
    pub has_member: prost::alloc::vec::Vec<super::core::Reference>,
    /// Related measurements the observation is made from
    #[prost(message, repeated, tag="32")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="33")]
    pub component: prost::alloc::vec::Vec<us_core_pediatric_weight_for_height_observation_profile::Component>,
}
/// Nested message and enum types in `USCorePediatricWeightForHeightObservationProfile`.
pub mod us_core_pediatric_weight_for_height_observation_profile {
    /// registered | preliminary | final | amended +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::observation_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Often just a dateTime for Vital Signs
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof="effective_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Vital Signs value are recorded using the Quantity data type. For supporting
    /// observations such as Cuff size could use other datatypes such as
    /// CodeableConcept.
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof="value_x::Choice", tags="1")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Quantity(super::super::super::core::Quantity),
        }
    }
    /// Provides guide for interpretation
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReferenceRange {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag="4")]
        pub low: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag="5")]
        pub high: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag="7")]
        pub applies_to: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag="8")]
        pub age: ::core::option::Option<super::super::core::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag="9")]
        pub text: ::core::option::Option<super::super::core::String>,
    }
    /// Used when reporting systolic and diastolic blood pressure.
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Component {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub value: ::core::option::Option<component::ValueX>,
        /// Why the component result is missing
        #[prost(message, optional, tag="6")]
        pub data_absent_reason: ::core::option::Option<super::super::core::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, repeated, tag="7")]
        pub interpretation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag="8")]
        pub reference_range: prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Vital Sign Value recorded with UCUM
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof="value_x::Choice", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag="3")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag="4")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="5")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag="6")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag="7")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag="8")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag="9")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag="10")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag="11")]
                Period(super::super::super::super::core::Period),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreEncounterProfile.
/// An interaction during which services are provided to the patient.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-encounter>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreEncounterProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Identifier(s) by which this encounter is known
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<us_core_encounter_profile::StatusCode>,
    #[prost(message, repeated, tag="12")]
    pub status_history: prost::alloc::vec::Vec<us_core_encounter_profile::StatusHistory>,
    /// Classification of patient encounter
    #[prost(message, optional, tag="13")]
    pub class_value: ::core::option::Option<super::core::Coding>,
    #[prost(message, repeated, tag="14")]
    pub class_history: prost::alloc::vec::Vec<us_core_encounter_profile::ClassHistory>,
    /// Specific type of encounter
    #[prost(message, repeated, tag="15")]
    pub r#type: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Specific type of service
    #[prost(message, optional, tag="16")]
    pub service_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Indicates the urgency of the encounter
    #[prost(message, optional, tag="17")]
    pub priority: ::core::option::Option<super::core::CodeableConcept>,
    /// The patient or group present at the encounter
    #[prost(message, optional, tag="18")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Episode(s) of care that this encounter should be recorded against
    #[prost(message, repeated, tag="19")]
    pub episode_of_care: prost::alloc::vec::Vec<super::core::Reference>,
    /// The ServiceRequest that initiated this encounter
    #[prost(message, repeated, tag="20")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="21")]
    pub participant: prost::alloc::vec::Vec<us_core_encounter_profile::Participant>,
    /// The appointment that scheduled this encounter
    #[prost(message, repeated, tag="22")]
    pub appointment: prost::alloc::vec::Vec<super::core::Reference>,
    /// The start and end time of the encounter
    #[prost(message, optional, tag="23")]
    pub period: ::core::option::Option<super::core::Period>,
    /// Quantity of time the encounter lasted (less time absent)
    #[prost(message, optional, tag="24")]
    pub length: ::core::option::Option<super::core::Duration>,
    /// Coded reason the encounter takes place
    #[prost(message, repeated, tag="25")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Reason the encounter takes place (reference)
    #[prost(message, repeated, tag="26")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="27")]
    pub diagnosis: prost::alloc::vec::Vec<us_core_encounter_profile::Diagnosis>,
    /// The set of accounts that may be used for billing for this Encounter
    #[prost(message, repeated, tag="28")]
    pub account: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="29")]
    pub hospitalization: ::core::option::Option<us_core_encounter_profile::Hospitalization>,
    #[prost(message, repeated, tag="30")]
    pub location: prost::alloc::vec::Vec<us_core_encounter_profile::Location>,
    /// The organization (facility) responsible for this encounter
    #[prost(message, optional, tag="31")]
    pub service_provider: ::core::option::Option<super::core::Reference>,
    /// Another Encounter this encounter is part of
    #[prost(message, optional, tag="32")]
    pub part_of: ::core::option::Option<super::core::Reference>,
}
/// Nested message and enum types in `USCoreEncounterProfile`.
pub mod us_core_encounter_profile {
    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled
    /// +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::encounter_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// List of past encounter statuses
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusHistory {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub status: ::core::option::Option<status_history::StatusCode>,
        /// The time that the episode was in the specified status
        #[prost(message, optional, tag="5")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// Nested message and enum types in `StatusHistory`.
    pub mod status_history {
        /// planned | arrived | triaged | in-progress | onleave | finished |
        /// cancelled +
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct StatusCode {
            #[prost(enumeration="super::super::super::core::encounter_status_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// List of past encounter classes
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ClassHistory {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// inpatient | outpatient | ambulatory | emergency +
        #[prost(message, optional, tag="4")]
        pub class_value: ::core::option::Option<super::super::core::Coding>,
        /// The time that the episode was in the specified class
        #[prost(message, optional, tag="5")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// List of participants involved in the encounter
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Participant {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Role of participant in encounter
        #[prost(message, repeated, tag="4")]
        pub r#type: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Period of time during the encounter that the participant participated
        #[prost(message, optional, tag="5")]
        pub period: ::core::option::Option<super::super::core::Period>,
        /// Persons involved in the encounter other than the patient
        #[prost(message, optional, tag="6")]
        pub individual: ::core::option::Option<super::super::core::Reference>,
    }
    /// The list of diagnosis relevant to this encounter
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Diagnosis {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The diagnosis or procedure relevant to the encounter
        #[prost(message, optional, tag="4")]
        pub condition: ::core::option::Option<super::super::core::Reference>,
        /// Role that this diagnosis has within the encounter (e.g. admission,
        /// billing, discharge …)
        #[prost(message, optional, tag="5")]
        pub r#use: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Ranking of the diagnosis (for each role type)
        #[prost(message, optional, tag="6")]
        pub rank: ::core::option::Option<super::super::core::PositiveInt>,
    }
    /// Details about the admission to a healthcare service
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Hospitalization {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Pre-admission identifier
        #[prost(message, optional, tag="4")]
        pub pre_admission_identifier: ::core::option::Option<super::super::core::Identifier>,
        /// The location/organization from which the patient came before admission
        #[prost(message, optional, tag="5")]
        pub origin: ::core::option::Option<super::super::core::Reference>,
        /// From where patient was admitted (physician referral, transfer)
        #[prost(message, optional, tag="6")]
        pub admit_source: ::core::option::Option<super::super::core::CodeableConcept>,
        /// The type of hospital re-admission that has occurred (if any). If the
        /// value is absent, then this is not identified as a readmission
        #[prost(message, optional, tag="7")]
        pub re_admission: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Diet preferences reported by the patient
        #[prost(message, repeated, tag="8")]
        pub diet_preference: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Special courtesies (VIP, board member)
        #[prost(message, repeated, tag="9")]
        pub special_courtesy: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Wheelchair, translator, stretcher, etc.
        #[prost(message, repeated, tag="10")]
        pub special_arrangement: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Location/organization to which the patient is discharged
        #[prost(message, optional, tag="11")]
        pub destination: ::core::option::Option<super::super::core::Reference>,
        /// Category or kind of location after discharge
        #[prost(message, optional, tag="12")]
        pub discharge_disposition: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// List of locations where the patient has been
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Location {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Location the encounter takes place
        #[prost(message, optional, tag="4")]
        pub location: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag="5")]
        pub status: ::core::option::Option<location::StatusCode>,
        /// The physical type of the location (usually the level in the location
        /// hierachy - bed room ward etc.)
        #[prost(message, optional, tag="6")]
        pub physical_type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Time period during which the patient was present at the location
        #[prost(message, optional, tag="7")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// Nested message and enum types in `Location`.
    pub mod location {
        /// planned | active | reserved | completed
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct StatusCode {
            #[prost(enumeration="super::super::super::core::encounter_location_status_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreDocumentReferenceProfile.
/// A reference to a document.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-documentreference>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreDocumentReferenceProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Master Version Specific Identifier
    #[prost(message, optional, tag="10")]
    pub master_identifier: ::core::option::Option<super::core::Identifier>,
    /// Other identifiers for the document
    #[prost(message, repeated, tag="11")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag="12")]
    pub status: ::core::option::Option<us_core_document_reference_profile::StatusCode>,
    #[prost(message, optional, tag="13")]
    pub doc_status: ::core::option::Option<us_core_document_reference_profile::DocStatusCode>,
    /// Kind of document (LOINC if possible)
    #[prost(message, optional, tag="14")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    /// Categorization of document
    #[prost(message, repeated, tag="15")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Who/what is the subject of the document
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// When this document reference was created
    #[prost(message, optional, tag="17")]
    pub date: ::core::option::Option<super::core::Instant>,
    /// Who and/or what authored the document
    #[prost(message, repeated, tag="18")]
    pub author: prost::alloc::vec::Vec<super::core::Reference>,
    /// Who/what authenticated the document
    #[prost(message, optional, tag="19")]
    pub authenticator: ::core::option::Option<super::core::Reference>,
    /// Organization which maintains the document
    #[prost(message, optional, tag="20")]
    pub custodian: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="21")]
    pub relates_to: prost::alloc::vec::Vec<us_core_document_reference_profile::RelatesTo>,
    /// Human-readable description
    #[prost(message, optional, tag="22")]
    pub description: ::core::option::Option<super::core::String>,
    /// Document security-tags
    #[prost(message, repeated, tag="23")]
    pub security_label: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag="24")]
    pub content: ::core::option::Option<us_core_document_reference_profile::Content>,
    #[prost(message, optional, tag="25")]
    pub context: ::core::option::Option<us_core_document_reference_profile::Context>,
}
/// Nested message and enum types in `USCoreDocumentReferenceProfile`.
pub mod us_core_document_reference_profile {
    /// current | superseded | entered-in-error
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::document_reference_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// preliminary | final | amended | entered-in-error
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DocStatusCode {
        #[prost(enumeration="super::super::core::composition_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Relationships to other documents
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct RelatesTo {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<relates_to::CodeType>,
        /// Target of the relationship
        #[prost(message, optional, tag="5")]
        pub target: ::core::option::Option<super::super::core::Reference>,
    }
    /// Nested message and enum types in `RelatesTo`.
    pub mod relates_to {
        /// replaces | transforms | signs | appends
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct CodeType {
            #[prost(enumeration="super::super::super::core::document_relationship_type_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// Document referenced
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Content {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Where to access the document
        #[prost(message, optional, tag="4")]
        pub attachment: ::core::option::Option<super::super::core::Attachment>,
        /// Format/content rules for the document
        #[prost(message, optional, tag="5")]
        pub format: ::core::option::Option<super::super::core::Coding>,
    }
    /// Clinical context of document
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Context {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Context of the document  content
        #[prost(message, optional, tag="4")]
        pub encounter: ::core::option::Option<super::super::core::Reference>,
        /// Main clinical acts documented
        #[prost(message, repeated, tag="5")]
        pub event: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Time of service that is being documented
        #[prost(message, optional, tag="6")]
        pub period: ::core::option::Option<super::super::core::Period>,
        /// Kind of facility where patient was seen
        #[prost(message, optional, tag="7")]
        pub facility_type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Additional details about where the content was created (e.g. clinical
        /// specialty)
        #[prost(message, optional, tag="8")]
        pub practice_setting: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Patient demographics from source
        #[prost(message, optional, tag="9")]
        pub source_patient_info: ::core::option::Option<super::super::core::Reference>,
        /// Related identifiers or resources
        #[prost(message, repeated, tag="10")]
        pub related: prost::alloc::vec::Vec<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for USCoreCareTeam.
/// Planned participants in the coordination and delivery of care for a patient
/// or group. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-careteam>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreCareTeam {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this team
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<us_core_care_team::StatusCode>,
    /// Type of team
    #[prost(message, repeated, tag="12")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Name of the team, such as crisis assessment team
    #[prost(message, optional, tag="13")]
    pub name: ::core::option::Option<super::core::String>,
    /// Who care team is for
    #[prost(message, optional, tag="14")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag="15")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Time period team covers
    #[prost(message, optional, tag="16")]
    pub period: ::core::option::Option<super::core::Period>,
    #[prost(message, repeated, tag="17")]
    pub participant: prost::alloc::vec::Vec<us_core_care_team::Participant>,
    /// Why the care team exists
    #[prost(message, repeated, tag="18")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why the care team exists
    #[prost(message, repeated, tag="19")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Organization responsible for the care team
    #[prost(message, repeated, tag="20")]
    pub managing_organization: prost::alloc::vec::Vec<super::core::Reference>,
    /// A contact detail for the care team (that applies to all members)
    #[prost(message, repeated, tag="21")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Comments made about the CareTeam
    #[prost(message, repeated, tag="22")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `USCoreCareTeam`.
pub mod us_core_care_team {
    /// proposed | active | suspended | inactive | entered-in-error
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::care_team_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Members of the team
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Participant {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of involvement
        #[prost(message, optional, tag="4")]
        pub role: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Who is involved
        #[prost(message, optional, tag="5")]
        pub member: ::core::option::Option<super::super::core::Reference>,
        /// Organization of the practitioner
        #[prost(message, optional, tag="6")]
        pub on_behalf_of: ::core::option::Option<super::super::core::Reference>,
        /// Time period of participant
        #[prost(message, optional, tag="7")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
}
/// Auto-generated from StructureDefinition for USCoreProvenance.
/// US Core Provenance.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-provenance>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreProvenance {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// The Resource this Provenance record supports
    #[prost(message, repeated, tag="10")]
    pub target: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="11")]
    pub occurred: ::core::option::Option<us_core_provenance::OccurredX>,
    /// Timestamp when the activity was recorded / updated
    #[prost(message, optional, tag="12")]
    pub recorded: ::core::option::Option<super::core::Instant>,
    /// Policy or plan the activity was defined by
    #[prost(message, repeated, tag="13")]
    pub policy: prost::alloc::vec::Vec<super::core::Uri>,
    /// Where the activity occurred, if relevant
    #[prost(message, optional, tag="14")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Reason the activity is occurring
    #[prost(message, repeated, tag="15")]
    pub reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Activity that occurred
    #[prost(message, optional, tag="16")]
    pub activity: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, repeated, tag="17")]
    pub agent: prost::alloc::vec::Vec<us_core_provenance::Agent>,
    #[prost(message, repeated, tag="18")]
    pub entity: prost::alloc::vec::Vec<us_core_provenance::Entity>,
    /// Signature on target
    #[prost(message, repeated, tag="19")]
    pub signature: prost::alloc::vec::Vec<super::core::Signature>,
}
/// Nested message and enum types in `USCoreProvenance`.
pub mod us_core_provenance {
    /// When the activity occurred
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OccurredX {
        #[prost(oneof="occurred_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<occurred_x::Choice>,
    }
    /// Nested message and enum types in `OccurredX`.
    pub mod occurred_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Period(super::super::super::core::Period),
            #[prost(message, tag="2")]
            DateTime(super::super::super::core::DateTime),
        }
    }
    /// Actor involved
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Agent {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// How the agent participated
        #[prost(message, optional, tag="4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// What the agents role was
        #[prost(message, repeated, tag="5")]
        pub role: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Who participated
        #[prost(message, optional, tag="6")]
        pub who: ::core::option::Option<super::super::core::Reference>,
        /// Who the agent is representing
        #[prost(message, optional, tag="7")]
        pub on_behalf_of: ::core::option::Option<super::super::core::Reference>,
    }
    /// An entity used in this activity
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Entity {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub role: ::core::option::Option<entity::RoleCode>,
        /// Identity of entity
        #[prost(message, optional, tag="5")]
        pub what: ::core::option::Option<super::super::core::Reference>,
        /// Entity is attributed to this agent
        #[prost(message, repeated, tag="6")]
        pub agent: prost::alloc::vec::Vec<Agent>,
    }
    /// Nested message and enum types in `Entity`.
    pub mod entity {
        /// derivation | revision | quotation | source | removal
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct RoleCode {
            #[prost(enumeration="super::super::super::core::provenance_entity_role_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreImmunizationProfile.
/// Immunization event information.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-immunization>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreImmunizationProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<us_core_immunization_profile::StatusCode>,
    /// Reason not done
    #[prost(message, optional, tag="12")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// Vaccine Product Type (bind to CVX)
    #[prost(message, optional, tag="13")]
    pub vaccine_code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who was immunized
    #[prost(message, optional, tag="14")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Encounter immunization was part of
    #[prost(message, optional, tag="15")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="16")]
    pub occurrence: ::core::option::Option<us_core_immunization_profile::OccurrenceX>,
    /// When the immunization was first captured in the subject's record
    #[prost(message, optional, tag="17")]
    pub recorded: ::core::option::Option<super::core::DateTime>,
    /// Indicates context the data was recorded in
    #[prost(message, optional, tag="18")]
    pub primary_source: ::core::option::Option<super::core::Boolean>,
    /// Indicates the source of a secondarily reported record
    #[prost(message, optional, tag="19")]
    pub report_origin: ::core::option::Option<super::core::CodeableConcept>,
    /// Where immunization occurred
    #[prost(message, optional, tag="20")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Vaccine manufacturer
    #[prost(message, optional, tag="21")]
    pub manufacturer: ::core::option::Option<super::core::Reference>,
    /// Vaccine lot number
    #[prost(message, optional, tag="22")]
    pub lot_number: ::core::option::Option<super::core::String>,
    /// Vaccine expiration date
    #[prost(message, optional, tag="23")]
    pub expiration_date: ::core::option::Option<super::core::Date>,
    /// Body site vaccine  was administered
    #[prost(message, optional, tag="24")]
    pub site: ::core::option::Option<super::core::CodeableConcept>,
    /// How vaccine entered body
    #[prost(message, optional, tag="25")]
    pub route: ::core::option::Option<super::core::CodeableConcept>,
    /// Amount of vaccine administered
    #[prost(message, optional, tag="26")]
    pub dose_quantity: ::core::option::Option<super::core::SimpleQuantity>,
    #[prost(message, repeated, tag="27")]
    pub performer: prost::alloc::vec::Vec<us_core_immunization_profile::Performer>,
    /// Additional immunization notes
    #[prost(message, repeated, tag="28")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Why immunization occurred
    #[prost(message, repeated, tag="29")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Why immunization occurred
    #[prost(message, repeated, tag="30")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Dose potency
    #[prost(message, optional, tag="31")]
    pub is_subpotent: ::core::option::Option<super::core::Boolean>,
    /// Reason for being subpotent
    #[prost(message, repeated, tag="32")]
    pub subpotent_reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, repeated, tag="33")]
    pub education: prost::alloc::vec::Vec<us_core_immunization_profile::Education>,
    /// Patient eligibility for a vaccination program
    #[prost(message, repeated, tag="34")]
    pub program_eligibility: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Funding source for the vaccine
    #[prost(message, optional, tag="35")]
    pub funding_source: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, repeated, tag="36")]
    pub reaction: prost::alloc::vec::Vec<us_core_immunization_profile::Reaction>,
    #[prost(message, repeated, tag="37")]
    pub protocol_applied: prost::alloc::vec::Vec<us_core_immunization_profile::ProtocolApplied>,
}
/// Nested message and enum types in `USCoreImmunizationProfile`.
pub mod us_core_immunization_profile {
    /// completed | entered-in-error | not-done
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::immunization_status_codes_value_set::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Vaccine administration date
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OccurrenceX {
        #[prost(oneof="occurrence_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<occurrence_x::Choice>,
    }
    /// Nested message and enum types in `OccurrenceX`.
    pub mod occurrence_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            StringValue(super::super::super::core::String),
        }
    }
    /// Who performed event
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Performer {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// What type of performance was done
        #[prost(message, optional, tag="4")]
        pub function: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Individual or organization who was performing
        #[prost(message, optional, tag="5")]
        pub actor: ::core::option::Option<super::super::core::Reference>,
    }
    /// Educational material presented to patient
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Education {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Educational material document identifier
        #[prost(message, optional, tag="4")]
        pub document_type: ::core::option::Option<super::super::core::String>,
        /// Educational material reference pointer
        #[prost(message, optional, tag="5")]
        pub reference: ::core::option::Option<super::super::core::Uri>,
        /// Educational material publication date
        #[prost(message, optional, tag="6")]
        pub publication_date: ::core::option::Option<super::super::core::DateTime>,
        /// Educational material presentation date
        #[prost(message, optional, tag="7")]
        pub presentation_date: ::core::option::Option<super::super::core::DateTime>,
    }
    /// Details of a reaction that follows immunization
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Reaction {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// When reaction started
        #[prost(message, optional, tag="4")]
        pub date: ::core::option::Option<super::super::core::DateTime>,
        /// Additional information on reaction
        #[prost(message, optional, tag="5")]
        pub detail: ::core::option::Option<super::super::core::Reference>,
        /// Indicates self-reported reaction
        #[prost(message, optional, tag="6")]
        pub reported: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Protocol followed by the provider
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ProtocolApplied {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Name of vaccine series
        #[prost(message, optional, tag="4")]
        pub series: ::core::option::Option<super::super::core::String>,
        /// Who is responsible for publishing the recommendations
        #[prost(message, optional, tag="5")]
        pub authority: ::core::option::Option<super::super::core::Reference>,
        /// Vaccine preventatable disease being targetted
        #[prost(message, repeated, tag="6")]
        pub target_disease: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag="7")]
        pub dose_number: ::core::option::Option<protocol_applied::DoseNumberX>,
        #[prost(message, optional, tag="8")]
        pub series_doses: ::core::option::Option<protocol_applied::SeriesDosesX>,
    }
    /// Nested message and enum types in `ProtocolApplied`.
    pub mod protocol_applied {
        /// Dose number within series
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DoseNumberX {
            #[prost(oneof="dose_number_x::Choice", tags="1, 2")]
            pub choice: ::core::option::Option<dose_number_x::Choice>,
        }
        /// Nested message and enum types in `DoseNumberX`.
        pub mod dose_number_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag="2")]
                StringValue(super::super::super::super::core::String),
            }
        }
        /// Recommended number of doses for immunity
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct SeriesDosesX {
            #[prost(oneof="series_doses_x::Choice", tags="1, 2")]
            pub choice: ::core::option::Option<series_doses_x::Choice>,
        }
        /// Nested message and enum types in `SeriesDosesX`.
        pub mod series_doses_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                PositiveInt(super::super::super::super::core::PositiveInt),
                #[prost(message, tag="2")]
                StringValue(super::super::super::super::core::String),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreBirthSexExtension.
/// Extension.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct PatientUsCoreBirthSexExtension {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    #[prost(message, optional, tag="3")]
    pub value_code: ::core::option::Option<patient_us_core_birth_sex_extension::ValueCode>,
}
/// Nested message and enum types in `PatientUSCoreBirthSexExtension`.
pub mod patient_us_core_birth_sex_extension {
    /// Value of extension
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueCode {
        #[prost(enumeration="super::birth_sex_value_set::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
}
/// Auto-generated from StructureDefinition for
/// USCoreDiagnosticReportProfileNoteExchange. US Core Diagnostic Report Profile
/// for Report and Note exchange. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-diagnosticreport-note>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreDiagnosticReportProfileNoteExchange {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier for report
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// What was requested
    #[prost(message, repeated, tag="11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="12")]
    pub status: ::core::option::Option<us_core_diagnostic_report_profile_note_exchange::StatusCode>,
    /// Service category
    #[prost(message, repeated, tag="13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// US Core Report Code
    #[prost(message, optional, tag="14")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// The subject of the report - usually, but not always, the patient
    #[prost(message, optional, tag="15")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Health care event when test ordered
    #[prost(message, optional, tag="16")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="17")]
    pub effective: ::core::option::Option<us_core_diagnostic_report_profile_note_exchange::EffectiveX>,
    /// DateTime this version was made
    #[prost(message, optional, tag="18")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Responsible Diagnostic Service
    #[prost(message, repeated, tag="19")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    /// Primary result interpreter
    #[prost(message, repeated, tag="20")]
    pub results_interpreter: prost::alloc::vec::Vec<super::core::Reference>,
    /// Specimens this report is based on
    #[prost(message, repeated, tag="21")]
    pub specimen: prost::alloc::vec::Vec<super::core::Reference>,
    /// Observations
    #[prost(message, repeated, tag="22")]
    pub result: prost::alloc::vec::Vec<super::core::Reference>,
    /// Reference to full details of imaging associated with the diagnostic report
    #[prost(message, repeated, tag="23")]
    pub imaging_study: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="24")]
    pub media: prost::alloc::vec::Vec<us_core_diagnostic_report_profile_note_exchange::Media>,
    /// Clinical conclusion (interpretation) of test results
    #[prost(message, optional, tag="25")]
    pub conclusion: ::core::option::Option<super::core::String>,
    /// Codes for the clinical conclusion of test results
    #[prost(message, repeated, tag="26")]
    pub conclusion_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Entire report as issued
    #[prost(message, repeated, tag="27")]
    pub presented_form: prost::alloc::vec::Vec<super::core::Attachment>,
}
/// Nested message and enum types in `USCoreDiagnosticReportProfileNoteExchange`.
pub mod us_core_diagnostic_report_profile_note_exchange {
    /// registered | partial | preliminary | final +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::diagnostic_report_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Time of the report or note
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof="effective_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Key images associated with this report
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Media {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Comment about the image (e.g. explanation)
        #[prost(message, optional, tag="4")]
        pub comment: ::core::option::Option<super::super::core::String>,
        /// Reference to the image source
        #[prost(message, optional, tag="5")]
        pub link: ::core::option::Option<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for
/// USCoreLaboratoryResultObservationProfile. Measurements and simple assertions.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation-lab>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreLaboratoryResultObservationProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag="11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<us_core_laboratory_result_observation_profile::StatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag="14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Laboratory Test Name
    #[prost(message, optional, tag="15")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who and/or what the observation is about
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// What the observation is about, when it is not about the subject of record
    #[prost(message, repeated, tag="17")]
    pub focus: prost::alloc::vec::Vec<super::core::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag="18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="19")]
    pub effective: ::core::option::Option<us_core_laboratory_result_observation_profile::EffectiveX>,
    /// Date/Time this version was made available
    #[prost(message, optional, tag="20")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag="21")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="22")]
    pub value: ::core::option::Option<us_core_laboratory_result_observation_profile::ValueX>,
    /// Why the result is missing
    #[prost(message, optional, tag="23")]
    pub data_absent_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, repeated, tag="24")]
    pub interpretation: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments about the observation
    #[prost(message, repeated, tag="25")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Observed body part
    #[prost(message, optional, tag="26")]
    pub body_site: ::core::option::Option<super::core::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag="27")]
    pub method: ::core::option::Option<super::core::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag="28")]
    pub specimen: ::core::option::Option<super::core::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag="29")]
    pub device: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="30")]
    pub reference_range: prost::alloc::vec::Vec<us_core_laboratory_result_observation_profile::ReferenceRange>,
    /// Related resource that belongs to the Observation group
    #[prost(message, repeated, tag="31")]
    pub has_member: prost::alloc::vec::Vec<super::core::Reference>,
    /// Related measurements the observation is made from
    #[prost(message, repeated, tag="32")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="33")]
    pub component: prost::alloc::vec::Vec<us_core_laboratory_result_observation_profile::Component>,
}
/// Nested message and enum types in `USCoreLaboratoryResultObservationProfile`.
pub mod us_core_laboratory_result_observation_profile {
    /// registered | preliminary | final | amended +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::observation_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Clinically relevant time/time-period for observation
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof="effective_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Result Value
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof="value_x::Choice", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Quantity(super::super::super::core::Quantity),
            #[prost(message, tag="2")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag="3")]
            StringValue(super::super::super::core::String),
            #[prost(message, tag="4")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag="5")]
            Integer(super::super::super::core::Integer),
            #[prost(message, tag="6")]
            Range(super::super::super::core::Range),
            #[prost(message, tag="7")]
            Ratio(super::super::super::core::Ratio),
            #[prost(message, tag="8")]
            SampledData(super::super::super::core::SampledData),
            #[prost(message, tag="9")]
            Time(super::super::super::core::Time),
            #[prost(message, tag="10")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="11")]
            Period(super::super::super::core::Period),
        }
    }
    /// Provides guide for interpretation
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReferenceRange {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag="4")]
        pub low: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag="5")]
        pub high: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag="7")]
        pub applies_to: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag="8")]
        pub age: ::core::option::Option<super::super::core::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag="9")]
        pub text: ::core::option::Option<super::super::core::String>,
    }
    /// Component results
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Component {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub value: ::core::option::Option<component::ValueX>,
        /// Why the component result is missing
        #[prost(message, optional, tag="6")]
        pub data_absent_reason: ::core::option::Option<super::super::core::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, repeated, tag="7")]
        pub interpretation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag="8")]
        pub reference_range: prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Actual component result
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof="value_x::Choice", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag="3")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag="4")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="5")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag="6")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag="7")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag="8")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag="9")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag="10")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag="11")]
                Period(super::super::super::super::core::Period),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreLocation.
/// Details and position information for a physical place.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-location>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreLocation {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Unique code or number identifying the location to its users
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<us_core_location::StatusCode>,
    /// The operational status of the location (typically only for a bed/room)
    #[prost(message, optional, tag="12")]
    pub operational_status: ::core::option::Option<super::core::Coding>,
    /// Name of the location as used by humans
    #[prost(message, optional, tag="13")]
    pub name: ::core::option::Option<super::core::String>,
    /// A list of alternate names that the location is known as, or was known as,
    /// in the past
    #[prost(message, repeated, tag="14")]
    pub alias: prost::alloc::vec::Vec<super::core::String>,
    /// Additional details about the location that could be displayed as further
    /// information to identify the location beyond its name
    #[prost(message, optional, tag="15")]
    pub description: ::core::option::Option<super::core::String>,
    #[prost(message, optional, tag="16")]
    pub mode: ::core::option::Option<us_core_location::ModeCode>,
    /// Type of function performed
    #[prost(message, repeated, tag="17")]
    pub r#type: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Contact details of the location
    #[prost(message, repeated, tag="18")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Physical location
    #[prost(message, optional, tag="19")]
    pub address: ::core::option::Option<super::core::Address>,
    /// Physical form of the location
    #[prost(message, optional, tag="20")]
    pub physical_type: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag="21")]
    pub position: ::core::option::Option<us_core_location::Position>,
    /// Organization responsible for provisioning and upkeep
    #[prost(message, optional, tag="22")]
    pub managing_organization: ::core::option::Option<super::core::Reference>,
    /// Another Location this one is physically a part of
    #[prost(message, optional, tag="23")]
    pub part_of: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="24")]
    pub hours_of_operation: prost::alloc::vec::Vec<us_core_location::HoursOfOperation>,
    /// Description of availability exceptions
    #[prost(message, optional, tag="25")]
    pub availability_exceptions: ::core::option::Option<super::core::String>,
    /// Technical endpoints providing access to services operated for the location
    #[prost(message, repeated, tag="26")]
    pub endpoint: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `USCoreLocation`.
pub mod us_core_location {
    /// active | suspended | inactive
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::location_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// instance | kind
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ModeCode {
        #[prost(enumeration="super::super::core::location_mode_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// The absolute geographic location
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Position {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Longitude with WGS84 datum
        #[prost(message, optional, tag="4")]
        pub longitude: ::core::option::Option<super::super::core::Decimal>,
        /// Latitude with WGS84 datum
        #[prost(message, optional, tag="5")]
        pub latitude: ::core::option::Option<super::super::core::Decimal>,
        /// Altitude with WGS84 datum
        #[prost(message, optional, tag="6")]
        pub altitude: ::core::option::Option<super::super::core::Decimal>,
    }
    /// What days/times during a week is this location usually open
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct HoursOfOperation {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, repeated, tag="4")]
        pub days_of_week: prost::alloc::vec::Vec<hours_of_operation::DaysOfWeekCode>,
        /// The Location is open all day
        #[prost(message, optional, tag="5")]
        pub all_day: ::core::option::Option<super::super::core::Boolean>,
        /// Time that the Location opens
        #[prost(message, optional, tag="6")]
        pub opening_time: ::core::option::Option<super::super::core::Time>,
        /// Time that the Location closes
        #[prost(message, optional, tag="7")]
        pub closing_time: ::core::option::Option<super::super::core::Time>,
    }
    /// Nested message and enum types in `HoursOfOperation`.
    pub mod hours_of_operation {
        /// mon | tue | wed | thu | fri | sat | sun
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DaysOfWeekCode {
            #[prost(enumeration="super::super::super::core::days_of_week_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreCondition.
/// Detailed information about conditions, problems or diagnoses.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-condition>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreCondition {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this condition
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// active | recurrence | relapse | inactive | remission | resolved
    #[prost(message, optional, tag="11")]
    pub clinical_status: ::core::option::Option<super::core::CodeableConcept>,
    /// unconfirmed | provisional | differential | confirmed | refuted |
    /// entered-in-error
    #[prost(message, optional, tag="12")]
    pub verification_status: ::core::option::Option<super::core::CodeableConcept>,
    /// problem-list-item | encounter-diagnosis | health-concern
    #[prost(message, repeated, tag="13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Subjective severity of condition
    #[prost(message, optional, tag="14")]
    pub severity: ::core::option::Option<super::core::CodeableConcept>,
    /// Identification of the condition, problem or diagnosis
    #[prost(message, optional, tag="15")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Anatomical location, if relevant
    #[prost(message, repeated, tag="16")]
    pub body_site: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Who has the condition?
    #[prost(message, optional, tag="17")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag="18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="19")]
    pub onset: ::core::option::Option<us_core_condition::OnsetX>,
    #[prost(message, optional, tag="20")]
    pub abatement: ::core::option::Option<us_core_condition::AbatementX>,
    /// Date record was first recorded
    #[prost(message, optional, tag="21")]
    pub recorded_date: ::core::option::Option<super::core::DateTime>,
    /// Who recorded the condition
    #[prost(message, optional, tag="22")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Person who asserts this condition
    #[prost(message, optional, tag="23")]
    pub asserter: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="24")]
    pub stage: prost::alloc::vec::Vec<us_core_condition::Stage>,
    #[prost(message, repeated, tag="25")]
    pub evidence: prost::alloc::vec::Vec<us_core_condition::Evidence>,
    /// Additional information about the Condition
    #[prost(message, repeated, tag="26")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `UsCoreCondition`.
pub mod us_core_condition {
    /// Estimated or actual date,  date-time, or age
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OnsetX {
        #[prost(oneof="onset_x::Choice", tags="1, 2, 3, 4, 5")]
        pub choice: ::core::option::Option<onset_x::Choice>,
    }
    /// Nested message and enum types in `OnsetX`.
    pub mod onset_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Age(super::super::super::core::Age),
            #[prost(message, tag="3")]
            Period(super::super::super::core::Period),
            #[prost(message, tag="4")]
            Range(super::super::super::core::Range),
            #[prost(message, tag="5")]
            StringValue(super::super::super::core::String),
        }
    }
    /// When in resolution/remission
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct AbatementX {
        #[prost(oneof="abatement_x::Choice", tags="1, 2, 3, 4, 5")]
        pub choice: ::core::option::Option<abatement_x::Choice>,
    }
    /// Nested message and enum types in `AbatementX`.
    pub mod abatement_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Age(super::super::super::core::Age),
            #[prost(message, tag="3")]
            Period(super::super::super::core::Period),
            #[prost(message, tag="4")]
            Range(super::super::super::core::Range),
            #[prost(message, tag="5")]
            StringValue(super::super::super::core::String),
        }
    }
    /// Stage/grade, usually assessed formally
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Stage {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Simple summary (disease specific)
        #[prost(message, optional, tag="4")]
        pub summary: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Formal record of assessment
        #[prost(message, repeated, tag="5")]
        pub assessment: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// Kind of staging
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Supporting evidence
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Evidence {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Manifestation/symptom
        #[prost(message, repeated, tag="4")]
        pub code: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Supporting information found elsewhere
        #[prost(message, repeated, tag="5")]
        pub detail: prost::alloc::vec::Vec<super::super::core::Reference>,
    }
}
/// Auto-generated from StructureDefinition for USCorePulseOximetryProfile.
/// FHIR Oxygen Saturation Profile.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-pulse-oximetry>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCorePulseOximetryProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag="11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<us_core_pulse_oximetry_profile::StatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag="14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag="15")]
    pub code: ::core::option::Option<us_core_pulse_oximetry_profile::CodeableConceptForCode>,
    /// Who and/or what the observation is about
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// What the observation is about, when it is not about the subject of record
    #[prost(message, repeated, tag="17")]
    pub focus: prost::alloc::vec::Vec<super::core::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag="18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="19")]
    pub effective: ::core::option::Option<us_core_pulse_oximetry_profile::EffectiveX>,
    /// Date/Time this version was made available
    #[prost(message, optional, tag="20")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag="21")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="22")]
    pub value: ::core::option::Option<us_core_pulse_oximetry_profile::ValueX>,
    /// Why the result is missing
    #[prost(message, optional, tag="23")]
    pub data_absent_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, repeated, tag="24")]
    pub interpretation: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments about the observation
    #[prost(message, repeated, tag="25")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Observed body part
    #[prost(message, optional, tag="26")]
    pub body_site: ::core::option::Option<super::core::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag="27")]
    pub method: ::core::option::Option<super::core::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag="28")]
    pub specimen: ::core::option::Option<super::core::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag="29")]
    pub device: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="30")]
    pub reference_range: prost::alloc::vec::Vec<us_core_pulse_oximetry_profile::ReferenceRange>,
    /// Used when reporting vital signs panel components
    #[prost(message, repeated, tag="31")]
    pub has_member: prost::alloc::vec::Vec<super::core::Reference>,
    /// Related measurements the observation is made from
    #[prost(message, repeated, tag="32")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="33")]
    pub component: prost::alloc::vec::Vec<us_core_pulse_oximetry_profile::Component>,
}
/// Nested message and enum types in `USCorePulseOximetryProfile`.
pub mod us_core_pulse_oximetry_profile {
    /// registered | preliminary | final | amended +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::observation_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Oxygen Saturation by Pulse Oximetry
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CodeableConceptForCode {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Code defined by a terminology system
        #[prost(message, repeated, tag="3")]
        pub coding: prost::alloc::vec::Vec<super::super::core::Coding>,
        /// Plain text representation of the concept
        #[prost(message, optional, tag="4")]
        pub text: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="5")]
        pub oxygen_sat_code: ::core::option::Option<super::super::core::CodingWithFixedCode>,
        #[prost(message, optional, tag="6")]
        pub pulse_ox: ::core::option::Option<super::super::core::CodingWithFixedCode>,
    }
    /// Often just a dateTime for Vital Signs
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof="effective_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Vital Signs value are recorded using the Quantity data type. For supporting
    /// observations such as Cuff size could use other datatypes such as
    /// CodeableConcept.
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof="value_x::Choice", tags="1")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Quantity(super::super::super::core::Quantity),
        }
    }
    /// Provides guide for interpretation
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReferenceRange {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag="4")]
        pub low: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag="5")]
        pub high: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag="7")]
        pub applies_to: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag="8")]
        pub age: ::core::option::Option<super::super::core::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag="9")]
        pub text: ::core::option::Option<super::super::core::String>,
    }
    /// Used when reporting systolic and diastolic blood pressure.
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Component {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub value: ::core::option::Option<component::ValueX>,
        /// Why the component result is missing
        #[prost(message, optional, tag="6")]
        pub data_absent_reason: ::core::option::Option<super::super::core::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, repeated, tag="7")]
        pub interpretation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag="8")]
        pub reference_range: prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Vital Sign Value recorded with UCUM
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof="value_x::Choice", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag="3")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag="4")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="5")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag="6")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag="7")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag="8")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag="9")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag="10")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag="11")]
                Period(super::super::super::super::core::Period),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreImplantableDeviceProfile.
/// Item used in healthcare.
/// See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-implantable-device>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreImplantableDeviceProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Instance identifier
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// The reference to the definition for the device
    #[prost(message, optional, tag="11")]
    pub definition: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="12")]
    pub udi_carrier: ::core::option::Option<us_core_implantable_device_profile::UdiCarrier>,
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<us_core_implantable_device_profile::StatusCode>,
    /// online | paused | standby | offline | not-ready | transduc-discon |
    /// hw-discon | off
    #[prost(message, repeated, tag="14")]
    pub status_reason: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The distinct identification string
    #[prost(message, optional, tag="15")]
    pub distinct_identifier: ::core::option::Option<super::core::String>,
    /// Name of device manufacturer
    #[prost(message, optional, tag="16")]
    pub manufacturer: ::core::option::Option<super::core::String>,
    /// Date when the device was made
    #[prost(message, optional, tag="17")]
    pub manufacture_date: ::core::option::Option<super::core::DateTime>,
    /// Date and time of expiry of this device (if applicable)
    #[prost(message, optional, tag="18")]
    pub expiration_date: ::core::option::Option<super::core::DateTime>,
    /// Lot number of manufacture
    #[prost(message, optional, tag="19")]
    pub lot_number: ::core::option::Option<super::core::String>,
    /// Serial number assigned by the manufacturer
    #[prost(message, optional, tag="20")]
    pub serial_number: ::core::option::Option<super::core::String>,
    #[prost(message, repeated, tag="21")]
    pub device_name: prost::alloc::vec::Vec<us_core_implantable_device_profile::DeviceName>,
    /// The model number for the device
    #[prost(message, optional, tag="22")]
    pub model_number: ::core::option::Option<super::core::String>,
    /// The part number of the device
    #[prost(message, optional, tag="23")]
    pub part_number: ::core::option::Option<super::core::String>,
    /// The kind or type of device
    #[prost(message, optional, tag="24")]
    pub r#type: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, repeated, tag="25")]
    pub specialization: prost::alloc::vec::Vec<us_core_implantable_device_profile::Specialization>,
    #[prost(message, repeated, tag="26")]
    pub version: prost::alloc::vec::Vec<us_core_implantable_device_profile::Version>,
    #[prost(message, repeated, tag="27")]
    pub property: prost::alloc::vec::Vec<us_core_implantable_device_profile::Property>,
    /// Patient to whom Device is affixed
    #[prost(message, optional, tag="28")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Organization responsible for device
    #[prost(message, optional, tag="29")]
    pub owner: ::core::option::Option<super::core::Reference>,
    /// Details for human/organization for support
    #[prost(message, repeated, tag="30")]
    pub contact: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// Where the device is found
    #[prost(message, optional, tag="31")]
    pub location: ::core::option::Option<super::core::Reference>,
    /// Network address to contact device
    #[prost(message, optional, tag="32")]
    pub url: ::core::option::Option<super::core::Uri>,
    /// Device notes and comments
    #[prost(message, repeated, tag="33")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Safety Characteristics of Device
    #[prost(message, repeated, tag="34")]
    pub safety: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The parent device
    #[prost(message, optional, tag="35")]
    pub parent: ::core::option::Option<super::core::Reference>,
}
/// Nested message and enum types in `USCoreImplantableDeviceProfile`.
pub mod us_core_implantable_device_profile {
    /// Unique Device Identifier (UDI) Barcode string
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct UdiCarrier {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Mandatory fixed portion of UDI
        #[prost(message, optional, tag="4")]
        pub device_identifier: ::core::option::Option<super::super::core::String>,
        /// UDI Issuing Organization
        #[prost(message, optional, tag="5")]
        pub issuer: ::core::option::Option<super::super::core::Uri>,
        /// Regional UDI authority
        #[prost(message, optional, tag="6")]
        pub jurisdiction: ::core::option::Option<super::super::core::Uri>,
        /// UDI Machine Readable Barcode String
        #[prost(message, optional, tag="7")]
        pub carrier_aidc: ::core::option::Option<super::super::core::Base64Binary>,
        /// UDI Human Readable Barcode String
        #[prost(message, optional, tag="8")]
        pub carrier_hrf: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="9")]
        pub entry_type: ::core::option::Option<udi_carrier::EntryTypeCode>,
    }
    /// Nested message and enum types in `UdiCarrier`.
    pub mod udi_carrier {
        /// barcode | rfid | manual +
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct EntryTypeCode {
            #[prost(enumeration="super::super::super::core::udi_entry_type_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// active | inactive | entered-in-error | unknown
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::fhir_device_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// The name of the device as given by the manufacturer
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DeviceName {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The name of the device
        #[prost(message, optional, tag="4")]
        pub name: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="5")]
        pub r#type: ::core::option::Option<device_name::TypeCode>,
    }
    /// Nested message and enum types in `DeviceName`.
    pub mod device_name {
        /// udi-label-name | user-friendly-name | patient-reported-name |
        /// manufacturer-name | model-name | other
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct TypeCode {
            #[prost(enumeration="super::super::super::core::device_name_type_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// The capabilities supported on a  device, the standards to which the device
    /// conforms for a particular purpose, and used for the communication
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Specialization {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The standard that is used to operate and communicate
        #[prost(message, optional, tag="4")]
        pub system_type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// The version of the standard that is used to operate and communicate
        #[prost(message, optional, tag="5")]
        pub version: ::core::option::Option<super::super::core::String>,
    }
    /// The actual design of the device or software version running on the device
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Version {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The type of the device version
        #[prost(message, optional, tag="4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// A single component of the device version
        #[prost(message, optional, tag="5")]
        pub component: ::core::option::Option<super::super::core::Identifier>,
        /// The version text
        #[prost(message, optional, tag="6")]
        pub value: ::core::option::Option<super::super::core::String>,
    }
    /// The actual configuration settings of a device as it actually operates,
    /// e.g., regulation status, time properties
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Property {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Code that specifies the property DeviceDefinitionPropetyCode (Extensible)
        #[prost(message, optional, tag="4")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Property value as a quantity
        #[prost(message, repeated, tag="5")]
        pub value_quantity: prost::alloc::vec::Vec<super::super::core::Quantity>,
        /// Property value as a code, e.g., NTP4 (synced to NTP)
        #[prost(message, repeated, tag="6")]
        pub value_code: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
    }
}
/// Auto-generated from StructureDefinition for USCoreMedicationRequestProfile.
/// Ordering of medication for patient or group.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-medicationrequest>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreMedicationRequestProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External ids for this request
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag="11")]
    pub status: ::core::option::Option<us_core_medication_request_profile::StatusCode>,
    /// Reason for current status
    #[prost(message, optional, tag="12")]
    pub status_reason: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag="13")]
    pub intent: ::core::option::Option<us_core_medication_request_profile::IntentCode>,
    /// Type of medication usage
    #[prost(message, repeated, tag="14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    #[prost(message, optional, tag="15")]
    pub priority: ::core::option::Option<us_core_medication_request_profile::PriorityCode>,
    /// True if request is prohibiting action
    #[prost(message, optional, tag="16")]
    pub do_not_perform: ::core::option::Option<super::core::Boolean>,
    #[prost(message, optional, tag="17")]
    pub reported: ::core::option::Option<us_core_medication_request_profile::ReportedX>,
    #[prost(message, optional, tag="18")]
    pub medication: ::core::option::Option<us_core_medication_request_profile::MedicationX>,
    /// Who or group medication request is for
    #[prost(message, optional, tag="19")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of encounter/admission/stay
    #[prost(message, optional, tag="20")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Information to support ordering of the medication
    #[prost(message, repeated, tag="21")]
    pub supporting_information: prost::alloc::vec::Vec<super::core::Reference>,
    /// When request was initially authored
    #[prost(message, optional, tag="22")]
    pub authored_on: ::core::option::Option<super::core::DateTime>,
    /// Who/What requested the Request
    #[prost(message, optional, tag="23")]
    pub requester: ::core::option::Option<super::core::Reference>,
    /// Intended performer of administration
    #[prost(message, optional, tag="24")]
    pub performer: ::core::option::Option<super::core::Reference>,
    /// Desired kind of performer of the medication administration
    #[prost(message, optional, tag="25")]
    pub performer_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Person who entered the request
    #[prost(message, optional, tag="26")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Reason or indication for ordering or not ordering the medication
    #[prost(message, repeated, tag="27")]
    pub reason_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Condition or observation that supports why the prescription is being
    /// written
    #[prost(message, repeated, tag="28")]
    pub reason_reference: prost::alloc::vec::Vec<super::core::Reference>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag="29")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag="30")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// What request fulfills
    #[prost(message, repeated, tag="31")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Composite request this is part of
    #[prost(message, optional, tag="32")]
    pub group_identifier: ::core::option::Option<super::core::Identifier>,
    /// Overall pattern of medication administration
    #[prost(message, optional, tag="33")]
    pub course_of_therapy_type: ::core::option::Option<super::core::CodeableConcept>,
    /// Associated insurance coverage
    #[prost(message, repeated, tag="34")]
    pub insurance: prost::alloc::vec::Vec<super::core::Reference>,
    /// Information about the prescription
    #[prost(message, repeated, tag="35")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// How the medication should be taken
    #[prost(message, repeated, tag="36")]
    pub dosage_instruction: prost::alloc::vec::Vec<super::core::Dosage>,
    #[prost(message, optional, tag="37")]
    pub dispense_request: ::core::option::Option<us_core_medication_request_profile::DispenseRequest>,
    #[prost(message, optional, tag="38")]
    pub substitution: ::core::option::Option<us_core_medication_request_profile::Substitution>,
    /// An order/prescription that is being replaced
    #[prost(message, optional, tag="39")]
    pub prior_prescription: ::core::option::Option<super::core::Reference>,
    /// Clinical Issue with action
    #[prost(message, repeated, tag="40")]
    pub detected_issue: prost::alloc::vec::Vec<super::core::Reference>,
    /// A list of events of interest in the lifecycle
    #[prost(message, repeated, tag="41")]
    pub event_history: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `USCoreMedicationRequestProfile`.
pub mod us_core_medication_request_profile {
    /// active | on-hold | cancelled | completed | entered-in-error | stopped |
    /// draft | unknown
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::medicationrequest_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// proposal | plan | order | original-order | reflex-order | filler-order |
    /// instance-order | option
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct IntentCode {
        #[prost(enumeration="super::super::core::medication_request_intent_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// routine | urgent | asap | stat
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct PriorityCode {
        #[prost(enumeration="super::super::core::request_priority_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Reported rather than primary record
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReportedX {
        #[prost(oneof="reported_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<reported_x::Choice>,
    }
    /// Nested message and enum types in `ReportedX`.
    pub mod reported_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag="2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// Medication to be taken
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct MedicationX {
        #[prost(oneof="medication_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<medication_x::Choice>,
    }
    /// Nested message and enum types in `MedicationX`.
    pub mod medication_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            CodeableConcept(super::super::super::core::CodeableConcept),
            #[prost(message, tag="2")]
            Reference(super::super::super::core::Reference),
        }
    }
    /// Medication supply authorization
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DispenseRequest {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub initial_fill: ::core::option::Option<dispense_request::InitialFill>,
        /// Minimum period of time between dispenses
        #[prost(message, optional, tag="5")]
        pub dispense_interval: ::core::option::Option<super::super::core::Duration>,
        /// Time period supply is authorized for
        #[prost(message, optional, tag="6")]
        pub validity_period: ::core::option::Option<super::super::core::Period>,
        /// Number of refills authorized
        #[prost(message, optional, tag="7")]
        pub number_of_repeats_allowed: ::core::option::Option<super::super::core::UnsignedInt>,
        /// Amount of medication to supply per dispense
        #[prost(message, optional, tag="8")]
        pub quantity: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Number of days supply per dispense
        #[prost(message, optional, tag="9")]
        pub expected_supply_duration: ::core::option::Option<super::super::core::Duration>,
        /// Intended dispenser
        #[prost(message, optional, tag="10")]
        pub performer: ::core::option::Option<super::super::core::Reference>,
    }
    /// Nested message and enum types in `DispenseRequest`.
    pub mod dispense_request {
        /// First fill details
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct InitialFill {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag="1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag="2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag="3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// First fill quantity
            #[prost(message, optional, tag="4")]
            pub quantity: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            /// First fill duration
            #[prost(message, optional, tag="5")]
            pub duration: ::core::option::Option<super::super::super::core::Duration>,
        }
    }
    /// Any restrictions on medication substitution
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Substitution {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub allowed: ::core::option::Option<substitution::AllowedX>,
        /// Why should (not) substitution be made
        #[prost(message, optional, tag="5")]
        pub reason: ::core::option::Option<super::super::core::CodeableConcept>,
    }
    /// Nested message and enum types in `Substitution`.
    pub mod substitution {
        /// Whether substitution is allowed or not
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct AllowedX {
            #[prost(oneof="allowed_x::Choice", tags="1, 2")]
            pub choice: ::core::option::Option<allowed_x::Choice>,
        }
        /// Nested message and enum types in `AllowedX`.
        pub mod allowed_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreMedicationProfile.
/// Definition of a Medication.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-medication>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreMedicationProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business identifier for this medication
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Codes that identify this medication
    #[prost(message, optional, tag="11")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag="12")]
    pub status: ::core::option::Option<us_core_medication_profile::StatusCode>,
    /// Manufacturer of the item
    #[prost(message, optional, tag="13")]
    pub manufacturer: ::core::option::Option<super::core::Reference>,
    /// powder | tablets | capsule +
    #[prost(message, optional, tag="14")]
    pub form: ::core::option::Option<super::core::CodeableConcept>,
    /// Amount of drug in package
    #[prost(message, optional, tag="15")]
    pub amount: ::core::option::Option<super::core::Ratio>,
    #[prost(message, repeated, tag="16")]
    pub ingredient: prost::alloc::vec::Vec<us_core_medication_profile::Ingredient>,
    #[prost(message, optional, tag="17")]
    pub batch: ::core::option::Option<us_core_medication_profile::Batch>,
}
/// Nested message and enum types in `USCoreMedicationProfile`.
pub mod us_core_medication_profile {
    /// active | inactive | entered-in-error
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::medication_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Active or inactive ingredient
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Ingredient {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub item: ::core::option::Option<ingredient::ItemX>,
        /// Active ingredient indicator
        #[prost(message, optional, tag="5")]
        pub is_active: ::core::option::Option<super::super::core::Boolean>,
        /// Quantity of ingredient present
        #[prost(message, optional, tag="6")]
        pub strength: ::core::option::Option<super::super::core::Ratio>,
    }
    /// Nested message and enum types in `Ingredient`.
    pub mod ingredient {
        /// The actual ingredient or content
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ItemX {
            #[prost(oneof="item_x::Choice", tags="1, 2")]
            pub choice: ::core::option::Option<item_x::Choice>,
        }
        /// Nested message and enum types in `ItemX`.
        pub mod item_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag="2")]
                Reference(super::super::super::super::core::Reference),
            }
        }
    }
    /// Details about packaged medications
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Batch {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Identifier assigned to batch
        #[prost(message, optional, tag="4")]
        pub lot_number: ::core::option::Option<super::super::core::String>,
        /// When batch will expire
        #[prost(message, optional, tag="5")]
        pub expiration_date: ::core::option::Option<super::super::core::DateTime>,
    }
}
/// Auto-generated from StructureDefinition for USCoreGoalProfile.
/// Describes the intended objective(s) for a patient, group or organization.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-goal>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreGoalProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this goal
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    #[prost(message, optional, tag="11")]
    pub lifecycle_status: ::core::option::Option<us_core_goal_profile::LifecycleStatusCode>,
    /// in-progress | improving | worsening | no-change | achieved | sustaining |
    /// not-achieved | no-progress | not-attainable
    #[prost(message, optional, tag="12")]
    pub achievement_status: ::core::option::Option<super::core::CodeableConcept>,
    /// E.g. Treatment, dietary, behavioral, etc.
    #[prost(message, repeated, tag="13")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// high-priority | medium-priority | low-priority
    #[prost(message, optional, tag="14")]
    pub priority: ::core::option::Option<super::core::CodeableConcept>,
    /// Code or text describing goal
    #[prost(message, optional, tag="15")]
    pub description: ::core::option::Option<super::core::CodeableConcept>,
    /// Who this goal is intended for
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="17")]
    pub start: ::core::option::Option<us_core_goal_profile::StartX>,
    #[prost(message, repeated, tag="18")]
    pub target: prost::alloc::vec::Vec<us_core_goal_profile::Target>,
    /// When goal status took effect
    #[prost(message, optional, tag="19")]
    pub status_date: ::core::option::Option<super::core::Date>,
    /// Reason for current status
    #[prost(message, optional, tag="20")]
    pub status_reason: ::core::option::Option<super::core::String>,
    /// Who's responsible for creating Goal?
    #[prost(message, optional, tag="21")]
    pub expressed_by: ::core::option::Option<super::core::Reference>,
    /// Issues addressed by this goal
    #[prost(message, repeated, tag="22")]
    pub addresses: prost::alloc::vec::Vec<super::core::Reference>,
    /// Comments about the goal
    #[prost(message, repeated, tag="23")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// What result was achieved regarding the goal?
    #[prost(message, repeated, tag="24")]
    pub outcome_code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Observation that resulted from goal
    #[prost(message, repeated, tag="25")]
    pub outcome_reference: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `USCoreGoalProfile`.
pub mod us_core_goal_profile {
    /// proposed | planned | accepted | active | on-hold | completed | cancelled |
    /// entered-in-error | rejected
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct LifecycleStatusCode {
        #[prost(enumeration="super::super::core::goal_lifecycle_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// When goal pursuit begins
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StartX {
        #[prost(oneof="start_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<start_x::Choice>,
    }
    /// Nested message and enum types in `StartX`.
    pub mod start_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Date(super::super::super::core::Date),
            #[prost(message, tag="2")]
            CodeableConcept(super::super::super::core::CodeableConcept),
        }
    }
    /// Target outcome for the goal
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Target {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The parameter whose value is being tracked
        #[prost(message, optional, tag="4")]
        pub measure: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub detail: ::core::option::Option<target::DetailX>,
        #[prost(message, optional, tag="6")]
        pub due: ::core::option::Option<target::DueX>,
    }
    /// Nested message and enum types in `Target`.
    pub mod target {
        /// The target value to be achieved
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DetailX {
            #[prost(oneof="detail_x::Choice", tags="1, 2, 3, 4, 5, 6, 7")]
            pub choice: ::core::option::Option<detail_x::Choice>,
        }
        /// Nested message and enum types in `DetailX`.
        pub mod detail_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag="2")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag="3")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag="4")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag="5")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="6")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag="7")]
                Ratio(super::super::super::super::core::Ratio),
            }
        }
        /// Reach goal on or before
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DueX {
            #[prost(oneof="due_x::Choice", tags="1")]
            pub choice: ::core::option::Option<due_x::Choice>,
        }
        /// Nested message and enum types in `DueX`.
        pub mod due_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Date(super::super::super::super::core::Date),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreRaceExtension.
/// US Core Race Extension.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-race>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct PatientUsCoreRaceExtension {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    #[prost(message, repeated, tag="4")]
    pub omb_category: prost::alloc::vec::Vec<patient_us_core_race_extension::OmbCategoryCoding>,
    #[prost(message, repeated, tag="5")]
    pub detailed: prost::alloc::vec::Vec<patient_us_core_race_extension::DetailedCoding>,
    /// Race Text
    #[prost(message, optional, tag="6")]
    pub text: ::core::option::Option<super::core::String>,
}
/// Nested message and enum types in `PatientUSCoreRaceExtension`.
pub mod patient_us_core_race_extension {
    /// American Indian or Alaska Native|Asian|Black or African American|Native
    /// Hawaiian or Other Pacific Islander|White
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OmbCategoryCoding {
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub version: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="5")]
        pub code: ::core::option::Option<omb_category_coding::BoundCode>,
        #[prost(message, optional, tag="6")]
        pub display: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="7")]
        pub user_selected: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Nested message and enum types in `OmbCategoryCoding`.
    pub mod omb_category_coding {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct BoundCode {
            #[prost(enumeration="super::super::omb_race_categories_value_set::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// Extended race codes
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DetailedCoding {
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub version: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="5")]
        pub code: ::core::option::Option<detailed_coding::BoundCode>,
        #[prost(message, optional, tag="6")]
        pub display: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="7")]
        pub user_selected: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Nested message and enum types in `DetailedCoding`.
    pub mod detailed_coding {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct BoundCode {
            #[prost(enumeration="super::super::detailed_race_value_set::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for USCorePractitionerRoleProfile.
/// Roles/organizations the practitioner is associated with.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitionerrole>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCorePractitionerRoleProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifiers that are specific to a role/location
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether this practitioner role record is in active use
    #[prost(message, optional, tag="11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// The period during which the practitioner is authorized to perform in these
    /// role(s)
    #[prost(message, optional, tag="12")]
    pub period: ::core::option::Option<super::core::Period>,
    /// Practitioner that is able to provide the defined services for the
    /// organization
    #[prost(message, optional, tag="13")]
    pub practitioner: ::core::option::Option<super::core::Reference>,
    /// Organization where the roles are available
    #[prost(message, optional, tag="14")]
    pub organization: ::core::option::Option<super::core::Reference>,
    /// Roles which this practitioner may perform
    #[prost(message, repeated, tag="15")]
    pub code: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Specific specialty of the practitioner
    #[prost(message, repeated, tag="16")]
    pub specialty: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// The location(s) at which this practitioner provides care
    #[prost(message, repeated, tag="17")]
    pub location: prost::alloc::vec::Vec<super::core::Reference>,
    /// The list of healthcare services that this worker provides for this role's
    /// Organization/Location(s)
    #[prost(message, repeated, tag="18")]
    pub healthcare_service: prost::alloc::vec::Vec<super::core::Reference>,
    /// Contact details that are specific to the role/location/service
    #[prost(message, repeated, tag="19")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    #[prost(message, repeated, tag="20")]
    pub available_time: prost::alloc::vec::Vec<us_core_practitioner_role_profile::AvailableTime>,
    #[prost(message, repeated, tag="21")]
    pub not_available: prost::alloc::vec::Vec<us_core_practitioner_role_profile::NotAvailable>,
    /// Description of availability exceptions
    #[prost(message, optional, tag="22")]
    pub availability_exceptions: ::core::option::Option<super::core::String>,
    /// Technical endpoints providing access to services operated for the
    /// practitioner with this role
    #[prost(message, repeated, tag="23")]
    pub endpoint: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `USCorePractitionerRoleProfile`.
pub mod us_core_practitioner_role_profile {
    /// Times the Service Site is available
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct AvailableTime {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, repeated, tag="4")]
        pub days_of_week: prost::alloc::vec::Vec<available_time::DaysOfWeekCode>,
        /// Always available? e.g. 24 hour service
        #[prost(message, optional, tag="5")]
        pub all_day: ::core::option::Option<super::super::core::Boolean>,
        /// Opening time of day (ignored if allDay = true)
        #[prost(message, optional, tag="6")]
        pub available_start_time: ::core::option::Option<super::super::core::Time>,
        /// Closing time of day (ignored if allDay = true)
        #[prost(message, optional, tag="7")]
        pub available_end_time: ::core::option::Option<super::super::core::Time>,
    }
    /// Nested message and enum types in `AvailableTime`.
    pub mod available_time {
        /// mon | tue | wed | thu | fri | sat | sun
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct DaysOfWeekCode {
            #[prost(enumeration="super::super::super::core::days_of_week_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// Not available during this time due to provided reason
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct NotAvailable {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Reason presented to the user explaining why time not available
        #[prost(message, optional, tag="4")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Service not available from this date
        #[prost(message, optional, tag="5")]
        pub during: ::core::option::Option<super::super::core::Period>,
    }
}
/// Auto-generated from StructureDefinition for USCoreAllergyIntolerance.
/// Allergy or Intolerance (generally: Risk of adverse reaction to a substance).
/// See
/// <http://hl7.org/fhir/us/core/StructureDefinition/us-core-allergyintolerance>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreAllergyIntolerance {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External ids for this item
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// active | inactive | resolved
    #[prost(message, optional, tag="11")]
    pub clinical_status: ::core::option::Option<super::core::CodeableConcept>,
    /// unconfirmed | confirmed | refuted | entered-in-error
    #[prost(message, optional, tag="12")]
    pub verification_status: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag="13")]
    pub r#type: ::core::option::Option<us_core_allergy_intolerance::TypeCode>,
    #[prost(message, repeated, tag="14")]
    pub category: prost::alloc::vec::Vec<us_core_allergy_intolerance::CategoryCode>,
    #[prost(message, optional, tag="15")]
    pub criticality: ::core::option::Option<us_core_allergy_intolerance::CriticalityCode>,
    /// Code that identifies the allergy or intolerance
    #[prost(message, optional, tag="16")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who the sensitivity is for
    #[prost(message, optional, tag="17")]
    pub patient: ::core::option::Option<super::core::Reference>,
    /// Encounter when the allergy or intolerance was asserted
    #[prost(message, optional, tag="18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="19")]
    pub onset: ::core::option::Option<us_core_allergy_intolerance::OnsetX>,
    /// Date first version of the resource instance was recorded
    #[prost(message, optional, tag="20")]
    pub recorded_date: ::core::option::Option<super::core::DateTime>,
    /// Who recorded the sensitivity
    #[prost(message, optional, tag="21")]
    pub recorder: ::core::option::Option<super::core::Reference>,
    /// Source of the information about the allergy
    #[prost(message, optional, tag="22")]
    pub asserter: ::core::option::Option<super::core::Reference>,
    /// Date(/time) of last known occurrence of a reaction
    #[prost(message, optional, tag="23")]
    pub last_occurrence: ::core::option::Option<super::core::DateTime>,
    /// Additional text not captured in other fields
    #[prost(message, repeated, tag="24")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    #[prost(message, repeated, tag="25")]
    pub reaction: prost::alloc::vec::Vec<us_core_allergy_intolerance::Reaction>,
}
/// Nested message and enum types in `USCoreAllergyIntolerance`.
pub mod us_core_allergy_intolerance {
    /// allergy | intolerance - Underlying mechanism (if known)
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct TypeCode {
        #[prost(enumeration="super::super::core::allergy_intolerance_type_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// food | medication | environment | biologic
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CategoryCode {
        #[prost(enumeration="super::super::core::allergy_intolerance_category_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// low | high | unable-to-assess
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct CriticalityCode {
        #[prost(enumeration="super::super::core::allergy_intolerance_criticality_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// When allergy or intolerance was identified
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OnsetX {
        #[prost(oneof="onset_x::Choice", tags="1, 2, 3, 4, 5")]
        pub choice: ::core::option::Option<onset_x::Choice>,
    }
    /// Nested message and enum types in `OnsetX`.
    pub mod onset_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Age(super::super::super::core::Age),
            #[prost(message, tag="3")]
            Period(super::super::super::core::Period),
            #[prost(message, tag="4")]
            Range(super::super::super::core::Range),
            #[prost(message, tag="5")]
            StringValue(super::super::super::core::String),
        }
    }
    /// Adverse Reaction Events linked to exposure to substance
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Reaction {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Specific substance or pharmaceutical product considered to be responsible
        /// for event
        #[prost(message, optional, tag="4")]
        pub substance: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Clinical symptoms/signs associated with the Event
        #[prost(message, repeated, tag="5")]
        pub manifestation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Description of the event as a whole
        #[prost(message, optional, tag="6")]
        pub description: ::core::option::Option<super::super::core::String>,
        /// Date(/time) when manifestations showed
        #[prost(message, optional, tag="7")]
        pub onset: ::core::option::Option<super::super::core::DateTime>,
        #[prost(message, optional, tag="8")]
        pub severity: ::core::option::Option<reaction::SeverityCode>,
        /// How the subject was exposed to the substance
        #[prost(message, optional, tag="9")]
        pub exposure_route: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Text about event not captured in other fields
        #[prost(message, repeated, tag="10")]
        pub note: prost::alloc::vec::Vec<super::super::core::Annotation>,
    }
    /// Nested message and enum types in `Reaction`.
    pub mod reaction {
        /// mild | moderate | severe (of event as a whole)
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct SeverityCode {
            #[prost(enumeration="super::super::super::core::allergy_intolerance_severity_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreCarePlanProfile.
/// Healthcare plan for patient or group.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-careplan>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreCarePlanProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// External Ids for this plan
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Instantiates FHIR protocol or definition
    #[prost(message, repeated, tag="11")]
    pub instantiates_canonical: prost::alloc::vec::Vec<super::core::Canonical>,
    /// Instantiates external protocol or definition
    #[prost(message, repeated, tag="12")]
    pub instantiates_uri: prost::alloc::vec::Vec<super::core::Uri>,
    /// Fulfills CarePlan
    #[prost(message, repeated, tag="13")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// CarePlan replaced by this CarePlan
    #[prost(message, repeated, tag="14")]
    pub replaces: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced CarePlan
    #[prost(message, repeated, tag="15")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="16")]
    pub status: ::core::option::Option<us_core_care_plan_profile::StatusCode>,
    #[prost(message, optional, tag="17")]
    pub intent: ::core::option::Option<us_core_care_plan_profile::IntentCode>,
    /// Type of plan
    #[prost(message, repeated, tag="18")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Human-friendly name for the care plan
    #[prost(message, optional, tag="19")]
    pub title: ::core::option::Option<super::core::String>,
    /// Summary of nature of plan
    #[prost(message, optional, tag="20")]
    pub description: ::core::option::Option<super::core::String>,
    /// Who the care plan is for
    #[prost(message, optional, tag="21")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// Encounter created as part of
    #[prost(message, optional, tag="22")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    /// Time period plan covers
    #[prost(message, optional, tag="23")]
    pub period: ::core::option::Option<super::core::Period>,
    /// Date record was first recorded
    #[prost(message, optional, tag="24")]
    pub created: ::core::option::Option<super::core::DateTime>,
    /// Who is the designated responsible party
    #[prost(message, optional, tag="25")]
    pub author: ::core::option::Option<super::core::Reference>,
    /// Who provided the content of the care plan
    #[prost(message, repeated, tag="26")]
    pub contributor: prost::alloc::vec::Vec<super::core::Reference>,
    /// Who's involved in plan?
    #[prost(message, repeated, tag="27")]
    pub care_team: prost::alloc::vec::Vec<super::core::Reference>,
    /// Health issues this plan addresses
    #[prost(message, repeated, tag="28")]
    pub addresses: prost::alloc::vec::Vec<super::core::Reference>,
    /// Information considered as part of plan
    #[prost(message, repeated, tag="29")]
    pub supporting_info: prost::alloc::vec::Vec<super::core::Reference>,
    /// Desired outcome of plan
    #[prost(message, repeated, tag="30")]
    pub goal: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="31")]
    pub activity: prost::alloc::vec::Vec<us_core_care_plan_profile::Activity>,
    /// Comments about the plan
    #[prost(message, repeated, tag="32")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
}
/// Nested message and enum types in `USCoreCarePlanProfile`.
pub mod us_core_care_plan_profile {
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::request_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// proposal | plan | order | option
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct IntentCode {
        #[prost(enumeration="super::super::core::care_plan_intent_value_set::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Action to occur as part of plan
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Activity {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Results of the activity
        #[prost(message, repeated, tag="4")]
        pub outcome_codeable_concept: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Appointment, Encounter, Procedure, etc.
        #[prost(message, repeated, tag="5")]
        pub outcome_reference: prost::alloc::vec::Vec<super::super::core::Reference>,
        /// Comments about the activity status/progress
        #[prost(message, repeated, tag="6")]
        pub progress: prost::alloc::vec::Vec<super::super::core::Annotation>,
        /// Activity details defined in specific resource
        #[prost(message, optional, tag="7")]
        pub reference: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag="8")]
        pub detail: ::core::option::Option<activity::Detail>,
    }
    /// Nested message and enum types in `Activity`.
    pub mod activity {
        /// In-line definition of activity
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct Detail {
            /// Unique id for inter-element referencing
            #[prost(message, optional, tag="1")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            /// Additional content defined by implementations
            #[prost(message, repeated, tag="2")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            /// Extensions that cannot be ignored even if unrecognized
            #[prost(message, repeated, tag="3")]
            pub modifier_extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
            #[prost(message, optional, tag="4")]
            pub kind: ::core::option::Option<detail::KindCode>,
            /// Instantiates FHIR protocol or definition
            #[prost(message, repeated, tag="5")]
            pub instantiates_canonical: prost::alloc::vec::Vec<super::super::super::core::Canonical>,
            /// Instantiates external protocol or definition
            #[prost(message, repeated, tag="6")]
            pub instantiates_uri: prost::alloc::vec::Vec<super::super::super::core::Uri>,
            /// Detail type of activity
            #[prost(message, optional, tag="7")]
            pub code: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// Why activity should be done or why activity was prohibited
            #[prost(message, repeated, tag="8")]
            pub reason_code: prost::alloc::vec::Vec<super::super::super::core::CodeableConcept>,
            /// Why activity is needed
            #[prost(message, repeated, tag="9")]
            pub reason_reference: prost::alloc::vec::Vec<super::super::super::core::Reference>,
            /// Goals this activity relates to
            #[prost(message, repeated, tag="10")]
            pub goal: prost::alloc::vec::Vec<super::super::super::core::Reference>,
            #[prost(message, optional, tag="11")]
            pub status: ::core::option::Option<detail::StatusCode>,
            /// Reason for current status
            #[prost(message, optional, tag="12")]
            pub status_reason: ::core::option::Option<super::super::super::core::CodeableConcept>,
            /// If true, activity is prohibiting action
            #[prost(message, optional, tag="13")]
            pub do_not_perform: ::core::option::Option<super::super::super::core::Boolean>,
            #[prost(message, optional, tag="14")]
            pub scheduled: ::core::option::Option<detail::ScheduledX>,
            /// Where it should happen
            #[prost(message, optional, tag="15")]
            pub location: ::core::option::Option<super::super::super::core::Reference>,
            /// Who will be responsible?
            #[prost(message, repeated, tag="16")]
            pub performer: prost::alloc::vec::Vec<super::super::super::core::Reference>,
            #[prost(message, optional, tag="17")]
            pub product: ::core::option::Option<detail::ProductX>,
            /// How to consume/day?
            #[prost(message, optional, tag="18")]
            pub daily_amount: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            /// How much to administer/supply/consume
            #[prost(message, optional, tag="19")]
            pub quantity: ::core::option::Option<super::super::super::core::SimpleQuantity>,
            /// Extra info describing activity to perform
            #[prost(message, optional, tag="20")]
            pub description: ::core::option::Option<super::super::super::core::String>,
        }
        /// Nested message and enum types in `Detail`.
        pub mod detail {
            /// Appointment | CommunicationRequest | DeviceRequest | MedicationRequest
            /// | NutritionOrder | Task | ServiceRequest | VisionPrescription
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct KindCode {
                #[prost(enumeration="super::super::super::super::core::care_plan_activity_kind_value_set::Value", tag="1")]
                pub value: i32,
                #[prost(message, optional, tag="2")]
                pub id: ::core::option::Option<super::super::super::super::core::String>,
                #[prost(message, repeated, tag="3")]
                pub extension: prost::alloc::vec::Vec<super::super::super::super::core::Extension>,
            }
            /// not-started | scheduled | in-progress | on-hold | completed | cancelled
            /// | stopped | unknown | entered-in-error
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct StatusCode {
                #[prost(enumeration="super::super::super::super::core::care_plan_activity_status_code::Value", tag="1")]
                pub value: i32,
                #[prost(message, optional, tag="2")]
                pub id: ::core::option::Option<super::super::super::super::core::String>,
                #[prost(message, repeated, tag="3")]
                pub extension: prost::alloc::vec::Vec<super::super::super::super::core::Extension>,
            }
            /// When activity is to occur
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct ScheduledX {
                #[prost(oneof="scheduled_x::Choice", tags="1, 2, 3")]
                pub choice: ::core::option::Option<scheduled_x::Choice>,
            }
            /// Nested message and enum types in `ScheduledX`.
            pub mod scheduled_x {
                #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
                #[derive(Clone, PartialEq, prost::Oneof)]
                pub enum Choice {
                    #[prost(message, tag="1")]
                    Timing(super::super::super::super::super::core::Timing),
                    #[prost(message, tag="2")]
                    Period(super::super::super::super::super::core::Period),
                    #[prost(message, tag="3")]
                    StringValue(super::super::super::super::super::core::String),
                }
            }
            /// What is to be administered/supplied
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Message)]
            pub struct ProductX {
                #[prost(oneof="product_x::Choice", tags="1, 2")]
                pub choice: ::core::option::Option<product_x::Choice>,
            }
            /// Nested message and enum types in `ProductX`.
            pub mod product_x {
                #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
                #[derive(Clone, PartialEq, prost::Oneof)]
                pub enum Choice {
                    #[prost(message, tag="1")]
                    CodeableConcept(super::super::super::super::super::core::CodeableConcept),
                    #[prost(message, tag="2")]
                    Reference(super::super::super::super::super::core::Reference),
                }
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCorePatientProfile.
/// Information about an individual or animal receiving health care services.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCorePatientProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Extension
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// An identifier for this patient
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether this patient's record is in active use
    #[prost(message, optional, tag="11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// A name associated with the patient
    #[prost(message, repeated, tag="12")]
    pub name: prost::alloc::vec::Vec<super::core::HumanName>,
    /// A contact detail for the individual
    #[prost(message, repeated, tag="13")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    #[prost(message, optional, tag="14")]
    pub gender: ::core::option::Option<us_core_patient_profile::GenderCode>,
    /// The date of birth for the individual
    #[prost(message, optional, tag="15")]
    pub birth_date: ::core::option::Option<super::core::Date>,
    #[prost(message, optional, tag="16")]
    pub deceased: ::core::option::Option<us_core_patient_profile::DeceasedX>,
    /// An address for the individual
    #[prost(message, repeated, tag="17")]
    pub address: prost::alloc::vec::Vec<super::core::Address>,
    /// Marital (civil) status of a patient
    #[prost(message, optional, tag="18")]
    pub marital_status: ::core::option::Option<super::core::CodeableConcept>,
    #[prost(message, optional, tag="19")]
    pub multiple_birth: ::core::option::Option<us_core_patient_profile::MultipleBirthX>,
    /// Image of the patient
    #[prost(message, repeated, tag="20")]
    pub photo: prost::alloc::vec::Vec<super::core::Attachment>,
    #[prost(message, repeated, tag="21")]
    pub contact: prost::alloc::vec::Vec<us_core_patient_profile::Contact>,
    #[prost(message, repeated, tag="22")]
    pub communication: prost::alloc::vec::Vec<us_core_patient_profile::Communication>,
    /// Patient's nominated primary care provider
    #[prost(message, repeated, tag="23")]
    pub general_practitioner: prost::alloc::vec::Vec<super::core::Reference>,
    /// Organization that is the custodian of the patient record
    #[prost(message, optional, tag="24")]
    pub managing_organization: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="25")]
    pub link: prost::alloc::vec::Vec<us_core_patient_profile::Link>,
    /// US Core Race Extension
    #[prost(message, optional, tag="26")]
    pub race: ::core::option::Option<PatientUsCoreRaceExtension>,
    /// US Core ethnicity Extension
    #[prost(message, optional, tag="27")]
    pub ethnicity: ::core::option::Option<PatientUsCoreEthnicityExtension>,
    /// Extension
    #[prost(message, optional, tag="28")]
    pub birthsex: ::core::option::Option<patient_us_core_birth_sex_extension::ValueCode>,
}
/// Nested message and enum types in `USCorePatientProfile`.
pub mod us_core_patient_profile {
    /// male | female | other | unknown
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct GenderCode {
        #[prost(enumeration="super::super::core::administrative_gender_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Indicates if the individual is deceased or not
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DeceasedX {
        #[prost(oneof="deceased_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<deceased_x::Choice>,
    }
    /// Nested message and enum types in `DeceasedX`.
    pub mod deceased_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag="2")]
            DateTime(super::super::super::core::DateTime),
        }
    }
    /// Whether patient is part of a multiple birth
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct MultipleBirthX {
        #[prost(oneof="multiple_birth_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<multiple_birth_x::Choice>,
    }
    /// Nested message and enum types in `MultipleBirthX`.
    pub mod multiple_birth_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Boolean(super::super::super::core::Boolean),
            #[prost(message, tag="2")]
            Integer(super::super::super::core::Integer),
        }
    }
    /// A contact party (e.g. guardian, partner, friend) for the patient
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Contact {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The kind of relationship
        #[prost(message, repeated, tag="4")]
        pub relationship: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// A name associated with the contact person
        #[prost(message, optional, tag="5")]
        pub name: ::core::option::Option<super::super::core::HumanName>,
        /// A contact detail for the person
        #[prost(message, repeated, tag="6")]
        pub telecom: prost::alloc::vec::Vec<super::super::core::ContactPoint>,
        /// Address for the contact person
        #[prost(message, optional, tag="7")]
        pub address: ::core::option::Option<super::super::core::Address>,
        #[prost(message, optional, tag="8")]
        pub gender: ::core::option::Option<contact::GenderCode>,
        /// Organization that is associated with the contact
        #[prost(message, optional, tag="9")]
        pub organization: ::core::option::Option<super::super::core::Reference>,
        /// The period during which this contact person or organization is valid to
        /// be contacted relating to this patient
        #[prost(message, optional, tag="10")]
        pub period: ::core::option::Option<super::super::core::Period>,
    }
    /// Nested message and enum types in `Contact`.
    pub mod contact {
        /// male | female | other | unknown
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct GenderCode {
            #[prost(enumeration="super::super::super::core::administrative_gender_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// A language which may be used to communicate with the patient about his or
    /// her health
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Communication {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The language which can be used to communicate with the patient about his
        /// or her health
        #[prost(message, optional, tag="4")]
        pub language: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Language preference indicator
        #[prost(message, optional, tag="5")]
        pub preferred: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Link to another patient resource that concerns the same actual person
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Link {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The other patient or related person resource that the link refers to
        #[prost(message, optional, tag="4")]
        pub other: ::core::option::Option<super::super::core::Reference>,
        #[prost(message, optional, tag="5")]
        pub r#type: ::core::option::Option<link::TypeCode>,
    }
    /// Nested message and enum types in `Link`.
    pub mod link {
        /// replaced-by | replaces | refer | seealso
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct TypeCode {
            #[prost(enumeration="super::super::super::core::link_type_code::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreEthnicityExtension.
/// US Core ethnicity Extension.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct PatientUsCoreEthnicityExtension {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    #[prost(message, optional, tag="4")]
    pub omb_category: ::core::option::Option<patient_us_core_ethnicity_extension::OmbCategoryCoding>,
    #[prost(message, repeated, tag="5")]
    pub detailed: prost::alloc::vec::Vec<patient_us_core_ethnicity_extension::DetailedCoding>,
    /// ethnicity Text
    #[prost(message, optional, tag="6")]
    pub text: ::core::option::Option<super::core::String>,
}
/// Nested message and enum types in `PatientUSCoreEthnicityExtension`.
pub mod patient_us_core_ethnicity_extension {
    /// Hispanic or Latino|Not Hispanic or Latino
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct OmbCategoryCoding {
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub version: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="5")]
        pub code: ::core::option::Option<omb_category_coding::BoundCode>,
        #[prost(message, optional, tag="6")]
        pub display: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="7")]
        pub user_selected: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Nested message and enum types in `OmbCategoryCoding`.
    pub mod omb_category_coding {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct BoundCode {
            #[prost(enumeration="super::super::omb_ethnicity_categories_value_set::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
    /// Extended ethnicity codes
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct DetailedCoding {
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        #[prost(message, optional, tag="4")]
        pub version: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="5")]
        pub code: ::core::option::Option<detailed_coding::BoundCode>,
        #[prost(message, optional, tag="6")]
        pub display: ::core::option::Option<super::super::core::String>,
        #[prost(message, optional, tag="7")]
        pub user_selected: ::core::option::Option<super::super::core::Boolean>,
    }
    /// Nested message and enum types in `DetailedCoding`.
    pub mod detailed_coding {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct BoundCode {
            #[prost(enumeration="super::super::detailed_ethnicity_value_set::Value", tag="1")]
            pub value: i32,
            #[prost(message, optional, tag="2")]
            pub id: ::core::option::Option<super::super::super::core::String>,
            #[prost(message, repeated, tag="3")]
            pub extension: prost::alloc::vec::Vec<super::super::super::core::Extension>,
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreDirectEmailExtension.
/// Email is a "direct" email.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-direct>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreDirectEmail {
    /// Unique id for inter-element referencing
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::String>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="2")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Value of extension
    #[prost(message, optional, tag="3")]
    pub value_boolean: ::core::option::Option<super::core::Boolean>,
}
/// Auto-generated from StructureDefinition for USCoreSmokingStatusProfile.
/// Measurements and simple assertions.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-smokingstatus>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreSmokingStatusProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag="11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<us_core_smoking_status_profile::StatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag="14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Smoking Status
    #[prost(message, optional, tag="15")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who and/or what the observation is about
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// What the observation is about, when it is not about the subject of record
    #[prost(message, repeated, tag="17")]
    pub focus: prost::alloc::vec::Vec<super::core::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag="18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="19")]
    pub effective: ::core::option::Option<us_core_smoking_status_profile::EffectiveX>,
    /// Date/Time this version was made available
    #[prost(message, optional, tag="20")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag="21")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="22")]
    pub value: ::core::option::Option<us_core_smoking_status_profile::ValueX>,
    /// Why the result is missing
    #[prost(message, optional, tag="23")]
    pub data_absent_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, repeated, tag="24")]
    pub interpretation: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments about the observation
    #[prost(message, repeated, tag="25")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Observed body part
    #[prost(message, optional, tag="26")]
    pub body_site: ::core::option::Option<super::core::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag="27")]
    pub method: ::core::option::Option<super::core::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag="28")]
    pub specimen: ::core::option::Option<super::core::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag="29")]
    pub device: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="30")]
    pub reference_range: prost::alloc::vec::Vec<us_core_smoking_status_profile::ReferenceRange>,
    /// Related resource that belongs to the Observation group
    #[prost(message, repeated, tag="31")]
    pub has_member: prost::alloc::vec::Vec<super::core::Reference>,
    /// Related measurements the observation is made from
    #[prost(message, repeated, tag="32")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="33")]
    pub component: prost::alloc::vec::Vec<us_core_smoking_status_profile::Component>,
}
/// Nested message and enum types in `USCoreSmokingStatusProfile`.
pub mod us_core_smoking_status_profile {
    /// registered | preliminary | final | amended +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::us_core_observation_smoking_status_status_value_set::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Clinically relevant time/time-period for observation
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof="effective_x::Choice", tags="1, 2, 3, 4")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
            #[prost(message, tag="3")]
            Timing(super::super::super::core::Timing),
            #[prost(message, tag="4")]
            Instant(super::super::super::core::Instant),
        }
    }
    /// Actual result
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof="value_x::Choice", tags="2")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="2")]
            CodeableConcept(super::super::super::core::CodeableConcept),
        }
    }
    /// Provides guide for interpretation
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReferenceRange {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag="4")]
        pub low: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag="5")]
        pub high: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag="7")]
        pub applies_to: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag="8")]
        pub age: ::core::option::Option<super::super::core::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag="9")]
        pub text: ::core::option::Option<super::super::core::String>,
    }
    /// Component results
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Component {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub value: ::core::option::Option<component::ValueX>,
        /// Why the component result is missing
        #[prost(message, optional, tag="6")]
        pub data_absent_reason: ::core::option::Option<super::super::core::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, repeated, tag="7")]
        pub interpretation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag="8")]
        pub reference_range: prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Actual component result
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof="value_x::Choice", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag="3")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag="4")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="5")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag="6")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag="7")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag="8")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag="9")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag="10")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag="11")]
                Period(super::super::super::super::core::Period),
            }
        }
    }
}
/// Auto-generated from StructureDefinition for USCoreOrganizationProfile.
/// A grouping of people or organizations with a common purpose.
/// See <http://hl7.org/fhir/us/core/StructureDefinition/us-core-organization>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCoreOrganizationProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Identifies this organization  across multiple systems
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Whether the organization's record is still in active use
    #[prost(message, optional, tag="11")]
    pub active: ::core::option::Option<super::core::Boolean>,
    /// Kind of organization
    #[prost(message, repeated, tag="12")]
    pub r#type: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Name used for the organization
    #[prost(message, optional, tag="13")]
    pub name: ::core::option::Option<super::core::String>,
    /// A list of alternate names that the organization is known as, or was known
    /// as in the past
    #[prost(message, repeated, tag="14")]
    pub alias: prost::alloc::vec::Vec<super::core::String>,
    /// A contact detail for the organization
    #[prost(message, repeated, tag="15")]
    pub telecom: prost::alloc::vec::Vec<super::core::ContactPoint>,
    /// An address for the organization
    #[prost(message, repeated, tag="16")]
    pub address: prost::alloc::vec::Vec<super::core::Address>,
    /// The organization of which this organization forms a part
    #[prost(message, optional, tag="17")]
    pub part_of: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="18")]
    pub contact: prost::alloc::vec::Vec<us_core_organization_profile::Contact>,
    /// Technical endpoints providing access to services operated for the
    /// organization
    #[prost(message, repeated, tag="19")]
    pub endpoint: prost::alloc::vec::Vec<super::core::Reference>,
}
/// Nested message and enum types in `USCoreOrganizationProfile`.
pub mod us_core_organization_profile {
    /// Contact for the organization for a certain purpose
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Contact {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// The type of contact
        #[prost(message, optional, tag="4")]
        pub purpose: ::core::option::Option<super::super::core::CodeableConcept>,
        /// A name associated with the contact
        #[prost(message, optional, tag="5")]
        pub name: ::core::option::Option<super::super::core::HumanName>,
        /// Contact details (telephone, email, etc.)  for a contact
        #[prost(message, repeated, tag="6")]
        pub telecom: prost::alloc::vec::Vec<super::super::core::ContactPoint>,
        /// Visiting or postal addresses for the contact
        #[prost(message, optional, tag="7")]
        pub address: ::core::option::Option<super::super::core::Address>,
    }
}
/// Auto-generated from StructureDefinition for
/// USCorePediatricBMIforAgeObservationProfile. FHIR Vital Signs Profile. See
/// <http://hl7.org/fhir/us/core/StructureDefinition/pediatric-bmi-for-age>
#[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, prost::Message)]
pub struct UsCorePediatricBmIforAgeObservationProfile {
    /// Logical id of this artifact
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::core::Id>,
    /// Metadata about the resource
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<super::core::Meta>,
    /// A set of rules under which this content was created
    #[prost(message, optional, tag="3")]
    pub implicit_rules: ::core::option::Option<super::core::Uri>,
    /// Language of the resource content
    #[prost(message, optional, tag="4")]
    pub language: ::core::option::Option<super::core::Code>,
    /// Text summary of the resource, for human interpretation
    #[prost(message, optional, tag="5")]
    pub text: ::core::option::Option<super::core::Narrative>,
    /// Contained, inline Resources
    #[prost(message, repeated, tag="6")]
    pub contained: prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// Additional content defined by implementations
    #[prost(message, repeated, tag="8")]
    pub extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Extensions that cannot be ignored
    #[prost(message, repeated, tag="9")]
    pub modifier_extension: prost::alloc::vec::Vec<super::core::Extension>,
    /// Business Identifier for observation
    #[prost(message, repeated, tag="10")]
    pub identifier: prost::alloc::vec::Vec<super::core::Identifier>,
    /// Fulfills plan, proposal or order
    #[prost(message, repeated, tag="11")]
    pub based_on: prost::alloc::vec::Vec<super::core::Reference>,
    /// Part of referenced event
    #[prost(message, repeated, tag="12")]
    pub part_of: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="13")]
    pub status: ::core::option::Option<us_core_pediatric_bm_ifor_age_observation_profile::StatusCode>,
    /// Classification of  type of observation
    #[prost(message, repeated, tag="14")]
    pub category: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// BMI percentile per age and sex for youth 2-20
    #[prost(message, optional, tag="15")]
    pub code: ::core::option::Option<super::core::CodeableConcept>,
    /// Who and/or what the observation is about
    #[prost(message, optional, tag="16")]
    pub subject: ::core::option::Option<super::core::Reference>,
    /// What the observation is about, when it is not about the subject of record
    #[prost(message, repeated, tag="17")]
    pub focus: prost::alloc::vec::Vec<super::core::Reference>,
    /// Healthcare event during which this observation is made
    #[prost(message, optional, tag="18")]
    pub encounter: ::core::option::Option<super::core::Reference>,
    #[prost(message, optional, tag="19")]
    pub effective: ::core::option::Option<us_core_pediatric_bm_ifor_age_observation_profile::EffectiveX>,
    /// Date/Time this version was made available
    #[prost(message, optional, tag="20")]
    pub issued: ::core::option::Option<super::core::Instant>,
    /// Who is responsible for the observation
    #[prost(message, repeated, tag="21")]
    pub performer: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, optional, tag="22")]
    pub value: ::core::option::Option<us_core_pediatric_bm_ifor_age_observation_profile::ValueX>,
    /// Why the result is missing
    #[prost(message, optional, tag="23")]
    pub data_absent_reason: ::core::option::Option<super::core::CodeableConcept>,
    /// High, low, normal, etc.
    #[prost(message, repeated, tag="24")]
    pub interpretation: prost::alloc::vec::Vec<super::core::CodeableConcept>,
    /// Comments about the observation
    #[prost(message, repeated, tag="25")]
    pub note: prost::alloc::vec::Vec<super::core::Annotation>,
    /// Observed body part
    #[prost(message, optional, tag="26")]
    pub body_site: ::core::option::Option<super::core::CodeableConcept>,
    /// How it was done
    #[prost(message, optional, tag="27")]
    pub method: ::core::option::Option<super::core::CodeableConcept>,
    /// Specimen used for this observation
    #[prost(message, optional, tag="28")]
    pub specimen: ::core::option::Option<super::core::Reference>,
    /// (Measurement) Device
    #[prost(message, optional, tag="29")]
    pub device: ::core::option::Option<super::core::Reference>,
    #[prost(message, repeated, tag="30")]
    pub reference_range: prost::alloc::vec::Vec<us_core_pediatric_bm_ifor_age_observation_profile::ReferenceRange>,
    /// Used when reporting vital signs panel components
    #[prost(message, repeated, tag="31")]
    pub has_member: prost::alloc::vec::Vec<super::core::Reference>,
    /// Related measurements the observation is made from
    #[prost(message, repeated, tag="32")]
    pub derived_from: prost::alloc::vec::Vec<super::core::Reference>,
    #[prost(message, repeated, tag="33")]
    pub component: prost::alloc::vec::Vec<us_core_pediatric_bm_ifor_age_observation_profile::Component>,
}
/// Nested message and enum types in `USCorePediatricBMIforAgeObservationProfile`.
pub mod us_core_pediatric_bm_ifor_age_observation_profile {
    /// registered | preliminary | final | amended +
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct StatusCode {
        #[prost(enumeration="super::super::core::observation_status_code::Value", tag="1")]
        pub value: i32,
        #[prost(message, optional, tag="2")]
        pub id: ::core::option::Option<super::super::core::String>,
        #[prost(message, repeated, tag="3")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
    }
    /// Often just a dateTime for Vital Signs
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EffectiveX {
        #[prost(oneof="effective_x::Choice", tags="1, 2")]
        pub choice: ::core::option::Option<effective_x::Choice>,
    }
    /// Nested message and enum types in `EffectiveX`.
    pub mod effective_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            DateTime(super::super::super::core::DateTime),
            #[prost(message, tag="2")]
            Period(super::super::super::core::Period),
        }
    }
    /// Vital Signs value are recorded using the Quantity data type. For supporting
    /// observations such as Cuff size could use other datatypes such as
    /// CodeableConcept.
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ValueX {
        #[prost(oneof="value_x::Choice", tags="1")]
        pub choice: ::core::option::Option<value_x::Choice>,
    }
    /// Nested message and enum types in `ValueX`.
    pub mod value_x {
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Oneof)]
        pub enum Choice {
            #[prost(message, tag="1")]
            Quantity(super::super::super::core::Quantity),
        }
    }
    /// Provides guide for interpretation
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct ReferenceRange {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Low Range, if relevant
        #[prost(message, optional, tag="4")]
        pub low: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// High Range, if relevant
        #[prost(message, optional, tag="5")]
        pub high: ::core::option::Option<super::super::core::SimpleQuantity>,
        /// Reference range qualifier
        #[prost(message, optional, tag="6")]
        pub r#type: ::core::option::Option<super::super::core::CodeableConcept>,
        /// Reference range population
        #[prost(message, repeated, tag="7")]
        pub applies_to: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Applicable age range, if relevant
        #[prost(message, optional, tag="8")]
        pub age: ::core::option::Option<super::super::core::Range>,
        /// Text based reference range in an observation
        #[prost(message, optional, tag="9")]
        pub text: ::core::option::Option<super::super::core::String>,
    }
    /// Used when reporting systolic and diastolic blood pressure.
    #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Component {
        /// Unique id for inter-element referencing
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::super::core::String>,
        /// Additional content defined by implementations
        #[prost(message, repeated, tag="2")]
        pub extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Extensions that cannot be ignored even if unrecognized
        #[prost(message, repeated, tag="3")]
        pub modifier_extension: prost::alloc::vec::Vec<super::super::core::Extension>,
        /// Type of component observation (code / type)
        #[prost(message, optional, tag="4")]
        pub code: ::core::option::Option<super::super::core::CodeableConcept>,
        #[prost(message, optional, tag="5")]
        pub value: ::core::option::Option<component::ValueX>,
        /// Why the component result is missing
        #[prost(message, optional, tag="6")]
        pub data_absent_reason: ::core::option::Option<super::super::core::CodeableConcept>,
        /// High, low, normal, etc.
        #[prost(message, repeated, tag="7")]
        pub interpretation: prost::alloc::vec::Vec<super::super::core::CodeableConcept>,
        /// Provides guide for interpretation of component result
        #[prost(message, repeated, tag="8")]
        pub reference_range: prost::alloc::vec::Vec<ReferenceRange>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        /// Vital Sign Value recorded with UCUM
        #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ValueX {
            #[prost(oneof="value_x::Choice", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
            pub choice: ::core::option::Option<value_x::Choice>,
        }
        /// Nested message and enum types in `ValueX`.
        pub mod value_x {
            #[derive(Serialize,Deserialize)] #[serde(rename_all = "camelCase")]
            #[derive(Clone, PartialEq, prost::Oneof)]
            pub enum Choice {
                #[prost(message, tag="1")]
                Quantity(super::super::super::super::core::Quantity),
                #[prost(message, tag="2")]
                CodeableConcept(super::super::super::super::core::CodeableConcept),
                #[prost(message, tag="3")]
                StringValue(super::super::super::super::core::String),
                #[prost(message, tag="4")]
                Boolean(super::super::super::super::core::Boolean),
                #[prost(message, tag="5")]
                Integer(super::super::super::super::core::Integer),
                #[prost(message, tag="6")]
                Range(super::super::super::super::core::Range),
                #[prost(message, tag="7")]
                Ratio(super::super::super::super::core::Ratio),
                #[prost(message, tag="8")]
                SampledData(super::super::super::super::core::SampledData),
                #[prost(message, tag="9")]
                Time(super::super::super::super::core::Time),
                #[prost(message, tag="10")]
                DateTime(super::super::super::super::core::DateTime),
                #[prost(message, tag="11")]
                Period(super::super::super::super::core::Period),
            }
        }
    }
}
