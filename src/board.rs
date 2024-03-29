use crate::bitboard::BitManipulation;
use crate::bitboard::Bitboard;
use crate::piece::Pieces;

pub struct Board {
    pub game_state: GameState,

    pub pieces: [Bitboard; 6],

    pub black_pieces: Bitboard,
    pub white_pieces: Bitboard,
}

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub white_turn: bool,
    pub en_passant: Option<u32>,
    pub castling: Castling,
    pub half_moves: u32,
    pub full_moves: u32,
}

#[derive(PartialEq, Debug)]
pub struct Castling {
    pub white_queen_side: bool,
    pub white_king_side: bool,
    pub black_queen_side: bool,
    pub black_king_side: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            game_state: GameState {
                white_turn: true,
                en_passant: None,
                castling: Castling {
                    white_queen_side: true,
                    white_king_side: true,
                    black_queen_side: true,
                    black_king_side: true,
                },
                half_moves: 0,
                full_moves: 0,
            },

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
            'p' | 'P' => {
                self.pieces[Pieces::Pawns as usize] =
                    self.pieces[Pieces::Pawns as usize].toggle_square(index)
            }
            'r' | 'R' => {
                self.pieces[Pieces::Rooks as usize] =
                    self.pieces[Pieces::Rooks as usize].toggle_square(index)
            }
            'n' | 'N' => {
                self.pieces[Pieces::Knights as usize] =
                    self.pieces[Pieces::Knights as usize].toggle_square(index)
            }
            'b' | 'B' => {
                self.pieces[Pieces::Bishops as usize] =
                    self.pieces[Pieces::Bishops as usize].toggle_square(index)
            }
            'q' | 'Q' => {
                self.pieces[Pieces::Queens as usize] =
                    self.pieces[Pieces::Queens as usize].toggle_square(index)
            }
            'k' | 'K' => {
                self.pieces[Pieces::Kings as usize] =
                    self.pieces[Pieces::Kings as usize].toggle_square(index)
            }
            _ => println!("Unresolved character found"),
        }

        if piece.is_uppercase() {
            self.white_pieces = self.white_pieces.toggle_square(index);
        } else {
            self.black_pieces = self.black_pieces.toggle_square(index);
        }

        return self;
    }

    pub fn from_fen(fen_string: &str) -> Board {
        let fen_parts: Vec<&str> = fen_string.split_whitespace().collect();
        let mut board = Board::new();

        let pieces = fen_parts.first().expect("Not valid FEN string");
        let pieces: Vec<char> = pieces.replace('/', "").chars().collect();
        let mut square_index: u8 = 0;

        for piece in pieces {
            match piece {
                'r' | 'n' | 'b' | 'q' | 'k' | 'p' => {
                    board = board.add_piece(&piece, square_index);
                    square_index += 1;
                }
                'R' | 'N' | 'B' | 'Q' | 'K' | 'P' => {
                    board = board.add_piece(&piece, square_index);
                    square_index += 1;
                }
                '1'..='8' => {
                    let num = piece.to_digit(10).unwrap();
                    square_index += num as u8;
                }
                _ => println!("Unkown character found"),
            }
        }

        let is_white_active_color = fen_parts
            .get(1)
            .expect("Invalid FEN string. Active color missing.")
            .eq(&"w");

        let castling_rights = fen_parts
            .get(2)
            .expect("Invalid FEN string. Castling rights missing");

        let castling_right_white_king = castling_rights.contains("K");
        let castling_right_white_queen = castling_rights.contains("Q");

        let castling_right_black_king = castling_rights.contains("k");
        let castling_right_black_queen = castling_rights.contains("q");

        let castling_rights = Castling {
            white_king_side: castling_right_white_king,
            white_queen_side: castling_right_white_queen,
            black_king_side: castling_right_black_king,
            black_queen_side: castling_right_black_queen,
        };

        let en_passant = fen_parts
            .get(3)
            .expect("Invalid FEN string. En passant missing");
        let en_passant_square_num: Option<u32> = if en_passant.contains('-') {
            None
        } else {
            Some(square_to_num(en_passant.to_string()))
        };

        let half_moves: u32 = fen_parts
            .get(4)
            .expect("Invalid FEN string. Half moves missing")
            .parse()
            .unwrap();

        let full_moves: u32 = fen_parts
            .get(5)
            .expect("Invalid FEN string. Full moves missing")
            .parse()
            .unwrap();

        board.game_state = GameState {
            white_turn: is_white_active_color,
            castling: castling_rights,
            en_passant: en_passant_square_num,
            half_moves,
            full_moves,
        };

        return board;
    }
}

fn square_to_num(square: String) -> u32 {
    let chars: Vec<char> = square.chars().collect();
    let col_char = chars.get(0).expect("Invalid square");
    let row_num = chars
        .get(1)
        .expect("Invalid square")
        .to_digit(10)
        .expect("Invalid square");

    let col_num = match col_char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        _ => panic!("Invalid square num"),
    };

    return (64 - (row_num * 8)) + col_num;
}

fn map_bitboard_to_string(
    mut bitboard: Bitboard,
    mut board_string: Vec<char>,
    piece_char: char,
) -> Vec<char> {
    while bitboard > 0 {
        let square_index = bitboard.trailing_zeros() as usize;
        board_string[square_index] = piece_char;
        bitboard &= !(1 << square_index);
    }

    return board_string;
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let white_pawns = self.white_pieces & self.pieces[Pieces::Pawns as usize];
        let black_pawns = self.black_pieces & self.pieces[Pieces::Pawns as usize];

        let white_rooks = self.white_pieces & self.pieces[Pieces::Rooks as usize];
        let black_rooks = self.black_pieces & self.pieces[Pieces::Rooks as usize];

        let white_knights = self.white_pieces & self.pieces[Pieces::Knights as usize];
        let black_knights = self.black_pieces & self.pieces[Pieces::Knights as usize];

        let white_bishops = self.white_pieces & self.pieces[Pieces::Bishops as usize];
        let black_bishops = self.black_pieces & self.pieces[Pieces::Bishops as usize];

        let white_queens = self.white_pieces & self.pieces[Pieces::Queens as usize];
        let black_queens = self.black_pieces & self.pieces[Pieces::Queens as usize];

        let white_king = self.white_pieces & self.pieces[Pieces::Kings as usize];
        let black_king = self.black_pieces & self.pieces[Pieces::Kings as usize];

        let board_builder: Vec<char> = vec!['.'; 64];

        let board_builder = map_bitboard_to_string(white_pawns, board_builder, 'P');
        let board_builder = map_bitboard_to_string(black_pawns, board_builder, 'p');

        let board_builder = map_bitboard_to_string(white_rooks, board_builder, 'R');
        let board_builder = map_bitboard_to_string(black_rooks, board_builder, 'r');

        let board_builder = map_bitboard_to_string(white_knights, board_builder, 'N');
        let board_builder = map_bitboard_to_string(black_knights, board_builder, 'n');

        let board_builder = map_bitboard_to_string(white_bishops, board_builder, 'B');
        let board_builder = map_bitboard_to_string(black_bishops, board_builder, 'b');

        let board_builder = map_bitboard_to_string(white_queens, board_builder, 'Q');
        let board_builder = map_bitboard_to_string(black_queens, board_builder, 'q');

        let board_builder = map_bitboard_to_string(white_king, board_builder, 'K');
        let board_builder = map_bitboard_to_string(black_king, board_builder, 'k');

        let mut result = String::new();
        result.push_str("  a b c d e f g h\n");

        for (i, &square) in board_builder.iter().enumerate() {
            if i % 8 == 0 {
                result.push_str(&(8 - i / 8).to_string());
                result.push(' ');
            }

            result.push(square);
            result.push(' ');

            if i % 8 == 7 {
                result.push('\n');
            }
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fen_starting_pos() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(
            0b0000000011111111000000000000000000000000000000001111111100000000,
            board.pieces[Pieces::Pawns as usize]
        );
        assert_eq!(
            0b1000000100000000000000000000000000000000000000000000000010000001,
            board.pieces[Pieces::Rooks as usize]
        );
        assert_eq!(
            0b0100001000000000000000000000000000000000000000000000000001000010,
            board.pieces[Pieces::Knights as usize]
        );
        assert_eq!(
            0b0010010000000000000000000000000000000000000000000000000000100100,
            board.pieces[Pieces::Bishops as usize]
        );
        assert_eq!(
            0b0000100000000000000000000000000000000000000000000000000000001000,
            board.pieces[Pieces::Queens as usize]
        );
        assert_eq!(
            0b0001000000000000000000000000000000000000000000000000000000010000,
            board.pieces[Pieces::Kings as usize]
        );
        assert_eq!(
            0b1111111111111111000000000000000000000000000000000000000000000000,
            board.white_pieces
        );
        assert_eq!(
            0b0000000000000000000000000000000000000000000000001111111111111111,
            board.black_pieces
        );

        let expected_game_state = GameState {
            white_turn: true,
            castling: Castling {
                white_king_side: true,
                white_queen_side: true,
                black_king_side: true,
                black_queen_side: true,
            },
            en_passant: None,
            half_moves: 0,
            full_moves: 1,
        };

        assert_eq!(board.game_state, expected_game_state);
    }

    #[test]
    fn from_fen_mid_game() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b KQkq - 0 1");

        assert_eq!(
            0b0000000000000101000010000100000010010010000000001010100100000000,
            board.pieces[Pieces::Pawns as usize]
        );
        assert_eq!(
            0b0000000000000000000000000000000000000000000000000000000010000001,
            board.pieces[Pieces::Rooks as usize]
        );
        assert_eq!(
            0b0000000000000000000000000000000000001000001000010100000000000000,
            board.pieces[Pieces::Knights as usize]
        );
        assert_eq!(
            0b0100000000000000000000000000000000000000000000000001000000000100,
            board.pieces[Pieces::Bishops as usize]
        );
        assert_eq!(
            0b0000000100000000000000000000000000000000000000000000000000000000,
            board.pieces[Pieces::Queens as usize]
        );
        assert_eq!(
            0b0000000000010000000000000000000000000000000000000000000000001000,
            board.pieces[Pieces::Kings as usize]
        );
        assert_eq!(
            0b0000000000010101000010000100000010011000000000000101000000000000,
            board.white_pieces
        );
        assert_eq!(
            0b0100000100000000000000000000000000000010001000011010100110001101,
            board.black_pieces
        );

        let expected_game_state = GameState {
            white_turn: false,
            castling: Castling {
                white_king_side: true,
                white_queen_side: true,
                black_king_side: true,
                black_queen_side: true,
            },
            en_passant: None,
            half_moves: 0,
            full_moves: 1,
        };
        assert_eq!(board.game_state, expected_game_state);
    }

    #[test]
    fn en_passant_and_castling_rights() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQ c5 30 44");
        
        let expected_game_state = GameState {
            white_turn: false,
            castling: Castling {
                white_king_side: true,
                white_queen_side: true,
                black_king_side: false,
                black_queen_side: false,
            },
            en_passant: Some(26),
            half_moves: 30,
            full_moves: 44,
        };
        assert_eq!(board.game_state, expected_game_state);
    }

    #[test]
    fn board_display() {
        let result_string = "  a b c d e f g h\n8 r n b q k b n r \n7 p p p p p p p p \n6 . . . . . . . . \n5 . . . . . . . . \n4 . . . . . . . . \n3 . . . . . . . . \n2 P P P P P P P P \n1 R N B Q K B N R \n";

        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(result_string, format!("{board}"));
    }

    #[test]
    fn transfer_square_to_num() {
        let num = square_to_num("a8".to_string());
        assert_eq!(num, 0);

        let num = square_to_num("e4".to_string());
        assert_eq!(num, 36);

        let num = square_to_num("h2".to_string());
        assert_eq!(num, 55);
    }
}
