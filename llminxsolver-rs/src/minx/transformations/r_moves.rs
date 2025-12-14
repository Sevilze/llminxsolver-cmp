use crate::minx::moves::Move;
use crate::minx::position::{CornerPosition::*, EdgePosition::*};
use crate::minx::state::LLMinx;

impl LLMinx {
    pub fn move_r(&mut self) {
        let old_rc1_pos = self.corner_positions[RC1 as usize];
        let old_eu5_pos = self.edge_positions[UE5 as usize];
        let old_rc1_or = self.get_corner_orientation(RC1 as u8);
        let old_ue5_or = self.get_edge_orientation(UE5 as u8);

        self.corner_positions[RC1 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = old_rc1_pos;

        self.edge_positions[UE5 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = self.edge_positions[RE3 as usize];
        self.edge_positions[RE3 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = old_eu5_pos;

        self.set_corner_orientation(RC1 as u8, self.get_corner_orientation(RC5 as u8));
        self.set_corner_orientation(
            RC5 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC2 as u8)),
        );
        self.set_corner_orientation(
            UC2 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC3 as u8)),
        );
        self.set_corner_orientation(UC3 as u8, self.get_corner_orientation(FC5 as u8));
        self.set_corner_orientation(FC5 as u8, Self::corner_orient_cw(old_rc1_or));

        self.set_edge_orientation(UE5 as u8, self.get_edge_orientation(FE2 as u8));
        self.set_edge_orientation(FE2 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, self.get_edge_orientation(RE3 as u8));
        self.set_edge_orientation(RE3 as u8, self.get_edge_orientation(RE2 as u8));
        self.set_edge_orientation(RE2 as u8, old_ue5_or);

        self.record_move(Move::R);
    }

    pub fn move_ri(&mut self) {
        let old_rc1_pos = self.corner_positions[RC1 as usize];
        let old_eu5_pos = self.edge_positions[UE5 as usize];
        let old_rc1_or = self.get_corner_orientation(RC1 as u8);
        let old_ue5_or = self.get_edge_orientation(UE5 as u8);

        self.corner_positions[RC1 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = old_rc1_pos;

        self.edge_positions[UE5 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = self.edge_positions[RE3 as usize];
        self.edge_positions[RE3 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = old_eu5_pos;

        self.set_corner_orientation(
            RC1 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(FC5 as u8)),
        );
        self.set_corner_orientation(FC5 as u8, self.get_corner_orientation(UC3 as u8));
        self.set_corner_orientation(
            UC3 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC2 as u8)),
        );
        self.set_corner_orientation(
            UC2 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(RC5 as u8)),
        );
        self.set_corner_orientation(RC5 as u8, old_rc1_or);

        self.set_edge_orientation(UE5 as u8, self.get_edge_orientation(RE2 as u8));
        self.set_edge_orientation(RE2 as u8, self.get_edge_orientation(RE3 as u8));
        self.set_edge_orientation(RE3 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, self.get_edge_orientation(FE2 as u8));
        self.set_edge_orientation(FE2 as u8, old_ue5_or);

        self.record_move(Move::Ri);
    }

    pub fn move_r2(&mut self) {
        let old_rc1_pos = self.corner_positions[RC1 as usize];
        let old_eu5_pos = self.edge_positions[UE5 as usize];
        let old_rc1_or = self.get_corner_orientation(RC1 as u8);
        let old_ue5_or = self.get_edge_orientation(UE5 as u8);

        self.corner_positions[RC1 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = old_rc1_pos;

        self.edge_positions[UE5 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = self.edge_positions[RE3 as usize];
        self.edge_positions[RE3 as usize] = old_eu5_pos;

        self.set_corner_orientation(
            RC1 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC2 as u8)),
        );
        self.set_corner_orientation(
            UC2 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(FC5 as u8)),
        );
        self.set_corner_orientation(
            FC5 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(RC5 as u8)),
        );
        self.set_corner_orientation(
            RC5 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC3 as u8)),
        );
        self.set_corner_orientation(UC3 as u8, Self::corner_orient_cw(old_rc1_or));

        self.set_edge_orientation(UE5 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, self.get_edge_orientation(RE2 as u8));
        self.set_edge_orientation(RE2 as u8, self.get_edge_orientation(FE2 as u8));
        self.set_edge_orientation(FE2 as u8, self.get_edge_orientation(RE3 as u8));
        self.set_edge_orientation(RE3 as u8, old_ue5_or);

        self.record_move(Move::R2);
    }

    pub fn move_r2i(&mut self) {
        let old_rc1_pos = self.corner_positions[RC1 as usize];
        let old_eu5_pos = self.edge_positions[UE5 as usize];
        let old_rc1_or = self.get_corner_orientation(RC1 as u8);
        let old_ue5_or = self.get_edge_orientation(UE5 as u8);

        self.corner_positions[RC1 as usize] = self.corner_positions[UC3 as usize];
        self.corner_positions[UC3 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = self.corner_positions[FC5 as usize];
        self.corner_positions[FC5 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = old_rc1_pos;

        self.edge_positions[UE5 as usize] = self.edge_positions[RE3 as usize];
        self.edge_positions[RE3 as usize] = self.edge_positions[FE2 as usize];
        self.edge_positions[FE2 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = self.edge_positions[RE4 as usize];
        self.edge_positions[RE4 as usize] = old_eu5_pos;

        self.set_corner_orientation(
            RC1 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC3 as u8)),
        );
        self.set_corner_orientation(
            UC3 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(RC5 as u8)),
        );
        self.set_corner_orientation(
            RC5 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(FC5 as u8)),
        );
        self.set_corner_orientation(
            FC5 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC2 as u8)),
        );
        self.set_corner_orientation(UC2 as u8, Self::corner_orient_ccw(old_rc1_or));

        self.set_edge_orientation(UE5 as u8, self.get_edge_orientation(RE3 as u8));
        self.set_edge_orientation(RE3 as u8, self.get_edge_orientation(FE2 as u8));
        self.set_edge_orientation(FE2 as u8, self.get_edge_orientation(RE2 as u8));
        self.set_edge_orientation(RE2 as u8, self.get_edge_orientation(RE4 as u8));
        self.set_edge_orientation(RE4 as u8, old_ue5_or);

        self.record_move(Move::R2i);
    }
}
