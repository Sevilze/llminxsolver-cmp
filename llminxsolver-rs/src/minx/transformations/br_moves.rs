use crate::minx::moves::Move;
use crate::minx::position::{CornerPosition::*, EdgePosition::*};
use crate::minx::state::LLMinx;

impl LLMinx {
    pub fn move_br(&mut self) {
        let old_brc1_pos = self.corner_positions[BRC1 as usize];
        let old_ue4_pos = self.edge_positions[UE4 as usize];
        let old_brc1_or = self.get_corner_orientation(BRC1 as u8);
        let old_ue4_or = self.get_edge_orientation(UE4 as u8);

        self.corner_positions[BRC1 as usize] = self.corner_positions[BLC2 as usize];
        self.corner_positions[BLC2 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = old_brc1_pos;

        self.edge_positions[UE4 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = self.edge_positions[BRE3 as usize];
        self.edge_positions[BRE3 as usize] = self.edge_positions[BRE4 as usize];
        self.edge_positions[BRE4 as usize] = self.edge_positions[BLE5 as usize];
        self.edge_positions[BLE5 as usize] = old_ue4_pos;

        self.set_corner_orientation(BRC1 as u8, self.get_corner_orientation(BLC2 as u8));
        self.set_corner_orientation(
            BLC2 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC1 as u8)),
        );
        self.set_corner_orientation(
            UC1 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC2 as u8)),
        );
        self.set_corner_orientation(UC2 as u8, self.get_corner_orientation(RC5 as u8));
        self.set_corner_orientation(RC5 as u8, Self::corner_orient_cw(old_brc1_or));

        self.set_edge_orientation(UE4 as u8, (self.get_edge_orientation(RE2 as u8) ^ 1) & 1);
        self.set_edge_orientation(RE2 as u8, self.get_edge_orientation(BRE3 as u8));
        self.set_edge_orientation(BRE3 as u8, self.get_edge_orientation(BRE4 as u8));
        self.set_edge_orientation(BRE4 as u8, self.get_edge_orientation(BLE5 as u8));
        self.set_edge_orientation(BLE5 as u8, (old_ue4_or ^ 1) & 1);

        self.record_move(Move::bR);
    }

    pub fn move_bri(&mut self) {
        let old_brc1_pos = self.corner_positions[BRC1 as usize];
        let old_ue4_pos = self.edge_positions[UE4 as usize];
        let old_brc1_or = self.get_corner_orientation(BRC1 as u8);
        let old_ue4_or = self.get_edge_orientation(UE4 as u8);

        self.corner_positions[BRC1 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = self.corner_positions[BLC2 as usize];
        self.corner_positions[BLC2 as usize] = old_brc1_pos;

        self.edge_positions[UE4 as usize] = self.edge_positions[BLE5 as usize];
        self.edge_positions[BLE5 as usize] = self.edge_positions[BRE4 as usize];
        self.edge_positions[BRE4 as usize] = self.edge_positions[BRE3 as usize];
        self.edge_positions[BRE3 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = old_ue4_pos;

        self.set_corner_orientation(
            BRC1 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(RC5 as u8)),
        );
        self.set_corner_orientation(RC5 as u8, self.get_corner_orientation(UC2 as u8));
        self.set_corner_orientation(
            UC2 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC1 as u8)),
        );
        self.set_corner_orientation(
            UC1 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(BLC2 as u8)),
        );
        self.set_corner_orientation(BLC2 as u8, old_brc1_or);

        self.set_edge_orientation(UE4 as u8, (self.get_edge_orientation(BLE5 as u8) ^ 1) & 1);
        self.set_edge_orientation(BLE5 as u8, self.get_edge_orientation(BRE4 as u8));
        self.set_edge_orientation(BRE4 as u8, self.get_edge_orientation(BRE3 as u8));
        self.set_edge_orientation(BRE3 as u8, self.get_edge_orientation(RE2 as u8));
        self.set_edge_orientation(RE2 as u8, (old_ue4_or ^ 1) & 1);

        self.record_move(Move::bRi);
    }

    pub fn move_br2(&mut self) {
        let old_brc1_pos = self.corner_positions[BRC1 as usize];
        let old_ue4_pos = self.edge_positions[UE4 as usize];
        let old_brc1_or = self.get_corner_orientation(BRC1 as u8);
        let old_ue4_or = self.get_edge_orientation(UE4 as u8);

        self.corner_positions[BRC1 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = self.corner_positions[BLC2 as usize];
        self.corner_positions[BLC2 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = old_brc1_pos;

        self.edge_positions[UE4 as usize] = self.edge_positions[BRE3 as usize];
        self.edge_positions[BRE3 as usize] = self.edge_positions[BLE5 as usize];
        self.edge_positions[BLE5 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = self.edge_positions[BRE4 as usize];
        self.edge_positions[BRE4 as usize] = old_ue4_pos;

        self.set_corner_orientation(
            BRC1 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(UC1 as u8)),
        );
        self.set_corner_orientation(
            UC1 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(RC5 as u8)),
        );
        self.set_corner_orientation(
            RC5 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(BLC2 as u8)),
        );
        self.set_corner_orientation(
            BLC2 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC2 as u8)),
        );
        self.set_corner_orientation(UC2 as u8, Self::corner_orient_cw(old_brc1_or));

        self.set_edge_orientation(UE4 as u8, (self.get_edge_orientation(BRE3 as u8) ^ 1) & 1);
        self.set_edge_orientation(BRE3 as u8, self.get_edge_orientation(BLE5 as u8));
        self.set_edge_orientation(BLE5 as u8, self.get_edge_orientation(RE2 as u8));
        self.set_edge_orientation(RE2 as u8, self.get_edge_orientation(BRE4 as u8));
        self.set_edge_orientation(BRE4 as u8, (old_ue4_or ^ 1) & 1);

        self.record_move(Move::bR2);
    }

    pub fn move_br2i(&mut self) {
        let old_brc1_pos = self.corner_positions[BRC1 as usize];
        let old_ue4_pos = self.edge_positions[UE4 as usize];
        let old_brc1_or = self.get_corner_orientation(BRC1 as u8);
        let old_ue4_or = self.get_edge_orientation(UE4 as u8);

        self.corner_positions[BRC1 as usize] = self.corner_positions[UC2 as usize];
        self.corner_positions[UC2 as usize] = self.corner_positions[BLC2 as usize];
        self.corner_positions[BLC2 as usize] = self.corner_positions[RC5 as usize];
        self.corner_positions[RC5 as usize] = self.corner_positions[UC1 as usize];
        self.corner_positions[UC1 as usize] = old_brc1_pos;

        self.edge_positions[UE4 as usize] = self.edge_positions[BRE4 as usize];
        self.edge_positions[BRE4 as usize] = self.edge_positions[RE2 as usize];
        self.edge_positions[RE2 as usize] = self.edge_positions[BLE5 as usize];
        self.edge_positions[BLE5 as usize] = self.edge_positions[BRE3 as usize];
        self.edge_positions[BRE3 as usize] = old_ue4_pos;

        self.set_corner_orientation(
            BRC1 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC2 as u8)),
        );
        self.set_corner_orientation(
            UC2 as u8,
            Self::corner_orient_cw(self.get_corner_orientation(BLC2 as u8)),
        );
        self.set_corner_orientation(
            BLC2 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(RC5 as u8)),
        );
        self.set_corner_orientation(
            RC5 as u8,
            Self::corner_orient_ccw(self.get_corner_orientation(UC1 as u8)),
        );
        self.set_corner_orientation(UC1 as u8, Self::corner_orient_ccw(old_brc1_or));

        self.set_edge_orientation(UE4 as u8, (self.get_edge_orientation(BRE4 as u8) ^ 1) & 1);
        self.set_edge_orientation(BRE4 as u8, self.get_edge_orientation(RE2 as u8));
        self.set_edge_orientation(RE2 as u8, self.get_edge_orientation(BLE5 as u8));
        self.set_edge_orientation(BLE5 as u8, self.get_edge_orientation(BRE3 as u8));
        self.set_edge_orientation(BRE3 as u8, (old_ue4_or ^ 1) & 1);

        self.record_move(Move::bR2i);
    }
}
