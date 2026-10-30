// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// A sword's use ends the click at its hand, with no off-hand turn after it.
fn legacy_pvp_sword_has_use(
    info: &horizon_client::items::ItemInfo,
) -> ::core::ops::ControlFlow<bool> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    if !pvp::on(pvp::SWORD_BLOCK) || !pvp::is_sword(&info.id) {
        return Continue(());
    }
    pvp::skip_next_item_used();
    Break(true)
}
