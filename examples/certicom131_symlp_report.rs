//! Lay out the recorded `certicom131_symlp_measure` runs into the report
//! tables, the plotting data, and the derived cost-model rows.  Reads the
//! recorded values; the only computation is the cost model, which is
//! labelled derived everywhere it appears.
//!
//! ```sh
//! cargo run --release --example certicom131_symlp_report -- \
//!     research/certicom131_symlp_20261025
//! ```
//!
//! Outputs under the study directory: `tables/measured.md`,
//! `tables/costmodel.md`, `figures/plotdata.json` (for the matplotlib
//! step in the render venv).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

fn read_jsonl(dir: &Path, name: &str) -> Vec<Value> {
    let p = dir.join("runs").join(name);
    fs::read_to_string(&p)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

fn f(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0)
}
fn u(v: &Value, k: &str) -> u64 {
    v.get(k).and_then(|x| x.as_u64()).unwrap_or(0)
}
fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("")
}

fn main() {
    let dir = PathBuf::from(std::env::args().nth(1).expect("study directory"));
    fs::create_dir_all(dir.join("tables")).unwrap();
    fs::create_dir_all(dir.join("figures")).unwrap();

    let rate = read_jsonl(&dir, "rate.jsonl");
    let rho = read_jsonl(&dir, "rho.jsonl");
    let yld = read_jsonl(&dir, "yield.jsonl");

    // Rho reference step rate per family.
    let mut rho_rate: BTreeMap<String, f64> = BTreeMap::new();
    for r in &rho {
        rho_rate.insert(s(r, "family").into(), f(r, "steps_per_sec"));
    }

    // ── Measured table ────────────────────────────────────────────────
    let mut md = String::new();
    md.push_str("# Measured values\n\n");
    md.push_str(
        "Single thread, one field implementation per family, fixed seed \
20261025. Step rates are this host's; the index-calculus decomposition step \
and the Pollard-rho adding-walk step use the same field code in the same \
process, so their rates compare directly.\n\n",
    );

    md.push_str("## Reference: batched adding-walk (rho) step rate\n\n");
    md.push_str("| family | n | lanes | Msteps/s |\n|---|--:|--:|--:|\n");
    for r in &rho {
        let _ = writeln!(
            md,
            "| {} | {} | {} | {:.2} |",
            s(r, "family"),
            u(r, "n"),
            u(r, "lanes"),
            f(r, "steps_per_sec") / 1e6
        );
    }

    md.push_str("\n## Oracle step rate: symmetric vs per-point baseline\n\n");
    md.push_str(
        "`sym/base` is the per-step speed-up of the symmetric-coordinate \
sweep (enumerate `e_1 = x_1 + x_2`, batched inversion) over the per-point \
baseline (enumerate `x_1`, lift each with its own inversion). `IC/rho step` \
is the symmetric oracle's step rate over the rho reference for the same \
family.\n\n",
    );
    md.push_str("| family | curve | n | base Msteps/s | sym Msteps/s | sym/base | IC/rho step |\n");
    md.push_str("|---|---|--:|--:|--:|--:|--:|\n");
    // Pair base/sym rows by (family, curve, n).
    let mut by_key: BTreeMap<(String, String, u64), (f64, f64)> = BTreeMap::new();
    for r in &rate {
        let key = (s(r, "family").into(), s(r, "variant").into(), u(r, "n"));
        let e = by_key.entry(key).or_insert((0.0, 0.0));
        if r.get("symmetric")
            .and_then(|x| x.as_bool())
            .unwrap_or(false)
        {
            e.1 = f(r, "steps_per_sec");
        } else {
            e.0 = f(r, "steps_per_sec");
        }
    }
    let mut plot_rate = Vec::new();
    for ((fam, curve, n), (base, sym)) in &by_key {
        let rr = *rho_rate.get(fam).unwrap_or(&f64::NAN);
        let _ = writeln!(
            md,
            "| {fam} | {curve} | {n} | {:.2} | {:.2} | {:.1}× | {:.2} |",
            base / 1e6,
            sym / 1e6,
            sym / base,
            sym / rr
        );
        plot_rate.push(json!({
            "family": fam, "curve": curve, "n": n,
            "base": base, "sym": sym, "rho": rr,
        }));
    }

    md.push_str("\n## Full-subspace decomposition yield (binary calibration)\n\n");
    md.push_str(
        "A full sweep of `e_1 ∈ V_{l'}` over random targets, with the \
0/1/2-large-prime split against the factor base `V_l`. `direct` are relations \
with both abscissae in `V_l` (zero large primes); they are the only ones a \
pipeline without a large-prime graph can use.\n\n",
    );
    md.push_str("| n | l | l' | decomps/sweep | direct (0 lp) | 1 lp | 2 lp | direct fraction |\n");
    md.push_str("|--:|--:|--:|--:|--:|--:|--:|--:|\n");
    let mut plot_yield = Vec::new();
    for y in &yld {
        let (d0, d1, d2) = (u(y, "lp0"), u(y, "lp1"), u(y, "lp2"));
        let tot = (d0 + d1 + d2).max(1);
        let _ = writeln!(
            md,
            "| {} | {} | {} | {:.0} | {} | {} | {} | {:.1e} |",
            u(y, "n"),
            u(y, "l"),
            u(y, "lp_dim"),
            f(y, "decomps_per_sweep"),
            d0,
            d1,
            d2,
            d0 as f64 / tot as f64
        );
        plot_yield.push(json!({
            "n": u(y,"n"), "l": u(y,"l"), "lp": u(y,"lp_dim"),
            "d0": d0, "d1": d1, "d2": d2,
            "prob": f(y,"decomp_prob_per_step"),
        }));
    }
    fs::write(dir.join("tables/measured.md"), &md).unwrap();

    // ── Derived cost model ───────────────────────────────────────────
    // m=2 subspace index calculus at the balancing factor-base dimension
    // d = (n+1)/3: a solve needs about 2^{d-1} relations and each trial
    // succeeds with probability about 2^{2d-n-2}, so a solve takes about
    // 2^{d-1} / 2^{2d-n-2} = 2^{n-d+1} trials, smallest at the largest
    // determined d = (n+1)/3, i.e. about 2^{2n/3} decomposition steps
    // (the method's floor; PR #2493). Pollard rho with the negation map
    // needs about sqrt(pi r / 4) steps. The distance is reported in
    // decomposition/walk steps (both measured to cost about the same on
    // this host) and does not fold in the linear-algebra floor, which the
    // program gates (docs/ic/CERTICOM131_IC_PROGRAM.md §2) show is itself
    // above rho for m <= 3.
    let mut cm = String::new();
    cm.push_str("# Derived cost model (labelled derived)\n\n");
    cm.push_str(
        "All rows are computed, not measured. The step counts use the \
balancing factor-base dimension `l = round(n/3)` for the subspace `m = 2` \
method; the per-step cost is taken from the measured symmetric-oracle rate \
and the measured rho rate on this host. `r` is the prime-subgroup order \
(`2^{n-1}` used for the calibration curves, which are not point-counted).\n\n",
    );
    cm.push_str(
        "steps(IC, m=2) ≈ 2^{2n/3}  (2^{d-1} relations ÷ 2^{2d-n-2} success, d = (n+1)/3)\n\n",
    );
    cm.push_str("steps(rho) ≈ sqrt(π·r/4),  r ≈ 2^{n}·(cofactor factor)\n\n");
    cm.push_str("distance(steps) = steps(IC) / steps(rho) ≈ 2^{n/6} · constant\n\n");
    cm.push_str("| curve | n | log2 steps(IC m=2) | log2 steps(rho) | log2 distance (steps) | this matches family optimum Θ(r^{1/6})·rho? |\n");
    cm.push_str("|---|--:|--:|--:|--:|---|\n");
    let rows = [
        ("ECC2-131", 131u32, 130.0f64), // r ≈ 2^130 (cofactor 2)
        ("ECCp-131", 131, 131.0),       // r ≈ 2^131 (cofactor 1)
    ];
    let mut plot_dist = Vec::new();
    for (name, n, log2r) in rows {
        let ic = 2.0 * (n as f64) / 3.0; // 2^{2n/3}, the m=2 solve floor
        let rho_steps = 0.5 * (std::f64::consts::PI / 4.0).log2() + 0.5 * log2r;
        let dist = ic - rho_steps;
        let _ = writeln!(
            cm,
            "| {name} | {n} | {ic:.1} | {rho_steps:.1} | {dist:.1} | the step gap 2^{{{dist:.0}}} is far above the r^{{1/6}} ≈ 2^{{{:.0}}} family optimum |",
            log2r / 6.0
        );
        plot_dist.push(json!({"curve": name, "n": n, "ic": ic, "rho": rho_steps, "dist": dist}));
    }
    cm.push_str(
        "\nThe `m = 2` exponent `2n/3 ≈ 0.667 n` agrees with the merged \
algebraic ladder's measured trial slope 0.647 [0.639, 0.655] (PR #2493); the \
enumeration and the linear solve share the `2^{2n/3}` optimum. What this study \
adds over that baseline is measured and constant-factor: the symmetric \
decomposition step costs about the same as a rho step on this host (so the \
distance is the step-count gap, not a per-step penalty), and the \
double-large-prime graph recovers full relations from the 2-lp decompositions \
that dominate the measured yield. Neither changes the `2n/3` vs `n/2` exponent \
gap, so the distance to rho at n = 131 stays about `2^{n/6} ≈ 2^{22}` in steps \
before the linear-algebra floor, which the program gates place above rho for \
m ≤ 3 on both curves.\n",
    );
    fs::write(dir.join("tables/costmodel.md"), &cm).unwrap();

    // ── Plot data ────────────────────────────────────────────────────
    let plotdata = json!({
        "rate": plot_rate,
        "yield": plot_yield,
        "distance": plot_dist,
        "rho_rate": rho_rate,
    });
    fs::write(
        dir.join("figures/plotdata.json"),
        serde_json::to_string_pretty(&plotdata).unwrap(),
    )
    .unwrap();

    eprintln!(
        "wrote tables and figures/plotdata.json under {}",
        dir.display()
    );
}
