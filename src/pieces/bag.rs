//! 7-bag tetromino generator.
//!
//! Each "bag" is one shuffled copy of all 7 tetrominoes. When a bag is empty,
//! a fresh shuffled bag is queued. This guarantees that any 7 consecutive
//! pieces contain each kind exactly once, removing pathological dry spells
//! that uniform random would allow.

use bevy::prelude::Resource;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use super::tetromino::TetrominoKind;

#[derive(Resource)]
pub struct SevenBag {
    /// Pieces queued to be drawn next. Front of the queue (index 0) is the
    /// next piece. Always contains at least 1 entry after construction so
    /// `peek` never returns less than one piece.
    queue: Vec<TetrominoKind>,
    rng: StdRng,
}

impl SevenBag {
    /// Creates a bag seeded from system entropy.
    pub fn new() -> Self {
        Self::from_rng(StdRng::from_os_rng())
    }

    /// Creates a deterministic bag from a fixed seed. Used in tests.
    pub fn from_seed(seed: u64) -> Self {
        Self::from_rng(StdRng::seed_from_u64(seed))
    }

    fn from_rng(rng: StdRng) -> Self {
        let mut bag = Self {
            queue: Vec::with_capacity(14),
            rng,
        };
        bag.refill();
        bag
    }

    /// Draws the next piece, refilling the underlying bag if needed.
    pub fn next(&mut self) -> TetrominoKind {
        if self.queue.is_empty() {
            self.refill();
        }
        self.queue.remove(0)
    }

    /// Returns the next piece without consuming it. Refills the bag if it has
    /// been drained dry. Cheaper than [`peek`] when only the immediate next is
    /// needed (e.g. the UI preview panel).
    pub fn peek_next(&mut self) -> TetrominoKind {
        if self.queue.is_empty() {
            self.refill();
        }
        self.queue[0]
    }

    /// Returns the next `count` pieces without consuming them, refilling the
    /// underlying bag enough times to satisfy the request.
    pub fn peek(&mut self, count: usize) -> Vec<TetrominoKind> {
        while self.queue.len() < count {
            self.refill();
        }
        self.queue[..count].to_vec()
    }

    fn refill(&mut self) {
        let mut next_bag = TetrominoKind::ALL;
        next_bag.shuffle(&mut self.rng);
        self.queue.extend_from_slice(&next_bag);
    }
}

impl Default for SevenBag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_seven_pieces_contain_each_kind_once() {
        let mut bag = SevenBag::from_seed(42);
        for _ in 0..5 {
            let mut drawn: HashSet<TetrominoKind> = HashSet::new();
            for _ in 0..7 {
                drawn.insert(bag.next());
            }
            assert_eq!(drawn.len(), 7, "missing a kind in this bag: {:?}", drawn);
        }
    }

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut a = SevenBag::from_seed(123);
        let mut b = SevenBag::from_seed(123);
        for _ in 0..20 {
            assert_eq!(a.next(), b.next());
        }
    }

    #[test]
    fn peek_next_returns_head_without_consuming() {
        let mut bag = SevenBag::from_seed(99);
        let first = bag.peek_next();
        let again = bag.peek_next();
        let drawn = bag.next();
        assert_eq!(first, again);
        assert_eq!(first, drawn);
    }

    #[test]
    fn peek_does_not_consume() {
        let mut bag = SevenBag::from_seed(7);
        let preview = bag.peek(5);
        let drawn: Vec<_> = (0..5).map(|_| bag.next()).collect();
        assert_eq!(preview, drawn);
    }

    #[test]
    fn peek_can_cross_bag_boundary() {
        let mut bag = SevenBag::from_seed(7);
        let preview = bag.peek(10);
        assert_eq!(preview.len(), 10);
        let drawn: Vec<_> = (0..10).map(|_| bag.next()).collect();
        assert_eq!(preview, drawn);
    }
}
