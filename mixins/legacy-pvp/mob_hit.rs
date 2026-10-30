// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// `mob_hit` with 1.8's picking box: every box grown by the collision border,
// and a sneaking player still 1.8 tall. Reach stays the game's.
fn legacy_pvp_mob_hit(game: &Game<'_>) -> ::core::ops::ControlFlow<Option<(usize, f64)>> {
    use ::core::ops::ControlFlow::{Break, Continue};
    use horizon_client_render::legacy_pvp as pvp;
    use horizon_core::geometry::Vec3;
    if !pvp::on(pvp::PICK_MARGIN) {
        return Continue(());
    }
    let Some((eye, dir)) = game.aim() else {
        return Break(None);
    };
    let reach: f64 = if game.creative { 5.0 } else { 3.0 };
    let border = f64::from(pvp::COLLISION_BORDER);
    let mut best: Option<(usize, f64)> = None;
    for (i, m) in game.mobs.iter().enumerate() {
        if !m.alive() || Some(m.id) == game.riding {
            continue;
        }
        let mut bounds = m.bounds();
        if pvp::on(pvp::SNEAK_SIZE) && m.extra.crouching {
            let (lo, hi) = (bounds.min_position(), bounds.max_position());
            let top = lo.y + f64::from(pvp::PLAYER_HEIGHT);
            bounds = horizon_core::geometry::Aabb::new(lo, Vec3::new(hi.x, top, hi.z));
        }
        let grown = bounds.inflate(Vec3::new(border, border, border));
        if let Some(t) = ray_box(eye, dir, reach, grown) {
            if best.is_none_or(|b| t < b.1) {
                best = Some((i, t));
            }
        }
    }
    Break(best)
}
