use crate::types::{ChessMove, Color, GameStatus, Piece, PieceKind, Pos};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl CastlingRights {
    pub fn initial() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub board: [Option<Piece>; 64],
    pub turn: Color,
    pub castling: CastlingRights,
    pub en_passant_target: Option<Pos>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub repetition_counts: HashMap<u64, u32>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: [None; 64],
            turn: Color::White,
            castling: CastlingRights::initial(),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            repetition_counts: HashMap::new(),
        };
        game.setup_start_position();
        game.record_current_position();
        game
    }

    pub fn empty() -> Self {
        Self {
            board: [None; 64],
            turn: Color::White,
            castling: CastlingRights {
                white_king_side: false,
                white_queen_side: false,
                black_king_side: false,
                black_queen_side: false,
            },
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            repetition_counts: HashMap::new(),
        }
    }

    fn setup_start_position(&mut self) {
        let back_rank = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for (file, kind) in back_rank.into_iter().enumerate() {
            self.set_piece(
                Pos::new(file as u8, 0),
                Some(Piece {
                    kind,
                    color: Color::White,
                }),
            );
            self.set_piece(
                Pos::new(file as u8, 1),
                Some(Piece {
                    kind: PieceKind::Pawn,
                    color: Color::White,
                }),
            );
            self.set_piece(
                Pos::new(file as u8, 6),
                Some(Piece {
                    kind: PieceKind::Pawn,
                    color: Color::Black,
                }),
            );
            self.set_piece(
                Pos::new(file as u8, 7),
                Some(Piece {
                    kind,
                    color: Color::Black,
                }),
            );
        }
    }

    pub fn piece_at(&self, pos: Pos) -> Option<Piece> {
        self.board[pos.to_index()]
    }

    pub fn set_piece(&mut self, pos: Pos, piece: Option<Piece>) {
        self.board[pos.to_index()] = piece;
    }

    pub fn is_empty(&self, pos: Pos) -> bool {
        self.piece_at(pos).is_none()
    }

    pub fn find_king(&self, color: Color) -> Option<Pos> {
        self.board.iter().enumerate().find_map(|(index, piece)| {
            let piece = (*piece)?;
            (piece.color == color && piece.kind == PieceKind::King).then(|| Pos::from_index(index))
        })
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let Some(king_pos) = self.find_king(color) else {
            return false;
        };
        self.is_square_attacked(king_pos, color.opposite())
    }

    pub fn board_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.turn.hash(&mut hasher);
        self.castling.hash(&mut hasher);
        self.en_passant_target.hash(&mut hasher);
        for cell in self.board {
            cell.hash(&mut hasher);
        }
        hasher.finish()
    }

    pub fn record_current_position(&mut self) {
        let hash = self.board_hash();
        *self.repetition_counts.entry(hash).or_insert(0) += 1;
    }

    pub fn current_repetition_count(&self) -> u32 {
        self.repetition_counts
            .get(&self.board_hash())
            .copied()
            .unwrap_or(0)
    }

    pub fn status(&self) -> GameStatus {
        if self.current_repetition_count() >= 3 {
            return GameStatus::DrawThreefold;
        }
        if self.halfmove_clock >= 100 {
            return GameStatus::DrawFiftyMove;
        }
        if self.is_insufficient_material() {
            return GameStatus::DrawInsufficientMaterial;
        }
        let legal = self.legal_moves();
        if legal.is_empty() {
            if self.is_in_check(self.turn) {
                return GameStatus::Checkmate(self.turn);
            }
            return GameStatus::Stalemate;
        }
        GameStatus::InProgress
    }

    pub fn make_move(&mut self, mv: ChessMove) -> Result<(), String> {
        let legal = self.legal_moves();
        if !legal.contains(&mv) {
            return Err("illegal move".to_string());
        }
        self.apply_move_unchecked(mv);
        self.record_current_position();
        Ok(())
    }

    pub fn apply_move_unchecked(&mut self, mv: ChessMove) {
        let moving_piece = self
            .piece_at(mv.from)
            .expect("apply_move_unchecked called without a moving piece");
        let captured_before = self.piece_at(mv.to);

        self.update_castling_rights_for_piece(mv.from, moving_piece);
        if let Some(captured) = captured_before {
            self.update_castling_rights_for_capture(mv.to, captured);
        }

        let is_en_passant = moving_piece.kind == PieceKind::Pawn
            && self.en_passant_target == Some(mv.to)
            && self.is_empty(mv.to);
        if is_en_passant {
            let capture_rank = match moving_piece.color {
                Color::White => mv.to.rank.saturating_sub(1),
                Color::Black => mv.to.rank + 1,
            };
            self.set_piece(Pos::new(mv.to.file, capture_rank), None);
        }

        self.set_piece(mv.from, None);

        let is_castling = moving_piece.kind == PieceKind::King
            && (i32::from(mv.from.file) - i32::from(mv.to.file)).abs() == 2;
        if is_castling {
            self.apply_castle_rook_move(mv);
        }

        let promoted_piece =
            if moving_piece.kind == PieceKind::Pawn && (mv.to.rank == 0 || mv.to.rank == 7) {
                Piece {
                    kind: mv.promotion.unwrap_or(PieceKind::Queen),
                    color: moving_piece.color,
                }
            } else {
                moving_piece
            };
        self.set_piece(mv.to, Some(promoted_piece));

        let pawn_moved = moving_piece.kind == PieceKind::Pawn;
        let captured = captured_before.is_some() || is_en_passant;
        self.halfmove_clock = if pawn_moved || captured {
            0
        } else {
            self.halfmove_clock + 1
        };

        self.en_passant_target = None;
        if moving_piece.kind == PieceKind::Pawn {
            let rank_delta = i32::from(mv.to.rank) - i32::from(mv.from.rank);
            if rank_delta.abs() == 2 {
                let mid_rank = ((u16::from(mv.from.rank) + u16::from(mv.to.rank)) / 2) as u8;
                self.en_passant_target = Some(Pos::new(mv.from.file, mid_rank));
            }
        }

        if self.turn == Color::Black {
            self.fullmove_number += 1;
        }
        self.turn = self.turn.opposite();
    }

    fn apply_castle_rook_move(&mut self, mv: ChessMove) {
        let rank = mv.from.rank;
        if mv.to.file == 6 {
            let rook_from = Pos::new(7, rank);
            let rook_to = Pos::new(5, rank);
            let rook = self.piece_at(rook_from);
            self.set_piece(rook_from, None);
            self.set_piece(rook_to, rook);
        } else if mv.to.file == 2 {
            let rook_from = Pos::new(0, rank);
            let rook_to = Pos::new(3, rank);
            let rook = self.piece_at(rook_from);
            self.set_piece(rook_from, None);
            self.set_piece(rook_to, rook);
        }
    }

    fn update_castling_rights_for_piece(&mut self, from: Pos, piece: Piece) {
        match (piece.color, piece.kind, from.file, from.rank) {
            (Color::White, PieceKind::King, _, _) => {
                self.castling.white_king_side = false;
                self.castling.white_queen_side = false;
            }
            (Color::Black, PieceKind::King, _, _) => {
                self.castling.black_king_side = false;
                self.castling.black_queen_side = false;
            }
            (Color::White, PieceKind::Rook, 0, 0) => self.castling.white_queen_side = false,
            (Color::White, PieceKind::Rook, 7, 0) => self.castling.white_king_side = false,
            (Color::Black, PieceKind::Rook, 0, 7) => self.castling.black_queen_side = false,
            (Color::Black, PieceKind::Rook, 7, 7) => self.castling.black_king_side = false,
            _ => {}
        }
    }

    fn update_castling_rights_for_capture(&mut self, pos: Pos, piece: Piece) {
        match (piece.color, piece.kind, pos.file, pos.rank) {
            (Color::White, PieceKind::Rook, 0, 0) => self.castling.white_queen_side = false,
            (Color::White, PieceKind::Rook, 7, 0) => self.castling.white_king_side = false,
            (Color::Black, PieceKind::Rook, 0, 7) => self.castling.black_queen_side = false,
            (Color::Black, PieceKind::Rook, 7, 7) => self.castling.black_king_side = false,
            _ => {}
        }
    }

    fn is_insufficient_material(&self) -> bool {
        let mut white_minor = 0;
        let mut black_minor = 0;
        let mut white_other = 0;
        let mut black_other = 0;

        for piece in self.board.into_iter().flatten() {
            match piece.kind {
                PieceKind::King => {}
                PieceKind::Bishop | PieceKind::Knight => {
                    if piece.color == Color::White {
                        white_minor += 1;
                    } else {
                        black_minor += 1;
                    }
                }
                _ => {
                    if piece.color == Color::White {
                        white_other += 1;
                    } else {
                        black_other += 1;
                    }
                }
            }
        }

        if white_other > 0 || black_other > 0 {
            return false;
        }
        (white_minor <= 1) && (black_minor <= 1)
    }
}
