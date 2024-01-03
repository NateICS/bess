mod evaluate;
mod setup;

use evaluate::evaluate;
use setup::Board;

fn main() {
    let board = Board::new();

    // println!("{}", evaluate(board));

    loop {
        let mut coord = String::new();
        std::io::stdin()
            .read_line(&mut coord)
            .expect("Failed to read line.");

        let coord = coord.trim();

        println!("{}", coord);
        board.print();
    }
}
