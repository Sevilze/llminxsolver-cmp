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
