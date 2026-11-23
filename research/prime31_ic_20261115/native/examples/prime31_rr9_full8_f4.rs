//! Bounded public-input RR9 B8 full-eight-equation F4 screen.
//! Protocol: RR9_FULL8_F4_PROTOCOL_20261122.md. No signed leaves enter F4.
use prime31_algebraic_ic::{
    check_curve,
    f4_fp_tower::{f4_tower, free_of, RPoly, TowerF4Options, TowerRing, ONE},
    Arithmetic, PRIME31,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

const P: u64 = 2_147_483_647;
const XS: [u64; 8] = [0, 1, 2, 3, 5, 6, 8, 9];
const RESIDUALS: [(u64, u64); 3] = [
    (1_911_579_686, 548_820_096),
    (625_037_715, 2_100_627_251),
    (1_923_055_775, 307_731_458),
];
type Exp = [u8; 7];
type Poly = BTreeMap<Exp, u64>;

fn add(a: u64, b: u64) -> u64 { let z = a + b; if z >= P { z - P } else { z } }
fn sub(a: u64, b: u64) -> u64 { if a >= b { a - b } else { a + P - b } }
fn mul(a: u64, b: u64) -> u64 {
    let z = a * b;
    let t = (z & P) + (z >> 31);
    let u = (t & P) + (t >> 31);
    if u >= P { u - P } else { u }
}
fn power(mut a: u64, mut e: u64) -> u64 {
    let mut z = 1;
    while e != 0 { if e & 1 != 0 { z = mul(z, a); } a = mul(a, a); e >>= 1; }
    z
}
fn term(q: &mut Poly, c: u64, i: Option<usize>, j: Option<usize>) {
    if c == 0 { return; }
    let mut e = [0u8; 7];
    for k in [i, j].into_iter().flatten() { e[k] += 1; }
    let z = add(q.get(&e).copied().unwrap_or(0), c);
    if z == 0 { q.remove(&e); } else { q.insert(e, z); }
}
fn poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut z = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() { z[i+j] = add(z[i+j], mul(x, y)); }
    }
    z
}
fn h_base() -> Vec<u64> {
    XS.iter().fold(vec![1], |h, &x| poly_mul(&h, &[sub(0, x), 1]))
}
fn quadrics(r: u64, s: u64, h: &[u64]) -> [Poly; 8] {
    let mut q: [Poly; 8] = std::array::from_fn(|_| BTreeMap::new());
    let d = [add(mul(r, r), PRIME31.a), r, 1];
    let vvar = |i: usize| if i == 3 { None } else { Some(i + 4) };
    for i in 0..4 {
        for j in 0..4 {
            for (k, &dk) in d.iter().enumerate() {
                if i+j+k < 8 { term(&mut q[i+j+k], dk, vvar(i), vvar(j)); }
            }
            term(&mut q[i+j], r, Some(i), Some(j));
            if i+j+1 < 8 { term(&mut q[i+j+1], P-1, Some(i), Some(j)); }
            term(&mut q[i+j], sub(0, mul(2, s)), Some(i), vvar(j));
        }
    }
    for i in 0..8 { term(&mut q[i], sub(0, h[i]), None, None); }
    q
}
fn direct_m(c: &[u64; 7], r: u64, s: u64) -> Vec<u64> {
    let a = &c[..4];
    let v = [c[4], c[5], c[6], 1];
    let d = [add(mul(r, r), PRIME31.a), r, 1];
    let dvv = poly_mul(&d, &poly_mul(&v, &v));
    let xaa = poly_mul(&[sub(0, r), 1], &poly_mul(a, a));
    let av = poly_mul(a, &v);
    (0..9).map(|i| {
        let mut z = dvv[i];
        if i < xaa.len() { z = sub(z, xaa[i]); }
        if i < av.len() { z = sub(z, mul(mul(2, s), av[i])); }
        z
    }).collect()
}
#[cfg(test)]
fn eval(q: &Poly, c: &[u64; 7]) -> u64 {
    q.iter().fold(0, |sum, (e, &a)| {
        let mut z = a;
        for i in 0..7 { for _ in 0..e[i] { z = mul(z, c[i]); } }
        add(sum, z)
    })
}
fn linear_solution(basis: &[RPoly]) -> Option<[u64; 7]> {
    let mut rows = Vec::<[u64; 8]>::new();
    for g in basis {
        if g.degree() > 1 { continue; }
        let mut row = [0u64; 8];
        for (&mono, &coef) in g.monos.iter().zip(&g.coefs) {
            if mono == ONE { row[7] = add(row[7], u64::from(coef)); continue; }
            let e = free_of(mono);
            if e.iter().map(|&x| usize::from(x)).sum::<usize>() != 1 { return None; }
            let i = e.iter().position(|&x| x == 1)?;
            if i >= 7 { return None; }
            row[i] = add(row[i], u64::from(coef));
        }
        rows.push(row);
    }
    let mut pivot = 0;
    for col in 0..7 {
        let at = (pivot..rows.len()).find(|&i| rows[i][col] != 0)?;
        rows.swap(pivot, at);
        let inv = power(rows[pivot][col], P - 2);
        for k in col..8 { rows[pivot][k] = mul(rows[pivot][k], inv); }
        for i in 0..rows.len() {
            if i == pivot { continue; }
            let f = rows[i][col];
            if f != 0 { for k in col..8 { rows[i][k] = sub(rows[i][k], mul(f, rows[pivot][k])); } }
        }
        pivot += 1;
    }
    if rows.iter().skip(7).any(|r| r[..7].iter().all(|&x| x == 0) && r[7] != 0) { return None; }
    Some(std::array::from_fn(|i| sub(0, rows[i][7])))
}
fn full_point_check(c: &[u64; 7], residual: (u64, u64)) -> bool {
    let (r, s) = residual;
    let mut ar = Arithmetic::default();
    let mut total = None;
    for &x in &XS {
        let z = add(add(mul(mul(x, x), x), mul(PRIME31.a, x)), PRIME31.b);
        let Some(y) = ar.sqrt(z, P) else { return false; };
        let ax = c[..4].iter().rev().fold(0, |v, &a| add(mul(v, x), a));
        let vx = [c[4], c[5], c[6], 1].iter().rev().fold(0, |v, &a| add(mul(v, x), a));
        let f = |yy| add(mul(sub(x, r), ax), mul(add(yy, s), vx));
        let selected = match (f(y) == 0, f(sub(0, y)) == 0) {
            (true, false) => Some((x, y)),
            (false, true) => Some((x, sub(0, y))),
            _ => return false,
        };
        total = ar.add(PRIME31, total, selected);
    }
    total == Some(residual)
}
fn run(case: usize) -> Value {
    let started = Instant::now();
    let residual = RESIDUALS[case];
    check_curve(PRIME31, &mut Arithmetic::default()).unwrap();
    let h = h_base();
    let q = quadrics(residual.0, residual.1, &h);
    let ring = TowerRing { p: P, rules: vec![], n_free: 7 };
    let inputs: Vec<_> = q.iter().map(|g| {
        let raw: Vec<_> = g.iter().map(|(e, &c)| (e.iter().map(|&v| u32::from(v)).collect(), c)).collect();
        ring.from_raw(&raw)
    }).collect();
    let opts = TowerF4Options::new(9).with_budget(Duration::from_secs(60))
        .with_max_nnz(3_000_000).with_max_dense(1_000_000);
    let f4_start = Instant::now();
    let rep = f4_tower(&inputs, &ring, &opts);
    let f4_ns = f4_start.elapsed().as_nanos();
    let complete = !rep.timed_out && !rep.oversize && rep.pairs_above_bound == 0
        && rep.staircase_at_stop.is_none();
    let candidate = if complete && !rep.inconsistent { linear_solution(&rep.basis) } else { None };
    let direct_identity = candidate.as_ref().map(|c| direct_m(c, residual.0, residual.1) == h);
    let basis_check = candidate.as_ref().map(|c| rep.basis.iter().all(|g| ring.eval(g, c) == 0));
    let point_check = candidate.as_ref().map(|c| full_point_check(c, residual));
    json!({
        "schema": "prime31.rr9.full8-f4-screen.v1", "case": case,
        "public_residual": [residual.0, residual.1], "base_x": XS,
        "p": P, "n": PRIME31.n, "input_quadrics": 8,
        "no_sign_mask_or_secret_scalar_passed": true,
        "f4_wall_ns": f4_ns.to_string(), "full_wall_ns": started.elapsed().as_nanos().to_string(),
        "complete_basis": complete, "inconsistent": rep.inconsistent,
        "timed_out": rep.timed_out, "oversize": rep.oversize,
        "pairs_above_bound": rep.pairs_above_bound,
        "staircase_at_stop": rep.staircase_at_stop,
        "basis_size": rep.basis.len(), "degree_reached": rep.degree_reached,
        "solving_degree_max": rep.solving_degree_max,
        "max_rows": rep.max_rows, "max_cols": rep.max_cols,
        "max_nnz": rep.max_nnz, "max_dense_entries": rep.max_dense_entries,
        "muladds": rep.muladds, "steps": rep.steps,
        "candidate_coefficients": candidate,
        "original_eight_coefficients_checked": direct_identity,
        "candidate_vanishes_on_f4_basis": basis_check,
        "full_signed_point_sum_checked": point_check,
        "negative_unit_identity_replayed": false,
        "pcc": "unverified"
    })
}
fn main() {
    let case: usize = std::env::args().nth(1).expect("case 0, 1 or 2").parse().unwrap();
    assert!(case < 3);
    println!("{}", serde_json::to_string_pretty(&run(case)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_quadrics_match_direct_norm() {
        let h = h_base();
        for &(r, s) in &RESIDUALS {
            let q = quadrics(r, s, &h);
            for seed in 0..8u64 {
                let c: [u64; 7] = std::array::from_fn(|j| (seed * 71 + j as u64 * 29 + 3) % P);
                let m = direct_m(&c, r, s);
                assert_eq!(m[8], 1);
                for i in 0..8 { assert_eq!(eval(&q[i], &c), sub(m[i], h[i])); }
            }
        }
    }
}
