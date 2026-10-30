// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// `Minecraft.rightClickMouse` only re-equips when the stack changed; blocking
// with a sword leaves it, so the hand doesn't drop.
fn legacy_pvp_item_used() -> ::core::ops::ControlFlow<()> {
    use ::core::ops::ControlFlow::{Break, Continue};
    if horizon_client_render::legacy_pvp::take_skip_item_used() {
        return Break(());
    }
    Continue(())
}
