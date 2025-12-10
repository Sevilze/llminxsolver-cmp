use crate::minx::{CornerPosition, EdgePosition, Move};
use crate::pruner::{
    CompositePruner, CornerOrientationPruner, CornerPermutationPruner, EdgeOrientationPruner,
    EdgePermutationPruner, Pruner, SeparationPruner,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Metric {
    Fifth,
    Face,
}

impl Metric {
    pub fn description(&self) -> &'static str {
        match self {
            Metric::Fifth => "Fifth turn metric",
            Metric::Face => "Face turn metric",
        }
    }
}

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SearchMode {
    RU,
    RUF,
    RUL,
    RUFL,
    RUFLB,
}

impl SearchMode {
    pub fn description(&self) -> &'static str {
        match self {
            SearchMode::RU => "RU",
            SearchMode::RUF => "RUF",
            SearchMode::RUL => "RUL",
            SearchMode::RUFL => "RUFL",
            SearchMode::RUFLB => "RUFLB",
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        match self {
            SearchMode::RU => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
            ],
            SearchMode::RUF => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
                Move::F,
                Move::Fi,
                Move::F2,
                Move::F2i,
            ],
            SearchMode::RUL => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
                Move::L,
                Move::Li,
                Move::L2,
                Move::L2i,
            ],
            SearchMode::RUFL => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
                Move::F,
                Move::Fi,
                Move::F2,
                Move::F2i,
                Move::L,
                Move::Li,
                Move::L2,
                Move::L2i,
            ],
            SearchMode::RUFLB => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
                Move::F,
                Move::Fi,
                Move::F2,
                Move::F2i,
                Move::L,
                Move::Li,
                Move::L2,
                Move::L2i,
                Move::B,
                Move::Bi,
                Move::B2,
                Move::B2i,
            ],
        }
    }

    pub fn create_pruners(&self) -> Vec<Box<dyn Pruner>> {
        use CornerPosition::*;
        use EdgePosition::*;

        match self {
            SearchMode::RU => vec![
                Box::new(EdgePermutationPruner::new(
                    "Edge permutations RU",
                    "ruedgepermutations",
                    &[
                        UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                        RE3 as u8, RE4 as u8, FE2 as u8,
                    ],
                )),
                Box::new(CompositePruner::new(
                    "Corners RU",
                    "rucorners",
                    Box::new(CornerPermutationPruner::new(
                        "Corner permutations RU",
                        "rucornerpermutations",
                        &[
                            UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                            RC5 as u8, FC5 as u8,
                        ],
                    )),
                    Box::new(CornerOrientationPruner::new(
                        "Corner orientations RU",
                        "rucornerorientations",
                        &[
                            UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                            RC5 as u8, FC5 as u8,
                        ],
                    )),
                )),
            ],
            SearchMode::RUF => vec![
                Box::new(CornerPermutationPruner::new(
                    "Corner permutations RUF",
                    "rufcornerpermutations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, FC1 as u8, FC2 as u8,
                    ],
                )),
                Box::new(EdgePermutationPruner::new(
                    "Edge permutations RUF",
                    "rufedgepermutations",
                    &[
                        UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                        RE3 as u8, RE4 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8,
                    ],
                )),
                Box::new(CompositePruner::new(
                    "Orientations RUF",
                    "ruforientations",
                    Box::new(CornerOrientationPruner::new(
                        "Corner orientations RUF",
                        "rufcornerorientations",
                        &[
                            UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                            RC5 as u8, FC5 as u8, FC1 as u8, FC2 as u8,
                        ],
                    )),
                    Box::new(EdgeOrientationPruner::new(
                        "Edge orientations RUF",
                        "rufedgeorientations",
                        &[
                            UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                            RE3 as u8, RE4 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8,
                        ],
                    )),
                )),
            ],
            SearchMode::RUL => vec![
                Box::new(CornerOrientationPruner::new(
                    "Corner orientations RUL",
                    "rulcornerorientations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, FC2 as u8, LC1 as u8, LC2 as u8,
                    ],
                )),
                Box::new(CornerPermutationPruner::new(
                    "Corner permutations RUL",
                    "rulcornerpermutations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, FC2 as u8, LC1 as u8, LC2 as u8,
                    ],
                )),
            ],
            SearchMode::RUFL => vec![
                Box::new(EdgeOrientationPruner::new(
                    "Edge orientations RUFL",
                    "rufledgeorientations",
                    &[
                        UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                        RE3 as u8, RE4 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8,
                        LE3 as u8, LE4 as u8, LE5 as u8,
                    ],
                )),
                Box::new(CornerOrientationPruner::new(
                    "Corner orientations RUFL",
                    "ruflcornerorientations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, FC1 as u8, FC2 as u8, LC1 as u8, LC2 as u8,
                    ],
                )),
                Box::new(CornerPermutationPruner::new(
                    "Corner permutations RUFL",
                    "ruflcornerpermutations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, FC1 as u8, FC2 as u8, LC1 as u8, LC2 as u8,
                    ],
                )),
            ],
            SearchMode::RUFLB => vec![
                Box::new(EdgeOrientationPruner::new(
                    "Edge orientations RUFLB",
                    "ruflbedgeorientations",
                    &[
                        UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                        RE3 as u8, RE4 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8,
                        LE3 as u8, LE4 as u8, LE5 as u8, BE3 as u8, BE4 as u8, BE5 as u8,
                    ],
                )),
                Box::new(CornerOrientationPruner::new(
                    "Corner orientations RUFLB",
                    "ruflbcornerorientations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, FC1 as u8, FC2 as u8, LC1 as u8, LC2 as u8,
                        BC1 as u8, BC2 as u8,
                    ],
                )),
                Box::new(CompositePruner::new(
                    "Edge orientations / Corner separations RUFLB",
                    "ruflbedgeorientationscornerseparations",
                    Box::new(EdgeOrientationPruner::new(
                        "Edge orientations RUFLB",
                        "ruflbedgeorientations",
                        &[
                            UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                            RE3 as u8, RE4 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8,
                            LE3 as u8, LE4 as u8, LE5 as u8, BE3 as u8, BE4 as u8, BE5 as u8,
                        ],
                    )),
                    Box::new(SeparationPruner::new(
                        "Corner separations U RUFLB",
                        "ruflbcornerseparationsu",
                        &[UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8],
                        &[],
                    )),
                )),
                Box::new(SeparationPruner::new(
                    "Separations R RUFLB",
                    "ruflbseparationsr",
                    &[RC1 as u8, FC5 as u8, UC3 as u8, UC2 as u8, RC5 as u8],
                    &[FE2 as u8, RE2 as u8, RE3 as u8, RE4 as u8, UE5 as u8],
                )),
                Box::new(SeparationPruner::new(
                    "Separations L RUFLB",
                    "ruflbseparationsl",
                    &[LC1 as u8, LC2 as u8, FC2 as u8, UC4 as u8, UC5 as u8],
                    &[FE5 as u8, UE2 as u8, LE3 as u8, LE4 as u8, LE5 as u8],
                )),
                Box::new(SeparationPruner::new(
                    "Separations F RUFLB",
                    "ruflbseparationsf",
                    &[FC5 as u8, FC2 as u8, FC1 as u8, UC4 as u8, UC3 as u8],
                    &[UE1 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8],
                )),
                Box::new(SeparationPruner::new(
                    "Separations B RUFLB",
                    "ruflbseparationsb",
                    &[LC2 as u8, BC1 as u8, BC2 as u8, UC1 as u8, UC5 as u8],
                    &[LE5 as u8, BE3 as u8, BE4 as u8, BE5 as u8, UE3 as u8],
                )),
            ],
        }
    }
}

impl std::fmt::Display for SearchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
