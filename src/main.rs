mod evaluate;
mod setup;

use evaluate::evaluate;
use setup::Board;

fn main() {
    let mut board = Board::new();

    loop {
        board.print();

        let mut instruction = String::new();
        std::io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to read line.");

        let instruction = instruction.trim();

        board.move_piece(instruction);
    }
}
