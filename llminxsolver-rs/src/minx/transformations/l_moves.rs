use crate::minx::moves::Move;
use crate::minx::position::{CornerPosition::*, EdgePosition::*};
use crate::minx::state::LLMinx;

impl LLMinx {
    pub fn move_l(&mut self) {
        let old_lc1_pos = self.corner_positions[LC1 as usize];
        let old_ue2 = self.edge_positions[UE2 as usize];
        let old_lc1_or = self.get_corner_orientation(LC1 as u8);
        let old_ue2_or = self.get_edge_orientation(UE2 as u8);

        self.corner_positions[LC1 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = old_lc1_pos;

        self.edge_positions[UE2 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = self.edge_positions[LE4 as usize];
        self.edge_positions[LE4 as usize] = self.edge_positions[LE3 as usize];
        self.edge_positions[LE3 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = old_ue2;

        self.set_corner_orientation(
            LC1 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(FC2 as u8)),
        );
        self.set_corner_orientation(FC2 as u8, self.get_corner_orientation(UC4 as u8));
        self.set_corner_orientation(
            UC4 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC5 as u8)),
        );
        self.set_corner_orientation(
            UC5 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(LC2 as u8)),
        );
        self.set_corner_orientation(LC2 as u8, old_lc1_or);

        self.set_edge_orientation(UE2 as u8, self.get_edge_orientation(LE5 as u8));
        self.set_edge_orientation(LE5 as u8, self.get_edge_orientation(LE4 as u8));
        self.set_edge_orientation(LE4 as u8, self.get_edge_orientation(LE3 as u8));
        self.set_edge_orientation(LE3 as u8, self.get_edge_orientation(FE5 as u8));
        self.set_edge_orientation(FE5 as u8, old_ue2_or);

        self.record_move(Move::L);
    }

    pub fn move_li(&mut self) {
        let old_lc1_pos = self.corner_positions[LC1 as usize];
        let old_ue2 = self.edge_positions[UE2 as usize];
        let old_lc1_or = self.get_corner_orientation(LC1 as u8);
        let old_ue2_or = self.get_edge_orientation(UE2 as u8);

        self.corner_positions[LC1 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = old_lc1_pos;

        self.edge_positions[UE2 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = self.edge_positions[LE3 as usize];
        self.edge_positions[LE3 as usize] = self.edge_positions[LE4 as usize];
        self.edge_positions[LE4 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = old_ue2;

        self.set_corner_orientation(LC1 as u8, self.get_corner_orientation(LC2 as u8));
        self.set_corner_orientation(
            LC2 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC5 as u8)),
        );
        self.set_corner_orientation(
            UC5 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC4 as u8)),
        );
        self.set_corner_orientation(UC4 as u8, self.get_corner_orientation(FC2 as u8));
        self.set_corner_orientation(FC2 as u8, Self::corner_orient_ccw(old_lc1_or));

        self.set_edge_orientation(UE2 as u8, self.get_edge_orientation(FE5 as u8));
        self.set_edge_orientation(FE5 as u8, self.get_edge_orientation(LE3 as u8));
        self.set_edge_orientation(LE3 as u8, self.get_edge_orientation(LE4 as u8));
        self.set_edge_orientation(LE4 as u8, self.get_edge_orientation(LE5 as u8));
        self.set_edge_orientation(LE5 as u8, old_ue2_or);

        self.record_move(Move::Li);
    }

    pub fn move_l2(&mut self) {
        let old_lc1_pos = self.corner_positions[LC1 as usize];
        let old_ue2 = self.edge_positions[UE2 as usize];
        let old_lc1_or = self.get_corner_orientation(LC1 as u8);
        let old_ue2_or = self.get_edge_orientation(UE2 as u8);

        self.corner_positions[LC1 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = old_lc1_pos;

        self.edge_positions[UE2 as usize] = self.edge_positions[LE4 as usize];
        self.edge_positions[LE4 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = self.edge_positions[LE3 as usize];
        self.edge_positions[LE3 as usize] = old_ue2;

        self.set_corner_orientation(
            LC1 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC4 as u8)),
        );
        self.set_corner_orientation(
            UC4 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(LC2 as u8)),
        );
        self.set_corner_orientation(
            LC2 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(FC2 as u8)),
        );
        self.set_corner_orientation(
            FC2 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC5 as u8)),
        );
        self.set_corner_orientation(UC5 as u8, Self::corner_orient_cw(old_lc1_or));

        self.set_edge_orientation(UE2 as u8, self.get_edge_orientation(LE4 as u8));
        self.set_edge_orientation(LE4 as u8, self.get_edge_orientation(FE5 as u8));
        self.set_edge_orientation(FE5 as u8, self.get_edge_orientation(LE5 as u8));
        self.set_edge_orientation(LE5 as u8, self.get_edge_orientation(LE3 as u8));
        self.set_edge_orientation(LE3 as u8, old_ue2_or);

        self.record_move(Move::L2);
    }

    pub fn move_l2i(&mut self) {
        let old_lc1_pos = self.corner_positions[LC1 as usize];
        let old_ue2 = self.edge_positions[UE2 as usize];
        let old_lc1_or = self.get_corner_orientation(LC1 as u8);
        let old_ue2_or = self.get_edge_orientation(UE2 as u8);

        self.corner_positions[LC1 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = old_lc1_pos;

        self.edge_positions[UE2 as usize] = self.edge_positions[LE3 as usize];
        self.edge_positions[LE3 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = self.edge_positions[LE4 as usize];
        self.edge_positions[LE4 as usize] = old_ue2;

        self.set_corner_orientation(
            LC1 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC5 as u8)),
        );
        self.set_corner_orientation(
            UC5 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(FC2 as u8)),
        );
        self.set_corner_orientation(
            FC2 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(LC2 as u8)),
        );
        self.set_corner_orientation(
            LC2 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC4 as u8)),
        );
        self.set_corner_orientation(UC4 as u8, Self::corner_orient_ccw(old_lc1_or));

        self.set_edge_orientation(UE2 as u8, self.get_edge_orientation(LE3 as u8));
        self.set_edge_orientation(LE3 as u8, self.get_edge_orientation(LE5 as u8));
        self.set_edge_orientation(LE5 as u8, self.get_edge_orientation(FE5 as u8));
        self.set_edge_orientation(FE5 as u8, self.get_edge_orientation(LE4 as u8));
        self.set_edge_orientation(LE4 as u8, old_ue2_or);

        self.record_move(Move::L2i);
    }
}
