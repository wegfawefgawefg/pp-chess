#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::White => "White",
            Self::Black => "Black",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn symbol(self) -> char {
        let ch = match self.kind {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        };
        match self.color {
            Color::White => ch.to_ascii_uppercase(),
            Color::Black => ch,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub file: u8,
    pub rank: u8,
}

impl Pos {
    pub fn new(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }

    pub fn try_from_i32(file: i32, rank: i32) -> Option<Self> {
        if !(0..8).contains(&file) || !(0..8).contains(&rank) {
            return None;
        }
        Some(Self::new(file as u8, rank as u8))
    }

    pub fn to_index(self) -> usize {
        usize::from(self.rank) * 8 + usize::from(self.file)
    }

    pub fn from_index(index: usize) -> Self {
        Self {
            file: (index % 8) as u8,
            rank: (index / 8) as u8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChessMove {
    pub from: Pos,
    pub to: Pos,
    pub promotion: Option<PieceKind>,
}

impl ChessMove {
    pub fn new(from: Pos, to: Pos) -> Self {
        Self {
            from,
            to,
            promotion: None,
        }
    }

    pub fn with_promotion(from: Pos, to: Pos, promotion: PieceKind) -> Self {
        Self {
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Checkmate(Color),
    Stalemate,
    DrawThreefold,
    DrawFiftyMove,
    DrawInsufficientMaterial,
}
