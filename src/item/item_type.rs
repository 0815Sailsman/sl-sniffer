#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ItemType {
    //Armor
    CatarinaHelm,
    CatarinaArmor,
    CatarinaGauntlets,
    CatarinaLeggings,
    PaladinHelm,
    PaladinArmor,
    PaladinGauntlets,
    PaladinLeggings,
    DarkMask,
    DarkArmor,
    DarkGauntlets,
    DarkLeggings,
    BrigandHood,
    BrigandArmor,
    BrigandGauntlets,
    BrigandTrousers,
    ShadowMask,
    ShadowGarb,
    ShadowGauntlets,
    ShadowLeggings,
    BlackIronHelm,
    BlackIronArmor,
    BlackIronGauntlets,
    BlackIronLeggings,
    SmoughsHelm,
    SmoughsArmor,
    SmoughsGauntlets,
    SmoughsLeggings,
    SixEyedHelmoftheChannelers,
    RobeoftheChannelers,
    GauntletsoftheChannelers,
    WaistclothoftheChannelers,
    HelmofFavor,
    EmbracedArmorofFavor,
    GauntletsofFavor,
    LeggingsofFavor,
    HelmoftheWise,
    ArmoroftheGlorious,
    GauntletsoftheVanquisher,
    BootsoftheExplorer,
    StoneHelm,
    StoneArmor,
    StoneGauntlets,
    StoneLeggings,
    CrystallineHelm,
    CrystallineArmor,
    CrystallineGauntlets,
    CrystallineLeggings,
    MaskoftheSealer,
    CrimsonRobe,
    CrimsonGloves,
    CrimsonWaistcloth,
    MaskofVelka,
    BlackClericRobe,
    BlackManchette,
    BlackTights,
    IronHelm,
    ArmoroftheSun,
    IronBracelet,
    IronLeggings,
    ChainHelm,
    ChainArmor,
    LeatherGauntlets,
    ChainLeggings,
    ClericHelm,
    ClericArmor,
    ClericGauntlets,
    ClericLeggings,
    SunlightMaggot,
    HelmofThorns,
    ArmorofThorns,
    GauntletsofThorns,
    LeggingsofThorns,
    StandardHelm,
    HardLeatherArmor,
    HardLeatherGauntlets,
    HardLeatherBoots,
    SorcererHat,
    SorcererCloak,
    SorcererGauntlets,
    SorcererBoots,
    TatteredClothHood,
    TatteredClothRobe,
    TatteredClothManchette,
    HeavyBoots,
    PharissHat,
    LeatherArmor,
    LeatherGloves,
    LeatherBoots,
    PaintingGuardianHood,
    PaintingGuardianRobe,
    PaintingGuardianGloves,
    PaintingGuardianWaistcloth,
    OrnsteinsHelm,
    OrnsteinsArmor,
    OrnsteinsGauntlets,
    OrnsteinsLeggings,
    EasternHelm,
    EasternArmor,
    EasternGauntlets,
    EasternLeggings,
    XanthousCrown,
    XanthousOvercoat,
    XanthousGloves,
    XanthousWaistcloth,
    ThiefMask,
    BlackLeatherArmor,
    BlackLeatherGloves,
    BlackLeatherBoots,
    PriestsHat,
    HolyRobe,
    TravelingGlovesHoly,
    HolyTrousers,
    BlackKnightHelm,
    BlackKnightArmor,
    BlackKnightGauntlets,
    BlackKnightLeggings,
    CrownofDusk,
    AntiquatedDress,
    AntiquatedGloves,
    AntiquatedSkirt,
    WitchHat,
    WitchCloak,
    WitchGloves,
    WitchSkirt,
    EliteKnightHelm,
    EliteKnightArmor,
    EliteKnightGauntlets,
    EliteKnightLeggings,
    WandererHood,
    WandererCoat,
    WandererManchette,
    WandererBoots,
    BigHat,
    SageRobe,
    TravelingGlovesSage,
    TravelingBoots,
    KnightHelm,
    KnightArmor,
    KnightGauntlets,
    KnightLeggings,
    DingyHood,
    DingyRobe,
    DingyGloves,
    BloodStainedSkirt,
    MaidenHood,
    MaidenRobe,
    MaidenGloves,
    MaidenSkirt,
    SilverKnightHelm,
    SilverKnightArmor,
    SilverKnightGauntlets,
    SilverKnightLeggings,
    HavelsHelm,
    HavelsArmor,
    HavelsGauntlets,
    HavelsLeggings,
    BrassHelm,
    BrassArmor,
    BrassGauntlets,
    BrassLeggings,
    GoldHemmedBlackHood,
    GoldHemmedBlackCloak,
    GoldHemmedBlackGloves,
    GoldHemmedBlackSkirt,
    GolemHelm,
    GolemArmor,
    GolemGauntlets,
    GolemLeggings,
    HollowSoldierHelm,
    HollowSoldierArmor,
    HollowSoldierWaistcloth,
    SteelHelm,
    SteelArmor,
    SteelGauntlets,
    SteelLeggings,
    HollowThiefsHood,
    HollowThiefsLeatherArmor,
    HollowThiefsTights,
    BalderHelm,
    BalderArmor,
    BalderGauntlets,
    BalderLeggings,
    HollowWarriorHelm,
    HollowWarriorArmor,
    HollowWarriorWaistcloth,
    GiantHelm,
    GiantArmor,
    GiantGauntlets,
    GiantLeggings,
    CrownoftheDarkSun,
    MoonlightRobe,
    MoonlightGloves,
    MoonlightWaistcloth,
    CrownoftheGreatLord,
    RobeoftheGreatLord,
    BraceletoftheGreatLord,
    AnkletoftheGreatLord,
    Sack,
    SymbolofAvarice,
    RoyalHelm,
    MaskoftheFather,
    MaskoftheMother,
    MaskoftheChild,
    FangBoarHelm,
    GargoyleHelm,
    BlackSorcererHat,
    BlackSorcererCloak,
    BlackSorcererGauntlets,
    BlackSorcererBoots,
    HelmofArtorias,
    ArmorofArtorias,
    GauntletsofArtorias,
    LeggingsofArtorias,
    PorcelainMask,
    LordsBladeRobe,
    LordsBladeGloves,
    LordsBladeWaistcloth,
    GoughsHelm,
    GoughsArmor,
    GoughsGauntlets,
    GoughsLeggings,
    GuardianHelm,
    GuardianArmor,
    GuardianGauntlets,
    GuardianLeggings,
    SnickeringTopHat,
    ChestersLongCoat,
    ChestersGloves,
    ChestersTrousers,
    BloatedHead,
    BloatedSorcererHead,

    //Consumables
    EyeofDeath,
    CrackedRedEyeOrb,
    EstusFlask,
    ElizabethsMushroom,
    DivineBlessing,
    GreenBlossom,
    BloodredMossClump,
    PurpleMossClump,
    BloomingPurpleMossClump,
    PurgingStone,
    EggVermifuge,
    RepairPowder,
    ThrowingKnife,
    PoisonThrowingKnife,
    Firebomb,
    DungPie,
    AlluringSkull,
    LloydsTalisman,
    BlackFirebomb,
    CharcoalPineResin,
    GoldPineResin,
    TransientCurse,
    RottenPineResin,
    HomewardBone,
    PrismStone,
    Indictment,
    SouvenirofReprisal,
    SunlightMedal,
    Pendant,
    Rubbish,
    CopperCoin,
    SilverCoin,
    GoldCoin,
    FireKeeperSoulAnastaciaofAstora,
    FireKeeperSoulDarkmoonKnightess,
    FireKeeperSoulDaughterofChaos,
    FireKeeperSoulNewLondo,
    FireKeeperSoulBlighttown,
    FireKeeperSoulDukesArchives,
    FireKeeperSoulUndeadParish,
    SoulofaLostUndead,
    LargeSoulofaLostUndead,
    SoulofaNamelessSoldier,
    LargeSoulofaNamelessSoldier,
    SoulofaProudKnight,
    LargeSoulofaProudKnight,
    SoulofaBraveWarrior,
    LargeSoulofaBraveWarrior,
    SoulofaHero,
    SoulofaGreatHero,
    Humanity,
    TwinHumanities,
    SoulofQuelaag,
    SoulofSif,
    SoulofGwynLordofCinder,
    CoreofanIronGolem,
    SoulofOrnstein,
    SouloftheMoonlightButterfly,
    SoulofSmough,
    SoulofPriscilla,
    SoulofGwyndolin,
    GuardianSoul,
    SoulofArtorias,
    SoulofManus,

    //Key
    PeculiarDoll,
    BasementKey,
    CrestofArtorias,
    CageKey,
    ArchiveTowerCellKey,
    ArchiveTowerGiantDoorKey,
    ArchiveTowerGiantCellKey,
    BlighttownKey,
    KeytoNewLondoRuins,
    AnnexKey,
    DungeonCellKey,
    BigPilgrimsKey,
    UndeadAsylumF2EastKey,
    KeytotheSeal,
    KeytoDepths,
    UndeadAsylumF2WestKey,
    MysteryKey,
    SewerChamberKey,
    WatchtowerBasementKey,
    ArchivePrisonExtraKey,
    ResidenceKey,
    CrestKey,
    MasterKey,
    LordSoulNito,
    LordSoulBedofChaos,
    BequeathedLordSoulShardFourKings,
    BequeathedLordSoulShardSeath,
    Lordvessel,
    BrokenPendant,
    WeaponSmithbox,
    ArmorSmithbox,
    Repairbox,
    RiteofKindling,
    BottomlessBox,

    //melee
    Dagger,
    ParryingDagger,
    GhostBlade,
    BanditsKnife,
    PriscillasDagger,
    Shortsword,
    Longsword,
    Broadsword,
    BrokenStraightSword,
    BalderSideSword,
    CrystalStraightSword,
    SunlightStraightSword,
    BarbedStraightSword,
    SilverKnightStraightSword,
    AstorasStraightSword,
    Darksword,
    DrakeSword,
    StraightSwordHilt,
    BastardSword,
    Claymore,
    ManserpentGreatsword,
    Flamberge,
    CrystalGreatsword,
    StoneGreatsword,
    GreatswordofArtorias,
    MoonlightGreatsword,
    BlackKnightSword,
    GreatswordofArtoriasCursed,
    GreatLordGreatsword,
    Zweihander,
    Greatsword,
    DemonGreatMachete,
    DragonGreatsword,
    BlackKnightGreatsword,
    Scimitar,
    Falchion,
    Shotel,
    JaggedGhostBlade,
    PaintingGuardianSword,
    QuelaagsFurysword,
    Server,
    Murakumo,
    GravelordSword,
    Uchigatana,
    WashingPole,
    Iaito,
    ChaosBlade,
    MailBreaker,
    Rapier,
    Estoc,
    VelkasRapier,
    RicardsRapier,
    HandAxe,
    BattleAxe,
    CrescentAxe,
    ButcherKnife,
    GolemAxe,
    GargoyleTailAxe,
    Greataxe,
    DemonsGreataxe,
    DragonKingGreataxe,
    BlackKnightGreataxe,
    Club,
    Mace,
    MorningStar,
    Warpick,
    Pickaxe,
    ReinforcedClub,
    BlacksmithHammer,
    BlacksmithGiantHammer,
    HammerofVamos,
    GreatClub,
    Grant,
    DemonsGreatHammer,
    DragonTooth,
    LargeClub,
    SmoughsHammer,
    Caestus,
    Claw,
    DragonBoneFist,
    DarkHand,
    Spear,
    WingedSpear,
    Partizan,
    DemonsSpear,
    ChannelersTrident,
    SilverKnightSpear,
    Pike,
    DragonslayerSpear,
    MoonlightButterflyHorn,
    Halberd,
    GiantsHalberd,
    TitaniteCatchPole,
    GargoylesHalberd,
    BlackKnightHalberd,
    Lucerne,
    Scythe,
    GreatScythe,
    LifehuntScythe,
    Whip,
    NotchedWhip,
    GoldTracer,
    DarkSilverTracer,
    AbyssGreatsword,
    StoneGreataxe,
    FourprongedPlow,
    GuardianTail,
    ObsidianGreatsword,

    //ranged
    ShortBow,
    Longbow,
    BlackBowofPharis,
    DragonslayerGreatbow,
    CompositeBow,
    DarkmoonBow,
    LightCrossbow,
    HeavyCrossbow,
    Avelyn,
    SniperCrossbow,
    GoughsGreatbow,

    //ammo
    StandardArrow,
    LargeArrow,
    FeatherArrow,
    FireArrow,
    PoisonArrow,
    MoonlightArrow,
    WoodenArrow,
    DragonslayerArrow,
    GoughsGreatArrow,
    StandardBolt,
    HeavyBolt,
    SniperBolt,
    WoodBolt,
    LightningBolt,

    //rings
    HavelsRing,
    RedTearstoneRing,
    DarkmoonBladeCovenantRing,
    CatCovenantRing,
    CloranthyRing,
    FlameStoneplateRing,
    ThunderStoneplateRing,
    SpellStoneplateRing,
    SpeckledStoneplateRing,
    BloodbiteRing,
    PoisonbiteRing,
    TinyBeingsRing,
    CursebiteRing,
    WhiteSeanceRing,
    BellowingDragoncrestRing,
    DuskCrownRing,
    HornetRing,
    HawkRing,
    RingofSteelProtection,
    CovetousGoldSerpentRing,
    CovetousSilverSerpentRing,
    SlumberingDragoncrestRing,
    RingofFog,
    RustedIronRing,
    RingofSacrifice,
    RareRingofSacrifice,
    DarkWoodGrainRing,
    RingoftheSunPrincess,
    OldWitchsRing,
    CovenantofArtorias,
    OrangeCharredRing,
    LingeringDragoncrestRing,
    RingoftheEvilEye,
    RingofFavorandProtection,
    LeoRing,
    EastWoodGrainRing,
    WolfRing,
    BlueTearstoneRing,
    RingoftheSunsFirstborn,
    DarkmoonSeanceRing,
    CalamityRing,

    //Shields
    SkullLantern,
    EastWestShield,
    WoodenShield,
    LargeLeatherShield,
    SmallLeatherShield,
    TargetShield,
    Buckler,
    CrackedRoundShield,
    LeatherShield,
    PlankShield,
    CaduceusRoundShield,
    CrystalRingShield,
    HeaterShield,
    KnightShield,
    TowerKiteShield,
    GrassCrestShield,
    HollowSoldierShield,
    BalderShield,
    CrestShield,
    DragonCrestShield,
    WarriorsRoundShield,
    IronRoundShield,
    SpiderShield,
    SpikedShield,
    CrystalShield,
    SunlightShield,
    SilverKnightShield,
    BlackKnightShield,
    PierceShield,
    RedandWhiteRoundShield,
    CaduceusKiteShield,
    GargoylesShield,
    EagleShield,
    TowerShield,
    GiantShield,
    StoneGreatshield,
    HavelsGreatshield,
    BonewheelShield,
    GreatshieldofArtorias,
    EffigyShield,
    Sanctus,
    Bloodshield,
    BlackIronGreatshield,
    CleansingGreatshield,


    //spells
    SorcerySoulArrow,
    SorceryGreatSoulArrow,
    SorceryHeavySoulArrow,
    SorceryGreatHeavySoulArrow,
    SorceryHomingSoulmass,
    SorceryHomingCrystalSoulmass,
    SorcerySoulSpear,
    SorceryCrystalSoulSpear,
    SorceryMagicWeapon,
    SorceryGreatMagicWeapon,
    SorceryCrystalMagicWeapon,
    SorceryMagicShield,
    SorceryStrongMagicShield,
    SorceryHiddenWeapon,
    SorceryHiddenBody,
    SorceryCastLight,
    SorceryHush,
    SorceryAuralDecoy,
    SorceryRepair,
    SorceryFallControl,
    SorceryChameleon,
    SorceryResistCurse,
    SorceryRemedy,
    SorceryWhiteDragonBreath,
    SorceryDarkOrb,
    SorceryDarkBead,
    SorceryDarkFog,
    SorceryPursuers,
    PyromancyFireball,
    PyromancyFireOrb,
    PyromancyGreatFireball,
    PyromancyFirestorm,
    PyromancyFireTempest,
    PyromancyFireSurge,
    PyromancyFireWhip,
    PyromancyCombustion,
    PyromancyGreatCombustion,
    PyromancyPoisonMist,
    PyromancyToxicMist,
    PyromancyAcidSurge,
    PyromancyIronFlesh,
    PyromancyFlashSweat,
    PyromancyUndeadRapport,
    PyromancyPowerWithin,
    PyromancyGreatChaosFireball,
    PyromancyChaosStorm,
    PyromancyChaosFireWhip,
    PyromancyBlackFlame,
    MiracleHeal,
    MiracleGreatHeal,
    MiracleGreatHealExcerpt,
    MiracleSoothingSunlight,
    MiracleReplenishment,
    MiracleBountifulSunlight,
    MiracleGravelordSwordDance,
    MiracleGravelordGreatswordDance,
    MiracleHomeward,
    MiracleForce,
    MiracleWrathoftheGods,
    MiracleEmitForce,
    MiracleSeekGuidance,
    MiracleLightningSpear,
    MiracleGreatLightningSpear,
    MiracleSunlightSpear,
    MiracleMagicBarrier,
    MiracleGreatMagicBarrier,
    MiracleKarmicJustice,
    MiracleTranquilWalkofPeace,
    MiracleVowofSilence,
    MiracleSunlightBlade,
    MiracleDarkmoonBlade,

    //spells tools
    SorcerersCatalyst,
    BeatricesCatalyst,
    TinBanishmentCatalyst,
    LogansCatalyst,
    TinDarkmoonCatalyst,
    OolacileIvoryCatalyst,
    TinCrystallizationCatalyst,
    DemonsCatalyst,
    IzalithCatalyst,
    PyromancyFlame,
    PyromancyFlameAscended,
    Talisman,
    CanvasTalisman,
    ThorolundTalisman,
    IvoryTalisman,
    SunlightTalisman,
    DarkmoonTalisman,
    VelkasTalisman,
    ManusCatalyst,
    OolacileCatalyst,

    //Upgrade materials
    LargeEmber,
    VeryLargeEmber,
    CrystalEmber,
    LargeMagicEmber,
    EnchantedEmber,
    DivineEmber,
    LargeDivineEmber,
    DarkEmber,
    LargeFlameEmber,
    ChaosFlameEmber,
    TitaniteShard,
    LargeTitaniteShard,
    GreenTitaniteShard,
    TitaniteChunk,
    BlueTitaniteChunk,
    WhiteTitaniteChunk,
    RedTitaniteChunk,
    TitaniteSlab,
    BlueTitaniteSlab,
    WhiteTitaniteSlab,
    RedTitaniteSlab,
    DragonScale,
    DemonTitanite,
    TwinklingTitanite,

    //Usable items
    WhiteSignSoapstone,
    RedSignSoapstone,
    RedEyeOrb,
    BlackSeparationCrystal,
    OrangeGuidanceSoapstone,
    BookoftheGuilty,
    ServantRoster,
    BlueEyeOrb,
    DragonEye,
    BlackEyeOrb,
    Darksign,
    PurpleCowardsCrystal,
    SilverPendant,
    Binoculars,
    DragonHeadStone,
    DragonTorsoStone,
    DriedFinger,
    HelloCarving,
    ThankyouCarving,
    VerygoodCarving,
    ImsorryCarving,
    HelpmeCarving,
}