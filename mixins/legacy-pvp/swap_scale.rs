// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// 1.8 has no cooldown: the hand only dips when the item changes.
fn legacy_pvp_swap_scale() -> ::core::ops::ControlFlow<f32> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    if pvp::on(pvp::NO_LOWERING) {
        return Break(1.0);
    }
    Continue(())
}
