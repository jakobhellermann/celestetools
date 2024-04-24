#![allow(clippy::approx_constant)]
use super::{RenderMethod, RenderTexture};
use std::collections::HashMap;
use tiny_skia::Color;

#[rustfmt::skip]
pub fn render_methods() -> HashMap<&'static str, RenderMethod> {
    let mut textures = HashMap::new();

    textures.insert("AdventureHelper/BladeTrackSpinnerMultinode", RenderMethod::Textures(vec![RenderTexture { texture: "danger/blade00", justification: None, rotation: None },]));
    textures.insert("AdventureHelper/CustomCrystalHeart", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/3/00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("AdventureHelper/DustTrackSpinnerMultinode", RenderMethod::Textures(vec![RenderTexture { texture: "danger/dustcreature/base00", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "danger/dustcreature/center00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("AdventureHelper/GroupedFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("AdventureHelper/StarTrackSpinnerMultinode", RenderMethod::Textures(vec![RenderTexture { texture: "danger/starfish14", justification: None, rotation: None },]));
    textures.insert("Anonhelper/AnonCloud", RenderMethod::Textures(vec![RenderTexture { texture: "objects/AnonHelper/clouds/whitecloud00", justification: None, rotation: None },]));
    textures.insert("Anonhelper/CloudRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/AnonHelper/cloudRefill/idle00", justification: None, rotation: None },]));
    textures.insert("Anonhelper/FeatherBumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/AnonHelper/featherBumper/Idle22", justification: None, rotation: None },]));
    textures.insert("Anonhelper/FeatherRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/AnonHelper/featherRefill/idle00", justification: None, rotation: None },]));
    textures.insert("Anonhelper/InvisibleSeekerBarrier", RenderMethod::Rect { fill: Color::from_rgba8(64, 64, 64, 204), border: Color::from_rgba8(0, 0, 0, 0) });
    textures.insert("Anonhelper/JellyRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/AnonHelper/jellyRefill/idle00", justification: None, rotation: None },]));
    textures.insert("Anonhelper/OneUseBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/booster/booster00", justification: None, rotation: None },]));
    textures.insert("Anonhelper/SuperDashRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/AnonHelper/superDashRefill/idle00", justification: None, rotation: None },]));
    textures.insert("Anonhelper/WindCloud", RenderMethod::Textures(vec![RenderTexture { texture: "objects/AnonHelper/clouds/windcloud00", justification: None, rotation: None },]));
    textures.insert("ArphimigonHelper/AnchoredSpinnerParent", RenderMethod::Textures(vec![RenderTexture { texture: "danger/dustcreature/center00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/BadelineBoss", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badelineBoss/charge00", justification: None, rotation: None },]));
    textures.insert("ArphimigonHelper/BoostRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/boostRefill/idle00", justification: None, rotation: None },]));
    textures.insert("ArphimigonHelper/CatassaultPhase1", RenderMethod::Textures(vec![RenderTexture { texture: "objects/catassaultPhaseOne/main13", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/CoreMessage", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/core_message", justification: None, rotation: None },]));
    textures.insert("ArphimigonHelper/DashTriggeredCoreModeController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/coreFlipSwitch/switch01", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/DifficultRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/DifficultRefill/idle00", justification: None, rotation: None },]));
    textures.insert("ArphimigonHelper/ElementalCrystalSpinner", RenderMethod::Textures(vec![RenderTexture { texture: "danger/crystal/fg_white00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/ElementalModeToggle", RenderMethod::Textures(vec![RenderTexture { texture: "objects/elementalToggle/active", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/elementalToggle/top_lever", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/ElementalRuneTablet", RenderMethod::Textures(vec![RenderTexture { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("ArphimigonHelper/GiantClam", RenderMethod::Textures(vec![RenderTexture { texture: "objects/giantClam/open100", justification: Some((0.0, 1.0)), rotation: None },]));
    textures.insert("ArphimigonHelper/HeartGem", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/3/00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/HeartOfTheStorm", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/3/00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/HeartOfTheStormContainer", RenderMethod::Textures(vec![RenderTexture { texture: "objects/crystalHeartContainer/empty", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/JellySpawner", RenderMethod::Textures(vec![RenderTexture { texture: "objects/jellySpawner/baseDisabled", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/RefillRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/refillRefill/idle00", justification: None, rotation: None },]));
    textures.insert("ArphimigonHelper/ShieldedGoldenBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/goldberry/idle00", justification: None, rotation: None },]));
    textures.insert("ArphimigonHelper/SnappingClam", RenderMethod::Textures(vec![RenderTexture { texture: "objects/snappingClam/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/TempleEyeball", RenderMethod::Textures(vec![RenderTexture { texture: "danger/templeeye/body00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonHelper/ThrowableRefillContainer", RenderMethod::Textures(vec![RenderTexture { texture: "objects/throwableRefillContainer/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonsDSides/MindFieldTouchSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/touchswitch/container", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ArphimigonsDSides/PlayerSeeker", RenderMethod::Textures(vec![RenderTexture { texture: "decals/5-temple/statue_e", justification: None, rotation: None },]));
    textures.insert("ArphimigonsDSidesAfterStory/CatsnugCollectible", RenderMethod::Textures(vec![RenderTexture { texture: "decals/arphimigon/catsnugSmall", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("AuraHelper/Bird", RenderMethod::Textures(vec![RenderTexture { texture: "objects/bird1", justification: None, rotation: None },]));
    textures.insert("AuraHelper/Fire", RenderMethod::Textures(vec![RenderTexture { texture: "objects/fire2", justification: None, rotation: None },]));
    textures.insert("AuraHelper/Health", RenderMethod::Textures(vec![RenderTexture { texture: "objects/health", justification: None, rotation: None },]));
    textures.insert("AuraHelper/IceKiller", RenderMethod::Textures(vec![RenderTexture { texture: "objects/icekiller", justification: None, rotation: None },]));
    textures.insert("AuraHelper/IceSlime", RenderMethod::Textures(vec![RenderTexture { texture: "objects/iceslime1", justification: None, rotation: None },]));
    textures.insert("AuraHelper/Insect", RenderMethod::Textures(vec![RenderTexture { texture: "objects/insect1", justification: None, rotation: None },]));
    textures.insert("AuraHelper/Lantern", RenderMethod::Textures(vec![RenderTexture { texture: "objects/lantern", justification: None, rotation: None },]));
    textures.insert("AuraHelper/Slime", RenderMethod::Textures(vec![RenderTexture { texture: "objects/slime1", justification: None, rotation: None },]));
    textures.insert("AurorasHelper/BulletHellController", RenderMethod::Textures(vec![RenderTexture { texture: "controllers/AurorasHelper/BulletHellController", justification: None, rotation: None },]));
    textures.insert("AurorasHelper/ChangeRespawnOrb", RenderMethod::Textures(vec![RenderTexture { texture: "objects/respawn_orb/idle00", justification: None, rotation: None },]));
    textures.insert("AurorasHelper/DieOnFlagsController", RenderMethod::Textures(vec![RenderTexture { texture: "controllers/AurorasHelper/DieOnFlagsController", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("AurorasHelper/FairySpawner", RenderMethod::Textures(vec![RenderTexture { texture: "objects/aurora_aquir/fairy_spawner/portal", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("AurorasHelper/FlagDirectionGem", RenderMethod::Textures(vec![RenderTexture { texture: "objects/reflectionHeart/gem", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("AurorasHelper/FriendlySeeker", RenderMethod::Textures(vec![RenderTexture { texture: "characters/monsters/predator73", justification: None, rotation: None },]));
    textures.insert("AurorasHelper/HorizontalCollisionDeathController", RenderMethod::Textures(vec![RenderTexture { texture: "controllers/AurorasHelper/HorizontalCollisionDeathController", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("AurorasHelper/InternetMemorial", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("AurorasHelper/PauseMusicWhenPausedController", RenderMethod::Textures(vec![RenderTexture { texture: "controllers/AurorasHelper/PauseMusicWhenPausedController", justification: None, rotation: None },]));
    textures.insert("AurorasHelper/SpeedLimitFlagController", RenderMethod::Textures(vec![RenderTexture { texture: "controllers/AurorasHelper/SpeedLimitFlagController", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("AurorasHelper/TimedFlagController", RenderMethod::Textures(vec![RenderTexture { texture: "controllers/AurorasHelper/TimedFlagController", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("AurorasHelper/WaveCrystal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/auroras_helper/mode_crystals/wave_crystal/idle00", justification: None, rotation: None },]));
    textures.insert("BounceHelper/BounceBumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/Bumper/Idle22", justification: None, rotation: None },]));
    textures.insert("BounceHelper/BounceDreamBlock", RenderMethod::Rect { fill: Color::from_rgba8(0, 0, 0, 255), border: Color::from_rgba8(255, 255, 255, 255) });
    textures.insert("BounceHelper/BounceFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("BounceHelper/BounceJellyfish", RenderMethod::Textures(vec![RenderTexture { texture: "objects/BounceHelper/bounceJellyfish/pink/idle0", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("BounceHelper/BounceRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/refill/idle00", justification: None, rotation: None },]));
    textures.insert("BrokemiaHelper/CelesteNetFlagSynchronizer", RenderMethod::Textures(vec![RenderTexture { texture: "Ahorn/BrokemiaHelper/CelesteNetFlagSynchronizer", justification: None, rotation: None },]));
    textures.insert("BrokemiaHelper/dashSpring", RenderMethod::Textures(vec![RenderTexture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("BrokemiaHelper/dashSpringDown", RenderMethod::Textures(vec![RenderTexture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)), rotation: Some(3.1415927) },]));
    textures.insert("BrokemiaHelper/moveBlockBarrier", RenderMethod::Rect { fill: Color::from_rgba8(115, 0, 115, 204), border: Color::from_rgba8(115, 0, 115, 204) });
    textures.insert("BrokemiaHelper/questionableFlagController", RenderMethod::Textures(vec![RenderTexture { texture: "Ahorn/BrokemiaHelper/questionableFlagController", justification: None, rotation: None },]));
    textures.insert("BrokemiaHelper/wallDashSpringLeft", RenderMethod::Textures(vec![RenderTexture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)), rotation: Some(1.5707964) },]));
    textures.insert("BrokemiaHelper/wallDashSpringRight", RenderMethod::Textures(vec![RenderTexture { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)), rotation: Some(-1.5707964) },]));
    textures.insert("CNY2024Helper/EasingBlackhole", RenderMethod::Textures(vec![RenderTexture { texture: "decals/ChineseNewYear2024/StarSapphire/GDDNblackhole/asmallblackholecanrotitself00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CNY2024Helper/IHPHKDialogEntity", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glider/idle0", justification: None, rotation: None },]));
    textures.insert("CherryHelper/AnterogradeController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/anterogradeController/icon", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CherryHelper/BadelineBot", RenderMethod::Textures(vec![RenderTexture { texture: "characters/player_badeline/sitDown00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("CherryHelper/DoorField", RenderMethod::Rect { fill: Color::from_rgba8(0, 0, 0, 255), border: Color::from_rgba8(51, 51, 153, 255) });
    textures.insert("CherryHelper/EntityToggleBell", RenderMethod::Textures(vec![RenderTexture { texture: "objects/itemToggleBell/bell00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CherryHelper/FallTeleport", RenderMethod::Textures(vec![RenderTexture { texture: "objects/temple/portal/portalframe", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CherryHelper/ItemCrystal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/itemCrystal/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CherryHelper/ItemCrystalPedestal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/itemCrystalPedestal/pedestal00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CherryHelper/NightItemLockfield", RenderMethod::Rect { fill: Color::from_rgba8(102, 102, 102, 102), border: Color::from_rgba8(102, 102, 102, 255) });
    textures.insert("CherryHelper/RottenBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/rottenberry/normal00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CherryHelper/ShadowBumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/shadowBumper/shadow22", justification: None, rotation: None },]));
    textures.insert("CherryHelper/ShadowDashRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/shadowDashRefill/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ChronoHelper/BoomBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/chronohelper/boomBooster/booster00", justification: None, rotation: None },]));
    textures.insert("ChronoHelper/LavaSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/chronohelper/lavaSwitch/switch_0.png", justification: None, rotation: None },]));
    textures.insert("ChronoHelper/LevelResetZone", RenderMethod::Rect { fill: Color::from_rgba8(64, 64, 64, 204), border: Color::from_rgba8(64, 64, 64, 204) });
    textures.insert("ChronoHelper/PersistentFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("ChronoHelper/ShatterRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/chronohelper/destroyRefill/idle00", justification: None, rotation: None },]));
    textures.insert("ChronoHelper/ShatterSpinner", RenderMethod::Textures(vec![RenderTexture { texture: "danger/crystal/fg00", justification: None, rotation: None },]));
    textures.insert("ChronoHelper/StaticDebrisDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("CollabUtils2/CollabCrystalHeart", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/0/00", justification: None, rotation: None },]));
    textures.insert("CollabUtils2/GoldenBerryPlayerRespawnPoint", RenderMethod::Textures(vec![RenderTexture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("CollabUtils2/GymMarker", RenderMethod::Textures(vec![RenderTexture { texture: "CollabUtils2/editor_gymmarker", justification: None, rotation: None },]));
    textures.insert("CollabUtils2/LobbyMapController", RenderMethod::Textures(vec![RenderTexture { texture: "CollabUtils2/editor_lobbymapmarker", justification: None, rotation: None },]));
    textures.insert("CollabUtils2/LobbyMapMarker", RenderMethod::Textures(vec![RenderTexture { texture: "CollabUtils2/editor_lobbymapmarker", justification: None, rotation: None },]));
    textures.insert("CollabUtils2/RainbowBerry", RenderMethod::Textures(vec![RenderTexture { texture: "CollabUtils2/rainbowBerry/rberry0030", justification: None, rotation: None },]));
    textures.insert("CollabUtils2/SilverBerry", RenderMethod::Textures(vec![RenderTexture { texture: "CollabUtils2/silverBerry/idle00", justification: None, rotation: None },]));
    textures.insert("CollabUtils2/SpeedBerry", RenderMethod::Textures(vec![RenderTexture { texture: "CollabUtils2/speedBerry/Idle_g06", justification: None, rotation: None },]));
    textures.insert("CollabUtils2/WarpPedestal", RenderMethod::Textures(vec![RenderTexture { texture: "CollabUtils2/placeholderorb/placeholderorb00", justification: Some((0.5, 0.95)), rotation: None },]));
    textures.insert("CommunalHelper/BadelineBoostKeepHoldables", RenderMethod::Textures(vec![RenderTexture { texture: "objects/badelineboost/idle00", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/CassetteJumpFixController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/cassetteJumpFixController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/CoreModeMusicController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/coreModeMusicController/iconEnable", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/CrystalHeart", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/ghost00", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/DreamBoosterAny", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/boosters/dreamBooster/idle00", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/DreamJellyfish", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/dreamJellyfish/jello", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CommunalHelper/DreamRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/dreamRefill/idle02", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/DreamStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/CommunalHelper/dreamberry/wings01", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/ElytraDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("CommunalHelper/GlowController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/glowController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/HeldBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/boosters/heldBooster/purple/booster00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("CommunalHelper/HintController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/hintController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/InputFlagController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/inputFlagController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/LightningController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/lightningController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/ManualCassetteController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/manualCassetteController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/NoOverlayLookout", RenderMethod::Textures(vec![RenderTexture { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("CommunalHelper/ResetStateCrystal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/resetStateCrystal/ghostIdle00", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/SJ/AirTimeMusicController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/strawberryJam/airTimeMusicController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/SJ/BulletTimeController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/strawberryJam/bulletTimeController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/SJ/ExpiringDashRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/refill/idle00", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/SJ/FlagBreakerBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/breakerBox/Idle00", justification: Some((0.25, 0.25)), rotation: None },]));
    textures.insert("CommunalHelper/SJ/PhotosensitiveFlagController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/strawberryJam/photosensitiveFlagController/icon", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/SeekerDashRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/seekerDashRefill/idle00", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/SyncedZipMoverActivationController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/syncedZipMoverActivationController/syncedZipMoverActivationController", justification: None, rotation: None },]));
    textures.insert("CommunalHelper/UnderwaterMusicController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CommunalHelper/underwaterMusicController/icon", justification: None, rotation: None },]));
    textures.insert("CrystalBombDetonator/CrystalBombDetonator", RenderMethod::Rect { fill: Color::from_rgba8(115, 0, 115, 204), border: Color::from_rgba8(115, 0, 115, 204) });
    textures.insert("DJMapHelper/badelineBoostDown", RenderMethod::Textures(vec![RenderTexture { texture: "objects/badelineboost/idle00", justification: None, rotation: None },]));
    textures.insert("DJMapHelper/badelineBoostTeleport", RenderMethod::Textures(vec![RenderTexture { texture: "objects/badelineboost/idle00", justification: None, rotation: None },]));
    textures.insert("DJMapHelper/colorfulFlyFeather", RenderMethod::Textures(vec![RenderTexture { texture: "objects/DJMapHelper/blueFlyFeather/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("DJMapHelper/colorfulRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/DJMapHelper/blueRefill/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("DJMapHelper/featherBarrier", RenderMethod::Rect { fill: Color::from_rgba8(64, 64, 192, 128), border: Color::from_rgba8(64, 64, 192, 255) });
    textures.insert("DJMapHelper/finalBossReversed", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badelineBoss/charge00", justification: None, rotation: None },]));
    textures.insert("DJMapHelper/flingBirdReversed", RenderMethod::Textures(vec![RenderTexture { texture: "characters/bird/Hover04", justification: None, rotation: None },]));
    textures.insert("DJMapHelper/oshiroBossRight", RenderMethod::Textures(vec![RenderTexture { texture: "characters/oshiro/boss13", justification: None, rotation: None },]));
    textures.insert("DJMapHelper/playSprite", RenderMethod::Textures(vec![RenderTexture { texture: "characters/oldlady/idle00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("DJMapHelper/shield", RenderMethod::Textures(vec![RenderTexture { texture: "objects/DJMapHelper/shield/shield", justification: None, rotation: None },]));
    textures.insert("DJMapHelper/springGreen", RenderMethod::Textures(vec![RenderTexture { texture: "objects/DJMapHelper/springGreen/00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("DJMapHelper/startPoint", RenderMethod::Textures(vec![RenderTexture { texture: "characters/player/sitDown15", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("DJMapHelper/theoCrystalBarrier", RenderMethod::Rect { fill: Color::from_rgba8(64, 128, 64, 204), border: Color::from_rgba8(64, 128, 64, 204) });
    textures.insert("DSModHelper/ReskinnableStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("DSidesPlatinum/HiddenStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/ghostberry/idle00", justification: None, rotation: None },]));
    textures.insert("EeveeHelper/CoreZoneStartController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/EeveeHelper/coreZoneStartController/icon", justification: None, rotation: None },]));
    textures.insert("EeveeHelper/CoreZoneToggle", RenderMethod::Textures(vec![RenderTexture { texture: "objects/coreFlipSwitch/switch01", justification: None, rotation: None },]));
    textures.insert("EeveeHelper/HoldableTiles", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("EeveeHelper/LenientCeilingPopController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/EeveeHelper/lenientCeilingPopController/icon", justification: None, rotation: None },]));
    textures.insert("EeveeHelper/NoDemoBindController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/EeveeHelper/noDemoBindController/icon", justification: None, rotation: None },]));
    textures.insert("EeveeHelper/PatientBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/EeveeHelper/patientBooster/booster00", justification: None, rotation: None },]));
    textures.insert("EeveeHelper/RoomChestExit", RenderMethod::Rect { fill: Color::from_rgba8(255, 179, 192, 102), border: Color::from_rgba8(255, 179, 192, 255) });
    textures.insert("ExtendedVariantMode/VariantToggleController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/ExtendedVariantMode/whydrawarectanglewhenyoucandrawapngofarectangleinstead", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("FactoryHelper/Battery", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/batteryBox/battery00", justification: None, rotation: None },]));
    textures.insert("FactoryHelper/BatteryBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/batteryBox/inactive0", justification: None, rotation: None },]));
    textures.insert("FactoryHelper/BoomBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/boomBox/idle00", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("FactoryHelper/DashFuseBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/dashFuseBox/idle00", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("FactoryHelper/DoorRusty", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/doorRusty/metaldoor00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("FactoryHelper/FactoryActivatorDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("FactoryHelper/KillerDebris", RenderMethod::Textures(vec![RenderTexture { texture: "danger/FactoryHelper/debris/fg_Bronze1", justification: None, rotation: None },]));
    textures.insert("FactoryHelper/MachineHeart", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/machineHeart/front0", justification: None, rotation: None },]));
    textures.insert("FactoryHelper/PowerLine", RenderMethod::Rect { fill: Color::from_rgba8(179, 179, 179, 255), border: Color::from_rgba8(179, 179, 179, 255) });
    textures.insert("FactoryHelper/RustBerry", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/rustBerry/berry_01", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/FactoryHelper/rustBerry/gear_01", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("FactoryHelper/RustyLamp", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/rustyLamp/rustyLamp00", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("FactoryHelper/ThrowBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FactoryHelper/crate/crate0", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("FactoryHelper/WindTunnel", RenderMethod::Rect { fill: Color::from_rgba8(179, 179, 179, 102), border: Color::from_rgba8(179, 179, 179, 255) });
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
    textures.insert("FemtoHelper/AssistHazardController", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/FemtoHelper/squishcontroller", justification: None, rotation: None },]));
    textures.insert("FemtoHelper/BackdropWindController", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/FemtoHelper/BackdropWindController", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("FemtoHelper/CustomMoonCreature", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/moon_creatures/tiny01", justification: None, rotation: None },]));
    textures.insert("FemtoHelper/LaCreatura", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FemtoHelper/butterfly/00", justification: None, rotation: None },]));
    textures.insert("FemtoHelper/OshiroCaller", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FemtoHelper/oshiroCaller/caller00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("FemtoHelper/VitalDrainController", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/Femtohelper/vitalcontroller", justification: None, rotation: None },]));
    textures.insert("FlaglinesAndSuch/BloomedOshiro", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FlaglinesAndSuch/bloomedoshiro/boss13", justification: None, rotation: None },]));
    textures.insert("FlaglinesAndSuch/BlueBlock", RenderMethod::Rect { fill: Color::from_rgba8(43, 136, 217, 255), border: Color::from_rgba8(68, 183, 255, 255) });
    textures.insert("FlaglinesAndSuch/BonfireLight", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/FlaglinesAndSuch/bonfireIcon", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("FlaglinesAndSuch/CustomCloud", RenderMethod::Textures(vec![RenderTexture { texture: "objects/clouds/cloud00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("FlaglinesAndSuch/CustomReflectionStatue", RenderMethod::Textures(vec![RenderTexture { texture: "objects/reflectionHeart/statue", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("FlaglinesAndSuch/DustNoShrinkController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/FlaglinesAndSuch/dust_no_shrink", justification: None, rotation: None },]));
    textures.insert("FlaglinesAndSuch/MusicParamOnFlag", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/FlaglinesAndSuch/flag_count_music", justification: None, rotation: None },]));
    textures.insert("FlaglinesAndSuch/NailHittableSprite", RenderMethod::Textures(vec![RenderTexture { texture: "glass", justification: None, rotation: None },]));
    textures.insert("FlaglinesAndSuch/Sawblade", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FlaglinesAndSuch/Sawblade/small00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("FlaglinesAndSuch/ShyGhost", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FlaglinesAndSuch/shyghost/chase00", justification: None, rotation: None },]));
    textures.insert("FlaglinesAndSuch/StandBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FlaglinesAndSuch/standbox/idle00", justification: None, rotation: None },]));
    textures.insert("FlaglinesAndSuch/Wingmould", RenderMethod::Textures(vec![RenderTexture { texture: "objects/FlaglinesAndSuch/Wingmould/idle00", justification: None, rotation: None },]));
    textures.insert("FrostHelper/CoreBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/FrostHelper/CoreBerry/Hot/CoreBerry_Hot00", justification: None, rotation: None },]));
    textures.insert("FrostHelper/CustomFlutterBird", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/flutterbird/idle00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("FrostHelper/KeyIce", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/FrostHelper/keyice/idle00", justification: None, rotation: None },]));
    textures.insert("FrostHelper/LightOccluderEntity", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 51), border: Color::from_rgba8(255, 255, 255, 255) });
    textures.insert("FrostHelper/TemporaryKey", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/FrostHelper/keytemp/idle00", justification: None, rotation: None },]));
    textures.insert("FurryHelper/GlitchWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("Galactica/BlackHole", RenderMethod::Textures(vec![RenderTexture { texture: "BlackHole/Blackhole00", justification: None, rotation: None },]));
    textures.insert("Galactica/StarLight", RenderMethod::Textures(vec![RenderTexture { texture: "StarLight/StarLight00", justification: None, rotation: None },]));
    textures.insert("Galactica/Wormhole", RenderMethod::Textures(vec![RenderTexture { texture: "Wormhole/Wormhole00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("GameHelper/DashMagnet", RenderMethod::Textures(vec![RenderTexture { texture: "objects/GameHelper/dash_magnet/idle1", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("GameHelper/DecalMover", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/GameHelper/decal_mover", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("GameHelper/Dispenser", RenderMethod::Textures(vec![RenderTexture { texture: "objects/GameHelper/dispenser", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("GameHelper/EntityRespriter", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/GameHelper/entity_respriter", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("GameHelper/FlagCollectBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: None, rotation: None },]));
    textures.insert("GameHelper/MovingSolid", RenderMethod::FakeTiles {
        material_key: "tileset",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("GameHelper/PlayerStateFlag", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/GameHelper/flag_controller", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("GameHelper/PushBoxButton", RenderMethod::Textures(vec![RenderTexture { texture: "objects/GameHelper/push_box_button/idle", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("GameHelper/SuperHotController", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/GameHelper/super_hot_controller", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("GameHelper/Trampoline", RenderMethod::Textures(vec![RenderTexture { texture: "objects/GameHelper/trampoline/idle", justification: Some((0.32, 0.95)), rotation: Some(1.5707964) },]));
    textures.insert("GlitchHelper/BlueGlitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glitch/glitchblue00", justification: None, rotation: None },]));
    textures.insert("GlitchHelper/Glitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glitch/glitchgreen00", justification: None, rotation: None },]));
    textures.insert("GlitchHelper/Mine", RenderMethod::Textures(vec![RenderTexture { texture: "objects/mine/tile", justification: Some((-0.5, -0.5)), rotation: None },]));
    textures.insert("GlitchHelper/PurpleGlitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glitch/glitchpurple00", justification: None, rotation: None },]));
    textures.insert("GlitchHelper/RedGlitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glitch/glitchred00", justification: None, rotation: None },]));
    textures.insert("HDGraphic", RenderMethod::Textures(vec![RenderTexture { texture: "HDGraphic", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("JungleHelper/AttachTriggerController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/JungleHelper/attach_trigger_trigger", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("JungleHelper/AutoFallingBlockDelayed", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("JungleHelper/BreakablePot", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/Breakable Pot/breakpotidle", justification: None, rotation: None },]));
    textures.insert("JungleHelper/CassetteCustomPreviewMusic", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/cassette/idle00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/CheatCodeController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/JungleHelper/cheat_code", justification: None, rotation: None },]));
    textures.insert("JungleHelper/Cobweb", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/Cobweb/idle00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/Cockatiel", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/Cockatiel/idle00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/EnforceSkinController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/JungleHelper/enforce_skin_controller", justification: None, rotation: None },]));
    textures.insert("JungleHelper/Firefly", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/Firefly/firefly00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/Hawk", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/hawk/hold03", justification: None, rotation: None },]));
    textures.insert("JungleHelper/Lantern", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/Lantern/LanternEntity/lantern_00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/RemoteKevinRefill", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/SlideBlockRefill/idle00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/RollingRock", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/RollingRock/boulder", justification: None, rotation: None },]));
    textures.insert("JungleHelper/Snake", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/Snake/IdleAggro/snake_idle00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/TheoStatue", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/TheoStatue/idle00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/Torch", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/TorchNight/TorchNightOff", justification: None, rotation: None },]));
    textures.insert("JungleHelper/TreasureChest", RenderMethod::Textures(vec![RenderTexture { texture: "JungleHelper/Treasure/TreasureIdle00", justification: None, rotation: None },]));
    textures.insert("JungleHelper/TreeDepthController", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/goldberry/wings01", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/BadelineSprite", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badeline/idle00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("MaxHelpingHand/BeeFireball", RenderMethod::Textures(vec![RenderTexture { texture: "objects/MaxHelpingHand/beeFireball/beefireball00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/Comment", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/comment", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("MaxHelpingHand/CustomCh3MemoOnFlagController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/CustomMemorialWithDreamingAttribute", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("MaxHelpingHand/CustomNPCSprite", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/custom_npc_xml", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("MaxHelpingHand/CustomSandwichLava", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/lava_sandwich", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/CustomSeekerBarrier", RenderMethod::Rect { fill: Color::from_rgba8(64, 64, 64, 204), border: Color::from_rgba8(64, 64, 64, 204) });
    textures.insert("MaxHelpingHand/CustomTutorialWithNoBird", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/greyscale_birb", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("MaxHelpingHand/CustomizableBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/CustomizableGlassBlock", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 153), border: Color::from_rgba8(255, 255, 255, 204) });
    textures.insert("MaxHelpingHand/CustomizableGlassBlockAreaController", RenderMethod::Rect { fill: Color::from_rgba8(102, 102, 255, 102), border: Color::from_rgba8(102, 102, 255, 255) });
    textures.insert("MaxHelpingHand/CustomizableGlassBlockController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/CustomizableGlassExitBlock", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 153), border: Color::from_rgba8(255, 255, 255, 204) });
    textures.insert("MaxHelpingHand/CustomizableGlassFallingBlock", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 153), border: Color::from_rgba8(255, 255, 255, 204) });
    textures.insert("MaxHelpingHand/DisableControlsController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/disable_controls", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/ExpandTriggerController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/expand_trigger_controller", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/FancyTextTutorial", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/greyscale_birb", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("MaxHelpingHand/FlagBadelineChaser", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("MaxHelpingHand/FlagBreakerBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/breakerBox/Idle00", justification: Some((0.25, 0.25)), rotation: None },]));
    textures.insert("MaxHelpingHand/FlagDecalXML", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/flag_decal_xml", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/FlagExitBlock", RenderMethod::FakeTiles {
        material_key: "tileType",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("MaxHelpingHand/FlagPickup", RenderMethod::Textures(vec![RenderTexture { texture: "MaxHelpingHand/flagpickup/Flag/Flag0", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/FlagRainbowSpinnerColorAreaController", RenderMethod::Rect { fill: Color::from_rgba8(102, 102, 255, 102), border: Color::from_rgba8(102, 102, 255, 255) });
    textures.insert("MaxHelpingHand/FlagRainbowSpinnerColorController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/GoldenStrawberryCustomConditions", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/goldberry/idle00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/HintsFlagController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/hints_flag_controller", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/HorizontalRoomWrapController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/horizontal_room_wrap", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/KevinBarrier", RenderMethod::Rect { fill: Color::from_rgba8(64, 64, 64, 204), border: Color::from_rgba8(64, 64, 64, 204) });
    textures.insert("MaxHelpingHand/LitBlueTorch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/temple/torch03", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/MultiNodeBumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/Bumper/Idle22", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/MultiRoomStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/NonPoppingStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/ParallaxFadeOutController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/ParallaxFadeSpeedController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/RainbowSpinnerColorAreaController", RenderMethod::Rect { fill: Color::from_rgba8(102, 102, 255, 102), border: Color::from_rgba8(102, 102, 255, 255) });
    textures.insert("MaxHelpingHand/RainbowSpinnerColorController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/RainbowSpinnerColorControllerDisabler", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/rainbowSpinnerColorControllerDisable", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/ReversibleRetentionBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/MaxHelpingHand/reversibleRetentionBooster/booster00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/RotatingBumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/Bumper/Idle22", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("MaxHelpingHand/SecretBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/moonBerry/normal00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SeekerBarrierColorController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SeekerBarrierColorControllerDisabler", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/rainbowSpinnerColorControllerDisable", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SetFlagOnActionController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/set_flag_on_action", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SetFlagOnButtonPressController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/set_flag_on_button", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SetFlagOnCompletionController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SetFlagOnFullClearController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SetFlagOnHeartCollectedController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SetFlagOnSpawnController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/SidewaysLava", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/rising_lava", justification: None, rotation: Some(1.5707964) },]));
    textures.insert("MaxHelpingHand/StaticPuffer", RenderMethod::Textures(vec![RenderTexture { texture: "objects/puffer/idle00", justification: None, rotation: None },]));
    textures.insert("MaxHelpingHand/StylegroundFadeController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("MemorialHelper/FlagCrystalHeart", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/white00", justification: None, rotation: None },]));
    textures.insert("MemorialHelper/ParallaxText", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 64), border: Color::from_rgba8(255, 255, 255, 192) });
    textures.insert("MoreDasheline/SpecialRefill", RenderMethod::Textures(vec![RenderTexture { texture: "moreDasheline/refill/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("NerdHelper/BouncyJellyfish", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glider/idle0", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ParrotHelper/FlagBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: None, rotation: None },]));
    textures.insert("ParrotHelper/FlagBerryGold", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/goldberry/idle00", justification: None, rotation: None },]));
    textures.insert("ParrotHelper/FlagBerryMoon", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/moonBerry/normal00", justification: None, rotation: None },]));
    textures.insert("PlatinumStrawberry/PlatinumBadelineBoost", RenderMethod::Textures(vec![RenderTexture { texture: "objects/badelineboost/idle00", justification: None, rotation: None },]));
    textures.insert("PlatinumStrawberry/PlatinumStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "SyrenyxPlatinumStrawberry/collectables/platinumberry/plat00", justification: None, rotation: None },]));
    textures.insert("PrismaticHelper/StylegroundsPanel", RenderMethod::Rect { fill: Color::from_rgba8(51, 153, 153, 153), border: Color::from_rgba8(51, 153, 153, 102) });
    textures.insert("PrismaticHelper/WorldPanel", RenderMethod::Rect { fill: Color::from_rgba8(128, 102, 153, 153), border: Color::from_rgba8(51, 38, 64, 102) });
    textures.insert("ReverseHelper/AnotherPurpleBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/VortexHelper/slingBooster/slingBooster00", justification: None, rotation: None },]));
    textures.insert("ReverseHelper/CornerBoostArea", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 25), border: Color::from_rgba8(255, 255, 255, 102) });
    textures.insert("ReverseHelper/CustomInvisibleBarrier", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 51), border: Color::from_rgba8(255, 255, 255, 51) });
    textures.insert("ReverseHelper/DreamToggle", RenderMethod::Textures(vec![RenderTexture { texture: "objects/ReverseHelper/DreamToggleSwitch/switch01", justification: None, rotation: None },]));
    textures.insert("ReverseHelper/ForceyJellyfish", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glider/idle0", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ReverseHelper/HoldableRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/refill/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ReverseHelper/SaferFireIceBarrier", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 25), border: Color::from_rgba8(255, 255, 255, 102) });
    textures.insert("ReverseHelper/ZiplineZipmover", RenderMethod::Textures(vec![RenderTexture { texture: "isafriend/objects/zipline/handle", justification: None, rotation: None },]));
    textures.insert("SJ2021/MaskedOutline", RenderMethod::Textures(vec![RenderTexture { texture: "objects/SJ2021/maskedOutlineController", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("SSMHelper/CrystalBombBadelineBoss", RenderMethod::Textures(vec![RenderTexture { texture: "objects/SSMHelper/crystalBombBadelineBoss/charge00", justification: None, rotation: None },]));
    textures.insert("SSMHelper/DelayedUltraIndicatorController", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/SSMHelper/dultraindicatorcontroller", justification: None, rotation: None },]));
    textures.insert("SSMHelper/ForceCassetteBlockController", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/SSMHelper/forcecassetteblockcontroller", justification: None, rotation: None },]));
    textures.insert("SSMHelper/MovingSolidThingy", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 255), border: Color::from_rgba8(255, 255, 255, 255) });
    textures.insert("SSMHelper/ResizableDashSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/SSMHelper/bigDashSwitch/bigSwitch00", justification: None, rotation: None },]));
    textures.insert("SSMHelper/ZeroGravBoundsController", RenderMethod::Textures(vec![RenderTexture { texture: "loenn/SSMHelper/zerogravcontroller", justification: None, rotation: None },]));
    textures.insert("SaladimHelper/BitsMagicLantern", RenderMethod::Textures(vec![RenderTexture { texture: "SaladimHelper/entities/bitsMagicLantern/static0", justification: None, rotation: None },]));
    textures.insert("SaladimHelper/BitsMagicLanternController", RenderMethod::Textures(vec![RenderTexture { texture: "SaladimHelper/entities/bitsMagicLantern/controller", justification: None, rotation: None },]));
    textures.insert("SaladimHelper/CollectableCoin", RenderMethod::Textures(vec![RenderTexture { texture: "SaladimHelper/entities/collectableCoin/idle00", justification: None, rotation: None },]));
    textures.insert("SaladimHelper/CustomAscendManager", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/summit_background_manager", justification: None, rotation: None },]));
    textures.insert("SaladimHelper/DelayedFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("ShrimpHelper/BonkKrill", RenderMethod::Textures(vec![RenderTexture { texture: "krill/SC2023/ShrimpHelper/asset/loennThingImSorry2", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ShrimpHelper/ShreoGate", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 255), border: Color::from_rgba8(255, 255, 255, 255) });
    textures.insert("ShrimpHelper/Sprimp", RenderMethod::Textures(vec![RenderTexture { texture: "sprimp/SC2023/ShrimpHelper/asset/dissipate", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("ShroomHelper/CrumbleBlockOnTouch", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("ShroomHelper/DoubleRefillBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/sh_doublerefillbooster/boosterPink00", justification: None, rotation: None },]));
    textures.insert("ShroomHelper/OneDashWingedStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/ghostgoldberry/wings01", justification: None, rotation: None },]));
    textures.insert("ShroomHelper/RealityDistortionField", RenderMethod::Rect { fill: Color::from_rgba8(0, 0, 255, 255), border: Color::from_rgba8(0, 0, 255, 255) });
    textures.insert("ShroomHelper/ShroomBookInteraction", RenderMethod::Rect { fill: Color::from_rgba8(106, 13, 173, 255), border: Color::from_rgba8(106, 13, 173, 255) });
    textures.insert("ShroomHelper/ShroomDashSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/sh_dashswitch/dashButtonMirror00", justification: Some((0.5, 0.5)), rotation: None },]));
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
    textures.insert("SorbetHelper/KillZone", RenderMethod::Rect { fill: Color::from_rgba8(176, 99, 100, 76), border: Color::from_rgba8(145, 59, 95, 179) });
    textures.insert("SpekioToolbox/LinkedDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("SpekioToolbox/ProjectileBlockField", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 102, 51), border: Color::from_rgba8(255, 255, 102, 255) });
    textures.insert("SpekioToolbox/ToggleTouchSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/touchswitch/container", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/touchswitch/icon00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("SummitBackgroundManager", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/summit_background_manager", justification: None, rotation: None },]));
    textures.insert("TeraHelper/activeTera", RenderMethod::Textures(vec![RenderTexture { texture: "TeraHelper/objects/tera/Block/Any", justification: None, rotation: None },]));
    textures.insert("TeraHelper/teraBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/booster/booster00", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "TeraHelper/objects/tera/Block/Normal", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("TeraHelper/teraRefill", RenderMethod::Textures(vec![RenderTexture { texture: "TeraHelper/objects/tera/Block/Normal", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("TeraHelper/teraTouchSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/touchswitch/container", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "TeraHelper/objects/tera/TouchSwitch/Normal00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("TheoJelly", RenderMethod::Textures(vec![RenderTexture { texture: "objects/TheoJelly/idle0", justification: None, rotation: None },]));
    textures.insert("VivHelper/BumperWrapper", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/bumperWrapper", justification: None, rotation: None },]));
    textures.insert("VivHelper/CustomCoreMessage", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/core_message", justification: None, rotation: None },]));
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
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("VivHelper/CustomPlaybackWatchtower", RenderMethod::Textures(vec![RenderTexture { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("VivHelper/CustomTorch", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/torch/grayTorchUnlit", justification: None, rotation: None },]));
    textures.insert("VivHelper/DashBumper", RenderMethod::Textures(vec![RenderTexture { texture: "VivHelper/dashBumper/idle00", justification: None, rotation: None },]));
    textures.insert("VivHelper/DebrisLimiter", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/DebrisLimiter", justification: None, rotation: None },]));
    textures.insert("VivHelper/EarlyFlagSetter", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/flagBeforeAwake", justification: None, rotation: None },]));
    textures.insert("VivHelper/EnergyCrystal", RenderMethod::Textures(vec![RenderTexture { texture: "VivHelper/entities/gem", justification: None, rotation: None },]));
    textures.insert("VivHelper/EnterBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VivHelper/EvilBumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/Bumper/Evil22", justification: None, rotation: None },]));
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
    textures.insert("VivHelper/FollowTorch", RenderMethod::Textures(vec![RenderTexture { texture: "FollowTorchSprites/ThorcVar/DefaultTorch00", justification: None, rotation: None },]));
    textures.insert("VivHelper/GoldenBerryToFlag", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/GoldenBerryToFlag", justification: None, rotation: None },]));
    textures.insert("VivHelper/HideRoomInMap", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/HiddenRoom", justification: None, rotation: None },]));
    textures.insert("VivHelper/PinkBooster", RenderMethod::Textures(vec![RenderTexture { texture: "VivHelper/boosters/boosterPink00", justification: None, rotation: None },]));
    textures.insert("VivHelper/PreviousBerriesToFlag", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/PrevBerriesToFlag", justification: None, rotation: None },]));
    textures.insert("VivHelper/RedDashRefill", RenderMethod::Textures(vec![RenderTexture { texture: "VivHelper/redDashRefill/redIdle00", justification: None, rotation: None },]));
    textures.insert("VivHelper/RefillPotion", RenderMethod::Textures(vec![RenderTexture { texture: "VivHelper/Potions/PotRefill00", justification: None, rotation: None },]));
    textures.insert("VivHelper/RefilllessBumper", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/VivHelper/norefillBumper", justification: None, rotation: None },]));
    textures.insert("VivHelper/WarpDashRefill", RenderMethod::Textures(vec![RenderTexture { texture: "VivHelper/TSStelerefill/idle00", justification: None, rotation: None },]));
    textures.insert("VortexHelper/AutoFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("VortexHelper/BowlPuffer", RenderMethod::Textures(vec![RenderTexture { texture: "objects/VortexHelper/pufferBowl/idle00", justification: None, rotation: None },]));
    textures.insert("VortexHelper/DashBubble", RenderMethod::Textures(vec![RenderTexture { texture: "objects/VortexHelper/dashBubble/idle00", justification: None, rotation: None },]));
    textures.insert("VortexHelper/PufferBarrier", RenderMethod::Rect { fill: Color::from_rgba8(255, 189, 74, 180), border: Color::from_rgba8(255, 189, 74, 180) });
    textures.insert("VortexHelper/PurpleBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/VortexHelper/slingBooster/slingBooster00", justification: None, rotation: None },]));
    textures.insert("VortexHelper/VortexCustomBumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/VortexHelper/vortexCustomBumper/green22", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/BreakBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CellLock", RenderMethod::Textures(vec![RenderTexture { texture: "objects/XaphanHelper/CellLock/normal00", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/XaphanHelper/CellLock/blue00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("XaphanHelper/CustomBadelineBoss", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badelineBoss/charge00", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/CustomCheckpoint", RenderMethod::Textures(vec![RenderTexture { texture: "objects/XaphanHelper/CustomCheckpoint/bg00", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/CustomCoverupWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
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
    textures.insert("XaphanHelper/CustomEndScreenController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/customEndScreenController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/CustomExitBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CustomFakeWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/CustomTorch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/XaphanHelper/CustomTorch/torch00", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/Elevator", RenderMethod::Textures(vec![RenderTexture { texture: "objects/XaphanHelper/Elevator/elevator00", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/ElevatorBarrier", RenderMethod::Rect { fill: Color::from_rgba8(102, 102, 102, 204), border: Color::from_rgba8(0, 0, 0, 0) });
    textures.insert("XaphanHelper/FlagBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("XaphanHelper/FlagDashSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/temple/dashButtonMirror00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("XaphanHelper/HeatController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/heatController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/InGameMapController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/mapController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/InGameMapHintController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/hintController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/InGameMapRoomAdjustController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/roomAdjustController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/InGameMapRoomController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/roomController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/InGameMapSubAreaController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/subAreaController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/InGameMapTilesController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/tilesController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/JumpBlocksFlipSoundController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/sound_source", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/MergeChaptersController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/mergeChaptersController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/SetStatsFlagsController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/setStatsFlagsController ", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/TeleportToOtherSidePortal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/XaphanHelper/TeleportToOtherSidePortal/A-Side00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("XaphanHelper/TimedDashSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/temple/dashButtonMirror00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("XaphanHelper/TimedStrawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/TimedTempleGate", RenderMethod::Textures(vec![RenderTexture { texture: "objects/door/TempleDoorB00", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/TimerRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/XaphanHelper/TimerRefill/idle00", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/UpgradeController", RenderMethod::Textures(vec![RenderTexture { texture: "util/XaphanHelper/Loenn/upgradeController", justification: None, rotation: None },]));
    textures.insert("XaphanHelper/WarpStation", RenderMethod::Textures(vec![RenderTexture { texture: "objects/XaphanHelper/WarpStation/idle00", justification: None, rotation: None },]));
    textures.insert("YetAnotherHelper/BubbleField", RenderMethod::Rect { fill: Color::from_rgba8(0, 0, 255, 102), border: Color::from_rgba8(255, 255, 255, 128) });
    textures.insert("YetAnotherHelper/FlagKillBarrier", RenderMethod::Rect { fill: Color::from_rgba8(202, 97, 97, 153), border: Color::from_rgba8(202, 81, 76, 179) });
    textures.insert("YetAnotherHelper/SpikeJumpThruController", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/YetAnotherHelper/spikeJumpThruController", justification: None, rotation: None },]));
    textures.insert("YetAnotherHelper/StickyJellyfish", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn/YetAnotherHelper/stickyJellyfish", justification: None, rotation: None },]));
    textures.insert("badelineBoost", RenderMethod::Textures(vec![RenderTexture { texture: "objects/badelineboost/idle00", justification: None, rotation: None },]));
    textures.insert("batteries/battery", RenderMethod::Textures(vec![RenderTexture { texture: "batteries/battery/full0", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("batteries/power_refill", RenderMethod::Textures(vec![RenderTexture { texture: "batteries/power_refill/idle00", justification: None, rotation: None },]));
    textures.insert("batteries/recharge_platform", RenderMethod::Textures(vec![RenderTexture { texture: "batteries/recharge_platform/base0", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("bgSwitch/bgModeToggle", RenderMethod::Textures(vec![RenderTexture { texture: "objects/BGswitch/bgflipswitch/switch01", justification: None, rotation: None },]));
    textures.insert("bigSpinner", RenderMethod::Textures(vec![RenderTexture { texture: "objects/Bumper/Idle22", justification: None, rotation: None },]));
    textures.insert("bird", RenderMethod::Textures(vec![RenderTexture { texture: "characters/bird/crow00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("birdPath", RenderMethod::Textures(vec![RenderTexture { texture: "characters/bird/flyup00", justification: None, rotation: None },]));
    textures.insert("blackGem", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/0/00", justification: None, rotation: None },]));
    textures.insert("blockField", RenderMethod::Rect { fill: Color::from_rgba8(102, 102, 255, 102), border: Color::from_rgba8(102, 102, 255, 255) });
    textures.insert("bonfire", RenderMethod::Textures(vec![RenderTexture { texture: "objects/campfire/fire08", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("booster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/booster/booster00", justification: None, rotation: None },]));
    textures.insert("brokemiahelper/cassetteCassette", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/cassette/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("brokemiahelper/cassetteDreamBlock", RenderMethod::Rect { fill: Color::from_rgba8(0, 0, 0, 255), border: Color::from_rgba8(73, 170, 240, 255) });
    textures.insert("brokemiahelper/cassetteIntroCar", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/car/body", justification: Some((0.5, 1.0)), rotation: None },RenderTexture { texture: "scenery/car/wheels", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("brokemiahelper/cassetteSpinner", RenderMethod::Textures(vec![RenderTexture { texture: "danger/crystal/fg_white00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("canyon/pushblock", RenderMethod::Textures(vec![RenderTexture { texture: "objects/canyon/pushblock/idle", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("canyon/spinorb", RenderMethod::Textures(vec![RenderTexture { texture: "objects/canyon/spinorb/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("cassette", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/cassette/idle00", justification: None, rotation: None },]));
    textures.insert("cavern/crystalBombField", RenderMethod::Rect { fill: Color::from_rgba8(115, 0, 115, 204), border: Color::from_rgba8(115, 0, 115, 204) });
    textures.insert("cavern/fakecavernheart", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/0/00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("cliffside_flag", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/cliffside/flag00", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("cloud", RenderMethod::Textures(vec![RenderTexture { texture: "objects/clouds/cloud00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("clutterDoor", RenderMethod::Rect { fill: Color::from_rgba8(74, 71, 135, 255), border: Color::from_rgba8(255, 255, 255, 255) });
    textures.insert("conditionBlock", RenderMethod::FakeTiles {
        material_key: "tileType",
        blend_key: true,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("coreMessage", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/core_message", justification: None, rotation: None },]));
    textures.insert("coreModeToggle", RenderMethod::Textures(vec![RenderTexture { texture: "objects/coreFlipSwitch/switch01", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/ForceInteractionsController", RenderMethod::Textures(vec![RenderTexture { texture: "corkr900/CoopHelper/InteractionsController/icon00", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/GroupButton", RenderMethod::Textures(vec![RenderTexture { texture: "corkr900/CoopHelper/GroupSwitch/button00", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SessionPicker", RenderMethod::Textures(vec![RenderTexture { texture: "corkr900/CoopHelper/SessionPicker/idle00", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedBooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/booster/booster00", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedCloud", RenderMethod::Textures(vec![RenderTexture { texture: "objects/clouds/cloud00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedCoreModeToggle", RenderMethod::Textures(vec![RenderTexture { texture: "objects/coreFlipSwitch/switch01", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedDashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("corkr900CoopHelper/SyncedFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("corkr900CoopHelper/SyncedKey", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/key/idle00", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedLightningBreakerBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/breakerBox/Idle00", justification: Some((0.25, 0.25)), rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedPuffer", RenderMethod::Textures(vec![RenderTexture { texture: "objects/puffer/idle00", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedRefill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/refill/idle00", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedSeeker", RenderMethod::Textures(vec![RenderTexture { texture: "characters/monsters/predator73", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedSummitBackgroundManager", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/summit_background_manager", justification: None, rotation: None },]));
    textures.insert("corkr900CoopHelper/SyncedTouchSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/touchswitch/container", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/touchswitch/icon00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("coverupWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("cpopBlock", RenderMethod::Textures(vec![RenderTexture { texture: "cpopBlock", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("crumbleWallOnRumble", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("cutsceneNode", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/cutscene_node", justification: None, rotation: None },]));
    textures.insert("darkChaser", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("darkChaserEnd", RenderMethod::Rect { fill: Color::from_rgba8(102, 0, 102, 102), border: Color::from_rgba8(102, 0, 102, 255) });
    textures.insert("dashBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("dreamBlock", RenderMethod::Rect { fill: Color::from_rgba8(0, 0, 0, 255), border: Color::from_rgba8(255, 255, 255, 255) });
    textures.insert("dreamHeartGem", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/0/00", justification: None, rotation: None },]));
    textures.insert("dreammirror", RenderMethod::Textures(vec![RenderTexture { texture: "objects/mirror/frame", justification: Some((0.5, 1.0)), rotation: None },RenderTexture { texture: "objects/mirror/glassbreak00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("everest/coreMessage", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/core_message", justification: None, rotation: None },]));
    textures.insert("everest/customBirdTutorial", RenderMethod::Textures(vec![RenderTexture { texture: "characters/bird/crow00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("everest/memorial", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("everest/npc", RenderMethod::Textures(vec![RenderTexture { texture: "characters/00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("everest/starClimbGraphicsController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("exitBlock", RenderMethod::FakeTiles {
        material_key: "tileType",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("eyebomb", RenderMethod::Textures(vec![RenderTexture { texture: "objects/puffer/idle00", justification: None, rotation: None },]));
    textures.insert("fakeBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
        x: None,
        y: None,
    });
    textures.insert("fakeHeart", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/heartGem/0/00", justification: None, rotation: None },]));
    textures.insert("fakeWall", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: true,
        layer: Some("tilesFg"),
        color: Some(Color::from_rgba8(255, 255, 255, 179)),
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
    textures.insert("finalBoss", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badelineBoss/charge00", justification: None, rotation: None },]));
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
    textures.insert("fireBall", RenderMethod::Textures(vec![RenderTexture { texture: "objects/fireball/fireball01", justification: None, rotation: None },]));
    textures.insert("fireBarrier", RenderMethod::Rect { fill: Color::from_rgba8(209, 9, 1, 102), border: Color::from_rgba8(246, 98, 18, 255) });
    textures.insert("flingBird", RenderMethod::Textures(vec![RenderTexture { texture: "characters/bird/Hover04", justification: None, rotation: None },]));
    textures.insert("flingBirdIntro", RenderMethod::Textures(vec![RenderTexture { texture: "characters/bird/Hover04", justification: None, rotation: None },]));
    textures.insert("flutterbird", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/flutterbird/idle00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("foregroundDebris", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/fgdebris/rock_b00", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "scenery/fgdebris/rock_b01", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("friendlyGhost", RenderMethod::Textures(vec![RenderTexture { texture: "characters/oshiro/boss13", justification: None, rotation: None },]));
    textures.insert("glassBlock", RenderMethod::Rect { fill: Color::from_rgba8(255, 255, 255, 153), border: Color::from_rgba8(255, 255, 255, 204) });
    textures.insert("glider", RenderMethod::Textures(vec![RenderTexture { texture: "objects/glider/idle0", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("goldenBerry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/goldberry/idle00", justification: None, rotation: None },]));
    textures.insert("iceBlock", RenderMethod::Rect { fill: Color::from_rgba8(76, 168, 214, 102), border: Color::from_rgba8(108, 214, 235, 255) });
    textures.insert("introCar", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/car/body", justification: Some((0.5, 1.0)), rotation: None },RenderTexture { texture: "scenery/car/wheels", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("introCrusher", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("invisibleBarrier", RenderMethod::Rect { fill: Color::from_rgba8(102, 102, 102, 204), border: Color::from_rgba8(0, 0, 0, 0) });
    textures.insert("key", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/key/idle00", justification: None, rotation: None },]));
    textures.insert("lightning", RenderMethod::Rect { fill: Color::from_rgba8(140, 248, 245, 102), border: Color::from_rgba8(253, 245, 120, 255) });
    textures.insert("lightningBlock", RenderMethod::Textures(vec![RenderTexture { texture: "objects/breakerBox/Idle00", justification: Some((0.25, 0.25)), rotation: None },]));
    textures.insert("luaCutscenes/luaTalker", RenderMethod::Rect { fill: Color::from_rgba8(0, 255, 255, 102), border: Color::from_rgba8(0, 255, 255, 255) });
    textures.insert("memorial", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("memorialTextController", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/goldberry/wings01", justification: None, rotation: None },]));
    textures.insert("moonCreature", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/moon_creatures/tiny05", justification: None, rotation: None },]));
    textures.insert("negaBlock", RenderMethod::Rect { fill: Color::from_rgba8(255, 0, 0, 255), border: Color::from_rgba8(255, 0, 0, 255) });
    textures.insert("outback/movingtouchswitch", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/outback/movingtouchswitch/container", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "collectables/outback/movingtouchswitch/icon00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("outback/portal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/outback/portal/idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("pandorasBox/airBubbles", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/airBubbles/idle00", justification: None, rotation: None },]));
    textures.insert("pandorasBox/checkpoint", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/checkpoint/flag/active_idle00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("pandorasBox/dashToggleBlock", RenderMethod::Rect { fill: Color::from_rgba8(204, 76, 255, 102), border: Color::from_rgba8(204, 76, 255, 255) });
    textures.insert("pandorasBox/dreamDashController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/controllerIcons/dreamDashController", justification: None, rotation: None },]));
    textures.insert("pandorasBox/dustSpriteColorController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/controllerIcons/dustSpriteColorController", justification: None, rotation: None },]));
    textures.insert("pandorasBox/entityActivator", RenderMethod::Rect { fill: Color::from_rgba8(255, 179, 179, 102), border: Color::from_rgba8(179, 179, 255, 255) });
    textures.insert("pandorasBox/flagToggleSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/flagToggleSwitch/switch01", justification: None, rotation: None },]));
    textures.insert("pandorasBox/gate", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/gate/gate0", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("pandorasBox/introCar", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/car/body", justification: Some((0.5, 1.0)), rotation: None },RenderTexture { texture: "scenery/car/wheels", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("pandorasBox/lamp", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/lamp/base", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/pandorasBox/lamp/idle0", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("pandorasBox/laserEmitter", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/laser/emitter/idle0", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("pandorasBox/laserNoteBlock", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/laser/noteblock/noteblock_horizontal", justification: None, rotation: None },]));
    textures.insert("pandorasBox/laserSensor", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/laser/sensor/metal_ring", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/pandorasBox/laser/sensor/light_ring", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/pandorasBox/laser/sensor/orb", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("pandorasBox/lever", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/lever/lever0", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("pandorasBox/pandorasBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/pandorasBox/box_idle0", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("pandorasBox/playerClone", RenderMethod::Textures(vec![RenderTexture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("pandorasBox/propellerBox", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/propellerBox/default/default_charges00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("pandorasBox/shell", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/shells/koopa/shell_idle00", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/pandorasBox/shells/koopa/deco_idle00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("pandorasBox/timefield", RenderMethod::Rect { fill: Color::from_rgba8(128, 255, 255, 102), border: Color::from_rgba8(128, 255, 255, 255) });
    textures.insert("pandorasBox/waterDrowningController", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pandorasBox/controllerIcons/waterDrowningController", justification: None, rotation: None },]));
    textures.insert("payphone", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/payphone", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("picoconsole", RenderMethod::Textures(vec![RenderTexture { texture: "objects/pico8Console", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("plateau", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/fallplateau", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("playbackTutorial", RenderMethod::Textures(vec![RenderTexture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("player", RenderMethod::Textures(vec![RenderTexture { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("playerSeeker", RenderMethod::Textures(vec![RenderTexture { texture: "decals/5-temple/statue_e", justification: None, rotation: None },]));
    textures.insert("powerSourceNumber", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/powersource_numbers/1", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "scenery/powersource_numbers/1_glow", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("progHelper/adjustableFallingBlock", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("progHelper/linearIntroCrusher", RenderMethod::FakeTiles {
        material_key: "tiletype",
        blend_key: false,
        layer: None,
        color: None,
        x: None,
        y: None,
    });
    textures.insert("quizController", RenderMethod::Textures(vec![RenderTexture { texture: "quizController", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("refill", RenderMethod::Textures(vec![RenderTexture { texture: "objects/refill/idle00", justification: None, rotation: None },]));
    textures.insert("reflectionHeartStatue", RenderMethod::Textures(vec![RenderTexture { texture: "objects/reflectionHeart/statue", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("ridgeGate", RenderMethod::Textures(vec![RenderTexture { texture: "objects/ridgeGate", justification: Some((0.0, 0.0)), rotation: None },]));
    textures.insert("risingLava", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/rising_lava", justification: None, rotation: None },]));
    textures.insert("rotateSpinner", RenderMethod::Textures(vec![RenderTexture { texture: "danger/blade00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("sandwichLava", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/lava_sandwich", justification: None, rotation: None },]));
    textures.insert("seeker", RenderMethod::Textures(vec![RenderTexture { texture: "characters/monsters/predator73", justification: None, rotation: None },]));
    textures.insert("seekerBarrier", RenderMethod::Rect { fill: Color::from_rgba8(64, 64, 64, 204), border: Color::from_rgba8(64, 64, 64, 204) });
    textures.insert("seekerStatue", RenderMethod::Textures(vec![RenderTexture { texture: "decals/5-temple/statue_e", justification: None, rotation: None },]));
    textures.insert("spring", RenderMethod::Textures(vec![RenderTexture { texture: "objects/spring/00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("starClimbController", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/northern_lights", justification: None, rotation: None },]));
    textures.insert("strawberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/strawberry/normal00", justification: None, rotation: None },]));
    textures.insert("summitGemManager", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/summit_gem_manager", justification: None, rotation: None },]));
    textures.insert("summitcloud", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/summitclouds/cloud00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("summitgem", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/summitgems/0/gem00", justification: None, rotation: None },]));
    textures.insert("tentacles", RenderMethod::Textures(vec![RenderTexture { texture: "@Internal@/tentacles", justification: None, rotation: None },]));
    textures.insert("theoCrystalPedestal", RenderMethod::Textures(vec![RenderTexture { texture: "characters/theoCrystal/pedestal", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("torch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/temple/torch00", justification: None, rotation: None },]));
    textures.insert("touchSwitch", RenderMethod::Textures(vec![RenderTexture { texture: "objects/touchswitch/container", justification: Some((0.5, 0.5)), rotation: None },RenderTexture { texture: "objects/touchswitch/icon00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("towerviewer", RenderMethod::Textures(vec![RenderTexture { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("trackSpinner", RenderMethod::Textures(vec![RenderTexture { texture: "danger/blade00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("vitellary/boostbumper", RenderMethod::Textures(vec![RenderTexture { texture: "objects/boostBumper/booster00", justification: None, rotation: None },]));
    textures.insert("vitellary/cassetteflags", RenderMethod::Textures(vec![RenderTexture { texture: "CrystallineHelper/FLCC/ahorn_cassetteflagcontroller", justification: None, rotation: None },]));
    textures.insert("vitellary/custompuffer", RenderMethod::Textures(vec![RenderTexture { texture: "objects/puffer/idle00", justification: None, rotation: None },]));
    textures.insert("vitellary/dashcodecontroller", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn_dashcodecontroller", justification: None, rotation: None },]));
    textures.insert("vitellary/energybooster", RenderMethod::Textures(vec![RenderTexture { texture: "objects/CrystallineHelper/FLCC/energyBooster/booster00", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("vitellary/fillcrystal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/crystals/fill/idle00", justification: None, rotation: None },]));
    textures.insert("vitellary/flagsequencecontroller", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn_flagsequencecontroller", justification: None, rotation: None },]));
    textures.insert("vitellary/goodtelecrystal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/crystals/tele/right/idle00", justification: None, rotation: None },]));
    textures.insert("vitellary/interactivechaser", RenderMethod::Textures(vec![RenderTexture { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("vitellary/keyberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/keyberry/normal03", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("vitellary/lockedintrocar", RenderMethod::Textures(vec![RenderTexture { texture: "scenery/car/body", justification: Some((0.5, 1.0)), rotation: None },RenderTexture { texture: "scenery/car/wheels", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("vitellary/returnkeyberry", RenderMethod::Textures(vec![RenderTexture { texture: "collectables/keyberry/normal03", justification: Some((0.5, 0.5)), rotation: None },]));
    textures.insert("vitellary/roomname", RenderMethod::Textures(vec![RenderTexture { texture: "ahorn_roomname", justification: None, rotation: None },]));
    textures.insert("vitellary/starcrystal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/crystals/star/idle00", justification: None, rotation: None },]));
    textures.insert("vitellary/timecrystal", RenderMethod::Textures(vec![RenderTexture { texture: "objects/crystals/time/idle00", justification: None, rotation: None },]));
    textures.insert("wallSpringLeft", RenderMethod::Textures(vec![RenderTexture { texture: "objects/spring/00", justification: Some((0.5, 1.0)), rotation: Some(1.5707964) },]));
    textures.insert("wallSpringRight", RenderMethod::Textures(vec![RenderTexture { texture: "objects/spring/00", justification: Some((0.5, 1.0)), rotation: Some(-1.5707964) },]));
    textures.insert("wavedashmachine", RenderMethod::Textures(vec![RenderTexture { texture: "objects/wavedashtutorial/building_back", justification: Some((0.5, 1.0)), rotation: None },RenderTexture { texture: "objects/wavedashtutorial/building_front_left", justification: Some((0.5, 1.0)), rotation: None },RenderTexture { texture: "objects/wavedashtutorial/building_front_right", justification: Some((0.5, 1.0)), rotation: None },]));
    textures.insert("whiteblock", RenderMethod::Textures(vec![RenderTexture { texture: "objects/whiteblock", justification: Some((0.0, 0.0)), rotation: None },]));

    textures
}
