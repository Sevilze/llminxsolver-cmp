use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CornerPosition {
    UC1 = 0,
    UC2 = 1,
    UC3 = 2,
    UC4 = 3,
    UC5 = 4,
    RC1 = 5,
    RC5 = 6,
    FC5 = 7,
    FC1 = 8,
    FC2 = 9,
    LC1 = 10,
    LC2 = 11,
    BLC1 = 12,
    BLC2 = 13,
    BRC1 = 14,
    DC1 = 15,
    DC2 = 16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EdgePosition {
    UE1 = 0,
    UE2 = 1,
    UE3 = 2,
    UE4 = 3,
    UE5 = 4,
    RE2 = 5,
    RE3 = 6,
    RE4 = 7,
    FE2 = 8,
    FE3 = 9,
    FE4 = 10,
    FE5 = 11,
    LE3 = 12,
    LE4 = 13,
    LE5 = 14,
    BLE3 = 15,
    BLE4 = 16,
    BLE5 = 17,
    BRE3 = 18,
    BRE4 = 19,
    DE3 = 20,
    DE4 = 21,
    DE5 = 22,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Orientation {
    Neutral = 0,
    Positive = 1,
    Negative = 2,
    Ignore = 3,
}

impl Orientation {
    #[inline]
    pub fn clockwise(self) -> Self {
        match self {
            Orientation::Neutral => Orientation::Positive,
            Orientation::Positive => Orientation::Negative,
            Orientation::Negative => Orientation::Neutral,
            Orientation::Ignore => Orientation::Ignore,
        }
    }

    #[inline]
    pub fn counter_clockwise(self) -> Self {
        match self {
            Orientation::Neutral => Orientation::Negative,
            Orientation::Positive => Orientation::Neutral,
            Orientation::Negative => Orientation::Positive,
            Orientation::Ignore => Orientation::Ignore,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_position_values() {
        assert_eq!(CornerPosition::UC1 as u8, 0);
        assert_eq!(CornerPosition::UC2 as u8, 1);
        assert_eq!(CornerPosition::UC5 as u8, 4);
        assert_eq!(CornerPosition::DC2 as u8, 16);
    }

    #[test]
    fn test_edge_position_values() {
        assert_eq!(EdgePosition::UE1 as u8, 0);
        assert_eq!(EdgePosition::UE5 as u8, 4);
        assert_eq!(EdgePosition::DE5 as u8, 22);
    }

    #[test]
    fn test_orientation_values() {
        assert_eq!(Orientation::Neutral as u8, 0);
        assert_eq!(Orientation::Positive as u8, 1);
        assert_eq!(Orientation::Negative as u8, 2);
        assert_eq!(Orientation::Ignore as u8, 3);
    }

    #[test]
    fn test_orientation_clockwise() {
        assert_eq!(Orientation::Neutral.clockwise(), Orientation::Positive);
        assert_eq!(Orientation::Positive.clockwise(), Orientation::Negative);
        assert_eq!(Orientation::Negative.clockwise(), Orientation::Neutral);
        assert_eq!(Orientation::Ignore.clockwise(), Orientation::Ignore);
    }

    #[test]
    fn test_orientation_counter_clockwise() {
        assert_eq!(
            Orientation::Neutral.counter_clockwise(),
            Orientation::Negative
        );
        assert_eq!(
            Orientation::Positive.counter_clockwise(),
            Orientation::Neutral
        );
        assert_eq!(
            Orientation::Negative.counter_clockwise(),
            Orientation::Positive
        );
        assert_eq!(Orientation::Ignore.counter_clockwise(), Orientation::Ignore);
    }

    #[test]
    fn test_orientation_clockwise_cycle() {
        let o = Orientation::Neutral;
        assert_eq!(o.clockwise().clockwise().clockwise(), Orientation::Neutral);
    }

    #[test]
    fn test_orientation_counter_clockwise_cycle() {
        let o = Orientation::Neutral;
        assert_eq!(
            o.counter_clockwise()
                .counter_clockwise()
                .counter_clockwise(),
            Orientation::Neutral
        );
    }

    #[test]
    fn test_corner_position_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(CornerPosition::UC1);
        set.insert(CornerPosition::UC1);
        assert_eq!(set.len(), 1);
        set.insert(CornerPosition::UC2);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_edge_position_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(EdgePosition::UE1);
        set.insert(EdgePosition::UE1);
        assert_eq!(set.len(), 1);
        set.insert(EdgePosition::UE2);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_orientation_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Orientation::Neutral);
        set.insert(Orientation::Neutral);
        assert_eq!(set.len(), 1);
        set.insert(Orientation::Positive);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_corner_position_debug() {
        let cp = CornerPosition::UC1;
        let debug_str = format!("{:?}", cp);
        assert_eq!(debug_str, "UC1");
    }

    #[test]
    fn test_edge_position_debug() {
        let ep = EdgePosition::UE1;
        let debug_str = format!("{:?}", ep);
        assert_eq!(debug_str, "UE1");
    }

    #[test]
    fn test_orientation_debug() {
        let o = Orientation::Neutral;
        let debug_str = format!("{:?}", o);
        assert_eq!(debug_str, "Neutral");
    }

    #[test]
    fn test_corner_position_serde() {
        let cp = CornerPosition::UC1;
        let serialized = serde_json::to_string(&cp).unwrap();
        let deserialized: CornerPosition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cp, deserialized);
    }

    #[test]
    fn test_edge_position_serde() {
        let ep = EdgePosition::UE1;
        let serialized = serde_json::to_string(&ep).unwrap();
        let deserialized: EdgePosition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ep, deserialized);
    }

    #[test]
    fn test_orientation_serde() {
        let o = Orientation::Positive;
        let serialized = serde_json::to_string(&o).unwrap();
        let deserialized: Orientation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(o, deserialized);
    }
}
