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

#[derive(Debug)]
struct Bitboards {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
}

impl Bitboards {
    fn new() -> Self {
        Self {
            // 8 pawns on ranks 2 for white
            white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            // 2 knights on b1 and g1 for white
            white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            // 2 bishops on c1 and f1 for white
            white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            // 2 rooks on a1 and h1 for white
            white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            // 1 queen on d1 for white
            white_queens: 0b0000000000000000000000000000000000000000000000000000000000010000,
            // 1 king on e1 for white
            white_king: 0b0000000000000000000000000000000000000000000000000000000000001000,

            // 8 pawns on ranks 7 for black
            black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            // 2 knights on b8 and g8 for black
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            // 2 bishops on c8 and f8 for black
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            // 2 rooks on a8 and h8 for black
            black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            // 1 queen on d8 for black
            black_queens: 0b0001000000000000000000000000000000000000000000000000000000000000,
            // 1 king on e8 for black
            black_king: 0b0000100000000000000000000000000000000000000000000000000000000000,
        }
    }

    // Check if a particular square is occupied
    fn is_occupied(&self, pos: usize) -> bool {
        let all_pieces = self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
            | self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king;
        (all_pieces >> pos) & 1 == 1
    }

    fn is_attacked_by_white_pawns(&self, pos: usize) -> bool {
        if pos >= 16 {
            // to avoid underflow
            return ((self.white_pawns >> 9) & 1 << (pos - 9) != 0) || // Capture to the left
                   ((self.white_pawns >> 7) & 1 << (pos - 7) != 0); // Capture to the right
        }
        false
    }

    fn is_attacked_by_white_knights(&self, pos: usize) -> bool;
    fn is_attacked_by_white_bishops(&self, pos: usize) -> bool;
    fn is_attacked_by_white_rooks(&self, pos: usize) -> bool;
    fn is_attacked_by_white_queens(&self, pos: usize) -> bool;
    fn is_attacked_by_white_king(&self, pos: usize) -> bool;
    fn is_attacked_by_black_pawns(&self, pos: usize) -> bool;
    fn is_attacked_by_black_knights(&self, pos: usize) -> bool;
    fn is_attacked_by_black_bishops(&self, pos: usize) -> bool;
    fn is_attacked_by_black_rooks(&self, pos: usize) -> bool;
    fn is_attacked_by_black_queens(&self, pos: usize) -> bool;
    fn is_attacked_by_black_king(&self, pos: usize) -> bool;

    fn is_attacked_by_black_pieces(&self, pos: usize) -> bool {
        self.is_attacked_by_black_pawns(pos)
            || self.is_attacked_by_black_knights(pos)
            || self.is_attacked_by_black_bishops(pos)
            || self.is_attacked_by_black_rooks(pos)
            || self.is_attacked_by_black_queens(pos)
            || self.is_attacked_by_black_king(pos)
    }

    fn is_attacked_by_white_pieces(&self, pos: usize) -> bool {
        self.is_attacked_by_white_pawns(pos)
            || self.is_attacked_by_white_knights(pos)
            || self.is_attacked_by_white_bishops(pos)
            || self.is_attacked_by_white_rooks(pos)
            || self.is_attacked_by_white_queens(pos)
            || self.is_attacked_by_white_king(pos)
    }

    fn is_attacked(&self, pos: usize) -> bool {
        self.is_attacked_by_white_pieces(pos) || self.is_attacked_by_black_pieces(pos)
    }

    // Check if either king is in check
    // Note: This is a placeholder, actual implementation would involve checking moves
    fn is_in_check(&self) -> bool {
        // TODO: Implement
        false
    }

    // Check if coordinates are on the board
    fn is_coord_on_board(x: usize, y: usize) -> bool {
        x < 8 && y < 8
    }

    fn print_board(&self) {
        println!("+-----------------+");
        for rank in (0..8).rev() {
            let mut rank_str = format!("{} |", rank + 1); // print rank number on the left side
            for file in 0..8 {
                let pos = rank * 8 + file;
                let mut piece_char = '.'; // represent empty squares with '.'

                if (self.white_pawns >> pos) & 1 == 1 {
                    piece_char = WHITE_PAWN;
                } else if (self.white_knights >> pos) & 1 == 1 {
                    piece_char = WHITE_KNIGHT;
                } else if (self.white_bishops >> pos) & 1 == 1 {
                    piece_char = WHITE_BISHOP;
                } else if (self.white_rooks >> pos) & 1 == 1 {
                    piece_char = WHITE_ROOK;
                } else if (self.white_queens >> pos) & 1 == 1 {
                    piece_char = WHITE_QUEEN;
                } else if (self.white_king >> pos) & 1 == 1 {
                    piece_char = WHITE_KING;
                } else if (self.black_pawns >> pos) & 1 == 1 {
                    piece_char = BLACK_PAWN;
                } else if (self.black_knights >> pos) & 1 == 1 {
                    piece_char = BLACK_KNIGHT;
                } else if (self.black_bishops >> pos) & 1 == 1 {
                    piece_char = BLACK_BISHOP;
                } else if (self.black_rooks >> pos) & 1 == 1 {
                    piece_char = BLACK_ROOK;
                } else if (self.black_queens >> pos) & 1 == 1 {
                    piece_char = BLACK_QUEEN;
                } else if (self.black_king >> pos) & 1 == 1 {
                    piece_char = BLACK_KING;
                }

                rank_str.push(' ');
                rank_str.push(piece_char);
            }
            rank_str.push('|');
            println!("{}", rank_str);
        }
        println!("+-----------------+");
        println!("    a b c d e f g h"); // print file letters at the bottom
    }
}

fn main() {
    let boards = Bitboards::new();
    boards.print_board();
}
