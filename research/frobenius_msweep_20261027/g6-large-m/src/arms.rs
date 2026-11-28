//! Arms: slot point sets for the ordered-slot (rotated-partition) design.
//! Slot i is a coordinate subspace V_i of the field with x in V_i (x = 0 gives the 2-torsion point T);
//! its point set S_i = {points with x in V_i}. Dimensions are the balanced split of n into m parts, identical in every arm.
//!
//! NI   normal basis, interleaved partition slot_i = {c : c = i mod m}  (the n83_rotated_m10 rule: conjugates[m*row+slot])
//! NC   normal basis, contiguous partition
//! NR   normal basis, random coordinate partition
//! ONB  type-II optimal normal basis, interleaved (n = 11, 23 only)
//! UP   generic (non-normal) basis, slots redrawn so that each slot has exactly the size of the paired NI/ONB slot
//! UP2  an independent second UP draw (A/A arm)
//! DUP  UP with one slot replaced by a copy of another (positive control: forced exchange symmetry)

use crate::curve::{Curve, Pt};
use crate::field::Fq;
use crate::normal;
use crate::rng::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arm {
    NI,
    NC,
    NR,
    ONB,
    UP,
    UP2,
    DUP,
}

impl Arm {
    pub fn name(&self) -> &'static str {
        match self {
            Arm::NI => "NI",
            Arm::NC => "NC",
            Arm::NR => "NR",
            Arm::ONB => "ONB",
            Arm::UP => "UP",
            Arm::UP2 => "UP2",
            Arm::DUP => "DUP",
        }
    }
}

/// Balanced slot dimensions: n = sum, the first n mod m slots one larger.
pub fn dims(n: usize, m: usize) -> Vec<usize> {
    assert!(m >= 1 && m <= n);
    (0..m).map(|i| n / m + (i < n % m) as usize).collect()
}

pub fn partition_interleaved(n: usize, m: usize) -> Vec<Vec<usize>> {
    (0..m).map(|i| (i..n).step_by(m).collect()).collect()
}

pub fn partition_contiguous(n: usize, m: usize) -> Vec<Vec<usize>> {
    let d = dims(n, m);
    let mut s = 0;
    d.iter()
        .map(|&k| {
            let v: Vec<usize> = (s..s + k).collect();
            s += k;
            v
        })
        .collect()
}

pub fn partition_random(n: usize, m: usize, rng: &mut Rng) -> Vec<Vec<usize>> {
    let mut idx: Vec<usize> = (0..n).collect();
    rng.shuffle(&mut idx);
    let d = dims(n, m);
    let mut s = 0;
    d.iter()
        .map(|&k| {
            let v = idx[s..s + k].to_vec();
            s += k;
            v
        })
        .collect()
}

fn slots_from(c: &Curve, basis: &[u32], part: &[Vec<usize>]) -> Vec<Vec<Pt>> {
    part.iter()
        .map(|cs| {
            let vs: Vec<u32> = cs.iter().map(|&i| basis[i]).collect();
            c.points_for_xs(&normal::span(&vs))
        })
        .collect()
}

/// NI / NC / NR: a random normal basis (seeded) with the named partition.
pub fn build_normal(f: &Fq, c: &Curve, arm: Arm, m: usize, seed: u64) -> Vec<Vec<Pt>> {
    let n = f.n as usize;
    let mut rng = Rng::new(seed);
    let basis = normal::random_normal(f, &mut rng);
    let part = match arm {
        Arm::NI => partition_interleaved(n, m),
        Arm::NC => partition_contiguous(n, m),
        Arm::NR => partition_random(n, m, &mut rng),
        _ => panic!("build_normal: not a normal-basis arm"),
    };
    slots_from(c, &basis, &part)
}

/// A generic basis (no Frobenius structure), interleaved dims, NO profile matching: used only for the size census `U`.
pub fn build_generic(f: &Fq, c: &Curve, m: usize, seed: u64) -> Vec<Vec<Pt>> {
    let mut rng = Rng::new(seed);
    let basis = normal::random_basis(f, &mut rng);
    slots_from(c, &basis, &partition_interleaved(f.n as usize, m))
}

/// The type-II ONB with the interleaved partition, or None when the construction does not apply.
pub fn build_onb(f: &Fq, c: &Curve, m: usize) -> Option<Vec<Vec<Pt>>> {
    let basis = normal::type2_onb(f)?;
    Some(slots_from(
        c,
        &basis,
        &partition_interleaved(f.n as usize, m),
    ))
}

pub fn sizes(slots: &[Vec<Pt>]) -> Vec<usize> {
    slots.iter().map(|s| s.len()).collect()
}

pub fn log2_tuples(sz: &[usize]) -> f64 {
    sz.iter().map(|&s| (s as f64).log2()).sum()
}

/// A generic-basis partition whose slot i has exactly profile[i] points (rejection per slot), the slots being
/// independent coordinate subspaces of dimensions dims(n, m) that together span the field. None if it gives up.
pub fn build_matched_generic(
    f: &Fq,
    c: &Curve,
    m: usize,
    seed: u64,
    profile: &[usize],
) -> Option<Vec<Vec<Pt>>> {
    let n = f.n as usize;
    let d = dims(n, m);
    assert_eq!(profile.len(), m);
    let mut rng = Rng::new(seed);
    'attempt: for _ in 0..400 {
        let mut all: Vec<u32> = Vec::new();
        let mut slots: Vec<Vec<Pt>> = Vec::new();
        for i in 0..m {
            let mut found = false;
            for _ in 0..20000 {
                let mut vs: Vec<u32> = Vec::new();
                while vs.len() < d[i] {
                    let x = rng.below(f.q as u64) as u32;
                    let mut w = vs.clone();
                    w.push(x);
                    if normal::rank_f2(&w) == w.len() {
                        vs.push(x);
                    }
                }
                let pts = c.points_for_xs(&normal::span(&vs));
                if pts.len() == profile[i] {
                    all.extend_from_slice(&vs);
                    slots.push(pts);
                    found = true;
                    break;
                }
            }
            if !found {
                return None;
            }
        }
        if normal::rank_f2(&all) == n {
            return Some(slots);
        }
        continue 'attempt;
    }
    None
}

/// Positive control: replace slot j by a copy of slot i, for the pair i < j with the closest sizes (ties: the larger slots, so that
/// the control is not a no-op on two single-point slots; the result equals `base` only if every slot has one point).
pub fn build_dup(base: &[Vec<Pt>]) -> Vec<Vec<Pt>> {
    let m = base.len();
    let (mut bi, mut bj, mut best) = (0, 1, usize::MAX);
    for i in 0..m {
        for j in i + 1..m {
            let dsz = base[i].len().abs_diff(base[j].len());
            let larger = base[i].len().min(base[j].len()) > base[bi].len().min(base[bj].len());
            if dsz < best || (dsz == best && larger) {
                best = dsz;
                bi = i;
                bj = j;
            }
        }
    }
    let mut v = base.to_vec();
    v[bj] = base[bi].clone();
    v
}
