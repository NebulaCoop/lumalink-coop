# Player numbers

Every number in the game's table of how the player moves, by the name a layer's `player_constants` and the plugin calls use. Written from the game's sources.

```json
"player_constants": {
  "mario": { "mJumpHeight": [null, null, 60.0] },
  "luigi": { "mJumpHeight": [null, null, 45.0] },
  "both":  { "mFireModeTime": 2400 },
  "wario": { "mJumpHeight": [20.0, null, 30.0] }
}
```

Another key is a character's id from a layer's `characters` (this mod's or another one running, NebulaCoop): its own numbers, first its base's as the game has them. `both` is Mario and Luigi.

A number is a number; a list sets places in order, and `null` keeps the game's number at that place. Times are in frames, at 60 a second. Speeds are units a frame; a unit is a centimetre. A plugin reads and writes these with `player_constant` and `set_player_constant`, at any time.

`lumalink list player` prints the names.

| Name | Kind | Reads as | Mario | Luigi |
|---|---|---|---|---|
| `mJumpHeight` | number, 9 of them | Jump height | {22.0, 26.0, 36.0, 24.0, 32.0, 15.0, 32.0, 22.0, 1.0} | {22.0, 26.0, 34.71, 24.0, 32.0, 15.0, 32.0, 22.0, 1.0} |
| `mGravityJumping` | number, 12 of them | Gravity jumping | {1.8, 1.8, 1.8, 1.8, 1.8, 0.95, 1.8, 2.2, 0.4, 0.9, 1.8, 0.5} | {1.6, 1.6, 1.6, 1.6, 1.6, 0.9, 1.6, 2.0, 0.35, 0.9, 1.6, 0.4} |
| `mGravityAirWalk` | number | Gravity air walk | 0.1 | same |
| `mGravityRatioA` | number | Gravity ratio a | 0.5 | same |
| `mJumpFrontSpeed` | number | Jump front speed | 12.5 | same |
| `mJumpTornadoSpeed` | number | Jump tornado speed | 5.0 | same |
| `mDropFrontSpeed` | number | Drop front speed | 30.0 | same |
| `mJumpConnectTime` | whole | Jump connect time | 7 | same |
| `mJumpConnectSpeed` | number | Jump connect speed | 0.6 | same |
| `mSquatJumpFrontSpeed` | number | Squat jump front speed | 16.5 | same |
| `mSquatJumpBackSpeed` | number | Squat jump back speed | 5.0 | same |
| `mTurnJumpFrontSpeed` | number | Turn jump front speed | 5.0 | same |
| `mMaxDropSpeed` | number, 5 of them | Max drop speed | {35.0, 20.0, 5.0, 6.0, 15.0} | same |
| `mGrReductionDropTimer` | whole | Gr reduction drop timer | 30 | same |
| `mDropWaitTime` | whole | Drop wait time | 5 | same |
| `mJumpFrontReduction` | number | Jump front reduction | 0.98 | same |
| `mJumpFrontReductionBeginTime` | whole | Jump front reduction begin time | 15 | same |
| `mSquatJumpFrontReduction` | number | Squat jump front reduction | 0.99 | same |
| `mSquatJumpFrontReductionBTime` | whole | Squat jump front reduction b time | 30 | same |
| `mMudFloorJumpWeakRatio` | number | Mud floor jump weak ratio | 0.5 | same |
| `mGravityHipDrop` | number | Gravity hip drop | 12.0 | same |
| `mLimitSpeedHipDrop` | number | Limit speed hip drop | 150.0 | same |
| `mHipDropLimitHeight` | number | Hip drop limit height | 100.0 | same |
| `mHipDropZeroGrTime` | whole | Hip drop zero gr time | 2 | same |
| `mHipDropLimitTimerAfterTornado` | whole | Hip drop limit timer after tornado | 15 | same |
| `mSlipUpSpdCtrlTimer` | whole | Slip up spd ctrl timer | 8 | same |
| `mSlipUpSpdRatio` | number | Slip up spd ratio | 0.9 | same |
| `mSlipUpHeight` | number | Slip up height | 16.0 | same |
| `mSlipUpFront` | number | Slip up front | 8.0 | same |
| `mSlipUpFrontWeak` | number | Slip up front weak | 3.0 | same |
| `mSlipUpHeightHang` | number | Slip up height hang | 12.0 | same |
| `mSlipUpFrontHang` | number | Slip up front hang | 5.0 | same |
| `mSlipUpContinueHeight` | number | Slip up continue height | 150.0 | same |
| `mAirWalkTime` | whole | Air walk time | 180 | same |
| `mAirWalkTimerFact1` | number | Air walk timer fact 1 | 3.0 | same |
| `mAirWalkTimerFact2` | whole | Air walk timer fact 2 | 300 | same |
| `mAirWalkTimerFact3` | number | Air walk timer fact 3 | 0.15 | same |
| `mMaxJumpSpeed` | number | Max jump speed | 32.0 | same |
| `mAirWalkBackBonus` | number | Air walk back bonus | 1.3 | same |
| `mAirWalkSpeedKiller` | number | Air walk speed killer | 0.1 | same |
| `mMaxBackJumpSpeed` | number | Max back jump speed | 10.0 | same |
| `mAirWalkBonus` | number | Air walk bonus | 3.0 | same |
| `mBackJumpLimitFrame` | whole | Back jump limit frame | 60 | same |
| `mBackJumpRatio` | number | Back jump ratio | 3.0 | same |
| `mWaitNeutralTimer` | whole | Wait neutral timer | 15 | same |
| `mHardLandingHeight` | number | Hard landing height | 2500.0 | same |
| `mHardLandStunTimer` | whole | Hard land stun timer | 60 | same |
| `mCodeJumpPower` | number, 4 of them | Code jump power | {30.0, 60.0, 100.0, 30.0} | same |
| `mTrampleNormal` | number | Trample normal | 19.0 | same |
| `mTrampleLong` | number | Trample long | 4.0 | 6.0 |
| `mTrampleNormalTaco` | number | Trample normal taco | 28.0 | same |
| `mTrampleBegoma` | number | Trample begoma | 40.0 | same |
| `mTrampleBegomaRotRise` | number | Trample begoma rot rise | 0.6 | same |
| `mTrampleBegomaRotFall` | number | Trample begoma rot fall | 0.25 | same |
| `mTrampleBegomaOpenTime` | whole | Trample begoma open time | 12 | same |
| `mDropUnderPowerMin` | number | Drop under power min | 10.0 | same |
| `mDropUnderPowerMax` | number | Drop under power max | 20.0 | same |
| `mMsgJumpGroundPowerXZ` | number | Msg jump ground power xz | 10.0 | same |
| `mMsgJumpGroundPowerY` | number | Msg jump ground power y | 20.0 | same |
| `mMsgJumpAirPower` | number | Msg jump air power | 15.0 | same |
| `mBilliardJumpExtra` | number | Billiard jump extra | 5.0 | same |
| `mFutureJumpReqLimitTime` | whole | Future jump req limit time | 4 | same |
| `mWarpPodJumpX` | number | Warp pod jump x | 20.0 | same |
| `mWarpPodJumpY` | number | Warp pod jump y | 30.0 | same |
| `mJumpHeightDamage` | number | Jump height damage | 15.0 | same |
| `mJumpDistDamage` | number | Jump dist damage | 9.0 | same |
| `mGravityDamage` | number | Gravity damage | 1.0 | same |
| `mJumpHeightLargeDamage` | number | Jump height large damage | 25.0 | same |
| `mJumpDistLargeDamage` | number | Jump dist large damage | 20.0 | same |
| `mSlideDistFaint` | number | Slide dist faint | 15.0 | same |
| `mFaintTimer1` | whole | Faint timer 1 | 15 | same |
| `mFaintTimer2` | whole | Faint timer 2 | 15 | same |
| `mFaintFriction1` | number | Faint friction 1 | 0.9 | same |
| `mFaintFriction2` | number | Faint friction 2 | 0.9 | same |
| `mSlideDistFlip` | number | Slide dist flip | 15.0 | same |
| `mSlideDistFlipSpin` | number | Slide dist flip spin | 7.5 | same |
| `mFlipTimer1` | whole | Flip timer 1 | 15 | same |
| `mFlipTimer2` | whole | Flip timer 2 | 15 | same |
| `mFlipFriction1` | number | Flip friction 1 | 0.98 | same |
| `mFlipFriction2` | number | Flip friction 2 | 0.97 | same |
| `mFlipFriction3` | number | Flip friction 3 | 0.6 | same |
| `mFireRunTimer1` | whole | Fire run timer 1 | 40 | same |
| `mFireRunTimer2` | whole | Fire run timer 2 | 30 | same |
| `mFireRunTimer3` | whole | Fire run timer 3 | 15 | same |
| `mFireRunSpeed` | number | Fire run speed | 16.0 | same |
| `mFireRunTurnRatio` | number | Fire run turn ratio | 0.1 | same |
| `mFireRunGravity` | number | Fire run gravity | 5.0 | same |
| `mFireRunFirstJump` | number | Fire run first jump | 50.0 | same |
| `mFireDanceTurnRatio` | number | Fire dance turn ratio | 0.2 | same |
| `mFireDanceGravityRise` | number | Fire dance gravity rise | 1.0 | same |
| `mFireDanceGravityDrop` | number | Fire dance gravity drop | 0.75 | same |
| `mFireDanceFirstJump` | number | Fire dance first jump | 40.0 | same |
| `mFireDanceSecondJump` | number | Fire dance second jump | 20.0 | same |
| `mFireDanceMoveSpeed` | number | Fire dance move speed | 10.0 | same |
| `mFireDanceMoveAcc` | number | Fire dance move acc | 0.5 | same |
| `mBlackHoleRadiusRatio` | number | Black hole radius ratio | 0.992 | same |
| `mBlackHoleScaleSpeed` | number | Black hole scale speed | 0.002 | same |
| `mBlackHoleScaleLimit` | number | Black hole scale limit | 0.7 | same |
| `mBlackHoleRotateSpeed` | number | Black hole rotate speed | 0.0018 | same |
| `mBlackHoleRotateLimit` | number | Black hole rotate limit | 0.7 | same |
| `mBlackHoleFirstRadius` | number | Black hole first radius | 1.0 | same |
| `mBlackHoleHideTime` | whole | Black hole hide time | 165 | same |
| `mWaterInnerFaintTime` | whole | Water inner faint time | 60 | same |
| `mSwimFaintSpeed` | number | Swim faint speed | 10.0 | same |
| `mWaterInnerDamageTime` | whole | Water inner damage time | 60 | same |
| `mSwimDamageSpeed` | number | Swim damage speed | 10.0 | same |
| `mSwimSurfaceDamageSpeed` | number | Swim surface damage speed | 5.0 | same |
| `mWaterSurfaceDamageJump` | number | Water surface damage jump | 26.0 | same |
| `mSwimDamageTimerEx1` | whole | Swim damage timer ex 1 | 90 | same |
| `mSwimDamageSpeedEx1` | number | Swim damage speed ex 1 | 35.0 | same |
| `mJumpHeightFlow` | number | Jump height flow | 18.0 | same |
| `mJumpDistFlow` | number | Jump dist flow | 2.0 | same |
| `mGravityFlow` | number | Gravity flow | 0.3 | same |
| `mFlatAngle` | number | Flat angle | 45.0 | same |
| `mSlipAngle` | number | Slip angle | 60.0 | same |
| `mForceWallAngle` | number | Force wall angle | 85.0 | same |
| `mDebugMoveSpeed` | number | Debug move speed | 120.0 | same |
| `mWalkSpeed` | number | Walk speed | 13.0 | 15.0 |
| `mInertiaStandardStop` | number | Inertia standard stop | 0.94 | 0.99 |
| `mInertiaStandardMax` | number | Inertia standard max | 0.9 | 0.92 |
| `mInertiaBdash` | number | Inertia bdash | 0.98 | same |
| `mInertiaBdashAfter` | number | Inertia bdash after | 0.992 | same |
| `mInertiaStop` | number | Inertia stop | 0.85 | 0.95 |
| `mInertiaSquat` | number | Inertia squat | 0.95 | 0.97 |
| `mInertiaTurnSlip` | number | Inertia turn slip | 0.91 | 0.98 |
| `mInertiaTurning` | number | Inertia turning | 0.85 | 0.9 |
| `mInertiaJumpFinish` | number | Inertia jump finish | 0.8 | same |
| `mInertiaReflectSlip` | number | Inertia reflect slip | 1.0 | same |
| `mInertiaStartSpin` | number | Inertia start spin | 0.95 | same |
| `mInertiaOverSpeed` | number | Inertia over speed | 0.8 | same |
| `mTurnAngleSpeed` | number | Turn angle speed | 0.9 | same |
| `mTurnAngleSpeed2` | number | Turn angle speed 2 | 0.15 | same |
| `mTurnAngleSpeed3` | number | Turn angle speed 3 | 0.1 | same |
| `mTurnAngleSpeedSlowWalk` | number | Turn angle speed slow walk | 0.04 | same |
| `mGravityStandard` | number | Gravity standard | 30.0 | same |
| `mGravityGrounding` | number | Gravity grounding | 20.0 | same |
| `mDashMultiply` | number | Dash multiply | 2.0 | same |
| `mCameraAngleLimit` | number | Camera angle limit | 0.05 | same |
| `mStartSpinTime` | whole | Start spin time | 30 | same |
| `mDashAfterTime` | whole | Dash after time | 40 | same |
| `mAutoBdashTime` | whole | Auto bdash time | 10000 | same |
| `mSlowStartTime` | whole | Slow start time | 10 | 20 |
| `mStartSpinAnimeRatio` | number | Start spin anime ratio | 1.25 | same |
| `mSlopeAnimeRatio` | number | Slope anime ratio | 0.5 | same |
| `mSlopeSpinAnimeRatio` | number | Slope spin anime ratio | 0.75 | same |
| `mItemDashRatio` | number | Item dash ratio | 1.5 | same |
| `mItemDashTimer` | whole | Item dash timer | 180 | same |
| `mInertiaIceStandardStop` | number | Inertia ice standard stop | 0.985 | same |
| `mInertiaIceStandardMax` | number | Inertia ice standard max | 0.995 | same |
| `mInertiaIceStartSpin` | number | Inertia ice start spin | 0.98 | same |
| `mInertiaIceStop` | number | Inertia ice stop | 0.992 | same |
| `mInertiaIceTurn` | number | Inertia ice turn | 0.2 | same |
| `mInertiaSlipStandardStop` | number | Inertia slip standard stop | 0.98 | same |
| `mInertiaSlipStandardMax` | number | Inertia slip standard max | 0.99 | same |
| `mInertiaSlipStartSpin` | number | Inertia slip start spin | 0.985 | same |
| `mInertiaSlipStop` | number | Inertia slip stop | 0.98 | same |
| `mInertiaSlipTurn` | number | Inertia slip turn | 0.98 | same |
| `mTurnSlipAngle` | number | Turn slip angle | 2.6 | same |
| `mTurnReadyTime` | whole | Turn ready time | 10 | 20 |
| `mTurnSlipNeutral` | number | Turn slip neutral | 0.7 | same |
| `mSlipSpeed` | number | Slip speed | 0.25 | same |
| `mFastTurnSpeed` | number | Fast turn speed | 0.1 | same |
| `mTurnSlipTime` | whole | Turn slip time | 6 | same |
| `mTurnSlipTimeB` | whole | Turn slip time b | 60 | same |
| `mTurnJumpInhibitTime` | whole | Turn jump inhibit time | 0 | same |
| `mLandTurnHeight` | number | Land turn height | 120.0 | same |
| `mInertiaBrake` | number | Inertia brake | 0.85 | 0.95 |
| `mStandingTurnTime` | whole | Standing turn time | 10 | same |
| `mWeakTurnTime` | whole | Weak turn time | 15 | same |
| `mBrakeFirstTimer` | whole | Brake first timer | 15 | 60 |
| `mBrakeSecondTimer` | whole | Brake second timer | 33 | 60 |
| `mSlopeSpeedMax` | number | Slope speed max | 22.0 | same |
| `mSlopeAccel` | number | Slope accel | 0.01 | same |
| `mSlopeSpeedMaxBraking` | number | Slope speed max braking | 20.0 | same |
| `mSlopeFinishInertia` | number | Slope finish inertia | 0.95 | same |
| `mSlopeCancelInertia` | number | Slope cancel inertia | 0.94 | same |
| `mSlopeAnimeFinishSpeed` | number | Slope anime finish speed | 15.0 | same |
| `mSlopeDashAngleFactor` | number | Slope dash angle factor | 20.0 | same |
| `mSlopeDashSpeedFactor` | number | Slope dash speed factor | 0.1 | same |
| `mSlopeDashAccelFactor` | number | Slope dash accel factor | 50.0 | same |
| `mSlopeDashAccelTime` | whole | Slope dash accel time | 75 | same |
| `mSlopeSideMoveInertia` | number | Slope side move inertia | 0.95 | same |
| `mSlopeSideStopInertia` | number | Slope side stop inertia | 0.98 | same |
| `mSlopeSideMoveSpeed` | number | Slope side move speed | 10.0 | same |
| `mSlopeCurveAssist` | number | Slope curve assist | 0.05 | same |
| `mSlipMoveTurnAngleRad` | number | Slip move turn angle rad | 0.03 | same |
| `mWalkStepHeight` | number | Walk step height | 50.0 | same |
| `mTiltRatio` | number | Tilt ratio | 40.0 | same |
| `mLookDownRatio` | number | Look down ratio | 0.4 | same |
| `mSpeedStep` | number, 3 of them | Speed step | {0.05, 0.75, 0.95} | same |
| `mMaxAnmSpeedA` | number | Max anm speed a | 6.0 | same |
| `mMinAnmSpeedA` | number | Min anm speed a | 2.5 | same |
| `mMaxAnmSpeedB` | number | Max anm speed b | 4.0 | same |
| `mMinAnmSpeedB` | number | Min anm speed b | 2.5 | same |
| `mMaxAnmSpeedC` | number | Max anm speed c | 1.33 | same |
| `mMinAnmSpeedC` | number | Min anm speed c | 0.67 | same |
| `mWeightBlendRatio` | number | Weight blend ratio | 0.2 | same |
| `mMudFloorSlipRatio` | number | Mud floor slip ratio | 2.0 | same |
| `mStickAngleMargin` | number | Stick angle margin | 0.1 | same |
| `mStickMarginX` | number | Stick margin x | 0.25 | same |
| `mStickMarginY` | number | Stick margin y | 0.2 | same |
| `mStickMarginYstart` | number | Stick margin ystart | 0.5 | same |
| `mStickMarginXstart` | number | Stick margin xstart | 0.5 | same |
| `mStickHeavyMinRatio` | number | Stick heavy min ratio | 0.3 | same |
| `mStickHeavyMinAngle` | number | Stick heavy min angle | 0.7854 | same |
| `mStickHeavyMaxAngle` | number | Stick heavy max angle | 1.5708 | same |
| `mSpeedSquatWalkLower` | number | Speed squat walk lower | 0.05 | same |
| `mSquatWalkStep` | number | Squat walk step | 6.0 | same |
| `mSquatWalkMaxSpeed` | number | Squat walk max speed | 0.2 | same |
| `mSquatWalkMinSpeed` | number | Squat walk min speed | 0.1 | same |
| `mWindSlideLimit` | number | Wind slide limit | 1.0 | same |
| `mWindSlideFriction` | number | Wind slide friction | 1.2 | same |
| `mWindForwardFriction` | number | Wind forward friction | 0.5 | same |
| `mWindSlideFrictionAgainst` | number | Wind slide friction against | 0.8 | same |
| `mWindJumpingFriction` | number | Wind jumping friction | 0.1 | same |
| `mTornadoTime` | whole | Tornado time | 45 | same |
| `mTornadoTimeAir` | whole | Tornado time air | 150 | same |
| `mAirWalkTimeTornado` | whole | Air walk time tornado | 150 | same |
| `mTornadoRestartTime` | whole | Tornado restart time | 60 | same |
| `mSpinDifferMargin` | number | Spin differ margin | 1.5 | same |
| `mSpinGoalAngle` | number | Spin goal angle | 4.5 | same |
| `mSpinInputLimitTime` | whole | Spin input limit time | 25 | same |
| `mSpinTime` | whole | Spin time | 120 | same |
| `mSpinSpeed` | number | Spin speed | 1.8 | same |
| `mSpinWalkRatio` | number | Spin walk ratio | 1.1 | same |
| `mSpinJumpRatio` | number | Spin jump ratio | 1.3 | same |
| `mSpinWallReboundSpeed` | number | Spin wall rebound speed | 2.0 | same |
| `mSpinFinishTime` | whole | Spin finish time | 49 | same |
| `mSpinFinishRate` | number | Spin finish rate | 0.2 | same |
| `mTornadoMultiply` | number | Tornado multiply | 0.5 | same |
| `mTurnAngleSpeedTornado` | number | Turn angle speed tornado | 0.1 | same |
| `mInertiaTornadoAccel` | number | Inertia tornado accel | 0.9 | same |
| `mInertiaTornadoBrake` | number | Inertia tornado brake | 0.96 | same |
| `mTornadoChargeTime` | whole | Tornado charge time | 60 | same |
| `mTornadoChargeUpTime` | whole | Tornado charge up time | 300 | same |
| `mTornadoZeroGravityTimer` | whole | Tornado zero gravity timer | 0 | same |
| `mTornadoBoostPower` | number | Tornado boost power | 0.0001 | same |
| `mTornadoBoostAttn` | number | Tornado boost attn | 1.532 | same |
| `mTornadoBoostTimer` | whole | Tornado boost timer | 28 | same |
| `mTornadoBoostAttnMini` | number | Tornado boost attn mini | 1.6 | same |
| `mTornadoBoostTimerMini` | whole | Tornado boost timer mini | 25 | same |
| `mTornadoTiltCancel` | number | Tornado tilt cancel | 0.96 | same |
| `mTornadoTiltAngle` | number | Tornado tilt angle | 0.33 | same |
| `mTornadoTiltSpeed` | number | Tornado tilt speed | 0.018 | same |
| `mTornadoTiltOffSpeed` | number | Tornado tilt off speed | 0.1 | same |
| `mTornadoTiltNear` | number | Tornado tilt near | 0.4 | same |
| `mTornadoReflectTimer` | whole | Tornado reflect timer | 30 | same |
| `mTornadoReflectSpeed` | number | Tornado reflect speed | 1.0 | same |
| `mAirWalkTimeSpin` | whole | Air walk time spin | 24 | same |
| `mSpinCoinPullRadius` | number | Spin coin pull radius | 600.0 | same |
| `mCoinPullAngleSpeedRatio` | number | Coin pull angle speed ratio | 0.75 | same |
| `mCoinPullDistSpeedRatio` | number | Coin pull dist speed ratio | 0.75 | same |
| `mSpinJumpCount` | whole | Spin jump count | 2 | same |
| `mSpinIntervalTime` | whole | Spin interval time | 45 | same |
| `mSpinJumpHeight` | number | Spin jump height | 12.0 | 12.35 |
| `mSpinJumpGravity` | number | Spin jump gravity | 0.25 | 0.45 |
| `mWallStickGrHeight` | number | Wall stick gr height | 100.0 | same |
| `mWallStickFrHeight` | number | Wall stick fr height | 50.0 | same |
| `mWallStickStepHeight` | number | Wall stick step height | 325.0 | same |
| `mWallDropSpeedStop` | number | Wall drop speed stop | 4.0 | same |
| `mWallDropSpeedNormal` | number | Wall drop speed normal | 14.0 | same |
| `mWallJumpPowerXZ` | number | Wall jump power xz | 13.0 | same |
| `mWallJumpPowerY` | number | Wall jump power y | 30.0 | same |
| `mWallBackJumpPowerXZ` | number | Wall back jump power xz | 6.0 | same |
| `mWallBackJumpPowerY` | number | Wall back jump power y | 10.0 | same |
| `mWallStickTime` | whole | Wall stick time | 15 | same |
| `mWallStickTimeIce` | whole | Wall stick time ice | 60 | same |
| `mWallReleaseTime` | whole | Wall release time | 180 | same |
| `mWallSideMoveRatio` | number | Wall side move ratio | 0.003 | same |
| `mJumpHeightBlown` | number | Jump height blown | 10.0 | same |
| `mGravityBlown` | number | Gravity blown | 1.0 | same |
| `mWallHangGrHeight` | number | Wall hang gr height | 225.0 | same |
| `mWallHangMyHeight` | number | Wall hang my height | 150.0 | same |
| `mHangBlendTime` | whole | Hang blend time | 10 | same |
| `mWallPushAngleRange` | number | Wall push angle range | 20.0 | same |
| `mWallFrontAngleRange` | number | Wall front angle range | 0.5 | same |
| `mWallBackAngleRange` | number | Wall back angle range | 0.707 | same |
| `mWallStickCancelAngle` | number | Wall stick cancel angle | 0.1 | same |
| `mWallBackHangStickPower` | number | Wall back hang stick power | 0.8 | same |
| `mWallBackHangWalkSpeed` | number | Wall back hang walk speed | 0.5 | same |
| `mWallTriJumpMargin` | number | Wall tri jump margin | 0.996 | same |
| `mWallSpinFlipGround` | number | Wall spin flip ground | 5.0 | same |
| `mWallSpinHopGround` | number | Wall spin hop ground | 12.0 | same |
| `mWallSpinFlipAirRatio` | number | Wall spin flip air ratio | 0.75 | same |
| `mClapCoinTime` | whole | Clap coin time | 6 | same |
| `mClapCoinPullRate` | number | Clap coin pull rate | 0.33 | same |
| `mClapJumpBonusRising` | number | Clap jump bonus rising | 10.0 | same |
| `mClapJumpBonusFalling` | number | Clap jump bonus falling | 15.0 | same |
| `mHopLimit` | number | Hop limit | 10.0 | same |
| `mSwimFrontAcc` | number | Swim front acc | 4.0 | same |
| `mSwimFrontMaxSpeed` | number | Swim front max speed | 11.5 | same |
| `mSwimFrontSpinSpeed` | number | Swim front spin speed | 70.0 | same |
| `mSwimFrontNormalSpeed` | number | Swim front normal speed | 7.5 | same |
| `mSwimFrontOnWaterSpeed` | number | Swim front on water speed | 7.5 | same |
| `mSwimFrontJetSpeed` | number | Swim front jet speed | 18.0 | same |
| `mSwimFrontJetSpeedSlow` | number | Swim front jet speed slow | 14.0 | same |
| `mSwimAccRatio` | number | Swim acc ratio | 1.5 | same |
| `mSwimAccMinRatio` | number | Swim acc min ratio | 0.1 | same |
| `mSwimFrontIne` | number | Swim front ine | 0.985 | same |
| `mSwimStopIne` | number | Swim stop ine | 0.992 | same |
| `mSwimStopIneSurface` | number | Swim stop ine surface | 0.98 | same |
| `mSwimRotSpeedX` | number | Swim rot speed x | 0.04 | same |
| `mSwimRotSpeedZ` | number | Swim rot speed z | 0.012 | same |
| `mSwimRotSpeedZStop` | number | Swim rot speed z stop | 2.0 | same |
| `mSwimRotXIne` | number | Swim rot x ine | 0.5 | same |
| `mSwimRotZIne` | number | Swim rot z ine | 0.6 | same |
| `mSwimRotXIneT` | number | Swim rot x ine t | 0.5 | same |
| `mSwimRotZIneT` | number | Swim rot z ine t | 0.6 | same |
| `mSwimUpAcc` | number | Swim up acc | 0.1 | same |
| `mSwimUpMaxSpeed` | number | Swim up max speed | 1.0 | same |
| `mWeightBlendRatioSwim` | number | Weight blend ratio swim | 0.04 | same |
| `mWeightBlendRatioSwimB` | number | Weight blend ratio swim b | 0.2 | same |
| `mSwimFreeXback` | number | Swim free xback | 0.1 | same |
| `mSwimFreeXback2` | number | Swim free xback 2 | 0.04 | same |
| `mSwimSpdYratio` | number | Swim spd yratio | 1.0 | same |
| `mSwimRotSpeedRatioSurface` | number | Swim rot speed ratio surface | 1.0 | same |
| `mJumpSinkTimer` | whole | Jump sink timer | 12 | same |
| `mJumpDamageSinkTimer` | whole | Jump damage sink timer | 40 | same |
| `mSwimToWalkSpd` | number | Swim to walk spd | 10.0 | same |
| `mWaterDecInterval` | whole | Water dec interval | 180 | same |
| `mWaterDecSpinAcc` | whole | Water dec spin acc | 0 | same |
| `mOxygenMax` | whole | Oxygen max | 3600 | same |
| `mSwimXJetRotRatio` | number | Swim x jet rot ratio | 0.4 | same |
| `mSwimXJetRotRatioRed` | number | Swim x jet rot ratio red | 0.35 | same |
| `mTurboReductionStream` | number | Turbo reduction stream | 0.5 | same |
| `mTurboReductionHitWall` | number | Turbo reduction hit wall | 0.1 | same |
| `mTurboReductionHitFloor` | number | Turbo reduction hit floor | 0.995 | same |
| `mSwimJetPeriod1` | whole | Swim jet period 1 | 180 | same |
| `mSwimJetPeriod2` | whole | Swim jet period 2 | 600 | same |
| `mSwimJetPeriod1Ratio` | number | Swim jet period 1 ratio | 2.0 | same |
| `mSwimJetPeriod3Ratio` | number | Swim jet period 3 ratio | 0.5 | same |
| `mZsinkAngleX` | number | Zsink angle x | 3.1 | same |
| `mZsinkMoveTimer` | whole | Zsink move timer | 15 | same |
| `mZsinkSinkTimer` | whole | Zsink sink timer | 50 | same |
| `mZsinkStartTimer` | whole | Zsink start timer | 35 | same |
| `mZsinkFrontSpeed` | number | Zsink front speed | 3.0 | same |
| `mSwimSpecStartTimerA` | whole, 3 of them | Swim spec start timer a | {30, 25, 20} | same |
| `mSwimSpecStartTimerB` | whole, 3 of them | Swim spec start timer b | {40, 30, 25} | same |
| `mSwimSpinReadyTime` | whole | Swim spin ready time | 5 | same |
| `mSwimSpinTime` | whole | Swim spin time | 33 | same |
| `mSwimDashTime` | whole | Swim dash time | 30 | same |
| `mSwimSpinFrontTime` | whole | Swim spin front time | 24 | same |
| `mSwimSpinYspeedTime` | whole | Swim spin yspeed time | 50 | same |
| `mSwimSpinDownStep` | whole | Swim spin down step | 30 | same |
| `mSwimSpinSpeed` | number | Swim spin speed | 0.0 | same |
| `mSwimSpinAcc1` | number | Swim spin acc 1 | 0.95 | same |
| `mSwimSpinAcc2` | number | Swim spin acc 2 | 0.96 | same |
| `mSwimSpinMissOfsTime` | whole | Swim spin miss ofs time | 30 | same |
| `mSwimSpinAccRatio` | number | Swim spin acc ratio | 0.7 | same |
| `mSwimSpinSurfaceAccRatio` | number | Swim spin surface acc ratio | 0.7 | same |
| `mSwimSpinAccRatioR` | number | Swim spin acc ratio r | 0.1 | same |
| `mSwimSpinSurfaceAccRatioR` | number | Swim spin surface acc ratio r | 0.1 | same |
| `mSwimRingDashChargeTime` | whole | Swim ring dash charge time | 15 | same |
| `mSwimRingDashTime` | whole | Swim ring dash time | 300 | same |
| `mSwimRingDashFinishTime` | whole | Swim ring dash finish time | 120 | same |
| `mSwimRingDashSpeedRatio` | number | Swim ring dash speed ratio | 2.5 | same |
| `mSwimSurfaceSpeed` | number | Swim surface speed | 1.8 | same |
| `mSwimWaterSpeed` | number | Swim water speed | 1.6 | same |
| `mSwimDriftSpeed` | number | Swim drift speed | 1.0 | same |
| `mSwimConnectIn` | whole | Swim connect in | 35 | same |
| `mSwimConnectOut` | whole | Swim connect out | 60 | same |
| `mSwimTiltZup` | number | Swim tilt zup | 5.0 | same |
| `mSwimTiltZdown` | number | Swim tilt zdown | 10.0 | same |
| `mSwimTiltReal` | number | Swim tilt real | 1.5 | same |
| `mSwimTiltSpd` | number | Swim tilt spd | 0.95 | same |
| `mSwimUpSurfaceDist` | number | Swim up surface dist | 100.0 | same |
| `mSwimUpStrongDist` | number | Swim up strong dist | 500.0 | same |
| `mSwimUpStrongRatio` | number | Swim up strong ratio | 1.5 | same |
| `mSwimUpWeakDist` | number | Swim up weak dist | 600.0 | same |
| `mSwimUpBottomDist` | number | Swim up bottom dist | 300.0 | same |
| `mSwimUpWeakRatio` | number | Swim up weak ratio | 0.1 | same |
| `mSwimReverseSinkRatio` | number | Swim reverse sink ratio | 8.0 | same |
| `mSliderBrakeIne` | number | Slider brake ine | 0.9 | same |
| `mSliderWeightIne` | number | Slider weight ine | 0.9 | same |
| `mSliderSlopePow` | number | Slider slope pow | 0.5 | same |
| `mSliderWeightPow` | number | Slider weight pow | 0.9 | same |
| `mSliderMaxSpeed` | number | Slider max speed | 32.0 | same |
| `mSliderBrakePow` | number | Slider brake pow | 0.1 | same |
| `mSliderFrontTurnRatio` | number | Slider front turn ratio | 0.08 | same |
| `mSliderHeadRotateRatio` | number | Slider head rotate ratio | 0.08 | same |
| `mSliderTiltRatio` | number | Slider tilt ratio | 0.3 | same |
| `mBeeFlyRandomFactor` | number | Bee fly random factor | 0.0 | same |
| `mBeeFlyConstantFactor` | number | Bee fly constant factor | 5.0 | same |
| `mBeeAirWalkAcc` | number | Bee air walk acc | 0.1 | same |
| `mBeeAirWalkLimit` | number | Bee air walk limit | 8.0 | same |
| `mBeeAirWalkTurnSpd` | number | Bee air walk turn spd | 0.05 | same |
| `mBeeSpeedRotateRatio` | number | Bee speed rotate ratio | 0.02 | same |
| `mBeeFreeDropAcc` | number | Bee free drop acc | 0.1 | same |
| `mBeeFreeDropMaxSpd` | number | Bee free drop max spd | 5.0 | same |
| `mBeePushRiseGravityEraser` | number | Bee push rise gravity eraser | 0.9 | same |
| `mBeeStickJumpPower` | number | Bee stick jump power | 20.0 | same |
| `mBeeStickJumpBonus` | number | Bee stick jump bonus | 0.0 | same |
| `mBeeStickAngleLimit` | number | Bee stick angle limit | 0.3 | same |
| `mBeeUpSpeedMax` | number | Bee up speed max | 5.0 | same |
| `mBeeUpAccelRatio` | number | Bee up accel ratio | 1.0 | same |
| `mBeeAccelRatio` | number | Bee accel ratio | 0.02 | same |
| `mBeeUpDownKiller` | number | Bee up down killer | 0.3 | same |
| `mBeeGravityReviveTime` | whole | Bee gravity revive time | 120 | same |
| `mBeeGravityPowerTime` | whole | Bee gravity power time | 60 | same |
| `mBeeAirWalkInhibitTime` | whole | Bee air walk inhibit time | 30 | same |
| `mBeeGravityPowerTimeV` | whole | Bee gravity power time v | 120 | same |
| `mBeeAirWalkInhibitTimeV` | whole | Bee air walk inhibit time v | 25 | same |
| `mBeeGravityPowerTimeD` | whole | Bee gravity power time d | 60 | same |
| `mBeeAirWalkInhibitTimeD` | whole | Bee air walk inhibit time d | 35 | same |
| `mBeeWallWalkCancelRadius` | number | Bee wall walk cancel radius | 1000.0 | same |
| `mBeeWallWalkCancelRadiusShort` | number | Bee wall walk cancel radius short | 300.0 | same |
| `mBeePoseDelayAngleAir` | number | Bee pose delay angle air | 0.5 | same |
| `mBeePoseDelayAngleGround` | number | Bee pose delay angle ground | 0.13 | same |
| `mBeePoseDelayAccel` | number | Bee pose delay accel | 1.0 | same |
| `mBeePoseLimitAngleAir` | number | Bee pose limit angle air | 0.5 | same |
| `mBeePoseLimitAngleGround` | number | Bee pose limit angle ground | 0.15 | same |
| `mBeePoseHeadToFootLength` | number | Bee pose head to foot length | 120.0 | same |
| `mBeePoseDelaySpeedLimit` | number | Bee pose delay speed limit | 20.0 | same |
| `mBeePoseFrictionStop` | number | Bee pose friction stop | 0.98 | same |
| `mBeePoseTransBlendingRatioStop` | number | Bee pose trans blending ratio stop | 0.8 | same |
| `mBeePoseFrictionMove` | number | Bee pose friction move | 0.95 | same |
| `mBeePoseTransBlendingRatioMove` | number | Bee pose trans blending ratio move | 0.9 | same |
| `mRabbitTurnRatio` | number | Rabbit turn ratio | 0.08 | same |
| `mRabbitTurnRatio2` | number | Rabbit turn ratio 2 | 0.2 | same |
| `mRabbitGravityRise` | number | Rabbit gravity rise | 1.0 | same |
| `mRabbitGravityDrop` | number | Rabbit gravity drop | 0.75 | same |
| `mRabbitFirstJump` | number | Rabbit first jump | 22.0 | same |
| `mRabbitMoveSpeed` | number | Rabbit move speed | 10.0 | same |
| `mRabbitMoveAcc` | number | Rabbit move acc | 0.5 | same |
| `mRabbitMoveAcc2` | number | Rabbit move acc 2 | 0.075 | 0.05 |
| `mRabbitMoveAcc3` | number | Rabbit move acc 3 | 0.25 | same |
| `mRabbitFirstJump2` | number | Rabbit first jump 2 | 60.0 | same |
| `mRabbitGravityRise2` | number | Rabbit gravity rise 2 | 1.2 | same |
| `mRabbitGravityDrop2` | number | Rabbit gravity drop 2 | 1.0 | same |
| `mRabbitChargeTime2` | whole | Rabbit charge time 2 | 23 | same |
| `mHopperLandingTime` | whole | Hopper landing time | 10 | same |
| `mTeresaHorizontalSpeedMax` | number | Teresa horizontal speed max | 10.0 | same |
| `mTeresaAlphaLevelMax` | number | Teresa alpha level max | 8.0 | same |
| `mTeresaAlphaLevelInc` | number | Teresa alpha level inc | 0.5 | same |
| `mTeresaAlphaLevelDec` | number | Teresa alpha level dec | 0.2 | same |
| `mTeresaWallThroughTime` | whole | Teresa wall through time | 90 | same |
| `mTeresaAngleUp` | number | Teresa angle up | 0.5 | same |
| `mTeresaAngleDown` | number | Teresa angle down | 0.7 | 0.75 |
| `mTeresaWallReflectPower` | number | Teresa wall reflect power | 0.0 | 10.0 |
| `mTeresaWallReflectReduction` | number | Teresa wall reflect reduction | 0.98 | 0.95 |
| `mTeresaWallReflectTime` | whole | Teresa wall reflect time | 10 | 30 |
| `mTeresaWaitHeight` | number | Teresa wait height | 50.0 | same |
| `mTeresaDropDownHeight` | number | Teresa drop down height | 50.0 | same |
| `mTeresaBodyRadius` | number | Teresa body radius | 90.0 | same |
| `mTeresaAirWalkTurnSpd` | number | Teresa air walk turn spd | 0.08 | 0.02 |
| `mTeresaDropSpeedMax` | number | Teresa drop speed max | 0.8 | 0.5 |
| `mTeresaRiseSpeedMax` | number | Teresa rise speed max | 0.8 | 0.5 |
| `mTeresaDropBase` | number | Teresa drop base | 8.0 | 12.0 |
| `mTeresaUpInertia` | number | Teresa up inertia | 0.96 | 0.99 |
| `mTeresaAccelTime` | whole | Teresa accel time | 2 | 10 |
| `mTeresaTrgOnPushTime1` | whole | Teresa trg on push time 1 | 8 | 5 |
| `mTeresaTrgOnPushTime2` | whole | Teresa trg on push time 2 | 2 | same |
| `mTeresaOnePushAccel1` | number | Teresa one push accel 1 | 0.02 | 0.05 |
| `mTeresaOnePushAccel2` | number | Teresa one push accel 2 | 0.005 | 0.01 |
| `mTeresaNoPushDownRatio` | number | Teresa no push down ratio | 0.99 | same |
| `mTeresaRisingBrake` | number | Teresa rising brake | 0.2 | same |
| `mTeresaWindMovingBrake1` | number | Teresa wind moving brake 1 | 0.9 | same |
| `mTeresaWindMovingBrake2` | number | Teresa wind moving brake 2 | 0.8 | same |
| `mTeresaWindMovingBrake3` | number | Teresa wind moving brake 3 | 0.999 | same |
| `mSilhouetteZoffset` | number | Silhouette zoffset | 0.0 | same |
| `mRibbonColor` | whole, 4 of them | Ribbon color | {255, 255, 0, 192} | same |
| `mDamageFogLow` | whole | Damage fog low | 40 | same |
| `mDamageFogHigh` | whole | Damage fog high | 192 | same |
| `mStarPieceFogLevel` | number | Star piece fog level | 0.6 | same |
| `mStarPieceFogTime` | whole | Star piece fog time | 25 | same |
| `mSearchLightAlphaLevel` | whole | Search light alpha level | 85 | same |
| `mSearchLightBlurAlpha` | whole, 3 of them | Search light blur alpha | {40, 20, 10} | same |
| `mSearchLightBlurScale` | number, 3 of them | Search light blur scale | {2.0, 3.0, 4.0} | same |
| `mSearchLightColor` | whole, 4 of them | Search light color | {80, 80, 80, 210} | same |
| `mRasterV` | number | Raster v | 0.0 | same |
| `mRasterSpeed` | whole | Raster speed | 0 | same |
| `mRasterRange` | number | Raster range | 0.0 | same |
| `mRacketHoldTime` | whole | Racket hold time | 1200 | same |
| `mTornadoHoldTime` | whole | Tornado hold time | 1200 | same |
| `mMetalHoldTime` | whole | Metal hold time | 1200 | same |
| `mFireModeTime` | whole | Fire mode time | 1200 | same |
| `mIceModeTime` | whole | Ice mode time | 1200 | same |
| `mHeadAngleFixMargin` | number | Head angle fix margin | 0.01 | same |
| `mFrontAngleFixMargin` | number | Front angle fix margin | 0.01 | same |
| `mCameraHeadRotationTimer` | whole | Camera head rotation timer | 16 | same |
| `mHeadRotateRatio` | number | Head rotate ratio | 0.1 | same |
| `mVibrationAbsorbAngleF` | number | Vibration absorb angle f | 0.1 | same |
| `mVibrationAbsorbAngleH` | number | Vibration absorb angle h | 0.01 | same |
| `mVibrationAbsorbAngleS` | number | Vibration absorb angle s | 0.2 | same |
| `mSlopeAnimBlendRatio` | number | Slope anim blend ratio | 0.15 | same |
| `mRotateHeadVecSpeedByGravityL` | number | Rotate head vec speed by gravity l | 0.5 | same |
| `mRotateHeadVecSpeedByGravityM` | number | Rotate head vec speed by gravity m | 0.1 | same |
| `mRotateHeadVecSpeedByGravityS` | number | Rotate head vec speed by gravity s | 0.01 | same |
| `mLookMaxAngleH` | number | Look max angle h | 0.7 | same |
| `mLookMaxAngleVP` | number | Look max angle vp | 0.5 | same |
| `mLookMaxAngleVM` | number | Look max angle vm | 0.1 | same |
| `mLookShoulderMoveMax` | number | Look shoulder move max | 8.0 | same |
| `mRushInBlendTimer` | whole | Rush in blend timer | 4 | same |
