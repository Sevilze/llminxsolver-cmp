use super::{FingerState, MoveResult, SimulationContext, overwork};

pub fn handle_d(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    let l_ow = overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult).max(overwork(
        &ctx.l_middle,
        "home",
        ctx.speed,
        ctx.params.over_work_mult,
    ));
    let r_ow = overwork(&ctx.r_ring, "dflick", ctx.speed, ctx.params.over_work_mult).max(overwork(
        &ctx.r_middle,
        "home",
        ctx.speed,
        ctx.params.over_work_mult,
    ));
    if ctx.l_wrist == 0 && (ctx.r_wrist != 0 || l_ow <= r_ow) {
        ctx.speed += overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('B') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.ring_mult;
        } else {
            ctx.speed += ctx.params.ring_mult;
        }
        ctx.l_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.r_wrist == 0 && !prev_move.starts_with('B') {
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

pub fn handle_di(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    let r_ow = overwork(&ctx.r_ring, "home", ctx.speed, ctx.params.over_work_mult).max(overwork(
        &ctx.r_middle,
        "home",
        ctx.speed,
        ctx.params.over_work_mult,
    ));
    let l_ow = overwork(&ctx.l_ring, "dflick", ctx.speed, ctx.params.over_work_mult).max(overwork(
        &ctx.l_middle,
        "home",
        ctx.speed,
        ctx.params.over_work_mult,
    ));
    if ctx.r_wrist == 0 && (ctx.l_wrist != 0 || r_ow <= l_ow) {
        ctx.speed += overwork(&ctx.r_ring, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('B') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.ring_mult;
        } else {
            ctx.speed += ctx.params.ring_mult;
        }
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.l_wrist == 0 && !prev_move.starts_with('B') {
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

pub fn handle_d2(ctx: &mut SimulationContext, j: usize, prev_move: &str) -> MoveResult {
    let r_ow = overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult).max(overwork(
        &ctx.r_ring,
        "home",
        ctx.speed,
        ctx.params.over_work_mult,
    ));
    let l_ow = overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult).max(overwork(
        &ctx.l_ring,
        "home",
        ctx.speed,
        ctx.params.over_work_mult,
    ));
    if ctx.r_wrist == 0 && (ctx.l_wrist != 0 || r_ow <= l_ow) {
        ctx.speed += overwork(&ctx.r_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.r_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('B') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.double * ctx.params.ring_mult;
        } else {
            ctx.speed += ctx.params.double * ctx.params.ring_mult;
        }
        ctx.r_ring = FingerState {
            time: ctx.speed,
            location: "dflick",
        };
    } else if ctx.l_wrist == 0 {
        ctx.speed += overwork(&ctx.l_middle, "home", ctx.speed, ctx.params.over_work_mult);
        ctx.speed += overwork(&ctx.l_ring, "home", ctx.speed, ctx.params.over_work_mult);
        if prev_move.starts_with('B') {
            ctx.speed += ctx.params.moveblock * 0.5 + ctx.params.double * ctx.params.ring_mult;
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
