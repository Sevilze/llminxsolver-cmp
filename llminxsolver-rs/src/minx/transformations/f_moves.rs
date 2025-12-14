use crate::minx::moves::Move;
use crate::minx::position::{CornerPosition::*, EdgePosition::*};
use crate::minx::state::LLMinx;

impl LLMinx {
    pub fn move_f(&mut self) {
        let old_fc1_pos = self.corner_positions[FC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_fc1_or = self.get_corner_orientation(FC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[FC1 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = old_fc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = self.edge_positions[FE4 as usize];
        self.edge_positions[FE4 as usize] = self.edge_positions[FE3 as usize];
        self.edge_positions[FE3 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = old_ue1_pos;

        self.set_corner_orientation(FC1 as u8, self.get_corner_orientation(FC5 as u8));
        self.set_corner_orientation(FC5 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC3 as u8)));
        self.set_corner_orientation(UC3 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC4 as u8)));
        self.set_corner_orientation(UC4 as u8, Self::corner_orient_cw(self.get_corner_orientation(FC2 as u8)));
        self.set_corner_orientation(FC2 as u8, old_fc1_or);

        self.set_edge_orientation(UE1 as u8, (self.get_edge_orientation(FE5 as u8) ^ 1) & 1);
        self.set_edge_orientation(FE5 as u8, self.get_edge_orientation(FE4 as u8));
        self.set_edge_orientation(FE4 as u8, self.get_edge_orientation(FE3 as u8));
        self.set_edge_orientation(FE3 as u8, self.get_edge_orientation(FE2 as u8));
        self.set_edge_orientation(FE2 as u8, (old_ue1_or ^ 1) & 1);

        self.record_move(Move::F);
    }

    pub fn move_fi(&mut self) {
        let old_fc1_pos = self.corner_positions[FC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_fc1_or = self.get_corner_orientation(FC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[FC1 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = old_fc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = self.edge_positions[FE3 as usize];
        self.edge_positions[FE3 as usize] = self.edge_positions[FE4 as usize];
        self.edge_positions[FE4 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = old_ue1_pos;

        self.set_corner_orientation(FC1 as u8, self.get_corner_orientation(FC2 as u8));
        self.set_corner_orientation(FC2 as u8, Self::corner_orient_ccw(self.get_corner_orientation(UC4 as u8)));
        self.set_corner_orientation(UC4 as u8, Self::corner_orient_ccw(self.get_corner_orientation(UC3 as u8)));
        self.set_corner_orientation(UC3 as u8, Self::corner_orient_ccw(self.get_corner_orientation(FC5 as u8)));
        self.set_corner_orientation(FC5 as u8, old_fc1_or);

        self.set_edge_orientation(UE1 as u8, (self.get_edge_orientation(FE2 as u8) ^ 1) & 1);
        self.set_edge_orientation(FE2 as u8, self.get_edge_orientation(FE3 as u8));
        self.set_edge_orientation(FE3 as u8, self.get_edge_orientation(FE4 as u8));
        self.set_edge_orientation(FE4 as u8, self.get_edge_orientation(FE5 as u8));
        self.set_edge_orientation(FE5 as u8, (old_ue1_or ^ 1) & 1);

        self.record_move(Move::Fi);
    }

    pub fn move_f2(&mut self) {
        let old_fc1_pos = self.corner_positions[FC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_fc1_or = self.get_corner_orientation(FC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[FC1 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = old_fc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[FE4 as usize];
        self.edge_positions[FE4 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = self.edge_positions[FE3 as usize];
        self.edge_positions[FE3 as usize] = old_ue1_pos;

        self.set_corner_orientation(FC1 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC3 as u8)));
        self.set_corner_orientation(UC3 as u8, Self::corner_orient_ccw(self.get_corner_orientation(FC2 as u8)));
        self.set_corner_orientation(FC2 as u8, self.get_corner_orientation(FC5 as u8));
        self.set_corner_orientation(FC5 as u8, Self::corner_orient_ccw(self.get_corner_orientation(UC4 as u8)));
        self.set_corner_orientation(UC4 as u8, Self::corner_orient_cw(old_fc1_or));

        self.set_edge_orientation(UE1 as u8, (self.get_edge_orientation(FE4 as u8) ^ 1) & 1);
        self.set_edge_orientation(FE4 as u8, self.get_edge_orientation(FE2 as u8));
        self.set_edge_orientation(FE2 as u8, self.get_edge_orientation(FE5 as u8));
        self.set_edge_orientation(FE5 as u8, self.get_edge_orientation(FE3 as u8));
        self.set_edge_orientation(FE3 as u8, (old_ue1_or ^ 1) & 1);

        self.record_move(Move::F2);
    }

    pub fn move_f2i(&mut self) {
        let old_fc1_pos = self.corner_positions[FC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_fc1_or = self.get_corner_orientation(FC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[FC1 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[FC2 as usize];
        self.corner_positions[FC2 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = old_fc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[FE3 as usize];
        self.edge_positions[FE3 as usize] = self.edge_positions[FE5 as usize];
        self.edge_positions[FE5 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = self.edge_positions[FE4 as usize];
        self.edge_positions[FE4 as usize] = old_ue1_pos;

        self.set_corner_orientation(FC1 as u8, Self::corner_orient_ccw(self.get_corner_orientation(UC4 as u8)));
        self.set_corner_orientation(UC4 as u8, Self::corner_orient_cw(self.get_corner_orientation(FC5 as u8)));
        self.set_corner_orientation(FC5 as u8, self.get_corner_orientation(FC2 as u8));
        self.set_corner_orientation(FC2 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC3 as u8)));
        self.set_corner_orientation(UC3 as u8, Self::corner_orient_ccw(old_fc1_or));

        self.set_edge_orientation(UE1 as u8, (self.get_edge_orientation(FE3 as u8) ^ 1) & 1);
        self.set_edge_orientation(FE3 as u8, self.get_edge_orientation(FE5 as u8));
        self.set_edge_orientation(FE5 as u8, self.get_edge_orientation(FE2 as u8));
        self.set_edge_orientation(FE2 as u8, self.get_edge_orientation(FE4 as u8));
        self.set_edge_orientation(FE4 as u8, (old_ue1_or ^ 1) & 1);

        self.record_move(Move::F2i);
    }
}
