pub struct Board {
    pub bitboards: [u64; 12],
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboards: [
                255 << 8,
                (1 << 1) + (1 << 6),
                (1 << 2) + (1 << 5),
                (1 << 0) + (1 << 7),
                1 << 3,
                1 << 4,
                255 << 48,
                (1 << 57) + (1 << 62),
                (1 << 58) + (1 << 61),
                (1 << 56) + (1 << 63),
                1 << 59,
                1 << 60,
            ],
        }
    }

    pub fn print(&self) {
        let symbols = ["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k"];

        for rank in (0..8).rev() {
            print!("{}   ", rank + 1);

            'file: for file in 0..8 {

                let square = rank * 8 + file;

                for (i, &bitboard) in self.bitboards.iter().enumerate() {
                    if (bitboard >> square) & 1 == 1 {
                        print!("{} ", symbols[i]);
                        continue 'file;
                    }
                }

                print!(". ");
            }

            println!();
        }

        println!("\n    A B C D E F G H");
    }

    pub fn move_piece(&mut self, instruction: &str) {
        // Doesn't work

        let start = &instruction[..2];
        let finish = &instruction[3..5];

        let source_square = self.square_from_string(start);
        let destination_square = self.square_from_string(finish);

        for bitboard in &mut self.bitboards {
            *bitboard &= !(1 << source_square);
        }

        for bitboard in &mut self.bitboards {
            *bitboard |= 1 << destination_square;
        }
    }

    fn square_from_string(&self, square: &str) -> usize {
        let file = square.chars().nth(0).unwrap() as usize - 'a' as usize;
        let rank = square.chars().nth(1).unwrap() as usize - '1' as usize;

        rank * 8 + file
    }
}
