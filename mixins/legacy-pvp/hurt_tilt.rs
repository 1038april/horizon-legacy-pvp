// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// `EntityLivingBase.handleStatusUpdate(2)` sets `attackedAtYaw` to 0 on the
// client: the view always tilts the same way, whatever hit it.
fn legacy_pvp_hurt_tilt(health: &mut PlayerHealth) -> ::core::ops::ControlFlow<()> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    if !pvp::on(pvp::HURT_TILT) {
        return Continue(());
    }
    health.animate_hurt();
    health.hurt_dir = 0.0;
    Break(())
}
