//! Weil-descended decomposition systems for P_1 + ... + P_m = R (P_i with x in V)
//! as a left chain of m - 1 merges, in four representations:
//!
//! - `Chain`: S_3(x_a, x_b, x_c) = 0 per merge (cubic in bits while x_a, x_b, x_c are unknown);
//! - `QuadGeneric`: Chain plus a structureless auxiliary w = x_a x_b (the confound control);
//! - `E2Aux`: the norm form of the line through the merge, auxiliary mu = e_2;
//! - `Line`: E2Aux plus the slope lambda with lambda^2 + lambda = x_a + x_b + x_c + a.
//!
//! Every auxiliary is introduced only on merges whose S_3 is cubic in bits (all three
//! abscissae unknown); the final merge has x_c = x_R known.

use crate::anf::{stats, Ctx, Fe, Stats};
use crate::curve::{Curve, Pt};
use crate::gf::{Gf, Rng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rep {
    Chain,
    QuadGeneric,
    E2Aux,
    Line,
}

impl Rep {
    pub const ALL: [Rep; 4] = [Rep::Chain, Rep::QuadGeneric, Rep::E2Aux, Rep::Line];
    pub fn name(self) -> &'static str {
        match self {
            Rep::Chain => "chain_s3",
            Rep::QuadGeneric => "quad_generic",
            Rep::E2Aux => "e2_aux",
            Rep::Line => "line",
        }
    }
}

/// A planted instance: factor-base bits, the points, and every intermediate sum.
pub struct Plant {
    pub basis: Vec<u64>,
    pub fb_bits: Vec<Vec<bool>>,
    pub pts: Vec<Pt>,
    pub sums: Vec<Pt>, // sums[k] = P_1 + ... + P_{k+2}; the last is R
}

pub fn random_basis(f: &Gf, l: usize, rng: &mut Rng) -> Vec<u64> {
    loop {
        let b: Vec<u64> = (0..l).map(|_| rng.nonzero(f)).collect();
        if rank(&b) == l {
            return b;
        }
    }
}

fn rank(v: &[u64]) -> usize {
    let mut rows = v.to_vec();
    let mut r = 0;
    for bit in (0..64).rev() {
        if let Some(p) = (r..rows.len()).find(|&i| rows[i] >> bit & 1 == 1) {
            rows.swap(r, p);
            for i in 0..rows.len() {
                if i != r && rows[i] >> bit & 1 == 1 {
                    rows[i] ^= rows[r];
                }
            }
            r += 1;
        }
    }
    r
}

fn x_of(p: Pt) -> Option<u64> {
    match p {
        Pt::A(x, _) => Some(x),
        Pt::Inf => None,
    }
}

/// Draw a basis and a relation whose intermediate sums are all affine with distinct
/// abscissae per merge; the basis is redrawn when it yields no such relation.
pub fn plant(c: &Curve, l: usize, m: usize, rng: &mut Rng) -> Plant {
    loop {
        let basis = random_basis(&c.f, l, rng);
        if let Some(p) = try_plant(c, &basis, m, rng) {
            return p;
        }
    }
}

fn try_plant(c: &Curve, basis: &[u64], m: usize, rng: &mut Rng) -> Option<Plant> {
    'retry: for _ in 0..4096 {
        let (mut bits, mut pts) = (vec![], vec![]);
        let mut draws = 0;
        while pts.len() < m {
            draws += 1;
            if draws > 4096 {
                return None;
            }
            let b: Vec<bool> = (0..basis.len()).map(|_| rng.next() & 1 == 1).collect();
            let x = basis
                .iter()
                .zip(&b)
                .filter(|(_, &on)| on)
                .fold(0, |a, (&v, _)| a ^ v);
            if let Some(p) = c.lift(x) {
                pts.push(if rng.next() & 1 == 1 { c.neg(p) } else { p });
                bits.push(b);
            }
        }
        let mut sums = vec![];
        let mut acc = pts[0];
        for p in &pts[1..] {
            let (xa, xb) = (x_of(acc), x_of(*p));
            if xa.is_none() || xa == xb {
                continue 'retry;
            }
            acc = c.add(acc, *p);
            if x_of(acc).is_none() {
                continue 'retry;
            }
            sums.push(acc);
        }
        return Some(Plant {
            basis: basis.to_vec(),
            fb_bits: bits,
            pts,
            sums,
        });
    }
    None
}

pub struct Built {
    pub eqs: Vec<Fe>,
    pub nvars: usize,
    pub planted: Vec<bool>,
}

fn push_value(planted: &mut Vec<bool>, f: &Gf, v: u64) {
    for i in 0..f.n {
        planted.push(v >> i & 1 == 1);
    }
}

pub fn build(c: &Curve, rep: Rep, p: &Plant) -> Built {
    let f = &c.f;
    let mut cx = Ctx::new(*f);
    let mut planted = vec![];
    let m = p.pts.len();
    let mut xs: Vec<Fe> = vec![];
    for i in 0..m {
        xs.push(cx.in_span(&p.basis));
        planted.extend(&p.fb_bits[i]);
    }
    let bconst = cx.constant(c.b);
    let aconst = cx.constant(c.a);
    let mut eqs = vec![];
    let mut cur = xs[0].clone();
    let mut cur_pt = p.pts[0];
    for k in 1..m {
        let last = k == m - 1;
        let sum_pt = p.sums[k - 1];
        let xc_val = x_of(sum_pt).unwrap();
        let xc = if last {
            cx.constant(xc_val)
        } else {
            let u = cx.free();
            push_value(&mut planted, f, xc_val);
            u
        };
        let (xa, xb) = (cur.clone(), xs[k].clone());
        let cubic = !last; // all three abscissae unknown
        let (xav, xbv) = (x_of(cur_pt).unwrap(), x_of(p.pts[k]).unwrap());
        let s3 = |cx: &Ctx, w: Option<&Fe>| -> Fe {
            // S_3 = e2^2 + x_a x_b x_c + b, with e2^2 = (x_a x_b)^2 + (x_a x_c)^2 + (x_b x_c)^2
            let ab = match w {
                Some(w) => w.clone(),
                None => cx.mul(&xa, &xb),
            };
            let sq = cx.add(
                &cx.add(&cx.sq(&ab), &cx.mul(&cx.sq(&xa), &cx.sq(&xc))),
                &cx.mul(&cx.sq(&xb), &cx.sq(&xc)),
            );
            cx.add(&cx.add(&sq, &cx.mul(&ab, &xc)), &bconst)
        };
        match rep {
            Rep::Chain => eqs.push(s3(&cx, None)),
            Rep::QuadGeneric => {
                if cubic {
                    let w = cx.free();
                    push_value(&mut planted, f, f.mul(xav, xbv));
                    eqs.push(cx.add(&w, &cx.mul(&xa, &xb)));
                    eqs.push(s3(&cx, Some(&w)));
                } else {
                    eqs.push(s3(&cx, None));
                }
            }
            Rep::E2Aux | Rep::Line => {
                if cubic || rep == Rep::Line {
                    let mu = cx.free();
                    let e2 = f.mul(xav, xbv) ^ f.mul(xav, xc_val) ^ f.mul(xbv, xc_val);
                    push_value(&mut planted, f, e2);
                    let apb = cx.add(&xa, &xb);
                    // (ii) x_a x_b + x_c (x_a + x_b) + mu = 0
                    eqs.push(cx.add(&cx.add(&cx.mul(&xa, &xb), &cx.mul(&xc, &apb)), &mu));
                    // (iii') mu x_c + x_c^2 (x_a + x_b) + mu^2 + b = 0
                    eqs.push(cx.add(
                        &cx.add(&cx.mul(&mu, &xc), &cx.mul(&cx.sq(&xc), &apb)),
                        &cx.add(&cx.sq(&mu), &bconst),
                    ));
                } else {
                    eqs.push(s3(&cx, None));
                }
                if rep == Rep::Line {
                    let lam = cx.free();
                    // slope of the line through cur_pt and p.pts[k] (it also meets -(sum))
                    let (ya, yb) = match (cur_pt, p.pts[k]) {
                        (Pt::A(_, ya), Pt::A(_, yb)) => (ya, yb),
                        _ => unreachable!(),
                    };
                    push_value(&mut planted, f, f.mul(ya ^ yb, f.inv(xav ^ xbv)));
                    // (i) x_a + x_b + x_c + a + lambda^2 + lambda = 0
                    let lhs = cx.add(&cx.add(&xa, &xb), &cx.add(&xc, &aconst));
                    eqs.push(cx.add(&lhs, &cx.add(&cx.sq(&lam), &lam)));
                }
            }
        }
        cur = xc;
        cur_pt = sum_pt;
    }
    Built {
        eqs,
        nvars: cx.nvars,
        planted,
    }
}

pub fn system_stats(b: &Built) -> Stats {
    stats(b.nvars, &b.eqs)
}

/// Every equation vanishes at the planted assignment.
pub fn holds(b: &Built) -> bool {
    b.eqs
        .iter()
        .all(|e| e.0.iter().all(|c| !c.eval(&b.planted)))
}
