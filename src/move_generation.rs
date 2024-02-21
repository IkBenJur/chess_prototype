use crate::{
    bitboard::{Bitboard, FILE_A, FILE_B, FILE_G, FILE_H, RANK4, RANK5},
    board::Board,
    piece::Pieces,
    r#move::Move,
};

fn pawn_moves_single_push(board: &Board, white_turn: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut pawns = board.pieces[Pieces::Pawns as usize]
        & if white_turn {
            board.white_pieces
        } else {
            board.black_pieces
        };

    let empty_tiles = !(board.white_pieces ^ board.black_pieces);

    while pawns > 0 {
        let from: u32 = pawns.trailing_zeros();

        let pawn_move = if white_turn {
            1 << from >> 8 & empty_tiles
        } else {
            1 << from << 8 & empty_tiles
        };

        if pawn_move > 0 {
            let to = pawn_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Pawns,
            });
        }

        pawns &= !(1 << from);
    }

    return moves;
}

fn pawn_moves_double_push(board: &Board, white_turn: bool) -> Vec<Move> {
    let moves_one_push: Vec<Move> = pawn_moves_single_push(&board, white_turn);
    let mut moves: Vec<Move> = Vec::new();

    let empty_tiles = !(board.white_pieces ^ board.black_pieces);

    for one_push in moves_one_push {
        let pawn_move = if white_turn {
            0 | (1 << one_push.to) >> 8 & empty_tiles & RANK4
        } else {
            0 | (1 << one_push.to) << 8 & empty_tiles & RANK5
        };

        if pawn_move > 0 {
            let from = one_push.from;
            let to = pawn_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Pawns,
            });
        }
    }

    return moves;
}

fn knight_moves(board: &Board, white_turn: bool) -> Bitboard {
    let knights = board.pieces[Pieces::Knights as usize]
        & if white_turn {
            board.white_pieces
        } else {
            board.black_pieces
        };

    let knight_moves = (knights << 15)
        | (knights << 17)
        | (knights << 6)
        | (knights << 10)
        | (knights >> 10)
        | (knights >> 6)
        | (knights >> 17)
        | (knights >> 15);

    return knight_moves;
}

#[cfg(test)]
mod tests {
    use crate::move_generation::knight_moves;

    use super::*;

    #[test]
    fn single_push_pawn_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        let expected_white_moves = vec![
            Move {
                from: 28,
                to: 20,
                piece: Pieces::Pawns,
            },
            Move {
                from: 31,
                to: 23,
                piece: Pieces::Pawns,
            },
            Move {
                from: 38,
                to: 30,
                piece: Pieces::Pawns,
            },
            Move {
                from: 43,
                to: 35,
                piece: Pieces::Pawns,
            },
            Move {
                from: 48,
                to: 40,
                piece: Pieces::Pawns,
            },
            Move {
                from: 50,
                to: 42,
                piece: Pieces::Pawns,
            },
        ];
        let white_moves = pawn_moves_single_push(&board, true);
        assert_eq!(expected_white_moves, white_moves);

        let expected_black_moves = vec![
            Move {
                from: 11,
                to: 19,
                piece: Pieces::Pawns,
            },
            Move {
                from: 15,
                to: 23,
                piece: Pieces::Pawns,
            },
            Move {
                from: 25,
                to: 33,
                piece: Pieces::Pawns,
            },
        ];
        let black_moves = pawn_moves_single_push(&board, false);
        assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn double_push_pawn_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        let expected_white_moves = vec![
            Move {
                from: 48,
                to: 32,
                piece: Pieces::Pawns,
            },
            Move {
                from: 50,
                to: 34,
                piece: Pieces::Pawns,
            },
        ];
        let white_moves = pawn_moves_double_push(&board, true);
        assert_eq!(expected_white_moves, white_moves);

        let expected_black_moves: Vec<Move> = vec![];
        let black_moves = pawn_moves_double_push(&board, false);
        assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn find_knight_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");
        
        let white_moves: Bitboard = knight_moves(&board, true);
        assert_eq!(
            0b0000000000000000000101000010001010100001001100100001010100010000,
            white_moves
        );

        let black_moves: Bitboard = knight_moves(&board, false);
        assert_eq!(
            0b0000000000000000000000000101001010001100010000001000110001010010,
            black_moves
        );
    }
}
