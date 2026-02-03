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
    RUD,
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
            SearchMode::RUD => "RUD",
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
            SearchMode::RUD => vec![
                Move::R,
                Move::Ri,
                Move::R2,
                Move::R2i,
                Move::U,
                Move::Ui,
                Move::U2,
                Move::U2i,
                Move::D,
                Move::Di,
                Move::D2,
                Move::D2i,
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
            SearchMode::RUD => vec![
                Box::new(CornerOrientationPruner::new(
                    "Corner orientations RUD",
                    "rudcornerorientations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        FC5 as u8, RC5 as u8, DC1 as u8, DC2 as u8, FC1 as u8,
                    ],
                )),
                Box::new(CornerPermutationPruner::new(
                    "Corner permutations RUD",
                    "rudcornerpermutations",
                    &[
                        UC1 as u8, UC2 as u8, UC3 as u8, UC4 as u8, UC5 as u8, RC1 as u8,
                        FC5 as u8, RC5 as u8, DC1 as u8, DC2 as u8, FC1 as u8,
                    ],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_fifth() {
        assert_eq!(Metric::Fifth.description(), "Fifth turn metric");
        assert_eq!(format!("{}", Metric::Fifth), "Fifth turn metric");
    }

    #[test]
    fn test_metric_face() {
        assert_eq!(Metric::Face.description(), "Face turn metric");
        assert_eq!(format!("{}", Metric::Face), "Face turn metric");
    }

    #[test]
    fn test_metric_clone() {
        let metric = Metric::Fifth;
        let cloned = metric;
        assert_eq!(metric, cloned);
    }

    #[test]
    fn test_search_mode_ru_description() {
        assert_eq!(SearchMode::RU.description(), "RU");
    }

    #[test]
    fn test_search_mode_ruf_description() {
        assert_eq!(SearchMode::RUF.description(), "RUF");
    }

    #[test]
    fn test_search_mode_rul_description() {
        assert_eq!(SearchMode::RUL.description(), "RUL");
    }

    #[test]
    fn test_search_mode_rufl_description() {
        assert_eq!(SearchMode::RUFL.description(), "RUFL");
    }

    #[test]
    fn test_search_mode_ruflbl_description() {
        assert_eq!(SearchMode::RUFLbL.description(), "RUFLbL");
    }

    #[test]
    fn test_search_mode_rubl_description() {
        assert_eq!(SearchMode::RUbL.description(), "RUbL");
    }

    #[test]
    fn test_search_mode_rubr_description() {
        assert_eq!(SearchMode::RUbR.description(), "RUbR");
    }

    #[test]
    fn test_search_mode_rud_description() {
        assert_eq!(SearchMode::RUD.description(), "RUD");
    }

    #[test]
    fn test_search_mode_display() {
        assert_eq!(format!("{}", SearchMode::RU), "RU");
        assert_eq!(format!("{}", SearchMode::RUF), "RUF");
    }

    #[test]
    fn test_search_mode_possible_moves_ru() {
        let moves = SearchMode::RU.possible_moves();
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&Move::R));
        assert!(moves.contains(&Move::U));
    }

    #[test]
    fn test_search_mode_possible_moves_ruf() {
        let moves = SearchMode::RUF.possible_moves();
        assert_eq!(moves.len(), 12);
        assert!(moves.contains(&Move::R));
        assert!(moves.contains(&Move::U));
        assert!(moves.contains(&Move::F));
    }

    #[test]
    fn test_search_mode_possible_moves_rul() {
        let moves = SearchMode::RUL.possible_moves();
        assert_eq!(moves.len(), 12);
        assert!(moves.contains(&Move::L));
    }

    #[test]
    fn test_search_mode_possible_moves_rufl() {
        let moves = SearchMode::RUFL.possible_moves();
        assert_eq!(moves.len(), 16);
        assert!(moves.contains(&Move::L));
    }

    #[test]
    fn test_search_mode_possible_moves_ruflbl() {
        let moves = SearchMode::RUFLbL.possible_moves();
        assert_eq!(moves.len(), 20);
        assert!(moves.contains(&Move::bL));
    }

    #[test]
    fn test_search_mode_possible_moves_rubl() {
        let moves = SearchMode::RUbL.possible_moves();
        assert_eq!(moves.len(), 12);
        assert!(moves.contains(&Move::bL));
    }

    #[test]
    fn test_search_mode_possible_moves_rubr() {
        let moves = SearchMode::RUbR.possible_moves();
        assert_eq!(moves.len(), 12);
        assert!(moves.contains(&Move::bR));
    }

    #[test]
    fn test_search_mode_possible_moves_rud() {
        let moves = SearchMode::RUD.possible_moves();
        assert_eq!(moves.len(), 12);
        assert!(moves.contains(&Move::D));
    }

    #[test]
    fn test_search_mode_create_pruners_ru() {
        let pruners = SearchMode::RU.create_pruners();
        assert!(!pruners.is_empty());
        // RU has edge permutation and composite (corner perm + corner orient)
        assert_eq!(pruners.len(), 2);
    }

    #[test]
    fn test_search_mode_create_pruners_ruf() {
        let pruners = SearchMode::RUF.create_pruners();
        assert!(!pruners.is_empty());
        // RUF has corner perm, edge perm, and composite orientations
        assert_eq!(pruners.len(), 3);
    }

    #[test]
    fn test_search_mode_create_pruners_rul() {
        let pruners = SearchMode::RUL.create_pruners();
        assert!(!pruners.is_empty());
    }

    #[test]
    fn test_search_mode_create_pruners_rufl() {
        let pruners = SearchMode::RUFL.create_pruners();
        assert!(!pruners.is_empty());
    }

    #[test]
    fn test_search_mode_create_pruners_ruflbl() {
        let pruners = SearchMode::RUFLbL.create_pruners();
        assert!(!pruners.is_empty());
    }

    #[test]
    fn test_search_mode_create_pruners_rubl() {
        let pruners = SearchMode::RUbL.create_pruners();
        assert!(!pruners.is_empty());
    }

    #[test]
    fn test_search_mode_create_pruners_rubr() {
        let pruners = SearchMode::RUbR.create_pruners();
        assert!(!pruners.is_empty());
    }

    #[test]
    fn test_search_mode_create_pruners_rud() {
        let pruners = SearchMode::RUD.create_pruners();
        assert!(!pruners.is_empty());
    }

    #[test]
    fn test_search_mode_serialize() {
        // Test that SearchMode can be serialized
        let mode = SearchMode::RU;
        let json = serde_json::to_string(&mode).unwrap();
        assert!(!json.is_empty());

        // Test deserialization
        let deserialized: SearchMode = serde_json::from_str(&json).unwrap();
        assert_eq!(mode, deserialized);
    }

    #[test]
    fn test_search_mode_all_variants_serialize() {
        let modes = [
            SearchMode::RU,
            SearchMode::RUF,
            SearchMode::RUL,
            SearchMode::RUFL,
            SearchMode::RUFLbL,
            SearchMode::RUbL,
            SearchMode::RUbR,
            SearchMode::RUD,
        ];

        for mode in &modes {
            let json = serde_json::to_string(mode).unwrap();
            let deserialized: SearchMode = serde_json::from_str(&json).unwrap();
            assert_eq!(*mode, deserialized);
        }
    }

    #[test]
    fn test_metric_serialize() {
        let metric = Metric::Fifth;
        let json = serde_json::to_string(&metric).unwrap();
        let deserialized: Metric = serde_json::from_str(&json).unwrap();
        assert_eq!(metric, deserialized);

        let metric = Metric::Face;
        let json = serde_json::to_string(&metric).unwrap();
        let deserialized: Metric = serde_json::from_str(&json).unwrap();
        assert_eq!(metric, deserialized);
    }

    #[test]
    fn test_search_mode_clone() {
        let mode = SearchMode::RU;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_search_mode_eq() {
        assert_eq!(SearchMode::RU, SearchMode::RU);
        assert_ne!(SearchMode::RU, SearchMode::RUF);
    }

    #[test]
    fn test_metric_eq() {
        assert_eq!(Metric::Fifth, Metric::Fifth);
        assert_eq!(Metric::Face, Metric::Face);
        assert_ne!(Metric::Fifth, Metric::Face);
    }
}
