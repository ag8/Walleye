pub use crate::board::*;
pub use crate::board::{PieceColor::*, PieceKind::*};

/*
    Evaluation function based on https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function
*/

#[rustfmt::skip]
const MG_PAWN_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_PAWN_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_PAWN_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_PAWN_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_KNIGHT_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_KNIGHT_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_KNIGHT_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_KNIGHT_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_BISHOP_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_BISHOP_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_BISHOP_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_BISHOP_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_ROOK_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_ROOK_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_ROOK_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_ROOK_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_QUEEN_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_QUEEN_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_QUEEN_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_QUEEN_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const MG_KING_TABLE_WHITE: [[i32; 8]; 8] = [
    [-1000, -1000, -1000, -1000, -1000, -1000, -1000, -1000],
    [-1000, -1000, -1000, -1000, -1000, -1000, -1000, -1000],
    [-1000, -1000, -1000, -1000, -1000, -1000, -1000, -1000],
    [-1000, -1000, -1000, -1000, -1000, -1000, -1000, -1000],
    [-50, -300, -500, -500, -500, -500, -300, -50],
    [0, -100, -100, -100, -100, -100, -100, 0],
    [10, 10, 10, 0, 0, 10, 10, 10],
    [50, 50, 50, 50, 50, 50, 50, 50]
];

#[rustfmt::skip]
const MG_KING_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_KING_TABLE_WHITE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

#[rustfmt::skip]
const EG_KING_TABLE_BLACK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
];

fn mg_table(kind: PieceKind, color: PieceColor) -> &'static [[i32; 8]; 8] {
    if color == White {
        match kind {
            Pawn => &MG_PAWN_TABLE_WHITE,
            Bishop => &MG_BISHOP_TABLE_WHITE,
            Knight => &MG_KNIGHT_TABLE_WHITE,
            Rook => &MG_ROOK_TABLE_WHITE,
            King => &MG_KING_TABLE_WHITE,
            Queen => &MG_QUEEN_TABLE_WHITE,
        }
    } else {
        match kind {
            Pawn => &MG_PAWN_TABLE_BLACK,
            Bishop => &MG_BISHOP_TABLE_BLACK,
            Knight => &MG_KNIGHT_TABLE_BLACK,
            Rook => &MG_ROOK_TABLE_BLACK,
            King => &MG_KING_TABLE_BLACK,
            Queen => &MG_QUEEN_TABLE_BLACK,
        }
    }
}

fn eg_table(kind: PieceKind, color: PieceColor) -> &'static [[i32; 8]; 8] {
    if color == White {
        match kind {
            Pawn => &EG_PAWN_TABLE_WHITE,
            Bishop => &EG_BISHOP_TABLE_WHITE,
            Knight => &EG_KNIGHT_TABLE_WHITE,
            Rook => &EG_ROOK_TABLE_WHITE,
            King => &EG_KING_TABLE_WHITE,
            Queen => &EG_QUEEN_TABLE_WHITE,
        }
    } else {
        match kind {
            Pawn => &EG_PAWN_TABLE_BLACK,
            Bishop => &EG_BISHOP_TABLE_BLACK,
            Knight => &EG_KNIGHT_TABLE_BLACK,
            Rook => &EG_ROOK_TABLE_BLACK,
            King => &EG_KING_TABLE_BLACK,
            Queen => &EG_QUEEN_TABLE_BLACK,
        }
    }
}

fn mg_piece_val(kind: PieceKind) -> i32 {
    match kind {
        Pawn => 82,
        Knight => 337,
        Bishop => 365,
        Rook => 477,
        Queen => 1025,
        King => 0,
    }
}

fn eg_piece_val(kind: PieceKind) -> i32 {
    match kind {
        Pawn => 94,
        Knight => 281,
        Bishop => 297,
        Rook => 512,
        Queen => 936,
        King => 0,
    }
}

fn game_phase_val(kind: PieceKind) -> i32 {
    match kind {
        Pawn => 0,
        Knight => 1,
        Bishop => 1,
        Rook => 2,
        Queen => 4,
        King => 0,
    }
}

/*
    Return how good a position is from the perspective of the current player
*/
pub fn get_evaluation(board: &BoardState) -> i32 {
    let mut white_mg = 0;
    let mut black_mg = 0;
    let mut white_eg = 0;
    let mut black_eg = 0;
    let mut game_phase = 0;

    for row in BOARD_START..BOARD_END {
        for col in BOARD_START..BOARD_END {
            if let Square::Full(Piece { color, kind }) = board.board[row][col] {
                game_phase += game_phase_val(kind);
                if color == White {
                    white_mg +=
                        mg_table(kind, color)[row - BOARD_START][col - BOARD_START] + mg_piece_val(kind);
                    white_eg +=
                        eg_table(kind, color)[row - BOARD_START][col - BOARD_START] + eg_piece_val(kind);
                } else {
                    black_mg += mg_table(kind, color)[row - BOARD_START][col - BOARD_START] + mg_piece_val(kind);
                    black_eg += eg_table(kind, color)[row - BOARD_START][col - BOARD_START] + eg_piece_val(kind);
                }
            }
        }
    }

    let mut white_king_col = 0;
    let mut white_king_row = 0;
    for row in BOARD_START..BOARD_END {
        for col in BOARD_START..BOARD_END {
            if let Square::Full(Piece { color, kind }) = board.board[row][col] {
                if color == White {
                    if kind == King {
                        white_king_col = col;
                        white_king_row = row;
                    }
                }
            }
        }
    }

    for row in BOARD_START..BOARD_END {
        for col in BOARD_START..BOARD_END {
            if let Square::Full(Piece { color, kind }) = board.board[row][col] {
                if col == white_king_col {
                    if row > white_king_row {
                        if kind == Pawn {
                            if color == White {
                                white_mg += 150;
                            } else {
                                let is_emp1 = board.board[row - 1][col - 1].is_empty();
                                let is_emp2 = board.board[row - 1][col + 1].is_empty();
                                if is_emp1 && is_emp2 {
                                    white_mg += 400;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mg_score;
    let eg_score;
    if board.to_move == White {
        mg_score = white_mg - black_mg;
        eg_score = white_eg - black_eg;
    } else {
        mg_score = black_mg - white_mg;
        eg_score = black_eg - white_eg;
    }

    let mut mg_phase = game_phase;

    // /* in case of early promotion */
    if mg_phase > 24 {
        mg_phase = 24;
    }
    let eg_phase = 24 - mg_phase;
    (mg_score * mg_phase + eg_score * eg_phase) / 24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_evaluation_equal() {
        let b = BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
        assert_eq!(get_evaluation(&b), 0);
    }
}
