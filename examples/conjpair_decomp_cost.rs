//! Decomposition cost on a conjugate-pair line against F_p, with the
//! repository's counted k = 4 Gaudry solver (`gaudry_quartic`).
//! Protocol: research/lift_dense_conjugate_pair_decomp_20261106/PROTOCOL.md
//! (frozen at 5387a0bd9c).
//!
//!   cargo run --release --example conjpair_decomp_cost -- \
//!       --p 29 --curves 2 --out <rows.jsonl> [--aa]
//!
//! The solver is called through its public API only; this example builds the
//! curve with full 2-torsion, the factor-base lines, the symmetrised S₅ of
//! each line (by the same interpolation `SymmetrisedS5::precompute` uses, at
//! x = x₀ + u·r), runs the solve, and checks every solve against an exact
//! meet-in-the-middle decomposition over the line's factor base.

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{
    solve_system4_with, Curve4, Fp4, PlainOptions, Pt4, QuarticSolve, SymmetrisedS5, E4,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

fn pow_mod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}
fn inv_mod(a: u64, p: u64) -> u64 {
    pow_mod(a, p - 2, p)
}

fn exponents4(d: u8) -> Vec<[u8; 4]> {
    let mut v = Vec::new();
    for a in 0..=d {
        for b in 0..=(d - a) {
            for c in 0..=(d - a - b) {
                for e in 0..=(d - a - b - c) {
                    v.push([a, b, c, e]);
                }
            }
        }
    }
    v
}

fn elementary(f: &Fp4, x: &[E4; 4]) -> [E4; 4] {
    let e1 = f.add(&f.add(&x[0], &x[1]), &f.add(&x[2], &x[3]));
    let mut e2 = E4::ZERO;
    let mut e3 = E4::ZERO;
    for i in 0..4 {
        for j in (i + 1)..4 {
            let xij = f.mul(&x[i], &x[j]);
            e2 = f.add(&e2, &xij);
            for k in (j + 1)..4 {
                e3 = f.add(&e3, &f.mul(&xij, &x[k]));
            }
        }
    }
    let e4 = f.mul(&f.mul(&x[0], &x[1]), &f.mul(&x[2], &x[3]));
    [e1, e2, e3, e4]
}

fn mono_values(f: &Fp4, e: &[E4; 4], monos: &[[u8; 4]]) -> Vec<E4> {
    let mut pows = [[E4::ONE; 9]; 4];
    for i in 0..4 {
        for k in 1..9 {
            pows[i][k] = f.mul(&pows[i][k - 1], &e[i]);
        }
    }
    monos
        .iter()
        .map(|m| {
            f.mul(
                &f.mul(&pows[0][m[0] as usize], &pows[1][m[1] as usize]),
                &f.mul(&pows[2][m[2] as usize], &pows[3][m[3] as usize]),
            )
        })
        .collect()
}

/// Inverse Vandermonde at nodes 0..8 over F_p: coefficient j = Σ_k V⁻¹[j][k] value_k.
fn vandermonde_inv(p: u64) -> [[u64; 9]; 9] {
    let mut m: Vec<Vec<u64>> = (0..9u64)
        .map(|k| {
            let mut row: Vec<u64> = (0..9).map(|j| pow_mod(k, j, p)).collect();
            row.extend((0..9).map(|i| u64::from(i == k)));
            row
        })
        .collect();
    for c in 0..9 {
        let r = (c..9).find(|&r| m[r][c] != 0).unwrap();
        m.swap(r, c);
        let iv = inv_mod(m[c][c], p);
        for x in m[c].iter_mut() {
            *x = *x * iv % p;
        }
        for r in 0..9 {
            if r != c && m[r][c] != 0 {
                let k = m[r][c];
                for j in 0..18 {
                    m[r][j] = (m[r][j] + p * p - k * m[c][j]) % p;
                }
            }
        }
    }
    core::array::from_fn(|j| core::array::from_fn(|k| m[j][9 + k]))
}

/// The nine coefficients in X of S₅(x₁..x₄, X).
fn coeffs_in_last(curve: &Curve4, x: &[E4; 4], vinv: &[[u64; 9]; 9]) -> [E4; 9] {
    let f = &curve.f;
    let vals: Vec<E4> = (0..9u64)
        .map(|k| curve.s5(&[x[0], x[1], x[2], x[3], f.from_fp(k)]))
        .collect();
    core::array::from_fn(|j| {
        vals.iter()
            .enumerate()
            .fold(E4::ZERO, |acc, (k, v)| f.add(&acc, &f.scale(v, vinv[j][k])))
    })
}

/// H(e(t), X) with S₅(x₀+u t₁, …, x₀+u t₄, X) = H(e₁(t), …, e₄(t), X):
/// interpolation at random t ∈ K⁴, Gauss–Jordan over F_{p⁴}, checked on fresh
/// samples. Not charged (once per line, like `precompute`).
fn h_for_line(curve: &Curve4, x0: &E4, u: &E4, rng: &mut StdRng) -> SymmetrisedS5 {
    let f = &curve.f;
    let p = f.p;
    let vinv = vandermonde_inv(p);
    let monos = exponents4(8);
    let n = monos.len();
    let xs_of = |r: &[E4; 4]| -> [E4; 4] { core::array::from_fn(|i| f.add(x0, &f.mul(u, &r[i]))) };
    loop {
        let mut sys: Vec<Vec<E4>> = Vec::with_capacity(n);
        for _ in 0..n {
            let r: [E4; 4] = core::array::from_fn(|_| f.random(rng));
            let e = elementary(f, &r);
            let mut row = mono_values(f, &e, &monos);
            row.extend(coeffs_in_last(curve, &xs_of(&r), &vinv));
            sys.push(row);
        }
        let mut ok = true;
        for c in 0..n {
            let Some(r) = (c..n).find(|&r| !sys[r][c].is_zero()) else {
                ok = false;
                break;
            };
            sys.swap(r, c);
            let inv = f.inv(&sys[c][c]);
            let pivot: Vec<E4> = sys[c].iter().map(|v| f.mul(v, &inv)).collect();
            sys[c] = pivot.clone();
            sys.par_iter_mut().enumerate().for_each(|(r, row)| {
                if r == c || row[c].is_zero() {
                    return;
                }
                let fl = Fp4::new(p);
                let k = row[c];
                for j in c..row.len() {
                    if !pivot[j].is_zero() {
                        let v = fl.mul(&k, &pivot[j]);
                        row[j] = fl.sub(&row[j], &v);
                    }
                }
            });
        }
        if !ok {
            continue;
        }
        let coef: Vec<[E4; 9]> = (0..n)
            .map(|r| core::array::from_fn(|j| sys[r][n + j]))
            .collect();
        // fresh check against the resultant
        let good = (0..8).all(|_| {
            let r: [E4; 4] = core::array::from_fn(|_| f.random(rng));
            let want = coeffs_in_last(curve, &xs_of(&r), &vinv);
            let e = elementary(f, &r);
            let mv = mono_values(f, &e, &monos);
            (0..9).all(|j| {
                mv.iter()
                    .zip(&coef)
                    .fold(E4::ZERO, |acc, (m, c)| f.add(&acc, &f.mul(m, &c[j])))
                    == want[j]
            })
        });
        assert!(good, "line H does not reproduce the resultant");
        return SymmetrisedS5 {
            monos,
            coef,
            precompute_muls: 0,
        };
    }
}

// ── conjugate-pair lines on E4 ────────────────────────────────────────────

fn frob(f: &Fp4, a: &E4, k: u32) -> E4 {
    f.pow(a, (f.p as u128).pow(k))
}

fn nullspace(cols: &[E4], p: u64) -> Vec<Vec<u64>> {
    let m = cols.len();
    let mut a: Vec<Vec<u64>> = (0..4)
        .map(|i| (0..m).map(|j| cols[j].0[i]).collect())
        .collect();
    let mut piv: Vec<usize> = vec![];
    let mut r = 0;
    for c in 0..m {
        let Some(pr) = (r..4).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, pr);
        let iv = inv_mod(a[r][c], p);
        for x in a[r].iter_mut() {
            *x = *x * iv % p;
        }
        for i in 0..4 {
            if i != r && a[i][c] != 0 {
                let k = a[i][c];
                for j in 0..m {
                    a[i][j] = (a[i][j] + p * p - k * a[r][j]) % p;
                }
            }
        }
        piv.push(c);
        r += 1;
        if r == 4 {
            break;
        }
    }
    (0..m)
        .filter(|c| !piv.contains(c))
        .map(|fc| {
            let mut v = vec![0u64; m];
            v[fc] = 1;
            for (ri, &pc) in piv.iter().enumerate() {
                v[pc] = (p - a[ri][fc]) % p;
            }
            v
        })
        .collect()
}

fn conj_lines(f: &Fp4, es: &[E4; 3]) -> Vec<(E4, E4)> {
    let p = f.p;
    let zb: Vec<E4> = (0..4)
        .map(|i| {
            let mut e = [0u64; 4];
            e[i] = 1;
            E4(e)
        })
        .collect();
    let k0b = [E4([1, 0, 0, 0]), E4([0, 0, 1, 0])];
    for g in &k0b {
        assert_eq!(frob(f, g, 2), *g, "K0 basis");
    }
    let mut out: Vec<(Vec<E4>, (E4, E4))> = vec![];
    for lone in 0..3 {
        let oth: Vec<usize> = (0..3).filter(|&i| i != lone).collect();
        for &(ia, ib) in &[(oth[0], oth[1]), (oth[1], oth[0])] {
            let da = f.sub(&es[ia], &es[lone]);
            let db = f.sub(&es[ib], &es[lone]);
            for kk in [1u32, 3] {
                let sda = frob(f, &da, kk);
                let mut cols: Vec<E4> = zb
                    .iter()
                    .map(|v| f.sub(&f.mul(&db, v), &f.mul(&sda, &frob(f, v, kk))))
                    .collect();
                cols.extend(k0b.iter().map(|g| f.sub(g, &frob(f, g, kk))));
                for v in nullspace(&cols, p) {
                    let mut w = E4::ZERO;
                    for (j, b) in zb.iter().enumerate() {
                        w = f.add(&w, &f.scale(b, v[j]));
                    }
                    if w.is_zero() {
                        continue;
                    }
                    let mut g = E4::ZERO;
                    for (j, b) in k0b.iter().enumerate() {
                        g = f.add(&g, &f.scale(b, v[4 + j]));
                    }
                    let u = f.inv(&w);
                    let x0 = f.sub(&es[lone], &f.mul(&g, &u));
                    let mut xs: Vec<E4> = (0..p).map(|t| f.add(&x0, &f.scale(&u, t))).collect();
                    xs.sort();
                    if !out.iter().any(|e| e.0 == xs) {
                        out.push((xs, (x0, u)));
                    }
                }
            }
        }
    }
    out.into_iter().map(|e| e.1).collect()
}

// ── oracle ────────────────────────────────────────────────────────────────

fn fb_points(curve: &Curve4, x0: &E4, u: &E4, rng: &mut StdRng) -> Vec<(u64, Pt4)> {
    let f = &curve.f;
    let mut v = vec![];
    for t in 0..f.p {
        let x = f.add(x0, &f.scale(u, t));
        if let Some(pt) = curve.lift_x(&x, rng) {
            v.push((t, pt));
            if !pt.y.is_zero() {
                v.push((t, curve.neg(&pt)));
            }
        }
    }
    v
}

/// x-multisets (as sorted t values) of all 4-point decompositions of r.
fn mitm(curve: &Curve4, fb: &[(u64, Pt4)], r: &Pt4) -> HashSet<[u64; 4]> {
    let mut table: HashMap<Pt4, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..fb.len() {
        for j in i..fb.len() {
            table
                .entry(curve.add(&fb[i].1, &fb[j].1))
                .or_default()
                .push((i, j));
        }
    }
    let mut out = HashSet::new();
    for (s, pairs) in &table {
        let need = curve.sub(r, s);
        if let Some(other) = table.get(&need) {
            for &(i, j) in pairs {
                for &(k, l) in other {
                    let mut ts = [fb[i].0, fb[j].0, fb[k].0, fb[l].0];
                    ts.sort();
                    out.insert(ts);
                }
            }
        }
    }
    out
}

/// Roots in F_p (with multiplicity) of t⁴ − e1 t³ + e2 t² − e3 t + e4.
fn split_brute(e: &[u64; 4], p: u64) -> Option<[u64; 4]> {
    let mut c = vec![e[3], (p - e[2]) % p, e[1], (p - e[0]) % p, 1]; // low to high
    let mut roots = vec![];
    while c.len() > 1 {
        let r = (0..p).find(|&t| c.iter().rev().fold(0u64, |acc, &a| (acc * t + a) % p) == 0)?;
        roots.push(r);
        // synthetic division by (t - r)
        let n = c.len() - 1;
        let mut q = vec![0u64; n];
        let mut acc = 0u64;
        for i in (0..=n).rev() {
            acc = (acc * r + c[i]) % p;
            if i > 0 {
                q[i - 1] = acc;
            }
        }
        c = q;
    }
    let mut out = [roots[0], roots[1], roots[2], roots[3]];
    out.sort();
    Some(out)
}

// ── driver ────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn solve_one(
    curve: &Curve4,
    h: &SymmetrisedS5,
    fb: &[(u64, Pt4)],
    r: &Pt4,
    x0: &E4,
    u: &E4,
    rng: &mut StdRng,
    label: &str,
    macaulay_degree: u8,
) -> String {
    let f = &curve.f;
    let p = f.p;
    let t0 = Instant::now();
    let before = f.muls();
    let comps = h.weil_restrict(f, &r.x);
    let weil = f.muls() - before;
    let mut st = QuarticSolve::default();
    let opts = PlainOptions {
        chains: true,
        sparsest: true,
    };
    let sols = solve_system4_with(
        &comps,
        opts,
        8,
        macaulay_degree,
        p,
        rng,
        &mut st,
        false,
        None,
        None,
    );
    st.weil_muls += weil;
    st.close_public();
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    // solver x-multisets: e -> split -> all x lift
    let mut solver: HashSet<[u64; 4]> = HashSet::new();
    let n_e = sols.as_ref().map(|s| s.len()).unwrap_or(0);
    if let Some(es) = &sols {
        for e in es {
            if let Some(ts) = split_brute(e, p) {
                let lifts = ts
                    .iter()
                    .all(|&t| f.is_square(&curve.rhs(&f.add(x0, &f.scale(u, t)))));
                if lifts {
                    solver.insert(ts);
                }
            }
        }
    }
    let oracle = mitm(curve, fb, r);
    let mut sv: Vec<_> = solver.iter().collect();
    sv.sort();
    format!(
        "{{\"kind\":\"solve\",{label},\"solved\":{},\"rows\":{},\"cols\":{},\"pivots\":{},\"dim\":{},\"total_muls\":{},\"weil_muls\":{},\"echelon_muls\":{},\"nf_muls\":{},\"charpoly_muls\":{},\"eigenvector_muls\":{},\"e_solutions\":{n_e},\"solver_decomps\":{},\"oracle_decomps\":{},\"match\":{},\"decomps\":\"{:?}\",\"wall_ms\":{ms:.0}}}",
        sols.is_some(),
        st.rows,
        st.cols,
        st.pivots,
        st.dim,
        st.total_muls,
        st.weil_muls,
        st.echelon_muls,
        st.nf_muls,
        st.charpoly_muls,
        st.eigenvector_muls,
        solver.len(),
        oracle.len(),
        solver == oracle,
        sv
    )
}

// ── summarizer (PROTOCOL rules K1–K3 and A1's M1) ────────────────────────

fn jrows(path: &str) -> Vec<serde_json::Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
fn u(v: &serde_json::Value, k: &str) -> u64 {
    v[k].as_u64()
        .unwrap_or_else(|| v[k].to_string().parse().unwrap_or(0))
}
fn b(v: &serde_json::Value, k: &str) -> bool {
    v[k].as_bool().unwrap_or(false)
}
fn s<'a>(v: &'a serde_json::Value, k: &str) -> &'a str {
    v[k].as_str().unwrap_or("")
}

/// `--summarize <main rows> <a1 rows> <out dir>`: SUMMARY.md, summary.json,
/// validator-report.json (pins both row files by SHA-256).
fn summarize(main_rows: &str, a1_rows: &str, out: &str, claimed: Option<&str>) {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;
    let rows = jrows(main_rows);
    let a1 = jrows(a1_rows);
    let solves: Vec<&serde_json::Value> = rows
        .iter()
        .filter(|r| s(r, "kind") == "solve" && !b(r, "aa"))
        .collect();
    let aas: Vec<&serde_json::Value> = rows.iter().filter(|r| b(r, "aa")).collect();
    let controls_ok = rows
        .iter()
        .filter(|r| s(r, "kind") == "control")
        .all(|r| b(r, "std_H_equals_precompute"));
    let mut md = String::new();
    let mut vals: Vec<(String, f64)> = vec![];
    // K1
    let k1_bad: Vec<String> = solves
        .iter()
        .filter(|r| u(r, "dim") != 4096 || u(r, "rows") != 41780 || u(r, "cols") != 40920)
        .map(|r| {
            format!(
                "curve {} {} {} (dim {})",
                u(r, "curve"),
                s(r, "arm"),
                s(r, "target"),
                u(r, "dim")
            )
        })
        .collect();
    // K3
    let k3_bad: Vec<String> = solves
        .iter()
        .filter(|r| !b(r, "match"))
        .map(|r| {
            format!(
                "curve {} {} {}: solver {} vs oracle {}",
                u(r, "curve"),
                s(r, "arm"),
                s(r, "target"),
                u(r, "solver_decomps"),
                u(r, "oracle_decomps")
            )
        })
        .collect();
    // K2 per common target
    let mut k2_rows = String::new();
    let mut k2_ok = true;
    for c in [1u64, 2] {
        for t in ["common0", "common1"] {
            let g = |arm: &str| {
                solves
                    .iter()
                    .find(|r| u(r, "curve") == c && s(r, "arm") == arm && s(r, "target") == t)
                    .copied()
            };
            if let (Some(a), Some(z)) = (g("conj"), g("std")) {
                let ratio = u(a, "total_muls") as f64 / u(z, "total_muls") as f64;
                let both = b(a, "solved") && b(z, "solved") && b(a, "match") && b(z, "match");
                if !(0.9..=1.1).contains(&ratio) || !both {
                    k2_ok = false;
                }
                writeln!(
                    k2_rows,
                    "| {c} | {t} | {:.4e} ({}) | {:.4e} ({}) | {ratio:.3} |",
                    u(a, "total_muls") as f64,
                    if b(a, "match") { "ok" } else { "FAILED" },
                    u(z, "total_muls") as f64,
                    if b(z, "match") { "ok" } else { "FAILED" }
                )
                .unwrap();
                vals.push((format!("ratio_c{c}_{t}_x1000"), (ratio * 1000.0).round()));
            }
        }
    }
    let med = |arm: &str| {
        let mut v: Vec<f64> = solves
            .iter()
            .filter(|r| s(r, "arm") == arm)
            .map(|r| u(r, "total_muls") as f64)
            .collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = v.len();
        if n % 2 == 1 {
            v[n / 2]
        } else {
            (v[n / 2 - 1] + v[n / 2]) / 2.0
        }
    };
    let med_ratio = med("conj") / med("std");
    let k2_med_ok = (0.95..=1.05).contains(&med_ratio);
    let aa_ok = aas.iter().all(|a| {
        solves.iter().any(|r| {
            u(r, "curve") == u(a, "curve")
                && s(r, "arm") == s(a, "arm")
                && s(r, "target") == s(a, "target")
                && u(r, "total_muls") == u(a, "total_muls")
        })
    });
    vals.push(("k1_violations".into(), k1_bad.len() as f64));
    vals.push(("k3_violations".into(), k3_bad.len() as f64));
    vals.push(("median_ratio_x1000".into(), (med_ratio * 1000.0).round()));
    vals.push(("solves".into(), solves.len() as f64));
    // A1 / M1
    let mut a1_tab = String::new();
    let mut m1: Vec<String> = vec![];
    let a1s: Vec<&serde_json::Value> = a1.iter().filter(|r| s(r, "kind") == "solve").collect();
    let mut targets: Vec<(u64, String)> = a1s
        .iter()
        .map(|r| (u(r, "curve"), s(r, "target").to_string()))
        .collect();
    targets.dedup();
    for (c, t) in &targets {
        let mut first_ok: Option<(u64, u64)> = None;
        for r in a1s
            .iter()
            .filter(|r| u(r, "curve") == *c && s(r, "target") == t)
        {
            let ok = b(r, "solved") && b(r, "match");
            writeln!(
                a1_tab,
                "| {c} | {t} | {} | {} | {} | {:.4e} | {}/{} | {} |",
                u(r, "macaulay_degree"),
                u(r, "rows"),
                u(r, "dim"),
                u(r, "total_muls") as f64,
                u(r, "solver_decomps"),
                u(r, "oracle_decomps"),
                if ok { "ok" } else { "FAILED" }
            )
            .unwrap();
            if ok && first_ok.is_none() {
                first_ok = Some((u(r, "macaulay_degree"), u(r, "total_muls")));
            }
        }
        let std_cost = solves
            .iter()
            .filter(|r| u(r, "curve") == *c && s(r, "arm") == "std")
            .map(|r| u(r, "total_muls") as f64)
            .sum::<f64>()
            / solves
                .iter()
                .filter(|r| u(r, "curve") == *c && s(r, "arm") == "std")
                .count() as f64;
        match first_ok {
            Some((d, m)) => {
                m1.push(format!("curve {c} {t}: first working degree {d}, {:.4e} muls = {:.2}× the std arm's mean degree-29 cost on that curve", m as f64, m as f64 / std_cost));
                vals.push((format!("m1_c{c}_degree"), d as f64));
            }
            None => {
                m1.push(format!("curve {c} {t}: no degree ≤ 32 works"));
                vals.push((format!("m1_c{c}_degree"), 0.0));
            }
        }
    }
    let v = |x: bool| if x { "PASS" } else { "FAIL" };
    writeln!(md, "# Summary (coded rules, PROTOCOL.md)\n").unwrap();
    writeln!(
        md,
        "- Control: std H equals `SymmetrisedS5::precompute` on every curve: **{}**",
        v(controls_ok)
    )
    .unwrap();
    writeln!(
        md,
        "- **K1** (rows 41,780, cols 40,920, dim 4,096 on every solve): **{}** {:?}",
        v(k1_bad.is_empty()),
        k1_bad
    )
    .unwrap();
    writeln!(md, "- **K2** (common-target ratio in [0.9, 1.1] with both solves correct): **{}**; median ratio over all solves {med_ratio:.3} (band [0.95, 1.05]: **{}**)", v(k2_ok), v(k2_med_ok)).unwrap();
    writeln!(
        md,
        "- **K3** (solver = oracle on every solve): **{}** {:?}",
        v(k3_bad.is_empty()),
        k3_bad
    )
    .unwrap();
    writeln!(
        md,
        "- **A/A** (repeated solve, identical count): **{}**\n",
        v(aa_ok)
    )
    .unwrap();
    writeln!(md, "## Common targets\n\n| curve | target | conj total_muls | std total_muls | conj/std |\n|--:|:--|--:|--:|--:|\n{k2_rows}").unwrap();
    writeln!(md, "## All solves (Macaulay degree 29)\n\n| curve | arm | target | dim | total_muls | solver/oracle | ok |\n|--:|:--|:--|--:|--:|--:|:--|").unwrap();
    for r in &solves {
        writeln!(
            md,
            "| {} | {} | {} | {} | {:.4e} | {}/{} | {} |",
            u(r, "curve"),
            s(r, "arm"),
            s(r, "target"),
            u(r, "dim"),
            u(r, "total_muls") as f64,
            u(r, "solver_decomps"),
            u(r, "oracle_decomps"),
            if b(r, "match") { "ok" } else { "FAILED" }
        )
        .unwrap();
    }
    writeln!(md, "\n## Amendment A1: decomposable conj targets at Macaulay degree 30–32\n\n| curve | target | degree | rows | dim | total_muls | solver/oracle | ok |\n|--:|:--|--:|--:|--:|--:|--:|:--|\n{a1_tab}\nM1:\n").unwrap();
    for l in &m1 {
        writeln!(md, "- {l}").unwrap();
    }
    std::fs::create_dir_all(out).unwrap();
    std::fs::write(format!("{out}/SUMMARY.md"), &md).unwrap();
    let js = format!(
        "{{\"K1\":{},\"K2\":{k2_ok},\"K2_median\":{k2_med_ok},\"K3\":{},\"AA\":{aa_ok},\"control\":{controls_ok},\"values\":{{{}}}}}\n",
        k1_bad.is_empty(),
        k3_bad.is_empty(),
        vals.iter().map(|(k, v)| format!("\"{k}\":{v}")).collect::<Vec<_>>().join(",")
    );
    std::fs::write(format!("{out}/summary.json"), &js).unwrap();
    let sha = |p: &str| hex::encode(Sha256::digest(std::fs::read(p).unwrap()));
    // claimed values come from a separate file written from the README
    let claimed_map: HashMap<String, f64> = claimed
        .map(|p| {
            std::fs::read_to_string(p)
                .unwrap()
                .lines()
                .filter_map(|l| {
                    let t = l.trim().trim_end_matches(',');
                    let (k, v) = t.split_once(':')?;
                    Some((
                        k.trim().trim_matches('"').to_string(),
                        v.trim().parse().ok()?,
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    let mut status_ok = claimed.is_some();
    let heads: Vec<String> = vals
        .iter()
        .filter_map(|(k, v)| {
            let c = claimed_map.get(k)?;
            if c != v {
                status_ok = false;
            }
            Some(format!(
                "    {{\"name\": \"{k}\", \"claimed\": {c}, \"recomputed\": {v}, \"rel_tol\": 0}}"
            ))
        })
        .collect();
    if heads.is_empty() {
        status_ok = false;
    }
    let rel = |p: &str| {
        std::path::Path::new(p)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    };
    let rep = format!(
        "{{\n  \"schema\": \"validator-report\",\n  \"status\": \"{}\",\n  \"study\": \"research/lift_dense_conjugate_pair_decomp_20261106\",\n  \"recomputed_by\": {{\"tool\": \"examples/conjpair_decomp_cost.rs --summarize\"}},\n  \"note\": \"Producer-run recompute of the summary values from the pinned rows; unsigned, so independence is not established.\",\n  \"raw_data\": [\n    {{\"path\": \"{}\", \"sha256\": \"{}\"}},\n    {{\"path\": \"{}\", \"sha256\": \"{}\"}}\n  ],\n  \"headline\": [\n{}\n  ]\n}}\n",
        if status_ok { "pass" } else { "fail" },
        rel(main_rows), sha(main_rows), rel(a1_rows), sha(a1_rows), heads.join(",\n")
    );
    std::fs::write(format!("{out}/validator-report.json"), rep).unwrap();
    print!("{md}");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("--summarize") {
        summarize(
            &args[1],
            &args[2],
            &args[3],
            args.get(4).map(|s| s.as_str()),
        );
        return;
    }
    let (mut p, mut ncurves, mut out, mut aa) = (29u64, 2usize, String::from("rows.jsonl"), false);
    // Amendment A1: Macaulay degrees to try, and an optional restriction to
    // one arm / one curve index (curves are still built in order so that the
    // random stream, and hence curves and targets, are unchanged).
    let mut macdegs: Vec<u8> = vec![29];
    let mut only_arm: Option<String> = None;
    let mut only_curve: Option<usize> = None;
    let mut only_failing = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--p" => {
                i += 1;
                p = args[i].parse().unwrap()
            }
            "--curves" => {
                i += 1;
                ncurves = args[i].parse().unwrap()
            }
            "--out" => {
                i += 1;
                out = args[i].clone()
            }
            "--aa" => aa = true,
            "--macdeg" => {
                i += 1;
                macdegs = args[i].split(',').map(|v| v.parse().unwrap()).collect()
            }
            "--only-arm" => {
                i += 1;
                only_arm = Some(args[i].clone())
            }
            "--only-curve" => {
                i += 1;
                only_curve = Some(args[i].parse().unwrap())
            }
            "--only-decomposable" => only_failing = true,
            o => panic!("unknown {o}"),
        }
        i += 1;
    }
    let mut fh = File::create(&out).unwrap();
    let mut rng = StdRng::seed_from_u64(0x20261106_c0de);
    let mut done = 0;
    let mut ci = 0;
    while done < ncurves {
        ci += 1;
        // full 2-torsion: roots r1, r2, r3 = -r1 - r2 in K
        let f0 = Fp4::new(p);
        let r1 = f0.random(&mut rng);
        let r2 = f0.random(&mut rng);
        let r3 = f0.neg(&f0.add(&r1, &r2));
        if r1 == r2 || r1 == r3 || r2 == r3 {
            continue;
        }
        let a = f0.add(
            &f0.add(&f0.mul(&r1, &r2), &f0.mul(&r1, &r3)),
            &f0.mul(&r2, &r3),
        );
        let b = f0.neg(&f0.mul(&f0.mul(&r1, &r2), &r3));
        let curve = Curve4::with_params(p, a, b);
        let f = &curve.f;
        // reject j in F_{p^2}
        let a3 = f.mul(&f.sq(&a), &a);
        let den = f.add(&f.scale(&a3, 4), &f.scale(&f.sq(&b), 27));
        if den.is_zero() {
            continue;
        }
        let j = f.mul(&f.scale(&a3, 4 * 1728 % p), &f.inv(&den));
        if frob(f, &j, 2) == j {
            writeln!(
                fh,
                "{{\"kind\":\"curve\",\"p\":{p},\"curve\":{ci},\"skipped\":\"j_in_Fp2\"}}"
            )
            .unwrap();
            continue;
        }
        let es = [r1, r2, r3];
        for e in &es {
            assert!(curve.rhs(e).is_zero(), "root is a 2-torsion abscissa");
        }
        let lines = conj_lines(f, &es);
        let conj = lines
            .iter()
            .find(|(x0, u)| (0..p).all(|t| f.is_square(&curve.rhs(&f.add(x0, &f.scale(u, t))))));
        let Some(&(cx0, cu)) = conj else {
            writeln!(fh, "{{\"kind\":\"curve\",\"p\":{p},\"curve\":{ci},\"lines\":{},\"skipped\":\"no_all_lift\"}}", lines.len()).unwrap();
            continue;
        };
        done += 1;
        writeln!(
            fh,
            "{{\"kind\":\"curve\",\"p\":{p},\"curve\":{ci},\"lines\":{},\"skipped\":\"\"}}",
            lines.len()
        )
        .unwrap();
        // positive control: std H equals SymmetrisedS5::precompute
        let h_std = h_for_line(&curve, &E4::ZERO, &E4::ONE, &mut rng);
        let pre = SymmetrisedS5::precompute(&curve, &mut rng);
        let mut ref_map: HashMap<[u8; 4], [E4; 9]> = HashMap::new();
        for (m, c) in pre.monos.iter().zip(&pre.coef) {
            ref_map.insert(*m, *c);
        }
        let std_equal = h_std
            .monos
            .iter()
            .zip(&h_std.coef)
            .all(|(m, c)| ref_map.get(m) == Some(c));
        let h_conj = h_for_line(&curve, &cx0, &cu, &mut rng);
        writeln!(fh, "{{\"kind\":\"control\",\"p\":{p},\"curve\":{ci},\"std_H_equals_precompute\":{std_equal}}}").unwrap();
        fh.flush().unwrap();
        let fb_std = fb_points(&curve, &E4::ZERO, &E4::ONE, &mut rng);
        let fb_conj = fb_points(&curve, &cx0, &cu, &mut rng);
        let rand_pt = |rng: &mut StdRng| loop {
            let x = f.random(rng);
            if let Some(pt) = curve.lift_x(&x, rng) {
                break pt;
            }
        };
        let common: Vec<Pt4> = (0..2).map(|_| rand_pt(&mut rng)).collect();
        for (arm, x0, u, h, fb) in [
            ("conj", cx0, cu, &h_conj, &fb_conj),
            ("std", E4::ZERO, E4::ONE, &h_std, &fb_std),
        ] {
            // planted: sum of 4 random factor-base points of this arm
            let mut planted = Pt4::INF;
            let mut pts = vec![];
            for _ in 0..4 {
                let k = rng.gen_range(0..fb.len());
                pts.push(fb[k].0);
                planted = curve.add(&planted, &fb[k].1);
            }
            pts.sort();
            let mut targets: Vec<(String, Pt4)> = vec![(format!("planted{:?}", pts), planted)];
            for (ti, t) in common.iter().enumerate() {
                targets.push((format!("common{ti}"), *t));
            }
            for (name, r) in &targets {
                if r.inf {
                    continue;
                }
                if only_arm.as_deref().is_some_and(|a| a != arm)
                    || only_curve.is_some_and(|c| c != ci)
                {
                    continue;
                }
                if only_failing && mitm(&curve, fb, r).is_empty() {
                    continue;
                }
                for &md in &macdegs {
                    let label = if macdegs == [29] {
                        format!("\"p\":{p},\"curve\":{ci},\"arm\":\"{arm}\",\"target\":\"{name}\",\"fb_points\":{}", fb.len())
                    } else {
                        format!("\"p\":{p},\"curve\":{ci},\"arm\":\"{arm}\",\"target\":\"{name}\",\"fb_points\":{},\"macaulay_degree\":{md}", fb.len())
                    };
                    let mut srng = StdRng::seed_from_u64(0xdec0 + ci as u64);
                    let row = solve_one(&curve, h, fb, r, &x0, &u, &mut srng, &label, md);
                    eprintln!("{row}");
                    writeln!(fh, "{row}").unwrap();
                    fh.flush().unwrap();
                }
                if aa && name == "common0" {
                    let mut srng2 = StdRng::seed_from_u64(0xdec0 + ci as u64);
                    let row2 = solve_one(
                        &curve,
                        h,
                        fb,
                        r,
                        &x0,
                        &u,
                        &mut srng2,
                        &format!(
                            "\"p\":{p},\"curve\":{ci},\"arm\":\"{arm}\",\"target\":\"{name}\",\"fb_points\":{},\"aa\":true",
                            fb.len()
                        ),
                        29,
                    );
                    writeln!(fh, "{row2}").unwrap();
                    fh.flush().unwrap();
                }
            }
        }
    }
}
