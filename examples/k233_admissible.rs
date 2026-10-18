//! K-233 derived row of the admissible-degree inequality (Lemma 2 of
//! `research/ic_leads_20261018/LEMMAS.md`, lead L8) on Frobenius-stable
//! factor bases.  Derived calculation: no `S`, no measured cost.
//!
//! The formulas are those of the derived table of `dreg_score slope`
//! (`examples/dreg_score.rs`, "Derived (cost model, w = 1 …)"): relations
//! `2^ℓ/(2n)`, attempts `max(1, m!·2^{n−mℓ})`, budget per oracle call
//! `T_ρ/(R·A)`, oracle cost `C(N, ≤D)^w`.  Only the matched rho differs on
//! purpose: here `r` is the committed order of sect233k1
//! (`src/binary_ecc/curve.rs`, `BinaryCurve::sect233k1`), where
//! `dreg_score` uses `r = 2^n/cofactor`.
//!
//! Run: `cargo run --release --example k233_admissible`.

use std::f64::consts::PI;

const N: u64 = 233;
/// Large prime order of sect233k1 (cofactor 4), `src/binary_ecc/curve.rs`.
const R_HEX: &str = "8000000000000000000000000000069d5bb915bcd46efb1ad5f173abdf";
/// Largest degree tried; the same cap as `dreg_score slope`.
const D_MAX: u64 = 40;

/// log2 of a hex integer, from its leading 13 hex digits.
fn log2_hex(h: &str) -> f64 {
    let lead = &h[..13.min(h.len())];
    let v = u64::from_str_radix(lead, 16).unwrap() as f64;
    v.log2() + 4.0 * (h.len() - lead.len()) as f64
}

/// log2 of `Σ_{i≤d} C(nv, i)`, in the log domain.
fn log2_cols(nv: u64, d: u64) -> f64 {
    let mut terms = Vec::new();
    let mut lt = 0f64; // ln C(nv, 0)
    for i in 0..=d.min(nv) {
        if i > 0 {
            lt += ((nv - i + 1) as f64).ln() - (i as f64).ln();
        }
        terms.push(lt);
    }
    let mx = terms.iter().cloned().fold(f64::MIN, f64::max);
    let s: f64 = terms.iter().map(|t| (t - mx).exp()).sum();
    (mx + s.ln()) / std::f64::consts::LN_2
}

/// log2 of the budget per oracle call, `T_ρ/(R·A)` with `T_ρ = √(π r/(8n))`.
fn log2_budget(n: u64, m: u64, ell: u64, log2_r: f64) -> f64 {
    let rho = 0.5 * ((PI / (8.0 * n as f64)).log2() + log2_r);
    let fact: f64 = (1..=m).map(|i| i as f64).product();
    let attempts = (fact.log2() + n as f64 - (m * ell) as f64).max(0.0);
    let relations = ell as f64 - (2.0 * n as f64).log2();
    rho - relations - attempts
}

/// Largest `D` in `2..=D_MAX` with `w·log2 C(N, ≤D) ≤ log2 B`.
fn admissible(nv: u64, w: f64, log2_b: f64) -> Option<u64> {
    (2..=D_MAX).rev().find(|&d| w * log2_cols(nv, d) <= log2_b)
}

/// Sizes of the 2-cyclotomic cosets of Z/n (n odd).
fn cosets(n: u64) -> Vec<u64> {
    let mut seen = vec![false; n as usize];
    let mut sizes = Vec::new();
    for s in 0..n {
        if seen[s as usize] {
            continue;
        }
        let (mut x, mut len) = (s, 0);
        while !seen[x as usize] {
            seen[x as usize] = true;
            x = 2 * x % n;
            len += 1;
        }
        sizes.push(len);
    }
    sizes
}

fn ord2(n: u64) -> u64 {
    let (mut x, mut k) = (2 % n, 1);
    while x != 1 {
        x = 2 * x % n;
        k += 1;
    }
    k
}

/// Dimensions of Frobenius-stable F2-subspaces: subset sums of coset sizes.
fn stable_dims(sizes: &[u64]) -> Vec<u64> {
    let total: u64 = sizes.iter().sum();
    let mut ok = vec![false; total as usize + 1];
    ok[0] = true;
    for &s in sizes {
        for d in (0..=(total - s) as usize).rev() {
            if ok[d] {
                ok[d + s as usize] = true;
            }
        }
    }
    (0..=total).filter(|&d| ok[d as usize]).collect()
}

fn main() {
    let log2_r = log2_hex(R_HEX);
    let sizes = cosets(N);
    let ord = ord2(N);
    let ones = sizes.iter().filter(|&&s| s == 1).count();
    let full = sizes.iter().filter(|&&s| s == ord).count();
    println!("# K-233 derived row (derived calculation; no S, no measured cost)\n");
    println!(
        "ord_{N}(2) = {ord}; cosets: {} total, {ones} of size 1, {full} of size {ord}; check {}",
        sizes.len(),
        if ord == 29 && ones == 1 && full == 8 && sizes.len() == 9 {
            "PASS"
        } else {
            "FAIL"
        }
    );
    let dims = stable_dims(&sizes);
    println!(
        "log2 r = {log2_r:.4} (r = 0x{R_HEX}); log2 T_rho = {:.3}",
        0.5 * ((PI / (8.0 * N as f64)).log2() + log2_r)
    );
    println!(
        "stable dimensions (a in 0..8, c in 0..1): {}\n",
        dims.iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("| m | ℓ | m·ℓ | log₂ B | D (chained, w=1) | D (chained, w=2.2) | D (elim, w=1) | D (elim, w=2.2) | D=4 fits (any) |");
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|---|");
    let show = |a: Option<u64>| a.map_or("none".to_string(), |d| d.to_string());
    // (m, ℓ, mℓ, [D for chained w1, chained w2.2, elim w1, elim w2.2])
    let mut rows: Vec<(u64, u64, [Option<u64>; 4])> = Vec::new();
    let mut jsons = Vec::new();
    for m in 3..=6u64 {
        for &ell in &dims {
            let ml = m * ell;
            // mℓ in a sensible range around n
            if (ml as f64) < 0.6 * N as f64 || (ml as f64) > 1.5 * N as f64 {
                continue;
            }
            let b = log2_budget(N, m, ell, log2_r);
            let n_ch = ml + (m - 2) * N;
            let d = [
                admissible(n_ch, 1.0, b),
                admissible(n_ch, 2.2, b),
                admissible(ml, 1.0, b),
                admissible(ml, 2.2, b),
            ];
            let fits4 = d.iter().any(|x| x.is_some_and(|v| v >= 4));
            println!(
                "| {m} | {ell} | {ml} | {b:.1} | {} | {} | {} | {} | {} |",
                show(d[0]),
                show(d[1]),
                show(d[2]),
                show(d[3]),
                if fits4 { "yes" } else { "no" }
            );
            jsons.push(serde_json::json!({
                "kind": "derived", "n": N, "m": m, "ell": ell, "m_ell": ml,
                "log2_budget": (b * 1000.0).round() / 1000.0,
                "d_chained_w1": d[0], "d_chained_w2_2": d[1],
                "d_elim_w1": d[2], "d_elim_w2_2": d[3],
                "d4_fits": fits4,
            }));
            rows.push((m, ell, d));
        }
    }
    println!();
    for line in &jsons {
        println!("{line}");
    }
    println!();
    let names = [
        "chained N, w = 1",
        "chained N, w = 2.2",
        "N = m·ℓ, w = 1",
        "N = m·ℓ, w = 2.2",
    ];
    for dmin in [4u64, 5] {
        for (k, name) in names.iter().enumerate() {
            let first = rows
                .iter()
                .filter(|(_, _, d)| d[k].is_some_and(|v| v >= dmin))
                .min_by_key(|(m, l, _)| (m * l, *m));
            match first {
                Some((m, l, d)) => println!(
                    "first D >= {dmin} fits, {name}: m = {m}, ℓ = {l}, m·ℓ = {} (D_adm = {})",
                    m * l,
                    d[k].unwrap()
                ),
                None => println!("first D >= {dmin} fits, {name}: none in the listed range"),
            }
        }
    }
    println!("\nPer m, first stable ℓ with D >= 4 (chained w=1 / chained w=2.2 / elim w=1 / elim w=2.2):");
    for m in 3..=6u64 {
        let f = |k: usize| {
            rows.iter()
                .filter(|(mm, _, d)| *mm == m && d[k].is_some_and(|v| v >= 4))
                .map(|(_, l, _)| *l)
                .min()
                .map_or("none".to_string(), |l| l.to_string())
        };
        println!("m = {m}: {} / {} / {} / {}", f(0), f(1), f(2), f(3));
    }
    println!("\nMeasured wall (research/dreg_closure_slope_20261018/RESULTS.md): closure degree 4 at m·ℓ <= 18, > 4 at m·ℓ = 21. Every row above is derived, not measured.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `dreg_score slope` derived table at n = 233 (r = 2^n/4, ℓ = ⌈n/m⌉):
    /// `| 233 | 3 | 78 | 40.2 | 5 | 6 |` and `| 233 | 5 | 47 | 67.9 | 8 | 12 |`.
    #[test]
    fn reproduces_dreg_score_slope_rows() {
        for (m, ell, budget, chained, elim) in
            [(3u64, 78u64, 40.2, 5u64, 6u64), (5, 47, 67.9, 8, 12)]
        {
            assert_eq!(233u64.div_ceil(m), ell);
            let b = log2_budget(233, m, ell, 233.0 - 2.0);
            assert!((b - budget).abs() < 0.05, "budget {b}");
            assert_eq!(admissible(m * ell + (m - 2) * 233, 1.0, b), Some(chained));
            assert_eq!(admissible(m * ell, 1.0, b), Some(elim));
        }
    }

    #[test]
    fn cosets_of_233() {
        let s = cosets(233);
        assert_eq!(ord2(233), 29);
        assert_eq!(s.iter().filter(|&&x| x == 1).count(), 1);
        assert_eq!(s.iter().filter(|&&x| x == 29).count(), 8);
    }
}
