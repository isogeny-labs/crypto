//! Census for the gap search `research/cover_decomposition_gap_search_20261018/`:
//! can the ECC2K-130 trace-zero variety `A` (dim 130, `#A(F_2) = r` prime) sit
//! in the Prym of a geometrically cyclic degree-131 cover `C → C_0` over `F_2`
//! with `g(C)` inside the index-calculus window `[130, ~300)`?
//!
//! Derivation (README §4): a trivial-Frobenius-action cover needs
//! `131 | #Jac(C_0)(F_2)` if unramified (class field theory over `F_2`), or
//! branch places of degree ≥ 130 if ramified (`131 | 2^d − 1 ⇒ 130 | d`);
//! twisted actions (`d > 1`) force the Prym to be `μ_d`-stable, which `A`'s
//! Weil numbers are not, and then need `g(C_0) ≥ d + 1`.  Inside the window
//! only one case survives: unramified, trivial action, `g(C_0) = 3`,
//! `#Jac(C_0)(F_2) = 131` (the Weil bound `(1+√2)^6 < 262` forces equality),
//! giving `g(C) = 263`.  This example enumerates every genus-3 curve over
//! `F_2` (hyperelliptic models and all plane quartics, via the module that
//! `RESEARCH_ECC2K130_CURVE_CONSTRUCTION.md` used) against every Weil
//! polynomial `P` of degree 6 with `P(1) = 131`.
//!
//! The same enumerator is run at genus 2 for the toy ladder `n ∈ {5, 7, 11, 13}`
//! (unramified `Z/n`-covers of genus-2 curves, `g(C) = n + 1`, Prym of
//! dimension `n − 1 = dim A_n`), recording the pairs `(#Jac, N_1)`; the
//! necessary condition derived in README §4 is `N_1 ∈ {1, n + 1}`.
//!
//! Run: `cargo run --release --example unramified_cyclic_cover_census`
//! (writes JSON to stdout).

use crypto_lib::cryptanalysis::curve_construction::enumerate::{
    hyperelliptic, plane_quartics, Target,
};
use serde::Serialize;
use std::collections::BTreeMap;

/// Monic `P(T) = T^6 + c1 T^5 + c2 T^4 + c3 T^3 + 2 c2 T^2 + 4 c1 T + 8`,
/// little-endian, with `P(1) = 131`.
fn genus3_targets() -> Vec<Target> {
    let mut v = Vec::new();
    for c1 in -6i128..=6 {
        for c2 in -40i128..=40 {
            let c3 = 122 - 5 * c1 - 3 * c2;
            if c3.abs() > 80 {
                continue;
            }
            v.push(Target {
                name: format!("P(1)=131:c1={c1},c2={c2},c3={c3}"),
                poly: vec![8, 4 * c1, 2 * c2, c3, c2, c1, 1],
            });
        }
    }
    v
}

/// Monic `P(T) = T^4 + c1 T^3 + c2 T^2 + 2 c1 T + 4` with `n | P(1)`.
fn genus2_targets(n: i128) -> Vec<Target> {
    let mut v = Vec::new();
    for c1 in -4i128..=4 {
        for c2 in -12i128..=12 {
            let p1 = 5 + 3 * c1 + c2;
            if p1 <= 0 || p1 % n != 0 {
                continue;
            }
            // N_1 = 2 + 1 − S_1 with S_1 = −c1, so N_1 = 3 + c1.
            v.push(Target {
                name: format!("n={n}:#Jac={p1},N1={},c1={c1},c2={c2}", 3 + c1),
                poly: vec![4, 2 * c1, c2, c1, 1],
            });
        }
    }
    v
}

#[derive(Serialize)]
struct Out {
    genus3_targets: usize,
    genus3_hyperelliptic_models_admitted: u64,
    genus3_hyperelliptic_hits: u64,
    genus3_quartic_models_admitted: u64,
    genus3_quartic_hits: u64,
    genus3_hit_examples: Vec<String>,
    genus2_models_admitted: u64,
    /// `n -> {"#Jac=..,N1=..": hit models}` over every target with `n | #Jac`.
    genus2_hits: BTreeMap<String, BTreeMap<String, u64>>,
}

fn main() {
    let t3 = genus3_targets();
    let hyp = hyperelliptic(3, &t3);
    let (quart, _) = plane_quartics(&t3);
    let mut hits3 = Vec::new();
    let hh: u64 = hyp.targets.iter().map(|t| t.hit_models).sum();
    let qh: u64 = quart.targets.iter().map(|t| t.hit_models).sum();
    for t in hyp.targets.iter().chain(quart.targets.iter()) {
        if t.hit_models > 0 {
            hits3.push(format!(
                "{}: {} models, e.g. {:?}",
                t.target, t.hit_models, t.examples
            ));
        }
    }
    let mut g2 = BTreeMap::new();
    let mut g2_admitted = 0;
    for n in [5i128, 7, 11, 13] {
        let t2 = genus2_targets(n);
        let r = hyperelliptic(2, &t2);
        g2_admitted = r.models_admitted;
        let mut m = BTreeMap::new();
        for t in &r.targets {
            if t.hit_models > 0 {
                let key = t.target.split(':').nth(1).unwrap_or(&t.target).to_string();
                *m.entry(key).or_insert(0) += t.hit_models;
            }
        }
        g2.insert(format!("n={n}"), m);
    }
    let out = Out {
        genus3_targets: t3.len(),
        genus3_hyperelliptic_models_admitted: hyp.models_admitted,
        genus3_hyperelliptic_hits: hh,
        genus3_quartic_models_admitted: quart.models_admitted,
        genus3_quartic_hits: qh,
        genus3_hit_examples: hits3,
        genus2_models_admitted: g2_admitted,
        genus2_hits: g2,
    };
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
