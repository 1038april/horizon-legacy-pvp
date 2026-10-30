// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// `PlayerController.attackEntity` runs `EntityPlayer.attackTargetEntityWithCurrentItem`
// on the client too. Only another player takes that hit there
// (`EntityOtherPlayerMP.attackEntityFrom` is true), so only then does the
// attacker slow down and stop sprinting. `reset_attack_strength` directly
// follows each attack packet.
fn legacy_pvp_sprint_cut(game: &mut crate::game::Game<'_>) {
    use horizon_client_render::legacy_pvp as pvp;
    let Some(id) = pvp::take_noted_attack() else {
        return;
    };
    let is_player = game
        .mobs
        .iter()
        .any(|m| m.id as i32 == id && &*m.kind.id == "minecraft:player");
    let knockback = game
        .held()
        .map_or(0, |s| s.enchantment_level("minecraft:knockback"));
    let sprinting = game.player.as_ref().is_some_and(|p| p.sprinting);
    if !is_player || !pvp::hit_slows_attacker(knockback, sprinting) {
        return;
    }
    if let Some(p) = game.player.as_mut() {
        p.velocity = horizon_core::geometry::Vec3::new(
            p.velocity.x * pvp::ATTACKER_SLOWDOWN,
            p.velocity.y,
            p.velocity.z * pvp::ATTACKER_SLOWDOWN,
        );
    }
    game.stop_sprint_after_extra_knockback();
}
