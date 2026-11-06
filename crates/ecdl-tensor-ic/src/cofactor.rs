//! Fixed-cofactor curves (Rust only; the Go prototype handles prime-order
//! curves alone). Design: `research/ecdl_tensor_ic_cofactor_20261106/DESIGN.md`.
//!
//! The group E(F_p) has order c·r with r prime and r > c, so gcd(c, r) = 1.
//! The generator and every target lie in the order-r subgroup G = [c]E.
//! [`Curve`] keeps its meaning: `order` is r and `g` lies in G. The
//! cofactor travels beside it in [`CofactorCurve`].
//!
//! [c] maps E onto G, and its kernel is the c points of order dividing c, so
//! every point of G has exactly c preimages in E. A relation network whose
//! slots take a free S_i ∈ E and constrain A_i = [c]S_i therefore counts
//! exactly c^m times as many assignments as the same network over the A_i,
//! while every witness still projects onto a relation in G.

use std::collections::HashSet;
use std::rc::Rc;

use rand::rngs::StdRng;

use crate::curve::{Curve, Point};
use crate::field::{add_mod, is_prime, mul_mod, pow_mod, rand_below};
use crate::relation::{new_affine_factor_bases, new_factor_bases, FactorBases};
use crate::tensor::{bits_key, FuncVertex, Network, PredVertex, SupportVertex, Vertex};
use crate::{Error, Result};

/// A curve with group order `cofactor · curve.order`, generator in G.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CofactorCurve {
    pub curve: Curve,
    pub cofactor: u64,
}

impl CofactorCurve {
    /// #E(F_p) = c · r.
    pub fn group_order(&self) -> u64 {
        self.cofactor * self.curve.order
    }

    /// Reports whether pt is a point of E lying in the order-r subgroup G.
    pub fn in_subgroup(&self, pt: Point) -> bool {
        self.curve.on_curve(pt) && self.curve.mul(self.curve.order, pt).inf
    }

    /// The projection A = [c]S onto G.
    pub fn project(&self, s: Point) -> Point {
        self.curve.mul(self.cofactor, s)
    }

    /// Every affine point of E with [c]S = a (toy sizes: enumerates E).
    pub fn preimages(&self, a: Point) -> Vec<Point> {
        self.curve
            .points()
            .into_iter()
            .filter(|&s| self.project(s) == a)
            .collect()
    }
}

/// Searches a random b-bit prime field for a curve with #E = c·r, r prime and
/// r > c, by naive point counting, and takes G = [c]S ≠ O for a random point
/// S. Toy sizes only.
pub fn random_cofactor_curve(rng: &mut StdRng, b: usize, cofactor: u64) -> Result<CofactorCurve> {
    if !(4..=26).contains(&b) {
        return Err(Error::new(
            "ecdl: field size must be between 4 and 26 bits for naive point counting",
        ));
    }
    if cofactor < 2 {
        return Err(Error::new(
            "ecdl: cofactor must be at least 2 (use random_prime_order_curve)",
        ));
    }
    let lo = 1u64 << (b - 1);
    for _tries in 0..100000 {
        let p = lo + rand_below(rng, lo);
        if p <= 3 || !is_prime(p) {
            continue;
        }
        let mut c = Curve {
            p,
            a: rand_below(rng, p),
            b: rand_below(rng, p),
            order: 0,
            g: Point::default(),
        };
        // 4a^3 + 27b^2 != 0 (mod p)
        let disc = add_mod(
            mul_mod(4, pow_mod(c.a, 3, p), p),
            mul_mod(27, mul_mod(c.b, c.b, p), p),
            p,
        );
        if disc == 0 {
            continue;
        }
        let pts = c.points();
        let n = pts.len() as u64 + 1;
        if !n.is_multiple_of(cofactor) {
            continue;
        }
        let r = n / cofactor;
        if r <= cofactor || !is_prime(r) {
            continue;
        }
        c.order = r;
        let cc = CofactorCurve { curve: c, cofactor };
        loop {
            let s = pts[rand_below(rng, pts.len() as u64) as usize];
            let g = cc.project(s);
            if !g.inf {
                c.g = g;
                return Ok(CofactorCurve { curve: c, cofactor });
            }
        }
    }
    Err(Error::new(
        "ecdl: no curve with the requested cofactor found",
    ))
}

/// Restricts listed factor bases to G: drops every point outside the
/// subgroup from each F_i and rebuilds `points` and `index`.
pub fn restrict_to_subgroup(fb: &mut FactorBases, cc: &CofactorCurve) {
    for set in fb.sets.iter_mut() {
        set.retain(|&pt| cc.in_subgroup(pt));
    }
    fb.points.clear();
    fb.index.clear();
    for set in &fb.sets {
        for &pt in set {
            if !fb.index.contains_key(&pt) {
                fb.index.insert(pt, fb.points.len());
                fb.points.push(pt);
            }
        }
    }
}

/// Factor bases over G. They are drawn as for a prime-order curve of order r
/// (so each hash keeps about c·target members on E, about target of them in
/// G), then restricted to G, and redrawn while any F_i ∩ G is empty.
/// `affine` selects the affine hash the gadget path needs.
pub fn cofactor_factor_bases(
    cc: &CofactorCurve,
    m: usize,
    rng: &mut StdRng,
    affine: bool,
) -> FactorBases {
    loop {
        let mut fb = if affine {
            new_affine_factor_bases(&cc.curve, m, rng)
        } else {
            new_factor_bases(&cc.curve, m, rng)
        };
        restrict_to_subgroup(&mut fb, cc);
        if fb.sets.iter().all(|set| !set.is_empty()) {
            return fb;
        }
    }
}

/// The whole-point relation network with free sources: edges are the bits
/// of S_1..S_m, of A_1..A_m and of the partial sums. Vertices are
///   - source(i): S_i is an affine point of E (a support tensor over all of
///     E; the toy enumerates the group, as factor-base listing does);
///   - project(i): A_i = [c]S_i, accepted only when F_i's membership
///     predicate holds on A_i (a function tensor);
///   - add(k) and the target predicate, as in [`crate::relation_network`].
///
/// Every edge has exactly two endpoints. Returns the network, the edge
/// indices of each S_i and those of each A_i.
pub fn cofactor_relation_network(
    cc: &CofactorCurve,
    fb: &FactorBases,
    r: Point,
) -> (Network, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let c = cc.curve;
    let w = c.point_width();
    let m = fb.sets.len();
    let mut next = 0usize;
    let mut block = || -> Vec<usize> {
        let e: Vec<usize> = (next..next + w).collect();
        next += w;
        e
    };
    let s_blocks: Vec<Vec<usize>> = (0..m).map(|_| block()).collect();
    let a_blocks: Vec<Vec<usize>> = (0..m).map(|_| block()).collect();
    let sources: Vec<Vec<bool>> = c.points().iter().map(|&pt| c.encode_point(pt)).collect();
    let mut n = Network::default();
    for i in 0..m {
        n.vertices.push(Vertex::Support(SupportVertex::new(
            s_blocks[i].clone(),
            sources.clone(),
        )));
        let cc = *cc;
        let members: HashSet<Point> = fb.sets[i].iter().copied().collect();
        n.vertices.push(Vertex::Func(FuncVertex {
            in_: s_blocks[i].clone(),
            out: a_blocks[i].clone(),
            f: Rc::new(move |inp: &[bool]| -> Option<Vec<bool>> {
                let a = cc.project(cc.curve.decode_point(inp)?);
                members.contains(&a).then(|| cc.curve.encode_point(a))
            }),
        }));
    }
    let mut s = a_blocks[0].clone();
    for k in 1..m {
        let out = block();
        let mut in_ = s.clone();
        in_.extend_from_slice(&a_blocks[k]);
        n.vertices.push(Vertex::Func(FuncVertex {
            in_,
            out: out.clone(),
            f: Rc::new(move |inp: &[bool]| -> Option<Vec<bool>> {
                let s = c.decode_point(&inp[..w]);
                let a = c.decode_point(&inp[w..]);
                match (s, a) {
                    (Some(s), Some(a)) => Some(c.encode_point(c.add(s, a))),
                    _ => None,
                }
            }),
        }));
        s = out;
    }
    let want = bits_key(&c.encode_point(r));
    n.vertices.push(Vertex::Pred(PredVertex {
        e: s,
        p: Rc::new(move |vals: &[bool]| bits_key(vals) == want),
    }));
    n.num_edges = next;
    (n, s_blocks, a_blocks)
}
