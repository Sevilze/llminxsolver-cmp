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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcc::types::MCCParams;

    fn new_ctx() -> SimulationContext<'static> {
        let params = Box::leak(Box::new(MCCParams::default()));
        SimulationContext::new(0, 0, 0.0, params)
    }

    #[test]
    fn test_handle_bl_and_bli_paths() {
        let mut ctx = new_ctx();
        ctx.r_wrist = 1;
        assert!(matches!(handle_bl(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = 1;
        assert!(matches!(handle_bli(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 2;
        ctx3.r_wrist = 2;
        assert!(matches!(
            handle_bl(&mut ctx3, 0, "U"),
            MoveResult::EarlyReturn(_)
        ));
        assert!(matches!(
            handle_bli(&mut ctx3, 0, "U"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_bl2_paths() {
        let mut ctx = new_ctx();
        ctx.r_wrist = 1;
        assert!(matches!(handle_bl2(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = 1;
        assert!(matches!(handle_bl2(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 2;
        ctx3.r_wrist = 2;
        assert!(matches!(
            handle_bl2(&mut ctx3, 0, "R"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_bl_additional_paths() {
        let mut ctx = new_ctx();
        ctx.l_wrist = -1;
        assert!(matches!(handle_bl(&mut ctx, 0, "U"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = 1;
        ctx2.l_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(handle_bl(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 0;
        ctx3.r_wrist = -1;
        assert!(matches!(handle_bl(&mut ctx3, 0, "U"), MoveResult::Success));

        let mut ctx4 = new_ctx();
        ctx4.r_wrist = -1;
        assert!(matches!(handle_bl(&mut ctx4, 0, "R"), MoveResult::Success));
    }

    #[test]
    fn test_handle_bli_additional_paths() {
        let mut ctx = new_ctx();
        ctx.r_wrist = -1;
        assert!(matches!(handle_bli(&mut ctx, 0, "U"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        ctx2.r_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(handle_bli(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = 0;
        ctx3.l_wrist = -1;
        assert!(matches!(handle_bli(&mut ctx3, 0, "U"), MoveResult::Success));

        let mut ctx4 = new_ctx();
        ctx4.l_wrist = -1;
        assert!(matches!(handle_bli(&mut ctx4, 0, "R"), MoveResult::Success));
    }

    #[test]
    fn test_handle_bl2_remaining_success_paths() {
        let mut ctx = new_ctx();
        ctx.l_wrist = -1;
        ctx.r_wrist = 0;
        assert!(matches!(handle_bl2(&mut ctx, 0, "U"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = -1;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_bl2(&mut ctx2, 0, "U"), MoveResult::Success));
    }

    #[test]
    fn test_handle_bl_branches_non_u_and_non_uflick_variants() {
        let mut ctx = new_ctx();
        ctx.r_wrist = -1;
        ctx.l_wrist = 2;
        assert!(matches!(handle_bl(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        ctx2.r_index = FingerState {
            time: -1.0,
            location: "home",
        };
        assert!(matches!(handle_bli(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = -1;
        ctx3.r_wrist = 2;
        assert!(matches!(handle_bli(&mut ctx3, 0, "R"), MoveResult::Success));

        let mut ctx4 = new_ctx();
        ctx4.l_wrist = -1;
        ctx4.r_wrist = -1;
        ctx4.r_middle.location = "x";
        ctx4.r_ring.location = "x";
        ctx4.l_middle.location = "home";
        ctx4.l_ring.location = "home";
        assert!(matches!(handle_bl2(&mut ctx4, 0, "R"), MoveResult::Success));
    }
}
