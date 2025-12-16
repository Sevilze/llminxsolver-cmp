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
    RUFLbL,
    RUbL,
    RUbR,
}

impl SearchMode {
    pub fn description(&self) -> &'static str {
        match self {
            SearchMode::RU => "RU",
            SearchMode::RUF => "RUF",
            SearchMode::RUL => "RUL",
            SearchMode::RUFL => "RUFL",
            SearchMode::RUFLbL => "RUFLbL",
            SearchMode::RUbL => "RUbL",
            SearchMode::RUbR => "RUbR",
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
            SearchMode::RUFLbL => vec![
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
                Move::bL,
                Move::bLi,
                Move::bL2,
                Move::bL2i,
            ],
            SearchMode::RUbL => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
                Move::bL,
                Move::bLi,
                Move::bL2,
                Move::bL2i,
            ],
            SearchMode::RUbR => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
                Move::bR,
                Move::bRi,
                Move::bR2,
                Move::bR2i,
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
            SearchMode::RUFLbL => vec![
                Box::new(EdgeOrientationPruner::new(
                    "Edge orientations RUFLbL",
                    "ruflbledgeorientations",
                    &[
                        UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                        RE3 as u8, RE4 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8,
                        LE3 as u8, LE4 as u8, LE5 as u8, BLE3 as u8, BLE4 as u8, BLE5 as u8,
                    ],
                )),
                Box::new(CornerOrientationPruner::new(
                    "Corner orientations RUFLbL",
                    "ruflblcornerorientations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, FC1 as u8, FC2 as u8, LC1 as u8, LC2 as u8,
                        BLC1 as u8, BLC2 as u8,
                    ],
                )),
                Box::new(CompositePruner::new(
                    "Edge orientations / Corner separations RUFLbL",
                    "ruflbledgeorientationscornerseparations",
                    Box::new(EdgeOrientationPruner::new(
                        "Edge orientations RUFLbL",
                        "ruflbledgeorientations",
                        &[
                            UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                            RE3 as u8, RE4 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8,
                            LE3 as u8, LE4 as u8, LE5 as u8, BLE3 as u8, BLE4 as u8, BLE5 as u8,
                        ],
                    )),
                    Box::new(SeparationPruner::new(
                        "Corner separations U RUFLbL",
                        "ruflblcornerseparationsu",
                        &[UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8],
                        &[],
                    )),
                )),
                Box::new(SeparationPruner::new(
                    "Separations R RUFLbL",
                    "ruflblseparationsr",
                    &[RC1 as u8, FC5 as u8, UC3 as u8, UC2 as u8, RC5 as u8],
                    &[FE2 as u8, RE2 as u8, RE3 as u8, RE4 as u8, UE5 as u8],
                )),
                Box::new(SeparationPruner::new(
                    "Separations L RUFLbL",
                    "ruflblseparationsl",
                    &[LC1 as u8, LC2 as u8, FC2 as u8, UC4 as u8, UC5 as u8],
                    &[FE5 as u8, UE2 as u8, LE3 as u8, LE4 as u8, LE5 as u8],
                )),
                Box::new(SeparationPruner::new(
                    "Separations F RUFLbL",
                    "ruflblseparationsf",
                    &[FC5 as u8, FC2 as u8, FC1 as u8, UC4 as u8, UC3 as u8],
                    &[UE1 as u8, FE2 as u8, FE3 as u8, FE4 as u8, FE5 as u8],
                )),
                Box::new(SeparationPruner::new(
                    "Separations bL RUFLbL",
                    "ruflblseparationsbl",
                    &[LC2 as u8, BLC1 as u8, BLC2 as u8, UC1 as u8, UC5 as u8],
                    &[LE5 as u8, BLE3 as u8, BLE4 as u8, BLE5 as u8, UE3 as u8],
                )),
            ],
            SearchMode::RUbL => vec![
                Box::new(EdgePermutationPruner::new(
                    "Edge permutations RUbL",
                    "rubledgepermutations",
                    &[
                        UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, LE5 as u8,
                        BLE3 as u8, BLE4 as u8, BLE5 as u8,
                    ],
                )),
                Box::new(CompositePruner::new(
                    "Corners RUbL",
                    "rublcorners",
                    Box::new(CornerPermutationPruner::new(
                        "Corner permutations RUbL",
                        "rublcornerpermutations",
                        &[
                            UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, LC2 as u8,
                            BLC1 as u8, BLC2 as u8,
                        ],
                    )),
                    Box::new(CornerOrientationPruner::new(
                        "Corner orientations RUbL",
                        "rublcornerorientations",
                        &[
                            UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, LC2 as u8,
                            BLC1 as u8, BLC2 as u8,
                        ],
                    )),
                )),
            ],
            SearchMode::RUbR => vec![
                Box::new(CornerPermutationPruner::new(
                    "Corner permutations RUbR",
                    "rubrcornerpermutations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        RC5 as u8, FC5 as u8, BLC2 as u8, BRC1 as u8,
                    ],
                )),
                Box::new(EdgePermutationPruner::new(
                    "Edge permutations RUbR",
                    "rubredgepermutations",
                    &[
                        UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                        RE3 as u8, RE4 as u8, FE2 as u8, BRE3 as u8, BRE4 as u8, BLE5 as u8,
                    ],
                )),
                Box::new(CompositePruner::new(
                    "Orientations RUbR",
                    "rubrorientations",
                    Box::new(CornerOrientationPruner::new(
                        "Corner orientations RUbR",
                        "rubrcornerorientations",
                        &[
                            UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                            RC5 as u8, FC5 as u8, BLC2 as u8, BRC1 as u8,
                        ],
                    )),
                    Box::new(EdgeOrientationPruner::new(
                        "Edge orientations RUbR",
                        "rubgedgeorientations",
                        &[
                            UE1 as u8, UE2 as u8, UE3 as u8, UE4 as u8, UE5 as u8, RE2 as u8,
                            RE3 as u8, RE4 as u8, FE2 as u8, BRE3 as u8, BRE4 as u8, BLE5 as u8,
                        ],
                    )),
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
