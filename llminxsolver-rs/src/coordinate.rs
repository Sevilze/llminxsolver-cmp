use std::sync::LazyLock;

pub static POWERS_OF_TWO: LazyLock<[u32; 35]> = LazyLock::new(|| {
    let mut arr = [0u32; 35];
    for (i, val) in arr.iter_mut().enumerate() {
        *val = 1 << i;
    }
    arr
});

pub static POWERS_OF_TWO_64: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut arr = [0u64; 64];
    for (i, val) in arr.iter_mut().enumerate() {
        *val = 1 << i;
    }
    arr
});

pub static POWERS_OF_TWO_SUBT_ONE: LazyLock<[u32; 35]> = LazyLock::new(|| {
    let mut arr = [0u32; 35];
    for (i, val) in arr.iter_mut().enumerate() {
        *val = (1 << i) - 1;
    }
    arr
});

pub static POWERS_OF_THREE: LazyLock<[u32; 20]> = LazyLock::new(|| {
    let mut arr = [0u32; 20];
    arr[0] = 1;
    for i in 1..20 {
        arr[i] = arr[i - 1] * 3;
    }
    arr
});

pub static FAC: LazyLock<[u32; 13]> = LazyLock::new(|| {
    let mut arr = [0u32; 13];
    arr[0] = 1;
    for i in 1..13 {
        arr[i] = arr[i - 1] * (i as u32);
    }
    arr
});

pub static CKN: LazyLock<[[u32; 9]; 24]> = LazyLock::new(|| {
    let mut arr = [[0u32; 9]; 24];
    for (i, row) in arr.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            *val = binomial(i as u32, j as u32) as u32;
        }
    }
    arr
});

pub struct CoordinateUtil;

impl CoordinateUtil {
    pub fn get_permutation_coordinate(permutation: &[u8], cubies: &[u8]) -> u32 {
        let mut coordinate = 0u32;
        let mut locations: Vec<u8> = cubies.to_vec();
        let cubie_count = cubies.len().saturating_sub(2);

        for (i, &piece) in cubies.iter().take(cubie_count).enumerate() {
            for j in 0..(locations.len() - i) {
                if permutation[locations[j] as usize] == piece {
                    coordinate *= (locations.len() - i) as u32;
                    coordinate += j as u32;
                    for k in j..(locations.len() - i - 1) {
                        locations[k] = locations[k + 1];
                    }
                    break;
                }
            }
        }
        coordinate
    }

    pub fn get_permutation(coordinate: u32, permutation: &mut [u8], cubies: &[u8]) {
        let mut coord = coordinate;
        let mut indices = vec![0usize; cubies.len()];
        let mut locations: Vec<u8> = cubies.to_vec();

        let mut factor = 3usize;
        let mut sum = 0usize;

        for i in (0..=(cubies.len().saturating_sub(3))).rev() {
            indices[i] = (coord as usize) % factor;
            sum += indices[i];
            coord /= factor as u32;
            factor += 1;
        }

        if cubies.len() >= 2 {
            indices[cubies.len() - 2] = sum % 2;
        }
        if !cubies.is_empty() {
            indices[cubies.len() - 1] = 0;
        }

        for i in 0..cubies.len() {
            let index = indices[i];
            permutation[locations[index] as usize] = cubies[i];
            for k in index..(locations.len() - i - 1) {
                locations[k] = locations[k + 1];
            }
        }
    }

    pub fn get_separation_coordinate(permutation: &[u8], cubies: &[u8]) -> u32 {
        let mut coordinate = 0u32;
        let mut count = 1usize;

        for (i, &position) in permutation.iter().enumerate() {
            if cubies.contains(&position) {
                coordinate += CKN[i][count];
                count += 1;
            }
        }
        coordinate
    }

    pub fn get_separation(coordinate: u32, permutation: &mut [u8], cubies: &[u8]) {
        permutation.fill(u8::MAX);
        let mut coord = coordinate;

        for count in (1..=cubies.len()).rev() {
            let mut position_left = permutation.len() - 1;
            while coord < CKN[position_left][count] {
                if position_left == 0 {
                    break;
                }
                position_left -= 1;
            }
            permutation[position_left] = cubies[count - 1];
            coord -= CKN[position_left][count];
        }
    }

    #[inline]
    pub fn get_edge_orientation_coordinate(edge_orientation: u32, edge_count: usize) -> u32 {
        POWERS_OF_TWO_SUBT_ONE[edge_count - 1] & edge_orientation
    }

    #[inline]
    pub fn get_edge_orientation(edge_coordinate: u32, edge_count: usize) -> u32 {
        let coord = POWERS_OF_TWO_SUBT_ONE[edge_count - 1] & edge_coordinate;
        coord | ((Self::get_parity(coord) as u32) << (edge_count - 1))
    }

    pub fn get_corner_orientation_coordinate(corner_orientation: u64, cubies: &[u8]) -> u32 {
        let mut coordinate = 0u32;
        for i in 0..(cubies.len().saturating_sub(1)) {
            coordinate += POWERS_OF_THREE[i] * ((corner_orientation >> (cubies[i] * 2)) & 3) as u32;
        }
        coordinate
    }

    pub fn get_corner_orientation(corner_coordinate: u32, cubies: &[u8]) -> u64 {
        let mut orientation = 0u64;
        let mut sum_orientation = 0u64;
        let mut coord = corner_coordinate;

        let len = cubies.len();
        for i in 0..(len.saturating_sub(1)) {
            let cubie_orientation = (coord % 3) as u64;
            sum_orientation += cubie_orientation;
            orientation += POWERS_OF_TWO_64[cubies[i] as usize * 2] * cubie_orientation;
            coord /= 3;
        }

        if len > 0 {
            orientation +=
                POWERS_OF_TWO_64[cubies[len - 1] as usize * 2] * ((3 - sum_orientation % 3) % 3);
        }
        orientation
    }

    #[inline]
    pub fn get_parity(value: u32) -> u8 {
        let mut x = value;
        x ^= x >> 16;
        x ^= x >> 8;
        x ^= x >> 4;
        x ^= x >> 2;
        x ^= x >> 1;
        (x & 1) as u8
    }
}

fn binomial(n: u32, k: u32) -> u64 {
    if n < k {
        return 0;
    }
    let mut result = 1u64;
    for i in (k + 1)..=n {
        result *= i as u64;
    }
    for i in 1..=(n - k) {
        result /= i as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powers() {
        assert_eq!(POWERS_OF_TWO[0], 1);
        assert_eq!(POWERS_OF_TWO[1], 2);
        assert_eq!(POWERS_OF_TWO[10], 1024);

        assert_eq!(POWERS_OF_THREE[0], 1);
        assert_eq!(POWERS_OF_THREE[1], 3);
        assert_eq!(POWERS_OF_THREE[5], 243);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(FAC[0], 1);
        assert_eq!(FAC[1], 1);
        assert_eq!(FAC[5], 120);
        assert_eq!(FAC[10], 3628800);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(10, 3), 120);
        assert_eq!(binomial(3, 5), 0);
    }

    #[test]
    fn test_parity() {
        assert_eq!(CoordinateUtil::get_parity(0), 0);
        assert_eq!(CoordinateUtil::get_parity(1), 1);
        assert_eq!(CoordinateUtil::get_parity(3), 0);
        assert_eq!(CoordinateUtil::get_parity(7), 1);
    }
}
