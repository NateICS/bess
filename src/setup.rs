pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            white_pawns: 255 << 8,
            white_knights: (1 << 1) + (1 << 6),
            white_bishops: (1 << 2) + (1 << 5),
            white_rooks: (1 << 0) + (1 << 7),
            white_queens: 1 << 3,
            white_king: 1 << 4,

            black_pawns: 255 << 48,
            black_knights: (1 << 57) + (1 << 62),
            black_bishops: (1 << 58) + (1 << 61),
            black_rooks: (1 << 56) + (1 << 63),
            black_queens: 1 << 59,
            black_king: 1 << 60,
        }
    }
}
