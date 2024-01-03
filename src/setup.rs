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

    pub fn print(&self) {
        let mut board = String::new();

        for rank in (0..8).rev() {
            print!("{}   ", 8 - rank);

            for file in 0..8 {
                let square = rank * 8 + file;

                let piece: char = if self.white_pawns & (1 << square) != 0 {
                    'P'
                } else if self.white_knights & (1 << square) != 0 {
                    'N'
                } else if self.white_bishops & (1 << square) != 0 {
                    'B'
                } else if self.white_rooks & (1 << square) != 0 {
                    'R'
                } else if self.white_queens & (1 << square) != 0 {
                    'Q'
                } else if self.white_king & (1 << square) != 0 {
                    'K'
                } else if self.black_pawns & (1 << square) != 0 {
                    'p'
                } else if self.black_knights & (1 << square) != 0 {
                    'n'
                } else if self.black_bishops & (1 << square) != 0 {
                    'b'
                } else if self.black_rooks & (1 << square) != 0 {
                    'r'
                } else if self.black_queens & (1 << square) != 0 {
                    'q'
                } else if self.black_king & (1 << square) != 0 {
                    'k'
                } else {
                    '.'
                };

                print!("{} ", piece);
            }

            println!()
        }

        println!("\n    A B C D E F G H");
    }
}
