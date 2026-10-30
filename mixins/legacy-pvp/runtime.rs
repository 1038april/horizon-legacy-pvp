// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Legacy PvP: the client behaving as the 1.8.9 client does on a 1.8 server.
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to `horizon-client-render`, where every patched function can see it.
//
// Every 1.8 value below was read from the official 1.8.9 client jar (SHA-1
// 3870888a6c3d349d3771a3e9d16c9bf5e076b908) with `javap -c`; the obfuscated
// `class.method` is named beside it. No game code is in this repository.
//
// Features are bits in one atomic: hooks run on the render path and, in forced
// single player, on the server thread, and read it with no lock.
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, AtomicU16, Ordering};

pub const ID: &str = "legacy_pvp";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Plays as the 1.8.9 client does on 1.8 servers: no attack cooldown visuals, sword blocking, 1.8 hit boxes.";

// `pk.ao` (`Entity.getCollisionBorderSize`), read by `bfk.a` (`getMouseOver`).
pub const COLLISION_BORDER: f32 = 0.1;
// `wn.aS` (`EntityPlayer.getEyeHeight`): 1.62, minus 0.08 while sneaking.
pub const EYE_HEIGHT: f32 = 1.62;
pub const SNEAK_EYE_OFFSET: f32 = 0.08;
// `wn.I`: one size, whatever the player does (no crouching box exists).
pub const PLAYER_WIDTH: f32 = 0.6;
pub const PLAYER_HEIGHT: f32 = 1.8;
// `wn.f` (`attackTargetEntityWithCurrentItem`): the attacker after a knockback hit.
pub const ATTACKER_SLOWDOWN: f64 = 0.6;

pub fn sneak_eye_height() -> f32 {
    EYE_HEIGHT - SNEAK_EYE_OFFSET
}

/// `wn.f`'s condition: knockback level, plus one when sprinting, above zero.
/// It has no attack-strength check, because 1.8 has no cooldown.
pub fn hit_slows_attacker(knockback: u32, sprinting: bool) -> bool {
    knockback + u32::from(sprinting) > 0
}

/// `ItemSword`, the only item with `EnumAction.BLOCK`.
pub fn is_sword(id: &str) -> bool {
    id.strip_prefix("minecraft:")
        .unwrap_or(id)
        .ends_with("_sword")
}

// The features, as bits of `Settings::features` and of what is on now.
pub const HIDE_INDICATOR: u16 = 1 << 0;
pub const NO_LOWERING: u16 = 1 << 1;
pub const SWING_FIRST: u16 = 1 << 2;
pub const SPRINT_CUT: u16 = 1 << 3;
pub const SWORD_BLOCK: u16 = 1 << 4;
pub const PICK_MARGIN: u16 = 1 << 5;
pub const SNEAK_SIZE: u16 = 1 << 6;
pub const INSTANT_EYE: u16 = 1 << 7;
pub const HURT_TILT: u16 = 1 << 8;
pub const SWING_WHILE_USING: u16 = 1 << 9;

// Key, label and default. Swing while using is 1.7's look, not 1.8.9's
// (`bfn.a(float)` passes swing 0 to every use pose), so it starts off.
pub const FEATURES: [(u16, &str, &str, bool); 10] = [
    (
        HIDE_INDICATOR,
        "hide_indicator",
        "Hide attack indicator",
        true,
    ),
    (NO_LOWERING, "no_lowering", "No weapon lowering", true),
    (
        SWING_FIRST,
        "swing_first",
        "Swing before attack packet",
        true,
    ),
    (
        SPRINT_CUT,
        "sprint_cut",
        "Slow down after sprint hits",
        true,
    ),
    (SWORD_BLOCK, "sword_block", "Sword blocking", true),
    (PICK_MARGIN, "pick_margin", "1.8 picking margin", true),
    (SNEAK_SIZE, "sneak_size", "1.8 sneak eye and box", true),
    (INSTANT_EYE, "instant_eye", "Instant eye height", true),
    (HURT_TILT, "hurt_tilt", "1.8 hurt tilt direction", true),
    (
        SWING_WHILE_USING,
        "swing_while_using",
        "Swing while using (1.7 look)",
        false,
    ),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// On for a server this window doesn't run, off in its own world.
    Auto,
    /// On everywhere. In single player the server thread shares the switches.
    Always,
    Never,
}

impl Mode {
    fn name(self) -> &'static str {
        match self {
            Mode::Auto => "auto",
            Mode::Always => "always",
            Mode::Never => "never",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Mode::Auto => "Auto (servers only)",
            Mode::Always => "Always on",
            Mode::Never => "Off",
        }
    }
    fn next(self) -> Self {
        match self {
            Mode::Auto => Mode::Always,
            Mode::Always => Mode::Never,
            Mode::Never => Mode::Auto,
        }
    }
    /// Whether the features apply. The client can't see a server's version
    /// through ViaVersion, only whether this window runs the world.
    pub fn applies(self, own_world: bool) -> bool {
        match self {
            Mode::Auto => !own_world,
            Mode::Always => true,
            Mode::Never => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub mode: Mode,
    pub features: u16,
}

impl Default for Settings {
    fn default() -> Self {
        let features = FEATURES
            .iter()
            .filter(|f| f.3)
            .fold(0, |bits, f| bits | f.0);
        Settings {
            mode: Mode::Auto,
            features,
        }
    }
}

impl Settings {
    /// From the stored files; a missing or unknown value keeps its default.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let mode = match get("mode").as_deref() {
            Some("always") => Mode::Always,
            Some("never") => Mode::Never,
            _ => Mode::Auto,
        };
        let mut settings = Settings {
            mode,
            ..Settings::default()
        };
        for (bit, key, _, _) in FEATURES {
            match get(key).as_deref() {
                Some("true") => settings.features |= bit,
                Some("false") => settings.features &= !bit,
                _ => {}
            }
        }
        settings
    }
    /// The page's buttons: the mode, then the features in order.
    pub fn labels(&self) -> Vec<String> {
        let mut labels = vec![format!("Mode: {}", self.mode.label())];
        for (bit, _, label, _) in FEATURES {
            let state = if self.features & bit != 0 {
                "ON"
            } else {
                "OFF"
            };
            labels.push(format!("{label}: {state}"));
        }
        labels
    }
    pub fn click(&mut self, button: usize) {
        if button == 0 {
            self.mode = self.mode.next();
        } else if let Some(feature) = FEATURES.get(button - 1) {
            self.features ^= feature.0;
        }
    }
    pub fn stored(&self) -> Vec<(&'static str, String)> {
        let mut stored = vec![("mode", self.mode.name().to_owned())];
        for (bit, key, _, _) in FEATURES {
            stored.push((key, (self.features & bit != 0).to_string()));
        }
        stored
    }
    /// The features on now.
    pub fn effective(&self, enabled: bool, own_world: bool) -> u16 {
        if enabled && self.mode.applies(own_world) {
            self.features
        } else {
            0
        }
    }
}

static LOADED: AtomicBool = AtomicBool::new(false);
static MODE: AtomicU8 = AtomicU8::new(0);
static FEATURE_BITS: AtomicU16 = AtomicU16::new(0);
static ACTIVE: AtomicU16 = AtomicU16::new(0);

pub fn settings_loaded() -> bool {
    LOADED.load(Ordering::Relaxed)
}

pub fn set_settings(settings: Settings) {
    let mode = match settings.mode {
        Mode::Auto => 0,
        Mode::Always => 1,
        Mode::Never => 2,
    };
    MODE.store(mode, Ordering::Relaxed);
    FEATURE_BITS.store(settings.features, Ordering::Relaxed);
    LOADED.store(true, Ordering::Relaxed);
}

/// Once a frame, from the window's game.
pub fn publish(enabled: bool, own_world: bool) {
    let mode = match MODE.load(Ordering::Relaxed) {
        1 => Mode::Always,
        2 => Mode::Never,
        _ => Mode::Auto,
    };
    let settings = Settings {
        mode,
        features: FEATURE_BITS.load(Ordering::Relaxed),
    };
    ACTIVE.store(settings.effective(enabled, own_world), Ordering::Relaxed);
}

pub fn on(feature: u16) -> bool {
    ACTIVE.load(Ordering::Relaxed) & feature != 0
}

// The attack that waits for its swing, and the one the sprint cut hasn't seen.
const NONE: i32 = i32::MIN;
static HELD_ATTACK: AtomicI32 = AtomicI32::new(NONE);
static NOTED_ATTACK: AtomicI32 = AtomicI32::new(NONE);
static FLUSHING: AtomicBool = AtomicBool::new(false);

pub fn set_flushing(flushing: bool) {
    FLUSHING.store(flushing, Ordering::Relaxed);
}

/// Whether this attack packet waits for its swing.
pub fn hold_attack(id: i32) -> bool {
    if FLUSHING.load(Ordering::Relaxed) || !on(SWING_FIRST) {
        return false;
    }
    HELD_ATTACK.store(id, Ordering::Relaxed);
    true
}

pub fn take_held_attack() -> Option<i32> {
    match HELD_ATTACK.swap(NONE, Ordering::Relaxed) {
        NONE => None,
        id => Some(id),
    }
}

/// An attack went out or was held; the resend of a held one doesn't count.
pub fn note_attack(id: i32) {
    if !FLUSHING.load(Ordering::Relaxed) && on(SPRINT_CUT) {
        NOTED_ATTACK.store(id, Ordering::Relaxed);
    }
}

pub fn take_noted_attack() -> Option<i32> {
    match NOTED_ATTACK.swap(NONE, Ordering::Relaxed) {
        NONE => None,
        id => Some(id),
    }
}

static SKIP_ITEM_USED: AtomicBool = AtomicBool::new(false);

pub fn skip_next_item_used() {
    SKIP_ITEM_USED.store(true, Ordering::Relaxed);
}

pub fn take_skip_item_used() -> bool {
    SKIP_ITEM_USED.swap(false, Ordering::Relaxed)
}

// Which arm blocks: 0 none, 1 right, 2 left.
static BLOCK_ARM: AtomicU8 = AtomicU8::new(0);

pub fn set_block_arm(left: Option<bool>) {
    let arm = match left {
        None => 0,
        Some(false) => 1,
        Some(true) => 2,
    };
    BLOCK_ARM.store(arm, Ordering::Relaxed);
}

pub fn block_arm_is(left: bool) -> bool {
    BLOCK_ARM.load(Ordering::Relaxed) == if left { 2 } else { 1 }
}

// 4x4 matrices, column-major like `glam::Mat4::from_cols_array`. The module
// can't name glam: the mod's own tests build with no dependencies.
pub type Mat = [f32; 16];
const IDENTITY: Mat = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

fn mul(a: &Mat, b: &Mat) -> Mat {
    let mut out = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = (0..4).map(|k| a[k * 4 + row] * b[col * 4 + k]).sum();
        }
    }
    out
}
fn translate(x: f32, y: f32, z: f32) -> Mat {
    let mut m = IDENTITY;
    m[12] = x;
    m[13] = y;
    m[14] = z;
    m
}
fn scale(s: f32) -> Mat {
    let mut m = IDENTITY;
    m[0] = s;
    m[5] = s;
    m[10] = s;
    m
}
fn rot_x(degrees: f32) -> Mat {
    let (s, c) = degrees.to_radians().sin_cos();
    let mut m = IDENTITY;
    m[5] = c;
    m[6] = s;
    m[9] = -s;
    m[10] = c;
    m
}
fn rot_y(degrees: f32) -> Mat {
    let (s, c) = degrees.to_radians().sin_cos();
    let mut m = IDENTITY;
    m[0] = c;
    m[2] = -s;
    m[8] = s;
    m[10] = c;
    m
}
fn rot_z(degrees: f32) -> Mat {
    let (s, c) = degrees.to_radians().sin_cos();
    let mut m = IDENTITY;
    m[0] = c;
    m[1] = s;
    m[4] = -s;
    m[5] = c;
    m
}
fn chain(parts: &[Mat]) -> Mat {
    parts.iter().fold(IDENTITY, |all, part| mul(&all, part))
}
// The same transform seen in a mirror: the left arm's.
fn mirrored(m: &Mat) -> Mat {
    let mut flip = IDENTITY;
    flip[0] = -1.0;
    mul(&mul(&flip, m), &flip)
}
fn inverse_display(translation: [f32; 3], rotation: [f32; 3], size: f32) -> Mat {
    let [x, y, z] = translation;
    let [rx, ry, rz] = rotation;
    chain(&[
        scale(1.0 / size),
        rot_z(-rz),
        rot_y(-ry),
        rot_x(-rx),
        translate(-x / 16.0, -y / 16.0, -z / 16.0),
    ])
}

// `bfn.d` (`doBlockTransformations`).
fn block_transform() -> Mat {
    chain(&[
        translate(-0.5, 0.2, 0.0),
        rot_y(30.0),
        rot_x(-80.0),
        rot_y(60.0),
    ])
}

// `bfn.b(float, float)` (`transformFirstPersonItem`), right arm.
fn hand_transform(inverse_height: f32, swing: f32) -> Mat {
    let pi = std::f32::consts::PI;
    let (square, root) = ((swing * swing * pi).sin(), (swing.sqrt() * pi).sin());
    chain(&[
        translate(0.56, -0.52, -0.71999997),
        translate(0.0, inverse_height * -0.6, 0.0),
        rot_y(45.0),
        rot_y(square * -20.0),
        rot_z(root * -20.0),
        rot_x(root * -80.0),
        scale(0.4),
    ])
}

/// The hand's pose for a blocking sword, as `item_pose` returns it: before
/// the 26.3 item model's own display transform. So it holds 1.8's whole chain,
/// `hand * block * 1.8 display`, and takes back 26.3's display. Display values
/// are `models/item/diamond_sword.json` (1.8.9) and `item/handheld.json` (26.3).
pub fn block_pose(left: bool, inverse_height: f32, swing: f32) -> Mat {
    let display_18 = chain(&[
        translate(0.0, 4.0 / 16.0, 2.0 / 16.0),
        rot_y(-135.0),
        rot_z(25.0),
        scale(1.7),
    ]);
    let undo_26 = inverse_display([1.13, 3.2, 1.13], [0.0, -90.0, 25.0], 0.68);
    let right = chain(&[
        hand_transform(inverse_height, swing),
        block_transform(),
        display_18,
        undo_26,
    ]);
    if left { mirrored(&right) } else { right }
}

/// A swing about the hand, for a use pose that has none: `applyItemArmAttackTransform`
/// at the place `item_pose` starts from.
pub fn swing_about_hand(left: bool, inverse_height: f32, swing: f32) -> Mat {
    let pi = std::f32::consts::PI;
    let (square, root) = ((swing * swing * pi).sin(), (swing.sqrt() * pi).sin());
    let hand = translate(0.56, -0.52 - inverse_height * 0.6, -0.72);
    let unhand = translate(-0.56, 0.52 + inverse_height * 0.6, 0.72);
    let right = chain(&[
        hand,
        translate(
            -0.4 * root,
            0.2 * (swing.sqrt() * pi * 2.0).sin(),
            -0.2 * (swing * pi).sin(),
        ),
        rot_y(45.0 - square * 20.0),
        rot_z(root * -20.0),
        rot_x(root * -80.0),
        rot_y(-45.0),
        unhand,
    ]);
    if left { mirrored(&right) } else { right }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // The switches are process-wide; the tests that touch them take turns.
    static SWITCHES: Mutex<()> = Mutex::new(());

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn values_are_the_1_8_9_clients() {
        assert_eq!(COLLISION_BORDER, 0.1);
        assert_eq!(PLAYER_WIDTH, 0.6);
        assert_eq!(PLAYER_HEIGHT, 1.8);
        assert_eq!(EYE_HEIGHT, 1.62);
        assert!(close(sneak_eye_height(), 1.54));
        assert_eq!(ATTACKER_SLOWDOWN, 0.6);
    }

    #[test]
    fn auto_is_on_only_for_a_world_this_window_doesnt_run() {
        assert!(Mode::Auto.applies(false));
        assert!(!Mode::Auto.applies(true));
        assert!(Mode::Always.applies(true));
        assert!(Mode::Always.applies(false));
        assert!(!Mode::Never.applies(false));
        assert!(!Mode::Never.applies(true));
    }

    #[test]
    fn turning_the_mod_off_turns_everything_off() {
        let settings = Settings::default();
        assert_eq!(settings.effective(false, false), 0);
        assert_eq!(settings.effective(true, false), settings.features);
        assert_eq!(settings.effective(true, true), 0);
    }

    #[test]
    fn every_feature_starts_on_but_the_swing_while_using() {
        let settings = Settings::default();
        for (bit, key, _, default) in FEATURES {
            assert_eq!(settings.features & bit != 0, default, "{key}");
        }
        assert_eq!(settings.features & SWING_WHILE_USING, 0);
    }

    #[test]
    fn each_button_toggles_only_its_own_feature() {
        for (i, (bit, key, _, _)) in FEATURES.into_iter().enumerate() {
            let mut settings = Settings::default();
            let before = settings.features;
            settings.click(i + 1);
            assert_eq!(settings.features, before ^ bit, "{key}");
            settings.click(i + 1);
            assert_eq!(settings.features, before, "{key}");
        }
    }

    #[test]
    fn the_first_button_cycles_the_mode() {
        let mut settings = Settings::default();
        assert_eq!(settings.mode, Mode::Auto);
        settings.click(0);
        assert_eq!(settings.mode, Mode::Always);
        settings.click(0);
        assert_eq!(settings.mode, Mode::Never);
        settings.click(0);
        assert_eq!(settings.mode, Mode::Auto);
    }

    #[test]
    fn settings_survive_being_stored() {
        let mut settings = Settings::default();
        settings.click(0);
        settings.click(1);
        settings.click(10);
        let stored = settings.stored();
        let read = Settings::read(|key| {
            stored
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.clone())
        });
        assert_eq!(read, settings);
    }

    #[test]
    fn nonsense_in_a_file_keeps_the_default() {
        let read = Settings::read(|_| Some("banana".to_owned()));
        assert_eq!(read, Settings::default());
    }

    #[test]
    fn the_page_labels_say_the_state() {
        let labels = Settings::default().labels();
        assert_eq!(labels.len(), FEATURES.len() + 1);
        assert_eq!(labels[0], "Mode: Auto (servers only)");
        assert_eq!(labels[1], "Hide attack indicator: ON");
        assert_eq!(labels[10], "Swing while using (1.7 look): OFF");
    }

    #[test]
    fn only_swords_block() {
        for id in [
            "minecraft:diamond_sword",
            "minecraft:golden_sword",
            "iron_sword",
        ] {
            assert!(is_sword(id), "{id}");
        }
        for id in [
            "minecraft:shield",
            "minecraft:diamond_axe",
            "minecraft:swordfish",
        ] {
            assert!(!is_sword(id), "{id}");
        }
    }

    #[test]
    fn the_cut_needs_sprint_or_knockback_and_never_the_cooldown() {
        assert!(!hit_slows_attacker(0, false));
        assert!(hit_slows_attacker(0, true));
        assert!(hit_slows_attacker(1, false));
        assert!(hit_slows_attacker(2, true));
    }

    #[test]
    fn an_attack_waits_for_its_swing_only_while_the_feature_is_on() {
        let _guard = SWITCHES.lock().unwrap();
        set_flushing(false);
        take_held_attack();
        take_noted_attack();

        set_settings(Settings::default());
        publish(true, false);
        assert!(hold_attack(7));
        note_attack(7);
        assert_eq!(take_noted_attack(), Some(7));
        assert_eq!(take_held_attack(), Some(7));
        assert_eq!(take_held_attack(), None);

        // The resend of the held attack neither waits nor slows again.
        set_flushing(true);
        assert!(!hold_attack(7));
        note_attack(7);
        assert_eq!(take_noted_attack(), None);
        set_flushing(false);

        // On the player's own world nothing is held back.
        publish(true, true);
        assert!(!hold_attack(7));
        note_attack(7);
        assert_eq!(take_noted_attack(), None);
        assert_eq!(take_held_attack(), None);
    }

    #[test]
    fn one_switch_stops_one_feature() {
        let _guard = SWITCHES.lock().unwrap();
        let mut settings = Settings::default();
        settings.features &= !SWORD_BLOCK;
        set_settings(settings);
        publish(true, false);
        assert!(!on(SWORD_BLOCK));
        assert!(on(PICK_MARGIN));
        publish(false, false);
        assert!(!on(PICK_MARGIN));
    }

    #[test]
    fn the_inverse_display_undoes_the_display() {
        let display = chain(&[
            translate(1.13 / 16.0, 3.2 / 16.0, 1.13 / 16.0),
            rot_y(-90.0),
            rot_z(25.0),
            scale(0.68),
        ]);
        let undo = inverse_display([1.13, 3.2, 1.13], [0.0, -90.0, 25.0], 0.68);
        let both = mul(&display, &undo);
        for (got, want) in both.iter().zip(IDENTITY) {
            assert!(close(*got, want), "{both:?}");
        }
    }

    #[test]
    fn the_block_pose_lands_the_item_where_1_8_9_does() {
        // With the 26.3 display after it, the chain is 1.8.9's exactly, so the
        // item's origin and its rotation can be checked against it directly.
        let display_26 = chain(&[
            translate(1.13 / 16.0, 3.2 / 16.0, 1.13 / 16.0),
            rot_y(-90.0),
            rot_z(25.0),
            scale(0.68),
        ]);
        let whole = mul(&block_pose(false, 0.0, 0.0), &display_26);
        let expected = chain(&[
            translate(0.56, -0.52, -0.71999997),
            rot_y(45.0),
            scale(0.4),
            translate(-0.5, 0.2, 0.0),
            rot_y(30.0),
            rot_x(-80.0),
            rot_y(60.0),
            translate(0.0, 0.25, 0.125),
            rot_y(-135.0),
            rot_z(25.0),
            scale(1.7),
        ]);
        for (got, want) in whole.iter().zip(expected) {
            assert!(close(*got, want), "{whole:?}");
        }
    }

    #[test]
    fn the_left_arm_mirrors_the_right() {
        let (right, left) = (block_pose(false, 0.3, 0.0), block_pose(true, 0.3, 0.0));
        assert!(close(left[12], -right[12]));
        assert!(close(left[13], right[13]));
        assert!(close(left[14], right[14]));
    }

    #[test]
    fn a_finished_swing_moves_nothing() {
        for (got, want) in swing_about_hand(false, 0.2, 0.0).iter().zip(IDENTITY) {
            assert!(close(*got, want));
        }
    }

    #[test]
    fn equipping_lowers_the_block_pose() {
        let (up, down) = (block_pose(false, 0.0, 0.0), block_pose(false, 1.0, 0.0));
        assert!(close(down[13], up[13] - 0.6));
    }
}
