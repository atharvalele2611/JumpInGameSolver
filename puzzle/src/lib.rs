use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Sized;

/// A generic puzzle solver using BFS with hashing of states.

/// Trait for puzzles that can be goal using BFS with hashing of states.
pub trait Puzzle {
    /// The type of moves for this puzzle.
    type Move;

    /// Determines whether or not the puzzle state represents a solved puzzle.
    fn is_goal(&self) -> bool;

    /// Enumerates all of the (legal) successor puzzle states of the current
    /// puzzle state, along with the move that leads to that successor puzzle
    /// state.
    ///
    /// The `Self: Sized` trait bound is a technical requirement for a trait
    /// method that requires the type of `Self` to be known at compile time (in
    /// this case, in order to know the size of each element of the result
    /// vector).  Most types in Rust are (and are assumed to be) `Sized`, but
    /// the `Self` type of a trait is an exception to that rule that types are
    /// assumed to be `Sized`.
    fn next(&self) -> Vec<(Self::Move, Self)>
    where
        Self: Sized;
}

/// Verify that a sequence of moves solves a puzzle.
///
/// Returns `Some(p)`, if `p` is the goal puzzle state reached from `p0` by the moves `ms`.
///
/// Returns `None`, if either the sequence of moves `ms` starting from `p0` is
/// not legal or if the puzzle state reached from `p0` by the moves `ms` is not
/// a goal state.
pub fn check<P>(p0: P, ms: &[P::Move]) -> Option<P>
where
    P: Puzzle + Debug,
    P::Move: Eq + Debug,
{
    let mut p = p0;
    'moves: for cm in ms {
        for (m, pp) in p.next() {
            if cm == &m {
                p = pp;
                println!("p {:?}", p);
                continue 'moves;
            }
        }
        return None;
    }
    if p.is_goal() {
        Some(p)
    } else {
        None
    }
}

/// Solve a puzzle using BFS with hashing of states.
///
/// Returns `Some((ms,p))` if puzzle `p0` can be solved by the sequence of moves
/// `ms` to a goal state `p`.  The sequence of moves `ms` should be one of the
/// shortest sequence of moves from `p0` to a goal state; that is, for any
/// sequence of moves `ns` from `p0` to a goal state, `ms.len() <= ns.len()`.
/// Note that there may not be a unique goal state for a puzzle, therefore, the
/// goal state `p` reached by the sequence of moves `ms` is returned.
///
/// Returns `None` if `p0` cannot be solved by any sequence of moves.
///
/// A BFS is used to find the shortest sequence of moves from `p0` to a goal
/// state.  A hash set or hash table is used to avoid redundant puzzle states
/// (e.g., different sequences of moves may lead to the same puzzle state).
///
/// The generic type parameter `P` must implement `Puzzle` (because it
/// represents a puzzle state), `Eq` and `Hash` (in order to for puzzle states
/// to serve as hash-set or hash-table keys), and `Clone` (in order for puzzle
/// states to be placed in both the hash set or hash table and the BFS queue).
///
/// For simplicity, we assume that the `P::Move` type implements `Clone`.
/// However, there is an implementation of `solve` that does not require this
/// trait bound (at the expense of performing additional copies of puzzle
/// states).
pub fn solve<P>(p0: P) -> Option<(Vec<P::Move>, P)>
where
    P: Puzzle + Eq + Hash + Clone,
    P::Move: Clone,
{
    let mut hash_map = HashMap::<P, Option<(P, P::Move)>>::new();
    let mut queue = VecDeque::new();
    queue.push_back(p0.clone());
    //inserting the initial puzzle state to hash map
    hash_map.insert(p0.clone(), None);
    // Loop till queue is not empty
    while !queue.is_empty() {
        let p = match queue.pop_front() {
            Some(v) => v,
            None => return None,
        };

        if p.is_goal() {
            let p1 = p.clone();
            // backtrack using predecessor
            let mut vec = match backtrack(hash_map, p1) {
                Some(vec) => vec,
                None => return None,
            };
            vec.reverse();
            return Some((vec, p));
        }

        for (m, puzz) in p.next() {
            match hash_map.entry(puzz.clone()) {
                Vacant(e) => {
                    queue.push_back(puzz);
                    e.insert(Some((p.clone(), m)));
                }
                _ => {}
            }
        }
    }

    None
}

fn backtrack<P>(mut hash_map: HashMap<P, Option<(P, P::Move)>>, mut p1: P) -> Option<Vec<P::Move>>
where
    P: Puzzle + Eq + Hash + Clone,
    P::Move: Clone,
{
    let mut vec = vec![];
    loop {
        let (predecssor, m) = match hash_map.remove(&p1) {
            Some(pm_o) => match pm_o {
                Some(pm) => pm,
                None => break,
            },
            None => return None,
        };
        vec.push(m);
        p1 = predecssor;
    }
    Some(vec)
}
#[allow(clippy::type_complexity)]
pub mod test;
