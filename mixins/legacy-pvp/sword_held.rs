// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// A blocking sword stays in use until the button lets go.
fn legacy_pvp_sword_held(stack: &ItemStack) -> ::core::ops::ControlFlow<bool> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    if pvp::on(pvp::SWORD_BLOCK) && pvp::is_sword(&stack.item) {
        return Break(true);
    }
    Continue(())
}
