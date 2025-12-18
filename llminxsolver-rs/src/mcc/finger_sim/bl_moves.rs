use super::{FingerState, MoveResult, SimulationContext, overwork};

pub fn handle_bl(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    if ctx.r_wrist == 1 {
        ctx.speed += overwork(&ctx.r_index, "home", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
    } else if ctx.l_wrist == -1 {
        ctx.speed += overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('U') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.ring_mult;
        } else {
            ctx.speed += ctx.params.ring_mult;
        }
        ctx.l_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.l_wrist == 1 && !prev_move.starts_with('U') && !prev_move.starts_with('D') {
        if ctx.l_index.location == "uflick" {
            ctx.speed += overwork(
                &ctx.l_index,
                "eido",
                ctx.speed,
                0.75 * ctx.params.over_work_mult,
            );
            ctx.speed = ctx.speed.max(ctx.l_oh_cool + 2.5);
        } else {
            ctx.speed += overwork(
                &ctx.l_index,
                "eido",
                ctx.speed,
                1.25 * ctx.params.over_work_mult,
            );
        }
        ctx.speed += 1.15 * ctx.params.push_mult;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
        ctx.l_oh_cool = ctx.speed;
    } else if ctx.l_wrist == 0 && (ctx.r_wrist == 1 || ctx.r_wrist == -1) {
        ctx.speed += overwork(
            &ctx.l_index,
            "top",
            ctx.speed,
            0.9 * ctx.params.over_work_mult,
        );
        if prev_move.starts_with('U') {
            ctx.speed += 1.45;
        } else {
            ctx.speed += 1.0;
        }
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "leftdb",
        };
    } else if ctx.r_wrist == -1 && !prev_move.starts_with('U') {
        ctx.speed += overwork(&ctx.r_ring, "dflick", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += ctx.params.ring_mult * ctx.params.push_mult;
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            ctx.l_wrist as f64,
            ctx.r_wrist as f64,
        ));
    }
    MoveResult::Success
}

pub fn handle_bli(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    if ctx.l_wrist == 1 {
        ctx.speed += overwork(&ctx.l_index, "home", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
    } else if ctx.r_wrist == -1 {
        ctx.speed += overwork(&ctx.r_ring, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('U') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.ring_mult;
        } else {
            ctx.speed += ctx.params.ring_mult;
        }
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.r_wrist == 1 && !prev_move.starts_with('U') && !prev_move.starts_with('D') {
        if ctx.r_index.location == "uflick" {
            ctx.speed += overwork(
                &ctx.r_index,
                "eido",
                ctx.speed,
                0.75 * ctx.params.over_work_mult,
            );
            ctx.speed = ctx.speed.max(ctx.r_oh_cool + 2.5);
        } else {
            ctx.speed += overwork(
                &ctx.r_index,
                "eido",
                ctx.speed,
                1.25 * ctx.params.over_work_mult,
            );
        }
        ctx.speed += 1.15 * ctx.params.push_mult;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
        ctx.r_oh_cool = ctx.speed;
    } else if ctx.r_wrist == 0 && (ctx.l_wrist == 1 || ctx.l_wrist == -1) {
        ctx.speed += overwork(
            &ctx.r_index,
            "top",
            ctx.speed,
            0.9 * ctx.params.over_work_mult,
        );
        if prev_move.starts_with('U') {
            ctx.speed += 1.45;
        } else {
            ctx.speed += 1.0;
        }
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "rightdb",
        };
    } else if ctx.l_wrist == -1 && !prev_move.starts_with('U') {
        ctx.speed += overwork(&ctx.l_ring, "dflick", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += ctx.params.ring_mult * ctx.params.push_mult;
        ctx.l_ring = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            ctx.l_wrist as f64,
            ctx.r_wrist as f64,
        ));
    }
    MoveResult::Success
}

pub fn handle_bl2(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    let r_ow = overwork(&ctx.r_index, "home", ctx.speed, ctx.params.over_work_mult)
        .max(overwork(
            &ctx.r_middle,
            "home",
            ctx.speed,
            ctx.params.over_work_mult,
        ))
        .max(overwork(
            &ctx.r_ring,
            "u2grip",
            ctx.speed,
            ctx.params.over_work_mult,
        ));
    let l_ow = overwork(&ctx.l_index, "home", ctx.speed, ctx.params.over_work_mult)
        .max(overwork(
            &ctx.l_middle,
            "home",
            ctx.speed,
            ctx.params.over_work_mult,
        ))
        .max(overwork(
            &ctx.l_ring,
            "u2grip",
            ctx.speed,
            ctx.params.over_work_mult,
        ));
    if ctx.r_wrist == 1 && (ctx.l_wrist != 1 || r_ow <= l_ow) {
        ctx.speed += overwork(&ctx.r_index, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_ring, "u2grip", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += ctx.params.double;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
        ctx.r_middle = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
    } else if ctx.l_wrist == 1 {
        ctx.speed += overwork(&ctx.l_index, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_ring, "u2grip", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += ctx.params.double;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
        ctx.l_middle = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
    } else if ctx.l_wrist == -1
        && (ctx.r_wrist != -1
            || overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult).max(overwork(
                &ctx.r_ring,
                "home",
                ctx.speed,
                ctx.params.over_work_mult,
            )) > overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult).max(
                overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult),
            ))
    {
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('U') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.double * ctx.params.ring_mult;
        } else {
            ctx.speed += ctx.params.double * ctx.params.ring_mult;
        }
        ctx.l_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.r_wrist == -1 {
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('U') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.double * ctx.params.ring_mult;
        } else {
            ctx.speed += ctx.params.double * ctx.params.ring_mult;
        }
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            ctx.l_wrist as f64,
            ctx.r_wrist as f64,
        ));
    }
    MoveResult::Success
}
