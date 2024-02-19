use crate::{
    bitboard::{Bitboard, RANK4, RANK5},
    board::Board,
    piece::Pieces,
};

fn pawn_moves_single_push(board: &Board, white_turn: bool) -> Bitboard {
    let pawns = board.pieces[Pieces::Pawns as usize]
        & if white_turn {
            board.white_pieces
        } else {
            board.black_pieces
        };

    let empty_tiles = !(board.white_pieces ^ board.black_pieces);

    let pawn_moves_single_push = if white_turn {
        pawns >> 8 & empty_tiles
    } else {
        pawns << 8 & empty_tiles
    };

    return pawn_moves_single_push;
}

fn pawn_moves_double_push(board: &Board, white_turn: bool) -> Bitboard {
    let pawn_moves_single_push = pawn_moves_single_push(&board, white_turn);
    let empty_tiles = !(board.white_pieces ^ board.black_pieces);

    let pawn_moves_double_push = if white_turn {
        pawn_moves_single_push >> 8 & empty_tiles & RANK4
    } else {
        pawn_moves_single_push << 8 & empty_tiles & RANK5
    };

    return pawn_moves_double_push;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_push_pawn_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        let white_moves: Bitboard = pawn_moves_single_push(&board, true);
        assert_eq!(
            0b0000000000000000000001010000100001000000100100000000000000000000,
            white_moves
        );

        let black_moves: Bitboard = pawn_moves_single_push(&board, false);
        assert_eq!(
            0b0000000000000000000000000000001000000000100010000000000000000000,
            black_moves
        );
    }

    #[test]
    fn double_push_pawn_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        let white_moves: Bitboard = pawn_moves_double_push(&board, true);
        assert_eq!(
            0b0000000000000000000000000000010100000000000000000000000000000000,
            white_moves
        );

        let black_moves: Bitboard = pawn_moves_double_push(&board, false);
        assert_eq!(
            0b0000000000000000000000000000000000000000000000000000000000000000,
            black_moves
        );
    }
}
