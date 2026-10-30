// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// A click on the settings page: change it, keep it, apply it.
fn legacy_pvp_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use horizon_client_render::legacy_pvp as pvp;
    if mod_id != pvp::ID {
        return;
    }
    let mut settings = pvp::Settings::read(|key| game.native_setting(pvp::ID, key));
    settings.click(button);
    pvp::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(pvp::ID, key, &value);
    }
}
