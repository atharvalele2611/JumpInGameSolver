use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use puzzle::Puzzle;

/// The `Object` type represents objects that may be placed (and moved) on the
/// gameboard.  (Note that there is no `Hole` variant, as the holes are always
/// at fixed positions on the gameboard.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Object {
    WhiteRabbit,
    BrownRabbit,
    GreyRabbit,
    Mushroom,
    Fox1Head,
    Fox1Tail,
    Fox2Head,
    Fox2Tail,
}
impl Object {
    pub fn is_rabbit(&self) -> bool {
        matches!(
            self,
            Object::WhiteRabbit | Object::BrownRabbit | Object::GreyRabbit
        )
    }
    pub fn is_foxhead(&self) -> bool {
        matches!(self, Object::Fox1Head | Object::Fox2Head)
    }
    pub fn is_foxtail(&self) -> bool {
        matches!(self, Object::Fox1Tail | Object::Fox2Tail)
    }
    pub fn is_fox(&self) -> bool {
        self.is_foxhead() || self.is_foxtail()
    }
    pub fn is_foxmatch(&self, othr: &Self) -> bool {
        matches!(
            (self, othr),
            (Object::Fox1Head, Object::Fox1Tail)
                | (Object::Fox1Tail, Object::Fox1Head)
                | (Object::Fox2Head, Object::Fox2Tail)
                | (Object::Fox2Tail, Object::Fox2Head)
        )
    }
    pub fn checking_for_obst(&self) -> bool {
        matches!(
            self,
            Object::WhiteRabbit
                | Object::GreyRabbit
                | Object::BrownRabbit
                | Object::Fox1Head
                | Object::Fox2Head
                | Object::Fox1Tail
                | Object::Fox2Tail
                | Object::Mushroom
        )
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Object::WhiteRabbit => "W",
            Object::BrownRabbit => "B",
            Object::GreyRabbit => "G",
            Object::Mushroom => "M",
            Object::Fox1Head => "F",
            Object::Fox1Tail => "X",
            Object::Fox2Head => "V", // "fox" in Dutch is "vos"
            Object::Fox2Tail => "S",
        })
    }
}

/// The (private) `pos` module ensures that the `Pos` type can only be created
/// via the `new` associated function and accessed via the `x`, `y`, and `xy`
/// methods.
mod pos {
    /// The `Pos` type represents a position on the gameboard.  The North-West
    /// corner of the gameboard is `Pos {x: 0, y: 0}`, the North-East corner of the
    /// gameboard `Pos {x: 4, y: 0}`, the South-West corner of the gameboard is
    /// `Pos {x: 0, y: 4}`, and the South-East corner of the gameboard is
    /// `Pos {x: 4, y: 4}`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        x: usize,
        y: usize,
    }
    impl Pos {
        pub(super) fn new(x: usize, y: usize) -> Self {
            if x >= 5 {
                panic!("Pos::new x (is {}) should be less than 5", x)
            }
            if y >= 5 {
                panic!("Pos::new y (is {}) should be less than 5", y)
            }
            Pos { x, y }
        }
        pub(super) fn xy(&self) -> (usize, usize) {
            (self.x, self.y)
        }
    }
}
use self::pos::Pos;

impl Pos {
    fn is_hole(&self) -> bool {
        let (x, y) = self.xy();
        ((x == 0 || x == 4) && (y == 0 || y == 4)) || (x == 2 && y == 2)
    }
    fn is_raised(&self) -> bool {
        let (x, y) = self.xy();
        self.is_hole() || ((x == 0 || x == 4) && y == 2) || (x == 2 && (y == 0 || y == 4))
    }
    /// Return `Some(pos)` if `pos` is the position on the gameboard that is one
    /// step from `self` in the direction `dir`.  Returns `None` if there is no
    /// position on the gameboard that is one step from `self` in the direction
    /// `dir` (i.e., would move off the edge of the gameboard).
    fn step(&self, dir: Direction) -> Option<Self> {
        let (x, y) = self.xy();
        let (x, y) = match dir {
            Direction::North => {
                if y == 0 {
                    return None;
                }
                (x, y - 1)
            }
            Direction::South => {
                if y == 4 {
                    return None;
                }
                (x, y + 1)
            }
            Direction::West => {
                if x == 0 {
                    return None;
                }
                (x - 1, y)
            }
            Direction::East => {
                if x == 4 {
                    return None;
                }
                (x + 1, y)
            }
        };
        Some(Pos::new(x, y))
    }
    /// An iterator over all positions of the gameboard.
    pub fn values() -> impl Iterator<Item = Self> {
        (0..5).flat_map(|y| (0..5).map(move |x| Pos::new(x, y)))
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (x, y) = self.xy();
        write!(f, "({},{})", x, y)
    }
}

/// The `Direction` type represents the cardinal directions, in which objects
/// may be moved on the gameboard.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
    /// An iterator over all directions.
    fn values() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Direction::North => "↑",
            Direction::South => "↓",
            Direction::West => "←",
            Direction::East => "→",
        })
    }
}

/// The (private) `jumpin` module ensures that the `JumpIN` type can only be
/// created via the `new` associated function and accessed via the `get` and
/// `get_mut` methods.
mod jumpin {
    use super::Object;
    use super::Pos;
    use serde::{Deserialize, Serialize};

    /// The `JumpIN` type represents a
    /// https://www.smartgames.eu/uk/one-player-games/jumpin[JumpIN'] puzzle state:
    /// a gameboard with placed objects.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct JumpIN([[Option<Object>; 5]; 5]);
    impl JumpIN {
        pub(super) fn new() -> Self {
            JumpIN([[None; 5]; 5])
        }
        /// Returns a reference to the gameboard at position `pos`.
        pub(super) fn get(&self, pos: Pos) -> &Option<Object> {
            let (x, y) = pos.xy();
            &self.0[x][y]
        }
        /// Returns a mutable reference to the gameboard at position `pos`.
        pub(super) fn get_mut(&mut self, pos: Pos) -> &mut Option<Object> {
            let (x, y) = pos.xy();
            &mut self.0[x][y]
        }
    }
}
pub use self::jumpin::JumpIN;

impl Display for JumpIN {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for pos in Pos::values() {
            match self.get(pos) {
                None => f.write_str(if pos.is_hole() { "H" } else { " " })?,
                Some(obj) => obj.fmt(f)?,
            }
            if pos.xy().0 == 4 {
                f.write_str("\n")?
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JumpINParseError;
impl FromStr for JumpIN {
    type Err = JumpINParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut jumpin = JumpIN::new();
        let mut white_rabbit = false;
        let mut brown_rabbit = false;
        let mut grey_rabbit = false;
        let mut mushrooms = 0;
        let mut fox1head = None;
        let mut fox1tail = None;
        let mut fox2head = None;
        let mut fox2tail = None;
        let mut cs = s.chars();
        for y in 0..5 {
            for x in 0..5 {
                let pos = Pos::new(x, y);
                let put_rabbit = |jumpin: &mut JumpIN, rabbit: &mut bool, obj| {
                    if *rabbit {
                        return Err(JumpINParseError);
                    };
                    *rabbit = true;
                    *jumpin.get_mut(pos) = Some(obj);
                    Ok(())
                };
                let put_mushroom = |jumpin: &mut JumpIN, mushrooms: &mut i32| {
                    if *mushrooms > 2 {
                        return Err(JumpINParseError);
                    };
                    *mushrooms += 1;
                    *jumpin.get_mut(pos) = Some(Object::Mushroom);
                    Ok(())
                };
                let put_fox = |jumpin: &mut JumpIN, fox: &mut Option<(usize, usize)>, obj| {
                    if pos.is_raised() {
                        return Err(JumpINParseError);
                    };
                    if fox.is_some() {
                        return Err(JumpINParseError);
                    };
                    *fox = Some((x, y));
                    *jumpin.get_mut(pos) = Some(obj);
                    Ok(())
                };
                match cs.next() {
                    None => return Err(JumpINParseError),
                    Some(c) => match c {
                        ' ' if pos.is_hole() => return Err(JumpINParseError),
                        ' ' => {}
                        'H' if !pos.is_hole() => return Err(JumpINParseError),
                        'H' => {}
                        'W' => put_rabbit(&mut jumpin, &mut white_rabbit, Object::WhiteRabbit)?,
                        'B' => put_rabbit(&mut jumpin, &mut brown_rabbit, Object::BrownRabbit)?,
                        'G' => put_rabbit(&mut jumpin, &mut grey_rabbit, Object::GreyRabbit)?,
                        'M' => put_mushroom(&mut jumpin, &mut mushrooms)?,
                        'F' => put_fox(&mut jumpin, &mut fox1head, Object::Fox1Head)?,
                        'X' => put_fox(&mut jumpin, &mut fox1tail, Object::Fox1Tail)?,
                        'V' => put_fox(&mut jumpin, &mut fox2head, Object::Fox2Head)?,
                        'S' => put_fox(&mut jumpin, &mut fox2tail, Object::Fox2Tail)?,
                        _ => return Err(JumpINParseError),
                    },
                }
            }
            match cs.next() {
                Some('\n') => (),
                _ => return Err(JumpINParseError),
            }
        }
        if cs.next().is_some() {
            return Err(JumpINParseError);
        }
        let chk_fox = |fox0, fox1| match (fox0, fox1) {
            (None, None) => Ok(()),
            (Some((x0, y0)), Some((x1, y1))) => {
                if (x0 == x1 && (y0 + 1 == y1 || y0 == y1 + 1))
                    || ((x0 + 1 == x1 || x0 == x1 + 1) && y0 == y1)
                {
                    Ok(())
                } else {
                    Err(JumpINParseError)
                }
            }
            (_, _) => Err(JumpINParseError),
        };
        chk_fox(fox1head, fox1tail)?;
        chk_fox(fox2head, fox2tail)?;
        if !(white_rabbit || brown_rabbit || grey_rabbit) {
            return Err(JumpINParseError);
        };
        Ok(jumpin)
    }
}

impl Puzzle for JumpIN {
    type Move = (Object, Direction);

    fn is_goal(&self) -> bool {
        // iterate through all positions
        for pos in Pos::values() {
            if let Some(obj) = self.get(pos) {
                if obj.is_rabbit() && !pos.is_hole() {
                    return false;
                }
            }
        }
        true
    }

    fn next(&self) -> Vec<(Self::Move, Self)> {
        let mut next = Vec::new();
        // iterate through all positions
        for pos in Pos::values() {
            // iterate through all directions
            for dir in Direction::values() {
                // try to move a rabbit

                if let Some((obj, jumpin)) = self.move_rabbit(pos, dir) {
                    // record the move and successor puzzle state
                    // println!("obj {} jumpin {}", obj , jumpin);
                    // println!("{}", jumpin);
                    next.push(((obj, dir), jumpin))
                }
                // try to move a fox
                if let Some((obj, jumpin)) = self.move_fox(pos, dir) {
                    // record the move and successor puzzle state
                    // println!("{}", jumpin);
                    next.push(((obj, dir), jumpin))
                }
            }
        }
        next
    }
}

impl JumpIN {
    /// Attempt to move a rabbit at position `pos` in the direction `dir`;
    /// if successful, return the rabbit object that was moved and the new
    /// gameboard (`JumpIN` puzzle state).
    ///
    /// This method implements the game rules with respect to rabbits.  Check
    /// that there is a rabbit at position `pos`, that the next position in the
    /// direction `dir` is an obstacle (a rabbit must jump over at least one
    /// obstacle), and find the position `posd` (destination) of the first empty
    /// space in the direction `dir`.  If these can be satisfied, then it should
    /// copy the current gameboard into a new gameboard, update `pos` in the new
    /// gameboard to `None` and update `posd` in the new gameboard to `Some`.
    fn move_rabbit(&self, pos: Pos, dir: Direction) -> Option<(Object, Self)> {
        // Your code here
        let self_copy = &mut self.clone();
        let obj_p = match self.get(pos) {
            Some(o) => o,
            None => return None,
        };
        // check if object is a rabbit object only then proceed
        if !obj_p.is_rabbit() {
            return None;
        }

        let posf = match pos.step(dir) {
            Some(posf) => match self.get(posf) {
                Some(ob) => {
                    // check if object is a not and obstacle object only then proceed
                    if !ob.checking_for_obst() {
                        return None;
                    }
                    posf
                }
                None => return None,
            },
            None => return None,
        };
        let mut p = posf;
        // step untill rabbit finds a valid position
        p = match get_next_empty_pos_rabbit(self, p, dir) {
            Some(p) => p,
            None => return None,
        };
        if p == posf {
            return None;
        }
        *self_copy.get_mut(p) = Some(*obj_p);
        *self_copy.get_mut(pos) = None;
        return Some((*obj_p, *self_copy));
    }

    /// Attempt to move a fox at position `pos` in the direction `dir`;
    /// if successful, return the fox head object that was moved and the new
    /// `JumpIN` puzzle state.
    ///
    /// This method implements the game rules with respect to foxes.  Let `posb`
    /// (backward) be the previous position in direction `dir` and let `posf`
    /// (forward) be the next position in direction `dir`.  Check that there are
    /// matching fox components at positions `posb` and `pos` and that `posf`
    /// is an empty space.  If so, then it should copy the current gameboard
    /// into a new gameboard, update `posf` in the new gameboard to `Some` of
    /// the object at `pos` in the current gameboard, update `pos` in the new
    /// gameboard to `Some` of the object at `posb` in the current gameboard,
    /// and update `posd` in the new gameboard to `None`.  Note that the object
    /// returned by the `move_fox` method should be a fox head object, which is
    /// not necessarily the object that is at position `pos` in the current
    /// gameboard.
    fn move_fox(&self, pos: Pos, dir: Direction) -> Option<(Object, Self)> {
        let self_copy = &mut self.clone();
        let current = match self.get(pos) {
            Some(o) => o,
            None => return None,
        };

        if !current.is_fox() {
            return None;
        }

        let posb = match pos.step(dir.rev()) {
            Some(p) => p,
            None => return None,
        };

        let back = match self.get(posb) {
            Some(o) => o,
            None => return None,
        };
        // match the fox head using current and back objects
        let (head, _) = match match_fox_head(current, back) {
            Some((h1, h2)) => (h1, h2),
            None => return None,
        };
        let posf = match pos.step(dir) {
            Some(p) => p,
            None => return None,
        };

        match self.get(posf) {
            Some(o) => {
                if o.checking_for_obst() {
                    return None;
                }
            }
            None => {}
        };

        *self_copy.get_mut(posf) = Some(*current);
        *self_copy.get_mut(pos) = Some(*back);
        *self_copy.get_mut(posb) = None;

        Some((head, *self_copy))
    }
}
fn get_next_empty_pos_rabbit(board: &JumpIN, mut p: Pos, dir: Direction) -> Option<Pos> {
    loop {
        match p.step(dir) {
            Some(new_pos) => match board.get(new_pos) {
                Some(ob) => {
                    if ob.checking_for_obst() {
                        p = new_pos;
                    } else {
                        return None;
                    }
                }
                None => {
                    p = new_pos;
                    break;
                }
            },
            None => return None,
        }
    }
    Some(p)
}

fn match_fox_head(current_obj: &Object, obj_posb: &Object) -> Option<(Object, Object)> {
    match (current_obj, obj_posb) {
        (Object::Fox1Tail, Object::Fox1Head) | (Object::Fox2Tail, Object::Fox2Head) => {
            return Some((*obj_posb, *current_obj));
        }
        (Object::Fox1Head, Object::Fox1Tail) | (Object::Fox2Head, Object::Fox2Tail) => {
            return Some((*current_obj, *obj_posb));
        }

        _ => return None,
    };
}

#[cfg(test)]
mod tests;
