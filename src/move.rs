use crate::piece::Pieces;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: u32,
    pub to: u32,
    pub piece: Pieces,
}