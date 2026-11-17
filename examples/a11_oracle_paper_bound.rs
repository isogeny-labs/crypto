//! **A11 as an algebraic decomposition oracle: a paper bound from committed
//! rows.**
//!
//! Before anyone builds an algebraic (F4) decomposition oracle for ecbench
//! (`research/a11_pipeline_pricing_20261019/FEASIBILITY.md`), this bounds
//! what the cheapest presentation measured so far, A11 (the norm chain,
//! `examples/fb_designs_slope.rs`), would cost per relation and per
//! discrete logarithm, and sets it beside what the group-law oracle that
//! ecbench already runs on the same factor base costs, measured.
//!
//! Inputs, all committed:
//! - the group-law arm: `research/ic_frontier_campaign_20261015/phase1/sessions/PKM20/records.jsonl`
//!   (`ic.pipeline`, `prime-kummer:t=8`, `rootfind-mitm:m=3`, p = 946177,
//!   r = 473453, 32 measured solves, with the matched `rho.negation` arm);
//! - the algebraic side: `research/factor_base_designs_20261015/runs/C1-rescore.jsonl`
//!   (A11 and A0 at m = 3, F4 row-reduction field multiplications per
//!   system, replayed with a matched trace).
//!
//! The bound is generous to A11 at every step:
//! - `field_ops` counts row-reduction multiplications only (symbolic
//!   preprocessing, memory, root finding and lifting are not counted);
//! - one group addition is priced at e ∈ {1, 12, 50} field multiplications
//!   (larger e makes A11 cheaper in GAE);
//! - the decomposition probability is the expected-count model
//!   P = 1 − exp(−8·C(N, 3)/r), N = 2^(t−1) abscissae on the curve, and a
//!   success yields one relation (no duplicates, no failed lifts);
//! - per discrete logarithm, only N + 1 relations are charged (no linear
//!   algebra, no setup);
//! - for t > 5 (not measured) A11's per-system cost is taken as its t = 5
//!   value, although it rose with every measured bit (an extrapolation that
//!   favours A11, marked as such).
//!
//! The F4 trace was shown identical on p₁ = 786433 and p₂ = 2013265921
//! (Amendment 2), so per-system counts measured at p₁ are used at PKM20's
//! prime. The group-law arm's unpriced native counters are priced at 0, 0.1
//! and 1 GAE each (AGENTS.md §5a); so are rho's.
//!
//! ```bash
//! cargo run --release --example a11_oracle_paper_bound > research/a11_pipeline_pricing_20261019/paper_bound.txt
//! ```
//!
//! A paper bound, not a measurement: it prices no pipeline and reports no S.

use std::collections::BTreeMap;

use serde_json::Value;

const PKM20: &str = "research/ic_frontier_campaign_20261015/phase1/sessions/PKM20/records.jsonl";
const C1: &str = "research/factor_base_designs_20261015/runs/C1-rescore.jsonl";
const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const EXCHANGE: [f64; 3] = [1.0, 12.0, 50.0];

fn rows(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("read {path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("a JSON row"))
        .collect()
}

/// Sum of every native counter whose name is in the arm's `unpriced` list
/// (`<name>_uncharged`), over all phases.
fn unpriced_total(r: &Value) -> f64 {
    let names: Vec<String> = r["cost"]["unpriced"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim_end_matches("_uncharged").to_string())
                .collect()
        })
        .unwrap_or_default();
    let mut total = 0.0;
    for ph in r["phases"].as_array().into_iter().flatten() {
        if let Some(nat) = ph["native"].as_object() {
            for (k, v) in nat {
                if names.iter().any(|n| n == k) {
                    total += v.as_f64().unwrap_or(0.0);
                }
            }
        }
    }
    total
}

fn relations(r: &Value) -> f64 {
    r["phases"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|ph| ph["native"]["relations"].as_f64())
        .sum()
}

fn stats(xs: &[f64]) -> (f64, f64, f64) {
    let n = xs.len() as f64;
    let mean = xs.iter().sum::<f64>() / n;
    let min = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let max = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, min, max)
}

fn choose3(n: f64) -> f64 {
    n * (n - 1.0) * (n - 2.0) / 6.0
}

fn main() {
    let pk = rows(PKM20);
    let measured: Vec<&Value> = pk.iter().filter(|r| r["warmup"] == false).collect();
    let r_order = measured[0]["workload"]["curve"]["r"].as_f64().expect("r");
    let p = measured[0]["workload"]["curve_spec"]["p"]
        .as_f64()
        .expect("p");

    println!("A11 as an algebraic decomposition oracle: paper bound from committed rows");
    println!("group-law arm: {PKM20}");
    println!("algebraic side: {C1}");
    println!("curve: p = {p}, r = {r_order}");
    println!();

    // ── Group-law arm and rho, measured, with the §5a band.
    println!("Measured (32 solves per arm; GAE per DLP and, for IC, per relation; unpriced natives at w GAE each)");
    let mut ic_per_dlp = BTreeMap::new();
    let mut rho_per_dlp = BTreeMap::new();
    for &w in &WEIGHTS {
        for arm in ["ic-pkm", "rho-neg"] {
            let sel: Vec<&&Value> = measured
                .iter()
                .filter(|r| r["arm"] == arm && r["outcome"]["status"] == "verified")
                .collect();
            let per_dlp: Vec<f64> = sel
                .iter()
                .map(|r| r["cost"]["total_gae"].as_f64().unwrap_or(0.0) + w * unpriced_total(r))
                .collect();
            let (m, lo, hi) = stats(&per_dlp);
            if arm == "ic-pkm" {
                let per_rel: Vec<f64> = sel
                    .iter()
                    .map(|r| {
                        (r["cost"]["total_gae"].as_f64().unwrap_or(0.0) + w * unpriced_total(r))
                            / relations(r)
                    })
                    .collect();
                let (rm, rlo, rhi) = stats(&per_rel);
                let rels: Vec<f64> = sel.iter().map(|r| relations(r)).collect();
                let (relm, _, _) = stats(&rels);
                println!(
                    "  w = {w:<3}  ic-pkm (group law, t = 8, m = 3): {} verified, per DLP mean {m:.0} [{lo:.0}, {hi:.0}] GAE, per relation mean {rm:.1} [{rlo:.1}, {rhi:.1}], relations per DLP {relm:.1}",
                    sel.len()
                );
                ic_per_dlp.insert(format!("{w}"), m);
            } else {
                println!(
                    "  w = {w:<3}  rho-neg (matched reference):        {} verified, per DLP mean {m:.0} [{lo:.0}, {hi:.0}] GAE",
                    sel.len()
                );
                rho_per_dlp.insert(format!("{w}"), m);
            }
        }
    }
    println!();

    // ── Algebraic side: F4 field multiplications per system.
    let c1 = rows(C1);
    let mut f: BTreeMap<(String, u64), Vec<f64>> = BTreeMap::new();
    for r in &c1 {
        let src = r["replay_of"].as_str().unwrap_or("");
        let first_run_or_f3 =
            src.starts_with("A11-m3") || src.starts_with("A0-m3") || src.starts_with("F3-");
        if r["m"] == 3 && r["trace_matches"] == true && first_run_or_f3 {
            let arm = r["arm"].as_str().unwrap_or("").to_string();
            f.entry((arm, r["t"].as_u64().unwrap_or(0)))
                .or_default()
                .push(r["field_ops"].as_f64().unwrap_or(0.0));
        }
    }
    println!("Algebraic oracle, lower bound (m = 3; F = mean F4 row-reduction multiplications per system, measured on p1)");
    println!("  P(t) = 1 − exp(−8·C(N,3)/r), N = 2^(t−1); per relation = F/(e·P) GAE; per DLP = (N + 1)·F/(e·P)");
    println!("  arm  t  N    F (field mults)   P(t)        per relation GAE at e = 1 / 12 / 50          per DLP GAE at e = 50");
    let mut best = f64::INFINITY;
    let mut best_at = String::new();
    for arm in ["A11", "A0"] {
        let mut last_f = 0.0;
        for t in 2u64..=12 {
            let measured_f = f
                .get(&(arm.to_string(), t))
                .map(|v| v.iter().sum::<f64>() / v.len() as f64);
            let (ff, tag) = match measured_f {
                Some(x) => {
                    last_f = x;
                    (x, "measured")
                }
                None if arm == "A11" && last_f > 0.0 => {
                    (last_f, "extrapolated (held at the last measured t)")
                }
                None => continue,
            };
            let n = (1u64 << (t - 1)) as f64;
            let e_count = 8.0 * choose3(n) / r_order;
            let pr = 1.0 - (-e_count).exp();
            if pr <= 0.0 {
                continue;
            }
            let per_rel: Vec<f64> = EXCHANGE.iter().map(|e| ff / (e * pr)).collect();
            let per_dlp = (n + 1.0) * ff / (50.0 * pr);
            if per_dlp < best {
                best = per_dlp;
                best_at = format!("{arm} t = {t} ({tag})");
            }
            println!(
                "  {arm:<4} {t:<2} {n:<4} {ff:<17.4e} {pr:<11.4e} {:.3e} / {:.3e} / {:.3e}          {per_dlp:.3e}   {tag}",
                per_rel[0], per_rel[1], per_rel[2]
            );
        }
    }
    println!();
    println!("Comparison per DLP (algebraic lower bound at e = 50, the most generous exchange; measured arms at each w)");
    println!("  cheapest algebraic lower bound: {best:.3e} GAE at {best_at}");
    for &w in &WEIGHTS {
        let ic = ic_per_dlp[&format!("{w}")];
        let rho = rho_per_dlp[&format!("{w}")];
        println!(
            "  w = {w:<3}  group-law IC {ic:.0} GAE, rho {rho:.0} GAE;  algebraic bound / group-law IC = {:.3e}, / rho = {:.3e}",
            best / ic,
            best / rho
        );
    }
    let holds = WEIGHTS.iter().all(|&w| best > ic_per_dlp[&format!("{w}")]);
    println!();
    println!(
        "Verdict (holds across the whole band): {}",
        if holds {
            "even the most generous lower bound for an A11 algebraic oracle costs more per DLP than the group-law oracle ecbench already runs, at every w in {0, 0.1, 1}"
        } else {
            "the algebraic lower bound is not above the group-law arm at every w: building the oracle could be worth measuring"
        }
    );
}
