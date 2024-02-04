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

