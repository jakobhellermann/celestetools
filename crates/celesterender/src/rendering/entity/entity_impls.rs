use super::RenderMethod;
use std::collections::HashMap;

#[rustfmt::skip]
pub fn render_methods() -> HashMap<&'static str, RenderMethod> {
    let mut textures = HashMap::new();

    textures.insert("AdventureHelper/BladeTrackSpinnerMultinode", RenderMethod::Texture { texture: "danger/blade00", justification: None });
    textures.insert("AdventureHelper/GroupedFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("AdventureHelper/StarTrackSpinnerMultinode", RenderMethod::Texture { texture: "danger/starfish14", justification: None });
    textures.insert("Anonhelper/AnonCloud", RenderMethod::Texture { texture: "objects/AnonHelper/clouds/whitecloud00", justification: None });
    textures.insert("Anonhelper/CloudRefill", RenderMethod::Texture { texture: "objects/AnonHelper/cloudRefill/idle00", justification: None });
    textures.insert("Anonhelper/FeatherBumper", RenderMethod::Texture { texture: "objects/AnonHelper/featherBumper/Idle22", justification: None });
    textures.insert("Anonhelper/FeatherRefill", RenderMethod::Texture { texture: "objects/AnonHelper/featherRefill/idle00", justification: None });
    textures.insert("Anonhelper/InvisibleSeekerBarrier", RenderMethod::Rect { fill: Some((64, 64, 64, 204)), border: Some((0, 0, 0, 0)) });
    textures.insert("Anonhelper/JellyRefill", RenderMethod::Texture { texture: "objects/AnonHelper/jellyRefill/idle00", justification: None });
    textures.insert("Anonhelper/OneUseBooster", RenderMethod::Texture { texture: "objects/booster/booster00", justification: None });
    textures.insert("Anonhelper/SuperDashRefill", RenderMethod::Texture { texture: "objects/AnonHelper/superDashRefill/idle00", justification: None });
    textures.insert("Anonhelper/WindCloud", RenderMethod::Texture { texture: "objects/AnonHelper/clouds/windcloud00", justification: None });
    textures.insert("ArphimigonsDSides/MindFieldTouchSwitch", RenderMethod::Texture { texture: "objects/touchswitch/container", justification: Some((0.5, 0.5)) });
    textures.insert("ArphimigonsDSides/PlayerSeeker", RenderMethod::Texture { texture: "decals/5-temple/statue_e", justification: None });
    textures.insert("ArphimigonsDSidesAfterStory/CatsnugCollectible", RenderMethod::Texture { texture: "decals/arphimigon/catsnugSmall", justification: Some((0.5, 0.5)) });
    textures.insert("BounceHelper/BounceBumper", RenderMethod::Texture { texture: "objects/Bumper/Idle22", justification: None });
    textures.insert("BounceHelper/BounceDreamBlock", RenderMethod::Rect { fill: Some((0, 0, 0, 255)), border: Some((255, 255, 255, 255)) });
    textures.insert("BounceHelper/BounceFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("BounceHelper/BounceRefill", RenderMethod::Texture { texture: "objects/refill/idle00", justification: None });
    textures.insert("BrokemiaHelper/CelesteNetFlagSynchronizer", RenderMethod::Texture { texture: "Ahorn/BrokemiaHelper/CelesteNetFlagSynchronizer", justification: None });
    textures.insert("BrokemiaHelper/dashSpring", RenderMethod::Texture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });
    textures.insert("BrokemiaHelper/dashSpringDown", RenderMethod::Texture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });
    textures.insert("BrokemiaHelper/moveBlockBarrier", RenderMethod::Rect { fill: Some((115, 0, 115, 204)), border: Some((115, 0, 115, 204)) });
    textures.insert("BrokemiaHelper/questionableFlagController", RenderMethod::Texture { texture: "Ahorn/BrokemiaHelper/questionableFlagController", justification: None });
    textures.insert("BrokemiaHelper/wallDashSpringLeft", RenderMethod::Texture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });
    textures.insert("BrokemiaHelper/wallDashSpringRight", RenderMethod::Texture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });
    textures.insert("CherryHelper/AnterogradeController", RenderMethod::Texture { texture: "objects/anterogradeController/icon", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/BadelineBot", RenderMethod::Texture { texture: "characters/player_badeline/sitDown00", justification: Some((0.5, 1.0)) });
    textures.insert("CherryHelper/DoorField", RenderMethod::Rect { fill: Some((0, 0, 0, 255)), border: Some((51, 51, 153, 255)) });
    textures.insert("CherryHelper/EntityToggleBell", RenderMethod::Texture { texture: "objects/itemToggleBell/bell00", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/FallTeleport", RenderMethod::Texture { texture: "objects/temple/portal/portalframe", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/ItemCrystal", RenderMethod::Texture { texture: "objects/itemCrystal/idle00", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/ItemCrystalPedestal", RenderMethod::Texture { texture: "objects/itemCrystalPedestal/pedestal00", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/NightItemLockfield", RenderMethod::Rect { fill: Some((102, 102, 102, 102)), border: Some((102, 102, 102, 255)) });
    textures.insert("CherryHelper/RottenBerry", RenderMethod::Texture { texture: "collectables/rottenberry/normal00", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/ShadowBumper", RenderMethod::Texture { texture: "objects/shadowBumper/shadow22", justification: None });
    textures.insert("CherryHelper/ShadowDashRefill", RenderMethod::Texture { texture: "objects/shadowDashRefill/idle00", justification: Some((0.5, 0.5)) });
    textures.insert("CollabUtils2/CollabCrystalHeart", RenderMethod::Texture { texture: "collectables/heartGem/0/00", justification: None });
    textures.insert("CollabUtils2/GoldenBerryPlayerRespawnPoint", RenderMethod::Texture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)) });
    textures.insert("CollabUtils2/GymMarker", RenderMethod::Texture { texture: "CollabUtils2/editor_gymmarker", justification: None });
    textures.insert("CollabUtils2/LobbyMapController", RenderMethod::Texture { texture: "CollabUtils2/editor_lobbymapmarker", justification: None });
    textures.insert("CollabUtils2/LobbyMapMarker", RenderMethod::Texture { texture: "CollabUtils2/editor_lobbymapmarker", justification: None });
    textures.insert("CollabUtils2/RainbowBerry", RenderMethod::Texture { texture: "CollabUtils2/rainbowBerry/rberry0030", justification: None });
    textures.insert("CollabUtils2/SilverBerry", RenderMethod::Texture { texture: "CollabUtils2/silverBerry/idle00", justification: None });
    textures.insert("CollabUtils2/SpeedBerry", RenderMethod::Texture { texture: "CollabUtils2/speedBerry/Idle_g06", justification: None });
    textures.insert("CollabUtils2/WarpPedestal", RenderMethod::Texture { texture: "CollabUtils2/placeholderorb/placeholderorb00", justification: Some((0.5, 0.95)) });
    textures.insert("CommunalHelper/BadelineBoostKeepHoldables", RenderMethod::Texture { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("CommunalHelper/CassetteJumpFixController", RenderMethod::Texture { texture: "objects/CommunalHelper/cassetteJumpFixController/icon", justification: None });
    textures.insert("CommunalHelper/CoreModeMusicController", RenderMethod::Texture { texture: "objects/CommunalHelper/coreModeMusicController/iconEnable", justification: None });
    textures.insert("CommunalHelper/CrystalHeart", RenderMethod::Texture { texture: "collectables/heartGem/ghost00", justification: None });
    textures.insert("CommunalHelper/DreamBoosterAny", RenderMethod::Texture { texture: "objects/CommunalHelper/boosters/dreamBooster/idle00", justification: None });
    textures.insert("CommunalHelper/DreamRefill", RenderMethod::Texture { texture: "objects/CommunalHelper/dreamRefill/idle02", justification: None });
    textures.insert("CommunalHelper/DreamStrawberry", RenderMethod::Texture { texture: "collectables/CommunalHelper/dreamberry/wings01", justification: None });
    textures.insert("CommunalHelper/ElytraDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("CommunalHelper/GlowController", RenderMethod::Texture { texture: "objects/CommunalHelper/glowController/icon", justification: None });
    textures.insert("CommunalHelper/HintController", RenderMethod::Texture { texture: "objects/CommunalHelper/hintController/icon", justification: None });
    textures.insert("CommunalHelper/InputFlagController", RenderMethod::Texture { texture: "objects/CommunalHelper/inputFlagController/icon", justification: None });
    textures.insert("CommunalHelper/LightningController", RenderMethod::Texture { texture: "objects/CommunalHelper/lightningController/icon", justification: None });
    textures.insert("CommunalHelper/ManualCassetteController", RenderMethod::Texture { texture: "objects/CommunalHelper/manualCassetteController/icon", justification: None });
    textures.insert("CommunalHelper/NoOverlayLookout", RenderMethod::Texture { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)) });
    textures.insert("CommunalHelper/ResetStateCrystal", RenderMethod::Texture { texture: "objects/CommunalHelper/resetStateCrystal/ghostIdle00", justification: None });
    textures.insert("CommunalHelper/SJ/AirTimeMusicController", RenderMethod::Texture { texture: "objects/CommunalHelper/strawberryJam/airTimeMusicController/icon", justification: None });
    textures.insert("CommunalHelper/SJ/BulletTimeController", RenderMethod::Texture { texture: "objects/CommunalHelper/strawberryJam/bulletTimeController/icon", justification: None });
    textures.insert("CommunalHelper/SJ/ExpiringDashRefill", RenderMethod::Texture { texture: "objects/refill/idle00", justification: None });
    textures.insert("CommunalHelper/SJ/FlagBreakerBox", RenderMethod::Texture { texture: "objects/breakerBox/Idle00", justification: Some((0.25, 0.25)) });
    textures.insert("CommunalHelper/SJ/PhotosensitiveFlagController", RenderMethod::Texture { texture: "objects/CommunalHelper/strawberryJam/photosensitiveFlagController/icon", justification: None });
    textures.insert("CommunalHelper/SeekerDashRefill", RenderMethod::Texture { texture: "objects/CommunalHelper/seekerDashRefill/idle00", justification: None });
    textures.insert("CommunalHelper/SyncedZipMoverActivationController", RenderMethod::Texture { texture: "objects/CommunalHelper/syncedZipMoverActivationController/syncedZipMoverActivationController", justification: None });
    textures.insert("CommunalHelper/UnderwaterMusicController", RenderMethod::Texture { texture: "objects/CommunalHelper/underwaterMusicController/icon", justification: None });
    textures.insert("CrystalBombDetonator/CrystalBombDetonator", RenderMethod::Rect { fill: Some((115, 0, 115, 204)), border: Some((115, 0, 115, 204)) });
    textures.insert("DJMapHelper/badelineBoostDown", RenderMethod::Texture { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("DJMapHelper/badelineBoostTeleport", RenderMethod::Texture { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("DJMapHelper/featherBarrier", RenderMethod::Rect { fill: Some((64, 64, 192, 128)), border: Some((64, 64, 192, 255)) });
    textures.insert("DJMapHelper/finalBossReversed", RenderMethod::Texture { texture: "characters/badelineBoss/charge00", justification: None });
    textures.insert("DJMapHelper/flingBirdReversed", RenderMethod::Texture { texture: "characters/bird/Hover04", justification: None });
    textures.insert("DJMapHelper/oshiroBossRight", RenderMethod::Texture { texture: "characters/oshiro/boss13", justification: None });
    textures.insert("DJMapHelper/playSprite", RenderMethod::Texture { texture: "characters/oldlady/idle00", justification: Some((0.5, 1.0)) });
    textures.insert("DJMapHelper/shield", RenderMethod::Texture { texture: "objects/DJMapHelper/shield/shield", justification: None });
    textures.insert("DJMapHelper/startPoint", RenderMethod::Texture { texture: "characters/player/sitDown15", justification: Some((0.5, 1.0)) });
    textures.insert("DJMapHelper/theoCrystalBarrier", RenderMethod::Rect { fill: Some((64, 128, 64, 204)), border: Some((64, 128, 64, 204)) });
    textures.insert("DSModHelper/ReskinnableStrawberry", RenderMethod::Texture { texture: "collectables/strawberry/normal00", justification: Some((0.5, 0.5)) });
    textures.insert("EeveeHelper/CoreZoneStartController", RenderMethod::Texture { texture: "objects/EeveeHelper/coreZoneStartController/icon", justification: None });
    textures.insert("EeveeHelper/CoreZoneToggle", RenderMethod::Texture { texture: "objects/coreFlipSwitch/switch01", justification: None });
    textures.insert("EeveeHelper/HoldableTiles", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("EeveeHelper/LenientCeilingPopController", RenderMethod::Texture { texture: "objects/EeveeHelper/lenientCeilingPopController/icon", justification: None });
    textures.insert("EeveeHelper/NoDemoBindController", RenderMethod::Texture { texture: "objects/EeveeHelper/noDemoBindController/icon", justification: None });
    textures.insert("EeveeHelper/PatientBooster", RenderMethod::Texture { texture: "objects/EeveeHelper/patientBooster/booster00", justification: None });
    textures.insert("EeveeHelper/RoomChestExit", RenderMethod::Rect { fill: Some((255, 179, 192, 102)), border: Some((255, 179, 192, 255)) });
    textures.insert("ExtendedVariantMode/VariantToggleController", RenderMethod::Texture { texture: "ahorn/ExtendedVariantMode/whydrawarectanglewhenyoucandrawapngofarectangleinstead", justification: Some((0.0, 0.0)) });
    textures.insert("FactoryHelper/Battery", RenderMethod::Texture { texture: "objects/FactoryHelper/batteryBox/battery00", justification: None });
    textures.insert("FactoryHelper/BatteryBox", RenderMethod::Texture { texture: "objects/FactoryHelper/batteryBox/inactive0", justification: None });
    textures.insert("FactoryHelper/BoomBox", RenderMethod::Texture { texture: "objects/FactoryHelper/boomBox/idle00", justification: Some((0.0, 0.0)) });
    textures.insert("FactoryHelper/DashFuseBox", RenderMethod::Texture { texture: "objects/FactoryHelper/dashFuseBox/idle00", justification: Some((0.0, 0.0)) });
    textures.insert("FactoryHelper/DoorRusty", RenderMethod::Texture { texture: "objects/FactoryHelper/doorRusty/metaldoor00", justification: Some((0.5, 1.0)) });
    textures.insert("FactoryHelper/FactoryActivatorDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("FactoryHelper/KillerDebris", RenderMethod::Texture { texture: "danger/FactoryHelper/debris/fg_Bronze1", justification: None });
    textures.insert("FactoryHelper/MachineHeart", RenderMethod::Texture { texture: "objects/FactoryHelper/machineHeart/front0", justification: None });
    textures.insert("FactoryHelper/PowerLine", RenderMethod::Rect { fill: Some((179, 179, 179, 255)), border: Some((179, 179, 179, 255)) });
    textures.insert("FactoryHelper/RustyLamp", RenderMethod::Texture { texture: "objects/FactoryHelper/rustyLamp/rustyLamp00", justification: Some((0.0, 0.0)) });
    textures.insert("FactoryHelper/ThrowBox", RenderMethod::Texture { texture: "objects/FactoryHelper/crate/crate0", justification: Some((0.0, 0.0)) });
    textures.insert("FactoryHelper/WindTunnel", RenderMethod::Rect { fill: Some((179, 179, 179, 102)), border: Some((179, 179, 179, 255)) });
    textures.insert("FancyTileEntities/BetterIntroCrusher", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("FancyTileEntities/BetterRidgeGate", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("FemtoHelper/AssistHazardController", RenderMethod::Texture { texture: "loenn/FemtoHelper/squishcontroller", justification: None });
    textures.insert("FemtoHelper/BackdropWindController", RenderMethod::Texture { texture: "loenn/FemtoHelper/BackdropWindController", justification: Some((0.5, 0.5)) });
    textures.insert("FemtoHelper/CustomMoonCreature", RenderMethod::Texture { texture: "scenery/moon_creatures/tiny01", justification: None });
    textures.insert("FemtoHelper/OshiroCaller", RenderMethod::Texture { texture: "objects/FemtoHelper/oshiroCaller/caller00", justification: Some((0.5, 0.5)) });
    textures.insert("FemtoHelper/VitalDrainController", RenderMethod::Texture { texture: "loenn/Femtohelper/vitalcontroller", justification: None });
    textures.insert("FlaglinesAndSuch/BloomedOshiro", RenderMethod::Texture { texture: "objects/FlaglinesAndSuch/bloomedoshiro/boss13", justification: None });
    textures.insert("FlaglinesAndSuch/BlueBlock", RenderMethod::Rect { fill: Some((43, 136, 217, 255)), border: Some((68, 183, 255, 255)) });
    textures.insert("FlaglinesAndSuch/BonfireLight", RenderMethod::Texture { texture: "ahorn/FlaglinesAndSuch/bonfireIcon", justification: Some((0.0, 0.0)) });
    textures.insert("FlaglinesAndSuch/DustNoShrinkController", RenderMethod::Texture { texture: "ahorn/FlaglinesAndSuch/dust_no_shrink", justification: None });
    textures.insert("FlaglinesAndSuch/MusicParamOnFlag", RenderMethod::Texture { texture: "ahorn/FlaglinesAndSuch/flag_count_music", justification: None });
    textures.insert("FlaglinesAndSuch/NailHittableSprite", RenderMethod::Texture { texture: "glass", justification: None });
    textures.insert("FlaglinesAndSuch/ShyGhost", RenderMethod::Texture { texture: "objects/FlaglinesAndSuch/shyghost/chase00", justification: None });
    textures.insert("FlaglinesAndSuch/StandBox", RenderMethod::Texture { texture: "objects/FlaglinesAndSuch/standbox/idle00", justification: None });
    textures.insert("FlaglinesAndSuch/Wingmould", RenderMethod::Texture { texture: "objects/FlaglinesAndSuch/Wingmould/idle00", justification: None });
    textures.insert("FrostHelper/CoreBerry", RenderMethod::Texture { texture: "collectables/FrostHelper/CoreBerry/Hot/CoreBerry_Hot00", justification: None });
    textures.insert("FrostHelper/KeyIce", RenderMethod::Texture { texture: "collectables/FrostHelper/keyice/idle00", justification: None });
    textures.insert("FrostHelper/LightOccluderEntity", RenderMethod::Rect { fill: Some((255, 255, 255, 51)), border: Some((255, 255, 255, 255)) });
    textures.insert("FrostHelper/TemporaryKey", RenderMethod::Texture { texture: "collectables/FrostHelper/keytemp/idle00", justification: None });
    textures.insert("FurryHelper/GlitchWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("JungleHelper/AttachTriggerController", RenderMethod::Texture { texture: "ahorn/JungleHelper/attach_trigger_trigger", justification: Some((0.0, 0.0)) });
    textures.insert("JungleHelper/AutoFallingBlockDelayed", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("JungleHelper/BreakablePot", RenderMethod::Texture { texture: "JungleHelper/Breakable Pot/breakpotidle", justification: None });
    textures.insert("JungleHelper/CassetteCustomPreviewMusic", RenderMethod::Texture { texture: "collectables/cassette/idle00", justification: None });
    textures.insert("JungleHelper/CheatCodeController", RenderMethod::Texture { texture: "ahorn/JungleHelper/cheat_code", justification: None });
    textures.insert("JungleHelper/Cobweb", RenderMethod::Texture { texture: "JungleHelper/Cobweb/idle00", justification: None });
    textures.insert("JungleHelper/Cockatiel", RenderMethod::Texture { texture: "JungleHelper/Cockatiel/idle00", justification: None });
    textures.insert("JungleHelper/EnforceSkinController", RenderMethod::Texture { texture: "ahorn/JungleHelper/enforce_skin_controller", justification: None });
    textures.insert("JungleHelper/Firefly", RenderMethod::Texture { texture: "JungleHelper/Firefly/firefly00", justification: None });
    textures.insert("JungleHelper/Hawk", RenderMethod::Texture { texture: "JungleHelper/hawk/hold03", justification: None });
    textures.insert("JungleHelper/Lantern", RenderMethod::Texture { texture: "JungleHelper/Lantern/LanternEntity/lantern_00", justification: None });
    textures.insert("JungleHelper/RemoteKevinRefill", RenderMethod::Texture { texture: "JungleHelper/SlideBlockRefill/idle00", justification: None });
    textures.insert("JungleHelper/RollingRock", RenderMethod::Texture { texture: "JungleHelper/RollingRock/boulder", justification: None });
    textures.insert("JungleHelper/Snake", RenderMethod::Texture { texture: "JungleHelper/Snake/IdleAggro/snake_idle00", justification: None });
    textures.insert("JungleHelper/TheoStatue", RenderMethod::Texture { texture: "JungleHelper/TheoStatue/idle00", justification: None });
    textures.insert("JungleHelper/Torch", RenderMethod::Texture { texture: "JungleHelper/TorchNight/TorchNightOff", justification: None });
    textures.insert("JungleHelper/TreasureChest", RenderMethod::Texture { texture: "JungleHelper/Treasure/TreasureIdle00", justification: None });
    textures.insert("JungleHelper/TreeDepthController", RenderMethod::Texture { texture: "collectables/goldberry/wings01", justification: None });
    textures.insert("MaxHelpingHand/BadelineSprite", RenderMethod::Texture { texture: "characters/badeline/idle00", justification: Some((0.5, 1.0)) });
    textures.insert("MaxHelpingHand/BeeFireball", RenderMethod::Texture { texture: "objects/MaxHelpingHand/beeFireball/beefireball00", justification: None });
    textures.insert("MaxHelpingHand/CustomCh3MemoOnFlagController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });
    textures.insert("MaxHelpingHand/CustomMemorialWithDreamingAttribute", RenderMethod::Texture { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)) });
    textures.insert("MaxHelpingHand/CustomNPCSprite", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/custom_npc_xml", justification: Some((0.5, 1.0)) });
    textures.insert("MaxHelpingHand/CustomSandwichLava", RenderMethod::Texture { texture: "@Internal@/lava_sandwich", justification: None });
    textures.insert("MaxHelpingHand/CustomSeekerBarrier", RenderMethod::Rect { fill: Some((64, 64, 64, 204)), border: Some((64, 64, 64, 204)) });
    textures.insert("MaxHelpingHand/CustomTutorialWithNoBird", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/greyscale_birb", justification: Some((0.5, 1.0)) });
    textures.insert("MaxHelpingHand/CustomizableBerry", RenderMethod::Texture { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("MaxHelpingHand/CustomizableGlassBlock", RenderMethod::Rect { fill: Some((255, 255, 255, 153)), border: Some((255, 255, 255, 204)) });
    textures.insert("MaxHelpingHand/CustomizableGlassBlockAreaController", RenderMethod::Rect { fill: Some((102, 102, 255, 102)), border: Some((102, 102, 255, 255)) });
    textures.insert("MaxHelpingHand/CustomizableGlassBlockController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/CustomizableGlassExitBlock", RenderMethod::Rect { fill: Some((255, 255, 255, 153)), border: Some((255, 255, 255, 204)) });
    textures.insert("MaxHelpingHand/CustomizableGlassFallingBlock", RenderMethod::Rect { fill: Some((255, 255, 255, 153)), border: Some((255, 255, 255, 204)) });
    textures.insert("MaxHelpingHand/DisableControlsController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/disable_controls", justification: None });
    textures.insert("MaxHelpingHand/ExpandTriggerController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/expand_trigger_controller", justification: None });
    textures.insert("MaxHelpingHand/FancyTextTutorial", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/greyscale_birb", justification: Some((0.5, 1.0)) });
    textures.insert("MaxHelpingHand/FlagBadelineChaser", RenderMethod::Texture { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)) });
    textures.insert("MaxHelpingHand/FlagBreakerBox", RenderMethod::Texture { texture: "objects/breakerBox/Idle00", justification: Some((0.25, 0.25)) });
    textures.insert("MaxHelpingHand/FlagDecalXML", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/flag_decal_xml", justification: None });
    textures.insert("MaxHelpingHand/FlagExitBlock", RenderMethod::FakeTiles {
        material_key: "tileType",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("MaxHelpingHand/FlagPickup", RenderMethod::Texture { texture: "MaxHelpingHand/flagpickup/Flag/Flag0", justification: None });
    textures.insert("MaxHelpingHand/FlagRainbowSpinnerColorAreaController", RenderMethod::Rect { fill: Some((102, 102, 255, 102)), border: Some((102, 102, 255, 255)) });
    textures.insert("MaxHelpingHand/FlagRainbowSpinnerColorController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/GoldenStrawberryCustomConditions", RenderMethod::Texture { texture: "collectables/goldberry/idle00", justification: None });
    textures.insert("MaxHelpingHand/HintsFlagController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/hints_flag_controller", justification: None });
    textures.insert("MaxHelpingHand/HorizontalRoomWrapController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/horizontal_room_wrap", justification: None });
    textures.insert("MaxHelpingHand/KevinBarrier", RenderMethod::Rect { fill: Some((64, 64, 64, 204)), border: Some((64, 64, 64, 204)) });
    textures.insert("MaxHelpingHand/LitBlueTorch", RenderMethod::Texture { texture: "objects/temple/torch03", justification: None });
    textures.insert("MaxHelpingHand/MultiNodeBumper", RenderMethod::Texture { texture: "objects/Bumper/Idle22", justification: None });
    textures.insert("MaxHelpingHand/MultiRoomStrawberry", RenderMethod::Texture { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("MaxHelpingHand/NonPoppingStrawberry", RenderMethod::Texture { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("MaxHelpingHand/OneWayInvisibleBarrierHorizontal", RenderMethod::Rect { fill: Some((102, 102, 102, 204)), border: Some((0, 0, 0, 0)) });
    textures.insert("MaxHelpingHand/ParallaxFadeOutController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/ParallaxFadeSpeedController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/RainbowSpinnerColorAreaController", RenderMethod::Rect { fill: Some((102, 102, 255, 102)), border: Some((102, 102, 255, 255)) });
    textures.insert("MaxHelpingHand/RainbowSpinnerColorController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/RainbowSpinnerColorControllerDisabler", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/rainbowSpinnerColorControllerDisable", justification: None });
    textures.insert("MaxHelpingHand/ReversibleRetentionBooster", RenderMethod::Texture { texture: "objects/MaxHelpingHand/reversibleRetentionBooster/booster00", justification: None });
    textures.insert("MaxHelpingHand/SecretBerry", RenderMethod::Texture { texture: "collectables/moonBerry/normal00", justification: None });
    textures.insert("MaxHelpingHand/SeekerBarrierColorController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/SeekerBarrierColorControllerDisabler", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/rainbowSpinnerColorControllerDisable", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnActionController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/set_flag_on_action", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnButtonPressController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/set_flag_on_button", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnCompletionController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnFullClearController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnHeartCollectedController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnSpawnController", RenderMethod::Texture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });
    textures.insert("MaxHelpingHand/SidewaysLava", RenderMethod::Texture { texture: "@Internal@/rising_lava", justification: None });
    textures.insert("MaxHelpingHand/StaticPuffer", RenderMethod::Texture { texture: "objects/puffer/idle00", justification: None });
    textures.insert("MaxHelpingHand/StylegroundFadeController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MemorialHelper/FlagCrystalHeart", RenderMethod::Texture { texture: "collectables/heartGem/white00", justification: None });
    textures.insert("MemorialHelper/ParallaxText", RenderMethod::Rect { fill: Some((255, 255, 255, 64)), border: Some((255, 255, 255, 192)) });
    textures.insert("SJ2021/MaskedOutline", RenderMethod::Texture { texture: "objects/SJ2021/maskedOutlineController", justification: Some((0.5, 0.5)) });
    textures.insert("ShroomHelper/CrumbleBlockOnTouch", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("ShroomHelper/DoubleRefillBooster", RenderMethod::Texture { texture: "objects/sh_doublerefillbooster/boosterPink00", justification: None });
    textures.insert("ShroomHelper/OneDashWingedStrawberry", RenderMethod::Texture { texture: "collectables/ghostgoldberry/wings01", justification: None });
    textures.insert("ShroomHelper/RealityDistortionField", RenderMethod::Rect { fill: Some((0, 0, 255, 255)), border: Some((0, 0, 255, 255)) });
    textures.insert("ShroomHelper/ShroomBookInteraction", RenderMethod::Rect { fill: Some((106, 13, 173, 255)), border: Some((106, 13, 173, 255)) });
    textures.insert("SorbetHelper/CrumbleOnFlagBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("SorbetHelper/DashFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("SorbetHelper/KillZone", RenderMethod::Rect { fill: Some((176, 99, 100, 76)), border: Some((145, 59, 95, 179)) });
    textures.insert("SummitBackgroundManager", RenderMethod::Texture { texture: "@Internal@/summit_background_manager", justification: None });
    textures.insert("VivHelper/BumperWrapper", RenderMethod::Texture { texture: "ahorn/VivHelper/bumperWrapper", justification: None });
    textures.insert("VivHelper/CustomCoreMessage", RenderMethod::Texture { texture: "@Internal@/core_message", justification: None });
    textures.insert("VivHelper/CustomDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VivHelper/CustomDepthTileEntity", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VivHelper/CustomFakeWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("VivHelper/CustomPlaybackWatchtower", RenderMethod::Texture { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)) });
    textures.insert("VivHelper/CustomTorch", RenderMethod::Texture { texture: "ahorn/VivHelper/torch/grayTorchUnlit", justification: None });
    textures.insert("VivHelper/DashBumper", RenderMethod::Texture { texture: "VivHelper/dashBumper/idle00", justification: None });
    textures.insert("VivHelper/DebrisLimiter", RenderMethod::Texture { texture: "ahorn/VivHelper/DebrisLimiter", justification: None });
    textures.insert("VivHelper/EarlyFlagSetter", RenderMethod::Texture { texture: "ahorn/VivHelper/flagBeforeAwake", justification: None });
    textures.insert("VivHelper/EnergyCrystal", RenderMethod::Texture { texture: "VivHelper/entities/gem", justification: None });
    textures.insert("VivHelper/EnterBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VivHelper/EvilBumper", RenderMethod::Texture { texture: "objects/Bumper/Evil22", justification: None });
    textures.insert("VivHelper/ExitDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VivHelper/FlagIntroCrusher", RenderMethod::FakeTiles {
        material_key: "tileType",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VivHelper/FloatyBreakBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VivHelper/FollowTorch", RenderMethod::Texture { texture: "FollowTorchSprites/ThorcVar/DefaultTorch00", justification: None });
    textures.insert("VivHelper/GoldenBerryToFlag", RenderMethod::Texture { texture: "ahorn/VivHelper/GoldenBerryToFlag", justification: None });
    textures.insert("VivHelper/HideRoomInMap", RenderMethod::Texture { texture: "ahorn/VivHelper/HiddenRoom", justification: None });
    textures.insert("VivHelper/OrangeBooster", RenderMethod::Texture { texture: "VivHelper/boosters/boosterOrange00", justification: None });
    textures.insert("VivHelper/PinkBooster", RenderMethod::Texture { texture: "VivHelper/boosters/boosterPink00", justification: None });
    textures.insert("VivHelper/PreviousBerriesToFlag", RenderMethod::Texture { texture: "ahorn/VivHelper/PrevBerriesToFlag", justification: None });
    textures.insert("VivHelper/RedDashRefill", RenderMethod::Texture { texture: "VivHelper/redDashRefill/redIdle00", justification: None });
    textures.insert("VivHelper/RefillPotion", RenderMethod::Texture { texture: "VivHelper/Potions/PotRefill00", justification: None });
    textures.insert("VivHelper/RefilllessBumper", RenderMethod::Texture { texture: "ahorn/VivHelper/norefillBumper", justification: None });
    textures.insert("VivHelper/WarpDashRefill", RenderMethod::Texture { texture: "VivHelper/TSStelerefill/idle00", justification: None });
    textures.insert("VortexHelper/AutoFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VortexHelper/BowlPuffer", RenderMethod::Texture { texture: "objects/VortexHelper/pufferBowl/idle00", justification: None });
    textures.insert("VortexHelper/DashBubble", RenderMethod::Texture { texture: "objects/VortexHelper/dashBubble/idle00", justification: None });
    textures.insert("VortexHelper/PufferBarrier", RenderMethod::Rect { fill: Some((255, 189, 74, 180)), border: Some((255, 189, 74, 180)) });
    textures.insert("VortexHelper/PurpleBooster", RenderMethod::Texture { texture: "objects/VortexHelper/slingBooster/slingBooster00", justification: None });
    textures.insert("VortexHelper/VortexCustomBumper", RenderMethod::Texture { texture: "objects/VortexHelper/vortexCustomBumper/green22", justification: None });
    textures.insert("XaphanHelper/BreakBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CustomBadelineBoss", RenderMethod::Texture { texture: "characters/badelineBoss/charge00", justification: None });
    textures.insert("XaphanHelper/CustomCheckpoint", RenderMethod::Texture { texture: "objects/XaphanHelper/CustomCheckpoint/bg00", justification: None });
    textures.insert("XaphanHelper/CustomCoverupWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CustomDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CustomEndScreenController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/customEndScreenController", justification: None });
    textures.insert("XaphanHelper/CustomExitBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CustomFakeWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CustomTorch", RenderMethod::Texture { texture: "objects/XaphanHelper/CustomTorch/torch00", justification: None });
    textures.insert("XaphanHelper/Elevator", RenderMethod::Texture { texture: "objects/XaphanHelper/Elevator/elevator00", justification: None });
    textures.insert("XaphanHelper/ElevatorBarrier", RenderMethod::Rect { fill: Some((102, 102, 102, 204)), border: Some((0, 0, 0, 0)) });
    textures.insert("XaphanHelper/FlagBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/HeatController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/heatController", justification: None });
    textures.insert("XaphanHelper/InGameMapController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/mapController", justification: None });
    textures.insert("XaphanHelper/InGameMapHintController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/hintController", justification: None });
    textures.insert("XaphanHelper/InGameMapRoomAdjustController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/roomAdjustController", justification: None });
    textures.insert("XaphanHelper/InGameMapRoomController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/roomController", justification: None });
    textures.insert("XaphanHelper/InGameMapSubAreaController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/subAreaController", justification: None });
    textures.insert("XaphanHelper/InGameMapTilesController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/tilesController", justification: None });
    textures.insert("XaphanHelper/JumpBlocksFlipSoundController", RenderMethod::Texture { texture: "@Internal@/sound_source", justification: None });
    textures.insert("XaphanHelper/MergeChaptersController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/mergeChaptersController", justification: None });
    textures.insert("XaphanHelper/SetStatsFlagsController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/setStatsFlagsController ", justification: None });
    textures.insert("XaphanHelper/TimedStrawberry", RenderMethod::Texture { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("XaphanHelper/TimedTempleGate", RenderMethod::Texture { texture: "objects/door/TempleDoorB00", justification: None });
    textures.insert("XaphanHelper/TimerRefill", RenderMethod::Texture { texture: "objects/XaphanHelper/TimerRefill/idle00", justification: None });
    textures.insert("XaphanHelper/UpgradeController", RenderMethod::Texture { texture: "util/XaphanHelper/Loenn/upgradeController", justification: None });
    textures.insert("XaphanHelper/WarpStation", RenderMethod::Texture { texture: "objects/XaphanHelper/WarpStation/idle00", justification: None });
    textures.insert("YetAnotherHelper/BubbleField", RenderMethod::Rect { fill: Some((0, 0, 255, 102)), border: Some((255, 255, 255, 128)) });
    textures.insert("YetAnotherHelper/FlagKillBarrier", RenderMethod::Rect { fill: Some((202, 97, 97, 153)), border: Some((202, 81, 76, 179)) });
    textures.insert("YetAnotherHelper/SpikeJumpThruController", RenderMethod::Texture { texture: "ahorn/YetAnotherHelper/spikeJumpThruController", justification: None });
    textures.insert("YetAnotherHelper/StickyJellyfish", RenderMethod::Texture { texture: "ahorn/YetAnotherHelper/stickyJellyfish", justification: None });
    textures.insert("badelineBoost", RenderMethod::Texture { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("batteries/battery", RenderMethod::Texture { texture: "batteries/battery/full0", justification: Some((0.5, 1.0)) });
    textures.insert("batteries/power_refill", RenderMethod::Texture { texture: "batteries/power_refill/idle00", justification: None });
    textures.insert("batteries/recharge_platform", RenderMethod::Texture { texture: "batteries/recharge_platform/base0", justification: Some((0.5, 1.0)) });
    textures.insert("bgSwitch/bgModeToggle", RenderMethod::Texture { texture: "objects/BGswitch/bgflipswitch/switch01", justification: None });
    textures.insert("bigSpinner", RenderMethod::Texture { texture: "objects/Bumper/Idle22", justification: None });
    textures.insert("bird", RenderMethod::Texture { texture: "characters/bird/crow00", justification: Some((0.5, 1.0)) });
    textures.insert("birdPath", RenderMethod::Texture { texture: "characters/bird/flyup00", justification: None });
    textures.insert("blackGem", RenderMethod::Texture { texture: "collectables/heartGem/0/00", justification: None });
    textures.insert("blockField", RenderMethod::Rect { fill: Some((102, 102, 255, 102)), border: Some((102, 102, 255, 255)) });
    textures.insert("bonfire", RenderMethod::Texture { texture: "objects/campfire/fire08", justification: Some((0.5, 1.0)) });
    textures.insert("booster", RenderMethod::Texture { texture: "objects/booster/booster00", justification: None });
    textures.insert("brokemiahelper/cassetteDreamBlock", RenderMethod::Rect { fill: Some((0, 0, 0, 255)), border: Some((73, 170, 240, 255)) });
    textures.insert("canyon/spinorb", RenderMethod::Texture { texture: "objects/canyon/spinorb/idle00", justification: Some((0.5, 0.5)) });
    textures.insert("cassette", RenderMethod::Texture { texture: "collectables/cassette/idle00", justification: None });
    textures.insert("cavern/crystalBombField", RenderMethod::Rect { fill: Some((115, 0, 115, 204)), border: Some((115, 0, 115, 204)) });
    textures.insert("cavern/fakecavernheart", RenderMethod::Texture { texture: "collectables/heartGem/0/00", justification: Some((0.5, 0.5)) });
    textures.insert("cliffside_flag", RenderMethod::Texture { texture: "scenery/cliffside/flag00", justification: Some((0.0, 0.0)) });
    textures.insert("clutterDoor", RenderMethod::Rect { fill: Some((74, 71, 135, 255)), border: Some((255, 255, 255, 255)) });
    textures.insert("conditionBlock", RenderMethod::FakeTiles {
        material_key: "tileType",
        blend_key: true,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("coreMessage", RenderMethod::Texture { texture: "@Internal@/core_message", justification: None });
    textures.insert("coreModeToggle", RenderMethod::Texture { texture: "objects/coreFlipSwitch/switch01", justification: None });
    textures.insert("coverupWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("crumbleWallOnRumble", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("cutsceneNode", RenderMethod::Texture { texture: "@Internal@/cutscene_node", justification: None });
    textures.insert("darkChaser", RenderMethod::Texture { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)) });
    textures.insert("darkChaserEnd", RenderMethod::Rect { fill: Some((102, 0, 102, 102)), border: Some((102, 0, 102, 255)) });
    textures.insert("dashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("dreamBlock", RenderMethod::Rect { fill: Some((0, 0, 0, 255)), border: Some((255, 255, 255, 255)) });
    textures.insert("dreamHeartGem", RenderMethod::Texture { texture: "collectables/heartGem/0/00", justification: None });
    textures.insert("everest/coreMessage", RenderMethod::Texture { texture: "@Internal@/core_message", justification: None });
    textures.insert("everest/customBirdTutorial", RenderMethod::Texture { texture: "characters/bird/crow00", justification: Some((0.5, 1.0)) });
    textures.insert("everest/memorial", RenderMethod::Texture { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)) });
    textures.insert("everest/npc", RenderMethod::Texture { texture: "characters/00", justification: Some((0.5, 1.0)) });
    textures.insert("everest/starClimbGraphicsController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("exitBlock", RenderMethod::FakeTiles {
        material_key: "tileType",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("eyebomb", RenderMethod::Texture { texture: "objects/puffer/idle00", justification: None });
    textures.insert("fakeBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("fakeHeart", RenderMethod::Texture { texture: "collectables/heartGem/0/00", justification: None });
    textures.insert("fakeWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: Some("tilesFg"),
        color: Some((255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("fallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("finalBoss", RenderMethod::Texture { texture: "characters/badelineBoss/charge00", justification: None });
    textures.insert("finalBossFallingBlock", RenderMethod::FakeTiles {
        material_key: "G",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("finalBossMovingBlock", RenderMethod::FakeTiles {
        material_key: "G",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("fireBall", RenderMethod::Texture { texture: "objects/fireball/fireball01", justification: None });
    textures.insert("fireBarrier", RenderMethod::Rect { fill: Some((209, 9, 1, 102)), border: Some((246, 98, 18, 255)) });
    textures.insert("flingBird", RenderMethod::Texture { texture: "characters/bird/Hover04", justification: None });
    textures.insert("flingBirdIntro", RenderMethod::Texture { texture: "characters/bird/Hover04", justification: None });
    textures.insert("friendlyGhost", RenderMethod::Texture { texture: "characters/oshiro/boss13", justification: None });
    textures.insert("glassBlock", RenderMethod::Rect { fill: Some((255, 255, 255, 153)), border: Some((255, 255, 255, 204)) });
    textures.insert("goldenBerry", RenderMethod::Texture { texture: "collectables/goldberry/idle00", justification: None });
    textures.insert("iceBlock", RenderMethod::Rect { fill: Some((76, 168, 214, 102)), border: Some((108, 214, 235, 255)) });
    textures.insert("introCrusher", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("invisibleBarrier", RenderMethod::Rect { fill: Some((102, 102, 102, 204)), border: Some((0, 0, 0, 0)) });
    textures.insert("key", RenderMethod::Texture { texture: "collectables/key/idle00", justification: None });
    textures.insert("lightning", RenderMethod::Rect { fill: Some((140, 248, 245, 102)), border: Some((253, 245, 120, 255)) });
    textures.insert("lightningBlock", RenderMethod::Texture { texture: "objects/breakerBox/Idle00", justification: Some((0.25, 0.25)) });
    textures.insert("luaCutscenes/luaTalker", RenderMethod::Rect { fill: Some((0, 255, 255, 102)), border: Some((0, 255, 255, 255)) });
    textures.insert("memorial", RenderMethod::Texture { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)) });
    textures.insert("memorialTextController", RenderMethod::Texture { texture: "collectables/goldberry/wings01", justification: None });
    textures.insert("negaBlock", RenderMethod::Rect { fill: Some((255, 0, 0, 255)), border: Some((255, 0, 0, 255)) });
    textures.insert("pandorasBox/airBubbles", RenderMethod::Texture { texture: "objects/pandorasBox/airBubbles/idle00", justification: None });
    textures.insert("pandorasBox/checkpoint", RenderMethod::Texture { texture: "objects/pandorasBox/checkpoint/flag/active_idle00", justification: Some((0.5, 1.0)) });
    textures.insert("pandorasBox/dashToggleBlock", RenderMethod::Rect { fill: Some((204, 76, 255, 102)), border: Some((204, 76, 255, 255)) });
    textures.insert("pandorasBox/dreamDashController", RenderMethod::Texture { texture: "objects/pandorasBox/controllerIcons/dreamDashController", justification: None });
    textures.insert("pandorasBox/dustSpriteColorController", RenderMethod::Texture { texture: "objects/pandorasBox/controllerIcons/dustSpriteColorController", justification: None });
    textures.insert("pandorasBox/entityActivator", RenderMethod::Rect { fill: Some((255, 179, 179, 102)), border: Some((179, 179, 255, 255)) });
    textures.insert("pandorasBox/flagToggleSwitch", RenderMethod::Texture { texture: "objects/pandorasBox/flagToggleSwitch/switch01", justification: None });
    textures.insert("pandorasBox/gate", RenderMethod::Texture { texture: "objects/pandorasBox/gate/gate0", justification: Some((0.0, 0.0)) });
    textures.insert("pandorasBox/laserEmitter", RenderMethod::Texture { texture: "objects/pandorasBox/laser/emitter/idle0", justification: Some((0.5, 1.0)) });
    textures.insert("pandorasBox/laserNoteBlock", RenderMethod::Texture { texture: "objects/pandorasBox/laser/noteblock/noteblock_horizontal", justification: None });
    textures.insert("pandorasBox/lever", RenderMethod::Texture { texture: "objects/pandorasBox/lever/lever0", justification: Some((0.5, 1.0)) });
    textures.insert("pandorasBox/pandorasBox", RenderMethod::Texture { texture: "objects/pandorasBox/pandorasBox/box_idle0", justification: Some((0.5, 1.0)) });
    textures.insert("pandorasBox/playerClone", RenderMethod::Texture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)) });
    textures.insert("pandorasBox/propellerBox", RenderMethod::Texture { texture: "objects/pandorasBox/propellerBox/default/default_charges00", justification: Some((0.5, 1.0)) });
    textures.insert("pandorasBox/timefield", RenderMethod::Rect { fill: Some((128, 255, 255, 102)), border: Some((128, 255, 255, 255)) });
    textures.insert("pandorasBox/waterDrowningController", RenderMethod::Texture { texture: "objects/pandorasBox/controllerIcons/waterDrowningController", justification: None });
    textures.insert("payphone", RenderMethod::Texture { texture: "scenery/payphone", justification: Some((0.5, 1.0)) });
    textures.insert("picoconsole", RenderMethod::Texture { texture: "objects/pico8Console", justification: Some((0.5, 1.0)) });
    textures.insert("plateau", RenderMethod::Texture { texture: "scenery/fallplateau", justification: Some((0.0, 0.0)) });
    textures.insert("playbackTutorial", RenderMethod::Texture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)) });
    textures.insert("player", RenderMethod::Texture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)) });
    textures.insert("playerSeeker", RenderMethod::Texture { texture: "decals/5-temple/statue_e", justification: None });
    textures.insert("refill", RenderMethod::Texture { texture: "objects/refill/idle00", justification: None });
    textures.insert("ridgeGate", RenderMethod::Texture { texture: "objects/ridgeGate", justification: Some((0.0, 0.0)) });
    textures.insert("risingLava", RenderMethod::Texture { texture: "@Internal@/rising_lava", justification: None });
    textures.insert("sandwichLava", RenderMethod::Texture { texture: "@Internal@/lava_sandwich", justification: None });
    textures.insert("seeker", RenderMethod::Texture { texture: "characters/monsters/predator73", justification: None });
    textures.insert("seekerBarrier", RenderMethod::Rect { fill: Some((64, 64, 64, 204)), border: Some((64, 64, 64, 204)) });
    textures.insert("seekerStatue", RenderMethod::Texture { texture: "decals/5-temple/statue_e", justification: None });
    textures.insert("spring", RenderMethod::Texture { texture: "objects/spring/00", justification: Some((0.5, 1.0)) });
    textures.insert("starClimbController", RenderMethod::Texture { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("strawberry", RenderMethod::Texture { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("summitGemManager", RenderMethod::Texture { texture: "@Internal@/summit_gem_manager", justification: None });
    textures.insert("summitgem", RenderMethod::Texture { texture: "collectables/summitgems/0/gem00", justification: None });
    textures.insert("tentacles", RenderMethod::Texture { texture: "@Internal@/tentacles", justification: None });
    textures.insert("theoCrystalPedestal", RenderMethod::Texture { texture: "characters/theoCrystal/pedestal", justification: Some((0.5, 1.0)) });
    textures.insert("torch", RenderMethod::Texture { texture: "objects/temple/torch00", justification: None });
    textures.insert("towerviewer", RenderMethod::Texture { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)) });
    textures.insert("vitellary/boostbumper", RenderMethod::Texture { texture: "objects/boostBumper/booster00", justification: None });
    textures.insert("vitellary/cassetteflags", RenderMethod::Texture { texture: "CrystallineHelper/FLCC/ahorn_cassetteflagcontroller", justification: None });
    textures.insert("vitellary/custompuffer", RenderMethod::Texture { texture: "objects/puffer/idle00", justification: None });
    textures.insert("vitellary/dashcodecontroller", RenderMethod::Texture { texture: "ahorn_dashcodecontroller", justification: None });
    textures.insert("vitellary/fillcrystal", RenderMethod::Texture { texture: "objects/crystals/fill/idle00", justification: None });
    textures.insert("vitellary/flagsequencecontroller", RenderMethod::Texture { texture: "ahorn_flagsequencecontroller", justification: None });
    textures.insert("vitellary/goodtelecrystal", RenderMethod::Texture { texture: "objects/crystals/tele/right/idle00", justification: None });
    textures.insert("vitellary/interactivechaser", RenderMethod::Texture { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)) });
    textures.insert("vitellary/roomname", RenderMethod::Texture { texture: "ahorn_roomname", justification: None });
    textures.insert("vitellary/starcrystal", RenderMethod::Texture { texture: "objects/crystals/star/idle00", justification: None });
    textures.insert("vitellary/timecrystal", RenderMethod::Texture { texture: "objects/crystals/time/idle00", justification: None });
    textures.insert("wallSpringLeft", RenderMethod::Texture { texture: "objects/spring/00", justification: Some((0.5, 1.0)) });
    textures.insert("wallSpringRight", RenderMethod::Texture { texture: "objects/spring/00", justification: Some((0.5, 1.0)) });
    textures.insert("whiteblock", RenderMethod::Texture { texture: "objects/whiteblock", justification: Some((0.0, 0.0)) });

    textures
}
