use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Move {
    R = 0,
    Ri = 1,
    R2 = 2,
    R2i = 3,
    L = 4,
    Li = 5,
    L2 = 6,
    L2i = 7,
    U = 8,
    Ui = 9,
    U2 = 10,
    U2i = 11,
    F = 12,
    Fi = 13,
    F2 = 14,
    F2i = 15,
    B = 16,
    Bi = 17,
    B2 = 18,
    B2i = 19,
}

impl Move {
    pub const ALL: [Move; 20] = [
        Move::R,
        Move::Ri,
        Move::R2,
        Move::R2i,
        Move::L,
        Move::Li,
        Move::L2,
        Move::L2i,
        Move::U,
        Move::Ui,
        Move::U2,
        Move::U2i,
        Move::F,
        Move::Fi,
        Move::F2,
        Move::F2i,
        Move::B,
        Move::Bi,
        Move::B2,
        Move::B2i,
    ];

    pub const INVERSE: [Move; 20] = [
        Move::Ri,
        Move::R,
        Move::R2i,
        Move::R2,
        Move::Li,
        Move::L,
        Move::L2i,
        Move::L2,
        Move::Ui,
        Move::U,
        Move::U2i,
        Move::U2,
        Move::Fi,
        Move::F,
        Move::F2i,
        Move::F2,
        Move::Bi,
        Move::B,
        Move::B2i,
        Move::B2,
    ];

    pub const STRINGS: [&'static str; 20] = [
        "R   ", "R'  ", "R2  ", "R2' ", "L   ", "L'  ", "L2  ", "L2' ", "U   ", "U'  ", "U2  ",
        "U2' ", "F   ", "F'  ", "F2  ", "F2' ", "B   ", "B'  ", "B2  ", "B2' ",
    ];

    #[inline]
    pub fn inverse(self) -> Move {
        Move::INVERSE[self as usize]
    }

    #[inline]
    pub fn face(self) -> u8 {
        (self as u8) / 4
    }

    #[inline]
    pub fn is_double(self) -> bool {
        (self as u8) % 4 >= 2
    }

    pub fn to_string(self) -> &'static str {
        Move::STRINGS[self as usize]
    }

    pub fn from_u8(v: u8) -> Option<Move> {
        if v < 20 {
            Some(Move::ALL[v as usize])
        } else {
            None
        }
    }
}
