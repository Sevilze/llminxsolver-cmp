use super::moves::Move;

pub const NUM_CORNERS: usize = 17;
pub const NUM_EDGES: usize = 23;
pub const MAX_SEARCH_DEPTH: usize = 100;

#[derive(Clone, Debug)]
pub struct LLMinx {
    pub(crate) corner_positions: [u8; NUM_CORNERS],
    pub(crate) edge_positions: [u8; NUM_EDGES],
    pub(crate) corner_orientations: u64,
    pub(crate) edge_orientations: u32,
    pub(crate) ignore_corner_positions: [bool; NUM_CORNERS],
    pub(crate) ignore_edge_positions: [bool; NUM_EDGES],
    pub(crate) ignore_corner_orientations: [bool; NUM_CORNERS],
    pub(crate) ignore_edge_orientations: [bool; NUM_EDGES],
    pub(crate) moves: Vec<Move>,
    pub(crate) last_move: Option<Move>,
}

impl Default for LLMinx {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMinx {
    pub fn new() -> Self {
        let mut corner_positions = [0u8; NUM_CORNERS];
        let mut edge_positions = [0u8; NUM_EDGES];

        for (i, pos) in corner_positions.iter_mut().enumerate() {
            *pos = i as u8;
        }
        for (i, pos) in edge_positions.iter_mut().enumerate() {
            *pos = i as u8;
        }

        LLMinx {
            corner_positions,
            edge_positions,
            corner_orientations: 0,
            edge_orientations: 0,
            ignore_corner_positions: [false; NUM_CORNERS],
            ignore_edge_positions: [false; NUM_EDGES],
            ignore_corner_orientations: [false; NUM_CORNERS],
            ignore_edge_orientations: [false; NUM_EDGES],
            moves: Vec::with_capacity(MAX_SEARCH_DEPTH),
            last_move: None,
        }
    }

    pub fn with_state(
        corner_positions: [u8; NUM_CORNERS],
        edge_positions: [u8; NUM_EDGES],
        corner_orientations: u64,
        edge_orientations: u32,
    ) -> Self {
        LLMinx {
            corner_positions,
            edge_positions,
            corner_orientations,
            edge_orientations,
            ignore_corner_positions: [false; NUM_CORNERS],
            ignore_edge_positions: [false; NUM_EDGES],
            ignore_corner_orientations: [false; NUM_CORNERS],
            ignore_edge_orientations: [false; NUM_EDGES],
            moves: Vec::with_capacity(MAX_SEARCH_DEPTH),
            last_move: None,
        }
    }

    #[inline]
    pub fn corner_positions(&self) -> &[u8; NUM_CORNERS] {
        &self.corner_positions
    }

    #[inline]
    pub fn corner_positions_mut(&mut self) -> &mut [u8; NUM_CORNERS] {
        &mut self.corner_positions
    }

    #[inline]
    pub fn set_corner_positions(&mut self, positions: [u8; NUM_CORNERS]) {
        self.corner_positions = positions;
    }

    #[inline]
    pub fn edge_positions(&self) -> &[u8; NUM_EDGES] {
        &self.edge_positions
    }

    #[inline]
    pub fn edge_positions_mut(&mut self) -> &mut [u8; NUM_EDGES] {
        &mut self.edge_positions
    }

    #[inline]
    pub fn set_edge_positions(&mut self, positions: [u8; NUM_EDGES]) {
        self.edge_positions = positions;
    }

    #[inline]
    pub fn corner_orientations(&self) -> u64 {
        self.corner_orientations
    }

    #[inline]
    pub fn set_corner_orientations(&mut self, orientations: u64) {
        self.corner_orientations = orientations;
    }

    #[inline]
    pub fn edge_orientations(&self) -> u32 {
        self.edge_orientations
    }

    #[inline]
    pub fn set_edge_orientations(&mut self, orientations: u32) {
        self.edge_orientations = orientations;
    }

    #[inline]
    pub fn get_corner_orientation(&self, piece: u8) -> u8 {
        ((self.corner_orientations >> (piece * 2)) & 3) as u8
    }

    #[inline]
    pub fn set_corner_orientation(&mut self, piece: u8, orientation: u8) {
        let mask = !(3u64 << (piece * 2));
        self.corner_orientations =
            (self.corner_orientations & mask) | ((orientation as u64) << (piece * 2));
    }

    #[inline]
    pub fn get_edge_orientation(&self, piece: u8) -> u8 {
        ((self.edge_orientations >> piece) & 1) as u8
    }

    #[inline]
    pub fn set_edge_orientation(&mut self, piece: u8, orientation: u8) {
        let mask = !(1u32 << piece);
        self.edge_orientations = (self.edge_orientations & mask) | ((orientation as u32) << piece);
    }

    pub fn ignore_corner_positions(&self) -> &[bool; NUM_CORNERS] {
        &self.ignore_corner_positions
    }

    pub fn set_ignore_corner_positions(&mut self, ignore: [bool; NUM_CORNERS]) {
        self.ignore_corner_positions = ignore;
    }

    pub fn ignore_edge_positions(&self) -> &[bool; NUM_EDGES] {
        &self.ignore_edge_positions
    }

    pub fn set_ignore_edge_positions(&mut self, ignore: [bool; NUM_EDGES]) {
        self.ignore_edge_positions = ignore;
    }

    pub fn ignore_corner_orientations(&self) -> &[bool; NUM_CORNERS] {
        &self.ignore_corner_orientations
    }

    pub fn set_ignore_corner_orientations(&mut self, ignore: [bool; NUM_CORNERS]) {
        self.ignore_corner_orientations = ignore;
    }

    pub fn ignore_edge_orientations(&self) -> &[bool; NUM_EDGES] {
        &self.ignore_edge_orientations
    }

    pub fn set_ignore_edge_orientations(&mut self, ignore: [bool; NUM_EDGES]) {
        self.ignore_edge_orientations = ignore;
    }

    #[inline]
    pub fn depth(&self) -> usize {
        self.moves.len()
    }

    #[inline]
    pub fn last_move(&self) -> Option<Move> {
        self.last_move
    }

    #[inline]
    pub fn moves(&self) -> &[Move] {
        &self.moves
    }

    pub fn clear_moves(&mut self) {
        self.moves.clear();
        self.last_move = None;
    }

    pub fn get_generating_moves(&self) -> String {
        let mut result = String::with_capacity(self.moves.len() * 4);
        let mut moves = self.moves.clone();
        while Self::simplify_moves(&mut moves) {}
        for m in moves {
            result.push_str(m.to_string());
        }
        result
    }

    fn simplify_moves(moves: &mut Vec<Move>) -> bool {
        for i in 1..moves.len() {
            if moves[i] == moves[i - 1] && (moves[i] as u8) % 4 < 2 {
                let new_move = Move::from_u8((moves[i - 1] as u8) + 2).unwrap();
                moves[i - 1] = new_move;
                moves.remove(i);
                return true;
            }
        }
        false
    }

    pub fn get_solving_moves(&self) -> String {
        let mut result = String::with_capacity(self.moves.len() * 4);
        for m in self.moves.iter().rev() {
            result.push_str(m.inverse().to_string());
        }
        result
    }

    pub fn get_fftm_length(&self) -> usize {
        let mut length = 0;
        for m in &self.moves {
            length += ((*m as u8) % 4) / 2 + 1;
        }
        length as usize
    }

    pub fn get_ftm_length(&self) -> usize {
        let mut length = self.moves.len();
        for i in 1..self.moves.len() {
            if self.moves[i].face() == self.moves[i - 1].face() {
                length -= 1;
            }
        }
        length
    }

    pub fn state_equals(&self, other: &LLMinx) -> bool {
        for i in 0..NUM_CORNERS {
            let piece = self.corner_positions[i];
            if self.corner_positions[i] != other.corner_positions[i]
                && !self.ignore_corner_positions[piece as usize]
            {
                return false;
            }
            if self.get_corner_orientation(i as u8) != other.get_corner_orientation(i as u8)
                && !self.ignore_corner_orientations[piece as usize]
            {
                return false;
            }
        }
        for i in 0..NUM_EDGES {
            let piece = self.edge_positions[i];
            if self.edge_positions[i] != other.edge_positions[i]
                && !self.ignore_edge_positions[piece as usize]
            {
                return false;
            }
            if self.get_edge_orientation(i as u8) != other.get_edge_orientation(i as u8)
                && !self.ignore_edge_orientations[piece as usize]
            {
                return false;
            }
        }
        true
    }
}

impl PartialEq for LLMinx {
    fn eq(&self, other: &Self) -> bool {
        self.state_equals(other)
    }
}

impl Eq for LLMinx {}
