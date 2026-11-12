//! `replay_check`: an independent recomputation of the headline numbers of
//! `research/chained_prime_transfer_20261111/` from the raw JSON, plus
//! re-verification of the solves with code that does not go through the
//! producer's bench binaries.
//!
//! ```text
//! replay_check diff    <committed.json> <fresh.json>
//! replay_check results <results.json>
//! replay_check census  <atlas_census.json>
//! replay_check sweep   <sweep.json>
//! replay_check sweep-indep <sweep.json> [maxc]   # recompute from scratch
//! replay_check tower   <tower.json>
//! replay_check cross-census                      # non-vacuity of phi2* phi1^* = 0
//! replay_check certs   <results.json>            # rho/IC re-solve + certificates
//! ```
//!
//! Every command prints one line per check, `PASS` or `FAIL`, and exits
//! non-zero when any check fails. Nothing here edits committed evidence.
//! Result sentences it prints are unverified assertions `[pcc:unverified]`.

use std::process::ExitCode;

use serde_json::Value;

use crypto_lib::cryptanalysis::chained_transfer::bielliptic::{Bielliptic, Quotient};
use crypto_lib::cryptanalysis::chained_transfer::chain::{Chain, Elt, Hop};
use crypto_lib::cryptanalysis::chained_transfer::ec::{Curve, Pt, VeluIsogeny};
use crypto_lib::cryptanalysis::chained_transfer::measure::{
    genus2_index_calculus, jac_instance, EcGroup, JacGroup,
};
use crypto_lib::cryptanalysis::genus2_fp::Genus2Curve;
use crypto_lib::cryptanalysis::ic_boundary::rho_reference;

// ---------------------------------------------------------------- reporting

struct Rep {
    fails: u32,
    passes: u32,
}

impl Rep {
    fn new() -> Rep {
        Rep {
            fails: 0,
            passes: 0,
        }
    }
    fn check(&mut self, name: &str, ok: bool, detail: &str) {
        if ok {
            self.passes += 1;
        } else {
            self.fails += 1;
        }
        println!("{} {name}: {detail}", if ok { "PASS" } else { "FAIL" });
    }
    fn finish(self) -> ExitCode {
        println!("summary: {} PASS, {} FAIL", self.passes, self.fails);
        if self.fails == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}

fn load(path: &str) -> Value {
    let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    serde_json::from_str(&s).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

fn gu(v: &Value, k: &str) -> u64 {
    v[k].as_u64()
        .unwrap_or_else(|| panic!("field {k} is not u64: {}", v[k]))
}

fn gf(v: &Value, k: &str) -> f64 {
    v[k].as_f64()
        .unwrap_or_else(|| panic!("field {k} is not f64: {}", v[k]))
}

fn gb(v: &Value, k: &str) -> bool {
    v[k].as_bool()
        .unwrap_or_else(|| panic!("field {k} is not bool: {}", v[k]))
}

/// Ordinary least-squares slope of `ys` against `xs`.
fn slope(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    sxy / sxx
}

fn arr(v: &Value) -> &Vec<Value> {
    v.as_array().expect("array")
}

// -------------------------------------------------------------------- diff

fn diff_values(path: &str, a: &Value, b: &Value, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let mut keys: Vec<&String> = x.keys().chain(y.keys()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                let p = format!("{path}.{k}");
                match (x.get(k), y.get(k)) {
                    (Some(u), Some(v)) => diff_values(&p, u, v, out),
                    (Some(u), None) => out.push(format!("{p}: only in committed ({u})")),
                    (None, Some(v)) => out.push(format!("{p}: only in fresh ({v})")),
                    (None, None) => {}
                }
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            if x.len() != y.len() {
                out.push(format!("{path}: length {} != {}", x.len(), y.len()));
            }
            for i in 0..x.len().min(y.len()) {
                diff_values(&format!("{path}[{i}]"), &x[i], &y[i], out);
            }
        }
        _ => {
            if a != b {
                out.push(format!("{path}: committed {a} != fresh {b}"));
            }
        }
    }
}

fn cmd_diff(a: &str, b: &str) -> ExitCode {
    let (va, vb) = (load(a), load(b));
    let mut d = vec![];
    diff_values("$", &va, &vb, &mut d);
    let mut r = Rep::new();
    r.check(
        "field-level identity",
        d.is_empty(),
        &format!("{} differing leaf fields between {a} and {b}", d.len()),
    );
    for l in d.iter().take(40) {
        println!("  diff {l}");
    }
    r.finish()
}

// ----------------------------------------------------------------- results

fn cmd_results(path: &str) -> ExitCode {
    let v = load(path);
    let rows = arr(&v);
    let mut r = Rep::new();
    r.check(
        "row count",
        rows.len() == 4,
        &format!("{} rows", rows.len()),
    );
    let want = ["9.8", "26.2", "36.0", "42.9"];
    let mut lp = vec![];
    let mut lic = vec![];
    let mut lrho = vec![];
    for (i, row) in rows.iter().enumerate() {
        let p = gu(row, "p");
        let rr = gu(row, "r");
        let ic = gu(row, "ic_jac_group_ops");
        let rho = gu(row, "rho_src_ops");
        let ratio = ic as f64 / rho as f64;
        let s = format!("{ratio:.1}");
        r.check(
            &format!("p={p} IC/rho"),
            s == want[i],
            &format!("{ic}/{rho} = {ratio:.4} -> {s} (claimed {})", want[i]),
        );
        let sr = (rr as f64).sqrt();
        println!(
            "  info p={p} r={rr}: S=ops/sqrt(r): rho_src {:.1}, rho_isog {:.1}, rho_jac {:.1}, IC {:.1} (asymptotic rho S about 1.3)",
            rho as f64 / sr,
            gu(row, "rho_isog_ops") as f64 / sr,
            gu(row, "rho_jac_ops") as f64 / sr,
            ic as f64 / sr
        );
        // structure
        let e1 = gu(row, "e1_order");
        let e2 = gu(row, "e2_order");
        r.check(
            &format!("p={p} jac_order = e1*e2"),
            gu(row, "jac_order") == e1 * e2,
            &format!("{} vs {}", gu(row, "jac_order"), e1 * e2),
        );
        r.check(
            &format!("p={p} r || #J (r | e1, r^2 not | e1, r not | e2)"),
            e1 % rr == 0 && (e1 / rr) % rr != 0 && e2 % rr != 0,
            &format!("e1={e1} e2={e2} r={rr}"),
        );
        r.check(
            &format!("p={p} relations = fb + 8, trials >= relations"),
            gu(row, "ic_jac_relations") == gu(row, "ic_jac_fb") + 8
                && gu(row, "ic_jac_trials") >= gu(row, "ic_jac_relations"),
            &format!(
                "fb {} rel {} trials {} (smooth fraction {:.3})",
                gu(row, "ic_jac_fb"),
                gu(row, "ic_jac_relations"),
                gu(row, "ic_jac_trials"),
                gu(row, "ic_jac_relations") as f64 / gu(row, "ic_jac_trials") as f64
            ),
        );
        r.check(
            &format!("p={p} recovered/chain flags"),
            gb(row, "ic_recovered_ok") && gb(row, "chain_verified"),
            "ic_recovered_ok and chain_verified both true in the raw JSON",
        );
        lp.push((p as f64).ln());
        lic.push((ic as f64).ln());
        lrho.push((rho as f64).ln());
    }
    let ratios: Vec<f64> = rows
        .iter()
        .map(|x| gu(x, "ic_jac_group_ops") as f64 / gu(x, "rho_src_ops") as f64)
        .collect();
    r.check(
        "ratio grows with p",
        ratios.windows(2).all(|w| w[1] > w[0]),
        &format!("{ratios:.2?}"),
    );
    let s = slope(&lp, &lic);
    r.check(
        "LSQ slope ln(IC ops) vs ln p",
        format!("{s:.2}") == "1.01",
        &format!("{s:.6} (claimed 1.01)"),
    );
    let sr = slope(&lp, &lrho);
    println!("  info LSQ slope ln(rho_src ops) vs ln p = {sr:.3} (rho here is set-up dominated)");
    r.finish()
}

// ------------------------------------------------------------------ census

fn cmd_census(path: &str) -> ExitCode {
    let v = load(path);
    let cs = arr(&v["censuses"]);
    let mut r = Rep::new();
    r.check("four primes", cs.len() == 4, &format!("{}", cs.len()));
    // ATLAS.md table, transcribed.
    let same_ord = [7u64, 4, 7, 3];
    let e1_ss = [4u64, 2, 3, 2];
    for (i, c) in cs.iter().enumerate() {
        let p = gu(c, "p");
        r.check(
            &format!("p={p} curves = 300"),
            gu(c, "curves") == 300,
            &format!("{}", gu(c, "curves")),
        );
        r.check(
            &format!("p={p} cross-map-zero 300/300"),
            gu(c, "cross_zero_all") == 300,
            &format!("{}/{}", gu(c, "cross_zero_all"), gu(c, "curves")),
        );
        r.check(
            &format!("p={p} factsDiff = 0"),
            gu(c, "same_order_facts_differ") == 0,
            &format!("{}", gu(c, "same_order_facts_differ")),
        );
        r.check(
            &format!("p={p} freeEasy = 0"),
            gu(c, "free_easier_node") == 0,
            &format!("{}", gu(c, "free_easier_node")),
        );
        r.check(
            &format!("p={p} ATLAS.md table row (same order, E1 anomalous, E1 ss)"),
            gu(c, "factors_same_order") == same_ord[i]
                && gu(c, "e1_anomalous") == 0
                && gu(c, "e1_supersingular") == e1_ss[i],
            &format!(
                "same {} anom {} ss {}",
                gu(c, "factors_same_order"),
                gu(c, "e1_anomalous"),
                gu(c, "e1_supersingular")
            ),
        );
    }
    let sc = &v["smart_control"];
    r.check(
        "Smart positive control",
        sc[0].as_u64() == Some(43) && sc[1].as_bool() == Some(true),
        &format!("{sc}"),
    );
    r.finish()
}

// ------------------------------------------------------------------- sweep

fn cmd_sweep(path: &str) -> ExitCode {
    let v = load(path);
    let reps = arr(&v);
    let mut r = Rep::new();
    let sum = |k: &str| -> u64 { reps.iter().map(|x| gu(x, k)).sum() };
    let n_zero = reps.iter().filter(|x| gu(x, "covers") == 0).count();
    r.check(
        "24 reports",
        reps.len() == 24,
        &format!(
            "{} reports ({} of them with covers = 0)",
            reps.len(),
            n_zero
        ),
    );
    r.check(
        "5641 covers",
        sum("covers") == 5641,
        &format!("{}", sum("covers")),
    );
    let maxd = reps
        .iter()
        .map(|x| gu(x, "distinct_complement_orders"))
        .max()
        .unwrap();
    r.check(
        "max distinct complement orders 141",
        maxd == 141,
        &format!("{maxd}"),
    );
    r.check(
        "3479 smaller-prime complements",
        sum("complement_smaller_prime") == 3479,
        &format!("{}", sum("complement_smaller_prime")),
    );
    r.check(
        "85 supersingular complements (ATLAS.md prose)",
        sum("complement_supersingular") == 85,
        &format!("{}", sum("complement_supersingular")),
    );
    r.check(
        "easier_and_reachable total 0",
        sum("easier_and_reachable") == 0,
        &format!("{}", sum("easier_and_reachable")),
    );
    r.check(
        "cover_never_carries_log true everywhere",
        reps.iter().all(|x| gb(x, "cover_never_carries_log")),
        &format!(
            "{} of {} true ({} of those have covers = 0, so true vacuously)",
            reps.iter()
                .filter(|x| gb(x, "cover_never_carries_log"))
                .count(),
            reps.len(),
            n_zero
        ),
    );
    // The hand-drawn cover-spectrum.svg claims, for F_1009 / order 990.
    let e = reps
        .iter()
        .find(|x| gu(x, "p") == 1009 && gu(x, "target_order") == 990)
        .expect("p=1009 order 990 report");
    println!(
        "  info sweep.json (maxc 600) p=1009 order 990: covers {} distinct {} smaller_prime {} isogenous {} (cover-spectrum.svg: 504 covers, 63 orders, 470 smoother, 22 isogenous)",
        gu(e, "covers"),
        gu(e, "distinct_complement_orders"),
        gu(e, "complement_smaller_prime"),
        gu(e, "complement_isogenous_to_target")
    );
    r.finish()
}

// ------------------------------------------------- independent sweep rebuild

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}

fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b % p) % p
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

/// Quadratic-character table of `F_p`: 0, 1 or -1 for each residue.
fn chi_table(p: u64) -> Vec<i8> {
    let mut t = vec![-1i8; p as usize];
    t[0] = 0;
    for y in 1..p {
        t[((y * y) % p) as usize] = 1;
    }
    t
}

/// `#E(F_p)` for `y^2 = x^3 + a2 x^2 + a4 x + a6`, by the character sum.
fn ec_count(chi: &[i8], p: u64, a2: u64, a4: u64, a6: u64) -> u64 {
    let mut n = p as i64 + 1;
    for x in 0..p {
        let v = addm(
            addm(
                addm(powm(x, 3, p), mulm(a2, mulm(x, x, p), p), p),
                mulm(a4, x, p),
                p,
            ),
            a6,
            p,
        );
        n += chi[v as usize] as i64;
    }
    n as u64
}

/// Nonzero discriminant of the monic cubic `x^3 + a x^2 + b x + c`.
fn cubic_squarefree(p: u64, a: u64, b: u64, c: u64) -> bool {
    let m = |x: u64, y: u64| mulm(x, y, p);
    let t1 = m(m(a, a), m(b, b));
    let t2 = m(4, m(b, m(b, b)));
    let t3 = m(4, m(m(a, m(a, a)), c));
    let t4 = m(27, m(c, c));
    let t5 = m(18, m(a, m(b, c)));
    let d = addm(subm(subm(subm(t1, t2, p), t3, p), t4, p), t5, p);
    d != 0
}

/// Largest prime `d` with `d || n` (exponent exactly one); 1 if none.
fn largest_exact_prime(mut n: u64) -> u64 {
    let mut best = 1;
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            if e == 1 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

#[derive(Default, Debug, PartialEq)]
struct SweepOut {
    p: u64,
    target_order: u64,
    target_lp: u64,
    covers: u64,
    distinct: u64,
    min_c: u64,
    max_c: u64,
    isog: u64,
    anom: u64,
    ss: u64,
    smaller: u64,
    smaller_std: u64,
    easier_reach: u64,
}

fn sweep_indep_one(chi: &[i8], p: u64, f: [u64; 4], maxc: u64) -> Option<SweepOut> {
    // target curve, monic model of Y^2 = F(X)
    let (c0, c1, c2, c3) = (f[0], f[1], f[2], f[3]);
    let ta4 = mulm(c1, c3, p);
    let ta6 = mulm(c0, mulm(c3, c3, p), p);
    if !cubic_squarefree(p, c2, ta4, ta6) {
        return None;
    }
    let torder = ec_count(chi, p, c2, ta4, ta6);
    let tlp = largest_exact_prime(torder);
    let t_special = torder == p || (p as i64 + 1 - torder as i64).rem_euclid(p as i64) == 0;
    let mut out = SweepOut {
        p,
        target_order: torder,
        target_lp: tlp,
        min_c: u64::MAX,
        ..Default::default()
    };
    let mut orders = std::collections::BTreeSet::new();
    let binom = [[1u64, 0, 0, 0], [1, 1, 0, 0], [1, 2, 1, 0], [1, 3, 3, 1]];
    for c in 0..maxc {
        // G(X) = F(X + c): g_i = sum_{j >= i} C(j, i) f_j c^(j - i)
        let mut g = [0u64; 4];
        for (i, gi) in g.iter_mut().enumerate() {
            for (j, fj) in f.iter().enumerate().skip(i) {
                *gi = addm(
                    *gi,
                    mulm(binom[j][i] % p, mulm(*fj, powm(c, (j - i) as u64, p), p), p),
                    p,
                );
            }
        }
        if g[0] == 0 || g[3] == 0 {
            continue;
        }
        // E1 : Y^2 = G(X) monic; E2 : the reversed cubic, monic.
        let a4 = mulm(g[1], g[3], p);
        let a6 = mulm(g[0], mulm(g[3], g[3], p), p);
        if !cubic_squarefree(p, g[2], a4, a6) {
            continue;
        }
        let b4 = mulm(g[2], g[0], p);
        let b6 = mulm(g[3], mulm(g[0], g[0], p), p);
        if !cubic_squarefree(p, g[1], b4, b6) {
            continue;
        }
        // rational Weierstrass point: x in 1..p with G(x^2) = 0
        let has_w = (1..p).any(|x| {
            let x2 = mulm(x, x, p);
            let mut v = g[3];
            for i in (0..3).rev() {
                v = addm(mulm(v, x2, p), g[i], p);
            }
            v == 0
        });
        if !has_w {
            continue;
        }
        if ec_count(chi, p, g[2], a4, a6) != torder {
            continue;
        }
        let e2 = ec_count(chi, p, g[1], b4, b6);
        let e2_tr = p as i64 + 1 - e2 as i64;
        let e2_an = e2 == p;
        let e2_ss = e2_tr.rem_euclid(p as i64) == 0;
        let e2_lp = largest_exact_prime(e2);
        out.covers += 1;
        orders.insert(e2);
        out.min_c = out.min_c.min(e2);
        out.max_c = out.max_c.max(e2);
        let iso = e2 == torder;
        out.isog += iso as u64;
        out.anom += e2_an as u64;
        out.ss += e2_ss as u64;
        out.smaller += (e2_lp < tlp) as u64;
        out.smaller_std += (largest_prime_factor(e2) < largest_prime_factor(torder)) as u64;
        let easier = e2_lp < tlp || (!t_special && (e2_an || e2_ss));
        out.easier_reach += (easier && iso) as u64;
    }
    out.distinct = orders.len() as u64;
    if out.min_c == u64::MAX {
        out.min_c = 0;
    }
    Some(out)
}

fn cmd_sweep_indep(path: &str, maxc: u64) -> ExitCode {
    let v = load(path);
    let reps = arr(&v);
    let mut r = Rep::new();
    let targets: [[u64; 4]; 6] = [
        [7, 3, 2, 1],
        [5, 1, 4, 1],
        [11, 2, 6, 1],
        [3, 9, 1, 2],
        [13, 4, 3, 1],
        [2, 7, 5, 3],
    ];
    let mut idx = 0usize;
    let mut tot = SweepOut::default();
    let mut maxd = 0;
    for p in [1009u64, 2003, 4001, 8009] {
        let chi = chi_table(p);
        for f in targets {
            let Some(o) = sweep_indep_one(&chi, p, f, maxc) else {
                println!("  info p={p} target {f:?}: no valid target (absent)");
                continue;
            };
            let c = &reps[idx];
            idx += 1;
            let same = gu(c, "p") == o.p
                && gu(c, "target_order") == o.target_order
                && gu(c, "target_largest_prime") == o.target_lp
                && gu(c, "covers") == o.covers
                && gu(c, "distinct_complement_orders") == o.distinct
                && gu(c, "min_complement_order") == o.min_c
                && gu(c, "max_complement_order") == o.max_c
                && gu(c, "complement_isogenous_to_target") == o.isog
                && gu(c, "complement_anomalous") == o.anom
                && gu(c, "complement_supersingular") == o.ss
                && gu(c, "complement_smaller_prime") == o.smaller
                && gu(c, "easier_and_reachable") == o.easier_reach;
            r.check(
                &format!("p={p} target {f:?} (order {})", o.target_order),
                same,
                &format!(
                    "covers {} distinct {} smaller {} isog {} ss {} (committed covers {} distinct {} smaller {} isog {} ss {})",
                    o.covers,
                    o.distinct,
                    o.smaller,
                    o.isog,
                    o.ss,
                    gu(c, "covers"),
                    gu(c, "distinct_complement_orders"),
                    gu(c, "complement_smaller_prime"),
                    gu(c, "complement_isogenous_to_target"),
                    gu(c, "complement_supersingular")
                ),
            );
            tot.covers += o.covers;
            tot.smaller += o.smaller;
            tot.smaller_std += o.smaller_std;
            tot.ss += o.ss;
            tot.easier_reach += o.easier_reach;
            maxd = maxd.max(o.distinct);
        }
    }
    println!(
        "  info sensitivity: with the standard largest prime factor (any exponent) in place of the largest exactly-once prime, smaller-prime complements = {} (committed definition {})",
        tot.smaller_std, tot.smaller
    );
    r.check(
        "independent totals",
        idx == reps.len() && tot.covers == 5641 && tot.smaller == 3479 && maxd == 141,
        &format!(
            "{idx} reports, covers {}, smaller-prime {}, max distinct {maxd}, supersingular {}, easier_and_reachable {}",
            tot.covers, tot.smaller, tot.ss, tot.easier_reach
        ),
    );
    r.finish()
}

// --------------------------------------------------------------- tower json

fn cmd_tower(path: &str) -> ExitCode {
    let v = load(path);
    let rows = arr(&v);
    let mut r = Rep::new();
    r.check("eight primes", rows.len() == 8, &format!("{}", rows.len()));
    let mut lp = vec![];
    let (mut l_e, mut l_2, mut l_3) = (vec![], vec![], vec![]);
    let (mut rmin, mut rmax) = (f64::MAX, 0f64);
    let (mut a3min, mut a2min, mut a2max) = (f64::MAX, f64::MAX, 0f64);
    let mut a2_01min = f64::MAX;
    let mut a2_1max = 0f64;
    // TOWER.md priced table, transcribed: (p, IC2 @0/@0.1/@1, IC3 @0/@0.1/@1)
    let doc: [(u64, [&str; 3], [&str; 3]); 4] = [
        (53, ["0.53", "2.09", "16.1"], ["1.29", "4.73", "35.7"]),
        (101, ["0.68", "4.25", "36.4"], ["1.58", "12.35", "109.3"]),
        (151, ["1.00", "7.19", "62.9"], ["3.20", "24.21", "213.3"]),
        (251, ["1.08", "10.97", "100.0"], ["3.34", "35.82", "328.1"]),
    ];
    for row in rows {
        let p = gu(row, "p");
        let rho = gf(row, "rho_e");
        let (i2, i3) = (gf(row, "ic_j2"), gf(row, "ic_j3"));
        let (l2, l3) = (
            gf(row, "ic_j2_linalg_mulmods"),
            gf(row, "ic_j3_linalg_mulmods"),
        );
        let q = i3 / i2;
        rmin = rmin.min(q);
        rmax = rmax.max(q);
        lp.push((p as f64).ln());
        l_e.push(rho.ln());
        l_2.push(i2.ln());
        l_3.push(i3.ln());
        let band = |ic: f64, la: f64, w: f64| (ic + w * la) / rho;
        a3min = a3min.min(band(i3, l3, 0.0));
        a2min = a2min.min(band(i2, l2, 0.0));
        a2max = a2max.max(band(i2, l2, 0.0));
        a2_01min = a2_01min.min(band(i2, l2, 0.1));
        a2_1max = a2_1max.max(band(i2, l2, 1.0));
        if let Some((_, d2, d3)) = doc.iter().find(|d| d.0 == p) {
            for (k, w) in [0.0, 0.1, 1.0].into_iter().enumerate() {
                let m2 = band(i2, l2, w);
                let m3 = band(i3, l3, w);
                // the doc prints 1 decimal at w = 1 and 2 decimals otherwise
                let (s2, s3) = if k == 2 {
                    (format!("{m2:.1}"), format!("{m3:.1}"))
                } else {
                    (format!("{m2:.2}"), format!("{m3:.2}"))
                };
                r.check(
                    &format!("p={p} priced w={w}: IC2/rho, IC3/rho vs TOWER.md"),
                    s2 == d2[k] && s3 == d3[k],
                    &format!("recomputed {s2}, {s3}; doc {}, {}", d2[k], d3[k]),
                );
            }
        }
        // the hom_checks and recovered flags
        r.check(
            &format!("p={p} all_recovered"),
            gb(row, "all_recovered"),
            &format!(
                "hom checks g2 {} g3 {}",
                gu(row, "hom_checks_g2"),
                gu(row, "hom_checks_g3")
            ),
        );
    }
    r.check(
        "IC3/IC2 range 2.3-3.4",
        format!("{rmin:.1}") == "2.3" && format!("{rmax:.1}") == "3.4",
        &format!("min {rmin:.3} max {rmax:.3}"),
    );
    let (s2, s3, se) = (slope(&lp, &l_2), slope(&lp, &l_3), slope(&lp, &l_e));
    r.check(
        "slope IC genus 2 = 0.85",
        format!("{s2:.2}") == "0.85",
        &format!("{s2:.6}"),
    );
    r.check(
        "slope IC genus 3 = 1.06",
        format!("{s3:.2}") == "1.06",
        &format!("{s3:.6}"),
    );
    r.check(
        "slope rho on E = 0.44",
        format!("{se:.2}") == "0.44",
        &format!("{se:.6}"),
    );
    r.check(
        "genus 3 >= 1.29x rho at 0 GAE (all 8 primes)",
        format!("{a3min:.2}") == "1.29",
        &format!("min over p of IC3/rho_E at w=0: {a3min:.4}"),
    );
    r.check(
        "genus 2 0.44-1.08x rho at 0 GAE (all 8 primes)",
        format!("{a2min:.2}") == "0.44" && format!("{a2max:.2}") == "1.08",
        &format!("min {a2min:.4} max {a2max:.4}"),
    );
    r.check(
        "genus 2 above rho at 0.1 and 1 GAE, 2.1-100x",
        format!("{a2_01min:.1}") == "2.1" && format!("{a2_1max:.1}") == "100.0",
        &format!("min at 0.1: {a2_01min:.3}; max at 1: {a2_1max:.3}"),
    );
    r.finish()
}

// ------------------------------------------------- cross-map non-vacuity

fn cmd_cross_census() -> ExitCode {
    let mut r = Rep::new();
    for p in [1009u64, 2003, 4001, 8009] {
        let (mut curves, mut slots, mut tested, mut zero_ok, mut ctrl_ok) =
            (0u64, 0u64, 0u64, 0u64, 0u64);
        let mut curves_all_pass = 0u64;
        for seed in 0..200_000u64 {
            if curves >= 300 {
                break;
            }
            let coeffs = [
                1 + (seed.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
                (seed.wrapping_mul(40_503) >> 5) % p,
                (seed.wrapping_mul(2_246_822_519) >> 7) % p,
                1 + (seed.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
            ];
            let Ok(b) = Bielliptic::new(p, coeffs) else {
                continue;
            };
            curves += 1;
            let mut all = true;
            for q in [Quotient::E1, Quotient::E2] {
                let other = if q == Quotient::E1 {
                    Quotient::E2
                } else {
                    Quotient::E1
                };
                let e = *b.curve(q);
                for s in 0..24u64 {
                    slots += 1;
                    let pt = e.point_from_seed(s.wrapping_mul(0x9E37_79B9) + 1);
                    if !b.fiber_generic(q, &pt) {
                        continue;
                    }
                    tested += 1;
                    let d = b.pull(q, &pt);
                    let z = b.push(other, &d).is_none();
                    let c = b.push(q, &d) == e.mul(&pt, 2);
                    zero_ok += z as u64;
                    ctrl_ok += c as u64;
                    all &= z;
                }
            }
            curves_all_pass += all as u64;
        }
        r.check(
            &format!("p={p} cross-map zero on non-skipped samples"),
            curves == 300 && curves_all_pass == 300 && zero_ok == tested && ctrl_ok == tested,
            &format!(
                "{curves} curves, {curves_all_pass} all-pass; {tested}/{slots} sample slots were generic fibers (the rest skipped by fiber_generic); phi2*phi1^*=0 held {zero_ok}/{tested}; control phi1*phi1^*=[2] held {ctrl_ok}/{tested}"
            ),
        );
    }
    r.finish()
}

// ------------------------------------------------------ independent EC code

type IP = Option<(u64, u64)>;

fn iadd(p: u64, a2: u64, a4: u64, x: IP, y: IP) -> IP {
    let ((x1, y1), (x2, y2)) = match (x, y) {
        (None, _) => return y,
        (_, None) => return x,
        (Some(a), Some(b)) => (a, b),
    };
    let lam = if x1 == x2 {
        if (y1 + y2) % p == 0 {
            return None;
        }
        let num = addm(
            addm(mulm(3, mulm(x1, x1, p), p), mulm(mulm(2, a2, p), x1, p), p),
            a4,
            p,
        );
        mulm(num, powm(mulm(2, y1, p), p - 2, p), p)
    } else {
        mulm(subm(y2, y1, p), powm(subm(x2, x1, p), p - 2, p), p)
    };
    let x3 = subm(subm(subm(mulm(lam, lam, p), a2, p), x1, p), x2, p);
    let y3 = subm(mulm(lam, subm(x1, x3, p), p), y1, p);
    Some((x3, y3))
}

fn imul(p: u64, a2: u64, a4: u64, pt: IP, mut n: u64) -> IP {
    let mut acc = None;
    let mut b = pt;
    while n > 0 {
        if n & 1 == 1 {
            acc = iadd(p, a2, a4, acc, b);
        }
        b = iadd(p, a2, a4, b, b);
        n >>= 1;
    }
    acc
}

fn ip(pt: &Pt) -> IP {
    pt.map(|(x, y)| (x.a, y.a))
}

fn on_curve(p: u64, c: &Curve, pt: IP) -> bool {
    match pt {
        None => true,
        Some((x, y)) => {
            let rhs = addm(
                addm(
                    addm(powm(x, 3, p), mulm(c.a2, mulm(x, x, p), p), p),
                    mulm(c.a4, x, p),
                    p,
                ),
                c.a6,
                p,
            );
            mulm(y, y, p) == rhs
        }
    }
}

// ----------------------------------------- replica of the bench measurement

fn largest_cyclic_prime(mut n: u64) -> u64 {
    let mut best = 1u64;
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

fn ec_instance(e: &Curve, r: u64, x: u64) -> Option<(Pt, Pt)> {
    let order = e.order();
    if !order.is_multiple_of(r) {
        return None;
    }
    let cof = order / r;
    for s in 1..5000u64 {
        let g = e.mul(&e.point_from_seed(s.wrapping_mul(0x9E37_79B9)), cof as u128);
        if g.is_some() && e.mul(&g, r as u128).is_none() {
            return Some((g, e.mul(&g, x as u128)));
        }
    }
    None
}

fn find_isogeny(e: &Curve, r: u64) -> Option<VeluIsogeny> {
    let order = e.order();
    for ell in [2u64, 3, 5, 7] {
        if !order.is_multiple_of(ell) || r == ell {
            continue;
        }
        let cof = order / ell;
        for s in 1..2000u64 {
            let g = e.mul(
                &e.point_from_seed(s.wrapping_mul(2_654_435_761)),
                cof as u128,
            );
            if g.is_none() {
                continue;
            }
            if let Some(iso) = VeluIsogeny::from_generator(e, &g) {
                return Some(iso);
            }
        }
    }
    None
}

#[derive(Debug)]
struct Solve {
    ops: u64,
    steps: u64,
    max_steps: u64,
    recovered: Option<u64>,
    verified: bool,
}

struct Detail {
    r: u64,
    x: u64,
    e1: Curve,
    gen: Pt,
    target: Pt,
    e1_order: u64,
    e2_order: u64,
    e2: Curve,
    jac: Genus2Curve,
    dg: crypto_lib::cryptanalysis::genus2_fp::Divisor,
    dt: crypto_lib::cryptanalysis::genus2_fp::Divisor,
    rho_src: Solve,
    rho_isog: Solve,
    isog_degree: u64,
    isog_ok: bool,
    rho_jac: Solve,
    ic_ops: u64,
    ic_trials: u64,
    ic_rel: u64,
    ic_fb: usize,
    ic_recovered: Option<u64>,
    chain_ok: bool,
    bi: Bielliptic,
    ic_la: u64,
    seed: u64,
    iso_inst: Option<(Curve, Pt, Pt)>,
}

enum Attempt {
    Skip(&'static str),
    Done(Box<Detail>),
}

fn solve_ec(e: &Curve, gen: Pt, target: Pt, r: u64, seed: u64) -> Solve {
    let g = EcGroup(*e);
    let max_steps = 200 * r;
    let out = rho_reference(&g, gen, target, r, seed, max_steps);
    Solve {
        ops: out.group_ops.adds + out.group_ops.doubles,
        steps: out.steps,
        max_steps,
        recovered: out.recovered,
        verified: out.verified,
    }
}

fn attempt(p: u64, c: [u64; 4], seed: u64) -> Attempt {
    let Ok(b) = Bielliptic::new(p, c) else {
        return Attempt::Skip("bielliptic refused");
    };
    let e1 = *b.curve(Quotient::E1);
    let e2 = *b.curve(Quotient::E2);
    let jac = b.jac.clone();
    let (e1_order, e2_order) = (e1.order(), e2.order());
    let jac_order = e1_order * e2_order;
    let r = largest_cyclic_prime(e1_order);
    if r < 50 || e2_order.is_multiple_of(r) {
        return Attempt::Skip("r < 50 or r | #E2");
    }
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let Some((gen, target)) = ec_instance(&e1, r, x) else {
        return Attempt::Skip("no ec instance");
    };
    let rho_src = solve_ec(&e1, gen, target, r, seed);
    let mut iso_inst: Option<(Curve, Pt, Pt)> = None;
    let (rho_isog, isog_degree, isog_ok) = match find_isogeny(&e1, r) {
        Some(iso) => {
            let e_prime = iso.codomain;
            let deg = iso.degree() as u64;
            match Hop::isogeny(iso, &gen, &target, r) {
                Ok(hop) => {
                    let ch = Chain::new(vec![hop]);
                    let ok = ch.verify(&Elt::E(gen), r, x as u128);
                    let (gp, tp) = match (ch.transport(&Elt::E(gen)), ch.transport(&Elt::E(target)))
                    {
                        (Elt::E(a), Elt::E(b)) => (a, b),
                        _ => (None, None),
                    };
                    iso_inst = Some((e_prime, gp, tp));
                    (solve_ec(&e_prime, gp, tp, r, seed ^ 0xA5), deg, ok)
                }
                Err(_) => (
                    Solve {
                        ops: 0,
                        steps: 0,
                        max_steps: 0,
                        recovered: None,
                        verified: false,
                    },
                    deg,
                    false,
                ),
            }
        }
        None => (
            Solve {
                ops: 0,
                steps: 0,
                max_steps: 0,
                recovered: None,
                verified: false,
            },
            0,
            false,
        ),
    };
    let Ok(hop) = Hop::pullback(&b, Quotient::E1, &gen, r) else {
        return Attempt::Skip("pullback hop refused");
    };
    let ch = Chain::new(vec![hop]);
    let chain_ok = ch.verify(&Elt::E(gen), r, x as u128);
    let Some((dg, dt)) = jac_instance(&jac, jac_order, r, x, seed) else {
        return Attempt::Skip("no jac instance");
    };
    let jg = JacGroup(jac.clone());
    let max_steps = 200 * r;
    let rj = rho_reference(&jg, dg, dt, r, seed ^ 0x5A, max_steps);
    let rho_jac = Solve {
        ops: rj.group_ops.adds + rj.group_ops.doubles,
        steps: rj.steps,
        max_steps,
        recovered: rj.recovered,
        verified: rj.verified,
    };
    let ic = genus2_index_calculus(&jac, &dg, &dt, r, p as usize, 50_000_000, seed);
    Attempt::Done(Box::new(Detail {
        r,
        x,
        e1,
        gen,
        target,
        e1_order,
        e2_order,
        e2,
        jac,
        dg,
        dt,
        rho_src,
        rho_isog,
        isog_degree,
        isog_ok,
        rho_jac,
        ic_ops: ic.group_ops,
        ic_trials: ic.trials,
        ic_rel: ic.relations,
        ic_fb: ic.fb_size,
        ic_recovered: ic.recovered,
        chain_ok,
        bi: b,
        ic_la: ic.la_field_ops,
        seed,
        iso_inst,
    }))
}

fn cmd_certs(path: &str) -> ExitCode {
    let v = load(path);
    let rows = arr(&v);
    let mut r = Rep::new();
    let primes = [211u64, 419, 821, 1619];
    let mut corrected_pts: Vec<(u64, u64)> = vec![];
    for (i, &p) in primes.iter().enumerate() {
        let committed = &rows[i];
        let (mut skipped, mut failed) = (0u32, 0u32);
        let mut why: std::collections::BTreeMap<&str, u32> = Default::default();
        let mut found: Option<(u64, Box<Detail>)> = None;
        for s in 0..4000u64 {
            let c = [
                1 + (s.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
                (s.wrapping_mul(40_503) >> 5) % p,
                (s.wrapping_mul(2_246_822_519) >> 7) % p,
                1 + (s.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
            ];
            match attempt(p, c, 0xC0FFEE + i as u64 * 101 + s) {
                Attempt::Skip(w) => {
                    skipped += 1;
                    *why.entry(w).or_insert(0) += 1;
                }
                Attempt::Done(d) => {
                    if d.ic_recovered == Some(d.x) && d.chain_ok && d.isog_ok {
                        found = Some((s, d));
                        break;
                    }
                    failed += 1;
                    println!(
                        "  info p={p} s={s}: measured but rejected by the producer's loop (ic_recovered {:?} planted {}, chain_ok {}, isog_ok {})",
                        d.ic_recovered, d.x, d.chain_ok, d.isog_ok
                    );
                }
            }
        }
        let Some((s, d)) = found else {
            r.check(
                &format!("p={p} instance found"),
                false,
                "no accepted instance",
            );
            continue;
        };
        println!(
            "  info p={p}: accepted at s={s}; {skipped} attempts skipped before measuring ({why:?}), {failed} measured then rejected"
        );
        // 1. same numbers as the committed row
        let same = gu(committed, "r") == d.r
            && gu(committed, "x") == d.x
            && gu(committed, "e1_order") == d.e1_order
            && gu(committed, "e2_order") == d.e2_order
            && gu(committed, "rho_src_ops") == d.rho_src.ops
            && gu(committed, "rho_isog_ops") == d.rho_isog.ops
            && gu(committed, "isog_degree") == d.isog_degree
            && gu(committed, "rho_jac_ops") == d.rho_jac.ops
            && gu(committed, "ic_jac_group_ops") == d.ic_ops
            && gu(committed, "ic_jac_trials") == d.ic_trials
            && gu(committed, "ic_jac_relations") == d.ic_rel
            && gu(committed, "ic_jac_fb") == d.ic_fb as u64;
        r.check(
            &format!("p={p} replica row equals committed row"),
            same,
            &format!(
                "r {} x {} rho_src {} rho_isog {} rho_jac {} ic {} trials {}",
                d.r, d.x, d.rho_src.ops, d.rho_isog.ops, d.rho_jac.ops, d.ic_ops, d.ic_trials
            ),
        );
        // 1b. Unpriced linear algebra the bench computes but does not emit (informational).
        {
            let rho = d.rho_src.ops as f64;
            let ic = d.ic_ops as f64;
            let la = d.ic_la as f64;
            println!(
                "  info p={p}: IC linear-algebra field mults (IcOutcome.la_field_ops, absent from results.json) = {}; IC/rho_src at 0 / 0.1 / 1 GAE per LA mult = {:.1} / {:.1} / {:.1}",
                d.ic_la,
                ic / rho,
                (ic + 0.1 * la) / rho,
                (ic + la) / rho
            );
        }
        // 1c. Trivial identity additions that the IC charges as group operations
        //     (measure.rs scalar_counted: `acc = c.add(&acc, &base); ops += 1` with acc = identity).
        {
            let mut rng = d.seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
            let mut next = || {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                rng
            };
            let mut trivial = 0u64;
            for _ in 0..d.ic_trials {
                let alpha = next() % d.r;
                let beta = next() % d.r;
                trivial += (alpha != 0) as u64 + (beta != 0) as u64;
            }
            let corrected = d.ic_ops - trivial;
            println!(
                "  info p={p}: IC ops {} include {} trivial add-to-identity ops; corrected IC {} => IC/rho_src {:.2} (reported {:.2})",
                d.ic_ops,
                trivial,
                corrected,
                corrected as f64 / d.rho_src.ops as f64,
                d.ic_ops as f64 / d.rho_src.ops as f64
            );
            corrected_pts.push((p, corrected));
        }
        // 1d. Run-to-run spread of the three rho columns over 40 seeds; the committed
        //     run has one seed per cell and states no spread.
        {
            let n = 40u64;
            let stats = |v: &[f64]| {
                let m = v.iter().sum::<f64>() / v.len() as f64;
                let sd = (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>()
                    / (v.len() as f64 - 1.0))
                    .sqrt();
                (m, sd)
            };
            let (mut a, mut b, mut c) = (vec![], vec![], vec![]);
            for s in 0..n {
                a.push(solve_ec(&d.e1, d.gen, d.target, d.r, 1000 + s).ops as f64);
                if let Some((ep, gp, tp)) = &d.iso_inst {
                    b.push(solve_ec(ep, *gp, *tp, d.r, 2000 + s).ops as f64);
                }
                let rj = rho_reference(
                    &JacGroup(d.jac.clone()),
                    d.dg,
                    d.dt,
                    d.r,
                    3000 + s,
                    200 * d.r,
                );
                c.push((rj.group_ops.adds + rj.group_ops.doubles) as f64);
            }
            let (ma, sa) = stats(&a);
            let (mb, sb) = stats(&b);
            let (mc, sc) = stats(&c);
            println!(
                "  info p={p}: rho ops over {n} seeds (mean +- sd): E1 {ma:.0} +- {sa:.0}; E1' {mb:.0} +- {sb:.0}; J(C) {mc:.0} +- {sc:.0}; means vs E1: E1' {:.2}x, J(C) {:.2}x; committed single seeds: E1 {} E1' {} J(C) {} (z vs E1 mean/sd: E1' {:+.2}, J(C) {:+.2}, E1 {:+.2})",
                mb / ma,
                mc / ma,
                d.rho_src.ops,
                d.rho_isog.ops,
                d.rho_jac.ops,
                (d.rho_isog.ops as f64 - ma) / sa,
                (d.rho_jac.ops as f64 - ma) / sa,
                (d.rho_src.ops as f64 - ma) / sa
            );
        }
        // 2. independent point counts of E1, E2
        let chi = chi_table(p);
        let n1 = ec_count(&chi, p, d.e1.a2, d.e1.a4, d.e1.a6);
        let n2 = ec_count(&chi, p, d.e2.a2, d.e2.a4, d.e2.a6);
        r.check(
            &format!("p={p} #E1, #E2 by independent character sum"),
            n1 == d.e1_order
                && n2 == d.e2_order
                && n1 == gu(committed, "e1_order")
                && n2 == gu(committed, "e2_order"),
            &format!("#E1 {n1} #E2 {n2}"),
        );
        // 3. #J by the library's point counter (F_p and F_p^2 counts), not by e1*e2
        let nj = d.jac.jacobian_order();
        r.check(
            &format!("p={p} #J(C) by L-polynomial = #E1 * #E2"),
            nj.as_ref().ok() == Some(&(n1 * n2)),
            &format!("{nj:?} vs {}", n1 * n2),
        );
        // 4. independent EC certificate for the planted log on E1
        let (a2, a4) = (d.e1.a2, d.e1.a4);
        let g = ip(&d.gen);
        let t = ip(&d.target);
        let cert_ec = on_curve(p, &d.e1, g)
            && on_curve(p, &d.e1, t)
            && g.is_some()
            && imul(p, a2, a4, g, d.r).is_none()
            && imul(p, a2, a4, g, d.x) == t;
        r.check(
            &format!("p={p} E1 instance: [r]G=O, [x]G=Q, on curve (independent affine code)"),
            cert_ec,
            &format!("r={} x={}", d.r, d.x),
        );
        // 5. the rho solves on E1, E1' and J: recovered == planted, step cap not hit
        for (name, sv) in [
            ("rho on E1", &d.rho_src),
            ("rho on E1'", &d.rho_isog),
            ("rho on J(C)", &d.rho_jac),
        ] {
            r.check(
                &format!("p={p} {name} recovered the planted log"),
                sv.recovered == Some(d.x) && sv.verified && sv.steps < sv.max_steps,
                &format!(
                    "recovered {:?}, planted {}, steps {} of cap {}, ops {}",
                    sv.recovered, d.x, sv.steps, sv.max_steps, sv.ops
                ),
            );
        }
        if let Some(rec) = d.rho_src.recovered {
            r.check(
                &format!("p={p} rho-on-E1 answer certified by independent code"),
                imul(p, a2, a4, g, rec) == t,
                &format!("[{rec}]G == Q"),
            );
        }
        // 6. Jacobian instance certificate with the library group law, separate flow
        let jr = d.jac.mul(&d.dg, d.r).is_identity();
        let jx = d.jac.mul(&d.dg, d.x) == d.dt;
        let icr = d.ic_recovered.map(|k| d.jac.mul(&d.dg, k) == d.dt);
        r.check(
            &format!("p={p} J instance: [r]DG=0, [x]DG=DT, IC answer certified"),
            jr && jx && !d.dg.is_identity() && icr == Some(true) && d.ic_recovered == Some(d.x),
            &format!("IC recovered {:?} (planted {})", d.ic_recovered, d.x),
        );
        // 6b. Group-law sanity on J(C), independent of the solvers: [#J]DG = 0 and
        //     random associativity / commutativity / validity checks.
        {
            use rand::{rngs::StdRng, SeedableRng};
            let nj = n1 * n2;
            let mut rng = StdRng::seed_from_u64(0x5EED_0000 + p);
            let mut ok = d.jac.mul(&d.dg, nj).is_identity();
            let mut n_checked = 0u32;
            for _ in 0..300 {
                let (a, b2, c) = (
                    d.jac.random(&mut rng),
                    d.jac.random(&mut rng),
                    d.jac.random(&mut rng),
                );
                let ab = d.jac.add(&a, &b2);
                ok &= ab == d.jac.add(&b2, &a)
                    && d.jac.add(&ab, &c) == d.jac.add(&a, &d.jac.add(&b2, &c))
                    && d.jac.is_valid(&ab)
                    && d.jac.add(&a, &d.jac.neg(&a)).is_identity()
                    && d.jac.double(&a) == d.jac.add(&a, &a);
                n_checked += 1;
            }
            r.check(
                &format!("p={p} J(C) group law: [#J]DG=0 and {n_checked} random triples (assoc, comm, inverse, double, valid)"),
                ok,
                "library Cantor arithmetic is self-consistent on the instances used",
            );
        }
        // 6c. Does the solved surrogate instance (DG, DT) carry the same log as the
        //     transported instance (phi^*(G), phi^*(Q))? Find lambda with
        //     phi^*(G) = [lambda] DG by exhaustion over the order-r group and test
        //     phi^*(Q) = [lambda] DT.
        {
            let pg = d.bi.pull(Quotient::E1, &d.gen);
            let pq = d.bi.pull(Quotient::E1, &d.target);
            let mut lam = None;
            let mut acc = crypto_lib::cryptanalysis::genus2_fp::Divisor::IDENTITY;
            for l in 1..d.r {
                acc = d.jac.add(&acc, &d.dg);
                if acc == pg {
                    lam = Some(l);
                    break;
                }
            }
            let ok = lam.is_some_and(|l| d.jac.mul(&d.dt, l) == pq);
            r.check(
                &format!(
                    "p={p} surrogate (DG,DT) is equivalent to the transported (phi*(G),phi*(Q))"
                ),
                ok,
                &format!(
                    "phi*(G) = [{lam:?}] DG and phi*(Q) = [lambda] DT {}",
                    if ok { "holds" } else { "FAILS" }
                ),
            );
        }
        // 7. r is prime and r || #J
        let isp = (2..d.r).take_while(|q| q * q <= d.r).all(|q| d.r % q != 0);
        let nn = n1 * n2;
        r.check(
            &format!("p={p} r prime and r || #J"),
            isp && nn % d.r == 0 && (nn / d.r) % d.r != 0,
            &format!("r={} #J={nn}", d.r),
        );
    }
    if corrected_pts.len() == 4 {
        let xs: Vec<f64> = corrected_pts
            .iter()
            .map(|(p, _)| (*p as f64).ln())
            .collect();
        let ys: Vec<f64> = corrected_pts
            .iter()
            .map(|(_, c)| (*c as f64).ln())
            .collect();
        println!(
            "  info LSQ slope of ln(IC ops minus trivial adds) vs ln p = {:.4} (reported slope uses the uncorrected counts)",
            slope(&xs, &ys)
        );
    }
    r.finish()
}

fn main() -> ExitCode {
    let a: Vec<String> = std::env::args().collect();
    let need = |n: usize| {
        if a.len() <= n {
            eprintln!("usage: replay_check diff|results|census|sweep|sweep-indep|tower|cross-census|certs ...");
            std::process::exit(2);
        }
    };
    need(1);
    match a[1].as_str() {
        "diff" => {
            need(3);
            cmd_diff(&a[2], &a[3])
        }
        "results" => {
            need(2);
            cmd_results(&a[2])
        }
        "census" => {
            need(2);
            cmd_census(&a[2])
        }
        "sweep" => {
            need(2);
            cmd_sweep(&a[2])
        }
        "sweep-indep" => {
            need(2);
            let maxc = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(600);
            cmd_sweep_indep(&a[2], maxc)
        }
        "tower" => {
            need(2);
            cmd_tower(&a[2])
        }
        "cross-census" => cmd_cross_census(),
        "tower-certs" => {
            need(2);
            cmd_tower_certs(&a[2])
        }
        "certs" => {
            need(2);
            cmd_certs(&a[2])
        }
        _ => {
            eprintln!("unknown command {}", a[1]);
            ExitCode::from(2)
        }
    }
}

/// Largest prime factor of `n` (any exponent), for the sensitivity check on the
/// producer's "smaller largest prime" definition.
fn largest_prime_factor(mut n: u64) -> u64 {
    let mut best = 1;
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            best = best.max(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

// ------------------------------------------------------ tower certificates

fn cmd_tower_certs(path: &str) -> ExitCode {
    use crypto_lib::cryptanalysis::chained_transfer::field::Fp;
    use crypto_lib::cryptanalysis::chained_transfer::tower::build;
    use crypto_lib::cryptanalysis::ecbench::hyper_ic::{solve, HyperIcParams};

    let v = load(path);
    let rows = arr(&v);
    let mut r = Rep::new();
    let params = HyperIcParams {
        fb_size: usize::MAX,
        large_primes: false,
        extra_relations: 8,
        branches: 64,
        d_step_in: 4,
        max_trials: 5_000_000,
    };
    for (i, &p) in [53u64, 79, 101, 131, 151, 181, 211, 251].iter().enumerate() {
        let committed = &rows[i];
        let Some(t) = build(p, 11 + i as u64) else {
            r.check(&format!("p={p} tower instance"), false, "build failed");
            continue;
        };
        let k = Fp::new(p).expect("prime");
        let e = t.e;
        let chi = chi_table(p);
        let n = ec_count(&chi, p, e.a2, e.a4, e.a6);
        let (g, tt) = (ip(&t.gen), ip(&t.target));
        let isp = (2..t.r).take_while(|q| q * q <= t.r).all(|q| t.r % q != 0);
        let ec_ok = on_curve(p, &e, g)
            && on_curve(p, &e, tt)
            && g.is_some()
            && imul(p, e.a2, e.a4, g, t.r).is_none()
            && imul(p, e.a2, e.a4, g, t.x) == tt
            && n % t.r == 0
            && (n / t.r) % t.r != 0
            && isp
            && gu(committed, "r") == t.r;
        r.check(
            &format!("p={p} E instance (independent affine code): [r]G=O, [x]G=Q, r || #E"),
            ec_ok,
            &format!("#E={n} r={} x={}", t.r, t.x),
        );
        let rho_e = rho_reference(&EcGroup(e), t.gen, t.target, t.r, 100, 1 << 24);
        r.check(
            &format!("p={p} rho on E (seed 100) recovered the planted log"),
            rho_e.recovered == Some(t.x)
                && rho_e
                    .recovered
                    .is_some_and(|c| imul(p, e.a2, e.a4, g, c) == tt),
            &format!("recovered {:?} planted {}", rho_e.recovered, t.x),
        );
        for (j, rg) in t.rungs.iter().enumerate() {
            let genus = rg.cover.genus;
            let cur = &rg.cover.curve;
            let key = if j == 0 {
                "hom_checks_g2"
            } else {
                "hom_checks_g3"
            };
            let order_ok = rg.jac_order % t.r == 0
                && (rg.jac_order / t.r) % t.r != 0
                && !rg.d.is_identity()
                && cur.mul_uncounted(rg.d, t.r).is_identity()
                && cur.mul_uncounted(rg.d, rg.jac_order).is_identity()
                && cur.mul_uncounted(rg.d, t.x) == rg.t;
            let sp = (p as f64).sqrt();
            let g2 = 2 * genus as i32;
            let (lo, hi) = ((sp - 1.0).powi(g2), (sp + 1.0).powi(g2));
            let hw = (rg.jac_order as f64) >= lo && (rg.jac_order as f64) <= hi;
            r.check(
                &format!("p={p} genus {genus}: [r]D=0, [#J]D=0, T=[x]D, r || #J, Hasse-Weil"),
                order_ok && hw,
                &format!("#J={} in [{lo:.0},{hi:.0}]", rg.jac_order),
            );
            // recount the homomorphism checks from the public cover map
            let (mut checks, mut all) = (0u32, true);
            for kk in 2..t.r.min(80) {
                if let Some((x, y)) = e.mul(&t.gen, kk as u128) {
                    if let Some(dq) = rg.cover.pull(k.sub(x.a, rg.sigma), y.a) {
                        checks += 1;
                        all &= dq == cur.mul_uncounted(rg.d, kk);
                    }
                }
            }
            r.check(
                &format!("p={p} genus {genus}: phi^*([k]G) = [k]phi^*(G), recounted"),
                all && checks > 0 && checks as u64 == gu(committed, key),
                &format!("{checks} checks (committed {key} = {})", gu(committed, key)),
            );
            let rj = rho_reference(cur, rg.d, rg.t, t.r, 200, 1 << 24);
            r.check(
                &format!("p={p} genus {genus}: rho (seed 200) recovered the planted log"),
                rj.recovered == Some(t.x)
                    && rj
                        .recovered
                        .is_some_and(|c| cur.mul_uncounted(rg.d, c) == rg.t),
                &format!("recovered {:?} planted {}", rj.recovered, t.x),
            );
            match solve(cur, rg.d, rg.t, t.r, rg.jac_order, 300, &params) {
                Ok(out) => r.check(
                    &format!("p={p} genus {genus}: hyper_ic (seed 300) recovered the planted log"),
                    out.recovered == Some(t.x)
                        && out
                            .recovered
                            .is_some_and(|c| cur.mul_uncounted(rg.d, c) == rg.t),
                    &format!("recovered {:?} planted {}", out.recovered, t.x),
                ),
                Err(e) => r.check(
                    &format!("p={p} genus {genus}: hyper_ic"),
                    false,
                    &format!("error {e}"),
                ),
            }
        }
    }
    r.finish()
}
