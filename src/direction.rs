use std::{ops::Index, slice::SliceIndex};

use crate::bitboard::Bitboard;

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Index<Direction> for Vec<Bitboard> {
    type Output = Bitboard;

    fn index(&self, direction: Direction) -> &Self::Output {
        match direction {
            Direction::North => &self[0],
            Direction::NorthEast => &self[1],
            Direction::East => &self[2],
            Direction::SouthEast => &self[3],
            Direction::South => &self[4],
            Direction::SouthWest => &self[5],
            Direction::West => &self[6],
            Direction::NorthWest => &self[7],
        }
    }
}

impl Index<Direction> for [u64] {
    type Output = u64;

    fn index(&self, direction: Direction) -> &Self::Output {
        match direction {
            Direction::North => &self[0],
            Direction::NorthEast => &self[1],
            Direction::East => &self[2],
            Direction::SouthEast => &self[3],
            Direction::South => &self[4],
            Direction::SouthWest => &self[5],
            Direction::West => &self[6],
            Direction::NorthWest => &self[7],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index_with_direction() {
        let attack_rays: Vec<Bitboard> = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(attack_rays[Direction::North], 1);
        assert_eq!(attack_rays[Direction::NorthEast], 2);
        assert_eq!(attack_rays[Direction::East], 3);
        assert_eq!(attack_rays[Direction::SouthEast], 4);
        assert_eq!(attack_rays[Direction::South], 5);
        assert_eq!(attack_rays[Direction::SouthWest], 6);
        assert_eq!(attack_rays[Direction::West], 7);
        assert_eq!(attack_rays[Direction::NorthWest], 8);
    }
}
