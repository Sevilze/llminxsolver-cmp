use super::types::VALID_MOVES;

pub fn replace_double(input: &str) -> String {
    let segs: Vec<&str> = input.split(' ').collect();
    let mut result: Vec<String> = Vec::with_capacity(segs.len());

    for seg in &segs {
        if !result.is_empty() {
            let last = result.last().unwrap();
            if last == *seg && (seg.len() == 1 || (seg.len() == 2 && seg.ends_with('\''))) {
                let base = seg.chars().next().unwrap();
                result.pop();
                result.push(format!("{}2", base));
                continue;
            }
        }
        result.push(seg.to_string());
    }

    result.join(" ")
}

pub fn is_valid_move(m: &str) -> bool {
    VALID_MOVES.contains(&m.to_lowercase().as_str())
}

pub fn process_alg(input: &str, ignore_auf: bool) -> Vec<String> {
    let alg = replace_double(input).replace("2'", "2");
    let mut split_seq: Vec<String> = alg
        .split(' ')
        .filter(|s| !s.is_empty() && is_valid_move(s))
        .map(|s| s.to_string())
        .collect();

    if ignore_auf {
        if !split_seq.is_empty() && split_seq[0].starts_with('U') {
            split_seq.remove(0);
        } else if split_seq.len() >= 2 {
            let first_lower = split_seq[0].to_lowercase();
            if first_lower.starts_with('d') && split_seq[1].starts_with('U') {
                let temp = split_seq[0].clone();
                split_seq[1] = temp;
                split_seq.remove(0);
            }
        }

        if !split_seq.is_empty() && split_seq.last().unwrap().starts_with('U') {
            split_seq.pop();
        } else if split_seq.len() >= 2 {
            let last = split_seq.last().unwrap().to_lowercase();
            let second_last = &split_seq[split_seq.len() - 2];
            if last.starts_with('d') && second_last.starts_with('U') {
                let temp = split_seq.last().unwrap().clone();
                let len = split_seq.len();
                split_seq[len - 2] = temp;
                split_seq.pop();
            }
        }
    }

    split_seq
}

pub fn get_move_count(algorithm: &str, metric: &str) -> u32 {
    let split_seq = process_alg(algorithm, true);

    if metric == "FTM" {
        return split_seq.len() as u32;
    }

    let mut count = 0u32;
    for m in &split_seq {
        if m.ends_with('2') {
            count += 2;
        } else {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_double_single_moves() {
        assert_eq!(replace_double("R R"), "R2");
        assert_eq!(replace_double("U U"), "U2");
        assert_eq!(replace_double("F F"), "F2");
    }

    #[test]
    fn test_replace_double_prime_moves() {
        assert_eq!(replace_double("R' R'"), "R2");
        assert_eq!(replace_double("U' U'"), "U2");
        assert_eq!(replace_double("F' F'"), "F2");
    }

    #[test]
    fn test_replace_double_mixed_sequence() {
        assert_eq!(replace_double("R U R U"), "R U R U");
        assert_eq!(replace_double("R R U U"), "R2 U2");
    }

    #[test]
    fn test_replace_double_no_double() {
        assert_eq!(replace_double("R U F"), "R U F");
        assert_eq!(replace_double("R U R'"), "R U R'");
    }

    #[test]
    fn test_replace_double_empty() {
        assert_eq!(replace_double(""), "");
    }

    #[test]
    fn test_replace_double_single() {
        assert_eq!(replace_double("R"), "R");
    }

    #[test]
    fn test_is_valid_move_valid() {
        assert!(is_valid_move("u"));
        assert!(is_valid_move("U"));
        assert!(is_valid_move("r"));
        assert!(is_valid_move("R"));
        assert!(is_valid_move("u'"));
        assert!(is_valid_move("U'"));
        assert!(is_valid_move("u2"));
        assert!(is_valid_move("U2"));
        assert!(is_valid_move("bl"));
        assert!(is_valid_move("bl'"));
        assert!(is_valid_move("bl2"));
    }

    #[test]
    fn test_is_valid_move_invalid() {
        assert!(!is_valid_move("invalid"));
        assert!(!is_valid_move("xyz"));
        assert!(!is_valid_move(""));
    }

    #[test]
    fn test_process_alg_ignore_auf_d_start() {
        let result = process_alg("d U R U F", true);
        assert_eq!(result, vec!["d", "R", "U", "F"]);
    }

    #[test]
    fn test_process_alg_ignore_auf_d_end() {
        let result = process_alg("R U F d U", true);
        assert_eq!(result, vec!["R", "U", "F", "d"]);
    }

    #[test]
    fn test_process_alg_filters_invalid() {
        let result = process_alg("R invalid U", false);
        assert_eq!(result, vec!["R", "U"]);
    }

    #[test]
    fn test_process_alg_empty() {
        let result = process_alg("", false);
        assert!(result.is_empty());
    }

    #[test]
    fn test_process_alg_replace_double() {
        let result = process_alg("R R U", false);
        assert_eq!(result, vec!["R2", "U"]);
    }

    #[test]
    fn test_process_alg_basic() {
        let result = process_alg("R U F", false);
        assert_eq!(result, vec!["R", "U", "F"]);
    }

    #[test]
    fn test_process_alg_ignore_auf_start_u() {
        let result = process_alg("U R U F", true);
        assert_eq!(result, vec!["R", "U", "F"]);
    }

    #[test]
    fn test_process_alg_ignore_auf_end_u() {
        let result = process_alg("R U F U'", true);
        assert_eq!(result, vec!["R", "U", "F"]);
    }

    #[test]
    fn test_process_alg_ignore_auf_both() {
        let result = process_alg("U R U F U'", true);
        assert_eq!(result, vec!["R", "U", "F"]);
    }

    #[test]
    fn test_process_alg_double_prime() {
        let result = process_alg("R2' U F", false);
        assert_eq!(result, vec!["R2", "U", "F"]);
    }

    #[test]
    fn test_process_alg_d_at_start_with_u() {
        let result = process_alg("d U2 R F", true);
        assert_eq!(result, vec!["d", "R", "F"]);
    }

    #[test]
    fn test_get_move_count_ftm() {
        assert_eq!(get_move_count("R U F", "FTM"), 3);
        assert_eq!(get_move_count("R2 U F", "FTM"), 3);
        assert_eq!(get_move_count("", "FTM"), 0);
    }

    #[test]
    fn test_get_move_count_qtm() {
        assert_eq!(get_move_count("R U F", "QTM"), 3);
        assert_eq!(get_move_count("R2 U F", "QTM"), 4);
        assert_eq!(get_move_count("R2 U2 F2", "QTM"), 6);
    }

    #[test]
    fn test_get_move_count_with_auf() {
        assert_eq!(get_move_count("U R U F U'", "FTM"), 3);
        assert_eq!(get_move_count("U R2 U F U'", "QTM"), 4);
    }
}
