#[derive(Clone, Copy)]
enum Pieces {
    Pawns,
    Rooks,
    Bishops,
    Knights,
    Kings,
    Queens,
}
struct Board {
    white_pawns: u64,
    black_pawns: u64,
    white_rooks: u64,
    black_rooks: u64,
    white_knights: u64,
    black_knights: u64,
    white_bishops: u64,
    black_bishops: u64,
    white_queens: u64,
    black_queens: u64,
    white_king: u64,
    black_king: u64,
}

impl Board {
    fn new() -> Self {
        Board {
            white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            white_queens: 0b0000000000000000000000000000000000000000000000000000000000010000,
            black_queens: 0b0000100000000000000000000000000000000000000000000000000000000000,
            white_king: 0b0000000000000000000000000000000000000000000000000000000000001000,
            black_king: 0b00001000000000000000000000000000000000000000000000000000000000000,
        }
    }
}

fn draw_board(board: &Board) -> String {
    let mut board_builder: Vec<Vec<char>> = vec![vec!['X'; 8]; 8];

    for row in 0..8 {
        for col in 0..8 {
            if board.white_pawns & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'P';
            }

            if board.white_rooks & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'R';
            }

            if board.white_knights & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'N';
            }
            
            if board.white_bishops & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'B';
            }
            
            if board.white_queens & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'Q';
            }
            
            if board.white_king & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'K';
            }
            
            if board.black_pawns & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'p';
            }
            
            if board.black_rooks & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'r';
            }
            
            if board.black_knights & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'n';
            }
            
            if board.black_bishops & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'b';
            }
            
            if board.black_queens & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'q';
            }
            
            if board.black_king & (1u64 << (col + row * 8)) != 0 {
                board_builder[row][col] = 'k';
            }
        }
    } 
    
    let mut result = String::new();

    result.push_str("  a b c d e f g h\n");

    for (i, row) in board_builder.iter().enumerate() {
        result.push_str(&(8 - i).to_string());
        result.push(' ');

        for &square in row {
            result.push(square);
            result.push(' ');
        }

        result.push('\n');
    }

    result
}

fn main() {
    let board = Board::new();
    let board_representation = draw_board(&board);
    println!("{}", board_representation);
}

