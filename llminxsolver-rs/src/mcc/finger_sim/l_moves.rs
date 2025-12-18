use super::{MoveResult, SimulationContext};

pub fn handle_l(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    if ctx.l_wrist == 2 {
        ctx.l_wrist = 0;
    } else if ctx.l_wrist > -1 && !(ctx.r_wrist >= 1 && ctx.l_wrist <= 0) {
        ctx.l_wrist -= 1;
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            (ctx.l_wrist - 1) as f64,
            ctx.r_wrist as f64,
        ));
    }
    ctx.speed += ctx.params.wrist_mult;
    MoveResult::Success
}

pub fn handle_li(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    if ctx.l_wrist < 2 && !(ctx.r_wrist <= -1 && ctx.l_wrist >= 0) {
        ctx.l_wrist += 1;
    } else {
        return MoveResult::EarlyReturn(ctx.make_early_return(
            j,
            (ctx.l_wrist + 1) as f64,
            ctx.r_wrist as f64,
        ));
    }
    ctx.speed += ctx.params.wrist_mult;
    MoveResult::Success
}

pub fn handle_l2(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    if ctx.l_wrist >= 1 && ctx.r_wrist < 1 {
        ctx.l_wrist = -1;
    } else if ctx.r_wrist > -1 {
        ctx.l_wrist += 2;
    } else {
        let new_l = if ctx.l_wrist > 0 {
            ctx.l_wrist - 2
        } else {
            ctx.l_wrist + 2
        };
        return MoveResult::EarlyReturn(ctx.make_early_return(j, new_l as f64, ctx.r_wrist as f64));
    }
    ctx.speed += ctx.params.double * ctx.params.wrist_mult;
    MoveResult::Success
}
