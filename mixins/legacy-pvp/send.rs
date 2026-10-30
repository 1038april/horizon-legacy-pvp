// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// `Minecraft.clickMouse` swings (and sends it) before `attackEntity`; 26.3 and
// `remote_start_attack` send the attack first. An attack waits here for the
// swing that follows it in the same call.
fn legacy_pvp_send(game: &crate::game::Game<'_>, packet: &S) -> ::core::ops::ControlFlow<()> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    match packet {
        S::Attack(p) => {
            pvp::note_attack(p.id);
            if pvp::hold_attack(p.id) {
                return Break(());
            }
            Continue(())
        }
        S::Punch(_) => {
            let Some(id) = pvp::take_held_attack() else {
                return Continue(());
            };
            game.send(S::Punch(Empty));
            pvp::set_flushing(true);
            game.send(S::Attack(play::Attack { id }));
            pvp::set_flushing(false);
            Break(())
        }
        _ => Continue(()),
    }
}
