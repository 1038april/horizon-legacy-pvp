// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// No attack indicator beside the hotbar.
fn legacy_pvp_hotbar(w: i32, h: i32, i: &HotbarInputs) -> ::core::ops::ControlFlow<Vec<GuiDraw>> {
    use crate::legacy_pvp as pvp;
    use ::core::ops::ControlFlow::{Break, Continue};
    if i.attack.is_none() || !pvp::on(pvp::HIDE_INDICATOR) {
        return Continue(());
    }
    let without = HotbarInputs {
        selected: i.selected,
        right_handed: i.right_handed,
        offhand: i.offhand,
        slots: i.slots,
        offhand_pop: i.offhand_pop,
        attack: None,
    };
    Break(hotbar(w, h, &without))
}
