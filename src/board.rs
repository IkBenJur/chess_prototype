use crate::bitboard::Bitboard;
use crate::bitboard::BitManipulation;
use crate::piece::Pieces;

pub struct Board {
    pieces: [Bitboard; 6],
    
    black_pieces: Bitboard,
    white_pieces: Bitboard,
}

impl Board {
    fn new() -> Self{
        Board {
            pieces: [
                0 as Bitboard,
                0 as Bitboard,
                0 as Bitboard,
                0 as Bitboard,
                0 as Bitboard,
                0 as Bitboard,
            ],

            black_pieces: 0 as Bitboard,
            white_pieces: 0 as Bitboard,
        }
    }

    fn add_piece(mut self, piece: &char, index: u8) -> Board {
        
        match piece {
            'p' | 'P' => self.pieces[Pieces::Pawns as usize] = self.pieces[Pieces::Pawns as usize].toggle_square(index),
            'r' | 'R' => self.pieces[Pieces::Rooks as usize] = self.pieces[Pieces::Rooks as usize].toggle_square(index),
            'n' | 'N' => self.pieces[Pieces::Knights as usize] = self.pieces[Pieces::Knights as usize].toggle_square(index),
            'b' | 'B' => self.pieces[Pieces::Bishops as usize] = self.pieces[Pieces::Bishops as usize].toggle_square(index),
            'q' | 'Q' => self.pieces[Pieces::Queens as usize] = self.pieces[Pieces::Queens as usize].toggle_square(index),
            'k' | 'K' => self.pieces[Pieces::Kings as usize] = self.pieces[Pieces::Kings as usize].toggle_square(index),
            _ => println!("Unresolved character found")
        }

        if piece.is_uppercase() {
            self.white_pieces = self.white_pieces.toggle_square(index);
        } else {
            self.black_pieces = self.black_pieces.toggle_square(index);
        }

        return self;
    }

    fn from_fen(fen_string: &str) -> Board {
        let fen_parts: Vec<&str> = fen_string.split_whitespace().collect();
        let mut board = Board::new();

        let pieces = fen_parts.first().expect("Not valid FEN string");
        let pieces: Vec<char> = pieces.replace('/', "").chars().collect();
        let mut square_index: u8 = 0;

        for piece in pieces{
            match piece {
                'r' | 'n' | 'b' | 'q' | 'k' | 'p' => {
                    board = board.add_piece(&piece, square_index); 
                    square_index += 1;
                },
                'R' | 'N' | 'B' | 'Q' | 'K' | 'P' => {
                    board = board.add_piece(&piece, square_index);
                    square_index += 1;
                },
                '1'..='8' => {
                    let num = piece.to_digit(10).unwrap();
                    square_index += num as u8;
                },
                _ => println!("Unkown character found"),
            }
        }

        return board;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn from_fen_starting_pos() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(0b0000000011111111000000000000000000000000000000001111111100000000, board.pieces[Pieces::Pawns as usize]);
        assert_eq!(0b1000000100000000000000000000000000000000000000000000000010000001, board.pieces[Pieces::Rooks as usize]);
        assert_eq!(0b0100001000000000000000000000000000000000000000000000000001000010, board.pieces[Pieces::Knights as usize]);
        assert_eq!(0b0010010000000000000000000000000000000000000000000000000000100100, board.pieces[Pieces::Bishops as usize]);
        assert_eq!(0b0000100000000000000000000000000000000000000000000000000000001000, board.pieces[Pieces::Queens as usize]);
        assert_eq!(0b0001000000000000000000000000000000000000000000000000000000010000, board.pieces[Pieces::Kings as usize]);
        assert_eq!(0b1111111111111111000000000000000000000000000000000000000000000000, board.white_pieces);
        assert_eq!(0b0000000000000000000000000000000000000000000000001111111111111111, board.black_pieces);
    }

    #[test]
    fn from_fen_mid_game() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        assert_eq!(0b0000000000000101000010000100000010010010000000001010100100000000, board.pieces[Pieces::Pawns as usize]);
        assert_eq!(0b0000000000000000000000000000000000000000000000000000000010000001, board.pieces[Pieces::Rooks as usize]);
        assert_eq!(0b0000000000000000000000000000000000001000001000010100000000000000, board.pieces[Pieces::Knights as usize]);
        assert_eq!(0b0100000000000000000000000000000000000000000000000001000000000100, board.pieces[Pieces::Bishops as usize]);
        assert_eq!(0b0000000100000000000000000000000000000000000000000000000000000000, board.pieces[Pieces::Queens as usize]);
        assert_eq!(0b0000000000010000000000000000000000000000000000000000000000001000, board.pieces[Pieces::Kings as usize]);
        assert_eq!(0b0000000000010101000010000100000010011000000000000101000000000000, board.white_pieces);
        assert_eq!(0b0100000100000000000000000000000000000010001000011010100110001101, board.black_pieces);
    }
}