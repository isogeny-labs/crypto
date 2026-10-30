//! Index-calculus driver (Go: solver.go).

use std::collections::HashMap;
use std::time::{Duration, Instant};

use num_bigint::BigUint;
use rand::rngs::StdRng;

use crate::curve::{Curve, Point, INFINITY};
use crate::field::{add_mod, inv_mod, mul_mod, rand_below, sub_mod};
use crate::gadgets::{bits_uint, concat, Arith, ArithRef};
use crate::linalg::left_kernel_vector_sparse;
use crate::pointgadget::gadget_relation_network;
use crate::relation::{
    mitm_find_relation, new_factor_bases, new_windowed_factor_bases, relation_network, FactorBases,
};
use crate::tensor::{find_witness, Counter, Network, NetworkStats};
use crate::{Error, Result};

/// Relation records [U]P + [V]Q = A_1 + ... + A_m.
#[derive(Debug, Clone, Default)]
pub struct Relation {
    pub u: u64,
    pub v: u64,
    pub points: Vec<Point>,
}

/// Stats describes one Solve call.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    /// N, the number of distinct factor-base points
    pub factor_base_size: usize,
    pub relations: usize,
    /// Targets tried, including ones with no decomposition
    pub attempts: usize,
    /// Fresh factor bases after V = 0 or a failed check
    pub restarts: usize,
    pub network: NetworkStats,
    pub elapsed: Duration,
}

/// Solver runs index calculus with m randomly hashed factor bases, finding
/// relations with a Boolean tensor contraction Counter.
pub struct Solver {
    pub curve: Curve,
    pub m: usize,
    pub counter: Box<dyn Counter>,
    pub rng: StdRng,
    /// MaxRestarts bounds the number of fresh factor-base draws (default 8
    /// when 0).
    pub max_restarts: usize,
    /// Gadgets uses affine factor bases and builds relation networks from the
    /// low-degree arithmetic gadgets (gadget_relation_network) instead of
    /// whole-point vertices. The Counter must then support hinted schedules,
    /// e.g. DFSCounter.
    pub gadgets: bool,
    /// WindowBits > 0 (with `gadgets`) adds the coordinate window x < p >> k
    /// to every factor base (with or without `gadgets`); see
    /// `new_windowed_factor_bases`.
    pub window_bits: usize,
    /// Mitm finds relations by meeting in the middle (`mitm_find_relation`)
    /// instead of contracting the network forward. It performs about
    /// sqrt(2r) point additions per attempt instead of r and does not use the
    /// counter.
    pub mitm: bool,
    /// Threads bounds the relation-search workers when `mitm` is set and no
    /// `opcount` tally is running. 0 means one per available core, 1 forces
    /// the sequential path. The results do not depend on it.
    pub threads: usize,
    /// Logf, if set, receives progress messages (already formatted; Go passes
    /// format + args).
    pub logf: Option<Box<dyn Fn(&str)>>,

    pub(crate) arith: Option<ArithRef>,
}

/// mitm_relation finds one decomposition of r and checks it: membership in
/// each F_i, every point on the curve, and the sum. Pure, so it can run on
/// worker threads.
fn mitm_relation(c: &Curve, fb: &FactorBases, r: Point) -> Result<Option<Relation>> {
    let pts = match mitm_find_relation(c, fb, r) {
        None => return Ok(None),
        Some(pts) => pts,
    };
    let mut rel = Relation::default();
    let mut sum = INFINITY;
    for (i, pt) in pts.iter().enumerate() {
        if !c.on_curve(*pt) || !fb.member(i, *pt) {
            return Err(Error::new("ecdl: mitm returned an invalid witness"));
        }
        rel.points.push(*pt);
        sum = c.add(sum, *pt);
    }
    if sum != r {
        return Err(Error::new(
            "ecdl: mitm returned a witness that does not sum to the target",
        ));
    }
    Ok(Some(rel))
}

/// parallel_mitm evaluates one target per element, keeping the results aligned
/// with the targets. The targets are fixed before dispatch, so the outcome
/// does not depend on the thread count.
fn parallel_mitm(
    c: &Curve,
    fb: &FactorBases,
    targets: &[(u64, u64, Point)],
    workers: usize,
    q: Point,
) -> Result<Vec<Option<Relation>>> {
    let n = targets.len();
    let mut out: Vec<Option<Relation>> = vec![None; n];
    let chunk = n.div_ceil(workers.max(1)).max(1);
    let mut rest: &mut [Option<Relation>] = &mut out;
    let mut start = 0;
    let failure = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        while start < n {
            let end = (start + chunk).min(n);
            let (head, tail) = rest.split_at_mut(end - start);
            rest = tail;
            let slice = &targets[start..end];
            handles.push(scope.spawn(move || -> Result<()> {
                for (slot, (u, v, _)) in head.iter_mut().zip(slice) {
                    let r = c.add(c.mul(*u, c.g), c.mul(*v, q));
                    *slot = mitm_relation(c, fb, r)?;
                }
                Ok(())
            }));
            start = end;
        }
        let mut failure: Option<Error> = None;
        for h in handles {
            match h.join() {
                Ok(Ok(())) => {}
                Ok(Err(e)) => failure = failure.or(Some(e)),
                Err(_) => {
                    failure = failure.or(Some(Error::new("ecdl: relation worker panicked")));
                }
            }
        }
        failure
    });
    match failure {
        Some(e) => Err(e),
        None => Ok(out),
    }
}

impl Solver {
    /// Convenience constructor (not in Go, which uses a struct literal):
    /// max_restarts = 0, gadgets = false, logf = None, arith = None.
    pub fn new(curve: Curve, m: usize, counter: Box<dyn Counter>, rng: StdRng) -> Solver {
        Solver {
            curve,
            m,
            counter,
            rng,
            max_restarts: 0,
            gadgets: false,
            window_bits: 0,
            mitm: false,
            threads: 0,
            logf: None,
            arith: None,
        }
    }

    /// Solve returns xi with Q = [xi]P, where P is the curve's generator. It
    /// only returns a value it has checked, so an answer is never wrong.
    /// Errors keep the Go messages ("ecdl: need m >= 2 factor bases", ...).
    /// Use [`Solver::solve_with_stats`] to also get the partial Stats on error,
    /// as Go's `(0, st, err)` return does.
    pub fn solve(&mut self, q: Point) -> Result<(u64, Stats)> {
        let (res, st) = self.solve_with_stats(q);
        res.map(|xi| (xi, st))
    }

    /// Same as [`Solver::solve`], but mirrors Go's `(xi, st, err)` triple: the
    /// Stats are returned on every path, including errors (where Elapsed is
    /// set only after the restarts are exhausted, as in Go).
    pub fn solve_with_stats(&mut self, q: Point) -> (Result<u64>, Stats) {
        let start = Instant::now();
        let mut st = Stats::default();
        let c = self.curve;
        if self.m < 2 {
            return (Err(Error::new("ecdl: need m >= 2 factor bases")), st);
        }
        if !c.on_curve(q) {
            return (Err(Error::new("ecdl: Q is not on the curve")), st);
        }
        if q.inf {
            return (Ok(0), st);
        }
        let max_restarts = if self.max_restarts == 0 {
            8
        } else {
            self.max_restarts
        };
        let r = c.order;
        for restart in 0..max_restarts {
            st.restarts = restart;

            // Step 1: factor-base selection.
            crate::opcount::set_phase("factor_base");
            // The coordinate window is a membership constraint on an affine
            // hash, so it applies whether or not the relation search uses
            // gadgets: it only changes how the sets are built and checked.
            let fb = if self.gadgets || self.window_bits > 0 {
                new_windowed_factor_bases(&c, self.m, &mut self.rng, self.window_bits)
            } else {
                new_factor_bases(&c, self.m, &mut self.rng)
            };
            st.factor_base_size = fb.points.len();
            let need = fb.points.len() + 1;
            self.logf(&format!(
                "restart {}: |F_i| = {}, N = {}, collecting {} relations",
                restart,
                fmt_ints(&set_sizes(&fb)),
                fb.points.len(),
                need
            ));

            // Step 2: relation finding by tensor contraction.
            let mut rels: Vec<Relation> = Vec::with_capacity(need);
            let workers = self.worker_count();
            while rels.len() < need {
                if self.mitm && !crate::opcount::active() {
                    // Draw the targets first, in order, so the batch is the
                    // same whatever the thread count; then evaluate them in
                    // parallel and keep the hits in target order.
                    let remaining = need - rels.len();
                    // The batch depends only on the remaining count, so every
                    // thread count draws the same targets.
                    let batch = (remaining * 4).clamp(1, 2048);
                    // Only the scalar draws are sequential; the target
                    // [u]P + [v]Q is computed inside the workers.
                    let mut targets: Vec<(u64, u64, Point)> = Vec::with_capacity(batch);
                    for _ in 0..batch {
                        let u = rand_below(&mut self.rng, r);
                        let v = rand_below(&mut self.rng, r);
                        targets.push((u, v, Point::default()));
                    }
                    st.attempts += batch;
                    crate::opcount::set_phase("relation_search");
                    let found = match parallel_mitm(&c, &fb, &targets, workers, q) {
                        Ok(f) => f,
                        Err(e) => return (Err(e), st),
                    };
                    for ((u, v, _), rel) in targets.into_iter().zip(found) {
                        let mut rel = match rel {
                            None => continue,
                            Some(rel) => rel,
                        };
                        rel.u = u;
                        rel.v = v;
                        rels.push(rel);
                    }
                    if rels.len().is_multiple_of(50) {
                        self.logf(&format!(
                            "  {}/{} relations after {} attempts",
                            rels.len(),
                            need,
                            st.attempts
                        ));
                    }
                    continue;
                }
                st.attempts += 1;
                let u = rand_below(&mut self.rng, r);
                let v = rand_below(&mut self.rng, r);
                crate::opcount::set_phase("relation_target");
                let target = c.add(c.mul(u, c.g), c.mul(v, q));
                crate::opcount::set_phase("relation_search");
                let (rel, ns) = match self.find_relation(&fb, target) {
                    Ok(v) => v,
                    Err(e) => return (Err(e), st),
                };
                st.network = ns;
                let mut rel = match rel {
                    None => continue,
                    Some(rel) => rel,
                };
                rel.u = u;
                rel.v = v;
                rels.push(rel);
                if rels.len().is_multiple_of(50) {
                    self.logf(&format!(
                        "  {}/{} relations after {} attempts",
                        rels.len(),
                        need,
                        st.attempts
                    ));
                }
            }
            st.relations += rels.len();

            // Step 3: linear algebra over F_r.
            crate::opcount::set_phase("linear_algebra");
            let mut rows: Vec<HashMap<usize, u64>> = Vec::with_capacity(rels.len());
            for rel in &rels {
                let mut row: HashMap<usize, u64> = HashMap::new();
                for pt in &rel.points {
                    let col = fb.index[pt];
                    let e = row.entry(col).or_insert(0);
                    *e = (*e + 1) % r;
                }
                rows.push(row);
            }
            let coef = match left_kernel_vector_sparse(&rows, fb.points.len(), r) {
                Some(c) => c,
                None => {
                    return (
                        Err(Error::new("ecdl: linear algebra found no dependency")),
                        st,
                    )
                }
            };
            let mut uu = 0u64;
            let mut vv = 0u64;
            for (j, &cj) in coef.iter().enumerate() {
                uu = add_mod(uu, mul_mod(cj, rels[j].u, r), r);
                vv = add_mod(vv, mul_mod(cj, rels[j].v, r), r);
            }

            // Step 4: [U]P + [V]Q = O, so xi = -U/V.
            if vv == 0 {
                self.logf("V = 0, restarting");
                continue;
            }
            let xi = sub_mod(0, mul_mod(uu, inv_mod(vv, r), r), r);
            crate::opcount::set_phase("recovery_check");
            if c.mul(xi, c.g) == q {
                st.elapsed = start.elapsed();
                return (Ok(xi), st);
            }
            self.logf("candidate failed verification, restarting");
        }
        st.elapsed = start.elapsed();
        (
            Err(Error::new(format!(
                "ecdl: no verified answer after {} restarts",
                max_restarts
            ))),
            st,
        )
    }

    /// findRelation looks for A_i in F_i with A_1 + ... + A_m = R, returning
    /// `None` if the contraction counts no solutions.
    /// worker_count is the number of relation-search threads to use.
    fn worker_count(&self) -> usize {
        if self.threads > 0 {
            return self.threads;
        }
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(32)
    }

    pub(crate) fn find_relation(
        &mut self,
        fb: &FactorBases,
        r: Point,
    ) -> Result<(Option<Relation>, NetworkStats)> {
        let c = self.curve;
        if self.mitm {
            return Ok((mitm_relation(&c, fb, r)?, NetworkStats::default()));
        }
        let net: Network;
        let a: Vec<Vec<usize>>;
        let decode: Box<dyn Fn(&[bool]) -> Option<Point>>;
        if self.gadgets {
            if self.arith.is_none() {
                self.arith = Some(Arith::new(&BigUint::from(c.p)));
            }
            let ar = self.arith.as_ref().unwrap().clone();
            let (n, pts) = gadget_relation_network(&c, &ar, fb, r);
            net = n;
            a = pts
                .iter()
                .map(|pt| concat(&[&pt.x.edges(), &pt.y.edges()]))
                .collect();
            let b = ar.bits;
            decode = Box::new(move |vals: &[bool]| {
                let low = |v: BigUint| v.to_u64_digits().first().copied().unwrap_or(0);
                let pt = Point {
                    x: low(bits_uint(&vals[..b])),
                    y: low(bits_uint(&vals[b..])),
                    inf: false,
                };
                if c.on_curve(pt) {
                    Some(pt)
                } else {
                    None
                }
            });
        } else {
            let (n, blocks) = relation_network(&c, fb, r);
            net = n;
            a = blocks;
            decode = Box::new(move |vals: &[bool]| c.decode_point(vals));
        }
        let ns = net.stats();
        let mut edges: Vec<usize> = Vec::new();
        for blk in &a {
            edges.extend_from_slice(blk);
        }
        let fixed = match find_witness(self.counter.as_mut(), &net, &edges)? {
            None => return Ok((None, ns)),
            Some(f) => f,
        };
        let mut rel = Relation::default();
        let mut sum = INFINITY;
        for (i, blk) in a.iter().enumerate() {
            let vals: Vec<bool> = blk.iter().map(|e| fixed[e]).collect();
            let pt = match decode(&vals) {
                Some(pt) if fb.member(i, pt) => pt,
                _ => return Err(Error::new("ecdl: counter returned an invalid witness")),
            };
            rel.points.push(pt);
            sum = c.add(sum, pt);
        }
        if sum != r {
            return Err(Error::new(
                "ecdl: counter returned a witness that does not sum to the target",
            ));
        }
        Ok((Some(rel), ns))
    }

    pub(crate) fn logf(&self, msg: &str) {
        if let Some(f) = &self.logf {
            f(msg);
        }
    }
}

/// Sizes of the F_i (Go: setSizes).
pub(crate) fn set_sizes(fb: &FactorBases) -> Vec<usize> {
    fb.sets.iter().map(|s| s.len()).collect()
}

/// Formats like Go's `%v` of a `[]int`: `[1 2 3]`.
fn fmt_ints(v: &[usize]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(" "))
}
