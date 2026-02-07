use super::state::LLMinx;
use crate::minx::moves::Move;

pub mod bl_moves;
pub mod br_moves;
pub mod d_moves;
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
            Move::bL => self.move_bl(),
            Move::bLi => self.move_bli(),
            Move::bL2 => self.move_bl2(),
            Move::bL2i => self.move_bl2i(),
            Move::bR => self.move_br(),
            Move::bRi => self.move_bri(),
            Move::bR2 => self.move_br2(),
            Move::bR2i => self.move_br2i(),
            Move::D => self.move_d(),
            Move::Di => self.move_di(),
            Move::D2 => self.move_d2(),
            Move::D2i => self.move_d2i(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_orient_cw() {
        assert_eq!(LLMinx::corner_orient_cw(0), 1);
        assert_eq!(LLMinx::corner_orient_cw(1), 2);
        assert_eq!(LLMinx::corner_orient_cw(2), 0);
        assert_eq!(LLMinx::corner_orient_cw(3), 3);
    }

    #[test]
    fn test_corner_orient_ccw() {
        assert_eq!(LLMinx::corner_orient_ccw(0), 2);
        assert_eq!(LLMinx::corner_orient_ccw(1), 0);
        assert_eq!(LLMinx::corner_orient_ccw(2), 1);
        assert_eq!(LLMinx::corner_orient_ccw(3), 3);
    }

    #[test]
    fn test_corner_orient_cycle() {
        for i in 0..3u8 {
            let cw = LLMinx::corner_orient_cw(i);
            let ccw = LLMinx::corner_orient_ccw(cw);
            assert_eq!(ccw, i);
        }
    }

    #[test]
    fn test_apply_move_all() {
        for m in Move::ALL {
            let mut minx = LLMinx::new();
            minx.apply_move(m);
            assert_eq!(minx.last_move(), Some(m));
            assert_eq!(minx.depth(), 1);
        }
    }

    #[test]
    fn test_undo_move() {
        let mut minx = LLMinx::new();
        let initial_corners = minx.corner_positions;
        let initial_edges = minx.edge_positions;

        minx.apply_move(Move::R);
        assert_ne!(minx.corner_positions, initial_corners);

        let undone = minx.undo_move();
        assert_eq!(undone, Some(Move::R));
        assert_eq!(minx.corner_positions, initial_corners);
        assert_eq!(minx.edge_positions, initial_edges);
    }

    #[test]
    fn test_undo_move_empty() {
        let mut minx = LLMinx::new();
        let undone = minx.undo_move();
        assert_eq!(undone, None);
    }

    #[test]
    fn test_undo_move_multiple() {
        let mut minx = LLMinx::new();
        minx.apply_move(Move::R);
        minx.apply_move(Move::U);

        minx.undo_move();
        assert_eq!(minx.last_move(), Some(Move::R));
        assert_eq!(minx.depth(), 1);

        minx.undo_move();
        assert_eq!(minx.last_move(), None);
        assert_eq!(minx.depth(), 0);
    }

    #[test]
    fn test_move_inverse_identity() {
        for m in Move::ALL {
            let mut minx = LLMinx::new();
            let initial_corners = minx.corner_positions;
            let initial_edges = minx.edge_positions;
            let initial_co = minx.corner_orientations;
            let initial_eo = minx.edge_orientations;

            minx.apply_move(m);
            minx.apply_move(m.inverse());

            assert_eq!(
                minx.corner_positions, initial_corners,
                "Move {:?} inverse failed for corners",
                m
            );
            assert_eq!(
                minx.edge_positions, initial_edges,
                "Move {:?} inverse failed for edges",
                m
            );
            assert_eq!(
                minx.corner_orientations, initial_co,
                "Move {:?} inverse failed for corner orientations",
                m
            );
            assert_eq!(
                minx.edge_orientations, initial_eo,
                "Move {:?} inverse failed for edge orientations",
                m
            );
        }
    }

    #[test]
    fn test_r_move_changes_state() {
        let mut minx = LLMinx::new();
        let initial_corners = minx.corner_positions;
        let initial_edges = minx.edge_positions;

        minx.apply_move(Move::R);

        assert_ne!(minx.corner_positions, initial_corners);
        assert_ne!(minx.edge_positions, initial_edges);
    }

    #[test]
    fn test_u_move_changes_state() {
        let mut minx = LLMinx::new();
        let initial_corners = minx.corner_positions;
        let initial_edges = minx.edge_positions;

        minx.apply_move(Move::U);

        assert_ne!(minx.corner_positions, initial_corners);
        assert_ne!(minx.edge_positions, initial_edges);
    }

    #[test]
    fn test_r5_returns_to_start() {
        let mut minx = LLMinx::new();
        let initial = LLMinx::new();

        for _ in 0..5 {
            minx.apply_move(Move::R);
        }

        assert_eq!(minx.corner_positions, initial.corner_positions);
        assert_eq!(minx.edge_positions, initial.edge_positions);
        assert_eq!(minx.corner_orientations, initial.corner_orientations);
        assert_eq!(minx.edge_orientations, initial.edge_orientations);
    }

    #[test]
    fn test_u5_returns_to_start() {
        let mut minx = LLMinx::new();
        let initial = LLMinx::new();

        for _ in 0..5 {
            minx.apply_move(Move::U);
        }

        assert_eq!(minx.corner_positions, initial.corner_positions);
        assert_eq!(minx.edge_positions, initial.edge_positions);
        assert_eq!(minx.corner_orientations, initial.corner_orientations);
        assert_eq!(minx.edge_orientations, initial.edge_orientations);
    }

    #[test]
    fn test_r2_equals_rr() {
        let mut minx1 = LLMinx::new();
        let mut minx2 = LLMinx::new();

        minx1.apply_move(Move::R);
        minx1.apply_move(Move::R);

        minx2.apply_move(Move::R2);

        assert_eq!(minx1.corner_positions, minx2.corner_positions);
        assert_eq!(minx1.edge_positions, minx2.edge_positions);
        assert_eq!(minx1.corner_orientations, minx2.corner_orientations);
        assert_eq!(minx1.edge_orientations, minx2.edge_orientations);
    }

    #[test]
    fn test_double_moves_equivalence() {
        let face_moves = [
            (Move::R, Move::R2),
            (Move::L, Move::L2),
            (Move::U, Move::U2),
            (Move::F, Move::F2),
            (Move::bL, Move::bL2),
            (Move::bR, Move::bR2),
            (Move::D, Move::D2),
        ];

        for (single, double) in face_moves {
            let mut minx1 = LLMinx::new();
            let mut minx2 = LLMinx::new();

            minx1.apply_move(single);
            minx1.apply_move(single);

            minx2.apply_move(double);

            assert_eq!(
                minx1.corner_positions, minx2.corner_positions,
                "Failed for {:?}",
                single
            );
            assert_eq!(
                minx1.edge_positions, minx2.edge_positions,
                "Failed for {:?}",
                single
            );
        }
    }

    #[test]
    fn test_record_move() {
        let mut minx = LLMinx::new();
        minx.record_move(Move::R);

        assert_eq!(minx.moves.len(), 1);
        assert_eq!(minx.moves[0], Move::R);
        assert_eq!(minx.last_move, Some(Move::R));
    }
}
