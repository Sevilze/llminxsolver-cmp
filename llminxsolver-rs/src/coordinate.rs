use std::sync::LazyLock;

pub static POWERS_OF_TWO: LazyLock<[u32; 32]> = LazyLock::new(|| {
    let mut arr = [0u32; 32];
    for (i, val) in arr.iter_mut().enumerate() {
        *val = 1u32 << i;
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

pub static POWERS_OF_TWO_SUBT_ONE: LazyLock<[u32; 32]> = LazyLock::new(|| {
    let mut arr = [0u32; 32];
    for (i, val) in arr.iter_mut().enumerate() {
        *val = (1u32 << i) - 1;
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

    let k = k.min(n - k);
    if k == 0 {
        return 1;
    }

    let mut result: u128 = 1;
    for i in 0..k {
        result = result * (n - i) as u128 / (i + 1) as u128;
    }

    result as u64
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

    #[test]
    fn test_powers_of_two_64() {
        assert_eq!(POWERS_OF_TWO_64[0], 1);
        assert_eq!(POWERS_OF_TWO_64[1], 2);
        assert_eq!(POWERS_OF_TWO_64[63], 1u64 << 63);
    }

    #[test]
    fn test_powers_of_two_subt_one() {
        assert_eq!(POWERS_OF_TWO_SUBT_ONE[0], 0);
        assert_eq!(POWERS_OF_TWO_SUBT_ONE[1], 1);
        assert_eq!(POWERS_OF_TWO_SUBT_ONE[4], 15);
    }

    #[test]
    fn test_get_permutation_coordinate() {
        let permutation = [0u8, 1, 2, 3, 4];
        let cubies = [0u8, 1, 2, 3, 4];
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);
        assert_eq!(coord, 0);
    }

    #[test]
    fn test_get_permutation_roundtrip() {
        let cubies = [0u8, 1, 2, 3, 4];
        let mut permutation = [0u8; 5];
        for (i, cubie) in cubies.iter().enumerate() {
            permutation[i] = *cubie;
        }
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);

        let mut result = [0u8; 5];
        CoordinateUtil::get_permutation(coord, &mut result, &cubies);
        assert_eq!(result, permutation);
    }

    #[test]
    fn test_edge_orientation_coordinate() {
        let edge_ori = 0b1010u32;
        let coord = CoordinateUtil::get_edge_orientation_coordinate(edge_ori, 5);
        assert_eq!(coord, edge_ori & 0b1111);
    }

    #[test]
    fn test_get_edge_orientation() {
        let coord = 0b1010u32;
        let ori = CoordinateUtil::get_edge_orientation(coord, 5);
        assert_eq!(ori & 0b1111, coord & 0b1111);
    }

    #[test]
    fn test_corner_orientation_coordinate() {
        let corner_ori = 0u64;
        let cubies = [0u8, 1, 2, 3, 4];
        let coord = CoordinateUtil::get_corner_orientation_coordinate(corner_ori, &cubies);
        assert_eq!(coord, 0);
    }

    #[test]
    fn test_get_corner_orientation() {
        let coord = 0u32;
        let cubies = [0u8, 1, 2, 3, 4];
        let ori = CoordinateUtil::get_corner_orientation(coord, &cubies);
        assert_eq!(ori, 0);
    }

    #[test]
    fn test_parity_all_zeros() {
        assert_eq!(CoordinateUtil::get_parity(0), 0);
    }

    #[test]
    fn test_parity_all_ones() {
        assert_eq!(CoordinateUtil::get_parity(0xFFFFFFFF), 0);
    }

    #[test]
    fn test_permutation_with_swap() {
        let permutation = [1u8, 0, 2, 3, 4];
        let cubies = [0u8, 1, 2, 3, 4];
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);
        assert!(coord > 0);
    }

    #[test]
    fn test_binomial_edge_cases() {
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(1, 0), 1);
        assert_eq!(binomial(1, 1), 1);
    }

    #[test]
    fn test_get_permutation_with_different_coord() {
        let cubies = [0u8, 1, 2, 3, 4];
        let mut result = [0u8; 5];
        CoordinateUtil::get_permutation(1, &mut result, &cubies);
        assert!(result != [0u8, 1, 2, 3, 4]);
    }

    #[test]
    fn test_corner_orientation_nonzero() {
        let cubies = [0u8, 1, 2, 3, 4];
        let coord = 1u32;
        let ori = CoordinateUtil::get_corner_orientation(coord, &cubies);
        assert!(ori > 0);
    }

    #[test]
    fn test_powers_of_three_values() {
        assert_eq!(POWERS_OF_THREE[2], 9);
        assert_eq!(POWERS_OF_THREE[3], 27);
        assert_eq!(POWERS_OF_THREE[4], 81);
    }

    #[test]
    fn test_edge_orientation_with_parity() {
        let coord = 0b1101u32;
        let ori = CoordinateUtil::get_edge_orientation(coord, 5);
        let parity = CoordinateUtil::get_parity(coord & 0b1111);
        let expected_bit = parity as u32;
        assert_eq!((ori >> 4) & 1, expected_bit);
    }

    #[test]
    fn test_corner_orientation_roundtrip() {
        let cubies = [0u8, 1, 2, 3, 4];
        for coord in 0..10u32 {
            let ori = CoordinateUtil::get_corner_orientation(coord, &cubies);
            let back_coord = CoordinateUtil::get_corner_orientation_coordinate(ori, &cubies);
            assert_eq!(back_coord, coord, "Failed for coord {}", coord);
        }
    }

    #[test]
    fn test_permutation_with_larger_set() {
        let cubies = [0u8, 1, 2, 3, 4, 5, 6];
        let mut permutation = [0u8; 7];
        for (i, cubie) in cubies.iter().enumerate() {
            permutation[i] = *cubie;
        }
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);

        let mut result = [0u8; 7];
        CoordinateUtil::get_permutation(coord, &mut result, &cubies);
        assert_eq!(result, permutation);
    }

    #[test]
    fn test_permutation_with_small_set() {
        let cubies = [0u8, 1];
        let permutation = [0u8, 1];
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);
        assert_eq!(coord, 0);
    }

    #[test]
    fn test_single_cubie() {
        let cubies = [0u8];
        let permutation = [0u8];
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);
        assert_eq!(coord, 0);
    }

    #[test]
    fn test_powers_of_two_all_indices() {
        for i in 0..32 {
            assert_eq!(POWERS_OF_TWO[i], 1u32 << i, "Failed at index {}", i);
        }
    }

    #[test]
    fn test_powers_of_two_64_all_indices() {
        for i in 0..64 {
            assert_eq!(POWERS_OF_TWO_64[i], 1u64 << i, "Failed at index {}", i);
        }
    }

    #[test]
    fn test_powers_of_two_subt_one_all_indices() {
        for i in 0..32 {
            assert_eq!(
                POWERS_OF_TWO_SUBT_ONE[i],
                (1u32 << i).wrapping_sub(1),
                "Failed at index {}",
                i
            );
        }
    }

    #[test]
    fn test_powers_of_three_all_indices() {
        let mut expected = 1u32;
        for i in 0..20 {
            assert_eq!(POWERS_OF_THREE[i], expected, "Failed at index {}", i);
            expected *= 3;
        }
    }

    #[test]
    fn test_factorial_all_indices() {
        let expected_factorials = [
            1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
        ];
        for (i, &expected) in expected_factorials.iter().enumerate() {
            assert_eq!(FAC[i], expected, "Factorial {} failed", i);
        }
    }

    #[test]
    fn test_edge_orientation_coordinate_various() {
        for edge_count in 2..8 {
            let max_coord = (1u32 << (edge_count - 1)) - 1;
            for coord in 0..=max_coord {
                let result = CoordinateUtil::get_edge_orientation_coordinate(coord, edge_count);
                assert!(result <= max_coord);
            }
        }
    }

    #[test]
    fn test_get_edge_orientation_various() {
        for edge_count in 2..8 {
            let max_coord = (1u32 << (edge_count - 1)) - 1;
            for coord in 0..=max_coord.min(15) {
                let ori = CoordinateUtil::get_edge_orientation(coord, edge_count);
                let parity =
                    CoordinateUtil::get_parity(coord & POWERS_OF_TWO_SUBT_ONE[edge_count - 1]);
                let computed_last_bit = (ori >> (edge_count - 1)) & 1;
                assert_eq!(computed_last_bit, parity as u32);
            }
        }
    }

    #[test]
    fn test_corner_orientation_coordinate_various_sizes() {
        for num_cubies in 2..6 {
            let cubies: Vec<u8> = (0..num_cubies).collect();
            for coord in 0..POWERS_OF_THREE[num_cubies as usize - 1].min(20) {
                let ori = CoordinateUtil::get_corner_orientation(coord, &cubies);
                let back = CoordinateUtil::get_corner_orientation_coordinate(ori, &cubies);
                assert_eq!(
                    back, coord,
                    "Failed for {} cubies at coord {}",
                    num_cubies, coord
                );
            }
        }
    }

    #[test]
    fn test_corner_orientation_empty_cubies() {
        let cubies: Vec<u8> = vec![];
        let ori = CoordinateUtil::get_corner_orientation(0, &cubies);
        assert_eq!(ori, 0);
        let coord = CoordinateUtil::get_corner_orientation_coordinate(0, &cubies);
        assert_eq!(coord, 0);
    }

    #[test]
    fn test_corner_orientation_single_cubie() {
        let cubies = [5u8];
        let ori = CoordinateUtil::get_corner_orientation(0, &cubies);
        assert_eq!(ori, 0);
    }

    #[test]
    fn test_get_permutation_empty_cubies() {
        let cubies: Vec<u8> = vec![];
        let permutation: Vec<u8> = vec![];
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);
        assert_eq!(coord, 0);
    }

    #[test]
    fn test_get_permutation_reversed() {
        let cubies = [0u8, 1, 2, 3, 4];
        let permutation = [4u8, 3, 2, 1, 0];
        let coord = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);
        assert!(coord > 0);

        let mut result = [0u8; 5];
        CoordinateUtil::get_permutation(coord, &mut result, &cubies);
        assert_eq!(result, permutation);
    }

    #[test]
    fn test_parity_various_values() {
        let test_cases = [
            (0b0, 0),
            (0b1, 1),
            (0b11, 0),
            (0b111, 1),
            (0b1111, 0),
            (0b10101, 1),
            (0b101010, 1),
            (0xAAAAAAAA, 0),
            (0x55555555, 0),
        ];
        for (value, expected) in test_cases {
            assert_eq!(
                CoordinateUtil::get_parity(value),
                expected,
                "Parity failed for 0x{:X}",
                value
            );
        }
    }

    #[test]
    fn test_binomial_symmetry() {
        for n in 0..15u32 {
            for k in 0..=n {
                assert_eq!(
                    binomial(n, k),
                    binomial(n, n - k),
                    "Symmetry failed for C({},{})",
                    n,
                    k
                );
            }
        }
    }

    #[test]
    fn test_binomial_pascals_triangle() {
        for n in 1..10u32 {
            for k in 1..n {
                assert_eq!(
                    binomial(n, k),
                    binomial(n - 1, k - 1) + binomial(n - 1, k),
                    "Pascal's identity failed for C({},{})",
                    n,
                    k
                );
            }
        }
    }

    #[test]
    fn test_permutation_all_coordinates() {
        let cubies = [0u8, 1, 2, 3];
        let max_coord = FAC[4] / 2;
        for coord in 0..max_coord {
            let mut permutation = [0u8; 4];
            CoordinateUtil::get_permutation(coord, &mut permutation, &cubies);
            let back = CoordinateUtil::get_permutation_coordinate(&permutation, &cubies);
            assert_eq!(back, coord, "Failed at coord {}", coord);
        }
    }

    #[test]
    fn test_separation_coordinate_and_roundtrip() {
        let permutation = [5u8, 0, 4, 1, 3, 2];
        let cubies = [0u8, 2, 4];

        let coord = CoordinateUtil::get_separation_coordinate(&permutation, &cubies);
        assert!(coord > 0);

        let mut reconstructed = [0u8; 6];
        CoordinateUtil::get_separation(coord, &mut reconstructed, &cubies);

        for cubie in cubies {
            assert!(reconstructed.contains(&cubie));
        }
    }

    #[test]
    fn test_separation_zero_coordinate_places_cubies() {
        let cubies = [1u8, 3, 5];
        let mut permutation = [0u8; 6];
        CoordinateUtil::get_separation(0, &mut permutation, &cubies);

        let present_count = permutation.iter().filter(|&&v| cubies.contains(&v)).count();
        assert_eq!(present_count, cubies.len());
    }
}
