use crate::minx::moves::Move;
use crate::minx::position::{CornerPosition::*, EdgePosition::*};
use crate::minx::state::LLMinx;

impl LLMinx {
    pub fn move_d(&mut self) {
        let old_dc1_pos = self.corner_positions[DC1 as usize];
        let old_fe3_pos = self.edge_positions[FE3 as usize];
        let old_dc1_or = self.get_corner_orientation(DC1 as u8);
        let old_fe3_or = self.get_edge_orientation(FE3 as u8);

        self.corner_positions[DC1 as usize] = self.corner_positions[RC1 as usize];
        self.corner_positions[RC1 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[FC1 as usize];
        self.corner_positions[FC1 as usize] = self.corner_positions[DC2 as usize];
        self.corner_positions[DC2 as usize] = old_dc1_pos;

        self.edge_positions[FE3 as usize] = self.edge_positions[DE5 as usize];
        self.edge_positions[DE5 as usize] = self.edge_positions[DE4 as usize];
        self.edge_positions[DE4 as usize] = self.edge_positions[DE3 as usize];
        self.edge_positions[DE3 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = old_fe3_pos;

        self.set_corner_orientation(DC1 as u8, self.get_corner_orientation(RC1 as u8));
        self.set_corner_orientation(RC1 as u8, self.get_corner_orientation(FC5 as u8));
        self.set_corner_orientation(FC5 as u8, self.get_corner_orientation(FC1 as u8));
        self.set_corner_orientation(FC1 as u8, self.get_corner_orientation(DC2 as u8));
        self.set_corner_orientation(DC2 as u8, old_dc1_or);

        self.set_edge_orientation(FE3 as u8, self.get_edge_orientation(DE5 as u8));
        self.set_edge_orientation(DE5 as u8, self.get_edge_orientation(DE4 as u8));
        self.set_edge_orientation(DE4 as u8, self.get_edge_orientation(DE3 as u8));
        self.set_edge_orientation(DE3 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, old_fe3_or);

        self.record_move(Move::D);
    }

    pub fn move_di(&mut self) {
        let old_dc1_pos = self.corner_positions[DC1 as usize];
        let old_fe3_pos = self.edge_positions[FE3 as usize];
        let old_dc1_or = self.get_corner_orientation(DC1 as u8);
        let old_fe3_or = self.get_edge_orientation(FE3 as u8);

        self.corner_positions[DC1 as usize] = self.corner_positions[DC2 as usize];
        self.corner_positions[DC2 as usize] = self.corner_positions[FC1 as usize];
        self.corner_positions[FC1 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[RC1 as usize];
        self.corner_positions[RC1 as usize] = old_dc1_pos;

        self.edge_positions[FE3 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = self.edge_positions[DE3 as usize];
        self.edge_positions[DE3 as usize] = self.edge_positions[DE4 as usize];
        self.edge_positions[DE4 as usize] = self.edge_positions[DE5 as usize];
        self.edge_positions[DE5 as usize] = old_fe3_pos;

        self.set_corner_orientation(DC1 as u8, self.get_corner_orientation(DC2 as u8));
        self.set_corner_orientation(DC2 as u8, self.get_corner_orientation(FC1 as u8));
        self.set_corner_orientation(FC1 as u8, self.get_corner_orientation(FC5 as u8));
        self.set_corner_orientation(FC5 as u8, self.get_corner_orientation(RC1 as u8));
        self.set_corner_orientation(RC1 as u8, old_dc1_or);

        self.set_edge_orientation(FE3 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, self.get_edge_orientation(DE3 as u8));
        self.set_edge_orientation(DE3 as u8, self.get_edge_orientation(DE4 as u8));
        self.set_edge_orientation(DE4 as u8, self.get_edge_orientation(DE5 as u8));
        self.set_edge_orientation(DE5 as u8, old_fe3_or);

        self.record_move(Move::Di);
    }

    pub fn move_d2(&mut self) {
        let old_dc1_pos = self.corner_positions[DC1 as usize];
        let old_fe3_pos = self.edge_positions[FE3 as usize];
        let old_dc1_or = self.get_corner_orientation(DC1 as u8);
        let old_fe3_or = self.get_edge_orientation(FE3 as u8);

        self.corner_positions[DC1 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[DC2 as usize];
        self.corner_positions[DC2 as usize] = self.corner_positions[RC1 as usize];
        self.corner_positions[RC1 as usize] = self.corner_positions[FC1 as usize];
        self.corner_positions[FC1 as usize] = old_dc1_pos;

        self.edge_positions[FE3 as usize] = self.edge_positions[DE4 as usize];
        self.edge_positions[DE4 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = self.edge_positions[DE5 as usize];
        self.edge_positions[DE5 as usize] = self.edge_positions[DE3 as usize];
        self.edge_positions[DE3 as usize] = old_fe3_pos;

        self.set_corner_orientation(DC1 as u8, self.get_corner_orientation(FC5 as u8));
        self.set_corner_orientation(FC5 as u8, self.get_corner_orientation(DC2 as u8));
        self.set_corner_orientation(DC2 as u8, self.get_corner_orientation(RC1 as u8));
        self.set_corner_orientation(RC1 as u8, self.get_corner_orientation(FC1 as u8));
        self.set_corner_orientation(FC1 as u8, old_dc1_or);

        self.set_edge_orientation(FE3 as u8, self.get_edge_orientation(DE4 as u8));
        self.set_edge_orientation(DE4 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, self.get_edge_orientation(DE5 as u8));
        self.set_edge_orientation(DE5 as u8, self.get_edge_orientation(DE3 as u8));
        self.set_edge_orientation(DE3 as u8, old_fe3_or);

        self.record_move(Move::D2);
    }

    pub fn move_d2i(&mut self) {
        let old_dc1_pos = self.corner_positions[DC1 as usize];
        let old_fe3_pos = self.edge_positions[FE3 as usize];
        let old_dc1_or = self.get_corner_orientation(DC1 as u8);
        let old_fe3_or = self.get_edge_orientation(FE3 as u8);

        self.corner_positions[DC1 as usize] = self.corner_positions[FC1 as usize];
        self.corner_positions[FC1 as usize] = self.corner_positions[RC1 as usize];
        self.corner_positions[RC1 as usize] = self.corner_positions[DC2 as usize];
        self.corner_positions[DC2 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = old_dc1_pos;

        self.edge_positions[FE3 as usize] = self.edge_positions[DE3 as usize];
        self.edge_positions[DE3 as usize] = self.edge_positions[DE5 as usize];
        self.edge_positions[DE5 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = self.edge_positions[DE4 as usize];
        self.edge_positions[DE4 as usize] = old_fe3_pos;

        self.set_corner_orientation(DC1 as u8, self.get_corner_orientation(FC1 as u8));
        self.set_corner_orientation(FC1 as u8, self.get_corner_orientation(RC1 as u8));
        self.set_corner_orientation(RC1 as u8, self.get_corner_orientation(DC2 as u8));
        self.set_corner_orientation(DC2 as u8, self.get_corner_orientation(FC5 as u8));
        self.set_corner_orientation(FC5 as u8, old_dc1_or);

        self.set_edge_orientation(FE3 as u8, self.get_edge_orientation(DE3 as u8));
        self.set_edge_orientation(DE3 as u8, self.get_edge_orientation(DE5 as u8));
        self.set_edge_orientation(DE5 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, self.get_edge_orientation(DE4 as u8));
        self.set_edge_orientation(DE4 as u8, old_fe3_or);

        self.record_move(Move::D2i);
    }
}
