use super::{MoveResult, SimulationContext};

pub fn handle_x(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    ctx.l_wrist += 1;
    ctx.r_wrist += 1;
    if ctx.l_wrist > 1 || ctx.r_wrist > 1 {
        return MoveResult::EarlyReturn([
            j as f64 + 1.0,
            ctx.speed,
            ctx.l_wrist as f64,
            ctx.r_wrist as f64,
            ctx.l_max_time(),
            ctx.r_max_time(),
        ]);
    }
    MoveResult::Success
}

pub fn handle_xi(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    ctx.l_wrist -= 1;
    ctx.r_wrist -= 1;
    if ctx.l_wrist < -1 || ctx.r_wrist < -1 {
        return MoveResult::EarlyReturn([
            j as f64 + 1.0,
            ctx.speed,
            ctx.l_wrist as f64,
            ctx.r_wrist as f64,
            ctx.l_max_time(),
            ctx.r_max_time(),
        ]);
    }
    MoveResult::Success
}

pub fn handle_x2(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    if ctx.l_wrist >= 1 && ctx.r_wrist >= 1 {
        ctx.l_wrist -= 2;
        ctx.r_wrist -= 2;
    } else if ctx.l_wrist <= -1 && ctx.r_wrist <= -1 {
        ctx.l_wrist += 2;
        ctx.r_wrist += 2;
    } else if ctx.l_wrist + ctx.r_wrist > 0 {
        return MoveResult::EarlyReturn([
            j as f64,
            ctx.speed,
            (ctx.l_wrist - 2) as f64,
            (ctx.r_wrist - 2) as f64,
            ctx.l_max_time(),
            ctx.r_max_time(),
        ]);
    } else {
        return MoveResult::EarlyReturn([
            j as f64,
            ctx.speed,
            (ctx.l_wrist + 2) as f64,
            (ctx.r_wrist + 2) as f64,
            ctx.l_max_time(),
            ctx.r_max_time(),
        ]);
    }
    MoveResult::Success
}

pub fn handle_y_z(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    ctx.speed += ctx.params.rotation;
    MoveResult::EarlyReturn([
        j as f64 + 1.0,
        ctx.speed,
        0.0,
        0.0,
        ctx.l_max_time(),
        ctx.r_max_time(),
    ])
}

pub fn handle_y2_z2(ctx: &mut SimulationContext, j: usize) -> MoveResult {
    ctx.speed += ctx.params.rotation * ctx.params.double;
    MoveResult::EarlyReturn([
        j as f64 + 1.0,
        ctx.speed,
        0.0,
        0.0,
        ctx.l_max_time(),
        ctx.r_max_time(),
    ])
}
