use super::types::MCCParams;

#[derive(Clone, Copy)]
struct FingerState {
    time: f64,
    location: &'static str,
}

impl FingerState {
    fn new() -> Self {
        Self { time: -1.0, location: "home" }
    }
}

fn overwork(finger: &FingerState, location_prefer: &str, speed: f64, penalty: f64) -> f64 {
    if finger.location != location_prefer && speed - finger.time < penalty {
        penalty - speed + finger.time
    } else {
        0.0
    }
}

pub fn test(
    split_seq: &[String],
    l_grip: i32,
    r_grip: i32,
    initial_speed: f64,
    params: &MCCParams,
) -> [f64; 6] {
    let mut l_thumb = FingerState::new();
    let mut l_index = FingerState::new();
    let mut l_middle = FingerState::new();
    let mut l_ring = FingerState::new();
    let mut r_thumb = FingerState::new();
    let mut r_index = FingerState::new();
    let mut r_middle = FingerState::new();
    let mut r_ring = FingerState::new();
    
    let mut l_oh_cool = -1.0;
    let mut r_oh_cool = -1.0;
    let mut l_wrist = l_grip;
    let mut r_wrist = r_grip;
    let mut grip = 1;
    let mut udgrip = -1;
    let mut prev_speed: Option<f64> = None;
    let mut first_move_speed: Option<f64> = None;
    let mut speed = initial_speed;

    for j in 0..split_seq.len() {
        let mv = &split_seq[j];
        let normal_move = mv.to_uppercase();
        let prev_move = if j == 0 { " ".to_string() } else { split_seq[j - 1].to_uppercase() };

        if let Some(ps) = prev_speed {
            first_move_speed = Some(speed);
            speed = ps;
        }

        if j < split_seq.len() - 1 {
            let next = &split_seq[j + 1];
            if (mv.starts_with('U') && next.starts_with('D')) || (mv.starts_with('D') && next.starts_with('U')) {
                prev_speed = Some(speed);
            }
        }

        let result = match normal_move.as_str() {
            "R'" => {
                if r_wrist == 2 { r_wrist = 0; }
                else if r_wrist > -1 && !(l_wrist >= 1 && r_wrist <= 0) { r_wrist -= 1; }
                else { return [j as f64, speed, l_wrist as f64, (r_wrist - 1) as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)]; }
                speed += params.wrist_mult;
                true
            }
            "R" => {
                if r_wrist < 2 && !(l_wrist <= -1 && r_wrist >= 0) { r_wrist += 1; }
                else { return [j as f64, speed, l_wrist as f64, (r_wrist + 1) as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)]; }
                speed += params.wrist_mult;
                true
            }
            "R2" => {
                if r_wrist >= 1 && l_wrist < 1 { r_wrist = -1; }
                else if l_wrist > -1 { r_wrist += 2; }
                else { 
                    let new_r = if r_wrist > 0 { r_wrist - 2 } else { r_wrist + 2 };
                    return [j as f64, speed, l_wrist as f64, new_r as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)]; 
                }
                speed += params.double * params.wrist_mult;
                true
            }
            "U" => {
                if r_wrist == 0 && (r_thumb.time + params.over_work_mult <= speed || r_thumb.location != "top") && r_index.location != "m" {
                    let ow_index = overwork(&r_index, "home", speed, params.over_work_mult);
                    let ow_middle = overwork(&r_middle, "home", speed, params.over_work_mult);
                    if ow_index <= ow_middle {
                        speed += ow_index + 1.0;
                        r_index = FingerState { time: speed, location: "uflick" };
                    } else {
                        speed += ow_middle + 1.0;
                        r_index = FingerState { time: speed, location: "uflick" };
                        r_middle = FingerState { time: speed, location: "uflick" };
                    }
                } else if r_wrist == 1 && l_wrist == 0 {
                    speed += overwork(&l_index, "uflick", speed, params.over_work_mult);
                    if prev_move == "B'" { speed += params.moveblock + params.push_mult; }
                    else if prev_move.starts_with("B'") { speed += params.moveblock * 0.5 + params.push_mult; }
                    else { speed += params.push_mult; }
                    l_index = FingerState { time: speed, location: "home" };
                } else if l_wrist == 0 && !prev_move.starts_with('F') && !prev_move.starts_with('B') {
                    if l_index.location == "uflick" {
                        speed += overwork(&l_index, "eido", speed, 0.75 * params.over_work_mult);
                        speed = speed.max(l_oh_cool + 2.5);
                    } else {
                        speed += overwork(&l_index, "eido", speed, 1.25 * params.over_work_mult);
                    }
                    speed += 1.15 * params.push_mult;
                    l_index = FingerState { time: speed, location: "uflick" };
                    l_oh_cool = speed;
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "U'" => {
                if l_wrist == 0 && (l_thumb.time + params.over_work_mult <= speed || l_thumb.location != "top") && l_index.location != "m" {
                    let ow_index = overwork(&l_index, "home", speed, params.over_work_mult);
                    let ow_middle = overwork(&l_middle, "home", speed, params.over_work_mult);
                    if ow_index <= ow_middle {
                        speed += ow_index + 1.0;
                        l_index = FingerState { time: speed, location: "uflick" };
                    } else {
                        speed += ow_middle + 1.0;
                        l_index = FingerState { time: speed, location: "uflick" };
                        l_middle = FingerState { time: speed, location: "uflick" };
                    }
                } else if l_wrist == 1 && r_wrist == 0 {
                    speed += overwork(&r_index, "uflick", speed, params.over_work_mult);
                    if prev_move == "B" { speed += params.moveblock + params.push_mult; }
                    else if prev_move.starts_with("B'") { speed += params.moveblock * 0.5 + params.push_mult; }
                    else { speed += params.push_mult; }
                    r_index = FingerState { time: speed, location: "home" };
                } else if r_wrist == 0 && !prev_move.starts_with('F') && !prev_move.starts_with('B') {
                    if r_index.location == "uflick" {
                        speed += overwork(&r_index, "eido", speed, 0.75 * params.over_work_mult);
                        speed = speed.max(r_oh_cool + 2.5);
                    } else {
                        speed += overwork(&r_index, "eido", speed, 1.25 * params.over_work_mult);
                    }
                    speed += 1.15 * params.push_mult;
                    r_index = FingerState { time: speed, location: "uflick" };
                    r_oh_cool = speed;
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "U2" => {
                let r_ow = overwork(&r_index, "home", speed, params.over_work_mult).max(overwork(&r_middle, "home", speed, params.over_work_mult)).max(overwork(&r_ring, "u2grip", speed, params.over_work_mult));
                let l_ow = overwork(&l_index, "home", speed, params.over_work_mult).max(overwork(&l_middle, "home", speed, params.over_work_mult)).max(overwork(&l_ring, "u2grip", speed, params.over_work_mult));
                if r_wrist == 0 && (l_index.location == "m" || l_wrist != 0 || r_ow <= l_ow) {
                    speed += overwork(&r_index, "home", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&r_ring, "u2grip", speed, params.moveblock * params.over_work_mult);
                    speed += params.double;
                    r_index = FingerState { time: speed, location: "uflick" };
                    r_middle = FingerState { time: speed, location: "uflick" };
                } else if l_wrist == 0 {
                    speed += overwork(&l_index, "home", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&l_ring, "u2grip", speed, params.moveblock * params.over_work_mult);
                    speed += params.double;
                    l_index = FingerState { time: speed, location: "uflick" };
                    l_middle = FingerState { time: speed, location: "uflick" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "D" => {
                let l_ow = overwork(&l_ring, "home", speed, params.over_work_mult).max(overwork(&l_middle, "home", speed, params.over_work_mult));
                let r_ow = overwork(&r_ring, "dflick", speed, params.over_work_mult).max(overwork(&r_middle, "home", speed, params.over_work_mult));
                if l_wrist == 0 && (r_wrist != 0 || l_ow <= r_ow) {
                    speed += overwork(&l_ring, "home", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('B') { speed += params.moveblock * 0.5 + params.ring_mult; } else { speed += params.ring_mult; }
                    l_ring = FingerState { time: speed, location: "dflick" };
                } else if r_wrist == 0 && !prev_move.starts_with('B') {
                    speed += overwork(&r_ring, "dflick", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += params.ring_mult * params.push_mult;
                    r_ring = FingerState { time: speed, location: "home" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "D'" => {
                let r_ow = overwork(&r_ring, "home", speed, params.over_work_mult).max(overwork(&r_middle, "home", speed, params.over_work_mult));
                let l_ow = overwork(&l_ring, "dflick", speed, params.over_work_mult).max(overwork(&l_middle, "home", speed, params.over_work_mult));
                if r_wrist == 0 && (l_wrist != 0 || r_ow <= l_ow) {
                    speed += overwork(&r_ring, "home", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('B') { speed += params.moveblock * 0.5 + params.ring_mult; } else { speed += params.ring_mult; }
                    r_ring = FingerState { time: speed, location: "dflick" };
                } else if l_wrist == 0 && !prev_move.starts_with('B') {
                    speed += overwork(&l_ring, "dflick", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += params.ring_mult * params.push_mult;
                    l_ring = FingerState { time: speed, location: "home" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "D2" => {
                let r_ow = overwork(&r_middle, "home", speed, params.over_work_mult).max(overwork(&r_ring, "home", speed, params.over_work_mult));
                let l_ow = overwork(&l_middle, "home", speed, params.over_work_mult).max(overwork(&l_ring, "home", speed, params.over_work_mult));
                if r_wrist == 0 && (l_wrist != 0 || r_ow <= l_ow) {
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&r_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('B') { speed += params.moveblock * 0.5 + params.double * params.ring_mult; } else { speed += params.double * params.ring_mult; }
                    r_ring = FingerState { time: speed, location: "dflick" };
                } else if l_wrist == 0 {
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&l_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('B') { speed += params.moveblock * 0.5 + params.double * params.ring_mult; } else { speed += params.double * params.ring_mult; }
                    l_ring = FingerState { time: speed, location: "dflick" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "L" => {
                if l_wrist == 2 { l_wrist = 0; }
                else if l_wrist > -1 && !(r_wrist >= 1 && l_wrist <= 0) { l_wrist -= 1; }
                else { return [j as f64, speed, (l_wrist - 1) as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)]; }
                speed += params.wrist_mult;
                true
            }
            "L'" => {
                if l_wrist < 2 && !(r_wrist <= -1 && l_wrist >= 0) { l_wrist += 1; }
                else { return [j as f64, speed, (l_wrist + 1) as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)]; }
                speed += params.wrist_mult;
                true
            }
            "L2" => {
                if l_wrist >= 1 && r_wrist < 1 { l_wrist = -1; }
                else if r_wrist > -1 { l_wrist += 2; }
                else {
                    let new_l = if l_wrist > 0 { l_wrist - 2 } else { l_wrist + 2 };
                    return [j as f64, speed, new_l as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                speed += params.double * params.wrist_mult;
                true
            }
            "F" => {
                if r_wrist == -1 {
                    speed += overwork(&r_index, "home", speed, params.over_work_mult) + 1.0;
                    r_index = FingerState { time: speed, location: "uflick" };
                } else if l_wrist == 1 && mv != "f" {
                    speed += overwork(&l_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += params.moveblock * 0.5 + params.ring_mult; } else { speed += 1.0; }
                    l_ring = FingerState { time: speed, location: "dflick" };
                } else if r_wrist == 1 && !prev_move.starts_with('D') && mv != "f" {
                    speed += overwork(&r_ring, "dflick", speed, params.over_work_mult);
                    speed += params.ring_mult * params.push_mult;
                    r_ring = FingerState { time: speed, location: "home" };
                } else if l_wrist == -1 && r_wrist == 0 && overwork(&r_index, "uflick", speed, params.over_work_mult) == 0.0 {
                    speed += 1.0;
                    r_index = FingerState { time: speed, location: "fflick" };
                } else if l_wrist == -1 && overwork(&l_index, "uflick", speed, params.over_work_mult) == 0.0 && !prev_move.starts_with('U') {
                    speed += params.push_mult;
                    l_index = FingerState { time: speed, location: "home" };
                } else if l_wrist == -1 && grip == -1 {
                    speed += overwork(&l_thumb, "top", speed, 0.9 * params.over_work_mult);
                    speed += overwork(&l_index, "top", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += 1.8; } else { speed += 1.0; }
                    l_wrist += 1;
                    l_thumb = FingerState { time: speed, location: "leftu" };
                    l_index = FingerState { time: speed, location: "top" };
                } else if l_wrist == 0 && grip == -1 {
                    speed += overwork(&l_thumb, "bottom", speed, params.over_work_mult);
                    speed += overwork(&l_index, "top", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += 2.05; } else { speed += 1.25; }
                    l_thumb = FingerState { time: speed, location: "top" };
                    l_index = FingerState { time: speed, location: "top" };
                } else if r_wrist == 0 && l_wrist == 0 && mv == "f" {
                    speed += overwork(&r_index, "uflick", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult) + 1.0;
                    r_index = FingerState { time: speed, location: "fflick" };
                } else if j == 0 && r_wrist == 0 && l_wrist == 0 {
                    speed += overwork(&r_thumb, "top", speed, params.over_work_mult) + 1.0;
                    r_thumb = FingerState { time: speed, location: "rdown" };
                    r_middle = FingerState { time: speed, location: "uflick" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "F'" => {
                if l_wrist == -1 {
                    speed += overwork(&l_index, "home", speed, params.over_work_mult) + 1.0;
                    l_index = FingerState { time: speed, location: "uflick" };
                } else if r_wrist == 1 && mv != "f" {
                    speed += overwork(&r_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += params.moveblock * 0.5 + params.ring_mult; } else { speed += 1.0; }
                    r_ring = FingerState { time: speed, location: "dflick" };
                } else if l_wrist == 1 && !prev_move.starts_with('D') && mv != "f" {
                    speed += overwork(&l_ring, "dflick", speed, params.over_work_mult);
                    speed += params.ring_mult * params.push_mult;
                    l_ring = FingerState { time: speed, location: "home" };
                } else if r_wrist == -1 && l_wrist == 0 && overwork(&l_index, "uflick", speed, params.over_work_mult) == 0.0 {
                    speed += 1.0;
                    l_index = FingerState { time: speed, location: "fflick" };
                } else if r_wrist == -1 && overwork(&r_index, "uflick", speed, params.over_work_mult) == 0.0 && !prev_move.starts_with('U') {
                    speed += params.push_mult;
                    r_index = FingerState { time: speed, location: "home" };
                } else if r_wrist == -1 && grip == 1 {
                    speed += overwork(&r_thumb, "top", speed, 0.9 * params.over_work_mult);
                    speed += overwork(&r_index, "top", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += 1.8; } else { speed += 1.0; }
                    r_wrist += 1;
                    r_thumb = FingerState { time: speed, location: "rightu" };
                    r_index = FingerState { time: speed, location: "top" };
                } else if r_wrist == 0 && grip == 1 {
                    speed += overwork(&r_thumb, "bottom", speed, params.over_work_mult);
                    speed += overwork(&r_index, "top", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += 2.05; } else { speed += 1.25; }
                    r_thumb = FingerState { time: speed, location: "top" };
                    r_index = FingerState { time: speed, location: "top" };
                } else if l_wrist == 0 && r_wrist == 0 && mv == "f'" {
                    speed += overwork(&l_index, "uflick", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult) + 1.0;
                    l_index = FingerState { time: speed, location: "fflick" };
                } else if j == 0 && r_wrist == 0 && l_wrist == 0 {
                    speed += overwork(&l_thumb, "top", speed, params.over_work_mult) + 1.0;
                    l_thumb = FingerState { time: speed, location: "rdown" };
                    l_middle = FingerState { time: speed, location: "uflick" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "F2" => {
                let r_ow = overwork(&r_index, "home", speed, params.over_work_mult).max(overwork(&r_middle, "home", speed, params.over_work_mult)).max(overwork(&r_ring, "u2grip", speed, params.over_work_mult));
                let l_ow = overwork(&l_index, "home", speed, params.over_work_mult).max(overwork(&l_middle, "home", speed, params.over_work_mult)).max(overwork(&l_ring, "u2grip", speed, params.over_work_mult));
                if r_wrist == -1 && (l_wrist != -1 || r_ow <= l_ow) {
                    speed += overwork(&r_index, "home", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&r_ring, "u2grip", speed, params.over_work_mult);
                    speed += params.double;
                    r_index = FingerState { time: speed, location: "uflick" };
                    r_middle = FingerState { time: speed, location: "uflick" };
                } else if l_wrist == -1 {
                    speed += overwork(&l_index, "home", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&l_ring, "u2grip", speed, params.over_work_mult);
                    speed += params.double;
                    l_index = FingerState { time: speed, location: "uflick" };
                    l_middle = FingerState { time: speed, location: "uflick" };
                } else if r_wrist == 1 && (l_wrist != 1 || overwork(&r_middle, "home", speed, params.over_work_mult).max(overwork(&r_ring, "home", speed, params.over_work_mult)) <= overwork(&l_middle, "home", speed, params.over_work_mult).max(overwork(&l_ring, "home", speed, params.over_work_mult))) {
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&r_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += params.double * params.ring_mult + params.moveblock * 0.5; } else { speed += params.double * params.ring_mult; }
                    r_ring = FingerState { time: speed, location: "dflick" };
                } else if l_wrist == 1 {
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&l_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('D') { speed += params.double * params.ring_mult + params.moveblock * 0.5; } else { speed += params.double * params.ring_mult; }
                    l_ring = FingerState { time: speed, location: "dflick" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "B" => {
                if r_wrist == 1 {
                    speed += overwork(&r_index, "home", speed, params.over_work_mult) + 1.0;
                    r_index = FingerState { time: speed, location: "uflick" };
                } else if l_wrist == -1 {
                    speed += overwork(&l_ring, "home", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('U') { speed += params.moveblock * 0.5 + params.ring_mult; } else { speed += params.ring_mult; }
                    l_ring = FingerState { time: speed, location: "dflick" };
                } else if l_wrist == 1 && !prev_move.starts_with('U') && !prev_move.starts_with('D') {
                    if l_index.location == "uflick" {
                        speed += overwork(&l_index, "eido", speed, 0.75 * params.over_work_mult);
                        speed = speed.max(l_oh_cool + 2.5);
                    } else {
                        speed += overwork(&l_index, "eido", speed, 1.25 * params.over_work_mult);
                    }
                    speed += 1.15 * params.push_mult;
                    l_index = FingerState { time: speed, location: "uflick" };
                    l_oh_cool = speed;
                } else if l_wrist == 0 && (r_wrist == 1 || r_wrist == -1) {
                    speed += overwork(&l_index, "top", speed, 0.9 * params.over_work_mult);
                    if prev_move.starts_with('U') { speed += 1.45; } else { speed += 1.0; }
                    l_index = FingerState { time: speed, location: "leftdb" };
                } else if r_wrist == -1 && !prev_move.starts_with('U') {
                    speed += overwork(&r_ring, "dflick", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += params.ring_mult * params.push_mult;
                    r_ring = FingerState { time: speed, location: "home" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "B'" => {
                if l_wrist == 1 {
                    speed += overwork(&l_index, "home", speed, params.over_work_mult) + 1.0;
                    l_index = FingerState { time: speed, location: "uflick" };
                } else if r_wrist == -1 {
                    speed += overwork(&r_ring, "home", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('U') { speed += params.moveblock * 0.5 + params.ring_mult; } else { speed += params.ring_mult; }
                    r_ring = FingerState { time: speed, location: "dflick" };
                } else if r_wrist == 1 && !prev_move.starts_with('U') && !prev_move.starts_with('D') {
                    if r_index.location == "uflick" {
                        speed += overwork(&r_index, "eido", speed, 0.75 * params.over_work_mult);
                        speed = speed.max(r_oh_cool + 2.5);
                    } else {
                        speed += overwork(&r_index, "eido", speed, 1.25 * params.over_work_mult);
                    }
                    speed += 1.15 * params.push_mult;
                    r_index = FingerState { time: speed, location: "uflick" };
                    r_oh_cool = speed;
                } else if r_wrist == 0 && (l_wrist == 1 || l_wrist == -1) {
                    speed += overwork(&r_index, "top", speed, 0.9 * params.over_work_mult);
                    if prev_move.starts_with('U') { speed += 1.45; } else { speed += 1.0; }
                    r_index = FingerState { time: speed, location: "rightdb" };
                } else if l_wrist == -1 && !prev_move.starts_with('U') {
                    speed += overwork(&l_ring, "dflick", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += params.ring_mult * params.push_mult;
                    l_ring = FingerState { time: speed, location: "home" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "B2" => {
                let r_ow = overwork(&r_index, "home", speed, params.over_work_mult).max(overwork(&r_middle, "home", speed, params.over_work_mult)).max(overwork(&r_ring, "u2grip", speed, params.over_work_mult));
                let l_ow = overwork(&l_index, "home", speed, params.over_work_mult).max(overwork(&l_middle, "home", speed, params.over_work_mult)).max(overwork(&l_ring, "u2grip", speed, params.over_work_mult));
                if r_wrist == 1 && (l_wrist != 1 || r_ow <= l_ow) {
                    speed += overwork(&r_index, "home", speed, params.over_work_mult);
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&r_ring, "u2grip", speed, params.over_work_mult);
                    speed += params.double;
                    r_index = FingerState { time: speed, location: "uflick" };
                    r_middle = FingerState { time: speed, location: "uflick" };
                } else if l_wrist == 1 {
                    speed += overwork(&l_index, "home", speed, params.over_work_mult);
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&l_ring, "u2grip", speed, params.over_work_mult);
                    speed += params.double;
                    l_index = FingerState { time: speed, location: "uflick" };
                    l_middle = FingerState { time: speed, location: "uflick" };
                } else if l_wrist == -1 && (r_wrist != -1 || overwork(&r_middle, "home", speed, params.over_work_mult).max(overwork(&r_ring, "home", speed, params.over_work_mult)) > overwork(&l_middle, "home", speed, params.over_work_mult).max(overwork(&l_ring, "home", speed, params.over_work_mult))) {
                    speed += overwork(&l_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&l_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('U') { speed += params.moveblock * 0.5 + params.double * params.ring_mult; } else { speed += params.double * params.ring_mult; }
                    l_ring = FingerState { time: speed, location: "dflick" };
                } else if r_wrist == -1 {
                    speed += overwork(&r_middle, "home", speed, params.over_work_mult);
                    speed += overwork(&r_ring, "home", speed, params.over_work_mult);
                    if prev_move.starts_with('U') { speed += params.moveblock * 0.5 + params.double * params.ring_mult; } else { speed += params.double * params.ring_mult; }
                    r_ring = FingerState { time: speed, location: "dflick" };
                } else {
                    return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "X" => {
                l_wrist += 1;
                r_wrist += 1;
                if l_wrist > 1 || r_wrist > 1 {
                    return [j as f64 + 1.0, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "X'" => {
                l_wrist -= 1;
                r_wrist -= 1;
                if l_wrist < -1 || r_wrist < -1 {
                    return [j as f64 + 1.0, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "X2" => {
                if l_wrist >= 1 && r_wrist >= 1 {
                    l_wrist -= 2;
                    r_wrist -= 2;
                } else if l_wrist <= -1 && r_wrist <= -1 {
                    l_wrist += 2;
                    r_wrist += 2;
                } else if l_wrist + r_wrist > 0 {
                    return [j as f64, speed, (l_wrist - 2) as f64, (r_wrist - 2) as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                } else {
                    return [j as f64, speed, (l_wrist + 2) as f64, (r_wrist + 2) as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
                }
                true
            }
            "Y" | "Y'" | "Z" | "Z'" => {
                speed += params.rotation;
                return [j as f64 + 1.0, speed, 0.0, 0.0, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
            }
            "Y2" | "Z2" => {
                speed += params.rotation * params.double;
                return [j as f64 + 1.0, speed, 0.0, 0.0, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
            }
            _ => {
                return [j as f64, speed, l_wrist as f64, r_wrist as f64, l_thumb.time.max(l_index.time).max(l_middle.time).max(l_ring.time), r_thumb.time.max(r_index.time).max(r_middle.time).max(r_ring.time)];
            }
        };

        if !result {
            continue;
        }

        if let Some(fms) = first_move_speed {
            speed = speed.max(fms) + 0.5;
            prev_speed = None;
            first_move_speed = None;
        }

        if (mv.starts_with('R') || mv.starts_with('l')) && grip == -1 {
            grip = 1;
            speed += 0.65;
        } else if (mv.starts_with('r') || mv.starts_with('L')) && grip == 1 {
            grip = -1;
            speed += 0.65;
        }

        if mv.starts_with('d') && udgrip == -1 {
            udgrip = 1;
            speed += 2.25;
        } else if (mv.starts_with('U') || mv.starts_with('u')) && udgrip == 1 {
            udgrip = -1;
            speed += 2.25;
        }

        if j >= 2 {
            let prev2 = &split_seq[j - 2];
            let prev1 = split_seq[j - 1].to_uppercase();
            if (normal_move == "R" && mv == prev2 && prev1 == "U'") || (normal_move == "R'" && mv == prev2 && prev1 == "U") {
                speed -= 0.5;
            } else if (normal_move == "R" && mv == prev2 && prev1 == "D'" && r_wrist == 1) || (normal_move == "R'" && mv == prev2 && prev1 == "D") {
                speed -= 0.3;
            }
        }

        if normal_move == "U" && (l_wrist == -1 || r_wrist == -1) {
            speed += params.destabilize;
        }
        if normal_move == "B" && (l_wrist == 0 || r_wrist == 0) {
            speed += params.destabilize;
        }
        if normal_move == "D" && (l_wrist == 1 || r_wrist == 1) {
            speed += params.destabilize;
        }
    }

    [-1.0, speed, l_grip as f64, r_grip as f64, 0.0, 0.0]
}
