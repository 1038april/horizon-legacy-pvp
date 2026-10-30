// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// The mod's row in Options > Mods.
fn legacy_pvp_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client_render::legacy_pvp as pvp;
    mods.push(crate::game::native_hooks::NativeMod {
        id: pvp::ID.into(),
        title: format!("Legacy PvP {}", pvp::VERSION),
        description: pvp::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
