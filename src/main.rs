mod bitboard;
mod board;
mod piece;



// struct Board {
//     pieces: [u64; 6],

//     black_pieces: u64,
//     white_pieces: u64,
// }

// impl Board {
//     fn new() -> Self {
//         Board {
//             pieces: [
//                 0b0000000011111111000000000000000000000000000000001111111100000000, //Pawns
//                 0b1000000100000000000000000000000000000000000000000000000010000001, //Rooks
//                 0b0100001000000000000000000000000000000000000000000000000001000010, //Knights
//                 0b0010010000000000000000000000000000000000000000000000000000100100, //Bishops
//                 0b0000100000000000000000000000000000000000000000000000000000001000, //Kings
//                 0b0001000000000000000000000000000000000000000000000000000000010000, //Queens
//             ],

//             white_pieces: 0b1111111111111111000000000000000000000000000000000000000000000000,
//             black_pieces: 0b0000000000000000000000000000000000000000000000001111111111111111,
//         }
//     }

//     fn from_fen(fen: &str) -> Self {
//         // let mut pawn_bitboard;
//         Board {
//             pieces: [
//                 0b0000000011111111000000000000000000000000000000001111111100000000, //Pawns
//                 0b1000000100000000000000000000000000000000000000000000000010000001, //Rooks
//                 0b0100001000000000000000000000000000000000000000000000000001000010, //Knights
//                 0b0010010000000000000000000000000000000000000000000000000000100100, //Bishops
//                 0b0000100000000000000000000000000000000000000000000000000000001000, //Kings
//                 0b0001000000000000000000000000000000000000000000000000000000010000, //Queens
//             ],

//             white_pieces: 0b1111111111111111000000000000000000000000000000000000000000000000,
//             black_pieces: 0b0000000000000000000000000000000000000000000000001111111111111111,
//         }
//     }

//     fn draw_pieces(&self, mut board_builder: Vec<char>, piece: Pieces, white: bool) -> Vec<char> {
//         let mut pieces = self.pieces[piece as usize] & if white {
//             self.white_pieces
//         } else {
//             self.black_pieces
//         };
    
//         while pieces > 0 {
//             let square_index = pieces.trailing_zeros() as usize;
//             board_builder[square_index] = piece_to_char(&piece, white);
//             pieces &= !(1u64 << square_index);
//         }
    
//         board_builder
//     }
    

//     fn draw_board(&self) -> String {
//         let board_builder: Vec<char> = vec!['X'; 64];

//         let board_builder =  self.draw_pieces(board_builder, Pieces::Pawns, true);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Pawns, false);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Rooks, true);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Rooks, false);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Knights, true);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Knights, false);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Bishops, true);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Bishops, false);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Queens, true);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Queens, false);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Kings, true);
//         let board_builder =  self.draw_pieces(board_builder, Pieces::Kings, false);
        
//         let mut result = String::new();

//         result.push_str("  a b c d e f g h\n");

//         for (i, &square) in board_builder.iter().enumerate() {
//             if i % 8 == 0 {
//                 result.push_str(&(8 - i / 8).to_string());
//                 result.push(' ');
//             }

//             result.push(square);
//             result.push(' ');

//             if i % 8 == 7 {
//                 result.push('\n');
//             }
//         }

//         return result;
//     }
// }

// fn piece_to_char(piece: &Pieces, white: bool) -> char {
//     match (piece, white) {
//         (Pieces::Pawns, true) => 'P',
//         (Pieces::Pawns, false) => 'p',
//         (Pieces::Rooks, true) => 'R',
//         (Pieces::Rooks, false) => 'r',
//         (Pieces::Knights, true) => 'N',
//         (Pieces::Knights, false) => 'n',
//         (Pieces::Bishops, true) => 'B',
//         (Pieces::Bishops, false) => 'b',
//         (Pieces::Kings, true) => 'K',
//         (Pieces::Kings, false) => 'k',
//         (Pieces::Queens, true) => 'Q',
//         (Pieces::Queens, false) => 'q',
//     }
// }

fn main() {
    // let board = Board::new();
    // println!("{}", board.draw_board());
}

