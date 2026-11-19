use serde::{Deserialize, Serialize};

pub const SIZE: usize = 9;
pub type Board = [[bool; SIZE]; SIZE];
pub type Shape = &'static [(usize, usize)];

const SHAPES: &[Shape] = &[
    &[(0, 0)],
    &[(0, 0), (1, 0)],
    &[(0, 0), (0, 1)],
    &[(0, 0), (1, 0), (2, 0)],
    &[(0, 0), (0, 1), (0, 2)],
    &[(0, 0), (0, 1), (1, 1)],
    &[(0, 0), (1, 0), (1, 1)],
    &[(0, 0), (1, 0), (0, 1)],
    &[(1, 0), (0, 1), (1, 1)],
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
    &[(0, 0), (1, 0), (2, 0), (1, 1)],
    &[(0, 1), (1, 1), (2, 1), (1, 0)],
    &[(0, 0), (0, 1), (0, 2), (1, 1)],
    &[(0, 0), (0, 1), (1, 1), (1, 2)],
    &[(1, 0), (0, 1), (1, 1), (0, 2)],
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub points: u32,
    pub cleared: u32,
    pub drawn: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    board: Board,
    pieces: [Option<usize>; 3],
    score: u32,
    best: u32,
    seed: u64,
}

impl Game {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            board: [[false; SIZE]; SIZE],
            pieces: [None; 3],
            score: 0,
            best: 0,
            seed: seed.max(1),
        };
        game.draw_pieces();
        game
    }

    pub fn restart(&mut self) {
        let best = self.best;
        *self = Self::new(self.next());
        self.best = best;
    }

    pub fn restored(mut self) -> Self {
        if self.seed == 0 || self.pieces.iter().flatten().any(|&id| id >= SHAPES.len()) {
            self.restart();
        }
        if self.pieces.iter().all(Option::is_none) {
            self.draw_pieces();
        }
        self.best = self.best.max(self.score);
        self
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn piece(&self, index: usize) -> Option<Shape> {
        self.pieces
            .get(index)
            .copied()
            .flatten()
            .map(|id| SHAPES[id])
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn best(&self) -> u32 {
        self.best
    }

    pub fn over(&self) -> bool {
        !(0..3)
            .any(|piece| (0..SIZE).any(|row| (0..SIZE).any(|column| self.fits(piece, column, row))))
    }

    pub fn fits(&self, piece: usize, column: usize, row: usize) -> bool {
        if column >= SIZE || row >= SIZE {
            return false;
        }
        self.piece(piece).is_some_and(|shape| {
            shape.iter().all(|&(x, y)| {
                column + x < SIZE && row + y < SIZE && !self.board[row + y][column + x]
            })
        })
    }

    pub fn place(&mut self, piece: usize, column: usize, row: usize) -> Option<Move> {
        if !self.fits(piece, column, row) {
            return None;
        }
        let shape = self.piece(piece)?;
        for &(x, y) in shape {
            self.board[row + y][column + x] = true;
        }
        self.pieces[piece] = None;
        let cleared = self.clear();
        let points = shape.len() as u32 + 12 * cleared;
        self.score = self.score.saturating_add(points);
        self.best = self.best.max(self.score);
        let drawn = self.pieces.iter().all(Option::is_none);
        if drawn {
            self.draw_pieces();
        }
        Some(Move {
            points,
            cleared,
            drawn,
        })
    }

    fn clear(&mut self) -> u32 {
        let mut mask = [[false; SIZE]; SIZE];
        let mut cleared = 0;
        for (y, row) in self.board.iter().enumerate() {
            if row.iter().all(|&filled| filled) {
                mask[y].fill(true);
                cleared += 1;
            }
        }
        for x in 0..SIZE {
            if self.board.iter().all(|row| row[x]) {
                for row in &mut mask {
                    row[x] = true;
                }
                cleared += 1;
            }
        }
        for top in (0..SIZE).step_by(3) {
            for left in (0..SIZE).step_by(3) {
                if self.board[top..top + 3]
                    .iter()
                    .all(|row| row[left..left + 3].iter().all(|&cell| cell))
                {
                    for row in &mut mask[top..top + 3] {
                        row[left..left + 3].fill(true);
                    }
                    cleared += 1;
                }
            }
        }
        for (row, clear) in self.board.iter_mut().zip(mask) {
            for (cell, remove) in row.iter_mut().zip(clear) {
                *cell &= !remove;
            }
        }
        cleared
    }

    fn draw_pieces(&mut self) {
        self.pieces = std::array::from_fn(|_| Some((self.next() % SHAPES.len() as u64) as usize));
    }

    fn next(&mut self) -> u64 {
        let mut state = self.seed.max(1);
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        self.seed = state;
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_move_can_clear_a_row_column_and_box_at_once() {
        let mut game = Game::new(1);
        game.pieces[0] = Some(0);
        for n in 1..SIZE {
            game.board[0][n] = true;
            game.board[n][0] = true;
        }
        for row in &mut game.board[1..3] {
            row[1..3].fill(true);
        }
        let placed = game.place(0, 0, 0).unwrap();
        assert_eq!(placed.cleared, 3);
        assert_eq!(game.score(), 37);
        assert!(!game.board().iter().flatten().any(|&occupied| occupied));
    }

    #[test]
    fn a_turn_draws_three_new_pieces_after_the_last_placement() {
        let mut game = Game::new(42);
        game.pieces = [Some(0); 3];
        assert!(!game.place(0, 0, 0).unwrap().drawn);
        assert!(game.place(0, 1, 0).is_none());
        assert!(game.place(1, 0, 0).is_none());
        assert!(!game.place(1, 1, 0).unwrap().drawn);
        assert!(game.place(2, 2, 0).unwrap().drawn);
        assert!((0..3).all(|piece| game.piece(piece).is_some()));
    }

    #[test]
    fn no_legal_placement_finishes_the_game_and_restart_keeps_best() {
        let mut game = Game::new(1);
        game.pieces = [Some(1); 3];
        game.board = [[true; SIZE]; SIZE];
        game.board[8][8] = false;
        assert!(game.over());
        assert!(!game.fits(0, 8, 8));
        assert!(!game.fits(0, usize::MAX, 0));
        game.best = 48;
        game.restart();
        assert_eq!(game.score(), 0);
        assert_eq!(game.best(), 48);
        assert!(!game.over());
    }
}
