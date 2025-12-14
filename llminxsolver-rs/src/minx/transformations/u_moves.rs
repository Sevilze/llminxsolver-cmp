use crate::minx::moves::Move;
use crate::minx::position::{CornerPosition::*, EdgePosition::*};
use crate::minx::state::LLMinx;

impl LLMinx {
    pub fn move_u(&mut self) {
        let old_uc1_pos = self.corner_positions[UC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_uc1_or = self.get_corner_orientation(UC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[UC1 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = old_uc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[UE5 as usize];
        self.edge_positions[UE5 as usize] = self.edge_positions[UE4 as usize];
        self.edge_positions[UE4 as usize] = self.edge_positions[UE3 as usize];
        self.edge_positions[UE3 as usize] = self.edge_positions[UE2 as usize];
        self.edge_positions[UE2 as usize] = old_ue1_pos;

        self.set_corner_orientation(UC1 as u8, self.get_corner_orientation(UC5 as u8));
        self.set_corner_orientation(UC5 as u8, self.get_corner_orientation(UC4 as u8));
        self.set_corner_orientation(UC4 as u8, self.get_corner_orientation(UC3 as u8));
        self.set_corner_orientation(UC3 as u8, self.get_corner_orientation(UC2 as u8));
        self.set_corner_orientation(UC2 as u8, old_uc1_or);

        self.set_edge_orientation(UE1 as u8, self.get_edge_orientation(UE5 as u8));
        self.set_edge_orientation(UE5 as u8, self.get_edge_orientation(UE4 as u8));
        self.set_edge_orientation(UE4 as u8, self.get_edge_orientation(UE3 as u8));
        self.set_edge_orientation(UE3 as u8, self.get_edge_orientation(UE2 as u8));
        self.set_edge_orientation(UE2 as u8, old_ue1_or);

        self.record_move(Move::U);
    }

    pub fn move_ui(&mut self) {
        let old_uc1_pos = self.corner_positions[UC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_uc1_or = self.get_corner_orientation(UC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[UC1 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = old_uc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[UE2 as usize];
        self.edge_positions[UE2 as usize] = self.edge_positions[UE3 as usize];
        self.edge_positions[UE3 as usize] = self.edge_positions[UE4 as usize];
        self.edge_positions[UE4 as usize] = self.edge_positions[UE5 as usize];
        self.edge_positions[UE5 as usize] = old_ue1_pos;

        self.set_corner_orientation(UC1 as u8, self.get_corner_orientation(UC2 as u8));
        self.set_corner_orientation(UC2 as u8, self.get_corner_orientation(UC3 as u8));
        self.set_corner_orientation(UC3 as u8, self.get_corner_orientation(UC4 as u8));
        self.set_corner_orientation(UC4 as u8, self.get_corner_orientation(UC5 as u8));
        self.set_corner_orientation(UC5 as u8, old_uc1_or);

        self.set_edge_orientation(UE1 as u8, self.get_edge_orientation(UE2 as u8));
        self.set_edge_orientation(UE2 as u8, self.get_edge_orientation(UE3 as u8));
        self.set_edge_orientation(UE3 as u8, self.get_edge_orientation(UE4 as u8));
        self.set_edge_orientation(UE4 as u8, self.get_edge_orientation(UE5 as u8));
        self.set_edge_orientation(UE5 as u8, old_ue1_or);

        self.record_move(Move::Ui);
    }

    pub fn move_u2(&mut self) {
        let old_uc1_pos = self.corner_positions[UC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_uc1_or = self.get_corner_orientation(UC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[UC1 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = old_uc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[UE4 as usize];
        self.edge_positions[UE4 as usize] = self.edge_positions[UE2 as usize];
        self.edge_positions[UE2 as usize] = self.edge_positions[UE5 as usize];
        self.edge_positions[UE5 as usize] = self.edge_positions[UE3 as usize];
        self.edge_positions[UE3 as usize] = old_ue1_pos;

        self.set_corner_orientation(UC1 as u8, self.get_corner_orientation(UC4 as u8));
        self.set_corner_orientation(UC4 as u8, self.get_corner_orientation(UC2 as u8));
        self.set_corner_orientation(UC2 as u8, self.get_corner_orientation(UC5 as u8));
        self.set_corner_orientation(UC5 as u8, self.get_corner_orientation(UC3 as u8));
        self.set_corner_orientation(UC3 as u8, old_uc1_or);

        self.set_edge_orientation(UE1 as u8, self.get_edge_orientation(UE4 as u8));
        self.set_edge_orientation(UE4 as u8, self.get_edge_orientation(UE2 as u8));
        self.set_edge_orientation(UE2 as u8, self.get_edge_orientation(UE5 as u8));
        self.set_edge_orientation(UE5 as u8, self.get_edge_orientation(UE3 as u8));
        self.set_edge_orientation(UE3 as u8, old_ue1_or);

        self.record_move(Move::U2);
    }

    pub fn move_u2i(&mut self) {
        let old_uc1_pos = self.corner_positions[UC1 as usize];
        let old_ue1_pos = self.edge_positions[UE1 as usize];
        let old_uc1_or = self.get_corner_orientation(UC1 as u8);
        let old_ue1_or = self.get_edge_orientation(UE1 as u8);

        self.corner_positions[UC1 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[UC5 as usize];
        self.corner_positions[UC5 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[UC4 as usize];
        self.corner_positions[UC4 as usize] = old_uc1_pos;

        self.edge_positions[UE1 as usize] = self.edge_positions[UE3 as usize];
        self.edge_positions[UE3 as usize] = self.edge_positions[UE5 as usize];
        self.edge_positions[UE5 as usize] = self.edge_positions[UE2 as usize];
        self.edge_positions[UE2 as usize] = self.edge_positions[UE4 as usize];
        self.edge_positions[UE4 as usize] = old_ue1_pos;

        self.set_corner_orientation(UC1 as u8, self.get_corner_orientation(UC3 as u8));
        self.set_corner_orientation(UC3 as u8, self.get_corner_orientation(UC5 as u8));
        self.set_corner_orientation(UC5 as u8, self.get_corner_orientation(UC2 as u8));
        self.set_corner_orientation(UC2 as u8, self.get_corner_orientation(UC4 as u8));
        self.set_corner_orientation(UC4 as u8, old_uc1_or);

        self.set_edge_orientation(UE1 as u8, self.get_edge_orientation(UE3 as u8));
        self.set_edge_orientation(UE3 as u8, self.get_edge_orientation(UE5 as u8));
        self.set_edge_orientation(UE5 as u8, self.get_edge_orientation(UE2 as u8));
        self.set_edge_orientation(UE2 as u8, self.get_edge_orientation(UE4 as u8));
        self.set_edge_orientation(UE4 as u8, old_ue1_or);

        self.record_move(Move::U2i);
    }
}
