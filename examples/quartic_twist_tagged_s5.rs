//! Registered mixed-line Semaev peeling pilot on the frozen p=37 twist trace.
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{
    solve_system4, solve_system_n, Curve4, Fp4, Pt4, QuarticSolve, SymmetrisedS5, E4,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};

pub(crate) type Terms = [(usize, i8); 4];

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn e(v: &Value) -> E4 {
    let a = v.as_array().expect("F4 element");
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn mm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn add(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}

fn s3_coeff(c: &Curve4, x: &E4, y: &E4) -> [E4; 3] {
    let f = &c.f;
    let sum = f.add(x, y);
    let prod = f.mul(x, y);
    let two_b = f.add(&c.b, &c.b);
    let t = f.add(&f.mul(&sum, &f.add(&prod, &c.a)), &two_b);
    let twice_twice_b = f.add(&two_b, &two_b);
    let u = f.sub(&prod, &c.a);
    [
        f.sub(&f.sq(&u), &f.mul(&twice_twice_b, &sum)),
        f.neg(&f.add(&t, &t)),
        f.sq(&f.sub(x, y)),
    ]
}

// The resultant of two degree-two S3 polynomials in their common variable.
fn s4_direct(c: &Curve4, x: &[E4; 4]) -> E4 {
    let f = &c.f;
    let a = s3_coeff(c, &x[0], &x[1]);
    let b = s3_coeff(c, &x[2], &x[3]);
    let t1 = f.sub(&f.mul(&a[2], &b[0]), &f.mul(&a[0], &b[2]));
    let t2 = f.sub(&f.mul(&a[2], &b[1]), &f.mul(&a[1], &b[2]));
    let t3 = f.sub(&f.mul(&a[1], &b[0]), &f.mul(&a[0], &b[1]));
    f.sub(&f.sq(&t1), &f.mul(&t2, &t3))
}

fn exponents3(degree: u8) -> Vec<[u8; 3]> {
    let mut out = Vec::new();
    for a in 0..=degree {
        for b in 0..=degree - a {
            for c in 0..=degree - a - b {
                out.push([a, b, c]);
            }
        }
    }
    out
}
fn e3(f: &Fp4, x: &[E4; 3]) -> [E4; 3] {
    [
        f.add(&f.add(&x[0], &x[1]), &x[2]),
        f.add(
            &f.add(&f.mul(&x[0], &x[1]), &f.mul(&x[0], &x[2])),
            &f.mul(&x[1], &x[2]),
        ),
        f.mul(&f.mul(&x[0], &x[1]), &x[2]),
    ]
}
fn mono3(f: &Fp4, x: &[E4; 3], exps: &[[u8; 3]]) -> Vec<E4> {
    let mut pw = [[E4::ONE; 5]; 3];
    for i in 0..3 {
        for j in 1..5 {
            pw[i][j] = f.mul(&pw[i][j - 1], &x[i]);
        }
    }
    exps.iter()
        .map(|v| {
            f.mul(
                &f.mul(&pw[0][v[0] as usize], &pw[1][v[1] as usize]),
                &pw[2][v[2] as usize],
            )
        })
        .collect()
}
fn vinv5(p: u64) -> [[u64; 5]; 5] {
    let mut rows = [[0u64; 10]; 5];
    for (i, row) in rows.iter_mut().enumerate() {
        let mut power = 1;
        for cell in row.iter_mut().take(5) {
            *cell = power;
            power = mm(power, i as u64, p);
        }
        row[5 + i] = 1;
    }
    for col in 0..5 {
        let pivot = (col..5).find(|&row| rows[row][col] != 0).unwrap();
        rows.swap(pivot, col);
        let inv = crypto_lib::cryptanalysis::residual_walk::inv_mod(rows[col][col], p);
        for j in col..10 {
            rows[col][j] = mm(rows[col][j], inv, p);
        }
        for row in 0..5 {
            if row == col {
                continue;
            }
            let factor = rows[row][col];
            for j in col..10 {
                rows[row][j] = sub(rows[row][j], mm(factor, rows[col][j], p), p);
            }
        }
    }
    std::array::from_fn(|i| std::array::from_fn(|j| rows[i][5 + j]))
}

pub(crate) struct SymS4 {
    monos: Vec<[u8; 3]>,
    coef: Vec<[E4; 5]>,
    pub(crate) precompute_muls: u64,
}
impl SymS4 {
    fn coeffs_last(c: &Curve4, x: &[E4; 3], vinv: &[[u64; 5]; 5]) -> [E4; 5] {
        let f = &c.f;
        let vals: [E4; 5] =
            std::array::from_fn(|i| s4_direct(c, &[x[0], x[1], x[2], f.from_fp(i as u64)]));
        std::array::from_fn(|j| {
            (0..5).fold(E4::ZERO, |acc, k| {
                f.add(&acc, &f.scale(&vals[k], vinv[j][k]))
            })
        })
    }
    pub(crate) fn precompute(c: &Curve4, rng: &mut StdRng) -> Self {
        let f = &c.f;
        let start = f.muls();
        let mons = exponents3(4);
        let n = mons.len();
        let vinv = vinv5(f.p);
        for _attempt in 0..100 {
            let mut sys = Vec::with_capacity(n);
            for _ in 0..n {
                let x = std::array::from_fn(|_| f.random(rng));
                let mut row = mono3(f, &e3(f, &x), &mons);
                row.extend(Self::coeffs_last(c, &x, &vinv));
                sys.push(row);
            }
            let mut singular = false;
            for col in 0..n {
                let Some(pivot) = (col..n).find(|&row| !sys[row][col].is_zero()) else {
                    singular = true;
                    break;
                };
                sys.swap(pivot, col);
                let inv = f.inv(&sys[col][col]);
                for j in col..n + 5 {
                    sys[col][j] = f.mul(&sys[col][j], &inv);
                }
                for row in 0..n {
                    if row == col || sys[row][col].is_zero() {
                        continue;
                    }
                    let factor = sys[row][col];
                    for j in col..n + 5 {
                        sys[row][j] = f.sub(&sys[row][j], &f.mul(&factor, &sys[col][j]));
                    }
                }
            }
            if singular {
                continue;
            }
            let coef = (0..n)
                .map(|row| std::array::from_fn(|j| sys[row][n + j]))
                .collect();
            let mut pre = Self {
                monos: mons.clone(),
                coef,
                precompute_muls: f.muls() - start,
            };
            for _ in 0..16 {
                let x = std::array::from_fn(|_| f.random(rng));
                let xr = f.random(rng);
                let mv = mono3(f, &e3(f, &x), &pre.monos);
                let mut xp = E4::ONE;
                let mut got = E4::ZERO;
                for j in 0..5 {
                    let cj = mv
                        .iter()
                        .zip(&pre.coef)
                        .fold(E4::ZERO, |sum, (m, c)| f.add(&sum, &f.mul(m, &c[j])));
                    got = f.add(&got, &f.mul(&cj, &xp));
                    xp = f.mul(&xp, &xr);
                }
                assert_eq!(got, s4_direct(c, &[x[0], x[1], x[2], xr]));
            }
            pre.precompute_muls = f.muls() - start;
            return pre;
        }
        panic!("S4 interpolation did not find a full-rank sample matrix");
    }
    fn weil_line(&self, f: &Fp4, xr: &E4, scale: &E4) -> [HashMap<[u8; 3], u64>; 4] {
        let mut xp = [E4::ONE; 5];
        for j in 1..5 {
            xp[j] = f.mul(&xp[j - 1], xr);
        }
        let mut sp = [E4::ONE; 13];
        for j in 1..13 {
            sp[j] = f.mul(&sp[j - 1], scale);
        }
        let mut out: [HashMap<[u8; 3], u64>; 4] = std::array::from_fn(|_| HashMap::new());
        for (&m, c) in self.monos.iter().zip(&self.coef) {
            let mut v = E4::ZERO;
            for j in 0..5 {
                v = f.add(&v, &f.mul(&c[j], &xp[j]));
            }
            let weight = m[0] as usize + 2 * m[1] as usize + 3 * m[2] as usize;
            v = f.mul(&v, &sp[weight]);
            for (i, o) in out.iter_mut().enumerate() {
                if v.0[i] != 0 {
                    o.insert(m, v.0[i]);
                }
            }
        }
        out
    }
}

fn s5_weil_line(pre: &SymmetrisedS5, f: &Fp4, xr: &E4, scale: &E4) -> [HashMap<[u8; 4], u64>; 4] {
    let mut xp = [E4::ONE; 9];
    for j in 1..9 {
        xp[j] = f.mul(&xp[j - 1], xr);
    }
    let mut sp = [E4::ONE; 33];
    for j in 1..33 {
        sp[j] = f.mul(&sp[j - 1], scale);
    }
    let mut out: [HashMap<[u8; 4], u64>; 4] = std::array::from_fn(|_| HashMap::new());
    for (&m, c) in pre.monos.iter().zip(&pre.coef) {
        let mut v = E4::ZERO;
        for j in 0..9 {
            v = f.add(&v, &f.mul(&c[j], &xp[j]));
        }
        let weight = m[0] as usize + 2 * m[1] as usize + 3 * m[2] as usize + 4 * m[3] as usize;
        v = f.mul(&v, &sp[weight]);
        for (i, o) in out.iter_mut().enumerate() {
            if v.0[i] != 0 {
                o.insert(m, v.0[i]);
            }
        }
    }
    out
}

fn s3_weil(c: &Curve4, xr: &E4) -> [HashMap<[u8; 2], u64>; 4] {
    let f = &c.f;
    let xr2 = f.sq(xr);
    let four_b = f.scale(&c.b, 4);
    let terms = [
        ([2, 0], xr2),
        ([0, 1], f.neg(&f.scale(&xr2, 4))),
        ([1, 1], f.neg(&f.scale(xr, 2))),
        ([0, 0], f.neg(&f.mul(&four_b, xr))),
        ([0, 2], E4::ONE),
        ([1, 0], f.neg(&four_b)),
    ];
    let mut out: [HashMap<[u8; 2], u64>; 4] = std::array::from_fn(|_| HashMap::new());
    for (m, value) in terms {
        for (j, comp) in out.iter_mut().enumerate() {
            if value.0[j] != 0 {
                let old = comp.get(&m).copied().unwrap_or(0);
                comp.insert(m, add(old, value.0[j], f.p));
            }
        }
    }
    for comp in &mut out {
        comp.retain(|_, v| *v != 0);
    }
    out
}

// A complete finite-field root search for the two symmetric unknowns. This
// avoids asking the generic Macaulay solver to recover variables that its
// overdetermined four-equation echelon has already reduced to constants.
fn solve_s3_fp_scan(comps: &[HashMap<[u8; 2], u64>; 4], p: u64) -> (Vec<[u64; 2]>, u64) {
    let mut out = Vec::new();
    let mut muls = 0;
    for e1 in 0..p {
        for e2 in 0..p {
            let squares = [mm(e1, e1, p), mm(e1, e2, p), mm(e2, e2, p)];
            muls += 3;
            let value = |exp: &[u8; 2]| match *exp {
                [0, 0] => 1,
                [1, 0] => e1,
                [0, 1] => e2,
                [2, 0] => squares[0],
                [1, 1] => squares[1],
                [0, 2] => squares[2],
                _ => panic!("unexpected S3 exponent"),
            };
            let mut valid = true;
            for comp in comps {
                let mut sum = 0;
                for (exp, &coefficient) in comp {
                    sum = add(sum, mm(coefficient, value(exp), p), p);
                    muls += 1;
                }
                valid &= sum == 0;
            }
            if valid {
                out.push([e1, e2]);
            }
        }
    }
    (out, muls)
}

fn split<const N: usize>(sym: &[u64; N], p: u64) -> Option<[u64; N]> {
    let mut roots = Vec::new();
    for x in 0..p {
        let mut acc = 1;
        for (j, &ej) in sym.iter().enumerate() {
            acc = mm(acc, x, p);
            acc = if j % 2 == 0 {
                sub(acc, ej, p)
            } else {
                add(acc, ej, p)
            };
        }
        if acc == 0 {
            roots.push(x);
        }
    }
    if roots.len() == N {
        Some(roots.try_into().unwrap())
    } else {
        None
    }
}

fn signed_relations(c: &Curve4, base: &[Pt4], target: &Pt4, ids: &[usize; 4]) -> BTreeSet<Terms> {
    let mut out = BTreeSet::new();
    if ids.iter().copied().collect::<BTreeSet<_>>().len() != 4 {
        return out;
    }
    for mask in 0..16 {
        let mut sum = Pt4::INF;
        let mut terms = [(0usize, 0i8); 4];
        for (j, &id) in ids.iter().enumerate() {
            let sign = if mask & (1 << j) == 0 { 1 } else { -1 };
            terms[j] = (id, sign);
            sum = c.add(
                &sum,
                &if sign == 1 {
                    base[id]
                } else {
                    c.neg(&base[id])
                },
            );
        }
        if sum == *target {
            terms.sort_unstable();
            out.insert(terms);
        }
    }
    out
}

fn point_oracle(c: &Curve4, base: &[Pt4], target: &Pt4, k: usize) -> BTreeSet<Terms> {
    let mut pairs: HashMap<Pt4, Vec<[(usize, i8); 2]>> = HashMap::new();
    let mut signed_pairs = Vec::new();
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let a = if si == 1 { base[i] } else { c.neg(&base[i]) };
                    let b = if sj == 1 { base[j] } else { c.neg(&base[j]) };
                    let sum = c.add(&a, &b);
                    let terms = [(i, si), (j, sj)];
                    pairs.entry(sum).or_default().push(terms);
                    signed_pairs.push((sum, terms));
                }
            }
        }
    }
    let mut out = BTreeSet::new();
    for (sum, left) in signed_pairs {
        let remain = c.sub(target, &sum);
        if let Some(rights) = pairs.get(&remain) {
            for right in rights {
                let mut terms = [left[0], left[1], right[0], right[1]];
                terms.sort_unstable();
                if terms.windows(2).any(|w| w[0].0 == w[1].0) {
                    continue;
                }
                if terms.iter().filter(|&&(id, _)| id >= 36).count() == k {
                    out.insert(terms);
                }
            }
        }
    }
    out
}

fn candidate_ids(
    f: &Fp4,
    by_x: &HashMap<E4, usize>,
    line0: &[u64],
    line2: &[u64],
) -> Option<[usize; 4]> {
    let mut ids = Vec::with_capacity(4);
    for &x in line0 {
        ids.push(*by_x.get(&f.from_fp(x))?);
    }
    let s = E4([0, 0, 1, 0]);
    for &x in line2 {
        ids.push(*by_x.get(&f.mul(&s, &f.from_fp(x)))?);
    }
    ids.try_into().ok()
}

#[allow(dead_code)] // Imported by the one-target producer; unused in this standalone pilot.
pub(crate) struct MixedCollect {
    pub relations: BTreeSet<Terms>,
    pub systems: u64,
    pub system_failures: u64,
    pub field_solutions: u64,
    pub split_failures: u64,
    pub foreign_or_repeat: u64,
    pub external_fp_muls: u64,
    pub internal_fp_muls: u64,
    pub group_ops: u64,
    pub wall_ms_l0: f64,
}

/// Complete polynomial relation collection for one mixed component pattern.
/// Only k=1,2,3 are accepted; neither same-line S5 arm is called.
#[allow(dead_code)] // Imported by the one-target producer; unused in this standalone pilot.
pub(crate) fn collect_mixed_pattern(
    c: &Curve4,
    base: &[Pt4],
    by_x: &HashMap<E4, usize>,
    residual: &Pt4,
    k: usize,
    s4: &SymS4,
    rng: &mut StdRng,
) -> MixedCollect {
    assert!((1..=3).contains(&k));
    let line0_len = base.iter().take_while(|pt| pt.x.0[2] == 0).count();
    assert!(line0_len > 0 && line0_len < base.len());
    assert!(base[..line0_len].iter().all(|pt| pt.x.0[1..] == [0, 0, 0]));
    assert!(base[line0_len..]
        .iter()
        .all(|pt| pt.x.0[0] == 0 && pt.x.0[1] == 0 && pt.x.0[3] == 0));
    let p = c.f.p;
    c.f.reset_muls();
    c.reset_ops();
    let start = Instant::now();
    let mut got = BTreeSet::new();
    let mut systems = 0;
    let mut failures = 0;
    let mut field_solutions = 0;
    let mut split_failures = 0;
    let mut foreign_or_repeat = 0;
    let mut internal_muls = 0;
    let mut process = |line0: Vec<u64>, line2: Vec<u64>| {
        field_solutions += 1;
        let Some(ids) = candidate_ids(&c.f, by_x, &line0, &line2) else {
            foreign_or_repeat += 1;
            return;
        };
        if ids.iter().copied().collect::<BTreeSet<_>>().len() != 4 {
            foreign_or_repeat += 1;
            return;
        }
        got.extend(signed_relations(c, base, residual, &ids));
    };
    if k == 1 || k == 3 {
        let peel = if k == 1 {
            line0_len..base.len()
        } else {
            0..line0_len
        };
        let scale = if k == 1 { E4::ONE } else { E4([0, 0, 1, 0]) };
        for id in peel {
            for sign in [-1, 1] {
                let pt = if sign == 1 {
                    base[id]
                } else {
                    c.neg(&base[id])
                };
                let remain = c.sub(residual, &pt);
                if remain.inf {
                    continue;
                }
                let mut st = QuarticSolve::default();
                let before = c.f.muls();
                let comps = s4.weil_line(&c.f, &remain.x, &scale);
                st.weil_muls = c.f.muls() - before;
                systems += 1;
                if let Some(solutions) = solve_system_n::<3>(&comps, 4, 10, p, rng, &mut st) {
                    for sol in solutions {
                        if let Some(xs) = split(&sol, p) {
                            let peeled_x = if k == 1 {
                                base[id].x.0[2]
                            } else {
                                base[id].x.0[0]
                            };
                            if k == 1 {
                                process(xs.to_vec(), vec![peeled_x]);
                            } else {
                                process(vec![peeled_x], xs.to_vec());
                            }
                        } else {
                            split_failures += 1;
                        }
                    }
                } else {
                    failures += 1;
                }
                st.close_public();
                internal_muls += st.total_muls - st.weil_muls;
            }
        }
    } else {
        for i in line0_len..base.len() {
            for j in i + 1..base.len() {
                for si in [-1, 1] {
                    for sj in [-1, 1] {
                        let pi = if si == 1 { base[i] } else { c.neg(&base[i]) };
                        let pj = if sj == 1 { base[j] } else { c.neg(&base[j]) };
                        let remain = c.sub(&c.sub(residual, &pi), &pj);
                        if remain.inf {
                            continue;
                        }
                        let comps = s3_weil(c, &remain.x);
                        systems += 1;
                        let (solutions, muls) = solve_s3_fp_scan(&comps, p);
                        internal_muls += muls;
                        for sol in solutions {
                            if let Some(xs) = split(&sol, p) {
                                process(xs.to_vec(), vec![base[i].x.0[2], base[j].x.0[2]]);
                            } else {
                                split_failures += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    MixedCollect {
        relations: got,
        systems,
        system_failures: failures,
        field_solutions,
        split_failures,
        foreign_or_repeat,
        external_fp_muls: c.f.muls(),
        internal_fp_muls: internal_muls,
        group_ops: c.ops(),
        wall_ms_l0: start.elapsed().as_secs_f64() * 1e3,
    }
}

fn run(k: usize, raw: &str, query_filter: Option<usize>, s5_degree: u8) -> Value {
    assert!(k <= 4);
    assert!((29..=31).contains(&s5_degree));
    if query_filter.is_some() || s5_degree != 29 {
        assert!((k == 0 || k == 4) && query_filter == Some(2));
    }
    let records: Vec<Value> = fs::read_to_string(raw)
        .expect("frozen mixed trace")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let rec = records
        .iter()
        .find(|r| u(&r["p"]) == 37 && u(&r["seed"]) == 0 && r["line_powers"] == json!([0, 2]))
        .expect("p37 mixed-line seed-zero record");
    let c = Curve4::with_params(37, E4::ZERO, E4([0, 0, 0, u(&rec["b0"])]));
    let g = point(&json!([rec["generator_x"], rec["generator_y"]]));
    let q = point(&json!([rec["target_x"], rec["target_y"]]));
    assert_eq!(c.mul(&g, 271), q);
    let base: Vec<Pt4> = rec["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert_eq!(base.len(), 48);
    assert!(base[..36].iter().all(|pt| pt.x.0[1..] == [0, 0, 0]));
    assert!(base[36..]
        .iter()
        .all(|pt| pt.x.0[0] == 0 && pt.x.0[1] == 0 && pt.x.0[3] == 0));
    let by_x: HashMap<E4, usize> = base.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    let mut rng = StdRng::seed_from_u64(0x5441_4747_4544_5335 ^ k as u64);
    c.f.reset_muls();
    c.reset_ops();
    let started = Instant::now();
    let s4 = if k == 1 || k == 3 {
        Some(SymS4::precompute(&c, &mut rng))
    } else {
        None
    };
    let s5 = if k == 0 || k == 4 {
        Some(SymmetrisedS5::precompute(&c, &mut rng))
    } else {
        None
    };
    let precompute_muls = c.f.muls();
    let precompute_ms_l0 = started.elapsed().as_secs_f64() * 1e3;
    let trace = rec["query_trace"].as_array().unwrap();
    let mut rows = Vec::new();
    for (query_index, z) in trace.iter().take(4).enumerate() {
        if query_filter.is_some_and(|wanted| wanted != query_index) {
            continue;
        }
        c.f.reset_muls();
        c.reset_ops();
        let (a, b) = (u(&z[0]), u(&z[1]));
        let residual = c.add(&c.mul(&g, a), &c.mul(&q, b));
        let reference = point_oracle(&c, &base, &residual, k);
        let reference_group_ops_excluded = c.ops();
        let reference_fp_muls_excluded = c.f.muls();
        c.f.reset_muls();
        c.reset_ops();
        let query_started = Instant::now();
        let mut got = BTreeSet::new();
        let mut systems = 0u64;
        let mut system_failures = 0u64;
        let mut field_solutions = 0u64;
        let mut split_failures = 0u64;
        let mut foreign_or_repeat = 0u64;
        let mut system_stats = Vec::new();
        let mut process = |line0: Vec<u64>, line2: Vec<u64>| {
            field_solutions += 1;
            let Some(ids) = candidate_ids(&c.f, &by_x, &line0, &line2) else {
                foreign_or_repeat += 1;
                return;
            };
            if ids.iter().copied().collect::<BTreeSet<_>>().len() != 4 {
                foreign_or_repeat += 1;
                return;
            }
            got.extend(signed_relations(&c, &base, &residual, &ids));
        };
        if k == 0 || k == 4 {
            let scale = if k == 0 { E4::ONE } else { E4([0, 0, 1, 0]) };
            let pre = s5.as_ref().unwrap();
            let mut st = QuarticSolve::default();
            let before = c.f.muls();
            let comps = s5_weil_line(pre, &c.f, &residual.x, &scale);
            st.weil_muls = c.f.muls() - before;
            systems += 1;
            if let Some(solutions) =
                solve_system4(&comps, 8, s5_degree, 37, &mut rng, &mut st, false)
            {
                for sol in solutions {
                    if let Some(xs) = split(&sol, 37) {
                        if k == 0 {
                            process(xs.to_vec(), Vec::new())
                        } else {
                            process(Vec::new(), xs.to_vec())
                        }
                    } else {
                        split_failures += 1;
                    }
                }
            } else {
                system_failures += 1;
            }
            st.close_public();
            system_stats.push(st);
        } else if k == 1 || k == 3 {
            let peel = if k == 1 { 36..48 } else { 0..36 };
            let scale = if k == 1 { E4::ONE } else { E4([0, 0, 1, 0]) };
            let pre = s4.as_ref().unwrap();
            for id in peel {
                for sign in [-1, 1] {
                    let pt = if sign == 1 {
                        base[id]
                    } else {
                        c.neg(&base[id])
                    };
                    let remain = c.sub(&residual, &pt);
                    if remain.inf {
                        continue;
                    }
                    let mut st = QuarticSolve::default();
                    let before = c.f.muls();
                    let comps = pre.weil_line(&c.f, &remain.x, &scale);
                    st.weil_muls = c.f.muls() - before;
                    systems += 1;
                    if let Some(solutions) =
                        solve_system_n::<3>(&comps, 4, 10, 37, &mut rng, &mut st)
                    {
                        for sol in solutions {
                            if let Some(xs) = split(&sol, 37) {
                                let peeled_x = if k == 1 {
                                    base[id].x.0[2]
                                } else {
                                    base[id].x.0[0]
                                };
                                if k == 1 {
                                    process(xs.to_vec(), vec![peeled_x])
                                } else {
                                    process(vec![peeled_x], xs.to_vec())
                                }
                            } else {
                                split_failures += 1;
                            }
                        }
                    } else {
                        system_failures += 1;
                    }
                    st.close_public();
                    system_stats.push(st);
                }
            }
        } else {
            for i in 36..48 {
                for j in i + 1..48 {
                    for si in [-1, 1] {
                        for sj in [-1, 1] {
                            let pi = if si == 1 { base[i] } else { c.neg(&base[i]) };
                            let pj = if sj == 1 { base[j] } else { c.neg(&base[j]) };
                            let remain = c.sub(&c.sub(&residual, &pi), &pj);
                            if remain.inf {
                                continue;
                            }
                            let mut st = QuarticSolve::default();
                            let before = c.f.muls();
                            let comps = s3_weil(&c, &remain.x);
                            st.weil_muls = c.f.muls() - before;
                            systems += 1;
                            let (solutions, scan_muls) = solve_s3_fp_scan(&comps, 37);
                            st.equation_degree = 2;
                            st.rows = 37 * 37;
                            st.cols = 6;
                            st.verify_muls = scan_muls;
                            st.outcome = "exhaustive-Fp2-root-scan".into();
                            for sol in solutions {
                                if let Some(xs) = split(&sol, 37) {
                                    process(xs.to_vec(), vec![base[i].x.0[2], base[j].x.0[2]]);
                                } else {
                                    split_failures += 1;
                                }
                            }
                            st.close_public();
                            system_stats.push(st);
                        }
                    }
                }
            }
        }
        let external_fp_muls = c.f.muls();
        let group_ops = c.ops();
        let internal_fp_muls: u64 = system_stats
            .iter()
            .map(|s| s.total_muls - s.weil_muls)
            .sum();
        let agree = got == reference;
        eprintln!(
            "k={k} residual {query_index}: {} systems, {} / {} signed relations, agree={agree}",
            systems,
            got.len(),
            reference.len()
        );
        rows.push(
            json!({"query_index":query_index,"a":a,"b":b,"recorded_total_hits":u(&z[2]),
            "reference_signed_relations":reference,"polynomial_signed_relations":got,
            "reference_fp_muls_excluded":reference_fp_muls_excluded,
            "reference_group_ops_excluded":reference_group_ops_excluded,
            "systems":systems,"system_failures":system_failures,"field_solutions":field_solutions,
            "splitting_failures":split_failures,"foreign_or_repeat":foreign_or_repeat,
            "external_fp_muls":external_fp_muls,"internal_fp_muls":internal_fp_muls,
            "group_ops":group_ops,"system_stats":system_stats,"agree":agree,
            "wall_ms_l0":query_started.elapsed().as_secs_f64()*1e3}),
        );
    }
    json!({"status":if rows.iter().all(|r|r["agree"]==true && r["system_failures"]==0){"pass"}else{"partial"},
        "p":37,"b0":u(&rec["b0"]),"n":u(&rec["n"]),"seed":0,"target_scalar":271,
        "query_index_filter":query_filter,"s5_macaulay_degree":s5_degree,
        "line_powers":[0,2],"base_size":base.len(),"line2_size":12,"k":k,
        "precompute_muls":precompute_muls,"precompute_ms_l0":precompute_ms_l0,
        "s4_interpolation_muls":s4.as_ref().map(|v|v.precompute_muls),
        "s5_interpolation_muls":s5.as_ref().map(|v|v.precompute_muls),
        "rows":rows})
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(
        (4..=6).contains(&args.len()),
        "usage: quartic_twist_tagged_s5 --k K MIXED_NDJSON [--query-index=2] [--s5-degree=30|31]"
    );
    assert_eq!(args[1], "--k");
    let k: usize = args[2].parse().expect("k");
    let mut query_filter = None;
    let mut s5_degree = 29;
    for option in &args[4..] {
        if let Some(value) = option.strip_prefix("--query-index=") {
            query_filter = Some(value.parse().unwrap());
        } else if let Some(value) = option.strip_prefix("--s5-degree=") {
            s5_degree = value.parse().unwrap();
        } else {
            panic!("unrecognized option: {option}");
        }
    }
    println!("{}", run(k, &args[3], query_filter, s5_degree));
}
