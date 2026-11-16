//! Bounded, complete finite-instance index-calculus control on the ordinary
//! prime31 full-prime-order curve. Reuses Arithmetic, Base and certificate from
//! the native Rust adapter. The 4+4 witness finder is explicitly GENERIC
//! group meet-in-the-middle, not the sought algebraic PKM/F4 oracle.
//!
//! Run separately from development tests:
//! cargo +1.100.0 run --release --locked --manifest-path \
//!   research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_split_ic
//! All claims remain [pcc:unverified] until independent replay and cost gates.
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::collections::HashMap;

const Q: Point = Some((1_547_953_785, 1_667_575_595));
const VERIFIER_SCALAR_ONLY: u64 = 661_068_045;
const MAX_QUERIES: usize = 256;
const IDENTITY_KEY: u64 = u64::MAX;

fn mul_mod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

fn sub_mod(a: u64, b: u64, n: u64) -> u64 {
    (a + n - b) % n
}

fn inv_mod(value: u64, n: u64) -> u64 {
    assert_ne!(value, 0);
    let (mut out, mut x, mut e) = (1_u64, value, n - 2);
    while e > 0 {
        if e & 1 != 0 {
            out = mul_mod(out, x, n);
        }
        x = mul_mod(x, x, n);
        e >>= 1;
    }
    out
}

fn point_key(point: Point) -> u64 {
    match point {
        None => IDENTITY_KEY,
        Some((x, y)) => {
            assert!(x < (1_u64 << 31) && y < (1_u64 << 31));
            (x << 31) | y
        }
    }
}

// The low 24 bits encode four distinct point indices in 6 bits each;
// bits 24..27 encode negative signs, one bit per point.
fn four_code(indices: [usize; 4], mask: u8) -> u64 {
    assert!(mask < 16);
    assert!(indices.windows(2).all(|w| w[0] < w[1]));
    assert!(indices[3] < 64);
    indices[0] as u64
        | ((indices[1] as u64) << 6)
        | ((indices[2] as u64) << 12)
        | ((indices[3] as u64) << 18)
        | ((mask as u64) << 24)
}

fn decode_four(code: u64) -> [(usize, i8); 4] {
    std::array::from_fn(|j| {
        let index = ((code >> (6 * j)) & 63) as usize;
        let sign = if (code >> (24 + j)) & 1 == 0 { 1 } else { -1 };
        (index, sign)
    })
}

fn signed_four(code: u64, base: &Base, ar: &mut Arithmetic) -> Point {
    let mut out = None;
    for (i, sign) in decode_four(code) {
        let p = Some(base.points[i]);
        out = ar.add(PRIME31, out, if sign < 0 { ar.neg(PRIME31, p) } else { p });
    }
    out
}

#[derive(Clone, Copy)]
struct Four {
    point: Point,
    code: u64,
}

// Every signed sum of four distinct points from a fixed half.
// Only these two independently chosen halves are searched; this is not the
// entire 8-summand relation space. Enumeration and selection are charged.
fn enumerate_four(
    base: &Base,
    start: usize,
    end: usize,
    ar: &mut Arithmetic,
) -> Vec<Four> {
    let mut result = Vec::new();
    for i in start..end {
        for j in i + 1..end {
            for k in j + 1..end {
                for l in k + 1..end {
                    let indices = [i, j, k, l];
                    for mask in 0..16_u8 {
                        let code = four_code(indices, mask);
                        result.push(Four {
                            point: signed_four(code, base, ar),
                            code,
                        });
                    }
                }
            }
        }
    }
    result
}

fn fill_row(row: &mut [u64], code: u64, n: u64) {
    for (index, sign) in decode_four(code) {
        row[index] = if sign == 1 {
            (row[index] + 1) % n
        } else {
            sub_mod(row[index], 1, n)
        };
    }
}

// Gaussian incremental elimination over the curve's prime group order.
// Rows are kept in increasing pivot order; last column is the RHS.
// This is the actual index-calculus linear stage, not a known-log lookup.
struct Linear {
    dim: usize,
    pivots: Vec<(usize, Vec<u64>)>,
}
impl Linear {
    fn new(dim: usize) -> Self {
        Self { dim, pivots: Vec::new() }
    }
    fn rank(&self) -> usize {
        self.pivots.len()
    }
    fn insert(&mut self, mut row: Vec<u64>, n: u64) -> Result<bool, String> {
        if row.len() != self.dim + 1 {
            return Err("wrong affine relation dimension".into());
        }
        for (j, pivot) in &self.pivots {
            let k = row[*j];
            if k != 0 {
                for c in *j..=self.dim {
                    row[c] = sub_mod(row[c], mul_mod(k, pivot[c], n), n);
                }
            }
        }
        let found = (0..self.dim).find(|&j| row[j] != 0);
        let Some(j) = found else {
            if row[self.dim] != 0 {
                return Err("inconsistent index-calculus relation".into());
            }
            return Ok(false);
        };
        let inv = inv_mod(row[j], n);
        for x in &mut row[j..] {
            *x = mul_mod(*x, inv, n);
        }
        let pos = self.pivots.partition_point(|(k, _)| *k < j);
        self.pivots.insert(pos, (j, row));
        Ok(true)
    }
    fn solve(&self, n: u64) -> Result<Vec<u64>, String> {
        if self.rank() != self.dim {
            return Err("insufficient rank to recover target and basis logs".into());
        }
        let mut solution = vec![0_u64; self.dim];
        for (j, row) in self.pivots.iter().rev() {
            let mut val = row[self.dim];
            for k in (*j + 1)..self.dim {
                val = sub_mod(val, mul_mod(row[k], solution[k], n), n);
            }
            solution[*j] = val;
        }
        Ok(solution)
    }
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self, modulus: u64) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        (z ^ (z >> 31)) % modulus
    }
}

fn main() -> Result<(), String> {
    let e = PRIME31;
    let mut ar = Arithmetic::default();
    check_curve(e, &mut ar)?;
    if Q.is_none() || !ar.on_curve(e, Q) {
        return Err("invalid public unknown-scalar target".into());
    }
    let base = Base::build(e, 6, &mut ar)?;
    if base.terminal != 933_677_148 || base.points.len() != 31 {
        return Err("prime31 Dickson depth-6 fixture changed".into());
    }
    let bsize = base.points.len();
    let midpoint = bsize / 2;
    let setup_before = ar.counts.group_additions;
    let left = enumerate_four(&base, 0, midpoint, &mut ar);
    let right = enumerate_four(&base, midpoint, bsize, &mut ar);
    let setup_additions = ar.counts.group_additions - setup_before;
    let mut first = HashMap::<u64, u64>::new();
    for f in &left {
        first.entry(point_key(f.point)).or_insert(f.code);
    }
    if left.len() != 21_840 || first.len() != 21_840 || right.len() != 29_120 {
        return Err("signed 4-sum preflight cardinality changed".into());
    }

    // Public mask construction: target scalar is never supplied to the
    // relation finder, elimination, or recovery. Only a,b and Q enter R.
    let mut masks = SplitMix(0x4d38_4e41_5455_5241);
    let mut linear = Linear::new(bsize + 1);
    let (mut calls, mut positive, mut raw_rows, mut dependent) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut witnesses: Vec<[u64; 4]> = Vec::new();
    for _ in 0..MAX_QUERIES {
        if linear.rank() == bsize + 1 {
            break;
        }
        let a = masks.next(e.n);
        let beta = masks.next(e.n);
        if beta == 0 {
            continue;
        }
        let ag = ar.scalar(e, a, Some(e.g));
        let bq = ar.scalar(e, beta, Q);
        let r = ar.add(e, ag, bq);
        calls += 1;
        let mut hit = false;
        for term in &right {
            let negative_right = ar.neg(e, term.point);
            let needed = ar.add(e, r, negative_right);
            let Some(&code) = first.get(&point_key(needed)) else {
                continue;
            };
            // Verify the exact *elliptic curve* equality before admitting a
            // polynomial relation row. This cost is not omitted.
            let left_witness = signed_four(code, &base, &mut ar);
            let right_witness = signed_four(term.code, &base, &mut ar);
            let witness = ar.add(e, left_witness, right_witness);
            if witness != r {
                return Err("invalid 4+4 relation witness".into());
            }
            let mut row = vec![0_u64; bsize + 2];
            fill_row(&mut row, code, e.n);
            fill_row(&mut row, term.code, e.n);
            row[bsize] = e.n - beta; // Σ eps_i log(P_i) − beta log(Q) = a
            row[bsize + 1] = a;
            raw_rows += 1;
            if linear.insert(row, e.n)? {
                witnesses.push([a, beta, code, term.code]);
            } else {
                dependent += 1;
            }
            hit = true;
            if linear.rank() == bsize + 1 {
                break;
            }
        }
        if hit {
            positive += 1;
        }
    }
    let logs = linear.solve(e.n)?;
    // Independent end-to-end public-point checks. The known-answer scalar
    // is *verifier-only*, never used to choose Q or recover the answer.
    for (i, p) in base.points.iter().enumerate() {
        if ar.scalar(e, logs[i], Some(e.g)) != Some(*p) {
            return Err(format!("basis point log verification failed at {i}"));
        }
    }
    let d = logs[bsize];
    if ar.scalar(e, d, Some(e.g)) != Q || d != VERIFIER_SCALAR_ONLY {
        return Err("public target scalar verification failed".into());
    }
    // Fail closed on any changes to the registered deterministic control,
    // without converting this diagnostic into an algorithmic speedup claim.
    if calls != 124 || positive != 26 || raw_rows != 32 || dependent != 0
        || linear.rank() != 32 || witnesses.len() != 32
    {
        return Err("prime31 split-m8 reference control changed".into());
    }
    println!("{}", serde_json::to_string_pretty(&json!({
        "standing": "[pcc:unverified]",
        "curve": {"p":e.p,"a":e.a,"b":e.b,"n":e.n,"g":e.g},
        "public_target": Q, "known_answer_verifier_only":VERIFIER_SCALAR_ONLY,
        "oracle":"generic group 4+4 MITM; NOT F4 / subgeneric / claimed cheaper",
        "depth":base.depth, "terminal":base.terminal,
        "B":bsize, "left_templates":left.len(),
        "left_distinct_points":first.len(), "right_templates":right.len(),
        "queries":calls, "positive_queries":positive,
        "raw_rows":raw_rows, "dependent_rows":dependent,
        "independent_rank":linear.rank(), "recovered_scalar":d,
        "verified_basis_logs":bsize, "setup_group_additions":setup_additions,
        "all_counted_group_additions":ar.counts.group_additions,
        "arithmetic":ar.counts, "witnesses":witnesses,
        "scope":"finite-instance regression; exact-head Rust execution pending"
    })).map_err(|e| e.to_string())?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_four_encoding_and_affine_row() {
        let code = four_code([1, 4, 15, 30], 5);
        assert_eq!(decode_four(code), [(1, -1), (4, 1), (15, -1), (30, 1)]);
        let mut row = vec![0_u64; 33];
        fill_row(&mut row, code, 977);
        assert_eq!((row[1],row[4],row[15],row[30]), (976,1,976,1));
    }

    #[test]
    fn modular_echelon_recovers_its_target_without_hidden_log() {
        let mut m = Linear::new(3);
        assert!(m.insert(vec![1,0,0,2],977).unwrap());
        assert!(m.insert(vec![0,2,0,6],977).unwrap());
        assert!(m.insert(vec![0,0,3,15],977).unwrap());
        assert!(!m.insert(vec![2,0,0,4],977).unwrap());
        assert_eq!(m.rank(),3);
        assert_eq!(m.solve(977).unwrap(),vec![2,3,5]);
    }
}
