// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Each frame of the window: which features apply here, and the state that
// the tick rewrites or the renderer reads.
fn legacy_pvp_frame(game: &mut crate::game::Game<'_>) {
    use horizon_client_render::legacy_pvp as pvp;
    if !game.native_window() {
        return;
    }
    if !pvp::settings_loaded() {
        let settings = pvp::Settings::read(|key| game.native_setting(pvp::ID, key));
        pvp::set_settings(settings);
    }
    pvp::publish(game.native_on(pvp::ID), game.has_singleplayer_server());

    // An attack held for its swing that no swing followed: never lose it.
    if let Some(id) = pvp::take_held_attack() {
        pvp::set_flushing(true);
        use horizon_protocol::packets::play;
        game.send(play::Serverbound::Attack(play::Attack { id }));
        pvp::set_flushing(false);
    }

    // First person: the arm blocking with a sword, for `item_pose`.
    let blocking = pvp::on(pvp::SWORD_BLOCK)
        && game
            .using
            .and_then(|(slot, _)| game.stack_in(slot))
            .is_some_and(|stack| pvp::is_sword(&stack.item));
    let main_left = game.settings.main_hand_left;
    let off_hand = game.using.is_some_and(|(slot, _)| slot == crate::game::shield::OFFHAND_SLOT);
    pvp::set_block_arm(blocking.then_some(main_left != off_hand));
    // The model F5 shows, which `tick_player_model` rewrites each tick.
    if let Some(m) = game.player_model.as_mut().filter(|_| blocking) {
        m.extra.blocking = true;
    }

    // Third person: another player's sword. `tick_remote_entities` only knows
    // `blocks_attacks`, from the item table, and rewrites this every tick.
    if pvp::on(pvp::SWORD_BLOCK) {
        for m in &mut game.mobs {
            let used = if m.extra.using_off_hand {
                &m.extra.off_hand
            } else {
                &m.extra.main_hand
            };
            if m.extra.using_item > 0 && used.as_deref().is_some_and(pvp::is_sword) {
                m.extra.blocking = true;
            }
        }
    }

    // `EntityRenderer.orientCamera` reads the eye height with no smoothing.
    if pvp::on(pvp::INSTANT_EYE) && game.sleeping.is_none() {
        if let Some(eye) = game.player.as_ref().map(|p| p.dimensions.eye_height) {
            game.eye = eye;
            game.previous_eye = eye;
        }
    }
}
