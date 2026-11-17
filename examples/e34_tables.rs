//! **E34 tables (§9.38/§9.39)**: identity against E29's Stage R records, the
//! cheap walks against the built walks, per-step costs, the §5a band and the
//! rankings.  Reads `experiments/38_twist24_cheap_rep/*.json` and E29's
//! `experiments/34_twist24_stage_r/stage_r_*.json`; writes `tables.md` and
//! `summary.json` into the E34 directory.
//!
//! ```text
//! cargo run --release --example e34_tables
//! ```

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{json, Value};

const DIR: &str = "experiments/38_twist24_cheap_rep";
const E29: &str = "experiments/34_twist24_stage_r";
const PS: [u64; 8] = [241, 337, 541, 601, 733, 769, 997, 1069];
const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const GAE: f64 = 97.0;
const GAE_KARATSUBA_OWN: f64 = 79.0;
const ARMS: [&str; 5] = [
    "Order6Built",
    "Order6Cheap",
    "Order24Built",
    "Order24Cheap",
    "Order24CheapMulLean",
];
const KINDS: [&str; 5] = ["cmp", "neg", "modn", "hash", "table"];
const WALK_FIELDS: [&str; 6] = [
    "steps",
    "group_ops",
    "cycle_escapes",
    "recovered",
    "correct",
    "visit_digest",
];

type Key = (u64, bool, String, u64);

fn load(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect(path)).expect(path)
}

fn u(v: &Value) -> f64 {
    v.as_u64().expect("count") as f64
}

#[derive(Clone, Debug, Default)]
struct Stats {
    walks: usize,
    all_correct: bool,
    steps: f64,
    muls: f64,
    native: f64,
    by_kind: BTreeMap<String, f64>,
    rep_calls: f64,
    rep_muls: f64,
    rep_by_kind: BTreeMap<String, f64>,
    table_muls: f64,
}

impl Stats {
    fn cost(&self, w: f64, gae: f64, full: bool) -> f64 {
        self.muls + w * gae * if full { self.native } else { self.steps }
    }
}

fn main() {
    // E29's records.
    let mut e29: BTreeMap<Key, Value> = BTreeMap::new();
    for name in ["stage_r_p241.json", "stage_r_rest.json"] {
        for row in load(&format!("{E29}/{name}")).as_array().unwrap() {
            let p = row["p"].as_u64().unwrap();
            for run in row["runs"].as_array().unwrap() {
                let kar = run["karatsuba"].as_bool().unwrap();
                for (arm, key) in [("Order6Built", "order6"), ("Order24Built", "order24")] {
                    for w in run[key].as_array().unwrap() {
                        let seed = w["seed"].as_u64().unwrap();
                        e29.insert((p, kar, arm.to_string(), seed), w.clone());
                    }
                }
            }
        }
    }
    // E34's walks.
    let mut walks: BTreeMap<Key, Value> = BTreeMap::new();
    let mut n_of: BTreeMap<u64, u64> = BTreeMap::new();
    for p in PS {
        let rows = load(&format!("{DIR}/stage_r_p{p}.json"));
        for row in rows.as_array().unwrap() {
            n_of.insert(p, row["n"].as_u64().unwrap());
            for w in row["walks"].as_array().unwrap() {
                let key = (
                    p,
                    w["karatsuba"].as_bool().unwrap(),
                    w["arm"].as_str().unwrap().to_string(),
                    w["run"]["seed"].as_u64().unwrap(),
                );
                walks.insert(key, w.clone());
            }
        }
    }

    // (a) counted copies against E29's records (E29's order-24 records carry
    // no visit digest).
    let (mut rec, mut same) = (0u64, 0u64);
    let mut replay_diffs: Vec<Value> = Vec::new();
    for (key, theirs) in &e29 {
        let Some(mine) = walks.get(key) else { continue };
        let mut mine = mine["run"].clone();
        if key.2 == "Order24Built" {
            mine["visit_digest"] = theirs["visit_digest"].clone();
        }
        rec += 1;
        if &mine == theirs {
            same += 1;
        } else {
            replay_diffs.push(json!([key.0, key.1, key.2, key.3]));
        }
    }

    // (c) cheap walks against the built walk of their order.
    let (mut cmp_n, mut cmp_same) = (0u64, 0u64);
    let mut walk_diffs: Vec<Value> = Vec::new();
    for ((p, kar, arm, seed), w) in &walks {
        let reference = match arm.as_str() {
            "Order6Cheap" => "Order6Built",
            "Order24Cheap" | "Order24CheapMulLean" => "Order24Built",
            _ => continue,
        };
        let r = &walks[&(*p, *kar, reference.to_string(), *seed)]["run"];
        cmp_n += 1;
        if WALK_FIELDS.iter().all(|f| w["run"][*f] == r[*f]) {
            cmp_same += 1;
        } else {
            walk_diffs.push(json!([p, kar, arm, seed]));
        }
    }
    let digests_across_arith = walks.iter().filter(|(k, _)| !k.1).all(|(k, w)| {
        walks[&(k.0, true, k.2.clone(), k.3)]["run"]["visit_digest"] == w["run"]["visit_digest"]
    });

    // Per curve, arithmetic and arm.
    let mut stats: BTreeMap<(u64, bool, String), Stats> = BTreeMap::new();
    for ((p, kar, arm, _), w) in &walks {
        let s = stats.entry((*p, *kar, arm.clone())).or_insert(Stats {
            all_correct: true,
            ..Default::default()
        });
        s.walks += 1;
        s.all_correct &= w["run"]["correct"].as_bool().unwrap();
        s.steps += u(&w["run"]["steps"]);
        s.muls += u(&w["run"]["fp_muls"]) - u(&w["run"]["verify_fp_muls"]);
        for k in KINDS {
            let v = u(&w["ops"][k]);
            s.native += v;
            *s.by_kind.entry(k.to_string()).or_default() += v;
        }
        s.rep_calls += u(&w["rep_calls"]);
        s.rep_muls += u(&w["rep_fp_muls"]);
        for k in ["cmp", "neg", "modn"] {
            *s.rep_by_kind.entry(k.to_string()).or_default() += u(&w["rep_ops"][k]);
        }
        s.table_muls += u(&w["table_fp_muls"]);
    }
    for s in stats.values_mut() {
        let m = s.walks as f64;
        s.steps /= m;
        s.muls /= m;
        s.native /= m;
        s.rep_calls /= m;
        s.rep_muls /= m;
        s.table_muls /= m;
        s.by_kind.values_mut().for_each(|v| *v /= m);
        s.rep_by_kind.values_mut().for_each(|v| *v /= m);
    }
    let st = |p: u64, kar: bool, arm: &str| &stats[&(p, kar, arm.to_string())];
    let sqrtn = |p: u64| (n_of[&p] as f64).sqrt();
    let pooled = |arm: &str, kar: bool, w: f64, gae: f64, full: bool| -> f64 {
        PS.iter()
            .map(|&p| st(p, kar, arm).cost(w, gae, full) / sqrtn(p))
            .sum()
    };
    let lab = |kar: bool| if kar { "karatsuba" } else { "schoolbook" };

    // The band.
    let pairs = [
        ("Order6Built", "Order24Built"),
        ("Order6Built", "Order24Cheap"),
        ("Order6Cheap", "Order24Cheap"),
        ("Order6Built", "Order24CheapMulLean"),
        ("Order6Cheap", "Order24CheapMulLean"),
    ];
    let mut band = serde_json::Map::new();
    let mut md_band: Vec<String> = Vec::new();
    for (a6, a24) in pairs {
        for kar in [false, true] {
            for full in [true, false] {
                let gaes: Vec<f64> = if kar && full {
                    vec![GAE, GAE_KARATSUBA_OWN]
                } else {
                    vec![GAE]
                };
                for gae in gaes {
                    let rows: Vec<(f64, f64, Vec<f64>)> = WEIGHTS
                        .iter()
                        .map(|&w| {
                            let per: Vec<f64> = PS
                                .iter()
                                .map(|&p| {
                                    st(p, kar, a6).cost(w, gae, full)
                                        / st(p, kar, a24).cost(w, gae, full)
                                })
                                .collect();
                            (
                                w,
                                pooled(a6, kar, w, gae, full) / pooled(a24, kar, w, gae, full),
                                per,
                            )
                        })
                        .collect();
                    let holds = rows.iter().all(|r| r.1 > 1.0) || rows.iter().all(|r| r.1 < 1.0);
                    let per_24 = (0..PS.len())
                        .filter(|&i| rows.iter().all(|r| r.2[i] > 1.0))
                        .count();
                    let per_6 = (0..PS.len())
                        .filter(|&i| rows.iter().all(|r| r.2[i] < 1.0))
                        .count();
                    let key = format!(
                        "{a6}/{a24}|{}|{}|gae{gae}",
                        lab(kar),
                        if full { "full" } else { "one_per_step" }
                    );
                    md_band.push(format!(
                        "**{key}** — pooled order holds across the band: {holds}; curves resolved for order 24 / order 6 / unresolved: {per_24} / {per_6} / {}\n\n| GAE | pooled | {} |\n|--:|--:|{}\n{}\n",
                        PS.len() - per_24 - per_6,
                        PS.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" | "),
                        "--:|".repeat(PS.len()),
                        rows.iter()
                            .map(|r| format!(
                                "| {} | {:.3} | {} |",
                                r.0,
                                r.1,
                                r.2.iter().map(|x| format!("{x:.3}")).collect::<Vec<_>>().join(" | ")
                            ))
                            .collect::<Vec<_>>()
                            .join("\n")
                    ));
                    band.insert(
                        key,
                        json!({
                            "order_holds_across_band": holds,
                            "curves_order24_cheaper_at_every_weight": per_24,
                            "curves_order6_cheaper_at_every_weight": per_6,
                            "rows": rows.iter().map(|r| json!({
                                "weight": r.0, "pooled": r.1,
                                "per_curve": PS.iter().zip(&r.2).map(|(p, x)| json!({"p": p, "ratio": x})).collect::<Vec<_>>(),
                            })).collect::<Vec<_>>(),
                        }),
                    );
                }
            }
        }
    }

    // Strongest walk of each order at each weight (pooled, full emission,
    // GAE 97); E18's reference is the order-6 walk as built.
    let mut strongest = serde_json::Map::new();
    let mut md_strong = Vec::new();
    for kar in [false, true] {
        let mut rows = Vec::new();
        for w in WEIGHTS {
            let c = |arm: &str| pooled(arm, kar, w, GAE, true);
            let best6 = ["Order6Built", "Order6Cheap"]
                .into_iter()
                .min_by(|a, b| c(a).total_cmp(&c(b)))
                .unwrap();
            let best24 = ["Order24Built", "Order24Cheap", "Order24CheapMulLean"]
                .into_iter()
                .min_by(|a, b| c(a).total_cmp(&c(b)))
                .unwrap();
            let r = c(best6) / c(best24);
            let r_e18 = c("Order6Built") / c(best6).min(c(best24));
            md_strong.push(format!(
                "| {} | {w} | {best6} | {best24} | {r:.3} | {r_e18:.3} | {:.2} |",
                lab(kar),
                4.0 * r_e18.max(1.0).log2()
            ));
            rows.push(json!({
                "weight": w, "order6": best6, "order24": best24, "strongest_ratio": r,
                "e18_reference_over_strongest": r_e18,
                "e18_handover_shift_bits": 4.0 * r_e18.max(1.0).log2(),
            }));
        }
        strongest.insert(lab(kar).to_string(), Value::Array(rows));
    }

    // Identity checks and negative-control walks.
    let ident = load(&format!("{DIR}/identity.json"));
    let mut id_md = Vec::new();
    let mut id_json = serde_json::Map::new();
    for arm in [
        "Order24Cheap",
        "Order24CheapMulLean",
        "Order6Cheap",
        "Order24OmitCandidate",
        "Order24WrongOrdinate",
    ] {
        let mut t = [0u64; 6];
        let mut curves = 0;
        for row in ident.as_array().unwrap() {
            for c in row["checks"].as_array().unwrap() {
                if c["arm"] == arm {
                    curves += 1;
                    for (i, f) in [
                        "members",
                        "mismatches",
                        "forced_x0_zero",
                        "scalar_checks",
                        "scalar_failures",
                    ]
                    .iter()
                    .enumerate()
                    {
                        t[i] += c[*f].as_u64().unwrap();
                    }
                    t[5] += u64::from(c["x_zero_point"].as_bool().unwrap());
                }
            }
        }
        id_md.push(format!(
            "| {arm} | {curves} | {} | {} | {} | {} | {} ({}) |",
            t[0], t[1], t[2], t[5], t[3], t[4]
        ));
        id_json.insert(
            arm.to_string(),
            json!({"curves": curves, "members": t[0], "mismatches": t[1], "forced_x0_zero": t[2],
                   "x_zero_points": t[5], "scalar_checks": t[3], "scalar_failures": t[4]}),
        );
    }
    let (mut nw, mut ndiff, mut ncorrect) = (0u64, 0u64, 0u64);
    for row in load(&format!("{DIR}/negative_walks.json"))
        .as_array()
        .unwrap()
    {
        let p = row["p"].as_u64().unwrap();
        for w in row["walks"].as_array().unwrap() {
            let key = (
                p,
                w["karatsuba"].as_bool().unwrap(),
                "Order24Built".to_string(),
                w["run"]["seed"].as_u64().unwrap(),
            );
            nw += 1;
            ndiff += u64::from(w["run"]["visit_digest"] != walks[&key]["run"]["visit_digest"]);
            ncorrect += u64::from(w["run"]["correct"].as_bool().unwrap());
        }
    }

    // Per-arm JSON and the per-step table.
    let mut per_arm = serde_json::Map::new();
    let mut md_steps = Vec::new();
    for kar in [false, true] {
        md_steps.push(format!(
            "\n## Per-step costs, {} (means over 16 walks)\n\n| p | arm | steps | muls/step | rep muls/call | native/step | cmp | neg | modn | hash | table | rep cmp/call | rep neg/call | rep modn/call |\n|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|",
            lab(kar)
        ));
        for p in PS {
            for arm in ARMS {
                let s = st(p, kar, arm);
                let k = |n: &str| s.by_kind[n] / s.steps;
                let rk = |n: &str| s.rep_by_kind[n] / s.rep_calls;
                md_steps.push(format!(
                    "| {p} | {arm} | {:.0} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
                    s.steps, s.muls / s.steps, s.rep_muls / s.rep_calls, s.native / s.steps,
                    k("cmp"), k("neg"), k("modn"), k("hash"), k("table"), rk("cmp"), rk("neg"), rk("modn")
                ));
                let by_kind: serde_json::Map<String, Value> =
                    KINDS.iter().map(|n| (n.to_string(), json!(k(n)))).collect();
                let rep_kind: serde_json::Map<String, Value> = ["cmp", "neg", "modn"]
                    .iter()
                    .map(|n| (n.to_string(), json!(rk(n))))
                    .collect();
                per_arm.insert(
                    format!("{p}|{}|{arm}", lab(kar)),
                    json!({
                        "walks": s.walks, "all_correct": s.all_correct, "steps": s.steps,
                        "muls": s.muls, "muls_per_step": s.muls / s.steps,
                        "native": s.native, "native_per_step": s.native / s.steps,
                        "native_per_step_by_kind": by_kind,
                        "rep_calls": s.rep_calls, "rep_muls_per_call": s.rep_muls / s.rep_calls,
                        "rep_ops_per_call": rep_kind,
                        "table_fp_muls": s.table_muls,
                    }),
                );
            }
        }
    }

    let summary = json!({
        "experiment": "E34", "note": "§9.38/§9.39", "curves": PS, "gae": GAE,
        "gae_karatsuba_own": GAE_KARATSUBA_OWN, "weights": WEIGHTS,
        "band_definition": "cost(w) = mean F_p multiplications (fp_muls - verify_fp_muls) + w * GAE * mean native operations (cmp+neg+modn+hash+table); one_per_step replaces native operations by steps (the E29 validator's convention); pooled = sum over curves of cost/sqrt(n); ratio = order 6 over order 24",
        "e29_replay": {"records": rec, "identical": same, "differences": replay_diffs},
        "walk_identity": {"compared": cmp_n, "identical": cmp_same, "fields": WALK_FIELDS, "differences": walk_diffs},
        "digests_equal_across_arithmetics": digests_across_arith,
        "identity_checks": id_json,
        "negative_control_walks": {"walks": nw, "digest_differs": ndiff, "correct": ncorrect},
        "per_arm": per_arm, "band": band, "strongest": strongest,
    });
    fs::write(
        Path::new(DIR).join("summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();

    let mut md = vec![
        "# E34 tables (§9.39)\n".to_string(),
        "Generated by `examples/e34_tables.rs` from this directory's records and E29's Stage R records.\n".to_string(),
        format!(
            "E29 replay: {same}/{rec} records identical in every field (E29's order-24 records carry no visit digest). Walk identity: {cmp_same}/{cmp_n} cheap walks equal their built walk in {}. Visit digests equal across arithmetics: {digests_across_arith}. Negative-control walks (omitted candidate, p = 241, schoolbook): digest differs on {ndiff}/{nw} (all {ncorrect} still recover the log).\n",
            WALK_FIELDS.join(", ")
        ),
        "## Identity checks (every class member of each point, against E29's representative)\n\n| arm | curves | members | mismatches | forced x₀ = 0 points | x = 0 points | scalar checks (failures) |\n|:--|--:|--:|--:|--:|--:|--:|".to_string(),
    ];
    md.extend(id_md);
    md.extend(md_steps);
    md.push("\n## The §5a band: order 6 / order 24 (> 1: order 24 cheaper)\n".to_string());
    md.extend(md_band);
    md.push("## Strongest walk of each order (pooled, every native operation priced, GAE 97)\n\n| arithmetic | GAE | order 6 | order 24 | strongest ratio | E18's walk / strongest | E18 handover shift (bits) |\n|:--|--:|:--|:--|--:|--:|--:|".to_string());
    md.extend(md_strong);
    fs::write(Path::new(DIR).join("tables.md"), md.join("\n") + "\n").unwrap();
    println!(
        "e34 tables: replay {same}/{rec}, walk identity {cmp_same}/{cmp_n}, negative {ndiff}/{nw}"
    );
}
