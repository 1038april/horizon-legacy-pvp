// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// A sword is used like a shield: held until released, the use packet, the
// release packet and the 0.2 slowdown all come from the shield's path. A real
// 1.8 client does the same for `EnumAction.BLOCK`. The server decides damage.
fn legacy_pvp_raise_sword(
    game: &mut crate::game::Game<'_>,
    slot: usize,
) -> ::core::ops::ControlFlow<bool> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    let sword = game.stack_in(slot).is_some_and(|s| pvp::is_sword(&s.item));
    if !pvp::on(pvp::SWORD_BLOCK) || !sword {
        return Continue(());
    }
    if game.using.is_none() {
        game.using = Some((slot, 0));
    }
    Break(true)
}
