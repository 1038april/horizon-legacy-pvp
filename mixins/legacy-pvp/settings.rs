// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// The mode and one button per feature.
fn legacy_pvp_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client_render::legacy_pvp as pvp;
    if mod_id != pvp::ID {
        return;
    }
    let game = &*game;
    let settings = pvp::Settings::read(|key| game.native_setting(pvp::ID, key));
    buttons.extend(settings.labels());
}
