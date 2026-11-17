//! The gates of `docs/ic/CERTICOM131_IC_PROGRAM.md`: for each index-calculus
//! family that applies to ECC2-131 or ECCp-131, the largest decomposition
//! solving degree `D` (or, for small-abscissa bases, the smallest lattice
//! reach `δ`) at which the family's optimised end-to-end cost could reach
//! the matched rho.  A cost model, not a measurement: every input is a
//! formula or a constant printed with the output.
//!
//! ```sh
//! cargo run --release --example certicom131_ic_gates -- --json gates.json
//! ```
//!
//! **Binary (FPPR / Diem vector-space base).**  Base: the points with
//! `x ∈ V`, `dim_{F₂} V = ℓ`, about `2^{ℓ−1}` columns modulo negation.  A
//! random combination decomposes into `m` base points with probability
//! `p_rel = min(1, 2^{mℓ−n}/m!)`.  Total, in group-addition equivalents (GAE):
//!
//! ```text
//! cost(ℓ) = 2^{ℓ−1}/p_rel · (1 + κ_dec · T_dec(D))  +  κ_LA · m · 2^{2(ℓ−1)}
//! T_dec(D) = M(D)^ω / 64,   M(D) = Σ_{k≤D} C(mℓ, k)
//! ```
//!
//! `T_dec` is the 64-bit word cost of eliminating the degree-`D` Macaulay
//! matrix of the Weil-restricted `S_{m+1}` system in its `mℓ` Boolean
//! unknowns (square-free monomials, the field equations built in).  The
//! one group addition per attempt (forming the random combination) is a
//! floor no solver removes.  `D = 0` stands for a free decomposition.
//!
//! **Prime, PKM tower** (Petit–Kosters–Messeng).  Base `|V| = 2^t`, `N = m·t`
//! tower unknowns over `F_p`, square-free monomial basis (PKM note §2.4):
//!
//! ```text
//! cost(t) = 2^t · m!·r/2^{mt} · (1 + κ_dec · M_N(D)^ω)  +  κ_LA · m · 2^{2t}
//! ```
//!
//! with `M_N(D) = Σ_{k≤D} C(N, k)` and `F_p` operations priced at `κ_dec`.
//!
//! **Prime, small abscissae** (Semaev + Coppersmith).  Base `x ≤ B = p^δ`;
//! with an `O(1)` lattice oracle the total is
//! `(m!/2^m)·p/B^{m−1} + κ_LA·m·B²` (`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`
//! §2.4); the gate is the least `δ` that reaches rho, against the
//! Jochemsz–May reach `δ_max = 1/(m(m+1)·2^{m−2})` (§2.2).
//!
//! Every gate is printed at two price points, the band over which the
//! verdicts must hold (AGENTS.md §5a): `generous` (`κ_dec = 2^{-10}`,
//! `κ_LA = 2^{-4}`, `ω = 2`) and `plain` (`κ_dec = κ_LA = 1`, `ω = 2.81`).

use crypto_lib::cryptanalysis::koblitz_bench::semi_regular_dreg;
use num_bigint::BigUint;
use serde_json::json;

/// log₂ of a big integer given in hex.
fn log2_hex(h: &str) -> f64 {
    let v = BigUint::parse_bytes(h.as_bytes(), 16).expect("hex");
    let bits = v.bits();
    let shift = bits.saturating_sub(64);
    let top: u64 = (&v >> shift).try_into().expect("64 bits");
    (top as f64).log2() + shift as f64
}

/// log₂ (2^a + 2^b).
fn lse(a: f64, b: f64) -> f64 {
    let (hi, lo) = if a > b { (a, b) } else { (b, a) };
    if lo == f64::NEG_INFINITY {
        return hi;
    }
    hi + (1.0 + (lo - hi).exp2()).log2()
}

/// log₂ C(n, k).
fn log2_binom(n: u64, k: u64) -> f64 {
    if k > n {
        return f64::NEG_INFINITY;
    }
    let k = k.min(n - k);
    (0..k)
        .map(|i| ((n - i) as f64 / (i + 1) as f64).log2())
        .sum()
}

/// log₂ Σ_{k≤D} C(n, k).
fn log2_mono(n: u64, d: u32) -> f64 {
    (0..=u64::from(d).min(n)).fold(f64::NEG_INFINITY, |acc, k| lse(acc, log2_binom(n, k)))
}

fn log2_fact(m: u32) -> f64 {
    (2..=m).map(|i| (i as f64).log2()).sum()
}

#[derive(Clone, Copy)]
struct Price {
    name: &'static str,
    dec: f64, // log₂ κ_dec
    la: f64,  // log₂ κ_LA
    omega: f64,
}

const PRICES: [Price; 2] = [
    Price {
        name: "generous",
        dec: -10.0,
        la: -4.0,
        omega: 2.0,
    },
    Price {
        name: "plain",
        dec: 0.0,
        la: 0.0,
        omega: 2.81,
    },
];

/// Binary FPPR: (log₂ min cost, argmin ℓ) at solving degree `d`
/// (`None` = free decomposition).
fn binary_best(n: u32, m: u32, d: Option<u32>, pr: Price, chain: bool) -> (f64, u32) {
    let mut best = (f64::INFINITY, 0);
    for ell in 2..=n {
        let lrel = ((m * ell) as f64 - n as f64 - log2_fact(m)).min(0.0);
        let attempts = (ell as f64 - 1.0) - lrel;
        let per = match d {
            None => 0.0,
            Some(d) => {
                let width = if chain {
                    (m - 2) * n + m * ell
                } else {
                    m * ell
                };
                lse(
                    0.0,
                    pr.dec + pr.omega * log2_mono(u64::from(width), d) - 6.0,
                )
            }
        };
        let la = pr.la + (m as f64).log2() + 2.0 * (ell as f64 - 1.0);
        let total = lse(attempts + per, la);
        if total < best.0 {
            best = (total, ell);
        }
    }
    best
}

/// Prime PKM tower: (log₂ min cost, argmin t) at degree `d`.
fn pkm_best(log2_r: f64, m: u32, d: Option<u32>, pr: Price) -> (f64, u32) {
    let mut best = (f64::INFINITY, 0);
    for t in 1..=140u32 {
        let lrel = ((m * t) as f64 - log2_r - log2_fact(m)).min(0.0);
        let attempts = t as f64 - lrel;
        let per = match d {
            None => 0.0,
            Some(d) => lse(0.0, pr.dec + pr.omega * log2_mono(u64::from(m * t), d)),
        };
        let la = pr.la + (m as f64).log2() + 2.0 * t as f64;
        let total = lse(attempts + per, la);
        if total < best.0 {
            best = (total, t);
        }
    }
    best
}

/// Largest `D` whose optimum reaches `rho`; `None` if even `D = 1` fails.
fn gate<F: Fn(Option<u32>) -> (f64, u32)>(rho: f64, f: F) -> Option<u32> {
    (1..=60u32).take_while(|&d| f(Some(d)).0 <= rho).last()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_out = args
        .iter()
        .position(|a| a == "--json")
        .and_then(|i| args.get(i + 1))
        .cloned();
    // ECC2-131: n = 131, h = 2, r from the Certicom record; ECCp-131: h = 1.
    let r2 = log2_hex("0400000000000000026ABB991FE311FE83");
    let rp = log2_hex("48E1D43F293469E317F7ED728F6B8E6F1");
    let pp = log2_hex("48E1D43F293469E33194C43186B3ABC0B");
    let rho = |lr: f64| 0.5 * (lr + (std::f64::consts::PI / 4.0).log2());
    let (rho2, rhop) = (rho(r2), rho(rp));
    println!("ECC2-131: log2 r = {r2:.3}, rho (negation map) = 2^{rho2:.2} GAE");
    println!("ECCp-131: log2 r = {rp:.3}, log2 p = {pp:.3}, rho = 2^{rhop:.2} GAE");

    let mut out = json!({
        "ecc2_131": {"log2_r": r2, "log2_rho": rho2, "rows": []},
        "eccp_131": {"log2_r": rp, "log2_p": pp, "log2_rho": rhop, "pkm": [], "small_x": []},
        "prices": PRICES.iter().map(|p| json!({"name": p.name, "log2_kappa_dec": p.dec, "log2_kappa_la": p.la, "omega": p.omega})).collect::<Vec<_>>(),
    });

    println!("\n## ECC2-131, FPPR vector-space base (n = 131, mℓ Boolean unknowns)\n");
    println!("| m | price | free-decomposition floor | ℓ* | gate D* | cost at D* | cost at D = 4 | cost at D = 5 |");
    println!("|--:|:--|--:|--:|--:|--:|--:|--:|");
    for (m, pr, chain) in (2..=6u32).flat_map(|m| {
        PRICES
            .into_iter()
            .flat_map(move |pr| [false, true].into_iter().map(move |c| (m, pr, c)))
    }) {
        {
            let (floor, ell0) = binary_best(131, m, None, pr, chain);
            let g = gate(rho2, |d| binary_best(131, m, d, pr, chain));
            let at = |d: u32| binary_best(131, m, Some(d), pr, chain);
            let (c5, l5) = at(5);
            let (cpq, lpq) = at(4);
            let gd = g.map(|d| (d, at(d)));
            println!(
                "| {m} | {}{} | 2^{floor:.1} (ℓ={ell0}) | {} | {} | {} | 2^{cpq:.1} (ℓ={lpq}) | 2^{c5:.1} (ℓ={l5}) |",
                pr.name,
                if chain { ", chained" } else { ", mℓ unknowns" },
                gd.map_or("-".into(), |(_, (_, l))| l.to_string()),
                g.map_or("none".into(), |d| d.to_string()),
                gd.map_or("-".into(), |(_, (c, _))| format!("2^{c:.1}")),
            );
            out["ecc2_131"]["rows"].as_array_mut().unwrap().push(json!({
                "m": m, "price": pr.name, "chained": chain, "log2_floor": floor, "ell_floor": ell0,
                "gate_d": g, "log2_cost_d5": c5, "ell_d5": l5,
                "log2_cost_d4": cpq, "ell_d4": lpq,
                "log2_cost_at_gate": gd.map(|(_, (c, _))| c), "ell_at_gate": gd.map(|(_, (_, l))| l),
            }));
        }
    }

    println!("\n## ECCp-131, PKM tower (N = m·t unknowns over F_p)\n");
    println!("| m | price | free-decomposition floor | gate D* | t* | cost at D = 7 (measured plateau) |");
    println!("|--:|:--|--:|--:|--:|--:|");
    for m in 2..=6u32 {
        for pr in PRICES {
            let (floor, t0) = pkm_best(rp, m, None, pr);
            let g = gate(rhop, |d| pkm_best(rp, m, d, pr));
            let (c7, t7) = pkm_best(rp, m, Some(7), pr);
            let tg = g.map(|d| pkm_best(rp, m, Some(d), pr).1);
            println!(
                "| {m} | {} | 2^{floor:.1} (t={t0}) | {} | {} | 2^{c7:.1} (t={t7}) |",
                pr.name,
                g.map_or("none".into(), |d| d.to_string()),
                tg.map_or("-".into(), |t| t.to_string()),
            );
            out["eccp_131"]["pkm"].as_array_mut().unwrap().push(json!({
                "m": m, "price": pr.name, "log2_floor": floor, "t_floor": t0,
                "gate_d": g, "t_at_gate": tg, "log2_cost_d7": c7, "t_d7": t7,
            }));
        }
    }

    println!("\n## ECCp-131, small-abscissa base with an O(1) lattice oracle\n");
    println!("| m | price | δ needed (relations ≤ rho) | δ allowed (LA ≤ rho) | lattice reach δ_max | shortfall |");
    println!("|--:|:--|--:|--:|--:|--:|");
    for m in 2..=6u32 {
        for pr in PRICES {
            // (m!/2^m)·p / p^{δ(m−1)} ≤ rho  and  κ_LA·m·p^{2δ} ≤ rho.
            let need = (pp + log2_fact(m) - m as f64 - rhop) / (pp * (m as f64 - 1.0));
            let allow = (rhop - pr.la - (m as f64).log2()) / (2.0 * pp);
            let reach = 1.0 / ((m * (m + 1)) as f64 * 2f64.powi(m as i32 - 2));
            println!(
                "| {m} | {} | {need:.4} | {allow:.4} | {reach:.4} | {:.1}× |",
                pr.name,
                need / reach
            );
            out["eccp_131"]["small_x"]
                .as_array_mut()
                .unwrap()
                .push(json!({
                    "m": m, "price": pr.name, "delta_needed": need, "delta_la_cap": allow,
                    "delta_lattice_reach": reach, "shortfall": need / reach,
                    "feasible_window": need <= allow,
                }));
        }
    }

    println!("\n## Semi-regular reference degree of the chained system (BFSY formula, a formula and not a measurement)\n");
    println!("The degree of regularity of `(m−2)n` cubic and `n` quadratic equations in `(m−2)n + mℓ` Boolean unknowns,");
    println!("`dreg_ladder`'s reference; Falsifier A's measured degrees equal it in every cell measured.\n");
    println!("| m | n | ℓ | unknowns | semi-regular D |");
    println!("|--:|--:|--:|--:|--:|");
    let mut sr = Vec::new();
    for m in [3u32, 4] {
        for n in [5u32, 7, 11, 13, 17, 19, 23, 31, 43, 63, 131] {
            let ell = (n as f64 / m as f64).round() as u32;
            let nv = ((m - 2) * n + m * ell) as usize;
            let degs: Vec<u32> = std::iter::repeat_n(3, ((m - 2) * n) as usize)
                .chain(std::iter::repeat_n(2, n as usize))
                .collect();
            let d = semi_regular_dreg(nv, &degs);
            println!(
                "| {m} | {n} | {ell} | {nv} | {} |",
                d.map_or("-".into(), |x| x.to_string())
            );
            sr.push(json!({"m": m, "n": n, "ell": ell, "unknowns": nv, "semi_regular_d": d}));
        }
    }
    out["semi_regular"] = json!(sr);
    if let Some(p) = json_out {
        std::fs::write(&p, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
        eprintln!("wrote {p}");
    }
}
