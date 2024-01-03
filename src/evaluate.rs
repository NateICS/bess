use crate::setup::Board;

pub fn evaluate(board: Board) -> u32 {
    let mut score = 0;

    let values = [1, 3, 3, 5, 9];

    score += board.white_pawns.count_ones() * values[0];
    score += board.white_knights.count_ones() * values[1];
    score += board.white_bishops.count_ones() * values[2];
    score += board.white_rooks.count_ones() * values[3];
    score += board.white_queens.count_ones() * values[4];

    score -= board.black_pawns.count_ones() * values[0];
    score -= board.black_knights.count_ones() * values[1];
    score -= board.black_bishops.count_ones() * values[2];
    score -= board.black_rooks.count_ones() * values[3];
    score -= board.black_queens.count_ones() * values[4];

    score
}
