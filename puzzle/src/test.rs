use super::Puzzle;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTree<M, V>(Option<Vec<(M, (V, MoveTree<M, V>))>>);

impl<M, V> MoveTree<M, V> {
    pub fn size(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(kvs) => kvs
                .iter()
                .fold(0, |size, (_, (_, nt))| size + 1 + nt.size()),
        }
    }

    pub fn build<P, F>(d: usize, p: &P, f: F) -> MoveTree<M, V>
    where
        P: Puzzle<Move = M>,
        F: Fn(P) -> V,
    {
        MoveTree::build_aux(d, p, &f)
    }

    fn build_aux<P, F>(d: usize, p: &P, f: &F) -> MoveTree<M, V>
    where
        P: Puzzle<Move = M>,
        F: Fn(P) -> V,
    {
        if d == 0 {
            MoveTree(None)
        } else {
            MoveTree(Some(
                p.next()
                    .into_iter()
                    .map(|(m, q)| {
                        let nt = MoveTree::build_aux(d - 1, &q, f);
                        (m, (f(q), nt))
                    })
                    .collect(),
            ))
        }
    }
}

pub enum MoveTreeVerifyError<M, E> {
    MissingMove(M),
    DuplicateMove(M),
    ExtraMove(M),
    ChkState(M, E),
}

impl<M, V> MoveTree<M, V> {
    pub fn verify<P, F, E>(&self, p: &P, chk: F) -> Result<(), (Vec<M>, MoveTreeVerifyError<M, E>)>
    where
        M: Eq + Clone,
        P: Puzzle<Move = M>,
        F: Fn(&P, &V) -> Result<(), E>,
    {
        self.verify_rec(&mut Vec::new(), p, &chk)
    }

    fn verify_rec<P, F, E>(
        &self,
        prefix: &mut Vec<M>,
        p: &P,
        chk: &F,
    ) -> Result<(), (Vec<M>, MoveTreeVerifyError<M, E>)>
    where
        M: Eq + Clone,
        P: Puzzle<Move = M>,
        F: Fn(&P, &V) -> Result<(), E>,
    {
        match &self.0 {
            None => Ok(()),
            Some(kvs) => {
                let mqs = p.next();
                for (k, _) in kvs.iter() {
                    if mqs.iter().all(|(m, _)| k != m) {
                        let prefix: Vec<P::Move> = prefix.drain(..).collect();
                        return Err((prefix, MoveTreeVerifyError::MissingMove(k.clone())));
                    }
                }
                for (m, _) in mqs.iter() {
                    if mqs.iter().filter(|(n, _)| m == n).count() > 1 {
                        let prefix: Vec<P::Move> = prefix.drain(..).collect();
                        return Err((prefix, MoveTreeVerifyError::DuplicateMove(m.clone())));
                    }
                }
                for (m, q) in mqs.into_iter() {
                    match kvs.iter().find(|(k, _)| &m == k) {
                        None => {
                            let prefix: Vec<P::Move> = prefix.drain(..).collect();
                            return Err((prefix, MoveTreeVerifyError::ExtraMove(m)));
                        }
                        Some((_, (v, nt))) => {
                            match chk(&q, v) {
                                Ok(()) => {}
                                Err(err) => {
                                    let prefix: Vec<P::Move> = prefix.drain(..).collect();
                                    return Err((prefix, MoveTreeVerifyError::ChkState(m, err)));
                                }
                            };
                            prefix.push(m);
                            nt.verify_rec(prefix, &q, chk)?;
                            prefix.pop();
                        }
                    }
                }
                Ok(())
            }
        }
    }
}
