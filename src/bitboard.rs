pub type Bitboard  = u64;

trait BitManipulation {
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
