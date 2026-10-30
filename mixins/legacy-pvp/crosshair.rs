// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// No attack indicator under the crosshair.
fn legacy_pvp_crosshair(
    w: i32,
    h: i32,
    attack: Option<f32>,
) -> ::core::ops::ControlFlow<Vec<GuiDraw>> {
    use crate::legacy_pvp as pvp;
    use ::core::ops::ControlFlow::{Break, Continue};
    if attack.is_some() && pvp::on(pvp::HIDE_INDICATOR) {
        return Break(crosshair(w, h, None, false));
    }
    Continue(())
}
