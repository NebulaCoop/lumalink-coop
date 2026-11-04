# Objects

Every name the game can make, as a level places it or `spawn` asks for it. Written from the game's object table. `lumalink list objects` prints the names.

- **Class** is what a thing of that name is. For `object_classes`, `like` is any of these names: a new name made the same way, with its own files.
- **Archive** is `ObjectData/<archive>.arc`. Empty: it has no archive of its own, or it's chosen when placed.

## Areas: invisible shapes that do something

| Name | Class | Archive |
|---|---|---|
| `SwitchCube` | SwitchArea |  |
| `SwitchSphere` | SwitchArea |  |
| `SwitchCylinder` | SwitchArea |  |
| `CubeCameraBox` | CubeCameraArea |  |
| `CubeCameraCylinder` | CubeCameraArea |  |
| `CubeCameraSphere` | CubeCameraArea |  |
| `CubeCameraBowl` | CubeCameraArea |  |
| `EffectCylinder` | AreaObj |  |
| `BindEndCube` | AreaObj |  |
| `DeathCube` | DeathArea |  |
| `DeathSphere` | DeathArea |  |
| `DeathCylinder` | DeathArea |  |
| `PullBackCube` | AreaObj |  |
| `PullBackCylinder` | AreaObj |  |
| `RestartCube` | RestartCube |  |
| `WarpCube` | WarpCube |  |
| `TripodBossStepStartArea` | TripodBossStepStartArea |  |
| `ChangeBgmCube` | ChangeBgmCube |  |
| `BgmProhibitArea` | BgmProhibitArea |  |
| `SoundEmitterCube` | SoundEmitterCube |  |
| `SoundEmitterSphere` | SoundEmitterSphere |  |
| `WaterCube` | WaterArea |  |
| `WaterSphere` | WaterArea |  |
| `WaterCylinder` | WaterArea |  |
| `FallsCube` | AreaObj |  |
| `PlaneModeCube` | AreaObj |  |
| `PlaneCircularModeCube` | AreaObj |  |
| `PipeModeCube` | AreaObj |  |
| `TowerModeCylinder` | AreaObj |  |
| `BloomCube` | BloomArea |  |
| `BloomSphere` | BloomArea |  |
| `BloomCylinder` | BloomArea |  |
| `SimpleBloomCube` | SimpleBloomArea |  |
| `SimpleBloomSphere` | SimpleBloomArea |  |
| `SimpleBloomCylinder` | SimpleBloomArea |  |
| `ScreenBlurCube` | ScreenBlurArea |  |
| `ScreenBlurSphere` | ScreenBlurArea |  |
| `ScreenBlurCylinder` | ScreenBlurArea |  |
| `DepthOfFieldCube` | DepthOfFieldArea |  |
| `DepthOfFieldSphere` | DepthOfFieldArea |  |
| `DepthOfFieldCylinder` | DepthOfFieldArea |  |
| `LightCtrlCube` | LightArea |  |
| `LightCtrlCylinder` | LightArea |  |
| `PlaneCollisionCube` | AreaObj |  |
| `ForbidTriangleJumpCube` | AreaObj |  |
| `ForbidWaterSearchCube` | AreaObj |  |
| `QuakeEffectAreaCube` | QuakeEffectArea |  |
| `HazeCube` | HazeCube |  |
| `LensFlareArea` | AreaObj |  |
| `CameraRepulsiveSphere` | CameraRepulsiveSphere |  |
| `CameraRepulsiveCylinder` | CameraRepulsiveCylinder |  |
| `SunLightAreaBox` | SunLightArea |  |
| `GlaringLightAreaCylinder` | GlaringLightArea |  |
| `BigBubbleGoalAreaBox` | BigBubbleGoalArea |  |
| `BigBubbleGoalAreaCylinder` | BigBubbleGoalArea |  |
| `BigBubbleGoalAreaSphere` | BigBubbleGoalArea |  |
| `BigBubbleCameraBox` | BigBubbleCameraArea |  |
| `BigBubbleCameraCylinder` | BigBubbleCameraArea |  |
| `BigBubbleCameraSphere` | BigBubbleCameraArea |  |
| `BigBubbleSwitchBox` | BigBubbleSwitchArea |  |
| `BigBubbleSwitchCylinder` | BigBubbleSwitchArea |  |
| `BigBubbleSwitchSphere` | BigBubbleSwitchArea |  |
| `ViewGroupCtrlCube` | AreaObj |  |
| `AudioEffectCube` | AudioEffectArea |  |
| `AudioEffectSphere` | AudioEffectArea |  |
| `AudioEffectCylinder` | AudioEffectArea |  |
| `PlayerSeCube` | PlayerSeArea |  |
| `PlayerSeSphere` | PlayerSeArea |  |
| `PlayerSeCylinder` | PlayerSeArea |  |
| `MercatorCube` | MercatorTransformCube |  |
| `AstroChangeStageCube` | AstroChangeStageCube |  |
| `BlueStarGuidanceCube` | AreaObj |  |
| `TicoSeedGuidanceCube` | AreaObj |  |
| `MessageAreaCube` | MessageArea |  |
| `MessageAreaCylinder` | MessageArea |  |
| `SmokeEffectColorAreaCube` | AreaObj |  |
| `BeeWallShortDistAreaCube` | AreaObj |  |
| `ExtraWallCheckArea` | AreaObj |  |
| `ExtraWallCheckCylinder` | AreaObj |  |
| `MirrorAreaCube` | AreaObj |  |
| `SpinGuidanceCube` | SpinGuidanceArea | SpinGuidance |
| `TamakoroMoveGuidanceCube` | SpinGuidanceArea | BallGuidance |
| `TamakoroJumpGuidanceCube` | SpinGuidanceArea | BallGuidance |
| `HeavySteeringCube` | AreaObj |  |
| `NonSleepCube` | AreaObj |  |
| `AreaMoveSphere` | AreaObj |  |
| `DodoryuClosedCylinder` | AreaObj |  |
| `DashChargeCylinder` | AreaObj |  |
| `RasterScrollCube` | AreaObj |  |
| `OnimasuCube` | AreaObj |  |
| `ForbidJumpCube` | AreaObj |  |
| `CollisionArea` | CollisionArea |  |
| `AstroOverlookAreaCylinder` | AreaObj |  |
| `CelestrialSphere` | AreaObj |  |
| `DarkMatterCube` | AreaObj |  |
| `DarkMatterCylinder` | AreaObj |  |

## Characters

| Name | Class | Archive |
|---|---|---|
| `Caretaker` | Caretaker | Caretaker |
| `Rabbit` | Rabbit | MoonRabbit |
| `Kinopio` | Kinopio | Kinopio |
| `KinopioAstro` | KinopioAstro | Kinopio |
| `HoneyBee` | HoneyBee | HoneyBee |
| `HoneyQueen` | HoneyQueen | HoneyQueen |
| `Peach` | Peach | Peach |
| `Penguin` | Penguin | Penguin |
| `PenguinCoach` | PenguinCoach | PenguinCoach |
| `PenguinMaster` | PenguinMaster | PenguinMaster |
| `PenguinRacer` | PenguinRacer | Penguin |
| `PenguinRacerLeader` | PenguinRacerLeader | Penguin |
| `PenguinSkater` | PenguinSkater | Penguin |
| `PenguinStudent` | PenguinStudent | Penguin |
| `SignBoard` | SignBoard | SignBoard |
| `Tico` | Tico |  |
| `TicoComet` | TicoComet | TicoComet |
| `TicoShop` | TicoShop | TicoShop |
| `TicoStarRing` | TicoStarRing | Tico |
| `TicoAstro` | TicoAstro |  |
| `TicoBaby` | Tico |  |
| `DemoRabbit` | DemoRabbit |  |
| `TicoFat` | TicoFat | TicoFat |
| `TicoGalaxy` | TicoGalaxy | TicoFat |
| `Rosetta` | Rosetta |  |
| `LuigiNPC` | LuigiNPC |  |
| `LuigiEvent` | LuigiNPC |  |
| `TeresaRacer` | TeresaRacer | TeresaRacer |
| `Butler` | Butler | Butler |
| `ButlerMap` | ButlerMap | Butler |
| `ButlerExplain` | ButlerExplain | Butler |

## Map objects

| Name | Class | Archive |
|---|---|---|
| `ConcreteBlock` | SimpleBreakableObj | ConcreteBlock |
| `Tsukidashikun` | Tsukidashikun | Tsukidashikun |
| `DesertMovingLand` | DesertMovingLand | DesertMovingLand |
| `DesertMovingLandSmall` | DesertMovingLand | DesertMovingLandSmall |
| `AsteroidA` | SimpleEnvironmentObj | AsteroidA |
| `AsteroidB` | SimpleEnvironmentObj | AsteroidB |
| `AsteroidC` | SimpleEnvironmentObj | AsteroidC |
| `AsteroidD` | SimpleEnvironmentObj | AsteroidD |
| `SpaceStickA` | SimpleEnvironmentObj | SpaceStickA |
| `SpaceStickB` | SimpleEnvironmentObj | SpaceStickB |
| `KillerGunnerDouble` | SimpleEnvironmentObj | KillerGunnerDouble |
| `KillerGunnerTriple` | SimpleEnvironmentObj | KillerGunnerTriple |
| `CollapsePlane` | CollapsePlane | CollapsePlane |
| `SpaceMine` | SpaceMine | SpaceMine |
| `LavaObstacleRockHomeB` | AnmModelSwitchMove | LavaObstacleRockHomeB |
| `LavaHomeVolcanoInnerFlow` | RailMoveObj | LavaHomeVolcanoInnerFlow |
| `LavaRotatePlanetStartStep` | RailMoveObj | LavaRotatePlanetStartStep |
| `LavaBreakBridge` | AnmModelGroundOnMove | LavaBreakBridge |
| `LavaBreakRockA` | AnmModelSwitchMoveEndKill | LavaBreakRockA |
| `LavaBreakRockB` | AnmModelGroundOnMove | LavaBreakRockB |
| `LavaBreakRockC` | AnmModelGroundOnMove | LavaBreakRockC |
| `LavaBreakRockD` | AnmModelGroundOnMove | LavaBreakRockD |
| `LavaBreakColumn` | LavaBreakColumn | LavaBreakColumn |
| `LavaShellTower` | LavaShellTower | LavaShellTower |
| `LavaFallDownColumnA` | AnmModelSwitchMove | LavaFallDownColumnA |
| `LavaFallDownColumnB` | AnmModelSwitchMove | LavaFallDownColumnB |
| `LavaSpaceStickA` | SimpleEnvironmentObj | LavaSpaceStickA |
| `LavaSpaceStickB` | SimpleEnvironmentObj | LavaSpaceStickB |
| `LavaBlackUFO` | SimpleEnvironmentObj | LavaBlackUFO |
| `CrystalCageMoving` | CrystalCageMoving | CrystalCageMoving |
| `SeaBottomBigDoorA` | RailDemoMoveObj | SeaBottomBigDoorA |
| `SeaBottomBigDoorB` | RailDemoMoveObj | SeaBottomBigDoorB |
| `SeaBottomBigDoorC` | RailDemoMoveObj | SeaBottomBigDoorC |
| `FloaterLandPartsFrame` | SimpleMapObj | FloaterLandPartsFrame |
| `PhantomCandlestand` | Candlestand | PhantomCandlestand |
| `CandlestandIceVolcano` | Candlestand | CandlestandIceVolcano |
| `TemplateStageGeometry` | SimpleMapObj | TemplateStageGeometry |
| `SunkenShip` | SimpleBreakableObj | SunkenShip |
| `WaterfallCaveCover` | WaterfallCaveCover | WaterfallCaveCover |
| `WaterfallCaveNoBreakCover` | SimpleMapObj | WaterfallCaveNoBreakCover |
| `OceanPierFloaterA` | OceanWaveFloater | OceanPierFloaterA |
| `HeavenlyBeachTopRock` | AnmModelSwitchMoveEndKill | HeavenlyBeachTopRock |
| `HeavenlyBeachUnderRock` | AnmModelSwitchMove | HeavenlyBeachUnderRock |
| `OceanHexagonFloater` | OceanWaveFloater | OceanHexagonFloater |
| `ShutterDoorB` | RailMoveObj | ShutterDoorB |
| `ShutterDoorC` | AnmModelSwitchMove | ShutterDoorC |
| `ShutterDoorD` | AnmModelSwitchMove | ShutterDoorD |
| `ShutterDoorE` | AnmModelSwitchMove | ShutterDoorE |
| `SeaBottomTriplePropellerStand` | SimpleMapObj | SeaBottomTriplePropellerStand |
| `OceanFloaterTowerRotateStepA` | RotateMoveObj | OceanFloaterTowerRotateStepA |
| `OceanFloaterTowerRotateStepB` | RotateMoveObj | OceanFloaterTowerRotateStepB |
| `OceanFloaterTowerRotateStepC` | RotateMoveObj | OceanFloaterTowerRotateStepC |
| `OceanFloaterTowerRotateStepD` | RotateMoveObj | OceanFloaterTowerRotateStepD |
| `IcicleRock` | SimpleBreakableObj | IcicleRock |
| `FlipPanel` | FlipPanel | FlipPanel |
| `FlipPanelReverse` | FlipPanel | FlipPanelReverse |
| `FlipPanelFrame` | SimpleMapObj | FlipPanelFrame |
| `PhantomTowerMoveStepA` | RailMoveObj | PhantomTowerMoveStepA |
| `SideSpikeMoveStepA` | SideSpikeMoveStep | SideSpikeMoveStepA |
| `SpaceMineRailA` | SimpleMapObj | SpaceMineRailA |
| `SpaceMineRail5m` | SimpleMapObj | SpaceMineRail5m |
| `SandUpDownTowerTop` | SimpleMapObjFarMax | SandUpDownTowerTop |
| `SandUpDownTowerBottom` | SimpleMapObjFarMax | SandUpDownTowerBottom |
| `SandUpDownKillerGunnerBase` | SimpleMapObj | SandUpDownKillerGunnerBase |
| `SandUpDownTriRock` | SandUpDownTriRock | SandUpDownTriRock |
| `CaretakerGarbage` | SimpleMapObj | CaretakerGarbage |
| `GlassBottleTall` | SimpleMapObj | GlassBottleTall |
| `HopperBeltConveyerRotatePartsA` | RotateMoveObj | HopperBeltConveyerRotatePartsA |
| `HopperBeltConveyerMovePartsA` | RailMoveObj | HopperBeltConveyerMovePartsA |
| `SandRiverLightA` | SwingLight | SandRiverLightA |
| `SnowBlockA` | SimpleBreakableObj | SnowBlockA |
| `DangerSignBoard` | SimpleMapObjPush | DangerSignBoard |
| `MirrorReflectionTest` | SimpleMirrorObj | MirrorReflectionTest |
| `MirrorReflectionPeachCastle` | SimpleMirrorObj | MirrorReflectionPeachCastle |
| `MirrorReflectionTwinFallLake` | SimpleMirrorObj | MirrorReflectionTwinFallLake |
| `MirrorModelTest` | SimpleMirrorReflectionObj | MirrorModelTest |
| `MirrorModelPeachCastle` | SimpleMirrorReflectionObj | MirrorModelPeachCastle |
| `MirrorModelTwinFallLake` | SimpleMirrorReflectionObj | MirrorModelTwinFallLake |
| `PhantomFirewood` | SimpleMapObj | PhantomFirewood |
| `ArrowBoard` | SimpleMapObj | ArrowBoard |
| `TeresaRoomGlaringLight` | GlaringLight | TeresaRoomGlaringLight |
| `SpaceDustWoodA` | SimpleEnvironmentObj | SpaceDustWoodA |
| `SpaceDustWoodB` | SimpleEnvironmentObj | SpaceDustWoodB |
| `TeresaRoomDoor` | AnmModelSwitchMoveEventCamera | TeresaRoomDoor |
| `ReverseGravityTowerInside` | SimpleMapObj | ReverseGravityTowerInside |
| `DropOfWaterCore` | SimpleMapObj | DropOfWaterCore |
| `ForestAppearStepA` | SimpleMapObj | ForestAppearStepA |
| `AstroDomeDemoAstroGalaxy` | AstroDomeDemoAstroGalaxy | AstroDomeDemoAstroGalaxy |
| `ForestWoodCover` | SimpleMapObj | ForestWoodCover |
| `KoopaShipManholeCover` | ManholeCover | KoopaShipManholeCover |
| `StarDustStepA` | SimpleMapObj | StarDustStepA |
| `StarDustStepB` | SimpleMapObj | StarDustStepB |
| `TreasureSpot` | TreasureSpot | TreasureSpot |
| `CoinFlower` | TreasureSpot | CoinFlower |
| `StarDustRollingStepA` | RotateMoveObj | StarDustRollingStepA |
| `AsteroidMoveA` | RailMoveObj | AsteroidMoveA |
| `StarPieceCluster` | SimpleMapObj | StarPieceCluster |
| `SpaceSeparatorA` | SimpleMapObj | SpaceSeparatorA |
| `SpaceSeparatorB` | SimpleMapObj | SpaceSeparatorB |
| `ForestNarrowStepA` | SimpleMapObj | ForestNarrowStepA |
| `ForestHomeGate` | SimpleMapObj | ForestHomeGate |
| `RGTowerPressGroundA` | RailMoveObjPress | RGTowerPressGroundA |
| `WeatherVane` | SimpleMapObj | WeatherVane |
| `PowerStarKeeperA` | RotateMoveObj | PowerStarKeeperA |
| `PowerStarKeeperB` | RotateMoveObj | PowerStarKeeperB |
| `PowerStarKeeperC` | RotateMoveObj | PowerStarKeeperC |
| `ForestPoihanaFenceA` | SimpleMapObj | ForestPoihanaFenceA |
| `ForestPoihanaFenceB` | SimpleMapObj | ForestPoihanaFenceB |
| `TeresaMansionBridgeA` | SimpleMapObj | TeresaMansionBridgeA |
| `TeresaMansionBridgeB` | SimpleMapObj | TeresaMansionBridgeB |
| `DarknessRoomDoorA` | DarknessRoomDoor | DarknessRoomDoorA |
| `TeresaMansionDoorA` | TypicalDoor | TeresaMansionDoorA |
| `TeresaMansionDoorB` | TypicalDoor | TeresaMansionDoorB |
| `TeresaMansionEntranceDoor` | TypicalDoorOpen | TeresaMansionEntranceDoor |
| `WaterBazookaTowerMoveStepA` | RotateMoveObj | WaterBazookaTowerMoveStepA |
| `WaterBazookaTowerMoveStepB` | RailMoveObj | WaterBazookaTowerMoveStepB |
| `WaterBazookaTowerMoveStepC` | RailMoveObj | WaterBazookaTowerMoveStepC |
| `QuestionBoxMoveStepA` | QuestionBoxGalleryObj | QuestionBoxMoveStepA |
| `QuestionBoxMoveStepB` | QuestionBoxGalleryObj | QuestionBoxMoveStepB |
| `QuestionBoxMoveStepC` | QuestionBoxGalleryObj | QuestionBoxMoveStepC |
| `ForestHomeBridge` | SimpleMapObj | ForestHomeBridge |
| `ForestBarricadeRockA` | SimpleMapObj | ForestBarricadeRockA |
| `ReverseGravityRoomPressGroundA` | RailMoveObjPress | ReverseGravityRoomPressGroundA |
| `ReverseGravityRoomPressGroundB` | RailMoveObjPress | ReverseGravityRoomPressGroundB |
| `ReverseGravityRoomPressGroundC` | RailMoveObjPress | ReverseGravityRoomPressGroundC |
| `ReverseGravityRoomPressGroundD` | RailMoveObjPress | ReverseGravityRoomPressGroundD |
| `ReverseGravityRoomPressGroundE` | RailMoveObjPress | ReverseGravityRoomPressGroundE |
| `RollingOvalPlanetParts` | RotateMoveObj | RollingOvalPlanetParts |
| `TeresaMansionLightA` | SwingLight | TeresaMansionLightA |
| `TeresaMansionLightB` | SwingLight | TeresaMansionLightB |
| `TeresaMansionCandlestand` | Candlestand | TeresaMansionCandlestand |
| `TeresaMansionHoleCover` | SimpleBreakableObj | TeresaMansionHoleCover |
| `KoopaStatue` | SimpleBreakableObj | KoopaStatue |
| `TeresaPicture` | SimpleTextureSwitchChangeObj | TeresaPicture |
| `KinokoOneUpPicture` | SimpleTextureSwitchChangeObj | KinokoOneUpPicture |
| `BattleShipMovePartsA` | RotateMoveObj | BattleShipMovePartsA |
| `BattleShipMovePartsB` | RotateMoveObj | BattleShipMovePartsB |
| `BattleShipElevator` | BattleShipElevator | BattleShipElevator |
| `BattleShipElevatorCover` | SimpleMapObj | BattleShipElevatorCover |
| `TeresaRaceSpaceStickA` | SimpleMapObj | TeresaRaceSpaceStickA |
| `TeresaRaceSpaceStickB` | SimpleMapObj | TeresaRaceSpaceStickB |
| `TeresaRaceSpaceStickC` | SimpleMapObj | TeresaRaceSpaceStickC |
| `TeresaRacePartsA` | RotateMoveObj | TeresaRacePartsA |
| `TeresaRacePartsB` | AnmModelBindMove | TeresaRacePartsB |
| `NeedlePlant` | NeedlePlant | NeedlePlant |
| `NeedlePlantFly` | NeedlePlant | NeedlePlantFly |
| `BeeWallClimbPartsA` | RailMoveObj | BeeWallClimbPartsA |
| `PeachCastleTownAfterAttack` | SimpleMapObj | PeachCastleTownAfterAttack |
| `PeachCastleTownBeforeAttack` | SimpleMapObj | PeachCastleTownBeforeAttack |
| `PeachCastleTownGate` | SimpleMapObj | PeachCastleTownGate |
| `PhantomDecoratePartsA` | SimpleEnvironmentObj | PhantomDecoratePartsA |
| `PhantomDecoratePartsB` | SimpleEnvironmentObj | PhantomDecoratePartsB |
| `PhantomDecoratePartsHole` | SimpleEnvironmentObj | PhantomDecoratePartsHole |
| `CocoonStepA` | SimpleMapObj | CocoonStepA |
| `CocoonStepB` | SimpleMapObj | CocoonStepB |
| `SpaceCannonLauncher` | SimpleMapObj | SpaceCannonLauncher |
| `TrapBaseA` | SimpleMapObj | TrapBaseA |
| `SweetsDecoratePartsSpoon` | RotateMoveObj | SweetsDecoratePartsSpoon |
| `SweetsDecoratePartsFork` | RotateMoveObj | SweetsDecoratePartsFork |
| `SeesawMoveNutA` | SeesawMoveNut | SeesawMoveNutA |
| `SeesawMoveNutB` | SeesawMoveNut | SeesawMoveNutB |
| `SeesawMoveNutC` | SeesawMoveNut | SeesawMoveNutC |
| `SandStreamMoveStepsA` | RotateMoveObj | SandStreamMoveStepsA |
| `SandStreamMoveStepsB` | RotateMoveObj | SandStreamMoveStepsB |
| `BroadBeanMoveStepA` | RailMoveObj | BroadBeanMoveStepA |
| `BroadBeanMoveStepB` | RailMoveObj | BroadBeanMoveStepB |
| `ColorPencil` | SimpleMapObj | ColorPencil |
| `TeresaRacePartsBallA` | SimpleMapObj | TeresaRacePartsBallA |
| `SandCapsulePressGround` | SandCapsulePressGround | SandCapsulePressGround |
| `RayGunPlanetPartsScrew` | RailRotateMoveObj | RayGunPlanetPartsScrew |
| `RayGunPlanetPartsGear` | RotateMoveObj | RayGunPlanetPartsGear |
| `BreakDownFixStepA` | SimpleMapObj | BreakDownFixStepA |
| `SandStreamHighTowerMoveStepA` | RailMoveObj | SandStreamHighTowerMoveStepA |
| `DriftWood` | DriftWood | DriftWood |
| `ToyFactoryDecoratePartsGearA` | RotateMoveObj | ToyFactoryDecoratePartsGearA |
| `MiniMechaKoopaPartsMoveStepA` | RailMoveObj | MiniMechaKoopaPartsMoveStepA |
| `MiniMechaKoopaPartsGear` | RotateMoveObj | MiniMechaKoopaPartsGear |
| `MiniMechaKoopaPartsCage` | RotateMoveObj | MiniMechaKoopaPartsCage |
| `AsteroidBlockRotateStepA` | RotateMoveObj | AsteroidBlockRotateStepA |
| `SandUpDownTowerBreakableWallA` | SimpleBreakableObj | SandUpDownTowerBreakableWallA |
| `SandUpDownTowerBreakableWallB` | SimpleBreakableObj | SandUpDownTowerBreakableWallB |
| `HoleDeathSandMoveStepA` | RailMoveObj | HoleDeathSandMoveStepA |
| `SandUpDownTowerMoveStepA` | RailMoveObj | SandUpDownTowerMoveStepA |
| `DeathSandLandPartsA` | SimpleMapObj | DeathSandLandPartsA |
| `DeathSandLandPartsB` | SimpleMapObj | DeathSandLandPartsB |
| `DeathSandLandPlatformStepA` | SimpleMapObj | DeathSandLandPlatformStepA |
| `DeathSandEnvironmentSapotenA` | SimpleEnvironmentObj | DeathSandEnvironmentSapotenA |
| `DeathSandEnvironmentSapotenB` | SimpleEnvironmentObj | DeathSandEnvironmentSapotenB |
| `DeathSandEnvironmentRock` | SimpleEnvironmentObj | DeathSandEnvironmentRock |
| `DeathSandEnvironmentPyramid` | SimpleEnvironmentObj | DeathSandEnvironmentPyramid |
| `UFOSandObstacleA` | SimpleMapObj | UFOSandObstacleA |
| `UFOSandObstacleB` | SimpleMapObj | UFOSandObstacleB |
| `UFOSandObstacleC` | SimpleMapObj | UFOSandObstacleC |
| `SweetDecoratePartsOrange` | SimpleEnvironmentObj | SweetDecoratePartsOrange |
| `CapsuleCage` | CapsuleCage | CapsuleCage |
| `KameckShipLv1` | SimpleMapObj | KameckShipLv1 |
| `LavaBallRisingPlanetLava` | LavaBallRisingPlanetLava | LavaBallRisingPlanetLava |
| `StrongBlock` | SimpleMapObj | StrongBlock |
| `ChoConveyorChocoA` | SimpleMapObj | ChoConveyorChocoA |
| `ChoConveyorMoveChocoA` | RailMoveObj | ChoConveyorMoveChocoA |
| `HoneyQueenLeafA` | SimpleSeesawObj | HoneyQueenLeafA |
| `ForestHomePartsTree` | SimpleMapObj | ForestHomePartsTree |
| `ForestHomePartsTreeTower` | SimpleMapObj | ForestHomePartsTreeTower |
| `PoltaBattlePlanetPartsA` | SimpleMapObj | PoltaBattlePlanetPartsA |
| `HeavenlyBeachTimerStepA` | SimpleTimerObj | HeavenlyBeachTimerStepA |
| `HeavenlyBeachTimerStepB` | SimpleTimerObj | HeavenlyBeachTimerStepB |
| `ReverseKingdomTreeA` | SimpleMapObj | ReverseKingdomTreeA |
| `WindMillPropeller` | RotateMoveObj | WindMillPropeller |
| `WindMillPropellerMini` | RotateMoveObj | WindMillPropellerMini |
| `LavaRotateStepsRotatePartsA` | RotateMoveObj | LavaRotateStepsRotatePartsA |
| `LavaRotateStepsRotatePartsB` | RotateMoveObj | LavaRotateStepsRotatePartsB |
| `LavaRotateStepsRotatePartsC` | RotateMoveObj | LavaRotateStepsRotatePartsC |
| `LavaRotateStepsRotatePartsD` | RotateMoveObj | LavaRotateStepsRotatePartsD |
| `QuickSand2DMovePartsA` | RotateMoveObj | QuickSand2DMovePartsA |
| `BiriBiriBegomanSpikePistonA` | RailMoveObj | BiriBiriBegomanSpikePistonA |
| `HugeBattleShipPlanetEntrance` | SimpleMapObj | HugeBattleShipPlanetEntrance |
| `MysteryGravityRoomBridgeA` | SimpleMapObj | MysteryGravityRoomBridgeA |
| `DeadLeaves` | DeadLeaves | DeadLeaves |
| `DeathPromenadeMovePartsSpuareA` | RailMoveObj | DeathPromenadeMovePartsSpuareA |
| `DeathPromenadeRotateCircleL` | RotateMoveObj | DeathPromenadeRotateCircleL |
| `DeathPromenadeRotateCircleS` | RotateMoveObj | DeathPromenadeRotateCircleS |
| `DarkHopperRotateStepA` | SimpleClipPartsObj | DarkHopperRotateStepA |
| `DarkHopperPlanetPartsA` | SimpleMapObj | DarkHopperPlanetPartsA |
| `DarkHopperPlanetPartsC` | SimpleMapObj | DarkHopperPlanetPartsC |
| `DarkHopperPlanetPartsD` | SimpleMapObj | DarkHopperPlanetPartsD |
| `ClockworkHandle` | SimpleBreakableObj | ClockworkHandle |
| `MiniMechaKoopaPartsFan` | SimpleMapObj | MiniMechaKoopaPartsFan |
| `RockRoadCircleA` | SimpleMapObj | RockRoadCircleA |
| `HellBallGuidePartsA` | SimpleMapObj | HellBallGuidePartsA |
| `HellBallRotatePartsA` | RotateMoveObj | HellBallRotatePartsA |
| `HellBallRotatePartsB` | RotateMoveObj | HellBallRotatePartsB |
| `HellBallRotatePartsC` | RotateMoveObj | HellBallRotatePartsC |
| `HellBallRotatePartsD` | RotateMoveObj | HellBallRotatePartsD |
| `HellBallRotatePartsE` | RotateMoveObj | HellBallRotatePartsE |
| `HellBallRotatePartsF` | RotateMoveObj | HellBallRotatePartsF |
| `HellBallRotatePartsG` | RotateMoveObj | HellBallRotatePartsG |
| `IceSlipRoad` | SimpleMapObj | IceSlipRoad |
| `CandyLiftA` | RotateMoveObj | CandyLiftA |
| `CandyLiftB` | RotateMoveObj | CandyLiftB |
| `SurfingRaceTutorialParts` | SimpleMapObj | SurfingRaceTutorialParts |
| `SurfingRaceMainGate` | SimpleMapObj | SurfingRaceMainGate |
| `SurfingRaceSubGate` | SimpleMapObj | SurfingRaceSubGate |
| `SurfingRaceStep` | SimpleMapObj | SurfingRaceStep |
| `SurfingRaceSignBoard` | SimpleMapObj | SurfingRaceSignBoard |
| `SurfingRaceVictoryStand` | SimpleMapObj | SurfingRaceVictoryStand |
| `HeavensDoorFlowerA` | SimpleMapObjNoSilhouetted | HeavensDoorFlowerA |
| `HeavensDoorHouseDoor` | SimpleMapObj | HeavensDoorHouseDoor |
| `HeavensDoorAppearStepAAfter` | SimpleMapObj | HeavensDoorAppearStepAAfter |
| `HeavensDoorAppearStepA` | HeavensDoorDemoObj | HeavensDoorAppearStepA |
| `HeavensDoorMiddleRotatePartsA` | RotateMoveObj | HeavensDoorMiddleRotatePartsA |
| `HeavensDoorMiddleRotatePartsB` | RotateMoveObj | HeavensDoorMiddleRotatePartsB |
| `HeavensDoorInsideCage` | HeavensDoorDemoObj | HeavensDoorInsideCage |
| `HeavensDoorInsidePlanetPartsA` | HeavensDoorDemoObj | HeavensDoorInsidePlanetPartsA |
| `HeavensDoorInsideRotatePartsA` | RotateMoveObj | HeavensDoorInsideRotatePartsA |
| `HeavensDoorInsideRotatePartsB` | RotateMoveObj | HeavensDoorInsideRotatePartsB |
| `HeavensDoorInsideRotatePartsC` | RotateMoveObj | HeavensDoorInsideRotatePartsC |
| `MechaKoopaPartsHead` | MechaKoopaPartsHead | MechaKoopaPartsHead |
| `MechaKoopaPartsCollar` | RotateMoveObj | MechaKoopaPartsCollar |
| `MechaKoopaPartsBreast` | RailMoveObjBreakAtEnd | MechaKoopaPartsBreast |
| `MechaKoopaPartsArmRight` | MechaKoopaPartsArm | MechaKoopaPartsArmRight |
| `MechaKoopaPartsArmLeft` | MechaKoopaPartsArm | MechaKoopaPartsArmLeft |
| `MechaKoopaPartsBody` | SimpleMapObj | MechaKoopaPartsBody |
| `MechaKoopaPartsCaterpillar` | AnmModelSwitchMove | MechaKoopaPartsCaterpillar |
| `MechaKoopaPartsRollerA` | SimpleMapObj | MechaKoopaPartsRollerA |
| `MechaKoopaPartsMoveStepA` | RailMoveObjSwitchShadow | MechaKoopaPartsMoveStepA |
| `MechaKoopaPartsWreckA` | SimpleMapObj | MechaKoopaPartsWreckA |
| `CannonFortressBreakStepA` | CannonFortressBreakStep | CannonFortressBreakStepA |
| `CannonFortressRotateBreakStepA` | CannonFortressBreakStep | CannonFortressRotateBreakStepA |
| `CannonFortressRotateBreakStepB` | CannonFortressBreakStep | CannonFortressRotateBreakStepB |
| `CannonFortressRotateBreakStepC` | CannonFortressBreakStep | CannonFortressRotateBreakStepC |
| `CannonFortressMoveWall` | RailMoveObjPress | CannonFortressMoveWall |
| `SpaceShipStep` | SpaceShipStep | SpaceShipStep |
| `QuarterRollGravityRoomArrow` | QuarterRollGravityRoomArrow | QuarterRollGravityRoomArrow |
| `IceVolcanoMoveStepA` | RailMoveObj | IceVolcanoMoveStepA |
| `IceVolcanoBalanceStepA` | SimpleSeesawObj | IceVolcanoBalanceStepA |
| `IceVolcanoBalanceStepB` | SimpleSeesawObj | IceVolcanoBalanceStepB |
| `IceVolcanoAppearStepA` | AnmModelSwitchMove | IceVolcanoAppearStepA |
| `IceVolcanoAppearStepB` | AnmModelSwitchMove | IceVolcanoAppearStepB |
| `IceVolcanoClimbingWall` | AnmModelSwitchMove | IceVolcanoClimbingWall |
| `IceMountainSeesawNoSlipA` | SimpleSeesawObj | IceMountainSeesawNoSlipA |
| `IceMountainSeesawSlipA` | SimpleSeesawObj | IceMountainSeesawSlipA |
| `IceMountainSeesawSlipB` | SimpleSeesawObj | IceMountainSeesawSlipB |
| `IceMountainSeesawSlipC` | SimpleSeesawObj | IceMountainSeesawSlipC |
| `IceMountainSeesawSlipD` | SimpleSeesawObj | IceMountainSeesawSlipD |
| `IceRingBumpyPartsA` | SimpleMapObj | IceRingBumpyPartsA |
| `IceWaterUpDownParts` | IceVolcanoUpDownPlane | IceWaterUpDownParts |
| `LavaUpDownParts` | IceVolcanoUpDownPlane | LavaUpDownParts |
| `IceUpDownShutterDoorA` | AnmModelSwitchMove | IceUpDownShutterDoorA |
| `IceLavaIslandSnowStepA` | SimpleMapObj | IceLavaIslandSnowStepA |
| `IceLavaIslandIceMovableStepA` | RailMoveObj | IceLavaIslandIceMovableStepA |
| `IceLavaIslandLavaMovableStepA` | RailMoveObj | IceLavaIslandLavaMovableStepA |
| `HoleBeltConveyerPartsA` | RailMoveObj | HoleBeltConveyerPartsA |
| `HoleBeltConveyerPartsB` | RailMoveObj | HoleBeltConveyerPartsB |
| `HoleBeltConveyerPartsC` | RailMoveObj | HoleBeltConveyerPartsC |
| `HoleBeltConveyerPartsD` | RailMoveObj | HoleBeltConveyerPartsD |
| `HoleBeltConveyerPartsE` | RailMoveObj | HoleBeltConveyerPartsE |
| `HoleBeltConveyerPartsF` | RailMoveObj | HoleBeltConveyerPartsF |
| `HoleBeltConveyerPartsG` | RotateMoveObj | HoleBeltConveyerPartsG |
| `HoleBeltConveyerPartsH` | RailMoveObj | HoleBeltConveyerPartsH |
| `ChallengeBallAccelCylinderA` | RotateMoveObj | ChallengeBallAccelCylinderA |
| `ChallengeBallGearA` | RotateMoveObj | ChallengeBallGearA |
| `ChallengeBallMoveGroundA` | RailMoveObj | ChallengeBallMoveGroundA |
| `ChallengeBallMoveGroundB` | RailMoveObj | ChallengeBallMoveGroundB |
| `ChallengeBallRotateBridgeA` | RotateMoveObj | ChallengeBallRotateBridgeA |
| `ChallengeBallVanishingRoadA` | SimpleMapObj | ChallengeBallVanishingRoadA |
| `TrialBubbleTimerMoveWallA` | TimerMoveWall | TrialBubbleTimerMoveWallA |
| `TrialBubbleRotateWallA` | RotateMoveObj | TrialBubbleRotateWallA |
| `TrialBubbleRevolvingPartsA` | RotateMoveObj | TrialBubbleRevolvingPartsA |
| `TrialBubbleMoveWallA` | RailMoveObj | TrialBubbleMoveWallA |
| `CubeBubbleExHomeStep` | SimpleMapObj | CubeBubbleExHomeStep |
| `CubeBubbleExStartStep` | SimpleMapObj | CubeBubbleExStartStep |
| `CubeBubbleExRotateWallS` | RotateMoveObj | CubeBubbleExRotateWallS |
| `CubeBubbleExRotateWallL` | RotateMoveObj | CubeBubbleExRotateWallL |
| `CubeBubbleExPartsA` | SimpleMapObj | CubeBubbleExPartsA |
| `UFOKinoko` | UFOKinoko | UFOKinoko |
| `UFOKinokoLanding` | SimpleMapObj | UFOKinokoLanding |
| `KoopaShipA` | SimpleMapObj | KoopaShipA |
| `KoopaShipB` | SimpleMapObj | KoopaShipB |
| `KoopaShipC` | SimpleMapObj | KoopaShipC |
| `KoopaShipD` | SimpleMapObj | KoopaShipD |
| `KoopaShipE` | SimpleMapObj | KoopaShipE |
| `KoopaShipSeesawStep` | SimpleSeesawObj | KoopaShipSeesawStep |
| `KoopaJrSmallShipA` | RailMoveObj | KoopaJrSmallShipA |
| `KoopaJrSmallShipAGuidePoint` | SimpleMapObj | KoopaJrSmallShipAGuidePoint |
| `KoopaJrKillerShipA` | SimpleMapObj | KoopaJrKillerShipA |
| `KoopaJrNormalShipA` | SimpleMapObj | KoopaJrNormalShipA |
| `WaterRoadCaveStepA` | RailMoveObj | WaterRoadCaveStepA |
| `WaterRoadCaveStepB` | SimpleMapObj | WaterRoadCaveStepB |
| `WaterRoadCaveRotateGround` | RotateMoveObj | WaterRoadCaveRotateGround |
| `SubmarineVolcanoInside` | SimpleMapObj | SubmarineVolcanoInside |
| `OnimasuPlanetPartsGoal` | SimpleMapObj | OnimasuPlanetPartsGoal |
| `OnimasuPlanetObstaclePartsA` | SimpleMapObj | OnimasuPlanetObstaclePartsA |
| `OnimasuPlanetRailMovePartsA` | RailMoveObj | OnimasuPlanetRailMovePartsA |
| `OnimasuPlanetRotatePartsA` | RotateMoveObj | OnimasuPlanetRotatePartsA |
| `OnimasuPlanetRotatePartsB` | RotateMoveObj | OnimasuPlanetRotatePartsB |
| `PhantomShipPropellerSmall` | AnmModelSwitchMove | PhantomShipPropellerSmall |
| `PhantomShipPropellerBig` | AnmModelSwitchMove | PhantomShipPropellerBig |
| `TakoBarrelA` | SimpleBreakableObj | TakoBarrelA |
| `TakoBarrelB` | SimpleMapObj | TakoBarrelB |
| `KoopaBattleMapStairturnAppear` | TimeAppearObj | KoopaBattleMapStairTurn |
| `KoopaVS1PartsSpiralRoad` | SimpleMapObj | KoopaVS1PartsSpiralRoad |
| `KoopaVS1PartsMoveStepA` | RailMoveObj | KoopaVS1PartsMoveStepA |
| `KoopaVS1PartsMoveStepB` | RailMoveObj | KoopaVS1PartsMoveStepB |
| `KoopaVS1PartsRotateMoveStepA` | RailRotateMoveObj | KoopaVS1PartsRotateMoveStepA |
| `KoopaVS1PartsRotateMoveStepB` | RailRotateMoveObj | KoopaVS1PartsRotateMoveStepB |
| `KoopaVS1PartsReverseGRoad` | SimpleMapObj | KoopaVS1PartsReverseGRoad |
| `KoopaVS1PartsStairRoad` | SimpleMapObj | KoopaVS1PartsStairRoad |
| `KoopaVS1PartsBattleStage` | SimpleMapObj | KoopaVS1PartsBattleStage |
| `KoopaVS2PartsReverseGRoadA` | SimpleMapObj | KoopaVS2PartsReverseGRoadA |
| `KoopaVS2PartsReverseGRoadB` | SimpleMapObj | KoopaVS2PartsReverseGRoadB |
| `KoopaVS2PartsStartRestStep` | SimpleMapObj | KoopaVS2PartsStartRestStep |
| `KoopaVS2PartsRestStepA` | SimpleMapObj | KoopaVS2PartsRestStepA |
| `KoopaVS2PartsRestStepB` | SimpleMapObj | KoopaVS2PartsRestStepB |
| `KoopaVS2PartsRestStepC` | SimpleMapObj | KoopaVS2PartsRestStepC |
| `KoopaVS2PartsRestClipC` | SimpleClipPartsObj | KoopaVS2PartsRestClipC |
| `KoopaVS2PartsRestStepD` | SimpleMapObj | KoopaVS2PartsRestStepD |
| `KoopaVS2PartsRestStepE` | SimpleMapObj | KoopaVS2PartsRestStepE |
| `KoopaVS2PartsRestStepF` | SimpleMapObj | KoopaVS2PartsRestStepF |
| `KoopaVS2PartsRestStepG` | SimpleMapObj | KoopaVS2PartsRestStepG |
| `KoopaVS2PartsDarkMatterA` | SimpleMapObj | KoopaVS2PartsDarkMatterA |
| `KoopaVS2PartsDarkMatterB` | SimpleMapObj | KoopaVS2PartsDarkMatterB |
| `KoopaVS2PartsDarkMatterC` | SimpleMapObj | KoopaVS2PartsDarkMatterC |
| `KoopaVS2PartsDarkMatterD` | SimpleMapObj | KoopaVS2PartsDarkMatterD |
| `KoopaVS2PartsDarkMatterE` | SimpleMapObj | KoopaVS2PartsDarkMatterE |
| `KoopaVS2PartsStartMoveStepA` | RotateMoveObj | KoopaVS2PartsStartMoveStepA |
| `KoopaVS2PartsStartMoveStepB` | RotateMoveObj | KoopaVS2PartsStartMoveStepB |
| `KoopaVS2PartsStartMoveStepC` | RailMoveObj | KoopaVS2PartsStartMoveStepC |
| `KoopaVS2PartsRollingStep` | RotateMoveObj | KoopaVS2PartsRollingStep |
| `KoopaVS2Parts2DMoveStepBarA` | RailMoveObj | KoopaVS2Parts2DMoveStepBarA |
| `KoopaVS2Parts2DMoveStepBarB` | RailMoveObj | KoopaVS2Parts2DMoveStepBarB |
| `KoopaVS2Parts2DMoveStepSBarB` | RailMoveObj | KoopaVS2Parts2DMoveStepSBarB |
| `KoopaVS2Parts2DMoveStepConvex` | RailMoveObjPress | KoopaVS2Parts2DMoveStepConvex |
| `KoopaVS2Parts2DMoveStepLShape` | RailMoveObj | KoopaVS2Parts2DMoveStepLShape |
| `KoopaVS2Parts2DMoveStepCross` | RailMoveObj | KoopaVS2Parts2DMoveStepCross |
| `KoopaVS2PartsJoinedMoveStep` | RailMoveObj | KoopaVS2PartsJoinedMoveStep |
| `KoopaVS2PartsJoinedMoveClip` | SimpleClipPartsObj | KoopaVS2PartsJoinedMoveClip |
| `KoopaVS2PartsSquareMoveStepA` | RailMoveObj | KoopaVS2PartsSquareMoveStepA |
| `KoopaVS2PartsSquareMoveClipA` | SimpleClipPartsObj | KoopaVS2PartsSquareMoveClipA |
| `KoopaVS2PartsSquareMoveStepB` | RailMoveObj | KoopaVS2PartsSquareMoveStepB |
| `KoopaVS2PartsSquareMoveClipB` | SimpleClipPartsObj | KoopaVS2PartsSquareMoveClipB |
| `KoopaVS2PartsClipAreaDisplayA` | RailMoveIndirectObj | KoopaVS2PartsClipAreaDisplayA |
| `KoopaVS2PartsStairBig` | SimpleMapObj | KoopaVS2PartsStairBig |
| `KoopaVS2Parts2DRailGuideA` | SimpleMapObj | KoopaVS2Parts2DRailGuideA |
| `KoopaVS3RotateStepA` | RotateMoveObj | KoopaVS3RotateStepA |
| `KoopaVS3RotateStepB` | RotateMoveObj | KoopaVS3RotateStepB |
| `KoopaVS3RotateStepD` | RotateMoveObj | KoopaVS3RotateStepD |
| `KoopaVS3RotateStepsA` | RotateMoveObj | KoopaVS3RotateStepsA |
| `KoopaVS3Parts2DWallA` | SimpleMapObj | KoopaVS3Parts2DWallA |
| `OceanRingRuinsMove` | RailMoveObj | OceanRingRuinsMove |
| `OceanRingRuinsGearSmall` | RotateMoveObj | OceanRingRuinsGearSmall |
| `OceanRingRuinsGearBig` | RotateMoveObj | OceanRingRuinsGearBig |
| `OceanRingRuinsColumn` | SimpleMapObj | OceanRingRuinsColumn |
| `OceanRingRuinsBase` | SimpleMapObj | OceanRingRuinsBase |
| `KameckShip` | SimpleMapObj | KameckShip |
| `BeachParasol` | SimpleMapObj | BeachParasol |
| `BeachChair` | SimpleMapObj | BeachChair |
| `GhostShipCaveMoveGroundA` | RailMoveObj | GhostShipCaveMoveGroundA |
| `GhostShipCaveMoveGroundB` | RailMoveObj | GhostShipCaveMoveGroundB |
| `PhantomCaveStepA` | SimpleMapObj | PhantomCaveStepA |
| `GhostShipCaveClosedRockA` | SimpleMapObj | GhostShipCaveClosedRockA |
| `GhostShipBrokenHead` | SimpleMapObj | GhostShipBrokenHead |
| `CannonUnderConstructionA` | SimpleMapObj | CannonUnderConstructionA |
| `CannonUnderConstructionB` | SimpleMapObj | CannonUnderConstructionB |
| `AstroMapBoard` | AstroMapBoard | AstroMapBoard |
| `AstroCore` | AstroCore | AstroCore |
| `AstroDome` | AstroDome |  |
| `AstroDomeEntrance` | AstroMapObj |  |
| `AstroStarPlate` | AstroMapObj |  |
| `AstroBaseA` | AstroMapObj | AstroBaseA |
| `AstroBaseB` | AstroMapObj | AstroBaseB |
| `AstroBaseC` | AstroMapObj | AstroBaseC |
| `AstroBaseKitchen` | AstroMapObj | AstroBaseKitchen |
| `AstroBaseCenterA` | AstroMapObj | AstroBaseCenterA |
| `AstroBaseCenterB` | AstroMapObj | AstroBaseCenterB |
| `AstroBaseCenterTop` | AstroMapObj | AstroBaseCenterTop |
| `AstroRotateStepA` | AstroMapObj | AstroRotateStepA |
| `AstroRotateStepB` | AstroMapObj | AstroRotateStepB |
| `AstroDecoratePartsA` | AstroMapObj | AstroDecoratePartsA |
| `AstroDecoratePartsGearA` | AstroMapObj | AstroDecoratePartsGearA |
| `AstroChildRoom` | AstroMapObj | AstroChildRoom |
| `AstroParking` | AstroMapObj | AstroParking |
| `AstroLibrary` | AstroMapObj | AstroLibrary |
| `AstroRoomLibrary` | SimpleMapObj | AstroRoomLibrary |
| `UFOKinokoUnderConstruction` | UFOKinokoUnderConstruction |  |
| `UFOKinokoLandingAstro` | SimpleMapObj | UFOKinokoLandingAstro |
| `WhiteRoom` | SimpleMapObj | WhiteRoom |
| `ZoneHalo` | Halo | ZoneHalo |
| `PowerStarHalo` | PowerStarHalo | PowerStarHalo |

## Not actors: can be placed, not spawned

| Name | Class | Archive |
|---|---|---|
| `TripodBossCoin` | TripodBossCoin |  |
| `TripodBossUpperKillerCannon` | TripodBossKillerGeneraterCircle | TripodBossKillerCannon |
| `TripodBossUnderKillerCannon` | TripodBossKillerGeneraterCircle | TripodBossKillerCannon |
| `TripodBossBottomKillerCannon` | TripodBossKillerGeneraterCircle | TripodBossKillerCannon |
| `ArrowSwitchTarget` | ArrowSwitchTarget |  |
| `PurpleCoinStarter` | PurpleCoinStarter |  |
| `CoinReplica` | CoinReplica |  |
| `BlueChipGroup` | BlueChipGroup |  |
| `YellowChipGroup` | YellowChipGroup |  |
| `RaceRail` | RaceRail |  |
| `SwitchSynchronizerReverse` | SwitchSynchronizer |  |
| `DemoGroup` | DemoExecutor |  |
| `DemoSubGroup` | DemoCastSubGroup |  |

## Other actors

| Name | Class | Archive |
|---|---|---|
| `Mario` | MarioActor |  |
| `MarioActor` | MarioActor |  |
| `TalkSyati` | Syati | Syati |
| `CollectTico` | CollectTico | StrayTico |
| `TrickRabbit` | TrickRabbit | TrickRabbit |
| `TrickRabbitFreeRun` | TrickRabbitFreeRun | TrickRabbit |
| `TrickRabbitFreeRunCollect` | TrickRabbitSnowCollect | TrickRabbit |
| `RunawayRabbitCollect` | RunawayRabbitCollect | TrickRabbit |
| `TrickRabbitGhost` | TrickRabbit | TrickRabbit |
| `Syati` | Syati | Syati |
| `TicoRail` | TicoRail | Tico |
| `TicoReading` | TicoReading | Tico |
| `TicoDomeLecture` | TicoDomeLecture | Tico |
| `RosettaReading` | RosettaReading | RosettaReading |
| `RosettaPictureBook` | RosettaPictureBook | RosettaPictureBook |
| `NPCSupportRail` | NPCSupportRail |  |
| `GhostPlayer` | GhostPlayer |  |
| `DemoKoopaJrShip` | DemoKoopaJrShip | KoopaJrShip |
| `Creeper` | Creeper | Creeper |
| `JetTurtle` | JetTurtle | Koura |
| `Fluff` | Fluff | Fluff |
| `FluffWind` | FluffWind |  |
| `Tamakoro` | Tamakoro | Tamakoro |
| `GoldenTurtle` | GoldenTurtle | KouraShine |
| `JumpBranch` | JumpBranch | JumpBranch |
| `Plant` | Plant | Plant |
| `Pole` | Pole | Pole |
| `PoleNoModel` | Pole | Pole |
| `PoleSquare` | Pole | PoleSquare |
| `PoleSquareNoModel` | Pole | PoleSquare |
| `SurfRay` | SurfRay | SurfRay |
| `SwingRope` | SwingRope | SwingRope |
| `SpaceCocoon` | SpaceCocoon | SpaceCocoon |
| `Trapeze` | Trapeze | Trapeze |
| `TreeCube` | Pole | TreeCube |
| `WaterRoad` | WaterRoad | WaterWave |
| `WarpPod` | WarpPod | WarpPod |
| `CoinBox` | CoinBox |  |
| `PunchBox` | PunchBox | PunchBox |
| `SwitchBox` | SwitchBox | SwitchBox |
| `NormalMapTestObj` | NormalMapTestObj | NormalMapTestObj |
| `Takobo` | Takobo | Takobo |
| `Karikari` | Karikari | Karipon |
| `KoteBug` | KoteBug | KoteBug |
| `Balloonfish` | Balloonfish | Balloonfish |
| `BombHei` | BombHei | BombHei |
| `Bomb` | BombHei | BombHei |
| `Jiraira` | Jiraira | Jiraira |
| `BombHeiLauncher` | BombHeiLauncher | BombHeiLauncher |
| `BombLauncher` | BombHeiLauncher | BombHeiLauncher |
| `SpinDriver` | SpinDriver | SpinDriver |
| `Kuribo` | Kuribo | Kuribo |
| `KuriboChief` | KuriboChief | KuriboChief |
| `KuriboMini` | KuriboMini | KuriboMini |
| `Kabokuri` | Kabokuri | Kabokuri |
| `TakoHei` | TakoHei | TakoHei |
| `OtaRock` | OtaRock | OtaRock |
| `Meramera` | Meramera | Meramera |
| `IceMeramera` | Meramera | IceMeramera |
| `EyeBeamer` | EyeBeamer | EyeBeamer |
| `BigBirikyu` | Birikyu | BigBirikyu |
| `Birikyu` | BirikyuWithFace | Birikyu |
| `HomingKiller` | HomingKillerLauncher | HomingKiller |
| `CocoSambo` | CocoSambo | CocoSamboBody |
| `SamboHead` | SamboHead | SamboHead |
| `BegomanBaby` | BegomanBaby | BegomanBaby |
| `BegomanSpring` | BegomanSpring | BegomanSpring |
| `BegomanSpringHide` | BegomanSpring | BegomanSpring |
| `BegomanSpike` | BegomanSpike | BegomanSpike |
| `BegomanLauncher` | BegomanLauncher | BegomanLauncher |
| `TogeBegomanLauncher` | BegomanLauncher | BegomanLauncher |
| `BegomanBabyLauncher` | BegomanLauncher | BegomanLauncher |
| `FireBubble` | FireBubble | FireBubble |
| `BallBeamer` | BallBeamer | BallBeamer |
| `JumpBeamer` | JumpBeamer | JumpBeamerBody |
| `JumpGuarder` | JumpGuarder | JumpGuarder |
| `RingBeamer` | RingBeamer | RingBeamer |
| `Pukupuku` | Pukupuku | Pukupuku |
| `BombTeresa` | BombTeresa | BombTeresa |
| `Teresa` | Teresa | Teresa |
| `TeresaChief` | Teresa | Teresa |
| `TeresaWater` | TeresaWater | TeresaWater |
| `Torpedo` | HomingKillerLauncher | Torpedo |
| `Dossun` | Dossun | Dossun |
| `Onimasu` | OnimasuJump | Onimasu |
| `OnimasuPivot` | OnimasuPivot | Onimasu |
| `Unizo` | Unizo | Unizo |
| `UnizoLand` | Unizo | UnizoLand |
| `UnizoShoal` | Unizo | UnizoShoal |
| `UnizoLauncher` | UnizoLauncher | UnizoLand |
| `Kiraira` | Kiraira | Kiraira |
| `KirairaRail` | Kiraira | Kiraira |
| `NokonokoLand` | NokonokoLand | NokonokoLand |
| `Gesso` | Gesso | Gesso |
| `Poihana` | Poihana | Poihana |
| `Snakehead` | Snakehead | Snakehead |
| `SnakeheadSmall` | Snakehead | SnakeheadSmall |
| `Jellyfish` | Jellyfish | Jellyfish |
| `JellyfishElectric` | JellyfishElectric | JellyfishElectric |
| `Kanina` | Kanina | Kanina |
| `KaninaRed` | Kanina | KaninaRed |
| `Mogu` | Mogu | Mogu |
| `Mogucchi` | Mogucchi | Mogucchi |
| `MogucchiRefuseTerritory` | MogucchiRefuseTerritory |  |
| `KoopaJrShip` | KoopaJrShip | KoopaJrShip |
| `Hanachan` | Hanachan | HanachanHead |
| `JumpSpider` | JumpSpider | JumpSpider |
| `StringSpider` | StringSpider | StringSpider |
| `PackunPetit` | PackunPetit | PackunPetit |
| `StinkBugSmall` | StinkBugSmall | StinkBugSmall |
| `StinkBugParent` | StinkBugParent | StinkBugParent |
| `BasaBasa` | BasaBasa | BasaBasa |
| `BasaBasaIce` | BasaBasa | BasaBasaIce |
| `Metbo` | Metbo | Metbo |
| `BombBird` | BombBird | BombBird |
| `MechanicKoopaMini` | MechanicKoopaMini | MechanicKoopaMini |
| `Karon` | Karon | Karon |
| `HammerHeadPackun` | HammerHeadPackun | PackunFlower |
| `DharmaSambo` | DharmaSambo | DharmaSambo |
| `SearchBeamer` | SearchBeamer | SearchBeamer |
| `Petari` | Petari | Petari |
| `MagnumKiller` | HomingKillerLauncher | MagnumKiller |
| `BossBegoman` | BossBegoman | BossBegoman |
| `BossStinkBug` | BossStinkBug | BossStinkBug |
| `Dodoryu` | Dodoryu | Dodoryu |
| `OtaKing` | OtaKing |  |
| `Polta` | Polta | Polta |
| `SkeletalFishBoss` | SkeletalFishBoss | SkeletalFishBoss |
| `SkeletalFishBaby` | SkeletalFishBaby | SnakeFish |
| `SkeletalFishBabyRail` | SkeletalFishBabyRail |  |
| `SkeletalFishBossRail` | SkeletalFishBossRail |  |
| `TombSpider` | TombSpider | TombSpider |
| `TripodBossBaseJointPosition` | TripodBossBaseJointPosition |  |
| `TripodBossKillerGenerater` | TripodBossKillerGenerater | TripodBossKillerCannon |
| `TripodBossStepPoint` | TripodBossStepPoint |  |
| `TripodBossCore` | TripodBossCore | TripodBossCore |
| `TripodBossKinokoOneUp` | TripodBossKinokoOneUp | KinokoOneUp |
| `TripodBossShell` | TripodBossShell | TripodBossShell |
| `TripodBossGuardWall` | TripodBossGuardWall | TripodBossGuardWall |
| `TriPodBossBodyA` | TripodBossFixParts | TriPodBossBodyA |
| `TripodBossBodyABloom` | TripodBossFixParts | TripodBossBodyABloom |
| `TriPodBossLeg1A` | TripodBossFixParts | TriPodBossLeg1A |
| `TriPodBossLeg1B` | TripodBossFixParts | TriPodBossLeg1B |
| `TriPodBossLeg1C` | TripodBossFixParts | TriPodBossLeg1C |
| `TriPodBossLeg2A` | TripodBossFixParts | TriPodBossLeg2A |
| `TriPodBossLeg2ABloom` | TripodBossFixParts | TriPodBossLeg2ABloom |
| `TriPodBossLeg3A` | TripodBossFixParts | TriPodBossLeg3A |
| `TriPodBossLeg3B` | TripodBossFixParts | TriPodBossLeg3B |
| `TriPodBossLeg3C` | TripodBossFixParts | TriPodBossLeg3C |
| `TripodBossFoot` | TripodBossFixParts | TripodBossFoot |
| `TripodBossFootBloom` | TripodBossFixParts | TripodBossFootBloom |
| `TriPodBossLegBottom` | TripodBossFixParts | TriPodBossLegBottom |
| `TripodBossEyeLight` | TripodBossFixParts | TripodBossEyeLight |
| `TripodBossEyeLightBloom` | TripodBossFixParts | TripodBossEyeLightBloom |
| `TriPodBossGearA` | TripodBossRotateParts | TriPodBossGearA |
| `TriPodBossGearB` | TripodBossRotateParts | TriPodBossGearB |
| `TriPodBossGearC` | TripodBossRotateParts | TriPodBossGearC |
| `TriPodBossGearSmallA` | TripodBossRotateParts | TriPodBossGearSmallA |
| `TriPodBossGearSmallB` | TripodBossRotateParts | TriPodBossGearSmallB |
| `TriPodBossGearSmallC` | TripodBossRotateParts | TriPodBossGearSmallC |
| `TriPodBossGearBody` | TripodBossRotateParts | TriPodBossGearBody |
| `SpiderAttachPoint` | SlingShooter | SpiderThreadAttachPoint |
| `SpiderCoin` | SpiderCoin | Coin |
| `SpiderMapBlock` | SpiderMapBlock | SpiderMapBlock |
| `ArrowSwitch` | ArrowSwitch | ArrowSwitch |
| `ArrowSwitchMulti` | ArrowSwitchMulti | ArrowSwitch |
| `AstroOverlookObj` | AstroOverlookObj |  |
| `BlueStarCupsulePlanet` | BlueStarCupsulePlanet | BlueStarCupsulePlanet |
| `BigFan` | BigFan | BigFan |
| `BigFanNoLeg` | BigFan | BigFanNoLeg |
| `ClipAreaDropLaser` | ClipAreaDropLaser | ClipVolumeSphere |
| `RevolvingWay` | RevolvingWay | RevolvingWay |
| `SoundEmitter` | SoundEmitter |  |
| `JumpHole` | JumpHole | JumpHole |
| `BallOpener` | BallOpener | BallOpener |
| `DragonHeadFlower` | DragonHeadFlower | DragonHeadFlower |
| `SphereRailDash` | SphereRailDash |  |
| `SnowplowSwitch` | SnowplowSwitch |  |
| `BallRail` | BallRail |  |
| `FallDownBridge` | FallDownBridge | FallDownBridge |
| `RockCreator` | RockCreator | Rock |
| `MeteoContainer` | MeteoContainer | MeteoContainer |
| `MagicBell` | MagicBell | Bell |
| `FlowerGroup` | PlantGroup | Flower |
| `FlowerBlueGroup` | PlantGroup | FlowerBlue |
| `CutBushGroup` | PlantGroup | CutBush |
| `SmallStone` | SmallStone | SmallStone |
| `CircleShell` | SmallStone | CircleShell |
| `CircleStrawberry` | SmallStone | CircleStrawberry |
| `TsukidashikunBig` | RailBlock | TsukidashikunBig |
| `TimerCoinBlock` | ItemBlock | CoinBlock |
| `SplashCoinBlock` | ItemBlock | CoinBlock |
| `TimerPieceBlock` | ItemBlock | CoinBlock |
| `SplashPieceBlock` | ItemBlock | CoinBlock |
| `ItemBlockSwitch` | ItemBlock | CoinBlock |
| `ShootingStar` | ShootingStar | ShootingStar |
| `FlameGun` | FlameGun | FlameGun |
| `BenefitItemOneUp` | BenefitItemOneUp | KinokoOneUp |
| `BenefitItemLifeUp` | BenefitItemLifeUp | KinokoLifeUp |
| `BenefitItemInvincible` | BenefitItemInvincible | PowerupInvincible |
| `CoinSpot` | CoinSpot |  |
| `CollisionBlocker` | CollisionBlocker |  |
| `LotusLeaf` | LotusLeaf | LotusLeaf |
| `PunchingKinoko` | PunchingKinoko | PunchingKinoko |
| `NoteFairy` | NoteFairy |  |
| `EarthenPipe` | EarthenPipe | EarthenPipe |
| `EarthenPipeInWater` | EarthenPipe | EarthenPipe |
| `ElectricRail` | ElectricRail | ElectricRail |
| `ElectricRailMoving` | ElectricRailMoving | ElectricRailMoving |
| `WanwanRolling` | RockCreator | WanwanRolling |
| `WanwanRollingMini` | RockCreator | WanwanRollingMini |
| `WanwanRollingGold` | RockCreator | WanwanRollingGold |
| `BreakableCage` | BreakableCage | BreakableCage |
| `BreakableCageRotate` | BreakableCage | BreakableCage |
| `BreakableCageL` | BreakableCage | BreakableCageL |
| `BreakableFixation` | BreakableCage | BreakableFixation |
| `BreakableTrash` | BreakableCage | BreakableTrash |
| `BigBubble` | BigBubble | BigBubble |
| `GCaptureTarget` | GCaptureTarget | GCaptureTarget |
| `BlackHole` | BlackHole | BlackHole |
| `BlackHoleCube` | BlackHole | BlackHole |
| `KillerGunnerSingle` | KillerGunnerSingle | KillerGunnerSingle |
| `PowerStar` | PowerStar | PowerStar |
| `GrandStar` | PowerStar | GrandStar |
| `PowerStarAppearPoint` | PowerStarAppearPoint |  |
| `CocoNut` | CocoNut |  |
| `CoconutTree` | CoconutTree | CoconutTree |
| `CoconutTreeLeaf` | CoconutTreeLeafGroup | CoconutTreeLeaf |
| `WoodBox` | WoodBox | WoodBox |
| `ItemBubble` | ItemBubble | ItemBubble |
| `SpinningBox` | SpinningBox | SpinningBox |
| `ElectricBall` | ElectricBall | ElectricBall |
| `MorphItemNeoHopper` | MorphItemNeoHopper | PowerupHopper |
| `MorphItemNeoBee` | MorphItemNeoBee | PowerUpBee |
| `MorphItemNeoTeresa` | MorphItemNeoTeresa | PowerupTeresa |
| `MorphItemNeoIce` | MorphItemNeoIce | PowerupIce |
| `MorphItemNeoFire` | MorphItemNeoFire | PowerupFire |
| `MorphItemNeoFoo` | MorphItemNeoFoo | PowerupFoo |
| `AirBubble` | AirBubble | AirBubble |
| `AirBubbleGenerator` | AirBubbleGenerator | AirBubbleGenerator |
| `Butterfly` | Butterfly | Butterfly |
| `Pompon2Plant` | PomponPlant | Pompon2Plant |
| `Pompon4Plant` | PomponPlant | Pompon4Plant |
| `LavaGeyser` | LavaGeyser | LavaGeyser |
| `LavaProminence` | LavaProminence | LavaProminence |
| `LavaProminenceTriple` | LavaProminenceTriple | LavaProminenceTriple |
| `LavaProminenceEnvironment` | LavaProminenceTriple | LavaProminenceEnvironment |
| `LavaProminenceWithoutShadow` | LavaProminence | LavaProminence |
| `LavaObstacleRockShell` | HipDropMoveObj | LavaObstacleRockShell |
| `LavaObstacleRockHomeA` | HipDropDemoMoveObj | LavaObstacleRockHomeA |
| `LavaHomeFloaterB` | LavaFloater | LavaHomeFloaterB |
| `LavaFloatingStepA` | LavaFloater | LavaFloatingStepA |
| `LavaFloatingStepB` | LavaFloater | LavaFloatingStepB |
| `LavaFloatingStepC` | LavaFloater | LavaFloatingStepC |
| `LavaFloatingStepD` | LavaFloater | LavaFloatingStepD |
| `LavaHomeSeesaw` | LavaHomeSeesaw | LavaHomeSeesaw |
| `LavaHomeVolcanoFlow` | LavaHomeVolcanoFlow | LavaHomeVolcanoFlow |
| `LavaShellTowerStep` | LavaGalaxyParts | LavaShellTowerStep |
| `LavaStepA` | LavaGalaxyParts | LavaStepA |
| `LavaStepB` | LavaGalaxyParts | LavaStepB |
| `LavaStepC` | LavaGalaxyParts | LavaStepC |
| `LavaSteam` | LavaSteam | LavaSteam |
| `LavaStrangeRock` | LavaStrangeRock | LavaStrangeRock |
| `LavaStrangeRockL` | LavaStrangeRock | LavaStrangeRockL |
| `LavaStrangeRockTable` | LavaStrangeRock | LavaStrangeRockTable |
| `CrystalCageS` | CrystalCage | CrystalCageS |
| `CrystalCageM` | CrystalCage | CrystalCageM |
| `CrystalCageL` | CrystalCage | CrystalCageL |
| `ShellfishCoin` | Shellfish | Shellfish |
| `ShellfishYellowChip` | Shellfish | Shellfish |
| `OceanFloaterTypeU` | OceanFloaterLandParts | OceanFloaterTypeU |
| `DashRing` | DashRing | DashRing |
| `MeteorStrike` | MeteorStrikeLauncher | MeteorStrike |
| `MeteorStrikeEnvironment` | MeteorStrikeLauncher | MeteorStrike |
| `MeteorCannon` | MeteorStrikeLauncher | MeteorStrike |
| `BrightObj` | BrightObj | LensFlare |
| `BrightSun` | BrightSun | LensFlare |
| `PhantomTorch` | PhantomTorch |  |
| `PhantomBonfire` | PhantomTorch |  |
| `MarioLauncher` | MarioLauncher | MarioLauncher |
| `MarioLauncherAttractor` | MarioLauncherAttractor | MarioLauncherAttractor |
| `WaterBazooka` | WaterBazooka | WaterBazooka |
| `ElectricBazooka` | WaterBazooka | ElectricBazooka |
| `WaterPressure` | WaterPressure | WaterPressure |
| `FirePressure` | FirePressure | FirePressure |
| `IronCannonLauncher` | IronCannonLauncher | IronCannonLauncher |
| `IronCannonLauncherPoint` | IronCannonLauncherPoint |  |
| `FirePressureRadiate` | FirePressureRadiate | FirePressure |
| `BeeFlowerHover` | BeeFlowerHover | BeeFlowerHover |
| `PalmIsland` | PalmIsland | PalmIsland |
| `IceMerameraKing` | IceMerameraKing | IceMerameraKing |
| `OceanSmallTurtle` | OceanSmallTurtle | OceanSmallTurtle |
| `SpringWaterFloaterA` | SpringWaterFloaterSpot | SpringWaterFloaterA |
| `SpringWaterFloaterB` | SpringWaterFloaterSpot | SpringWaterFloaterB |
| `SeaBottomTriplePropeller` | SeaBottomTriplePropeller | SeaBottomTriplePropeller |
| `LargeChain` | LargeChain | LargeChain |
| `ItemAppearStoneSpinDriver` | ItemAppearStone | ItemAppearStoneSpinDriver |
| `SnowFloor` | SnowFloor |  |
| `SnowFloorTile` | SnowFloorTile |  |
| `FlipPanelObserver` | FlipPanelObserver |  |
| `JumpStand` | JumpStand | JumpStand |
| `ChooChooTrain` | ChooChooTrain | ChooChooTrain |
| `SnowMan` | SnowMan | SnowMan |
| `ShockWaveGenerator` | ShockWaveGenerator | ShockWaveGenerator |
| `Banekiti` | Banekiti | Banekiti |
| `TrampleStar` | TrampleStar | TrampleStar |
| `HipDropRock` | HipDropRock | HipDropRock |
| `WatchTowerRotateStep` | WatchTowerRotateStep | WatchTowerRotateStep |
| `ForestObstacleWoodA` | HipDropDemoMoveObj | ForestObstacleWoodA |
| `AstroDomeAsteroid` | AstroDomeAsteroid | AstroDomeAsteroid |
| `AstroDomeBlueStar` | AstroDomeBlueStar | GCaptureTarget |
| `AstroDomeComet` | AstroDomeComet | AstroDomeComet |
| `SphereSelectorHandle` | SphereSelectorHandle |  |
| `RainCloud` | RainCloud | RainCloud |
| `FireBar` | FireBar | FireBarCore |
| `TwisterSan` | Sandstorm | TwisterSan |
| `TwisterSanSlim` | Sandstorm | TwisterSanSlim |
| `SunakazeKun` | Sandstorm | SunakazeKun |
| `WaterLeakPipe` | WaterLeakPipe | WaterLeakPipe |
| `RosettaChair` | RosettaChair | RosettaChair |
| `GravityLightA` | GravityLight | GravityLightA |
| `GravityLightRoad` | GravityLightRoad | GravityLightRoad |
| `FloaterOtakingFloaterA` | LavaFloater | FloaterOtakingFloaterA |
| `PrologueDirector` | PrologueDirector | DemoLetter |
| `FlexibleSandPlanetPartsA` | FlexibleSphere | FlexibleSandPlanetPartsA |
| `AssemblyBlockPartsA` | AssemblyBlock | AssemblyBlockPartsA |
| `AssemblyBlockPartsB` | AssemblyBlock | AssemblyBlockPartsB |
| `AssemblyBlockPartsC` | AssemblyBlock | AssemblyBlockPartsC |
| `AssemblyBlockPartsD` | AssemblyBlock | AssemblyBlockPartsD |
| `AssemblyBlockPartsE` | AssemblyBlock | AssemblyBlockPartsE |
| `AssemblyBlockPartsIceB` | AssemblyBlock | AssemblyBlockPartsIceB |
| `AssemblyBlockPartsIceC` | AssemblyBlock | AssemblyBlockPartsIceC |
| `AssemblyBlockPartsIceD` | AssemblyBlock | AssemblyBlockPartsIceD |
| `AssemblyBlockPartsIceE` | AssemblyBlock | AssemblyBlockPartsIceE |
| `AssemblyBlockPartsTimerA` | AssemblyBlock | AssemblyBlockPartsTimerA |
| `NutShipFleetBridge` | PhantomShipBridge | NutShipFleetBridge |
| `IceVolcanoFloaterA` | LavaFloater | IceVolcanoFloaterA |
| `IceVolcanoFloaterB` | LavaFloater | IceVolcanoFloaterB |
| `IceMountainFloaterA` | SimpleFloaterObj | IceMountainFloaterA |
| `UFONormalB` | UFOBreakable | UFONormalB |
| `UFONormalD` | UFOBreakable | UFONormalD |
| `UFOStrongA` | UFOSolid | UFOStrongA |
| `UFOBattleStageC` | UFOSolid | UFOBattleStageC |
| `UFOBattleStageD` | UFOSolid | UFOBattleStageD |
| `UFOBattleStageE` | UFOSolid | UFOBattleStageE |
| `UFOBlueStarCupsule` | BlueStarCupsulePlanet | UFOBlueStarCupsule |
| `SubmarineVolcanoBigColumn` | SubmarineVolcanoBigColumn | SubmarineVolcanoBigColumn |
| `SubmarineVolcanoSmallColumn` | SubmarineVolcanoBigColumn | SubmarineVolcanoSmallColumn |
| `PhantomShipHandle` | PhantomShipHandle | PhantomShipHandle |
| `PhantomShipBridge` | PhantomShipBridge | PhantomShipBridge |
| `KoopaRestarterVs3` | KoopaRestarterVs3 |  |
| `KoopaPowerUpSwitch` | KoopaPowerUpSwitch |  |
| `KoopaSwitchKeeper` | KoopaSwitchKeeper |  |
| `KoopaViewSwitchKeeper` | KoopaViewSwitchKeeper |  |
| `KoopaBattleMapStair` | KoopaBattleMapStair | KoopaBattleMapStair |
| `KoopaBattleMapPlate` | KoopaBattleMapPlate | KoopaPlate |
| `KoopaBattleMapCoinPlate` | KoopaBattleMapCoinPlate | KoopaPlateCoin |
| `KoopaBattleMapStairBig` | KoopaBattleMapStair | KoopaBattleMapStairBig |
| `KoopaBattleMapStairTurn` | KoopaBattleMapStair | KoopaBattleMapStairTurn |
| `TreasureBoxCrackedEmpty` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxCrackedCoin` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxCrackedYellowChip` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxCrackedBlueChip` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxCrackedKinokoOneUp` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxCrackedKinokoLifeUp` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxCrackedAirBubble` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxCrackedPowerStar` | TreasureBoxCracked | TreasureBoxCracked |
| `TreasureBoxEmpty` | TreasureBoxCracked | TreasureBox |
| `TreasureBoxCoin` | TreasureBoxCracked | TreasureBox |
| `TreasureBoxYellowChip` | TreasureBoxCracked | TreasureBox |
| `TreasureBoxBlueChip` | TreasureBoxCracked | TreasureBox |
| `TreasureBoxKinokoOneUp` | TreasureBoxCracked | TreasureBox |
| `TreasureBoxKinokoLifeUp` | TreasureBoxCracked | TreasureBox |
| `TreasureBoxGoldEmpty` | TreasureBoxCracked | TreasureBoxGold |
| `PhantomShipBoxFloaterA` | PhantomShipBoxFloater | PhantomShipBoxFloaterA |
| `PhantomShipBoxFloaterB` | PhantomShipBoxFloater | PhantomShipBoxFloaterB |
| `PhantomShipBoxFloaterC` | PhantomShipBoxFloater | PhantomShipBoxFloaterC |
| `GhostShipCavePipeCollision` | InvisiblePolygonObj | GhostShipCavePipeCollision |
| `AstroCountDownPlate` | AstroCountDownPlate | AstroCountDownPlate |
| `AstroTorchLightRed` | AstroEffectObj |  |
| `AstroTorchLightBlue` | AstroEffectObj |  |
| `EffectTicoS` | AstroEffectObj |  |
| `EffectTicoL` | AstroEffectObj |  |
| `AstroDomeSky` | AstroDomeSky |  |
| `VROrbit` | ProjectionMapSky | VROrbit |
| `VRDarkSpace` | Sky | VRDarkSpace |
| `VRSandwichSun` | ProjectionMapSky | VRSandwichSun |
| `SummerSky` | Sky | SummerSky |
| `GalaxySky` | Sky | GalaxySky |
| `MilkyWaySky` | Sky | MilkyWaySky |
| `HalfGalaxySky` | ProjectionMapSky | HalfGalaxySky |
| `GreenPlanetOrbitSky` | ProjectionMapSky | GreenPlanetOrbitSky |
| `PhantomSky` | Sky | PhantomSky |
| `KoopaVS1Sky` | ProjectionMapSky | KoopaVS1Sky |
| `KoopaVS2Sky` | Sky | KoopaVS2Sky |
| `FamicomMarioSky` | Sky | FamicomMarioSky |
| `DesertSky` | Sky | DesertSky |
| `ChildRoomSky` | Sky | ChildRoomSky |
| `AuroraSky` | Sky | AuroraSky |
| `CloudSky` | ProjectionMapSky | CloudSky |
| `RockPlanetOrbitSky` | ProjectionMapSky | RockPlanetOrbitSky |
| `StarrySky` | Sky | StarrySky |
| `AstroDomeSkyA` | Sky | AstroDomeSkyA |
| `HomeAir` | Air | HomeAir |
| `SphereAir` | PriorDrawAir | SphereAir |
| `SunsetAir` | ProjectionMapAir | SunsetAir |
| `FineAir` | ProjectionMapAir | FineAir |
| `DimensionAir` | AirFar100m | DimensionAir |
| `DarknessRoomAir` | Air | DarknessRoomAir |
| `TwilightAir` | Air | TwilightAir |
| `QuestionCoin` | QuestionCoin | QuestionCoin |
| `BlueChip` | BlueChip | BlueChip |
| `YellowChip` | YellowChip | YellowChip |
| `StarPiece` | StarPiece | StarPiece |
| `StarPieceFlow` | StarPieceGroup |  |
| `StarPieceGroup` | StarPieceGroup |  |
| `StarPieceFollowGroup` | StarPieceFollowGroup |  |
| `StarPieceMother` | StarPieceMother | StarPieceMother |
| `StarPieceSpot` | StarPieceSpot |  |
| `OceanBowl` | OceanBowl | WaterWave |
| `OceanRing` | OceanRing | OceanRing |
| `OceanRingAndFlag` | OceanRing | OceanRing |
| `OceanSphere` | OceanSphere | WaterWave |
| `WhirlPool` | WhirlPool |  |
| `WhirlPoolAccelerator` | WhirlPoolAccelerator | Whirlpool |
| `FishGroupA` | FishGroup | FishA |
| `FishGroupB` | FishGroup | FishB |
| `FishGroupC` | FishGroup | FishC |
| `FishGroupD` | FishGroup | FishD |
| `FishGroupE` | FishGroup | FishE |
| `FishGroupF` | FishGroup | FishF |
| `Flag` | Flag | Flag |
| `FlagKoopaA` | Flag | FlagKoopaA |
| `FlagKoopaB` | Flag | FlagKoopaB |
| `FlagRaceA` | Flag | FlagRaceA |
| `FlagKoopaCastle` | Flag | FlagKoopaCastle |
| `FlagPeachCastleA` | Flag | FlagPeachCastleA |
| `FlagPeachCastleB` | Flag | FlagPeachCastleB |
| `FlagPeachCastleC` | Flag | FlagPeachCastleC |
| `FlagSurfing` | Flag | FlagSurfing |
| `FlagTamakoro` | Flag | FlagTamakoro |
| `SeaGullGroup` | SeaGullGroup | SeaGull |
| `WaterPlant` | WaterPlant | WaterPlant |
| `HitWallTimerSwitch` | HitWallTimerSwitch | HitWallTimerSwitch |
| `PTimerSwitch` | PTimerSwitch | PTimerSwitch |
| `CrystalSwitch` | CrystalSwitch | CrystalSwitch |
| `HipDropSwitch` | HipDropSwitch | HipDropSwitch |
| `HipDropTimerSwitch` | HipDropTimerSwitch | HipDropTimerSwitch |
| `DesertLandUpSwitch` | DesertLandMoveSwitch | DesertLandUpSwitch |
| `DesertLandDownSwitch` | DesertLandMoveSwitch | DesertLandDownSwitch |
| `ScrewSwitch` | ScrewSwitch | ScrewSwitch |
| `ScrewSwitchReverse` | ScrewSwitchReverse | ScrewSwitchReverse |
| `PicketSwitch` | PicketSwitch | PicketSwitch |
| `PicketSwitchBig` | PicketSwitch | PicketSwitchBig |
| `ValveSwitch` | ValveSwitch | ValveSwitch |
| `KeySwitch` | KeySwitch | KeySwitch |
| `SpinLeverSwitch` | SpinLeverSwitch | SpinLeverSwitch |
| `EffectObjSample` | EffectObjR1000F50 |  |
| `ShootingStarArea` | RandomEffectObj |  |
| `IcePlanetLight` | EffectObjR100F50SyncClipping |  |
| `IceLayerBreak` | EffectObjR500F50 |  |
| `LavaSparksS` | EffectObj20x20x10SyncClipping |  |
| `LavaSparksL` | EffectObj50x50x10SyncClipping |  |
| `SpaceDustS` | EffectObj20x20x10SyncClipping |  |
| `SpaceDustL` | EffectObj50x50x10SyncClipping |  |
| `BirdLouseS` | EffectObj20x20x10SyncClipping |  |
| `BirdLouseL` | EffectObj50x50x10SyncClipping |  |
| `SnowS` | EffectObj10x10x10SyncClipping |  |
| `FallingSmallRock` | FallingSmallRock |  |
| `LavaHomeVolcanoFallingRock` | EffectObjR1000F50 |  |
| `Fountain` | Fountain |  |
| `FountainBig` | FountainBig | FountainBig |
| `WaterfallS` | EffectObjR1000F50 |  |
| `WaterfallL` | EffectObjR1000F50 |  |
| `DrainPipeBubbleA` | EffectObjR500F50 |  |
| `DrainPipeBubbleB` | EffectObjR500F50 |  |
| `WaterDropTop` | EffectObjR1000F50 |  |
| `WaterDropBottom` | EffectObjR1000F50 |  |
| `WaterDropMiddle` | EffectObjR1000F50 |  |
| `SubmarineSteam` | SubmarineSteam |  |
| `SubmarineVolcano` | SubmarineSteam |  |
| `WaterLayerBreak` | EffectObjR500F50 |  |
| `StarPieceClusterBreak` | EffectObjR500F50 |  |
| `FireworksA` | RandomEffectObj |  |
| `BattleShipExplosionRock` | EffectObjR500F50 |  |
| `BattleShipExplosionMetal` | EffectObjR500F50 |  |
| `ForestWaterfallS` | EffectObjR1000F50 |  |
| `ForestWaterfallL` | EffectObjR1000F50 |  |
| `SandBreezeS` | EffectObj20x20x10SyncClipping |  |
| `SandBreezeL` | EffectObj50x50x10SyncClipping |  |
| `SpringJetWater` | SpringJetWater |  |
| `SandUpDownKGBaseEffect` | SandUpDownEffectObj |  |
| `SandUpDownTowerTopEffect` | SandUpDownEffectObj |  |
| `UFOSandObstacleAEffect` | SandUpDownEffectObj |  |
| `UFOSandObstacleBEffect` | SandUpDownEffectObj |  |
| `UFOSandObstacleCEffect` | SandUpDownEffectObj |  |
| `GravityDust` | EffectObjGravityDust |  |
| `Steam` | SimpleEffectObj |  |
| `FallRedLeaf` | EffectObj10x10x10SyncClipping |  |
| `FallGreenLeaf` | EffectObj10x10x10SyncClipping |  |
| `IcicleRockLight` | EffectObjR100F50SyncClipping |  |
| `TwinFallLakeWaterFall` | EffectObjR1000F50 |  |
| `EffectTeresa` | EffectObj50x50x10SyncClipping |  |
| `UFOKinokoLandingBlackSmoke` | EffectObjR500F50 |  |
| `InvisibleWall10x10` | InvisiblePolygonObj | InvisibleWall10x10 |
| `InvisibleWall10x20` | InvisiblePolygonObj | InvisibleWall10x20 |
| `InvisibleWallJump10x10` | InvisiblePolygonObj | InvisibleWallJump10x10 |
| `InvisibleWallJump10x20` | InvisiblePolygonObj | InvisibleWallJump10x20 |
| `InvisibleWallGCapture10x10` | InvisiblePolygonObjGCapture | InvisibleWallGCapture10x10 |
| `InvisibleWallGCapture10x20` | InvisiblePolygonObjGCapture | InvisibleWallGCapture10x20 |
| `PolygonCodeRecoveryPlate` | InvisiblePolygonObj | PolygonCodeRecoveryPlate |
| `PolygonCodeRecoveryBowl` | InvisiblePolygonObj | PolygonCodeRecoveryBowl |
| `InvisibleWallBattleShipA` | TransparentWall | InvisibleWallBattleShipA |
| `InvisibleWaterfall` | TransparentWall | InvisibleWaterfall |
| `InvisibleWaterfallTwinFallLake` | TransparentWall | InvisibleWaterfallTwinFallLake |
| `StatusFloor` | TransparentWall | StatusFloor |
| `MiniEggStarGalaxy` | MiniatureGalaxy |  |
| `MiniHoneyBeeKingdomGalaxy` | MiniatureGalaxy |  |
| `MiniFlipPanelExGalaxy` | MiniatureGalaxy |  |
| `MiniSurfingLv1Galaxy` | MiniatureGalaxy |  |
| `MiniTriLegLv1Galaxy` | MiniatureGalaxy |  |
| `MiniStarDustGalaxy` | MiniatureGalaxy |  |
| `MiniTamakoroExLv1Galaxy` | MiniatureGalaxy |  |
| `MiniBattleShipGalaxy` | MiniatureGalaxy |  |
| `MiniBreakDownPlanetGalaxy` | MiniatureGalaxy |  |
| `MiniKoopaBattleVs1Galaxy` | MiniatureGalaxy |  |
| `MiniHeavenlyBeachGalaxy` | MiniatureGalaxy |  |
| `MiniCubeBubbleExLv1Galaxy` | MiniatureGalaxy |  |
| `MiniPhantomGalaxy` | MiniatureGalaxy |  |
| `MiniOceanFloaterLandGalaxy` | MiniatureGalaxy |  |
| `MiniKoopaJrShipLv1Galaxy` | MiniatureGalaxy |  |
| `MiniCosmosGardenGalaxy` | MiniatureGalaxy |  |
| `MiniIceVolcanoGalaxy` | MiniatureGalaxy |  |
| `MiniHoneyBeeExGalaxy` | MiniatureGalaxy |  |
| `MiniSandClockGalaxy` | MiniatureGalaxy |  |
| `MiniKoopaBattleVs2Galaxy` | MiniatureGalaxy |  |
| `MiniReverseKingdomGalaxy` | MiniatureGalaxy |  |
| `MiniOceanRingGalaxy` | MiniatureGalaxy |  |
| `MiniFactoryGalaxy` | MiniatureGalaxy |  |
| `MiniSkullSharkGalaxy` | MiniatureGalaxy |  |
| `MiniFloaterOtaKingGalaxy` | MiniatureGalaxy |  |
| `MiniOceanPhantomCaveGalaxy` | MiniatureGalaxy |  |
| `MiniCannonFleetGalaxy` | MiniatureGalaxy |  |
| `MiniDarkRoomGalaxy` | MiniatureGalaxy |  |
| `MiniHellProminenceGalaxy` | MiniatureGalaxy |  |
| `MiniKoopaBattleVs3Galaxy` | MiniatureGalaxy |  |
| `SurprisedGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpBeltConveyerExGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpCocoonExGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpTearDropGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpTeresaMario2DGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpSnowCapsuleGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpTransformationExGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpFishTunnelGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpTamakoroExLv2Galaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpSurfingLv2Galaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpCubeBubbleExLv2Galaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `SurpPeachCastleFinalGalaxy` | SurprisedGalaxy | MiniSurprisedGalaxy |
| `ClipFieldSwitch` | ClipFieldSwitch |  |
| `GroupSwitchWatcher` | GroupSwitchWatcher |  |
| `TimerSwitch` | TimerSwitch |  |
| `MovieStarter` | MovieStarter | CinemaSuper |
| `AstroDomeDemoStarter` | AstroDomeDemoStarter | SuperSpinDriver |
| `AstroDomeGalaxySelector` | AstroDomeGalaxySelector |  |
| `StarReturnDemoStarter` | StarReturnDemoStarter |  |
| `GrandStarReturnDemoStarter` | GrandStarReturnDemoStarter | GrandStar |
| `ScenarioStarter` | ScenarioStarter | Welcome |
| `FileSelector` | FileSelector |  |

