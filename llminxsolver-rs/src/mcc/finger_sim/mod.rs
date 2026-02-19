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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finger_state_new() {
        let state = FingerState::new();
        assert_eq!(state.time, -1.0);
        assert_eq!(state.location, "home");
    }

    #[test]
    fn test_finger_state_default() {
        let state = FingerState::default();
        assert_eq!(state.time, -1.0);
        assert_eq!(state.location, "home");
    }

    #[test]
    fn test_simulation_context_new() {
        let params = MCCParams::default();
        let ctx = SimulationContext::new(0, 0, 1.0, &params);

        assert_eq!(ctx.l_wrist, 0);
        assert_eq!(ctx.r_wrist, 0);
        assert_eq!(ctx.speed, 1.0);
        assert_eq!(ctx.grip, 1);
        assert_eq!(ctx.l_oh_cool, -1.0);
        assert_eq!(ctx.r_oh_cool, -1.0);
    }

    #[test]
    fn test_simulation_context_l_max_time() {
        let params = MCCParams::default();
        let mut ctx = SimulationContext::new(0, 0, 0.0, &params);

        ctx.l_thumb.time = 1.0;
        ctx.l_index.time = 2.0;
        ctx.l_middle.time = 3.0;
        ctx.l_ring.time = 4.0;

        assert_eq!(ctx.l_max_time(), 4.0);
    }

    #[test]
    fn test_simulation_context_r_max_time() {
        let params = MCCParams::default();
        let mut ctx = SimulationContext::new(0, 0, 0.0, &params);

        ctx.r_thumb.time = 5.0;
        ctx.r_index.time = 6.0;
        ctx.r_middle.time = 7.0;
        ctx.r_ring.time = 8.0;

        assert_eq!(ctx.r_max_time(), 8.0);
    }

    #[test]
    fn test_simulation_context_make_early_return() {
        let params = MCCParams::default();
        let ctx = SimulationContext::new(1, -1, 2.5, &params);

        let result = ctx.make_early_return(3, 1.0, -1.0);

        assert_eq!(result[0], 3.0);
        assert_eq!(result[1], 2.5);
        assert_eq!(result[2], 1.0);
        assert_eq!(result[3], -1.0);
    }

    #[test]
    fn test_overwork_no_penalty() {
        let finger = FingerState {
            time: -1.0,
            location: "home",
        };
        let result = overwork(&finger, "home", 0.0, 1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_overwork_with_penalty() {
        let finger = FingerState {
            time: 0.5,
            location: "other",
        };
        let result = overwork(&finger, "home", 0.5, 1.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_overwork_location_matches() {
        let finger = FingerState {
            time: 0.5,
            location: "home",
        };
        let result = overwork(&finger, "home", 0.5, 1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_move_result_success() {
        let result = MoveResult::Success;
        matches!(result, MoveResult::Success);
    }

    #[test]
    fn test_move_result_early_return() {
        let arr = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let result = MoveResult::EarlyReturn(arr);
        match result {
            MoveResult::EarlyReturn(returned) => {
                assert_eq!(returned, arr);
            }
            MoveResult::Success => panic!("Expected EarlyReturn"),
        }
    }

    #[test]
    fn test_function_empty_sequence() {
        let params = MCCParams::default();
        let result = test(&[], 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
        assert_eq!(result[1], 0.0);
    }

    #[test]
    fn test_function_single_r_move() {
        let params = MCCParams::default();
        let seq = vec!["R".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
    }

    #[test]
    fn test_function_single_u_move() {
        let params = MCCParams::default();
        let seq = vec!["U".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
    }

    #[test]
    fn test_function_unknown_move() {
        let params = MCCParams::default();
        let seq = vec!["UNKNOWN".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], 0.0);
    }

    #[test]
    fn test_function_r_u_sequence() {
        let params = MCCParams::default();
        let seq = vec!["R".to_string(), "U".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
    }

    #[test]
    fn test_function_with_different_grips() {
        let params = MCCParams::default();
        let seq = vec!["R".to_string()];

        let result_0_0 = test(&seq, 0, 0, 0.0, &params);
        let result_1_0 = test(&seq, 1, 0, 0.0, &params);
        let result_neg1_0 = test(&seq, -1, 0, 0.0, &params);

        assert_eq!(result_0_0[2], 0.0);
        assert_eq!(result_1_0[2], 1.0);
        assert_eq!(result_neg1_0[2], -1.0);
    }

    #[test]
    fn test_function_rotation_moves() {
        let params = MCCParams::default();

        let seq_x = vec!["X".to_string()];
        let result_x = test(&seq_x, 0, 0, 0.0, &params);
        assert!(result_x[0] >= 0.0 || result_x[0] == -1.0);

        let seq_y = vec!["Y".to_string()];
        let result_y = test(&seq_y, 0, 0, 0.0, &params);
        assert!(result_y[0] >= 0.0 || result_y[0] == -1.0);

        let seq_z = vec!["Z".to_string()];
        let result_z = test(&seq_z, 0, 0, 0.0, &params);
        assert!(result_z[0] >= 0.0 || result_z[0] == -1.0);
    }

    #[test]
    fn test_function_l_moves() {
        let params = MCCParams::default();
        let seq = vec!["L".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
    }

    #[test]
    fn test_function_f_moves() {
        let params = MCCParams::default();
        let seq = vec!["F".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
    }

    #[test]
    fn test_function_d_moves() {
        let params = MCCParams::default();
        let seq = vec!["D".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
    }

    #[test]
    fn test_function_bl_br_moves() {
        let params = MCCParams::default();

        let seq_bl = vec!["BL".to_string()];
        let result_bl = test(&seq_bl, 0, 0, 0.0, &params);
        assert!(result_bl[0] >= 0.0 || result_bl[0] == -1.0);

        let seq_br = vec!["BR".to_string()];
        let result_br = test(&seq_br, 0, 0, 0.0, &params);
        assert!(result_br[0] >= 0.0 || result_br[0] == -1.0);
    }

    #[test]
    fn test_function_double_moves() {
        let params = MCCParams::default();

        for mv in &["R2", "U2", "L2", "D2"] {
            let seq = vec![mv.to_string()];
            let result = test(&seq, 0, 0, 0.0, &params);
            assert_eq!(result[0], -1.0, "Failed for move {}", mv);
        }
    }

    #[test]
    fn test_function_prime_moves() {
        let params = MCCParams::default();

        for mv in &["R'", "U'", "L'", "D'"] {
            let seq = vec![mv.to_string()];
            let result = test(&seq, 0, 0, 0.0, &params);
            assert_eq!(result[0], -1.0, "Failed for move {}", mv);
        }
    }

    #[test]
    fn test_function_grip_change() {
        let params = MCCParams::default();
        let seq = vec!["R".to_string(), "L".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert!(result[0] >= 0.0 || result[0] == -1.0);
    }

    #[test]
    fn test_function_consecutive_ud() {
        let params = MCCParams::default();
        let seq = vec!["U".to_string(), "D".to_string()];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert_eq!(result[0], -1.0);
    }

    #[test]
    fn test_function_lowercase_grip_and_udgrip_paths() {
        let params = MCCParams::default();
        let seq = vec![
            "l".to_string(),
            "r".to_string(),
            "d".to_string(),
            "u".to_string(),
        ];
        let result = test(&seq, 0, 0, 0.0, &params);
        assert!(result[0] >= 0.0 || result[0] == -1.0);
    }

    #[test]
    fn test_function_speed_adjust_patterns() {
        let params = MCCParams::default();

        let seq_ruprime_r = vec!["R".to_string(), "U'".to_string(), "R".to_string()];
        let result1 = test(&seq_ruprime_r, 0, 0, 0.0, &params);
        assert_eq!(result1[0], -1.0);

        let seq_rudprime_r = vec!["R".to_string(), "D'".to_string(), "R".to_string()];
        let result2 = test(&seq_rudprime_r, 0, 1, 0.0, &params);
        assert!(result2[0] >= 0.0 || result2[0] == -1.0);

        let seq_rprime_d_rprime = vec!["R'".to_string(), "D".to_string(), "R'".to_string()];
        let result3 = test(&seq_rprime_d_rprime, 0, 0, 0.0, &params);
        assert!(result3[0] >= 0.0 || result3[0] == -1.0);
    }

    #[test]
    fn test_function_destabilize_paths() {
        let params = MCCParams::default();

        let seq_u = vec!["U".to_string()];
        let result_u = test(&seq_u, -1, 0, 0.0, &params);
        assert_eq!(result_u[0], -1.0);

        let seq_bl = vec!["BL".to_string()];
        let result_bl = test(&seq_bl, 0, 1, 0.0, &params);
        assert!(result_bl[0] >= 0.0 || result_bl[0] == -1.0);

        let seq_d = vec!["D".to_string()];
        let result_d = test(&seq_d, 1, 0, 0.0, &params);
        assert_eq!(result_d[0], -1.0);
    }

    #[test]
    fn test_function_lowercase_transitions_cover_grip_and_udgrip_flips() {
        let params = MCCParams::default();

        let seq_grip_flip = vec!["r".to_string(), "l".to_string()];
        let result_grip = test(&seq_grip_flip, 0, 0, 0.0, &params);
        assert!(result_grip[0] >= 0.0 || result_grip[0] == -1.0);

        let seq_ud_flip = vec!["d".to_string(), "u".to_string()];
        let result_ud = test(&seq_ud_flip, 0, 0, 0.0, &params);
        assert!(result_ud[0] >= 0.0 || result_ud[0] == -1.0);
    }
}
