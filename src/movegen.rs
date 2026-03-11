use crate::board::Game;
use crate::types::{ChessMove, Color, PieceKind, Pos};

impl Game {
    pub fn legal_moves(&self) -> Vec<ChessMove> {
        let mut legal = Vec::new();
        for mv in self.pseudo_legal_moves(self.turn) {
            let mut next = self.clone();
            next.apply_move_unchecked(mv);
            if !next.is_in_check(self.turn) {
                legal.push(mv);
            }
        }
        legal
    }

    pub fn pseudo_legal_moves(&self, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for index in 0..64 {
            let from = Pos::from_index(index);
            let Some(piece) = self.piece_at(from) else {
                continue;
            };
            if piece.color != color {
                continue;
            }
            match piece.kind {
                PieceKind::Pawn => self.add_pawn_moves(from, color, &mut moves),
                PieceKind::Knight => {
                    self.add_leaper_moves(from, color, &mut moves, &KNIGHT_DELTAS);
                }
                PieceKind::Bishop => {
                    self.add_slider_moves(from, color, &mut moves, &BISHOP_DIRS);
                }
                PieceKind::Rook => {
                    self.add_slider_moves(from, color, &mut moves, &ROOK_DIRS);
                }
                PieceKind::Queen => {
                    self.add_slider_moves(from, color, &mut moves, &QUEEN_DIRS);
                }
                PieceKind::King => {
                    self.add_leaper_moves(from, color, &mut moves, &KING_DELTAS);
                    self.add_castling_moves(from, color, &mut moves);
                }
            }
        }
        moves
    }

    pub fn is_square_attacked(&self, target: Pos, by: Color) -> bool {
        for index in 0..64 {
            let from = Pos::from_index(index);
            let Some(piece) = self.piece_at(from) else {
                continue;
            };
            if piece.color != by {
                continue;
            }
            match piece.kind {
                PieceKind::Pawn => {
                    let dir = match by {
                        Color::White => 1,
                        Color::Black => -1,
                    };
                    for file_delta in [-1, 1] {
                        if let Some(attack) = offset(from, file_delta, dir) {
                            if attack == target {
                                return true;
                            }
                        }
                    }
                }
                PieceKind::Knight => {
                    if squares_match_any(from, target, &KNIGHT_DELTAS) {
                        return true;
                    }
                }
                PieceKind::Bishop => {
                    if self.attacked_by_slider(from, target, &BISHOP_DIRS) {
                        return true;
                    }
                }
                PieceKind::Rook => {
                    if self.attacked_by_slider(from, target, &ROOK_DIRS) {
                        return true;
                    }
                }
                PieceKind::Queen => {
                    if self.attacked_by_slider(from, target, &QUEEN_DIRS) {
                        return true;
                    }
                }
                PieceKind::King => {
                    if squares_match_any(from, target, &KING_DELTAS) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn add_pawn_moves(&self, from: Pos, color: Color, moves: &mut Vec<ChessMove>) {
        let dir = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        let start_rank = match color {
            Color::White => 1,
            Color::Black => 6,
        };
        let promotion_rank = match color {
            Color::White => 7,
            Color::Black => 0,
        };

        if let Some(one_forward) = offset(from, 0, dir) {
            if self.is_empty(one_forward) {
                self.push_pawn_move(from, one_forward, promotion_rank, moves);
                if from.rank == start_rank {
                    if let Some(two_forward) = offset(from, 0, dir * 2) {
                        if self.is_empty(two_forward) {
                            moves.push(ChessMove::new(from, two_forward));
                        }
                    }
                }
            }
        }

        for file_delta in [-1, 1] {
            let Some(capture) = offset(from, file_delta, dir) else {
                continue;
            };
            if let Some(piece) = self.piece_at(capture) {
                if piece.color != color {
                    self.push_pawn_move(from, capture, promotion_rank, moves);
                }
            } else if self.en_passant_target == Some(capture) {
                moves.push(ChessMove::new(from, capture));
            }
        }
    }

    fn push_pawn_move(&self, from: Pos, to: Pos, promotion_rank: u8, moves: &mut Vec<ChessMove>) {
        if to.rank == promotion_rank {
            for promotion in [
                PieceKind::Queen,
                PieceKind::Rook,
                PieceKind::Bishop,
                PieceKind::Knight,
            ] {
                moves.push(ChessMove::with_promotion(from, to, promotion));
            }
        } else {
            moves.push(ChessMove::new(from, to));
        }
    }

    fn add_leaper_moves(
        &self,
        from: Pos,
        color: Color,
        moves: &mut Vec<ChessMove>,
        deltas: &[(i32, i32)],
    ) {
        for &(df, dr) in deltas {
            let Some(to) = offset(from, df, dr) else {
                continue;
            };
            match self.piece_at(to) {
                None => moves.push(ChessMove::new(from, to)),
                Some(piece) if piece.color != color => moves.push(ChessMove::new(from, to)),
                _ => {}
            }
        }
    }

    fn add_slider_moves(
        &self,
        from: Pos,
        color: Color,
        moves: &mut Vec<ChessMove>,
        dirs: &[(i32, i32)],
    ) {
        for &(df, dr) in dirs {
            let mut cursor = from;
            while let Some(next) = offset(cursor, df, dr) {
                match self.piece_at(next) {
                    None => moves.push(ChessMove::new(from, next)),
                    Some(piece) if piece.color != color => {
                        moves.push(ChessMove::new(from, next));
                        break;
                    }
                    Some(_) => break,
                }
                cursor = next;
            }
        }
    }

    fn attacked_by_slider(&self, from: Pos, target: Pos, dirs: &[(i32, i32)]) -> bool {
        for &(df, dr) in dirs {
            let mut cursor = from;
            while let Some(next) = offset(cursor, df, dr) {
                if next == target {
                    return true;
                }
                if self.piece_at(next).is_some() {
                    break;
                }
                cursor = next;
            }
        }
        false
    }

    fn add_castling_moves(&self, from: Pos, color: Color, moves: &mut Vec<ChessMove>) {
        let (rank, king_side, queen_side) = match color {
            Color::White => (
                0,
                self.castling.white_king_side,
                self.castling.white_queen_side,
            ),
            Color::Black => (
                7,
                self.castling.black_king_side,
                self.castling.black_queen_side,
            ),
        };
        if from != Pos::new(4, rank) || self.is_in_check(color) {
            return;
        }

        if king_side
            && self.is_empty(Pos::new(5, rank))
            && self.is_empty(Pos::new(6, rank))
            && !self.is_square_attacked(Pos::new(5, rank), color.opposite())
            && !self.is_square_attacked(Pos::new(6, rank), color.opposite())
        {
            moves.push(ChessMove::new(from, Pos::new(6, rank)));
        }

        if queen_side
            && self.is_empty(Pos::new(1, rank))
            && self.is_empty(Pos::new(2, rank))
            && self.is_empty(Pos::new(3, rank))
            && !self.is_square_attacked(Pos::new(3, rank), color.opposite())
            && !self.is_square_attacked(Pos::new(2, rank), color.opposite())
        {
            moves.push(ChessMove::new(from, Pos::new(2, rank)));
        }
    }
}

const KNIGHT_DELTAS: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];
const KING_DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const BISHOP_DIRS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const ROOK_DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const QUEEN_DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

fn offset(from: Pos, file_delta: i32, rank_delta: i32) -> Option<Pos> {
    Pos::try_from_i32(
        i32::from(from.file) + file_delta,
        i32::from(from.rank) + rank_delta,
    )
}

fn squares_match_any(from: Pos, target: Pos, deltas: &[(i32, i32)]) -> bool {
    deltas
        .iter()
        .filter_map(|&(df, dr)| offset(from, df, dr))
        .any(|pos| pos == target)
}
