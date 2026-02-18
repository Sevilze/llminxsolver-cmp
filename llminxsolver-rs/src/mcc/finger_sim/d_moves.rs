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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcc::types::MCCParams;

    fn new_ctx() -> SimulationContext<'static> {
        let params = Box::leak(Box::new(MCCParams::default()));
        SimulationContext::new(0, 0, 0.0, params)
    }

    #[test]
    fn test_handle_d_variants() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_d(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = 1;
        ctx2.r_wrist = 0;
        assert!(matches!(handle_d(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 2;
        ctx3.r_wrist = 2;
        assert!(matches!(
            handle_d(&mut ctx3, 0, "B"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_di_variants() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_di(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_di(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 2;
        ctx3.r_wrist = 2;
        assert!(matches!(
            handle_di(&mut ctx3, 0, "B"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_d2_variants() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_d2(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_d2(&mut ctx2, 0, "B"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 2;
        ctx3.r_wrist = 2;
        assert!(matches!(
            handle_d2(&mut ctx3, 0, "R"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_d_family_prev_b_moveblock_paths() {
        let mut ctx_d = new_ctx();
        ctx_d.r_wrist = 1;
        assert!(matches!(handle_d(&mut ctx_d, 0, "B"), MoveResult::Success));

        let mut ctx_di = new_ctx();
        ctx_di.l_wrist = 1;
        assert!(matches!(
            handle_di(&mut ctx_di, 0, "B"),
            MoveResult::Success
        ));

        let mut ctx_d2 = new_ctx();
        ctx_d2.l_wrist = 1;
        assert!(matches!(
            handle_d2(&mut ctx_d2, 0, "B"),
            MoveResult::Success
        ));
    }
}
