mod setup;
mod evaluate;

fn main() {
    let board = setup::Board::new();
    println!("{}", evaluate::evaluate(board));
}