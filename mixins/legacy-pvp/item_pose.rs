// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// The 1.8 blocking pose, and (off by default) a swing shown while using.
fn legacy_pvp_item_pose(
    left: bool,
    inverse_height: f32,
    attack: f32,
    pose: &ItemPose,
) -> ::core::ops::ControlFlow<glam::Mat4> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    let swing_using = pvp::on(pvp::SWING_WHILE_USING);
    if pvp::block_arm_is(left) {
        let swing = if swing_using { attack } else { 0.0 };
        let m = pvp::block_pose(left, inverse_height, swing);
        return Break(glam::Mat4::from_cols_array(&m));
    }
    let in_use = matches!(
        pose,
        ItemPose::Using | ItemPose::Eat { .. } | ItemPose::Bow { .. }
    );
    if swing_using && in_use && attack > 0.0 {
        let swing = pvp::swing_about_hand(left, inverse_height, attack);
        let swing = glam::Mat4::from_cols_array(&swing);
        return Break(swing * item_pose(left, inverse_height, 0.0, *pose));
    }
    Continue(())
}
