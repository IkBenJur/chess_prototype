pub type Bitboard  = u64;

pub const RANK1: Bitboard = 0b1111111100000000000000000000000000000000000000000000000000000000;
pub const RANK2: Bitboard = RANK1 >> 8;
pub const RANK3: Bitboard = RANK1 >> (8 * 2);
pub const RANK4: Bitboard = RANK1 >> (8 * 3);
pub const RANK5: Bitboard = RANK1 >> (8 * 4);
pub const RANK6: Bitboard = RANK1 >> (8 * 5);
pub const RANK7: Bitboard = RANK1 >> (8 * 6);
pub const RANK8: Bitboard = RANK1 >> (8 * 7);

pub const FILE_A: Bitboard = 0b1000000010000000100000001000000010000000100000001000000010000000;
pub const FILE_B: Bitboard = FILE_A >> 1;
pub const FILE_C: Bitboard = FILE_A >> 2;
pub const FILE_D: Bitboard = FILE_A >> 3;
pub const FILE_E: Bitboard = FILE_A >> 4;
pub const FILE_F: Bitboard = FILE_A >> 5;
pub const FILE_G: Bitboard = FILE_A >> 6;
pub const FILE_H: Bitboard = FILE_A >> 7;

pub trait BitManipulation {
    fn toggle_square(&self, index: u8) -> Bitboard;
}

impl BitManipulation for Bitboard {
    fn toggle_square(&self, index: u8) -> Bitboard {
        return *self ^ 1 << index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_square_on_bitboard() {
        let bitboard: Bitboard = 0;
        let bitboard = bitboard.toggle_square(2);
        assert_eq!(bitboard, 4);
    }

    #[test]
    fn toggle_square() {
        let bitboard: Bitboard = 0;
        let bitboard = bitboard.toggle_square(2).toggle_square(2);
        assert_eq!(bitboard, 0);
    }
}
