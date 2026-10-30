// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// 1.8 keeps the standing box while sneaking and lowers only the eye.
fn legacy_pvp_dimensions(pose: Pose, scale: f32) -> ::core::ops::ControlFlow<Dimensions> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    if !pvp::on(pvp::SNEAK_SIZE) || !matches!(pose, Pose::Crouching) {
        return Continue(());
    }
    Break(Dimensions {
        width: pvp::PLAYER_WIDTH * scale,
        height: pvp::PLAYER_HEIGHT * scale,
        eye_height: pvp::sneak_eye_height() * scale,
    })
}
