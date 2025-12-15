pub const VALID_MOVES: [&str; 27] = [
    "r", "r2", "r'", "u", "u'", "u2", "f", "f2", "f'",
    "d", "d2", "d'", "l", "l2", "l'", "b", "b2", "b'",
    "x", "x'", "x2", "y", "y'", "y2", "z", "z'", "z2"
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

pub type FingerState = (f64, &'static str);

pub enum TestResult {
    Continue([f64; 6]),
    Error(String),
}
