pub const VALID_MOVES: [&str; 30] = [
    "bl", "bl2", "bl'", "br", "br2", "br'", "r", "r2", "r'", "u", "u'", "u2", "f", "f2", "f'", "d",
    "d2", "d'", "l", "l2", "l'", "x", "x'", "x2", "y", "y'", "y2", "z", "z'", "z2",
];

#[derive(Clone, Copy)]
pub struct MCCParams {
    pub wrist_mult: f64,
    pub push_mult: f64,
    pub ring_mult: f64,
    pub destabilize: f64,
    pub add_regrip: f64,
    pub double: f64,
    pub over_work_mult: f64,
    pub moveblock: f64,
    pub rotation: f64,
}

impl Default for MCCParams {
    fn default() -> Self {
        Self {
            wrist_mult: 0.8,
            push_mult: 1.3,
            ring_mult: 1.4,
            destabilize: 0.5,
            add_regrip: 1.0,
            double: 1.65,
            over_work_mult: 2.25,
            moveblock: 0.8,
            rotation: 3.5,
        }
    }
}

pub type FingerStateTuple = (f64, &'static str);

pub enum TestResult {
    Continue([f64; 6]),
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcc_params_default() {
        let params = MCCParams::default();
        assert_eq!(params.wrist_mult, 0.8);
        assert_eq!(params.push_mult, 1.3);
        assert_eq!(params.ring_mult, 1.4);
        assert_eq!(params.destabilize, 0.5);
        assert_eq!(params.add_regrip, 1.0);
        assert_eq!(params.double, 1.65);
        assert_eq!(params.over_work_mult, 2.25);
        assert_eq!(params.moveblock, 0.8);
        assert_eq!(params.rotation, 3.5);
    }

    #[test]
    fn test_mcc_params_clone() {
        let params = MCCParams::default();
        let cloned = params;
        assert_eq!(params.wrist_mult, cloned.wrist_mult);
        assert_eq!(params.push_mult, cloned.push_mult);
    }

    #[test]
    fn test_valid_moves_array() {
        assert_eq!(VALID_MOVES.len(), 30);
        assert!(VALID_MOVES.contains(&"u"));
        assert!(VALID_MOVES.contains(&"u'"));
        assert!(VALID_MOVES.contains(&"u2"));
        assert!(VALID_MOVES.contains(&"r"));
        assert!(VALID_MOVES.contains(&"r'"));
        assert!(VALID_MOVES.contains(&"r2"));
        assert!(VALID_MOVES.contains(&"f"));
        assert!(VALID_MOVES.contains(&"f'"));
        assert!(VALID_MOVES.contains(&"f2"));
        assert!(VALID_MOVES.contains(&"d"));
        assert!(VALID_MOVES.contains(&"d'"));
        assert!(VALID_MOVES.contains(&"d2"));
        assert!(VALID_MOVES.contains(&"l"));
        assert!(VALID_MOVES.contains(&"l'"));
        assert!(VALID_MOVES.contains(&"l2"));
        assert!(VALID_MOVES.contains(&"bl"));
        assert!(VALID_MOVES.contains(&"bl'"));
        assert!(VALID_MOVES.contains(&"bl2"));
        assert!(VALID_MOVES.contains(&"br"));
        assert!(VALID_MOVES.contains(&"br'"));
        assert!(VALID_MOVES.contains(&"br2"));
        assert!(VALID_MOVES.contains(&"x"));
        assert!(VALID_MOVES.contains(&"x'"));
        assert!(VALID_MOVES.contains(&"x2"));
        assert!(VALID_MOVES.contains(&"y"));
        assert!(VALID_MOVES.contains(&"y'"));
        assert!(VALID_MOVES.contains(&"y2"));
        assert!(VALID_MOVES.contains(&"z"));
        assert!(VALID_MOVES.contains(&"z'"));
        assert!(VALID_MOVES.contains(&"z2"));
    }

    #[test]
    fn test_test_result_continue() {
        let state = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let result = TestResult::Continue(state);
        match result {
            TestResult::Continue(arr) => {
                assert_eq!(arr[0], 1.0);
                assert_eq!(arr[5], 6.0);
            }
            TestResult::Error(_) => panic!("Expected Continue"),
        }
    }

    #[test]
    fn test_test_result_error() {
        let result = TestResult::Error("test error".to_string());
        match result {
            TestResult::Error(msg) => assert_eq!(msg, "test error"),
            TestResult::Continue(_) => panic!("Expected Error"),
        }
    }
}
