use board::Board;

use crate::bitboard::{
    Bitboard, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, RANK1, RANK2, RANK3,
    RANK4, RANK5, RANK6, RANK7, RANK8,
};

mod bitboard;
mod board;
mod direction;
mod r#move;
mod move_generation;
mod piece;

const NUM_SQUARES: usize = 64;
const NUM_DIRECTIONS: usize = 8;

// Define directions (north, northeast, east, southeast, south, southwest, west, northwest)
const DIRECTIONS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn create_attack_rays() -> [[Bitboard; 8]; 64] {
    let mut attack_rays = [[0; 8]; 64];

    for square in 0..64 {
        let mut rays = [0; 8];

        for (dir, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
            let mut x = (square % 8) as i32;
            let mut y = (square / 8) as i32;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let target_square = y * 8 + x;
                rays[dir] |= 1u64 << target_square;

                x += dx;
                y += dy;
            }
            rays[dir] &= !(1 << square);
        }

        attack_rays[square] = rays;
    }

    return attack_rays;
}

// Bitboard representing the attacks for a given square and direction
// type AttackRays = [u64; NUM_DIRECTIONS];

fn main() {
    let board = Board::from_fen("r3k3/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 w KQkq - 0 1");
    let mut rook_attack_masks: Vec<u64> = Vec::new();
    let mut rook_attack_shifts: Vec<u32> = Vec::new();

    const ROOK_MAGICS: [Bitboard; 64] = [
        0x0080001020400080,
        0x0040001000200040,
        0x0080081000200080,
        0x0080040800100080,
        0x0080020400080080,
        0x0080010200040080,
        0x0080008001000200,
        0x0080002040800100,
        0x0000800020400080,
        0x0000400020005000,
        0x0000801000200080,
        0x0000800800100080,
        0x0000800400080080,
        0x0000800200040080,
        0x0000800100020080,
        0x0000800040800100,
        0x0000208000400080,
        0x0000404000201000,
        0x0000808010002000,
        0x0000808008001000,
        0x0000808004000800,
        0x0000808002000400,
        0x0000010100020004,
        0x0000020000408104,
        0x0000208080004000,
        0x0000200040005000,
        0x0000100080200080,
        0x0000080080100080,
        0x0000040080080080,
        0x0000020080040080,
        0x0000010080800200,
        0x0000800080004100,
        0x0000204000800080,
        0x0000200040401000,
        0x0000100080802000,
        0x0000080080801000,
        0x0000040080800800,
        0x0000020080800400,
        0x0000020001010004,
        0x0000800040800100,
        0x0000204000808000,
        0x0000200040008080,
        0x0000100020008080,
        0x0000080010008080,
        0x0000040008008080,
        0x0000020004008080,
        0x0000010002008080,
        0x0000004081020004,
        0x0000204000800080,
        0x0000200040008080,
        0x0000100020008080,
        0x0000080010008080,
        0x0000040008008080,
        0x0000020004008080,
        0x0000800100020080,
        0x0000800041000080,
        0x00FFFCDDFCED714A,
        0x007FFCDDFCED714A,
        0x003FFFCDFFD88096,
        0x0000040810002101,
        0x0001000204080011,
        0x0001000204000801,
        0x0001000082000401,
        0x0001FFFAABFAD1A2,
    ];

    let rays = create_attack_rays();

    for ray in rays[0]{
        println!("{:064b}", ray);
    }

    for i in 0..=63 {
        let mut rank_mask: u64 = 0;
        let mut file_mask: u64 = 0;
        let file = i % 8;
        let rank = i / 8;

        match rank {
            0 => rank_mask = RANK8,
            1 => rank_mask = RANK7,
            2 => rank_mask = RANK6,
            3 => rank_mask = RANK5,
            4 => rank_mask = RANK4,
            5 => rank_mask = RANK3,
            6 => rank_mask = RANK2,
            7 => rank_mask = RANK1,
            _ => (),
        }

        // Eerst
        //      North   0b0000000000000000000000000000000000000000000000000000000000000000
        //  NorthEast   0b0000000000000000000000000000000000000000000000000000000000000000
        //       East   0b0111111100000000000000000000000000000000000000000000000000000000
        //  SouthEast   0b0000000001000000001000000001000000001000000001000000001000000001
        //      South   0b0000000010000000100000001000000010000000100000001000000010000000
        //  SouthWest   0b0000000000000000000000000000000000000000000000000000000000000000
        //       West   0b0000000000000000000000000000000000000000000000000000000000000000
        //  NorthWest   0b0000000000000000000000000000000000000000000000000000000000000000

        //Twee (1 naar rechts)
        //      North   0b0000000000000000000000000000000000000000000000000000000000000000
        //  NorthEast   0b0000000000000000000000000000000000000000000000000000000000000000
        //       East   0b0011111100000000000000000000000000000000000000000000000000000000 org >> 1
        //  SouthEast   0b0000000000100000000100000000100000000100000000100000000100000000 org >> 1
        //      South   0b0000000001000000010000000100000001000000010000000100000001000000 org >> links
        //  SouthWest   0b0000000010000000000000000000000000000000000000000000000000000000 toggle bit 9 van next rank
        //       West   0b1000000000000000000000000000000000000000000000000000000000000000 toggle bit 1 van this rank
        //  NorthWest   0b0000000000000000000000000000000000000000000000000000000000000000

        match file {
            0 => file_mask = FILE_A,
            1 => file_mask = FILE_B,
            2 => file_mask = FILE_C,
            3 => file_mask = FILE_D,
            4 => file_mask = FILE_E,
            5 => file_mask = FILE_F,
            6 => file_mask = FILE_G,
            7 => file_mask = FILE_H,
            _ => (),
        }
        let edges = ((RANK1 | RANK8) & !rank_mask) | ((FILE_A | FILE_H) & !file_mask);
        let rook_attack = (rank_mask ^ file_mask) & !edges;
        rook_attack_masks.push(rook_attack);
        rook_attack_shifts.push(64 - rook_attack.count_ones());
    }

    println!("{}", board);
}
