use crate::{
    bitboard::{self, Bitboard, FILE_A, FILE_B, FILE_G, FILE_H, RANK4, RANK5},
    board::{self, Board},
    direction::Direction,
    piece::Pieces,
    r#move::Move,
};

fn pawn_moves_single_push(board: &Board, white_turn: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut pawns = board.pieces[Pieces::Pawns as usize]
        & if white_turn {
            board.white_pieces
        } else {
            board.black_pieces
        };

    let empty_tiles = !(board.white_pieces ^ board.black_pieces);

    while pawns > 0 {
        let from: u32 = pawns.trailing_zeros();

        let pawn_move = if white_turn {
            1 << from >> 8 & empty_tiles
        } else {
            1 << from << 8 & empty_tiles
        };

        if pawn_move > 0 {
            let to = pawn_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Pawns,
            });
        }

        pawns &= !(1 << from);
    }

    return moves;
}

fn pawn_moves_double_push(board: &Board, white_turn: bool) -> Vec<Move> {
    let moves_one_push: Vec<Move> = pawn_moves_single_push(&board, white_turn);
    let mut moves: Vec<Move> = Vec::new();

    let empty_tiles = !(board.white_pieces ^ board.black_pieces);

    for one_push in moves_one_push {
        let pawn_move = if white_turn {
            0 | (1 << one_push.to) >> 8 & empty_tiles & RANK4
        } else {
            0 | (1 << one_push.to) << 8 & empty_tiles & RANK5
        };

        if pawn_move > 0 {
            let from = one_push.from;
            let to = pawn_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Pawns,
            });
        }
    }

    return moves;
}

fn knight_moves(board: &Board, white_turn: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut knights = board.pieces[Pieces::Knights as usize]
        & if white_turn {
            board.white_pieces
        } else {
            board.black_pieces
        };

    let own_pieces = if white_turn {
        board.white_pieces
    } else {
        board.black_pieces
    };

    while knights > 0 {
        let from: u32 = knights.trailing_zeros();
        let knight_position_bitboard = 1 << from;

        let knight_move = (knight_position_bitboard << 17) & (!FILE_A & !own_pieces);
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        let knight_move = ((knight_position_bitboard << 10) & (!FILE_A & !FILE_B)) & !own_pieces;
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        let knight_move = ((knight_position_bitboard >> 6) & (!FILE_A & !FILE_B)) & !own_pieces;
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        let knight_move = ((knight_position_bitboard >> 15) & (!FILE_A)) & !own_pieces;
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        let knight_move = ((knight_position_bitboard << 15) & (!FILE_H)) & !own_pieces;
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        let knight_move = ((knight_position_bitboard << 6) & (!FILE_G & !FILE_H)) & !own_pieces;
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        let knight_move = ((knight_position_bitboard >> 10) & (!FILE_G & !FILE_H)) & !own_pieces;
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        let knight_move = ((knight_position_bitboard >> 17) & (!FILE_H)) & !own_pieces;
        if knight_move > 0 {
            let to = knight_move.trailing_zeros();
            moves.push(Move {
                from,
                to,
                piece: Pieces::Knights,
            });
        }

        knights &= !(1 << from);
    }

    return moves;
}

const COMPASS_DIRECTIONS: [(i32, i32); 8] = [
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

        for (dir, &(dx, dy)) in COMPASS_DIRECTIONS.iter().enumerate() {
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

const ATTACK_RAYS: [[Bitboard; 8]; 64] = [
    [0, 0, 254, 9241421688590303744, 72340172838076672, 0, 0, 0],
    [0, 0, 252, 36099303471055872, 144680345676153344, 256, 1, 0],
    [0, 0, 248, 141012904183808, 289360691352306688, 66048, 3, 0],
    [0, 0, 240, 550831656960, 578721382704613376, 16909312, 7, 0],
    [
        0,
        0,
        224,
        2151686144,
        1157442765409226752,
        4328785920,
        15,
        0,
    ],
    [
        0,
        0,
        192,
        8404992,
        2314885530818453504,
        1108169199616,
        31,
        0,
    ],
    [
        0,
        0,
        128,
        32768,
        4629771061636907008,
        283691315109888,
        63,
        0,
    ],
    [0, 0, 0, 0, 9259542123273814016, 72624976668147712, 127, 0],
    [1, 2, 65024, 4620710844295151616, 72340172838076416, 0, 0, 0],
    [
        2,
        4,
        64512,
        9241421688590303232,
        144680345676152832,
        65536,
        256,
        1,
    ],
    [
        4,
        8,
        63488,
        36099303471054848,
        289360691352305664,
        16908288,
        768,
        2,
    ],
    [
        8,
        16,
        61440,
        141012904181760,
        578721382704611328,
        4328783872,
        1792,
        4,
    ],
    [
        16,
        32,
        57344,
        550831652864,
        1157442765409222656,
        1108169195520,
        3840,
        8,
    ],
    [
        32,
        64,
        49152,
        2151677952,
        2314885530818445312,
        283691315101696,
        7936,
        16,
    ],
    [
        64,
        128,
        32768,
        8388608,
        4629771061636890624,
        72624976668131328,
        16128,
        32,
    ],
    [
        128,
        0,
        0,
        0,
        9259542123273781248,
        145249953336262656,
        32512,
        64,
    ],
    [
        257,
        516,
        16646144,
        2310355422147510272,
        72340172838010880,
        0,
        0,
        0,
    ],
    [
        514,
        1032,
        16515072,
        4620710844295020544,
        144680345676021760,
        16777216,
        65536,
        256,
    ],
    [
        1028,
        2064,
        16252928,
        9241421688590041088,
        289360691352043520,
        4328521728,
        196608,
        513,
    ],
    [
        2056,
        4128,
        15728640,
        36099303470530560,
        578721382704087040,
        1108168671232,
        458752,
        1026,
    ],
    [
        4112,
        8256,
        14680064,
        141012903133184,
        1157442765408174080,
        283691314053120,
        983040,
        2052,
    ],
    [
        8224,
        16512,
        12582912,
        550829555712,
        2314885530816348160,
        72624976666034176,
        2031616,
        4104,
    ],
    [
        16448,
        32768,
        8388608,
        2147483648,
        4629771061632696320,
        145249953332068352,
        4128768,
        8208,
    ],
    [
        32896,
        0,
        0,
        0,
        9259542123265392640,
        290499906664136704,
        8323072,
        16416,
    ],
    [
        65793,
        132104,
        4261412864,
        1155177711056977920,
        72340172821233664,
        0,
        0,
        0,
    ],
    [
        131586,
        264208,
        4227858432,
        2310355422113955840,
        144680345642467328,
        4294967296,
        16777216,
        65536,
    ],
    [
        263172,
        528416,
        4160749568,
        4620710844227911680,
        289360691284934656,
        1108101562368,
        50331648,
        131328,
    ],
    [
        526344,
        1056832,
        4026531840,
        9241421688455823360,
        578721382569869312,
        283691179835392,
        117440512,
        262657,
    ],
    [
        1052688,
        2113664,
        3758096384,
        36099303202095104,
        1157442765139738624,
        72624976397598720,
        251658240,
        525314,
    ],
    [
        2105376,
        4227072,
        3221225472,
        141012366262272,
        2314885530279477248,
        145249952795197440,
        520093696,
        1050628,
    ],
    [
        4210752,
        8388608,
        2147483648,
        549755813888,
        4629771060558954496,
        290499905590394880,
        1056964608,
        2101256,
    ],
    [
        8421504,
        0,
        0,
        0,
        9259542121117908992,
        580999811180789760,
        2130706432,
        4202512,
    ],
    [
        16843009,
        33818640,
        1090921693184,
        577588851233521664,
        72340168526266368,
        0,
        0,
        0,
    ],
    [
        33686018,
        67637280,
        1082331758592,
        1155177702467043328,
        144680337052532736,
        1099511627776,
        4294967296,
        16777216,
    ],
    [
        67372036,
        135274560,
        1065151889408,
        2310355404934086656,
        289360674105065472,
        283673999966208,
        12884901888,
        33619968,
    ],
    [
        134744072,
        270549120,
        1030792151040,
        4620710809868173312,
        578721348210130944,
        72624942037860352,
        30064771072,
        67240192,
    ],
    [
        269488144,
        541097984,
        962072674304,
        9241421619736346624,
        1157442696420261888,
        145249884075720704,
        64424509440,
        134480385,
    ],
    [
        538976288,
        1082130432,
        824633720832,
        36099165763141632,
        2314885392840523776,
        290499768151441408,
        133143986176,
        268960770,
    ],
    [
        1077952576,
        2147483648,
        549755813888,
        140737488355328,
        4629770785681047552,
        580999536302882816,
        270582939648,
        537921540,
    ],
    [
        2155905152,
        0,
        0,
        0,
        9259541571362095104,
        1161999072605765632,
        545460846592,
        1075843080,
    ],
    [
        4311810305,
        8657571872,
        279275953455104,
        288793326105133056,
        72339069014638592,
        0,
        0,
        0,
    ],
    [
        8623620610,
        17315143744,
        277076930199552,
        577586652210266112,
        144678138029277184,
        281474976710656,
        1099511627776,
        4294967296,
    ],
    [
        17247241220,
        34630287488,
        272678883688448,
        1155173304420532224,
        289356276058554368,
        72620543991349248,
        3298534883328,
        8606711808,
    ],
    [
        34494482440,
        69260574720,
        263882790666240,
        2310346608841064448,
        578712552117108736,
        145241087982698496,
        7696581394432,
        17213489152,
    ],
    [
        68988964880,
        138521083904,
        246290604621824,
        4620693217682128896,
        1157425104234217472,
        290482175965396992,
        16492674416640,
        34426978560,
    ],
    [
        137977929760,
        277025390592,
        211106232532992,
        9241386435364257792,
        2314850208468434944,
        580964351930793984,
        34084860461056,
        68853957121,
    ],
    [
        275955859520,
        549755813888,
        140737488355328,
        36028797018963968,
        4629700416936869888,
        1161928703861587968,
        69269232549888,
        137707914242,
    ],
    [
        551911719040,
        0,
        0,
        0,
        9259400833873739776,
        2323857407723175936,
        139637976727552,
        275415828484,
    ],
    [
        1103823438081,
        2216338399296,
        71494644084506624,
        144115188075855872,
        72057594037927936,
        0,
        0,
        0,
    ],
    [
        2207646876162,
        4432676798592,
        70931694131085312,
        288230376151711744,
        144115188075855872,
        72057594037927936,
        281474976710656,
        1099511627776,
    ],
    [
        4415293752324,
        8865353596928,
        69805794224242688,
        576460752303423488,
        288230376151711744,
        144115188075855872,
        844424930131968,
        2203318222848,
    ],
    [
        8830587504648,
        17730707128320,
        67553994410557440,
        1152921504606846976,
        576460752303423488,
        288230376151711744,
        1970324836974592,
        4406653222912,
    ],
    [
        17661175009296,
        35461397479424,
        63050394783186944,
        2305843009213693952,
        1152921504606846976,
        576460752303423488,
        4222124650659840,
        8813306511360,
    ],
    [
        35322350018592,
        70918499991552,
        54043195528445952,
        4611686018427387904,
        2305843009213693952,
        1152921504606846976,
        8725724278030336,
        17626613022976,
    ],
    [
        70644700037184,
        140737488355328,
        36028797018963968,
        9223372036854775808,
        4611686018427387904,
        2305843009213693952,
        17732923532771328,
        35253226045953,
    ],
    [
        141289400074368,
        0,
        0,
        0,
        9223372036854775808,
        4611686018427387904,
        35747322042253312,
        70506452091906,
    ],
    [
        282578800148737,
        567382630219904,
        18302628885633695744,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        565157600297474,
        1134765260439552,
        18158513697557839872,
        0,
        0,
        0,
        72057594037927936,
        281474976710656,
    ],
    [
        1130315200594948,
        2269530520813568,
        17870283321406128128,
        0,
        0,
        0,
        216172782113783808,
        564049465049088,
    ],
    [
        2260630401189896,
        4539061024849920,
        17293822569102704640,
        0,
        0,
        0,
        504403158265495552,
        1128103225065472,
    ],
    [
        4521260802379792,
        9078117754732544,
        16140901064495857664,
        0,
        0,
        0,
        1080863910568919040,
        2256206466908160,
    ],
    [
        9042521604759584,
        18155135997837312,
        13835058055282163712,
        0,
        0,
        0,
        2233785415175766016,
        4512412933881856,
    ],
    [
        18085043209519168,
        36028797018963968,
        9223372036854775808,
        0,
        0,
        0,
        4539628424389459968,
        9024825867763968,
    ],
    [
        36170086419038336,
        0,
        0,
        0,
        0,
        0,
        9151314442816847872,
        18049651735527937,
    ],
];

fn rook_moves(board: &Board, white_turn: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let mut rooks = if white_turn {
        board.pieces[Pieces::Rooks as usize] & board.white_pieces
    } else {
        board.pieces[Pieces::Rooks as usize] & board.black_pieces
    };

    let all_pieces = board.black_pieces | board.white_pieces;

    while rooks > 0 {
        let from = rooks.trailing_zeros();
        let directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        for direction in directions {
            let attack_ray = ATTACK_RAYS[from as usize][direction];
            // println!("{:064b}", attack_ray);
            let blockers = attack_ray & all_pieces;
            // println!("{:064b}", blockers);
            let mut rook_moves;

            if blockers > 0 {
                // println!("{:064b}\n", ATTACK_RAYS[blockers.trailing_zeros() as usize][direction]);
                rook_moves = match direction {
                    Direction::North => {
                        attack_ray ^ ATTACK_RAYS[63 - blockers.leading_zeros() as usize][direction]
                    }
                    Direction::East => {
                        attack_ray ^ ATTACK_RAYS[blockers.trailing_zeros() as usize][direction]
                    }
                    Direction::South => {
                        attack_ray ^ ATTACK_RAYS[blockers.trailing_zeros() as usize][direction]
                    }
                    Direction::West => {
                        attack_ray ^ ATTACK_RAYS[63 - blockers.leading_zeros() as usize][direction]
                    }
                    _ => 0,
                };
            } else {
                rook_moves = attack_ray;
            }

            rook_moves &= if white_turn {
                !board.white_pieces
            } else {
                !board.black_pieces
            };

            while rook_moves > 0 {
                let to = rook_moves.trailing_zeros();
                moves.push(Move {
                    from,
                    to,
                    piece: Pieces::Rooks,
                });

                rook_moves &= !(1 << to)
            }

            // println!("{:064b}\n", moves);
        }
        // let rook_attack_ray = ROOK_ATTACK_RAYS[from as usize];
        // let blockers = rook_attack_ray & all_pieces;
        // let blocker_ray = println!("\n{:064b}", rooks & !(1 << from));

        rooks &= !(1 << from)
    }

    return moves;
}

fn bishop_moves(board: &Board, white_turn: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let mut bishops = if white_turn {
        board.pieces[Pieces::Bishops as usize] & board.white_pieces
    } else {
        board.pieces[Pieces::Bishops as usize] & board.black_pieces
    };

    let all_pieces = board.black_pieces | board.white_pieces;

    while bishops > 0 {
        let from = bishops.trailing_zeros();
        let directions = [
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ];

        for direction in directions {
            let attack_ray = ATTACK_RAYS[from as usize][direction];
            let blockers = attack_ray & all_pieces;
            let mut bishop_moves;

            if blockers > 0 {
                bishop_moves = match direction {
                    Direction::NorthEast => {
                        attack_ray ^ ATTACK_RAYS[63 - blockers.leading_zeros() as usize][direction]
                    }
                    Direction::SouthEast => {
                        attack_ray ^ ATTACK_RAYS[blockers.trailing_zeros() as usize][direction]
                    }
                    Direction::SouthWest => {
                        attack_ray ^ ATTACK_RAYS[blockers.trailing_zeros() as usize][direction]
                    }
                    Direction::NorthWest => {
                        attack_ray ^ ATTACK_RAYS[63 - blockers.leading_zeros() as usize][direction]
                    }
                    _ => 0,
                };
            } else {
                bishop_moves = attack_ray;
            }

            bishop_moves &= if white_turn {
                !board.white_pieces
            } else {
                !board.black_pieces
            };

            while bishop_moves > 0 {
                let to = bishop_moves.trailing_zeros();
                moves.push(Move {
                    from,
                    to,
                    piece: Pieces::Bishops,
                });

                bishop_moves &= !(1 << to)
            }
        }

        bishops &= !(1 << from)
    }

    return moves;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_push_pawn_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        let expected_white_moves = vec![
            Move {
                from: 28,
                to: 20,
                piece: Pieces::Pawns,
            },
            Move {
                from: 31,
                to: 23,
                piece: Pieces::Pawns,
            },
            Move {
                from: 38,
                to: 30,
                piece: Pieces::Pawns,
            },
            Move {
                from: 43,
                to: 35,
                piece: Pieces::Pawns,
            },
            Move {
                from: 48,
                to: 40,
                piece: Pieces::Pawns,
            },
            Move {
                from: 50,
                to: 42,
                piece: Pieces::Pawns,
            },
        ];
        let white_moves = pawn_moves_single_push(&board, true);
        assert_eq!(expected_white_moves, white_moves);

        let expected_black_moves = vec![
            Move {
                from: 11,
                to: 19,
                piece: Pieces::Pawns,
            },
            Move {
                from: 15,
                to: 23,
                piece: Pieces::Pawns,
            },
            Move {
                from: 25,
                to: 33,
                piece: Pieces::Pawns,
            },
        ];
        let black_moves = pawn_moves_single_push(&board, false);
        assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn double_push_pawn_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        let expected_white_moves = vec![
            Move {
                from: 48,
                to: 32,
                piece: Pieces::Pawns,
            },
            Move {
                from: 50,
                to: 34,
                piece: Pieces::Pawns,
            },
        ];
        let white_moves = pawn_moves_double_push(&board, true);
        assert_eq!(expected_white_moves, white_moves);

        let expected_black_moves: Vec<Move> = vec![];
        let black_moves = pawn_moves_double_push(&board, false);
        assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn find_knight_moves() {
        let board = Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQkq - 0 1");

        let expected_white_moves = vec![
            Move {
                from: 14,
                to: 29,
                piece: Pieces::Knights,
            },
            Move {
                from: 14,
                to: 20,
                piece: Pieces::Knights,
            },
            Move {
                from: 14,
                to: 4,
                piece: Pieces::Knights,
            },
            Move {
                from: 27,
                to: 44,
                piece: Pieces::Knights,
            },
            Move {
                from: 27,
                to: 37,
                piece: Pieces::Knights,
            },
            Move {
                from: 27,
                to: 21,
                piece: Pieces::Knights,
            },
            Move {
                from: 27,
                to: 42,
                piece: Pieces::Knights,
            },
            Move {
                from: 27,
                to: 33,
                piece: Pieces::Knights,
            },
            Move {
                from: 27,
                to: 17,
                piece: Pieces::Knights,
            },
            Move {
                from: 27,
                to: 10,
                piece: Pieces::Knights,
            },
        ];
        let white_moves = knight_moves(&board, true);
        assert_eq!(expected_white_moves, white_moves);

        let expected_black_moves = vec![
            Move {
                from: 16,
                to: 33,
                piece: Pieces::Knights,
            },
            Move {
                from: 16,
                to: 26,
                piece: Pieces::Knights,
            },
            Move {
                from: 16,
                to: 10,
                piece: Pieces::Knights,
            },
            Move {
                from: 16,
                to: 1,
                piece: Pieces::Knights,
            },
            Move {
                from: 21,
                to: 38,
                piece: Pieces::Knights,
            },
            Move {
                from: 21,
                to: 31,
                piece: Pieces::Knights,
            },
            Move {
                from: 21,
                to: 6,
                piece: Pieces::Knights,
            },
            Move {
                from: 21,
                to: 36,
                piece: Pieces::Knights,
            },
            Move {
                from: 21,
                to: 27,
                piece: Pieces::Knights,
            },
            Move {
                from: 21,
                to: 4,
                piece: Pieces::Knights,
            },
        ];
        let black_moves = knight_moves(&board, false);
        assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn find_rook_moves() {
        let board = Board::from_fen("8/8/8/1k2N1B1/3p4/1K2p1Q1/1R5R/5b1r w - - 0 1");

        let expected_white_moves = vec![
            Move {
                from: 49,
                to: 50,
                piece: Pieces::Rooks,
            },
            Move {
                from: 49,
                to: 51,
                piece: Pieces::Rooks,
            },
            Move {
                from: 49,
                to: 52,
                piece: Pieces::Rooks,
            },
            Move {
                from: 49,
                to: 53,
                piece: Pieces::Rooks,
            },
            Move {
                from: 49,
                to: 54,
                piece: Pieces::Rooks,
            },
            Move {
                from: 49,
                to: 57,
                piece: Pieces::Rooks,
            },
            Move {
                from: 49,
                to: 48,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 7,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 15,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 23,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 31,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 39,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 47,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 63,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 50,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 51,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 52,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 53,
                piece: Pieces::Rooks,
            },
            Move {
                from: 55,
                to: 54,
                piece: Pieces::Rooks,
            },
        ];
        let white_moves = rook_moves(&board, true);
        assert_eq!(expected_white_moves, white_moves);

        let expected_black_moves = vec![
            Move {
                from: 63,
                to: 55,
                piece: Pieces::Rooks,
            },
            Move {
                from: 63,
                to: 62,
                piece: Pieces::Rooks,
            },
        ];
        let black_moves = rook_moves(&board, false);
        assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn find_bishop_moves() {
        let board = Board::from_fen("8/2prkp2/1nQpB1p1/1p2Pn2/p4Pq1/1b3N2/3B3K/2R2R2 w - - 0 1");
        
        let expected_white_moves: Vec<Move> = vec![
            Move {
                from: 20,
                to: 13,
                piece: Pieces::Bishops,
            },
            Move {
                from: 20,
                to: 29,
                piece: Pieces::Bishops,
            },
            Move {
                from: 20,
                to: 27,
                piece: Pieces::Bishops,
            },
            Move {
                from: 20,
                to: 34,
                piece: Pieces::Bishops,
            },
            Move {
                from: 20,
                to: 41,
                piece: Pieces::Bishops,
            },
            Move {
                from: 20,
                to: 11,
                piece: Pieces::Bishops,
            },
            Move {
                from: 51,
                to: 44,
                piece: Pieces::Bishops,
            },
            Move {
                from: 51,
                to: 60,
                piece: Pieces::Bishops,
            },
            Move {
                from: 51,
                to: 24,
                piece: Pieces::Bishops,
            },
            Move {
                from: 51,
                to: 33,
                piece: Pieces::Bishops,
            },
            Move {
                from: 51,
                to: 42,
                piece: Pieces::Bishops,
            },
        ];
        let white_moves = bishop_moves(&board, true);
        assert_eq!(expected_white_moves, white_moves);

        let expected_black_moves: Vec<Move> = vec![
            Move {
                from: 41,
                to: 20,
                piece: Pieces::Bishops,
            },
            Move {
                from: 41,
                to: 27,
                piece: Pieces::Bishops,
            },
            Move {
                from: 41,
                to: 34,
                piece: Pieces::Bishops,
            },
            Move {
                from: 41,
                to: 50,
                piece: Pieces::Bishops,
            },
            Move {
                from: 41,
                to: 59,
                piece: Pieces::Bishops,
            },
            Move {
                from: 41,
                to: 48,
                piece: Pieces::Bishops,
            },
        ];
        let black_moves = bishop_moves(&board, false);
        assert_eq!(expected_black_moves, black_moves);
    }
}
