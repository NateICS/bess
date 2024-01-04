use crate::setup::Board;

pub fn evaluate(board: Board) -> u32 {
    let mut score = 0;

    let values = [1, 3, 3, 5, 9];

    score += board.bitboards[0].count_ones() * values[0];
    score += board.bitboards[1].count_ones() * values[1];
    score += board.bitboards[2].count_ones() * values[2];
    score += board.bitboards[3].count_ones() * values[3];
    score += board.bitboards[4].count_ones() * values[4];

    score -= board.bitboards[6].count_ones() * values[0];
    score -= board.bitboards[7].count_ones() * values[1];
    score -= board.bitboards[8].count_ones() * values[2];
    score -= board.bitboards[9].count_ones() * values[3];
    score -= board.bitboards[10].count_ones() * values[4];

    score
}
