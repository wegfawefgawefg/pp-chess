use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

static WHITE_PAWN: char = 'P';
static WHITE_KNIGHT: char = 'N';
static WHITE_BISHOP: char = 'B';
static WHITE_ROOK: char = 'R';
static WHITE_QUEEN: char = 'Q';
static WHITE_KING: char = 'K';

static BLACK_PAWN: char = 'p';
static BLACK_KNIGHT: char = 'n';
static BLACK_BISHOP: char = 'b';
static BLACK_ROOK: char = 'r';
static BLACK_QUEEN: char = 'q';
static BLACK_KING: char = 'k';

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Move {
    pub from: Pos,
    pub to: Pos,
}

impl Move {
    pub fn new(from: Pos, to: Pos) -> Self {
        Self { from, to }
    }
}

impl Hash for Piece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.piece_type.hash(state);
        self.color.hash(state);
    }
}

#[derive(Debug)]
pub struct ChessGame {
    pub current_turn: Color,
    pub num_moves: u32,
    pub board: Vec<Vec<Option<Piece>>>,
    pub board_history: HashMap<u64, u32>,

    pub white_pawn_0_moved: bool,
    pub white_pawn_1_moved: bool,
    pub white_pawn_2_moved: bool,
    pub white_pawn_3_moved: bool,
    pub white_pawn_4_moved: bool,
    pub white_pawn_5_moved: bool,
    pub white_pawn_6_moved: bool,
    pub white_pawn_7_moved: bool,

    pub white_left_rook_moved: bool,
    pub white_right_rook_moved: bool,
    pub white_king_moved: bool,

    pub black_pawn_0_moved: bool,
    pub black_pawn_1_moved: bool,
    pub black_pawn_2_moved: bool,
    pub black_pawn_3_moved: bool,
    pub black_pawn_4_moved: bool,
    pub black_pawn_5_moved: bool,
    pub black_pawn_6_moved: bool,
    pub black_pawn_7_moved: bool,

    pub black_left_rook_moved: bool,
    pub black_right_rook_moved: bool,
    pub black_king_moved: bool,
}

impl ChessGame {
    fn new() -> Self {
        let starter_board = vec![
            vec![
                Some(Piece {
                    piece_type: PieceType::Rook,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Knight,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Queen,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::King,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Knight,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Rook,
                    color: Color::Black,
                }),
            ],
            vec![
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                }),
            ],
            vec![None; 8],
            vec![None; 8],
            vec![None; 8],
            vec![None; 8],
            vec![
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                }),
            ],
            vec![
                Some(Piece {
                    piece_type: PieceType::Rook,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Knight,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Queen,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::King,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Knight,
                    color: Color::White,
                }),
                Some(Piece {
                    piece_type: PieceType::Rook,
                    color: Color::White,
                }),
            ],
        ];

        Self {
            current_turn: Color::White,
            num_moves: 0,
            board: starter_board,
            board_history: HashMap::new(),

            white_pawn_0_moved: false,
            white_pawn_1_moved: false,
            white_pawn_2_moved: false,
            white_pawn_3_moved: false,
            white_pawn_4_moved: false,
            white_pawn_5_moved: false,
            white_pawn_6_moved: false,
            white_pawn_7_moved: false,
            white_left_rook_moved: false,
            white_right_rook_moved: false,
            white_king_moved: false,
            black_pawn_0_moved: false,
            black_pawn_1_moved: false,
            black_pawn_2_moved: false,
            black_pawn_3_moved: false,
            black_pawn_4_moved: false,
            black_pawn_5_moved: false,
            black_pawn_6_moved: false,
            black_pawn_7_moved: false,
            black_left_rook_moved: false,
            black_right_rook_moved: false,
            black_king_moved: false,
        }
    }

    fn hash_board(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(piece) => piece.hash(&mut hasher),
                    None => 0.hash(&mut hasher),
                }
            }
        }
        hasher.finish()
    }

    // Function to check if a board has been seen three times
    pub fn is_board_repeated(&mut self) -> bool {
        let board_hash = self.hash_board();
        let count = self.board_history.entry(board_hash).or_insert(0);
        *count += 1;

        *count >= 3
    }

    // Check if a particular square is occupied
    pub fn is_occupied(&self, pos: Pos) -> bool {
        self.board[pos.y][pos.x].is_some()
    }

    pub fn get_unfiltered_available_moves(&self, turn: Color) -> Vec<Pos> {
        let mut moves = Vec::new();

        // iterate through every square
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.board[y][x] {
                    if piece.color != turn {
                        continue;
                    }

                    

                    match piece.piece_type {
                        PieceType::Pawn => {
                            // check if the spot in front of you is occupied by any piece
                            match turn {
                                Color::Black => {
                                    let down_one = Move::new(Pos::new(x, y + 1);
                                    let two_below = Pos::new(x, y + 2);
                                    moves.push(two_below)
                                }
                                Color::White => {
                                    let above = Pos::new(x, y + 1);
                                    let two_above = Pos::new(x, y + 2);
                                    moves.push(above);
                                    moves.push(two_above)
                                }
                            }
                        }
                        PieceType::Bishop => {
                            for dy in -8..8 {
                                for dx in -8..8 {
                                    let m = Pos::new(x + dx, y + dy);
                                    moves.push(m)
                                }
                            }
                        }
                        _ => {}
                    };
                }
            }
        }
        moves
    }
    // fn is_attacked(&self, pos: usize) -> bool {
    //     self.is_attacked_by_white_pieces(pos) || self.is_attacked_by_black_pieces(pos)
    // }

    // // Check if either king is in check
    // // Note: This is a placeholder, actual implementation would involve checking moves
    // fn is_in_check(&self) -> bool {
    //     // TODO: Implement
    //     false
    // }

    // // Check if coordinates are on the board
    // fn is_coord_on_board(x: usize, y: usize) -> bool {
    //     x < 8 && y < 8
    // }

    pub fn apply_move(&mut self, m: Move) {
        let piece = self.board[m.from.y - 1][m.from.x];
        self.board[m.from.y][m.from.x] = None;
        self.board[m.to.y][m.to.x] = piece;

        self.num_moves += 1;
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        // increment board history
        let board_hash = self.hash_board();
        let count = self.board_history.entry(board_hash).or_insert(0);
        *count += 1;
    }

    pub fn print_board(&self) {
        for (r, row) in self.board.iter().enumerate() {
            print!("{}| ", 8 - r);
            for cell in row {
                let piece_char = match cell {
                    Some(piece) => match (piece.piece_type, piece.color) {
                        (PieceType::Pawn, Color::White) => 'P',
                        (PieceType::Knight, Color::White) => 'N',
                        (PieceType::Bishop, Color::White) => 'B',
                        (PieceType::Rook, Color::White) => 'R',
                        (PieceType::Queen, Color::White) => 'Q',
                        (PieceType::King, Color::White) => 'K',
                        (PieceType::Pawn, Color::Black) => 'p',
                        (PieceType::Knight, Color::Black) => 'n',
                        (PieceType::Bishop, Color::Black) => 'b',
                        (PieceType::Rook, Color::Black) => 'r',
                        (PieceType::Queen, Color::Black) => 'q',
                        (PieceType::King, Color::Black) => 'k',
                    },
                    None => '.',
                };
                print!("{} ", piece_char);
            }
            println!(); // New line at the end of each row
        }
        println!("   ---------------");
        println!("   a b c d e f g h\n");
    }
}

fn main() {
    let mut game = ChessGame::new();
    game.print_board();

    let m = Move {
        from: Pos { x: 4, y: 1 },
        to: Pos { x: 4, y: 2 },
    };
    game.apply_move(m);
    game.print_board();

    let m = Move {
        from: Pos { x: 3, y: 6 },
        to: Pos { x: 3, y: 5 },
    };
    game.apply_move(m);
    game.print_board();
}
