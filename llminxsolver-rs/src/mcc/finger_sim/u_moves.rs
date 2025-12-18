use super::{FingerState, MoveResult, SimulationContext, overwork};

pub fn handle_u(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    if ctx.r_wrist == 0
        && (ctx.r_thumb.time + ctx.params.over_work_mult <= ctx.speed
            || ctx.r_thumb.location != "top")
        && ctx.r_index.location != "m"
    {
        let ow_index = overwork(&ctx.r_index, "home", ctx.speed, ctx.params.over_work_mult);
        let ow_middle = overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        if ow_index <= ow_middle {
            ctx.speed += ow_index + 1.0;
            ctx.r_index = FingerState {
                time: ctx.speed,
                location: "uflick",
            };
        } else {
            ctx.speed += ow_middle + 1.0;
            ctx.r_index = FingerState {
                time: ctx.speed,
                location: "uflick",
            };
            ctx.r_middle = FingerState {
                time: ctx.speed,
                location: "uflick",
            };
        }
    } else if ctx.r_wrist == 1 && ctx.l_wrist == 0 {
        ctx.speed += overwork(&ctx.l_index, "uflick", ctx.speed, ctx.params.over_work_mult);
        if prev_move == "B'" {
            ctx.speed += ctx.params.moveblock + ctx.params.push_mult;
        } else if prev_move.starts_with("B'") {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.push_mult;
        } else {
            ctx.speed += ctx.params.push_mult;
        }
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else if ctx.l_wrist == 0 && !prev_move.starts_with('F') && !prev_move.starts_with('B') {
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
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            ctx.l_wrist as f64,
            ctx.r_wrist as f64,
        ));
    }
    MoveResult::Success
}

pub fn handle_ui(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    if ctx.l_wrist == 0
        && (ctx.l_thumb.time + ctx.params.over_work_mult <= ctx.speed
            || ctx.l_thumb.location != "top")
        && ctx.l_index.location != "m"
    {
        let ow_index = overwork(&ctx.l_index, "home", ctx.speed, ctx.params.over_work_mult);
        let ow_middle = overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        if ow_index <= ow_middle {
            ctx.speed += ow_index + 1.0;
            ctx.l_index = FingerState {
                time: ctx.speed,
                location: "uflick",
            };
        } else {
            ctx.speed += ow_middle + 1.0;
            ctx.l_index = FingerState {
                time: ctx.speed,
                location: "uflick",
            };
            ctx.l_middle = FingerState {
                time: ctx.speed,
                location: "uflick",
            };
        }
    } else if ctx.l_wrist == 1 && ctx.r_wrist == 0 {
        ctx.speed += overwork(&ctx.r_index, "uflick", ctx.speed, ctx.params.over_work_mult);
        if prev_move == "B" {
            ctx.speed += ctx.params.moveblock + ctx.params.push_mult;
        } else if prev_move.starts_with("B'") {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.push_mult;
        } else {
            ctx.speed += ctx.params.push_mult;
        }
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "home",
        };
    } else if ctx.r_wrist == 0 && !prev_move.starts_with('F') && !prev_move.starts_with('B') {
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
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            ctx.l_wrist as f64,
            ctx.r_wrist as f64,
        ));
    }
    MoveResult::Success
}

pub fn handle_u2(ctx: &mut SimulationContext, j: usize) -> MoveResult {
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
    if ctx.r_wrist == 0 && (ctx.l_index.location == "m" || ctx.l_wrist != 0 || r_ow <= l_ow) {
        ctx.speed += overwork(&ctx.r_index, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(
            &ctx.r_ring,
            "u2grip",
            ctx.speed,
            ctx.params.moveblock * ctx.params.over_work_mult,
        );
        ctx.speed += ctx.params.double;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
        ctx.r_middle = FingerState {
            time: ctx.speed,
            location: "uflick",
        };
    } else if ctx.l_wrist == 0 {
        ctx.speed += overwork(&ctx.l_index, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(
            &ctx.l_ring,
            "u2grip",
            ctx.speed,
            ctx.params.moveblock * ctx.params.over_work_mult,
        );
        ctx.speed += ctx.params.double;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "uflick",
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
