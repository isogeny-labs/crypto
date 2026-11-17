//! Hinted schedules and the depth-first counter (Go: schedule.go).

use std::collections::HashMap;
use std::rc::Rc;

use crate::tensor::{bit, Counter, Network, Vertex};
use crate::{Error, Result};

/// Step is one entry of a hinted schedule: a vertex to evaluate, or a hint.
/// `Vertex(i)` indexes `Network::vertices` (Go stores the vertex pointer).
#[derive(Clone)]
pub enum Step {
    Vertex(usize),
    Hint(EdgeHint),
}

/// Closure type of `EdgeHint::gen`: values of the Reads edges -> candidate
/// values (one `Vec<bool>` per candidate, aligned with Writes).
pub type HintGenFn = Rc<dyn Fn(&[bool]) -> Vec<Vec<bool>>>;

/// EdgeHint proposes candidate values for the Writes edges given the values of
/// the Reads edges. A hint is not a constraint: the count is defined by the
/// vertices alone. Hints only steer enumeration, so they must be complete:
/// every satisfying assignment must agree with one of the candidates. The
/// gadgets in this package use hints for witnesses that the vertices pin down
/// uniquely (quotients, inverses, field outputs), and their completeness is
/// tested exhaustively at small sizes.
#[derive(Clone)]
pub struct EdgeHint {
    pub reads: Vec<usize>,
    pub writes: Vec<usize>,
    pub gen: HintGenFn,
}

/// DFSCounter counts satisfying assignments by depth-first enumeration along
/// the network's schedule: support vertices and hints branch, function
/// vertices extend the assignment, predicate vertices prune. It handles
/// networks with or without a Schedule (empty schedule = every vertex in
/// order). Like SparseContractor it is exact and exponential; it stands in for
/// a real contraction solver. Errors: "ecdl: schedule reads edge {} before it
/// is bound", "ecdl: schedule step index out of range".
#[derive(Debug, Clone, Copy, Default)]
pub struct DFSCounter;

/// Go: type dfs. `assign`: -1 unbound, 0, 1.
struct Dfs<'a> {
    sched: Vec<Step>,
    vertices: &'a [Vertex],
    assign: Vec<i8>,
    fixed: &'a HashMap<usize, bool>,
    trail: Vec<usize>,
}

impl Counter for DFSCounter {
    fn count(&mut self, n: &Network, fixed: &HashMap<usize, bool>) -> Result<u64> {
        let sched: Vec<Step> = if n.schedule.is_empty() {
            (0..n.vertices.len()).map(Step::Vertex).collect()
        } else {
            n.schedule.clone()
        };
        let mut touched = vec![false; n.num_edges];
        for v in &n.vertices {
            for e in v.edges() {
                touched[e] = true;
            }
        }
        let mut d = Dfs {
            sched,
            vertices: &n.vertices,
            assign: vec![-1i8; n.num_edges],
            fixed,
            trail: Vec::new(),
        };
        let mut total = d.run(0)?;
        for (e, &t) in touched.iter().enumerate() {
            if !t && !fixed.contains_key(&e) {
                total = total.wrapping_mul(2);
            }
        }
        Ok(total)
    }
}

impl<'a> Dfs<'a> {
    /// bind assigns vals to edges, failing on a clash with an existing or fixed
    /// value. Newly bound edges are pushed on the trail.
    fn bind(&mut self, edges: &[usize], vals: &[bool]) -> bool {
        for (i, &e) in edges.iter().enumerate() {
            let b = vals[i];
            if let Some(&fb) = self.fixed.get(&e) {
                if fb != b {
                    return false;
                }
            }
            let bv = bit(b) as i8;
            if self.assign[e] == -1 {
                self.assign[e] = bv;
                self.trail.push(e);
            } else if self.assign[e] != bv {
                return false;
            }
        }
        true
    }

    fn undo(&mut self, mark: usize) {
        for &e in &self.trail[mark..] {
            self.assign[e] = -1;
        }
        self.trail.truncate(mark);
    }

    fn read(&self, edges: &[usize]) -> Result<Vec<bool>> {
        let mut vals = Vec::with_capacity(edges.len());
        for &e in edges {
            if self.assign[e] < 0 {
                return Err(Error::new(format!(
                    "ecdl: schedule reads edge {} before it is bound",
                    e
                )));
            }
            vals.push(self.assign[e] == 1);
        }
        Ok(vals)
    }

    /// Go uses `defer d.undo(mark)`; the Rust body must undo on every exit.
    fn run(&mut self, i: usize) -> Result<u64> {
        if i == self.sched.len() {
            return Ok(1);
        }
        let mark = self.trail.len();
        let r = self.run_step(i, mark);
        self.undo(mark);
        r
    }

    fn run_step(&mut self, i: usize, mark: usize) -> Result<u64> {
        let st = self.sched[i].clone();
        let vertices = self.vertices;
        match st {
            Step::Hint(h) => {
                let inp = self.read(&h.reads)?;
                let mut total: u64 = 0;
                for cand in (h.gen)(&inp) {
                    if self.bind(&h.writes, &cand) {
                        let k = self.run(i + 1)?;
                        total = total.wrapping_add(k);
                    }
                    self.undo(mark);
                }
                Ok(total)
            }
            Step::Vertex(vi) => match vertices.get(vi) {
                None => Err(Error::new("ecdl: schedule step index out of range")),
                Some(Vertex::Support(v)) => {
                    let mut total: u64 = 0;
                    for row in &v.rows {
                        if self.bind(&v.e, row) {
                            let k = self.run(i + 1)?;
                            total = total.wrapping_add(k);
                        }
                        self.undo(mark);
                    }
                    Ok(total)
                }
                Some(Vertex::Func(v)) => {
                    let inp = self.read(&v.in_)?;
                    match (v.f)(&inp) {
                        Some(out) if self.bind(&v.out, &out) => self.run(i + 1),
                        _ => Ok(0),
                    }
                }
                Some(Vertex::Pred(v)) => {
                    let vals = self.read(&v.e)?;
                    if !(v.p)(&vals) {
                        return Ok(0);
                    }
                    self.run(i + 1)
                }
            },
        }
    }
}
