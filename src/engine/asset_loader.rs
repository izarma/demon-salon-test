use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::Sample;
//use bevy_seedling::sample::Sample;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/imp/Imp-IDLE-Sprite-sheet.png")]
    pub imp_idle: Handle<Image>,
    #[asset(path = "images/imp/Imp-WALK-Sprite-sheet.png")]
    pub imp_walk: Handle<Image>,
    #[asset(path = "images/imp/Imp-JUMP-Sprite-sheet.png")]
    pub imp_jump: Handle<Image>,
    #[asset(path = "images/salon/control-panel.png")]
    pub control_panel: Handle<Image>,
    #[asset(path = "images/salon/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "images/salon/floor.png")]
    pub floor: Handle<Image>,
    #[asset(path = "images/salon/platform.png")]
    pub moving_platform: Handle<Image>,
    #[asset(path = "images/salon/Backet-of-Gold-Apples.png")]
    pub backet_gold_apples: Handle<Image>,
    #[asset(path = "images/salon/joystick.png")]
    pub joystick: Handle<Image>,
    #[asset(path = "images/goat/goat-base.png")]
    pub goat_base: Handle<Image>,
    #[asset(path = "images/goat/goat-body.png")]
    pub goat_body: Handle<Image>,
    #[asset(path = "images/goat/goat-jaw.png")]
    pub goat_jaw: Handle<Image>,
    #[asset(path = "images/goat/goat-ears.png")]
    pub goat_ears: Handle<Image>,
    #[asset(path = "images/goat/hair/hair-top.png")]
    pub hair_top: Handle<Image>,
    #[asset(path = "images/goat/hair/hair1-left.png")]
    pub hair1_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair1-right.png")]
    pub hair1_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair2-left.png")]
    pub hair2_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair2-right.png")]
    pub hair2_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair3-left.png")]
    pub hair3_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair3-right.png")]
    pub hair3_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair4-left.png")]
    pub hair4_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair4-right.png")]
    pub hair4_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair5-left.png")]
    pub hair5_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair5-right.png")]
    pub hair5_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair6-left.png")]
    pub hair6_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair6-right.png")]
    pub hair6_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair7-left.png")]
    pub hair7_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair7-right.png")]
    pub hair7_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair8-left.png")]
    pub hair8_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair8-right.png")]
    pub hair8_right: Handle<Image>,
    #[asset(path = "images/goat/hair/hair9-left.png")]
    pub hair9_left: Handle<Image>,
    #[asset(path = "images/goat/hair/hair9-right.png")]
    pub hair9_right: Handle<Image>,
    #[asset(path = "images/goat/hair/beard.png")]
    pub goat_beard: Handle<Image>,
    #[asset(path = "ui/game_over.png")]
    pub game_over_text: Handle<Image>,
    #[asset(path = "ui/Golden-Apple.png")]
    pub golden_apple: Handle<Image>,
    #[asset(path = "ui/lever-vertical.png")]
    pub lever_vertical: Handle<Image>,
    #[asset(path = "ui/lever-horizontal.png")]
    pub lever_horizontal: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/salon-background.ogg")]
    pub background: Handle<Sample>,
    #[asset(path = "audio/win.ogg")]
    pub win: Handle<Sample>,
    #[asset(path = "audio/lose.ogg")]
    pub lose: Handle<Sample>,
}
