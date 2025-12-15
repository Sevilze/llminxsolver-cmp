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
