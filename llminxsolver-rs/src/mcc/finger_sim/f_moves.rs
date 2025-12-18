use super::{FingerState, MoveResult, SimulationContext, overwork};

pub fn handle_f(ctx: &mut SimulationContext, j: usize, mv: &str, prev_move: &str) -> MoveResult {
    if ctx.r_wrist == -1 {
        ctx.speed += overwork(&ctx.r_index, "home", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
    } else if ctx.l_wrist == 1 && mv != "f" {
        ctx.speed += overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.ring_mult;
        } else {
            ctx.speed += 1.0;
        }
        ctx.l_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.r_wrist == 1 && !prev_move.starts_with('D') && mv != "f" {
        ctx.speed += overwork(&ctx.r_ring, "dflick", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += ctx.params.ring_mult * ctx.params.push_mult;
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else if ctx.l_wrist == -1
        && ctx.r_wrist == 0
        && overwork(&ctx.r_index, "uflick", ctx.speed, ctx.params.over_work_mult) == 0.0
    {
        ctx.speed += 1.0;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "fflick",
        };
    } else if ctx.l_wrist == -1
        && overwork(&ctx.l_index, "uflick", ctx.speed, ctx.params.over_work_mult) == 0.0
        && !prev_move.starts_with('U')
    {
        ctx.speed += ctx.params.push_mult;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else if ctx.l_wrist == -1 && ctx.grip == -1 {
        ctx.speed += overwork(
            &ctx.l_thumb,
            "top",
            ctx.speed,
            0.9 * ctx.params.over_work_mult,
        );
        ctx.speed += overwork(&ctx.l_index, "top", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += 1.8;
        } else {
            ctx.speed += 1.0;
        }
        ctx.l_wrist += 1;
        ctx.l_thumb = FingerState {
            time: ctx.speed,
            location: "leftu",
        };
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "top",
        };
    } else if ctx.l_wrist == 0 && ctx.grip == -1 {
        ctx.speed += overwork(&ctx.l_thumb, "bottom", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_index, "top", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += 2.05;
        } else {
            ctx.speed += 1.25;
        }
        ctx.l_thumb = FingerState {
            time: ctx.speed,
            location: "top",
        };
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "top",
        };
    } else if ctx.r_wrist == 0 && ctx.l_wrist == 0 && mv == "f" {
        ctx.speed += overwork(&ctx.r_index, "uflick", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "fflick",
        };
    } else if j == 0 && ctx.r_wrist == 0 && ctx.l_wrist == 0 {
        ctx.speed += overwork(&ctx.r_thumb, "top", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.r_thumb = FingerState {
            time: ctx.speed,
            location: "rdown",
        };
        ctx.r_middle = FingerState {
            time: ctx.speed,
            location: "uflick",
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

pub fn handle_fi(ctx: &mut SimulationContext, j: usize, mv: &str, prev_move: &str) -> MoveResult {
    if ctx.l_wrist == -1 {
        ctx.speed += overwork(&ctx.l_index, "home", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
    } else if ctx.r_wrist == 1 && mv != "f" {
        ctx.speed += overwork(&ctx.r_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.ring_mult;
        } else {
            ctx.speed += 1.0;
        }
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.l_wrist == 1 && !prev_move.starts_with('D') && mv != "f" {
        ctx.speed += overwork(&ctx.l_ring, "dflick", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += ctx.params.ring_mult * ctx.params.push_mult;
        ctx.l_ring = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else if ctx.r_wrist == -1
        && ctx.l_wrist == 0
        && overwork(&ctx.l_index, "uflick", ctx.speed, ctx.params.over_work_mult) == 0.0
    {
        ctx.speed += 1.0;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "fflick",
        };
    } else if ctx.r_wrist == -1
        && overwork(&ctx.r_index, "uflick", ctx.speed, ctx.params.over_work_mult) == 0.0
        && !prev_move.starts_with('U')
    {
        ctx.speed += ctx.params.push_mult;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else if ctx.r_wrist == -1 && ctx.grip == 1 {
        ctx.speed += overwork(
            &ctx.r_thumb,
            "top",
            ctx.speed,
            0.9 * ctx.params.over_work_mult,
        );
        ctx.speed += overwork(&ctx.r_index, "top", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += 1.8;
        } else {
            ctx.speed += 1.0;
        }
        ctx.r_wrist += 1;
        ctx.r_thumb = FingerState {
            time: ctx.speed,
            location: "rightu",
        };
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "top",
        };
    } else if ctx.r_wrist == 0 && ctx.grip == 1 {
        ctx.speed += overwork(&ctx.r_thumb, "bottom", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_index, "top", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += 2.05;
        } else {
            ctx.speed += 1.25;
        }
        ctx.r_thumb = FingerState {
            time: ctx.speed,
            location: "top",
        };
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "top",
        };
    } else if ctx.l_wrist == 0 && ctx.r_wrist == 0 && mv == "f'" {
        ctx.speed += overwork(&ctx.l_index, "uflick", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "fflick",
        };
    } else if j == 0 && ctx.r_wrist == 0 && ctx.l_wrist == 0 {
        ctx.speed += overwork(&ctx.l_thumb, "top", ctx.speed, ctx.params.over_work_mult) + 1.0;
        ctx.l_thumb = FingerState {
            time: ctx.speed,
            location: "rdown",
        };
        ctx.l_middle = FingerState {
            time: ctx.speed,
            location: "uflick",
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

pub fn handle_f2(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
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
    if ctx.r_wrist == -1 && (ctx.l_wrist != -1 || r_ow <= l_ow) {
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
    } else if ctx.l_wrist == -1 {
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
    } else if ctx.r_wrist == 1
        && (ctx.l_wrist != 1
            || overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult).max(overwork(
                &ctx.r_ring,
                "home",
                ctx.speed,
                ctx.params.over_work_mult,
            )) <= overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult).max(
                overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult),
            ))
    {
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += ctx.params.double * ctx.params.ring_mult + ctx.params.moveblock * 0.5;
        } else {
            ctx.speed += ctx.params.double * ctx.params.ring_mult;
        }
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.l_wrist == 1 {
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('D') {
            ctx.speed += ctx.params.double * ctx.params.ring_mult + ctx.params.moveblock * 0.5;
        } else {
            ctx.speed += ctx.params.double * ctx.params.ring_mult;
        }
        ctx.l_ring = FingerState {
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
