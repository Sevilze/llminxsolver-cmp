use super::state::LLMinx;
use crate::minx::moves::Move;

pub mod b_moves;
pub mod f_moves;
pub mod l_moves;
pub mod r_moves;
pub mod u_moves;

impl LLMinx {
    #[inline]
    pub(crate) fn corner_orient_cw(o: u8) -> u8 {
        match o {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => 3,
        }
    }

    #[inline]
    pub(crate) fn corner_orient_ccw(o: u8) -> u8 {
        match o {
            0 => 2,
            1 => 0,
            2 => 1,
            _ => 3,
        }
    }

    pub fn apply_move(&mut self, m: Move) {
        match m {
            Move::R => self.move_r(),
            Move::Ri => self.move_ri(),
            Move::R2 => self.move_r2(),
            Move::R2i => self.move_r2i(),
            Move::L => self.move_l(),
            Move::Li => self.move_li(),
            Move::L2 => self.move_l2(),
            Move::L2i => self.move_l2i(),
            Move::U => self.move_u(),
            Move::Ui => self.move_ui(),
            Move::U2 => self.move_u2(),
            Move::U2i => self.move_u2i(),
            Move::F => self.move_f(),
            Move::Fi => self.move_fi(),
            Move::F2 => self.move_f2(),
            Move::F2i => self.move_f2i(),
            Move::B => self.move_b(),
            Move::Bi => self.move_bi(),
            Move::B2 => self.move_b2(),
            Move::B2i => self.move_b2i(),
        }
    }

    pub fn undo_move(&mut self) -> Option<Move> {
        let last = self.last_move?;
        self.apply_move(last.inverse());
        self.moves.pop();
        self.moves.pop();
        self.last_move = self.moves.last().copied();
        Some(last)
    }

    pub(crate) fn record_move(&mut self, m: Move) {
        self.moves.push(m);
        self.last_move = Some(m);
    }
}
