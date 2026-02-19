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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcc::types::MCCParams;

    fn new_ctx() -> SimulationContext<'static> {
        let params = Box::leak(Box::new(MCCParams::default()));
        SimulationContext::new(0, 0, 0.0, params)
    }

    #[test]
    fn test_handle_f_basic_and_early_return() {
        let mut ctx = new_ctx();
        let ok = handle_f(&mut ctx, 0, "f", " ");
        assert!(matches!(ok, MoveResult::Success));

        ctx.l_wrist = 2;
        ctx.r_wrist = 2;
        let bad = handle_f(&mut ctx, 1, "F", "R");
        assert!(matches!(bad, MoveResult::EarlyReturn(_)));
    }

    #[test]
    fn test_handle_fi_basic_and_early_return() {
        let mut ctx = new_ctx();
        let ok = handle_fi(&mut ctx, 0, "f'", " ");
        assert!(matches!(ok, MoveResult::Success));

        ctx.l_wrist = 2;
        ctx.r_wrist = 2;
        let bad = handle_fi(&mut ctx, 1, "F'", "R");
        assert!(matches!(bad, MoveResult::EarlyReturn(_)));
    }

    #[test]
    fn test_handle_f_wrist_specific_paths() {
        let mut ctx = new_ctx();
        ctx.r_wrist = -1;
        assert!(matches!(
            handle_f(&mut ctx, 0, "F", " "),
            MoveResult::Success
        ));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = 1;
        assert!(matches!(
            handle_f(&mut ctx2, 0, "F", "D"),
            MoveResult::Success
        ));
    }

    #[test]
    fn test_handle_fi_wrist_specific_paths() {
        let mut ctx = new_ctx();
        ctx.l_wrist = -1;
        assert!(matches!(
            handle_fi(&mut ctx, 0, "F'", " "),
            MoveResult::Success
        ));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        assert!(matches!(
            handle_fi(&mut ctx2, 0, "F'", "D"),
            MoveResult::Success
        ));
    }

    #[test]
    fn test_handle_f2_variants() {
        let mut ctx = new_ctx();
        ctx.r_wrist = -1;
        assert!(matches!(handle_f2(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = -1;
        ctx2.r_wrist = 0;
        assert!(matches!(handle_f2(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = 1;
        assert!(matches!(handle_f2(&mut ctx3, 0, "D"), MoveResult::Success));

        let mut ctx4 = new_ctx();
        ctx4.l_wrist = 2;
        ctx4.r_wrist = 2;
        assert!(matches!(
            handle_f2(&mut ctx4, 0, "R"),
            MoveResult::EarlyReturn(_)
        ));
    }

    #[test]
    fn test_handle_fi_flat_wrist_fflick_branch() {
        let mut ctx = new_ctx();
        ctx.l_wrist = 0;
        ctx.r_wrist = 0;
        assert!(matches!(
            handle_fi(&mut ctx, 2, "f'", "R"),
            MoveResult::Success
        ));
    }

    #[test]
    fn test_handle_fi_lowercase_mv_fflick_branch() {
        let mut ctx = new_ctx();
        ctx.l_wrist = 0;
        ctx.r_wrist = 0;
        ctx.grip = -1;

        assert!(matches!(
            handle_fi(&mut ctx, 1, "f'", "R"),
            MoveResult::Success
        ));
        assert_eq!(ctx.l_index.location, "fflick");
    }

    #[test]
    fn test_handle_f2_l_wrist_one_non_d_prev_move_branch() {
        let mut ctx = new_ctx();
        ctx.l_wrist = 1;
        ctx.r_wrist = 0;
        assert!(matches!(handle_f2(&mut ctx, 1, "R"), MoveResult::Success));
    }

    #[test]
    fn test_handle_f_additional_branches() {
        let mut ctx = new_ctx();
        ctx.l_wrist = -1;
        ctx.r_wrist = 0;
        ctx.r_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(
            handle_f(&mut ctx, 1, "F", "R"),
            MoveResult::Success
        ));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = -1;
        ctx2.r_wrist = 1;
        ctx2.l_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(
            handle_f(&mut ctx2, 1, "F", "D"),
            MoveResult::Success
        ));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = -1;
        ctx3.grip = -1;
        ctx3.r_index = FingerState {
            time: ctx3.speed,
            location: "home",
        };
        assert!(matches!(
            handle_f(&mut ctx3, 1, "F", "D"),
            MoveResult::Success
        ));

        let mut ctx4 = new_ctx();
        ctx4.l_wrist = 0;
        ctx4.grip = -1;
        assert!(matches!(
            handle_f(&mut ctx4, 1, "F", "R"),
            MoveResult::Success
        ));

        let mut ctx5 = new_ctx();
        assert!(matches!(
            handle_f(&mut ctx5, 0, "F", "R"),
            MoveResult::Success
        ));
    }

    #[test]
    fn test_handle_fi_additional_branches() {
        let mut ctx = new_ctx();
        ctx.r_wrist = -1;
        ctx.l_wrist = 0;
        ctx.l_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(
            handle_fi(&mut ctx, 1, "F'", "R"),
            MoveResult::Success
        ));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = -1;
        ctx2.l_wrist = 1;
        ctx2.r_index = FingerState {
            time: -1.0,
            location: "uflick",
        };
        assert!(matches!(
            handle_fi(&mut ctx2, 1, "F'", "D"),
            MoveResult::Success
        ));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = -1;
        ctx3.grip = 1;
        ctx3.l_index = FingerState {
            time: ctx3.speed,
            location: "home",
        };
        assert!(matches!(
            handle_fi(&mut ctx3, 1, "F'", "D"),
            MoveResult::Success
        ));

        let mut ctx4 = new_ctx();
        ctx4.r_wrist = 0;
        ctx4.grip = 1;
        assert!(matches!(
            handle_fi(&mut ctx4, 1, "F'", "R"),
            MoveResult::Success
        ));

        let mut ctx5 = new_ctx();
        assert!(matches!(
            handle_fi(&mut ctx5, 0, "F'", "R"),
            MoveResult::Success
        ));
    }

    #[test]
    fn test_handle_f2_remaining_success_paths() {
        let mut ctx = new_ctx();
        ctx.l_wrist = -1;
        ctx.r_wrist = 0;
        assert!(matches!(handle_f2(&mut ctx, 0, "R"), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 1;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_f2(&mut ctx2, 0, "R"), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 1;
        ctx3.r_wrist = 0;
        assert!(matches!(handle_f2(&mut ctx3, 0, "D"), MoveResult::Success));
    }

    #[test]
    fn test_handle_f_specific_prev_move_branches() {
        let mut ctx = new_ctx();
        ctx.l_wrist = -1;
        ctx.grip = -1;
        assert!(matches!(
            handle_f(&mut ctx, 1, "F", "R"),
            MoveResult::Success
        ));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = 0;
        ctx2.grip = -1;
        assert!(matches!(
            handle_f(&mut ctx2, 1, "F", "D"),
            MoveResult::Success
        ));
    }

    #[test]
    fn test_handle_fi_specific_prev_move_and_j0_fallback() {
        let mut ctx = new_ctx();
        ctx.r_wrist = -1;
        ctx.grip = 1;
        assert!(matches!(
            handle_fi(&mut ctx, 1, "F'", "R"),
            MoveResult::Success
        ));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 0;
        ctx2.grip = 1;
        assert!(matches!(
            handle_fi(&mut ctx2, 1, "F'", "D"),
            MoveResult::Success
        ));

        let mut ctx3 = new_ctx();
        ctx3.grip = 0;
        assert!(matches!(
            handle_fi(&mut ctx3, 0, "F", "R"),
            MoveResult::Success
        ));
    }

    #[test]
    fn test_handle_f2_r_wrist_comparison_path() {
        let mut ctx = new_ctx();
        ctx.r_wrist = 1;
        ctx.l_wrist = 1;
        ctx.r_middle.location = "home";
        ctx.r_ring.location = "home";
        ctx.l_middle.location = "x";
        ctx.l_ring.location = "x";
        assert!(matches!(handle_f2(&mut ctx, 0, "R"), MoveResult::Success));
    }
}
