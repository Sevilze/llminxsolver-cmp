use super::types::MCCParams;

mod bl_moves;
mod br_moves;
mod d_moves;
mod f_moves;
mod l_moves;
mod r_moves;
mod rotation_moves;
mod u_moves;

#[derive(Clone, Copy)]
pub struct FingerState {
    pub time: f64,
    pub location: &'static str,
}

impl FingerState {
    pub fn new() -> Self {
        Self {
            time: -1.0,
            location: "home",
        }
    }
}

impl Default for FingerState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SimulationContext<'a> {
    pub l_thumb: FingerState,
    pub l_index: FingerState,
    pub l_middle: FingerState,
    pub l_ring: FingerState,
    pub r_thumb: FingerState,
    pub r_index: FingerState,
    pub r_middle: FingerState,
    pub r_ring: FingerState,
    pub l_oh_cool: f64,
    pub r_oh_cool: f64,
    pub l_wrist: i32,
    pub r_wrist: i32,
    pub grip: i32,
    pub speed: f64,
    pub params: &'a MCCParams,
}

impl<'a> SimulationContext<'a> {
    pub fn new(l_grip: i32, r_grip: i32, initial_speed: f64, params: &'a MCCParams) -> Self {
        Self {
            l_thumb: FingerState::new(),
            l_index: FingerState::new(),
            l_middle: FingerState::new(),
            l_ring: FingerState::new(),
            r_thumb: FingerState::new(),
            r_index: FingerState::new(),
            r_middle: FingerState::new(),
            r_ring: FingerState::new(),
            l_oh_cool: -1.0,
            r_oh_cool: -1.0,
            l_wrist: l_grip,
            r_wrist: r_grip,
            grip: 1,
            speed: initial_speed,
            params,
        }
    }

    pub fn l_max_time(&self) -> f64 {
        self.l_thumb
            .time
            .max(self.l_index.time)
            .max(self.l_middle.time)
            .max(self.l_ring.time)
    }

    pub fn r_max_time(&self) -> f64 {
        self.r_thumb
            .time
            .max(self.r_index.time)
            .max(self.r_middle.time)
            .max(self.r_ring.time)
    }

    pub fn make_early_return(&self, j: usize, l_wrist_val: f64, r_wrist_val: f64) -> [f64; 6] {
        [
            j as f64,
            self.speed,
            l_wrist_val,
            r_wrist_val,
            self.l_max_time(),
            self.r_max_time(),
        ]
    }
}

pub fn overwork(finger: &FingerState, location_prefer: &str, speed: f64, penalty: f64) -> f64 {
    if finger.location != location_prefer && speed - finger.time < penalty {
        penalty - speed + finger.time
    } else {
        0.0
    }
}

pub enum MoveResult {
    Success,
    EarlyReturn([f64; 6]),
}

pub fn test(
    split_seq: &[String],
    l_grip: i32,
    r_grip: i32,
    initial_speed: f64,
    params: &MCCParams,
) -> [f64; 6] {
    let mut ctx = SimulationContext::new(l_grip, r_grip, initial_speed, params);
    let mut udgrip = -1;
    let mut prev_speed: Option<f64> = None;
    let mut first_move_speed: Option<f64> = None;

    for j in 0..split_seq.len() {
        let mv = &split_seq[j];
        let normal_move = mv.to_uppercase();
        let prev_move = if j == 0 {
            " ".to_string()
        } else {
            split_seq[j - 1].to_uppercase()
        };

        if let Some(ps) = prev_speed {
            first_move_speed = Some(ctx.speed);
            ctx.speed = ps;
        }

        if j < split_seq.len() - 1 {
            let next = &split_seq[j + 1];
            if (mv.starts_with('U') && next.starts_with('D'))
                || (mv.starts_with('D') && next.starts_with('U'))
            {
                prev_speed = Some(ctx.speed);
            }
        }

        let result = match normal_move.as_str() {
            "R'" => r_moves::handle_ri(&mut ctx, j),
            "R" => r_moves::handle_r(&mut ctx, j),
            "R2" => r_moves::handle_r2(&mut ctx, j),
            "U" => u_moves::handle_u(&mut ctx, j, &prev_move),
            "U'" => u_moves::handle_ui(&mut ctx, j, &prev_move),
            "U2" => u_moves::handle_u2(&mut ctx, j),
            "D" => d_moves::handle_d(&mut ctx, j, &prev_move),
            "D'" => d_moves::handle_di(&mut ctx, j, &prev_move),
            "D2" => d_moves::handle_d2(&mut ctx, j, &prev_move),
            "L" => l_moves::handle_l(&mut ctx, j),
            "L'" => l_moves::handle_li(&mut ctx, j),
            "L2" => l_moves::handle_l2(&mut ctx, j),
            "F" => f_moves::handle_f(&mut ctx, j, mv, &prev_move),
            "F'" => f_moves::handle_fi(&mut ctx, j, mv, &prev_move),
            "F2" => f_moves::handle_f2(&mut ctx, j, &prev_move),

            "BL" => bl_moves::handle_bl(&mut ctx, j, &prev_move),
            "BL'" => bl_moves::handle_bli(&mut ctx, j, &prev_move),
            "BL2" => bl_moves::handle_bl2(&mut ctx, j, &prev_move),
            "BR" => br_moves::handle_br(&mut ctx, j, &prev_move),
            "BR'" => br_moves::handle_bri(&mut ctx, j, &prev_move),
            "BR2" => br_moves::handle_br2(&mut ctx, j, &prev_move),
            "X" => rotation_moves::handle_x(&mut ctx, j),
            "X'" => rotation_moves::handle_xi(&mut ctx, j),
            "X2" => rotation_moves::handle_x2(&mut ctx, j),
            "Y" | "Y'" | "Z" | "Z'" => rotation_moves::handle_y_z(&mut ctx, j),
            "Y2" | "Z2" => rotation_moves::handle_y2_z2(&mut ctx, j),
            _ => MoveResult::EarlyReturn(ctx.make_early_return(
                j,
                ctx.l_wrist as f64,
                ctx.r_wrist as f64,
            )),
        };

        match result {
            MoveResult::EarlyReturn(arr) => return arr,
            MoveResult::Success => {}
        }

        if let Some(fms) = first_move_speed {
            ctx.speed = ctx.speed.max(fms) + 0.5;
            prev_speed = None;
            first_move_speed = None;
        }

        if (mv.starts_with('R') || mv.starts_with('l')) && ctx.grip == -1 {
            ctx.grip = 1;
            ctx.speed += 0.65;
        } else if (mv.starts_with('r') || mv.starts_with('L')) && ctx.grip == 1 {
            ctx.grip = -1;
            ctx.speed += 0.65;
        }

        if mv.starts_with('d') && udgrip == -1 {
            udgrip = 1;
            ctx.speed += 2.25;
        } else if (mv.starts_with('U') || mv.starts_with('u')) && udgrip == 1 {
            udgrip = -1;
            ctx.speed += 2.25;
        }

        if j >= 2 {
            let prev2 = &split_seq[j - 2];
            let prev1 = split_seq[j - 1].to_uppercase();
            if (normal_move == "R" && mv == prev2 && prev1 == "U'")
                || (normal_move == "R'" && mv == prev2 && prev1 == "U")
            {
                ctx.speed -= 0.5;
            } else if (normal_move == "R" && mv == prev2 && prev1 == "D'" && ctx.r_wrist == 1)
                || (normal_move == "R'" && mv == prev2 && prev1 == "D")
            {
                ctx.speed -= 0.3;
            }
        }

        if normal_move == "U" && (ctx.l_wrist == -1 || ctx.r_wrist == -1) {
            ctx.speed += ctx.params.destabilize;
        }
        if (normal_move == "BL" || normal_move == "BR") && (ctx.l_wrist == 0 || ctx.r_wrist == 0) {
            ctx.speed += ctx.params.destabilize;
        }
        if normal_move == "D" && (ctx.l_wrist == 1 || ctx.r_wrist == 1) {
            ctx.speed += ctx.params.destabilize;
        }
    }

    [-1.0, ctx.speed, l_grip as f64, r_grip as f64, 0.0, 0.0]
}
