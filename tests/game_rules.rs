use pp_chess::board::Game;
use pp_chess::notation::parse_move;
use pp_chess::types::{Color, GameStatus, Piece, PieceKind, Pos};

fn play(game: &mut Game, uci: &str) {
    let mv = parse_move(uci).unwrap();
    game.make_move(mv).unwrap();
}

#[test]
fn starts_with_twenty_legal_moves() {
    let game = Game::new();
    assert_eq!(game.legal_moves().len(), 20);
}

#[test]
fn pawn_double_push_sets_en_passant_square() {
    let mut game = Game::new();
    play(&mut game, "e2e4");
    assert_eq!(game.en_passant_target, Some(Pos::new(4, 2)));
}

#[test]
fn en_passant_capture_works() {
    let mut game = Game::empty();
    game.turn = Color::White;
    game.set_piece(
        Pos::new(4, 4),
        Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        }),
    );
    game.set_piece(
        Pos::new(3, 4),
        Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::Black,
        }),
    );
    game.set_piece(
        Pos::new(4, 0),
        Some(Piece {
            kind: PieceKind::King,
            color: Color::White,
        }),
    );
    game.set_piece(
        Pos::new(4, 7),
        Some(Piece {
            kind: PieceKind::King,
            color: Color::Black,
        }),
    );
    game.en_passant_target = Some(Pos::new(3, 5));

    play(&mut game, "e5d6");
    assert!(game.piece_at(Pos::new(3, 4)).is_none());
    assert_eq!(game.piece_at(Pos::new(3, 5)).unwrap().color, Color::White);
}

#[test]
fn castling_moves_rook_too() {
    let mut game = Game::empty();
    game.turn = Color::White;
    game.castling.white_king_side = true;
    game.set_piece(
        Pos::new(4, 0),
        Some(Piece {
            kind: PieceKind::King,
            color: Color::White,
        }),
    );
    game.set_piece(
        Pos::new(7, 0),
        Some(Piece {
            kind: PieceKind::Rook,
            color: Color::White,
        }),
    );
    game.set_piece(
        Pos::new(4, 7),
        Some(Piece {
            kind: PieceKind::King,
            color: Color::Black,
        }),
    );

    play(&mut game, "e1g1");
    assert_eq!(game.piece_at(Pos::new(6, 0)).unwrap().kind, PieceKind::King);
    assert_eq!(game.piece_at(Pos::new(5, 0)).unwrap().kind, PieceKind::Rook);
}

#[test]
fn promotion_works() {
    let mut game = Game::empty();
    game.turn = Color::White;
    game.set_piece(
        Pos::new(0, 6),
        Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        }),
    );
    game.set_piece(
        Pos::new(4, 0),
        Some(Piece {
            kind: PieceKind::King,
            color: Color::White,
        }),
    );
    game.set_piece(
        Pos::new(4, 7),
        Some(Piece {
            kind: PieceKind::King,
            color: Color::Black,
        }),
    );

    play(&mut game, "a7a8q");
    assert_eq!(
        game.piece_at(Pos::new(0, 7)).unwrap().kind,
        PieceKind::Queen
    );
}

#[test]
fn fools_mate_is_checkmate() {
    let mut game = Game::new();
    play(&mut game, "f2f3");
    play(&mut game, "e7e5");
    play(&mut game, "g2g4");
    play(&mut game, "d8h4");

    assert_eq!(game.status(), GameStatus::Checkmate(Color::White));
}
