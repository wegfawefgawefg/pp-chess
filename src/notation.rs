use crate::board::Game;
use crate::types::{ChessMove, PieceKind, Pos};

pub fn parse_move(input: &str) -> Result<ChessMove, String> {
    let trimmed = input.trim().to_ascii_lowercase();
    if trimmed.len() < 4 {
        return Err("move must look like e2e4".to_string());
    }

    let from = parse_square(&trimmed[0..2])?;
    let to = parse_square(&trimmed[2..4])?;
    let promotion = if trimmed.len() >= 5 {
        Some(parse_promotion(trimmed.as_bytes()[4] as char)?)
    } else {
        None
    };

    Ok(ChessMove {
        from,
        to,
        promotion,
    })
}

pub fn move_to_string(mv: ChessMove) -> String {
    let mut out = format!("{}{}", square_name(mv.from), square_name(mv.to));
    if let Some(promotion) = mv.promotion {
        out.push(match promotion {
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::Pawn | PieceKind::King => 'q',
        });
    }
    out
}

pub fn board_to_string(game: &Game) -> String {
    let mut out = String::new();
    out.push_str("  a b c d e f g h\n");
    for rank in (0..8).rev() {
        out.push(char::from(b'1' + rank as u8));
        out.push(' ');
        for file in 0..8 {
            let pos = Pos::new(file, rank);
            let ch = game.piece_at(pos).map_or('.', |piece| piece.symbol());
            out.push(ch);
            if file < 7 {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    out
}

fn parse_square(text: &str) -> Result<Pos, String> {
    let bytes = text.as_bytes();
    if bytes.len() != 2 {
        return Err("square must look like e2".to_string());
    }
    let file = bytes[0];
    let rank = bytes[1];
    if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
        return Err(format!("invalid square: {text}"));
    }
    Ok(Pos::new(file - b'a', rank - b'1'))
}

fn parse_promotion(ch: char) -> Result<PieceKind, String> {
    match ch {
        'q' => Ok(PieceKind::Queen),
        'r' => Ok(PieceKind::Rook),
        'b' => Ok(PieceKind::Bishop),
        'n' => Ok(PieceKind::Knight),
        _ => Err(format!("invalid promotion piece: {ch}")),
    }
}

fn square_name(pos: Pos) -> String {
    format!(
        "{}{}",
        char::from(b'a' + pos.file),
        char::from(b'1' + pos.rank)
    )
}
