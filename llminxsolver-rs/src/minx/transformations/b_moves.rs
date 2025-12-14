use crate::minx::moves::Move;
use crate::minx::position::{CornerPosition::*, EdgePosition::*};
use crate::minx::state::LLMinx;

impl LLMinx {
    pub fn move_b(&mut self) {
        let old_bc1_pos = self.corner_positions[BC1 as usize];
        let old_ue3_pos = self.edge_positions[UE3 as usize];
        let old_bc1_or = self.get_corner_orientation(BC1 as u8);
        let old_ue3_or = self.get_edge_orientation(UE3 as u8);

        self.corner_positions[BC1 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = self.corner_positions[BC2 as usize];
        self.corner_positions[BC2 as usize] = old_bc1_pos;

        self.edge_positions[UE3 as usize] = self.edge_positions[BE5 as usize];
        self.edge_positions[BE5 as usize] = self.edge_positions[BE4 as usize];
        self.edge_positions[BE4 as usize] = self.edge_positions[BE3 as usize];
        self.edge_positions[BE3 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = old_ue3_pos;

        self.set_corner_orientation(BC1 as u8, Self::corner_orient_cw(self.get_corner_orientation(LC2 as u8)));
        self.set_corner_orientation(LC2 as u8, self.get_corner_orientation(UC5 as u8));
        self.set_corner_orientation(UC5 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC1 as u8)));
        self.set_corner_orientation(UC1 as u8, Self::corner_orient_cw(self.get_corner_orientation(BC2 as u8)));
        self.set_corner_orientation(BC2 as u8, old_bc1_or);

        self.set_edge_orientation(UE3 as u8, (self.get_edge_orientation(BE5 as u8) ^ 1) & 1);
        self.set_edge_orientation(BE5 as u8, self.get_edge_orientation(BE4 as u8));
        self.set_edge_orientation(BE4 as u8, self.get_edge_orientation(BE3 as u8));
        self.set_edge_orientation(BE3 as u8, self.get_edge_orientation(LE5 as u8));
        self.set_edge_orientation(LE5 as u8, (old_ue3_or ^ 1) & 1);

        self.record_move(Move::B);
    }

    pub fn move_bi(&mut self) {
        let old_bc1_pos = self.corner_positions[BC1 as usize];
        let old_ue3_pos = self.edge_positions[UE3 as usize];
        let old_bc1_or = self.get_corner_orientation(BC1 as u8);
        let old_ue3_or = self.get_edge_orientation(UE3 as u8);

        self.corner_positions[BC1 as usize] = self.corner_positions[BC2 as usize];
        self.corner_positions[BC2 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = old_bc1_pos;

        self.edge_positions[UE3 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = self.edge_positions[BE3 as usize];
        self.edge_positions[BE3 as usize] = self.edge_positions[BE4 as usize];
        self.edge_positions[BE4 as usize] = self.edge_positions[BE5 as usize];
        self.edge_positions[BE5 as usize] = old_ue3_pos;

        self.set_corner_orientation(BC1 as u8, self.get_corner_orientation(BC2 as u8));
        self.set_corner_orientation(BC2 as u8, Self::corner_orient_ccw(self.get_corner_orientation(UC1 as u8)));
        self.set_corner_orientation(UC1 as u8, Self::corner_orient_ccw(self.get_corner_orientation(UC5 as u8)));
        self.set_corner_orientation(UC5 as u8, self.get_corner_orientation(LC2 as u8));
        self.set_corner_orientation(LC2 as u8, Self::corner_orient_ccw(old_bc1_or));

        self.set_edge_orientation(UE3 as u8, (self.get_edge_orientation(LE5 as u8) ^ 1) & 1);
        self.set_edge_orientation(LE5 as u8, self.get_edge_orientation(BE3 as u8));
        self.set_edge_orientation(BE3 as u8, self.get_edge_orientation(BE4 as u8));
        self.set_edge_orientation(BE4 as u8, self.get_edge_orientation(BE5 as u8));
        self.set_edge_orientation(BE5 as u8, (old_ue3_or ^ 1) & 1);

        self.record_move(Move::Bi);
    }

    pub fn move_b2(&mut self) {
        let old_bc1_pos = self.corner_positions[BC1 as usize];
        let old_ue3_pos = self.edge_positions[UE3 as usize];
        let old_bc1_or = self.get_corner_orientation(BC1 as u8);
        let old_ue3_or = self.get_edge_orientation(UE3 as u8);

        self.corner_positions[BC1 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[BC2 as usize];
        self.corner_positions[BC2 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = old_bc1_pos;

        self.edge_positions[UE3 as usize] = self.edge_positions[BE4 as usize];
        self.edge_positions[BE4 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = self.edge_positions[BE5 as usize];
        self.edge_positions[BE5 as usize] = self.edge_positions[BE3 as usize];
        self.edge_positions[BE3 as usize] = old_ue3_pos;

        self.set_corner_orientation(BC1 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC5 as u8)));
        self.set_corner_orientation(UC5 as u8, Self::corner_orient_ccw(self.get_corner_orientation(BC2 as u8)));
        self.set_corner_orientation(BC2 as u8, Self::corner_orient_cw(self.get_corner_orientation(LC2 as u8)));
        self.set_corner_orientation(LC2 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC1 as u8)));
        self.set_corner_orientation(UC1 as u8, Self::corner_orient_cw(old_bc1_or));

        self.set_edge_orientation(UE3 as u8, (self.get_edge_orientation(BE4 as u8) ^ 1) & 1);
        self.set_edge_orientation(BE4 as u8, self.get_edge_orientation(LE5 as u8));
        self.set_edge_orientation(LE5 as u8, self.get_edge_orientation(BE5 as u8));
        self.set_edge_orientation(BE5 as u8, self.get_edge_orientation(BE3 as u8));
        self.set_edge_orientation(BE3 as u8, (old_ue3_or ^ 1) & 1);

        self.record_move(Move::B2);
    }

    pub fn move_b2i(&mut self) {
        let old_bc1_pos = self.corner_positions[BC1 as usize];
        let old_ue3_pos = self.edge_positions[UE3 as usize];
        let old_bc1_or = self.get_corner_orientation(BC1 as u8);
        let old_ue3_or = self.get_edge_orientation(UE3 as u8);

        self.corner_positions[BC1 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = self.corner_positions[LC2 as usize];
        self.corner_positions[LC2 as usize] = self.corner_positions[BC2 as usize];
        self.corner_positions[BC2 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = old_bc1_pos;

        self.edge_positions[UE3 as usize] = self.edge_positions[BE3 as usize];
        self.edge_positions[BE3 as usize] = self.edge_positions[BE5 as usize];
        self.edge_positions[BE5 as usize] = self.edge_positions[LE5 as usize];
        self.edge_positions[LE5 as usize] = self.edge_positions[BE4 as usize];
        self.edge_positions[BE4 as usize] = old_ue3_pos;

        self.set_corner_orientation(BC1 as u8, Self::corner_orient_ccw(self.get_corner_orientation(UC1 as u8)));
        self.set_corner_orientation(UC1 as u8, Self::corner_orient_ccw(self.get_corner_orientation(LC2 as u8)));
        self.set_corner_orientation(LC2 as u8, Self::corner_orient_ccw(self.get_corner_orientation(BC2 as u8)));
        self.set_corner_orientation(BC2 as u8, Self::corner_orient_cw(self.get_corner_orientation(UC5 as u8)));
        self.set_corner_orientation(UC5 as u8, Self::corner_orient_ccw(old_bc1_or));

        self.set_edge_orientation(UE3 as u8, (self.get_edge_orientation(BE3 as u8) ^ 1) & 1);
        self.set_edge_orientation(BE3 as u8, self.get_edge_orientation(BE5 as u8));
        self.set_edge_orientation(BE5 as u8, self.get_edge_orientation(LE5 as u8));
        self.set_edge_orientation(LE5 as u8, self.get_edge_orientation(BE4 as u8));
        self.set_edge_orientation(BE4 as u8, (old_ue3_or ^ 1) & 1);

        self.record_move(Move::B2i);
    }
}
