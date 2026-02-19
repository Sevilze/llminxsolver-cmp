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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcc::types::MCCParams;

    fn new_ctx() -> SimulationContext<'static> {
        let params = Box::leak(Box::new(MCCParams::default()));
        SimulationContext::new(0, 0, 0.0, params)
    }

    #[test]
    fn test_handle_u_success_and_early_return() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_u(&mut ctx, 0, "R"), MoveResult::Success));

        ctx.r_wrist = 2;
        ctx.l_wrist = 2;
        assert!(matches!(
            handle_u(&mut ctx, 1, "F"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_ui_success_and_early_return() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_ui(&mut ctx, 0, "R"), MoveResult::Success));

        ctx.r_wrist = 2;
        ctx.l_wrist = 2;
        assert!(matches!(
            handle_ui(&mut ctx, 1, "F"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_u2_paths() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_u2(&mut ctx, 0), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_u2(&mut ctx2, 0), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = 2;
        ctx3.l_wrist = 2;
        assert!(matches!(
            handle_u2(&mut ctx3, 0),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_u_additional_paths() {
        let mut ctx = new_ctx();
        ctx.r_wrist = 0;
        ctx.r_index = FingerState {
            time: ctx.speed,
            location: "other",
        };
        ctx.r_middle = FingerState {
            time: -1.0,
            location: "home",
        };
        assert!(matches!(handle_u(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_u(&mut ctx2, 0, "B'"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = 1;
        ctx3.l_wrist = 0;
        assert!(matches!(handle_u(&mut ctx3, 0, "B'2"), MoveResult::Success));

        let mut ctx4 = new_ctx();
        ctx4.l_wrist = 0;
        ctx4.r_wrist = -1;
        ctx4.l_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(handle_u(&mut ctx4, 0, "R"), MoveResult::Success));

        let mut ctx5 = new_ctx();
        ctx5.l_wrist = 0;
        ctx5.r_wrist = -1;
        ctx5.l_index = FingerState {
            time: -1.0,
            location: "home",
        };
        assert!(matches!(handle_u(&mut ctx5, 0, "R"), MoveResult::Success));
    }

    #[test]
    fn test_handle_ui_additional_paths() {
        let mut ctx = new_ctx();
        ctx.l_wrist = 0;
        ctx.l_index = FingerState {
            time: ctx.speed,
            location: "other",
        };
        ctx.l_middle = FingerState {
            time: -1.0,
            location: "home",
        };
        assert!(matches!(handle_ui(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = 1;
        ctx2.r_wrist = 0;
        assert!(matches!(handle_ui(&mut ctx2, 0, "B"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 1;
        ctx3.r_wrist = 0;
        assert!(matches!(
            handle_ui(&mut ctx3, 0, "B'2"),
            MoveResult::Success
        ));

        let mut ctx4 = new_ctx();
        ctx4.r_wrist = 0;
        ctx4.l_wrist = -1;
        ctx4.r_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(handle_ui(&mut ctx4, 0, "R"), MoveResult::Success));

        let mut ctx5 = new_ctx();
        ctx5.r_wrist = 0;
        ctx5.l_wrist = -1;
        ctx5.r_index = FingerState {
            time: -1.0,
            location: "home",
        };
        assert!(matches!(handle_ui(&mut ctx5, 0, "R"), MoveResult::Success));
    }

    #[test]
    fn test_handle_ui_prev_move_non_b_uses_push_mult_else() {
        let mut ctx = new_ctx();
        ctx.l_wrist = 1;
        ctx.r_wrist = 0;
        assert!(matches!(handle_ui(&mut ctx, 0, "R"), MoveResult::Success));
    }
}
