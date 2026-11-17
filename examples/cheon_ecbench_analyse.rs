//! Analysis of `research/cheon_ecbench_20261022/` (see `PROTOCOL.md`,
//! section 7).  Reads recorded values only: the two ecbench sessions'
//! `records.jsonl`, the frozen `curves.json`, PR #2422's driver runs (the
//! positive control) and the sealed bound records.  Writes `results.md`,
//! `results.json` and the SVG figures.  No measurement.
//!
//! ```text
//! cheon_ecbench_analyse --minus SESSION --minus-naive SESSION --plus SESSION \
//!     --curves curves.json --prev-runs cheon-all.jsonl \
//!     --bound NAME=bound.json ... --out DIR
//! ```

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt::Write as _;

const W: [f64; 3] = [0.0, 0.1, 1.0];
/// Prices of an oracle query: 0, 0.1, 1 GAE, and the server's own work.
const QN: [&str; 4] = ["0", "0.1", "1", "server"];

fn args_all(a: &[String], name: &str) -> Vec<String> {
    a.iter()
        .enumerate()
        .filter(|(_, x)| *x == name)
        .filter_map(|(i, _)| a.get(i + 1).cloned())
        .collect()
}

fn read_lines(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn fmt_n(x: f64) -> String {
    let s = format!("{:.0}", x);
    let mut out = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 && c != '-' {
            out.push(',');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

/// One verified run.
#[derive(Clone, Debug, Default)]
struct Obs {
    t: u64,
    ops: f64,
    unc: f64,
    q: f64,
    srv: f64,
    d: u64,
    phases: BTreeMap<String, f64>,
    uncharged: BTreeMap<String, f64>,
}

#[derive(Default)]
struct Cell {
    n: f64,
    n_exact: u64,
    requested: BTreeMap<String, u64>,
    runs: BTreeMap<String, Vec<Obs>>,
    failed: Vec<(String, u64, String)>,
}

/// Paired runs of `arm` and the reference on the same targets.
fn pairs<'a>(c: &'a Cell, arm: &str) -> Vec<(&'a Obs, &'a Obs)> {
    let reference = if arm == "cheon-naive" {
        "rho-neg@naive"
    } else {
        "rho-neg"
    };
    let (Some(a), Some(r)) = (c.runs.get(arm), c.runs.get(reference)) else {
        return vec![];
    };
    a.iter()
        .filter_map(|o| r.iter().find(|x| x.t == o.t).map(|x| (o, x)))
        .collect()
}

/// `sum cheon(w, q) / sum rho(w)` over pairs, `q` an index into [`QN`].
fn ratio(p: &[(&Obs, &Obs)], w: f64, q: usize) -> f64 {
    let qp = |o: &Obs| match q {
        0 => 0.0,
        1 => 0.1 * o.q,
        2 => o.q,
        _ => o.srv,
    };
    let num: f64 = p.iter().map(|(c, _)| c.ops + w * c.unc + qp(c)).sum();
    let den: f64 = p.iter().map(|(_, r)| r.ops + w * r.unc).sum();
    num / den
}

fn boot(p: &[(&Obs, &Obs)], w: f64, q: usize, seed: u64) -> (f64, f64) {
    let mut rng = Rng(seed);
    let mut v = Vec::new();
    for _ in 0..2000 {
        let s: Vec<(&Obs, &Obs)> = (0..p.len())
            .map(|_| p[(rng.next() % p.len() as u64) as usize])
            .collect();
        v.push(ratio(&s, w, q));
    }
    (pct(&mut v.clone(), 0.025), pct(&mut v, 0.975))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let one = |n: &str| args_all(&a, n).into_iter().next();
    let out = one("--out").expect("--out DIR");
    std::fs::create_dir_all(&out).unwrap();
    let curves: Value =
        serde_json::from_str(&std::fs::read_to_string(one("--curves").expect("--curves")).unwrap())
            .unwrap();
    let frozen_d: BTreeMap<(String, u64), u64> = curves["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            (
                (
                    c["family"].as_str().unwrap().to_string(),
                    c["bits"].as_u64().unwrap(),
                ),
                c["d"].as_u64().unwrap(),
            )
        })
        .collect();

    // (family, bits) -> cell
    let mut cells: BTreeMap<(String, u64), Cell> = BTreeMap::new();
    let mut session_ids = BTreeMap::new();
    for (fam, flag) in [
        ("minus", "--minus"),
        ("minus", "--minus-naive"),
        ("plus", "--plus"),
    ] {
        let dir = one(flag).unwrap_or_else(|| panic!("{flag} SESSION"));
        for rec in read_lines(&format!("{dir}/records.jsonl")) {
            if rec["warmup"].as_bool() == Some(true) {
                continue;
            }
            session_ids.insert(flag, rec["session_id"].as_str().unwrap().to_string());
            let bits = rec["workload"]["curve"]["field_bits"].as_u64().unwrap();
            let cell = cells.entry((fam.to_string(), bits)).or_default();
            cell.n = rec["workload"]["curve"]["r"].as_u64().unwrap() as f64;
            cell.n_exact = rec["workload"]["curve"]["r"].as_u64().unwrap();
            let mut arm = rec["arm"].as_str().unwrap().to_string();
            if flag == "--minus-naive" && arm == "rho-neg" {
                arm = "rho-neg@naive".into();
            }
            *cell.requested.entry(arm.clone()).or_insert(0) += 1;
            let t = rec["workload"]["target_index"].as_u64().unwrap();
            let status = rec["outcome"]["status"].as_str().unwrap();
            if status != "verified" {
                let why = rec["outcome"]["error"].as_str().unwrap_or("").to_string();
                cell.failed.push((arm, t, format!("{status} {why}")));
                continue;
            }
            let ctr = |k: &str| rec["counters"][k].as_f64().unwrap_or(0.0);
            let mut o = Obs {
                t,
                ops: rec["cost"]["total_gae"].as_f64().unwrap(),
                q: ctr("oracle_queries"),
                srv: ctr("oracle_server_group_ops"),
                d: rec["counters"]["d"].as_u64().unwrap_or(0),
                ..Default::default()
            };
            for p in rec["phases"].as_array().unwrap() {
                o.phases.insert(
                    p["name"].as_str().unwrap().into(),
                    p["gae"].as_f64().unwrap(),
                );
            }
            if let Some(m) = rec["counters"].as_object() {
                for (k, v) in m {
                    if k.ends_with("_uncharged") {
                        o.uncharged.insert(k.clone(), v.as_f64().unwrap());
                        o.unc += v.as_f64().unwrap();
                    }
                }
            }
            cell.runs.entry(arm).or_default().push(o);
        }
    }

    let bounds: BTreeMap<String, Value> = args_all(&a, "--bound")
        .iter()
        .map(|s| {
            let (n, p) = s.split_once('=').unwrap();
            (
                n.to_string(),
                serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap(),
            )
        })
        .collect();

    let mut md = String::new();
    let mut js = json!({"sessions": session_ids});
    let arms_of = |fam: &str| -> Vec<&'static str> {
        if fam == "minus" {
            vec!["cheon-comb", "cheon-naive"]
        } else {
            vec!["cheon-comb"]
        }
    };

    let all_arms = |fam: &str| -> Vec<&'static str> {
        let mut v = vec!["rho-neg"];
        if fam == "minus" {
            v.push("rho-neg@naive");
        }
        v.extend(arms_of(fam));
        v
    };

    // Table 1
    writeln!(
        md,
        "### Table 1. Measured operation counts, every verified run\n"
    )
    .unwrap();
    writeln!(md, "Unit: group operations (`ecbench.gae`, one affine addition or doubling = 1), every phase charged. `S = mean ops / sqrt(n)`. `ratio to rho` is `sum ops(Cheon) / sum ops(rho)` over the paired targets (same workload), with a 95 % bootstrap interval (2000 resamples of targets, fixed seed). `queries` and `server ops` are the static-DH oracle's resource, outside `ops`.\n").unwrap();
    writeln!(md, "| family | bits | n | d | arm | verified/requested | mean ops | min | max | S | ratio to rho (95 %) | oracle queries | server ops |").unwrap();
    writeln!(md, "|:--|--:|--:|--:|:--|:--|--:|--:|--:|--:|:--|--:|--:|").unwrap();
    let mut t1 = vec![];
    for ((fam, bits), c) in &cells {
        for arm in all_arms(fam) {
            if arm == "rho-neg@naive" {
                continue;
            }
            let Some(v) = c.runs.get(arm) else { continue };
            let ops: Vec<f64> = v.iter().map(|o| o.ops).collect();
            let req = c.requested.get(arm).copied().unwrap_or(0);
            let m = mean(&ops);
            let (rat, ci) = if arm.starts_with("rho-neg") {
                ("reference".to_string(), None)
            } else {
                let p = pairs(c, arm);
                let r = ratio(&p, 0.0, 0);
                let ci = boot(&p, 0.0, 0, 7 + *bits);
                (
                    format!("{:.2} [{:.2}, {:.2}]", r, ci.0, ci.1),
                    Some((r, ci)),
                )
            };
            let d = v.iter().map(|o| o.d).max().unwrap_or(0);
            writeln!(
                md,
                "| {fam} | {bits} | {} | {} | `{arm}` | {}/{req} | {} | {} | {} | {:.3} | {rat} | {} | {} |",
                c.n_exact,
                if d > 0 { d.to_string() } else { "".into() },
                v.len(),
                fmt_n(m),
                fmt_n(ops.iter().cloned().fold(f64::MAX, f64::min)),
                fmt_n(ops.iter().cloned().fold(f64::MIN, f64::max)),
                m / c.n.sqrt(),
                fmt_n(mean(&v.iter().map(|o| o.q).collect::<Vec<_>>())),
                fmt_n(mean(&v.iter().map(|o| o.srv).collect::<Vec<_>>())),
            )
            .unwrap();
            t1.push(json!({"family": fam, "bits": bits, "arm": arm, "n": c.n, "verified": v.len(),
                "mean_ops": m, "S": m / c.n.sqrt(), "ratio_to_rho": ci.map(|(r, c)| json!({"ratio": r, "ci95": [c.0, c.1]})),
                "mean_queries": mean(&v.iter().map(|o| o.q).collect::<Vec<_>>()),
                "mean_server_ops": mean(&v.iter().map(|o| o.srv).collect::<Vec<_>>())}));
        }
    }
    js["table1"] = json!(t1);
    writeln!(
        md,
        "\n### Table 1b. Failures, wrong answers and timeouts (preserved)\n"
    )
    .unwrap();
    let nfail: usize = cells.values().map(|c| c.failed.len()).sum();
    if nfail == 0 {
        writeln!(md, "None: every recorded run verified.\n").unwrap();
    } else {
        for ((fam, bits), c) in &cells {
            for (arm, t, why) in &c.failed {
                writeln!(md, "- {fam} {bits} bits, `{arm}`, target {t}: {why}").unwrap();
            }
        }
    }

    // Table 2: the w x q grid
    writeln!(
        md,
        "\n### Table 2. Pricing grid (AGENTS.md 5a, extended to oracle queries)\n"
    )
    .unwrap();
    writeln!(md, "Each cell is `sum (ops + w U + q Q) / sum (ops_rho + w U_rho)` over the paired targets: `U` the arm's unpriced native operations (every `*_uncharged` counter, rho's included), `w` in {{0, 0.1, 1}} GAE each, the same for both arms; `Q` the oracle queries, priced at `q` in {{0, 0.1, 1}} GAE each or at the server's own work (`server`); rho has no queries. The order is **resolved** only if every one of the 12 cells is on the same side of 1; `min` and `max` are the extreme cells, the interval is the 95 % bootstrap interval of the extreme cell on the side of 1.\n").unwrap();
    writeln!(
        md,
        "| family | bits | arm | min cell | max cell | w=0,q=0 | w=1,q=1 | w=1,q=server | order | 95 % interval of the extreme cell excludes 1 |"
    )
    .unwrap();
    writeln!(md, "|:--|--:|:--|--:|--:|--:|--:|--:|:--|:--|").unwrap();
    let mut t2 = vec![];
    for ((fam, bits), c) in &cells {
        for arm in arms_of(fam) {
            let p = pairs(c, arm);
            if p.is_empty() {
                continue;
            }
            let mut grid = vec![];
            for (wi, w) in W.iter().enumerate() {
                for qi in 0..4 {
                    grid.push((wi, qi, ratio(&p, *w, qi)));
                }
            }
            let lo = grid
                .iter()
                .cloned()
                .fold((0, 0, f64::MAX), |a, b| if b.2 < a.2 { b } else { a });
            let hi = grid
                .iter()
                .cloned()
                .fold((0, 0, f64::MIN), |a, b| if b.2 > a.2 { b } else { a });
            let (order, robust) = if hi.2 < 1.0 {
                let ci = boot(&p, W[hi.0], hi.1, 11 + *bits);
                (
                    "cheaper than rho at all 12 prices".to_string(),
                    format!(
                        "{} (worst cell upper bound {:.2})",
                        if ci.1 < 1.0 { "yes" } else { "no" },
                        ci.1
                    ),
                )
            } else if lo.2 > 1.0 {
                let ci = boot(&p, W[lo.0], lo.1, 13 + *bits);
                (
                    "dearer than rho at all 12 prices".to_string(),
                    format!(
                        "{} (best cell lower bound {:.2})",
                        if ci.0 > 1.0 { "yes" } else { "no" },
                        ci.0
                    ),
                )
            } else {
                ("unresolved".to_string(), "".to_string())
            };
            let cell = |w: usize, q: usize| {
                format!(
                    "{:.3}",
                    grid.iter().find(|g| g.0 == w && g.1 == q).unwrap().2
                )
            };
            writeln!(
                md,
                "| {fam} | {bits} | `{arm}` | {:.3} (w={}, q={}) | {:.3} (w={}, q={}) | {} | {} | {} | {order} | {robust} |",
                lo.2, W[lo.0], QN[lo.1], hi.2, W[hi.0], QN[hi.1], cell(0, 0), cell(2, 2), cell(2, 3)
            )
            .unwrap();
            t2.push(json!({"family": fam, "bits": bits, "arm": arm, "min": lo.2, "max": hi.2, "order": order, "interval_excludes_1": robust,
                "grid": grid.iter().map(|g| json!({"w": W[g.0], "q": QN[g.1], "ratio": g.2})).collect::<Vec<_>>()}));
        }
    }
    js["table2"] = json!(t2);

    // Table 2b: the gap pooled over tier medium (36 to 56 bits), per price.
    writeln!(md, "\n### Table 2b. Gap to the matched rho, pooled over tier medium (36 to 56 bits), at every price\n").unwrap();
    writeln!(md, "Pooled paired ratio `sum cost(Cheon) / sum cost(rho)` over every verified (curve, target) pair of 36 to 56 bits (the 32-bit toy size is left out), with a 95 % bootstrap interval over the pairs; rows are the price `w` of an unpriced native operation (both arms), columns the price `q` of an oracle query. This is a distance to rho, not a pass or fail.\n").unwrap();
    let mut t2b = vec![];
    for (fam, arm) in [
        ("minus", "cheon-comb"),
        ("minus", "cheon-naive"),
        ("plus", "cheon-comb"),
    ] {
        let pooled: Vec<(&Obs, &Obs)> = cells
            .iter()
            .filter(|(k, _)| k.0 == fam && k.1 >= 36)
            .flat_map(|(_, c)| pairs(c, arm))
            .collect();
        if pooled.is_empty() {
            continue;
        }
        writeln!(
            md,
            "`{arm}` on the `{fam}` curves, {} pairs:\n",
            pooled.len()
        )
        .unwrap();
        writeln!(md, "| w \\ q | 0 | 0.1 | 1 | server |").unwrap();
        writeln!(md, "|--:|--:|--:|--:|--:|").unwrap();
        for w in W {
            let mut row = format!("| {w} |");
            for qi in 0..4 {
                let r = ratio(&pooled, w, qi);
                let ci = boot(&pooled, w, qi, 17 + qi as u64);
                write!(row, " {:.3} [{:.3}, {:.3}] |", r, ci.0, ci.1).unwrap();
                t2b.push(json!({"family": fam, "arm": arm, "w": w, "q": QN[qi], "ratio": r, "ci95": [ci.0, ci.1], "pairs": pooled.len()}));
            }
            writeln!(md, "{row}").unwrap();
        }
        writeln!(md).unwrap();
    }
    js["table2b"] = json!(t2b);

    writeln!(md, "\n### Table 3. Unpriced counters and the oracle's price share (means over verified runs)\n").unwrap();
    writeln!(md, "| family | bits | arm | unpriced counters (mean count) | oracle queries | server ops | server ops as % of ops |").unwrap();
    writeln!(md, "|:--|--:|:--|:--|--:|--:|--:|").unwrap();
    for ((fam, bits), c) in &cells {
        for arm in all_arms(fam) {
            if arm == "rho-neg@naive" {
                continue;
            }
            let Some(v) = c.runs.get(arm) else { continue };
            let mut names: BTreeMap<&String, Vec<f64>> = BTreeMap::new();
            for o in v {
                for (k, x) in &o.uncharged {
                    names.entry(k).or_default().push(*x);
                }
            }
            let cs: Vec<String> = names
                .iter()
                .map(|(k, x)| format!("{k}={}", fmt_n(mean(x))))
                .collect();
            let ops = mean(&v.iter().map(|o| o.ops).collect::<Vec<_>>());
            let srv = mean(&v.iter().map(|o| o.srv).collect::<Vec<_>>());
            writeln!(
                md,
                "| {fam} | {bits} | `{arm}` | {} | {} | {} | {:.3} |",
                cs.join(", "),
                fmt_n(mean(&v.iter().map(|o| o.q).collect::<Vec<_>>())),
                fmt_n(srv),
                100.0 * srv / ops
            )
            .unwrap();
        }
    }

    // Positive control against PR #2422's driver records.
    if let Some(prev) = one("--prev-runs") {
        let prev = read_lines(&prev);
        let mut same = 0;
        let mut diff = vec![];
        let mut missing = 0;
        for p in &prev {
            let arm = match p["arm"].as_str().unwrap() {
                "cheon.pminus1_comb" | "cheon.pplus1_comb" => "cheon-comb",
                "cheon.pminus1_naive" => "cheon-naive",
                _ => continue,
            };
            let key = (
                p["family"].as_str().unwrap().to_string(),
                p["bits"].as_u64().unwrap(),
            );
            let t = p["target"].as_u64().unwrap();
            let Some(o) = cells
                .get(&key)
                .and_then(|c| c.runs.get(arm))
                .and_then(|v| v.iter().find(|o| o.t == t))
            else {
                missing += 1;
                continue;
            };
            let ok = (o.ops - p["ops"].as_f64().unwrap()).abs() < 0.5
                && o.phases
                    .iter()
                    .all(|(k, v)| (p["phases"][k].as_f64().unwrap_or(-1.0) - v).abs() < 0.5)
                && p["uncharged"]
                    .as_object()
                    .unwrap()
                    .iter()
                    .all(|(k, v)| o.uncharged.get(k).copied() == v.as_f64());
            if ok {
                same += 1;
            } else {
                diff.push(format!(
                    "{key:?} {arm} target {t}: ops {} vs {}",
                    o.ops, p["ops"]
                ));
            }
        }
        writeln!(md, "\n### Table 4. Positive control: the registered methods against PR #2422's driver runs\n").unwrap();
        writeln!(md, "For every (curve, arm, target) the earlier driver recorded, the ecbench method's `ops`, per-phase counts and uncharged counters are compared. Identical: **{same}**; different: **{}**; no counterpart in this session: {missing}.\n", diff.len()).unwrap();
        for d in diff.iter().take(20) {
            writeln!(md, "- DIFFERENT {d}").unwrap();
        }
        js["positive_control"] =
            json!({"identical": same, "different": diff, "unmatched": missing});
    }

    // d check
    let mut dbad = vec![];
    for ((fam, bits), c) in &cells {
        for arm in arms_of(fam) {
            for o in c.runs.get(arm).into_iter().flatten() {
                if frozen_d.get(&(fam.clone(), *bits)) != Some(&o.d) {
                    dbad.push(format!("{fam} {bits} {arm} target {}: d = {}", o.t, o.d));
                }
            }
        }
    }
    writeln!(md, "\nResolved `d` equals the frozen `curves.json` `d` on every verified run: **{}** ({} mismatches).\n", dbad.is_empty(), dbad.len()).unwrap();
    js["d_mismatches"] = json!(dbad);

    // Bounds
    if !bounds.is_empty() {
        writeln!(md, "### Table 5. Sealed bounds (tier medium, 36 to 56 bits), read from the bound records\n").unwrap();
        writeln!(md, "`gae = C r^alpha` fitted on every verified run (least squares, two-stage bootstrap). `ops x floor` is the mean of `S` over the generic floor `sqrt(pi/2A)`. `queries alpha` is the same fit for oracle queries; blank for rho.\n").unwrap();
        writeln!(md, "| bound | arm | weight | sizes | runs | alpha (95 %) | log2 C | ops x floor (95 %) | queries alpha (95 %) | bound id |").unwrap();
        writeln!(md, "|:--|:--|--:|--:|--:|:--|--:|:--|:--|:--|").unwrap();
        let ci = |v: &Value| match (v[0].as_f64(), v[1].as_f64()) {
            (Some(l), Some(h)) => format!("[{l:.3}, {h:.3}]"),
            _ => "".into(),
        };
        let mut t5 = vec![];
        for (name, b) in &bounds {
            let f = &b["fit"];
            let ops = &b["dimensions"]["ops"];
            let qa = if b["oracle"].is_object() {
                format!(
                    "{:.3} {}",
                    b["oracle"]["alpha"].as_f64().unwrap_or(f64::NAN),
                    ci(&b["oracle"]["alpha_ci95"])
                )
            } else {
                String::new()
            };
            writeln!(
                md,
                "| {name} | `{}` | {} | {} | {} | {:.3} {} | {:.2} | {:.3} {} | {qa} | `{}` |",
                b["method"]["id"].as_str().unwrap(),
                b["fit_options"]["weight"]
                    .as_f64()
                    .map(|w| w.to_string())
                    .unwrap_or("charged".into()),
                f["sizes"],
                b["provenance"]["verified"],
                f["alpha"].as_f64().unwrap_or(f64::NAN),
                ci(&f["alpha_ci95"]),
                f["log2_c"].as_f64().unwrap_or(f64::NAN),
                ops["value"].as_f64().unwrap_or(f64::NAN),
                ci(&ops["ci95"]),
                b["bound_id"].as_str().unwrap()
            )
            .unwrap();
            t5.push(json!({"name": name, "alpha": f["alpha"], "alpha_ci95": f["alpha_ci95"], "log2_c": f["log2_c"], "bound_id": b["bound_id"]}));
        }
        js["table5"] = json!(t5);
    }

    std::fs::write(format!("{out}/results.md"), &md).unwrap();
    std::fs::write(
        format!("{out}/results.json"),
        serde_json::to_string_pretty(&js).unwrap() + "\n",
    )
    .unwrap();
    figures(&out, &cells, &bounds);
    println!("{md}");
}

// ── SVG ──────────────────────────────────────────────────────────────────

struct Plot {
    w: f64,
    h: f64,
    ml: f64,
    mr: f64,
    mt: f64,
    mb: f64,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    s: String,
}

const COL_RHO: &str = "#6b7280";
const COL_A: &str = "#c2410c";
const COL_B: &str = "#1d4ed8";
const COL_C: &str = "#15803d";

impl Plot {
    fn new(w: f64, h: f64, x: (f64, f64), y: (f64, f64)) -> Self {
        Plot {
            w,
            h,
            ml: 62.0,
            mr: 16.0,
            mt: 34.0,
            mb: 46.0,
            x0: x.0,
            x1: x.1,
            y0: y.0,
            y1: y.1,
            s: String::new(),
        }
    }
    fn px(&self, x: f64) -> f64 {
        self.ml + (x - self.x0) / (self.x1 - self.x0) * (self.w - self.ml - self.mr)
    }
    fn py(&self, y: f64) -> f64 {
        self.h - self.mb - (y - self.y0) / (self.y1 - self.y0) * (self.h - self.mt - self.mb)
    }
    fn axes(&mut self, xt: f64, yt: f64, xl: &str, yl: &str, title: &str) {
        let mut x = (self.x0 / xt).ceil() * xt;
        while x <= self.x1 + 1e-9 {
            let p = self.px(x);
            write!(self.s, "<line x1='{p:.1}' y1='{:.1}' x2='{p:.1}' y2='{:.1}' stroke='var(--grid)'/><text x='{p:.1}' y='{:.1}' text-anchor='middle' class='t'>{x:.0}</text>", self.mt, self.h - self.mb, self.h - self.mb + 15.0).unwrap();
            x += xt;
        }
        let mut y = (self.y0 / yt).ceil() * yt;
        while y <= self.y1 + 1e-9 {
            let p = self.py(y);
            write!(self.s, "<line x1='{:.1}' y1='{p:.1}' x2='{:.1}' y2='{p:.1}' stroke='var(--grid)'/><text x='{:.1}' y='{:.1}' text-anchor='end' class='t'>{y:.1}</text>", self.ml, self.w - self.mr, self.ml - 6.0, p + 4.0).unwrap();
            y += yt;
        }
        write!(self.s, "<rect x='{:.1}' y='{:.1}' width='{:.1}' height='{:.1}' fill='none' stroke='var(--axis)'/>", self.ml, self.mt, self.w - self.ml - self.mr, self.h - self.mt - self.mb).unwrap();
        write!(
            self.s,
            "<text x='{:.1}' y='{:.1}' text-anchor='middle' class='l'>{xl}</text>",
            (self.ml + self.w - self.mr) / 2.0,
            self.h - 8.0
        )
        .unwrap();
        write!(self.s, "<text transform='translate(14,{:.1}) rotate(-90)' text-anchor='middle' class='l'>{yl}</text>", (self.mt + self.h - self.mb) / 2.0).unwrap();
        write!(
            self.s,
            "<text x='{:.1}' y='20' class='h'>{title}</text>",
            self.ml
        )
        .unwrap();
    }
    fn line(&mut self, pts: &[(f64, f64)], col: &str, dash: bool, w: f64) {
        let d: Vec<String> = pts
            .iter()
            .map(|(x, y)| format!("{:.1},{:.1}", self.px(*x), self.py(*y)))
            .collect();
        write!(
            self.s,
            "<polyline points='{}' fill='none' stroke='{col}' stroke-width='{w}'{}/>",
            d.join(" "),
            if dash { " stroke-dasharray='5 4'" } else { "" }
        )
        .unwrap();
    }
    fn dot(&mut self, x: f64, y: f64, col: &str, r: f64) {
        write!(
            self.s,
            "<circle cx='{:.1}' cy='{:.1}' r='{r}' fill='{col}'/>",
            self.px(x),
            self.py(y)
        )
        .unwrap();
    }
    fn vbar(&mut self, x: f64, lo: f64, hi: f64, col: &str) {
        write!(self.s, "<line x1='{p:.1}' y1='{:.1}' x2='{p:.1}' y2='{:.1}' stroke='{col}' stroke-width='1.5'/>", self.py(lo), self.py(hi), p = self.px(x)).unwrap();
    }
    fn legend(&mut self, items: &[(&str, &str, bool)]) {
        let mut y = self.mt + 16.0;
        for (col, label, dash) in items {
            let x = self.ml + 10.0;
            write!(self.s, "<line x1='{x:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{col}' stroke-width='2'{}/><text x='{:.1}' y='{:.1}' class='t'>{label}</text>", y - 4.0, x + 22.0, y - 4.0, if *dash { " stroke-dasharray='5 4'" } else { "" }, x + 28.0, y).unwrap();
            y += 15.0;
        }
    }
}

fn svg_wrap(w: f64, h: f64, body: &str, caption: &str) -> String {
    let maxc = (w / 5.6) as usize;
    let mut lines: Vec<String> = vec![];
    let mut cur = String::new();
    for word in caption.split_whitespace() {
        if !cur.is_empty() && cur.len() + 1 + word.len() > maxc {
            lines.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    lines.push(cur);
    let hh = h + 8.0 + 14.0 * lines.len() as f64;
    let mut cap = String::new();
    for (i, l) in lines.iter().enumerate() {
        write!(
            cap,
            "<text x='10' y='{:.1}' class='c'>{l}</text>",
            h + 12.0 + 14.0 * i as f64
        )
        .unwrap();
    }
    format!(
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {hh}' width='{w}' height='{hh}' font-family='system-ui, sans-serif'>\n<style>:root{{--bg:#ffffff;--fg:#111827;--grid:#e5e7eb;--axis:#6b7280}}@media (prefers-color-scheme: dark){{:root{{--bg:#111827;--fg:#e5e7eb;--grid:#374151;--axis:#9ca3af}}}}.t{{font-size:11px;fill:var(--fg)}}.l{{font-size:12px;fill:var(--fg)}}.h{{font-size:13px;font-weight:600;fill:var(--fg)}}.c{{font-size:11px;fill:var(--fg)}}</style>\n<rect width='100%' height='100%' fill='var(--bg)'/>\n{body}\n{cap}\n</svg>\n"
    )
}

fn figures(out: &str, cells: &BTreeMap<(String, u64), Cell>, bounds: &BTreeMap<String, Value>) {
    let sets = [
        ("minus", "rho-neg", COL_RHO, "rho.negation"),
        ("minus", "cheon-comb", COL_A, "Cheon n-1 comb"),
        ("minus", "cheon-naive", COL_B, "Cheon n-1 naive"),
        ("plus", "rho-neg", COL_RHO, "rho.negation"),
        ("plus", "cheon-comb", COL_A, "Cheon n+1 comb"),
    ];
    // Fig 1: ops vs log2 n, with the sealed fit lines when given.
    let (pw, ph) = (560.0, 400.0);
    let mut body = String::new();
    for (i, fam) in ["minus", "plus"].iter().enumerate() {
        let mut p = Plot::new(pw, ph, (30.0, 58.0), (14.0, 33.0));
        p.axes(
            4.0,
            2.0,
            "log2 n",
            "log2 group operations",
            if *fam == "minus" {
                "n-1 shape: Cheon vs rho"
            } else {
                "n+1 shape: Cheon vs rho"
            },
        );
        let mut legend = vec![];
        for (f, arm, col, label) in sets.iter().filter(|s| s.0 == *fam) {
            for ((cf, _), c) in cells.iter().filter(|(k, _)| k.0 == *f) {
                let _ = cf;
                let Some(v) = c.runs.get(*arm) else { continue };
                let ys: Vec<f64> = v.iter().map(|o| o.ops.log2()).collect();
                let x = c.n.log2();
                p.vbar(
                    x,
                    ys.iter().cloned().fold(f64::MAX, f64::min),
                    ys.iter().cloned().fold(f64::MIN, f64::max),
                    col,
                );
                p.dot(x, mean(&ys), col, 3.5);
            }
            let key = match *arm {
                "rho-neg" => format!("{fam}-rho"),
                "cheon-comb" => format!("{fam}-comb"),
                _ => format!("{fam}-naive"),
            };
            if let Some(b) = bounds.get(&key) {
                if let (Some(al), Some(c)) =
                    (b["fit"]["alpha"].as_f64(), b["fit"]["log2_c"].as_f64())
                {
                    p.line(
                        &[(35.5, c + al * 35.5), (56.0, c + al * 56.0)],
                        col,
                        false,
                        1.2,
                    );
                }
            }
            legend.push((*col, *label, false));
        }
        p.legend(&legend);
        write!(
            body,
            "<g transform='translate({},0)'>{}</g>",
            i as f64 * pw,
            p.s
        )
        .unwrap();
    }
    std::fs::write(format!("{out}/fig1_scaling_measured.svg"), svg_wrap(pw * 2.0, ph, &body, "Fig. 1. MEASURED log2 charged operations per curve (dots = mean over verified targets, bars = min..max), ecbench sessions of research/cheon_ecbench_20261022. Lines: the sealed bound fits gae = C r^alpha over 36 to 56 bits (the 32-bit toy size is outside the fit). Oracle queries are not in these operations.")).unwrap();

    // Fig 2: ratio to rho per size, band over the w x q grid.
    let mut p = Plot::new(620.0, 360.0, (30.0, 58.0), (-3.0, 7.0));
    p.axes(
        4.0,
        1.0,
        "bits of n",
        "log2 (Cheon cost / rho cost)  (0 = parity)",
        "Distance to the matched rho over the pricing band",
    );
    p.line(&[(30.0, 0.0), (58.0, 0.0)], "var(--axis)", false, 1.0);
    let mut leg = vec![];
    for (fam, arm, col, label) in [
        ("minus", "cheon-comb", COL_A, "n-1, comb"),
        ("minus", "cheon-naive", COL_B, "n-1, naive"),
        ("plus", "cheon-comb", COL_C, "n+1, comb"),
    ] {
        let mut mid = vec![];
        for (k, c) in cells.iter().filter(|(k, _)| k.0 == fam) {
            let pr = pairs(c, arm);
            if pr.is_empty() {
                continue;
            }
            let mut v = vec![];
            for w in W {
                for q in 0..4 {
                    v.push(ratio(&pr, w, q));
                }
            }
            let (lo, hi) = (
                v.iter().cloned().fold(f64::MAX, f64::min),
                v.iter().cloned().fold(f64::MIN, f64::max),
            );
            let x = k.1 as f64 + if arm == "cheon-naive" { 0.4 } else { -0.4 };
            p.vbar(x, lo.log2(), hi.log2(), col);
            let r0 = ratio(&pr, 0.0, 0);
            p.dot(x, r0.log2(), col, 3.5);
            mid.push((x, r0.log2()));
        }
        if mid.len() >= 2 {
            p.line(&mid, col, false, 1.5);
        }
        leg.push((col, label, false));
    }
    p.legend(&leg);
    std::fs::write(format!("{out}/fig2_ratio_to_rho.svg"), svg_wrap(620.0, 360.0, &p.s, "Fig. 2. log2 of the Cheon/rho cost ratio per size. Dots: charged operations only (w = 0, q = 0). Bars: the full range over the 12 pricing cells (w in {0, 0.1, 1} per unpriced native operation, q in {0, 0.1, 1, server work} per oracle query, the same w for both arms). Above 0 = Cheon dearer. Where a bar crosses 0 the order is unresolved. Source: results.json table2.")).unwrap();

    // Fig 3: oracle queries and server cost against n.
    let mut p = Plot::new(620.0, 360.0, (30.0, 58.0), (0.0, 20.0));
    p.axes(
        4.0,
        2.0,
        "bits of n",
        "log2 per run",
        "The oracle: queries and the server's work",
    );
    let mut leg = vec![];
    for (fam, col, ql) in [
        ("minus", COL_A, "n-1 queries (d - 1)"),
        ("plus", COL_C, "n+1 queries (2d - 1)"),
    ] {
        let (mut q, mut s) = (vec![], vec![]);
        for (k, c) in cells.iter().filter(|(k, _)| k.0 == fam) {
            if let Some(v) = c.runs.get("cheon-comb") {
                q.push((
                    k.1 as f64,
                    mean(&v.iter().map(|o| o.q).collect::<Vec<_>>()).log2(),
                ));
                s.push((
                    k.1 as f64,
                    mean(&v.iter().map(|o| o.srv).collect::<Vec<_>>()).log2(),
                ));
            }
        }
        p.line(&q, col, false, 1.8);
        for (x, y) in &q {
            p.dot(*x, *y, col, 3.5);
        }
        p.line(&s, col, true, 1.5);
        leg.push((col, ql, false));
    }
    leg.push(("var(--fg)", "dashed: server group operations", true));
    p.legend(&leg);
    std::fs::write(format!("{out}/fig3_oracle.svg"), svg_wrap(620.0, 360.0, &p.s, "Fig. 3. MEASURED mean static-DH oracle queries per run (solid) and the group operations the server spends answering them (dashed), by size. These are outside the charged operations of Fig. 1. Source: counters.oracle_queries and oracle_server_group_ops of the Cheon comb arms.")).unwrap();

    // Fig 4: phase shares, one panel per family.
    let (pw4, ph4) = (560.0, 340.0);
    let mut body = String::new();
    for (i, fam) in ["minus", "plus"].iter().enumerate() {
        let mut p = Plot::new(pw4, ph4, (30.0, 58.0), (0.0, 100.0));
        p.axes(
            4.0,
            20.0,
            "bits of n",
            "share of charged operations (%)",
            if *fam == "minus" {
                "n-1 comb: where the operations go"
            } else {
                "n+1 comb: where the operations go"
            },
        );
        let mut leg = vec![];
        let groups: Vec<(&str, &str, &str)> = if *fam == "minus" {
            vec![
                ("precompute", COL_B, "comb-table construction"),
                ("baby", COL_A, "baby steps"),
                ("giant", COL_C, "giant steps"),
            ]
        } else {
            vec![
                ("precompute", COL_B, "comb-table construction"),
                ("poly", COL_RHO, "polynomial multi-scalar sums"),
                ("baby", COL_A, "baby steps"),
                ("giant", COL_C, "giant steps"),
            ]
        };
        for (key, col, label) in groups {
            let mut pts = vec![];
            for (k, c) in cells.iter().filter(|(k, _)| k.0 == *fam) {
                if let Some(v) = c.runs.get("cheon-comb") {
                    let sh = mean(
                        &v.iter()
                            .map(|o| {
                                100.0
                                    * o.phases
                                        .iter()
                                        .filter(|(n, _)| n.contains(key))
                                        .map(|(_, x)| *x)
                                        .sum::<f64>()
                                    / o.ops
                            })
                            .collect::<Vec<_>>(),
                    );
                    pts.push((k.1 as f64, sh));
                }
            }
            p.line(&pts, col, false, 2.0);
            for (x, y) in &pts {
                p.dot(*x, *y, col, 3.5);
            }
            leg.push((col, label, false));
        }
        p.legend(&leg);
        write!(
            body,
            "<g transform='translate({},0)'>{}</g>",
            i as f64 * pw4,
            p.s
        )
        .unwrap();
    }
    std::fs::write(
        format!("{out}/fig4_phase_shares.svg"),
        svg_wrap(pw4 * 2.0, ph4, &body, "Fig. 4. MEASURED mean share of the charged operations by phase group, Cheon comb arms (phase names containing the group word; the shift and verification phases are the remainder). Source: the phases of each record."),
    )
    .unwrap();
}
