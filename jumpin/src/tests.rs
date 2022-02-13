use std::fs;

use super::{JumpIN, JumpINParseError};
use puzzle::{
    test::{MoveTree, MoveTreeVerifyError},
    Puzzle,
};

fn moves_to_string(ms: &[<JumpIN as Puzzle>::Move]) -> String {
    let mut s = String::new();
    let mut last_obj = None;
    for (obj, dir) in ms {
        if last_obj != Some(obj) {
            last_obj = Some(obj);
            s.push_str(&format!("{}", obj))
        }
        s.push_str(&format!("{}", dir))
    }
    s
}

fn parse_test(
    file_stem: &str,
    parse_test_res: Result<JumpIN, JumpINParseError>,
) -> Result<(), String> {
    let src =
        fs::read_to_string(format!("./assets/{file_stem}.txt", file_stem = file_stem)).unwrap();
    match (src.parse::<JumpIN>(), parse_test_res) {
        (Err(_), Err(_)) => Ok(()),
        (Err(_), Ok(_)) => Err(format!(
            "`JumpIN::from_str` of {file_stem} should not fail.",
            file_stem = file_stem
        )),
        (Ok(_), Err(_)) => Err(format!(
            "`JumpIN::from_str` of {file_stem} should fail.",
            file_stem = file_stem
        )),
        (Ok(puzzle), Ok(puzzle_res)) => {
            if puzzle == puzzle_res {
                Ok(())
            } else {
                Err(format!(
                    "`JumpIN::from_str` of {file_stem} succeeded, but does not equal expected JumpIN value.",
                    file_stem = file_stem
                ))
            }
        }
    }
}

fn check_test(
    file_stem: &str,
    puzzle: JumpIN,
    (mvs_soln, goal_soln): (Vec<<JumpIN as Puzzle>::Move>, JumpIN),
) -> Result<(), String> {
    match puzzle::check(puzzle, &mvs_soln) {
        None => {
            return Err(format!(
                "{file_stem} reference solution ({mvs}) failed `puzzle::check`",
                file_stem = file_stem,
                mvs = moves_to_string(&mvs_soln)
            ))
        }
        Some(goal_chk) => {
            if goal_soln != goal_chk {
                return Err(format!(
                    "{file_stem} reference solution ({mvs}) final gameboard does not equal `puzzle::check` final gameboard.",
                    file_stem = file_stem,
                    mvs = moves_to_string(&mvs_soln)
                ));
            }
        }
    };
    Ok(())
}

fn solve_test(
    file_stem: &str,
    puzzle: JumpIN,
    soln: Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)>,
) -> Result<(), String> {
    match (puzzle::solve(puzzle), soln) {
        (None, None) => Ok(()),
        (Some((mvs, _)), None) => Err(format!(
            "{file_stem} has solution ({mvs}), but reference has no solution; likely has an invalid move and/or an incorrect `JumpIN::is_goal`.",
            file_stem = file_stem,
            mvs = moves_to_string(&mvs)
        )),
        (None, Some((mvs_soln, _))) => Err(format!(
            "{file_stem} has no solution, but reference has solution ({mvs}).",
            file_stem = file_stem,
            mvs = moves_to_string(&mvs_soln)
        )),
        (Some((mvs, goal)), Some((mvs_soln, goal_soln))) => {
            match puzzle::check(puzzle, &mvs) {
                None => {
                    return Err(format!(
                        "{file_stem} solution ({mvs}) failed `puzzle::check`.",
                        file_stem = file_stem,
                        mvs = moves_to_string(&mvs)
                    ))
                }
                Some(goal_chk) => {
                    if goal != goal_chk {
                        return Err(format!(
                            "{file_stem} solution ({mvs}) final gameboard does not equal `puzzle::check` final gameboard.",
                            file_stem = file_stem,
                            mvs = moves_to_string(&mvs)
                        ));
                    }
                }
            };
            match mvs.len().cmp(&mvs_soln.len()) {
                std::cmp::Ordering::Greater => Err(format!(
                    "{file_stem} solution ({mvs}) is longer than reference solution ({mvs_soln}).",
                    file_stem = file_stem,
                    mvs = moves_to_string(&mvs),
                    mvs_soln = moves_to_string(&mvs_soln)
                )),
                std::cmp::Ordering::Equal => {
                    if goal == goal_soln {
                        Ok(())
                    } else {
                        Err(format!(
                            "{file_stem} solution ({mvs}) final gameboard does not equal reference solution ({mvs_soln}) final gameboard; likely has an incorrect `JumpIN::next` (invalid move) and/or an incorrect `JumpIN::is_goal`.",
                            file_stem = file_stem,
                            mvs = moves_to_string(&mvs),
                            mvs_soln = moves_to_string(&mvs_soln)
                        ))
                    }
                }
                std::cmp::Ordering::Less => Err(format!(
                    "{file_stem} solution ({mvs}) is shorter than reference solution ({mvs_soln}); likely has  incorrect `JumpIN::next` (invalid move) and/or an incorrect `JumpIN::is_goal`.",
                    file_stem = file_stem,
                    mvs = moves_to_string(&mvs),
                    mvs_soln = moves_to_string(&mvs_soln)
                )),
            }
        }
    }
}

fn moves_test(
    file_stem: &str,
    puzzle: &JumpIN,
    move_tree: &MoveTree<<JumpIN as Puzzle>::Move, JumpIN>,
) -> Result<(), String> {
    match move_tree.verify::<_, _, ()>(
        puzzle,
        |q, q_soln| if q == q_soln { Ok(()) } else { Err(()) },
    ) {
        Ok(()) => Ok(()),
        Err((ms, err)) => {
            let state = if ms.is_empty() {
                file_stem.to_string()
            } else {
                format!(
                    "{file_stem} with moves {mvs}",
                    file_stem = file_stem,
                    mvs = moves_to_string(&ms)
                )
            };
            match err {
                MoveTreeVerifyError::MissingMove(m) => Err(format!(
                    "{state}, `JumpIN::next` is missing move {m}",
                    state = state,
                    m = moves_to_string(&[m])
                )),
                MoveTreeVerifyError::DuplicateMove(m) => Err(format!(
                    "{state}, `JumpIN::next` has duplicate move {m}",
                    state = state,
                    m = moves_to_string(&[m])
                )),
                MoveTreeVerifyError::ExtraMove(m) => Err(format!(
                    "{state}, `JumpIN::next` has invalid move {m}",
                    state = state,
                    m = moves_to_string(&[m])
                )),
                MoveTreeVerifyError::ChkState(m, _) => Err(format!(
                    "{state}, `JumpIN::next` includes move {m} leading to an incorrect state",
                    state = state,
                    m = moves_to_string(&[m])
                )),
            }
        }
    }
}

mod badinput11 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput11", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod badinput13 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput13", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod wizard50 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("wizard50", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard50.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard50.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("wizard50", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("wizard50", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("wizard50", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 62905
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard50.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod starter12 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("starter12", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter12.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter12.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("starter12", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("starter12", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("starter12", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 540
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter12.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput07 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput07", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod badinput02 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput02", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod expert26 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("expert26", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert26.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert26.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("expert26", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("expert26", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("expert26", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 1225
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert26.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod junior14 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("junior14", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior14.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior14.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("junior14", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("junior14", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("junior14", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 855
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior14.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod master47 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("master47", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master47.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master47.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("master47", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("master47", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("master47", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 1965
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master47.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod junior13 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("junior13", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior13.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior13.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("junior13", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("junior13", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("junior13", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 266
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior13.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput10 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput10", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod expert36 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("expert36", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert36.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert36.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("expert36", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("expert36", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("expert36", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 634
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert36.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod trivial01 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("trivial01", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial01.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial01.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("trivial01", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("trivial01", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("trivial01", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 0
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial01.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput01 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput01", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod badinput06 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput06", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod badinput14 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput14", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod starter11 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("starter11", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter11.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter11.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("starter11", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("starter11", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("starter11", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 256
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter11.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput04 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput04", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod badinput15 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput15", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod junior24 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("junior24", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior24.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior24.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("junior24", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("junior24", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("junior24", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 9553
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior24.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod master38 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("master38", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master38.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master38.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("master38", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("master38", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("master38", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 5057
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master38.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod wizard59 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("wizard59", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard59.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard59.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("wizard59", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("wizard59", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("wizard59", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 5826
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard59.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput16 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput16", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod badinput12 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput12", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod expert35 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("expert35", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert35.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert35.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("expert35", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("expert35", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("expert35", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 1586
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert35.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput05 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput05", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod wizard60 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("wizard60", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard60.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard60.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("wizard60", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("wizard60", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("wizard60", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 81
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard60.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput09 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput09", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod starter01 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("starter01", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter01.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter01.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("starter01", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("starter01", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("starter01", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 29
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter01.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput19 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput19", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod junior20 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("junior20", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior20.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior20.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("junior20", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("junior20", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("junior20", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 386625
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior20.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod master37 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("master37", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master37.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master37.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("master37", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("master37", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("master37", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 4956
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master37.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod master48 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("master48", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master48.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master48.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("master48", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("master48", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("master48", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 1605
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/master48.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod wizard49 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("wizard49", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard49.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard49.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("wizard49", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("wizard49", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("wizard49", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 1378
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/wizard49.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod junior23 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("junior23", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior23.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior23.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("junior23", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("junior23", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("junior23", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 2769
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/junior23.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod starter02 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("starter02", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter02.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter02.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("starter02", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("starter02", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("starter02", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 783
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/starter02.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput03 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput03", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod badinput18 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput18", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod impossible {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("impossible", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/impossible.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/impossible.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("impossible", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("impossible", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 3675
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/impossible.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput17 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput17", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod trivial02 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("trivial02", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial02.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial02.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("trivial02", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("trivial02", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("trivial02", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 44
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial02.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod badinput08 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput08", parse_res())
    }
    fn parse_res() -> Result<JumpIN, JumpINParseError> {
        Err(JumpINParseError)
    }
}
mod expert25 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("expert25", Ok(puzzle()))
    }
    fn puzzle() -> JumpIN {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert25.puz.btc.z").unwrap(),
        ))
        .unwrap()
    }
    fn soln() -> Option<(Vec<<JumpIN as Puzzle>::Move>, JumpIN)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert25.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("expert25", puzzle(), soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("expert25", puzzle(), soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("expert25", &puzzle(), &move_tree())
    }
    fn move_tree() -> MoveTree<<JumpIN as Puzzle>::Move, JumpIN> {
        // move_tree.size() = 6241
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/expert25.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
