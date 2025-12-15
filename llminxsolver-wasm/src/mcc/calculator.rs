use super::finger_sim::test;
use super::parser::process_alg;
use super::types::MCCParams;

pub fn calculate_mcc(sequence: &str) -> f64 {
    calculate_mcc_with_params(sequence, &MCCParams::default())
}

pub fn calculate_mcc_with_params(sequence: &str, params: &MCCParams) -> f64 {
    let mut split_seq = process_alg(sequence, true);

    if split_seq.is_empty() {
        return 0.0;
    }

    let mut tests: Vec<[f64; 6]> = vec![
        test(&split_seq, 0, 0, 0.0, params),
        test(&split_seq, 0, -1, 1.0 + params.add_regrip, params),
        test(&split_seq, 0, 1, 1.0 + params.add_regrip, params),
        test(&split_seq, -1, 0, 1.0 + params.add_regrip, params),
        test(&split_seq, 1, 0, 1.0 + params.add_regrip, params),
    ];

    loop {
        let mut best_test = tests[0];
        for t in tests.iter().skip(1) {
            let prefer_t = (t[0] == -1.0 && (best_test[0] != -1.0 || best_test[1] > t[1]))
                || (t[0] > best_test[0] && best_test[0] != -1.0)
                || (t[0] == best_test[0] && t[1] < best_test[1] && best_test[0] != -1.0);
            if prefer_t {
                best_test = *t;
            }
        }

        if best_test[0] == -1.0 {
            return (best_test[1] * 10.0).round() / 10.0;
        }

        let idx = best_test[0] as usize;
        tests.clear();

        let prev_move_type = if idx >= 1 {
            split_seq[idx - 1].chars().next().unwrap_or(' ')
        } else {
            ' '
        };
        let prev2_type = if idx >= 2 {
            split_seq[idx - 2].chars().next().unwrap_or(' ')
        } else {
            ' '
        };

        let double_regrip = (best_test[2] > 1.0 || best_test[2] < -1.0)
            && (best_test[3] > 1.0 || best_test[3] < -1.0);

        for left_wrist in -1..=1 {
            for right_wrist in -1..=1 {
                let left_match = best_test[2] == left_wrist as f64;
                let right_match = best_test[3] == right_wrist as f64;

                if ['X', 'x', 'Y', 'y', 'Z', 'z'].contains(&prev_move_type) {
                    tests.push(test(
                        &split_seq[idx..],
                        left_wrist,
                        right_wrist,
                        best_test[1],
                        params,
                    ));
                } else {
                    let r_move_latency = if prev_move_type == 'R'
                        || prev2_type == 'R'
                        || prev_move_type == 'r'
                        || prev2_type == 'r'
                    {
                        1.0
                    } else {
                        0.0
                    };
                    let l_move_latency = if prev_move_type == 'L'
                        || prev2_type == 'L'
                        || prev_move_type == 'l'
                        || prev2_type == 'l'
                    {
                        1.0
                    } else {
                        0.0
                    };

                    if left_match || double_regrip {
                        let r_hand_latency = (2.0 - (best_test[1] - best_test[5])).max(0.0);
                        let penalty = r_hand_latency.max(r_move_latency).max(l_move_latency * 2.0);
                        tests.push(test(
                            &split_seq[idx..],
                            left_wrist,
                            right_wrist,
                            best_test[1] + penalty + params.add_regrip,
                            params,
                        ));
                    } else if right_match {
                        let l_hand_latency = (2.0 - (best_test[1] - best_test[4])).max(0.0);
                        let penalty = l_hand_latency.max(l_move_latency).max(r_move_latency * 2.0);
                        tests.push(test(
                            &split_seq[idx..],
                            left_wrist,
                            right_wrist,
                            best_test[1] + penalty + params.add_regrip,
                            params,
                        ));
                    }
                }
            }
        }

        split_seq = split_seq[idx..].to_vec();

        if tests.is_empty() {
            return f64::NAN;
        }
    }
}
