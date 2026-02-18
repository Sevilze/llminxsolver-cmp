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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcc::types::MCCParams;

    fn new_ctx() -> SimulationContext<'static> {
        let params = Box::leak(Box::new(MCCParams::default()));
        SimulationContext::new(0, 0, 0.0, params)
    }

    #[test]
    fn test_handle_x_and_xi() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_x(&mut ctx, 0), MoveResult::Success));
        assert!(matches!(handle_xi(&mut ctx, 1), MoveResult::Success));

        ctx.l_wrist = 1;
        ctx.r_wrist = 1;
        assert!(matches!(handle_x(&mut ctx, 2), MoveResult::EarlyReturn(_)));

        ctx.l_wrist = -1;
        ctx.r_wrist = -1;
        assert!(matches!(handle_xi(&mut ctx, 3), MoveResult::EarlyReturn(_)));
    }

    #[test]
    fn test_handle_x2_variants() {
        let mut ctx = new_ctx();
        ctx.l_wrist = 1;
        ctx.r_wrist = 1;
        assert!(matches!(handle_x2(&mut ctx, 0), MoveResult::Success));

        let mut ctx2 = new_ctx();
        ctx2.l_wrist = -1;
        ctx2.r_wrist = -1;
        assert!(matches!(handle_x2(&mut ctx2, 0), MoveResult::Success));

        let mut ctx3 = new_ctx();
        ctx3.l_wrist = 1;
        ctx3.r_wrist = 0;
        assert!(matches!(handle_x2(&mut ctx3, 0), MoveResult::EarlyReturn(_)));

        let mut ctx4 = new_ctx();
        ctx4.l_wrist = -1;
        ctx4.r_wrist = 0;
        assert!(matches!(handle_x2(&mut ctx4, 0), MoveResult::EarlyReturn(_)));
    }

    #[test]
    fn test_handle_y_z_variants() {
        let mut ctx = new_ctx();
        assert!(matches!(handle_y_z(&mut ctx, 0), MoveResult::EarlyReturn(_)));
        assert!(matches!(handle_y2_z2(&mut ctx, 1), MoveResult::EarlyReturn(_)));
        assert!(ctx.speed > 0.0);
    }
}
