use super::{MoveResult, SimulationContext};

pub fn handle_ri(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    if ctx.r_wrist == 2 {
        ctx.r_wrist = 0;
    } else if ctx.r_wrist > -1 && !(ctx.l_wrist >= 1 && ctx.r_wrist <= 0) {
        ctx.r_wrist -= 1;
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            ctx.l_wrist as f64,
            (ctx.r_wrist - 1) as f64,
        ));
    }
    ctx.speed += ctx.params.wrist_mult;
    MoveResult::Success
}

pub fn handle_r(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    if ctx.r_wrist < 2 && !(ctx.l_wrist <= -1 && ctx.r_wrist >= 0) {
        ctx.r_wrist += 1;
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            ctx.l_wrist as f64,
            (ctx.r_wrist + 1) as f64,
        ));
    }
    ctx.speed += ctx.params.wrist_mult;
    MoveResult::Success
}

pub fn handle_r2(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    if ctx.r_wrist >= 1 && ctx.l_wrist < 1 {
        ctx.r_wrist = -1;
    } else if ctx.l_wrist > -1 {
        ctx.r_wrist += 2;
    } else {
        let new_r = if ctx.r_wrist > 0 {
            ctx.r_wrist - 2
        } else {
            ctx.r_wrist + 2
        };
        return MoveResult::EarlyReturn(ctx.make_early_return(j, ctx.l_wrist as f64, new_r as f64));
    }
    ctx.speed += ctx.params.double * ctx.params.wrist_mult;
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
    fn test_handle_ri_paths() {
        let mut ctx = new_ctx();
        ctx.r_wrist = 2;
        assert!(matches!(handle_ri(&mut ctx, 0), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 0;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_ri(&mut ctx2, 0), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = 0;
        ctx3.l_wrist = 1;
        assert!(matches!(handle_ri(&mut ctx3, 0), MoveResult::EarlyReturn(_)));
    }

    #[test]
    fn test_handle_r_paths() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_r(&mut ctx, 0), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 2;
        assert!(matches!(handle_r(&mut ctx2, 0), MoveResult::EarlyReturn(_)));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = 0;
        ctx3.l_wrist = -1;
        assert!(matches!(handle_r(&mut ctx3, 0), MoveResult::EarlyReturn(_)));
    }

    #[test]
    fn test_handle_r2_paths() {
        let mut ctx = new_ctx();
        ctx.r_wrist = 1;
        ctx.l_wrist = 0;
        assert!(matches!(handle_r2(&mut ctx, 0), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.r_wrist = 0;
        ctx2.l_wrist = 0;
        assert!(matches!(handle_r2(&mut ctx2, 0), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.r_wrist = 0;
        ctx3.l_wrist = -1;
        assert!(matches!(handle_r2(&mut ctx3, 0), MoveResult::EarlyReturn(_)));
    }
}
