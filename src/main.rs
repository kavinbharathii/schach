
// Chess bitboard mapping and further improvements?!
#![allow(dead_code)]
#![allow(unused_assignments)]

struct Piece;

impl Piece {
    const NONE: u8 = 0;
    const KING: u8 = 1;
    const PAWN: u8 = 2;
    const KNIGHT: u8 = 3;
    const BISHOP: u8 = 4;
    const ROOK: u8 = 5;
    const QUEEN: u8 = 6;
    const WHITE: u8 = 8;
    const BLACK: u8 = 16;
}


fn main() {
    // const WHITE_PAWNS: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;

    const WHITE_KNIGHT: u8 = Piece::WHITE | Piece::KNIGHT;
    println!("{:b}", WHITE_KNIGHT);
}
