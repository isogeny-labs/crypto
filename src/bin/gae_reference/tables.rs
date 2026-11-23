//! `gae-reference retable` and `gae-reference reprice-ledger`: the tuned GAE
//! pricing tables and the re-pricing of the Round-5 ledger with them.
//!
//! **Definition.**  The legacy cost unit is one affine addition as
//! `ic_boundary::calibrate_group` times it (untuned `u128 %` arithmetic and an
//! inversion per addition).  The tuned unit (`gae_definition =
//! "tuned-batched-affine-add/v1"`) is one addition of the tuned step this tool
//! measures: Montgomery or carry-less field arithmetic and one shared
//! inversion per 256 lanes.  Every table entry is a ratio of a native
//! operation to that unit, so redefining the unit rescales each entry by
//! `R = ns_add_reference / ns_add_tuned` (the sweep) and by `1 / F`, the
//! speedup of the kernel that operation itself runs on, if it has one:
//!
//! ```text
//! ratio_tuned(unit) = ratio_old(unit) x R / F(unit)
//! (a) counted group operation          (double)            unchanged
//! (b) same untuned field arithmetic    F = max(1, measured speedup) (field.jsonl)
//! (c) tuned or field-independent       F = 1
//! ```
//!
//! The old ratio was measured on another host; multiplying it by this host's
//! `R` and `F` assumes that the ratio of the unit's time to the reference
//! addition's time carries across hosts.  `retable` records, per unit, how far
//! this host's own reference ratio is from the legacy one (`host_drift`).
//!
//! No arithmetic here is Python; the scripts that lay results out only read
//! the JSON this writes.

use std::collections::BTreeMap;

use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use serde_json::{json, Map, Value};

pub const GAE_DEFINITION: &str = "tuned-batched-affine-add/v1";

/// Kernel class of a native unit; see the module docs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    A,
    B,
    C,
}

impl Class {
    fn letter(self) -> &'static str {
        match self {
            Class::A => "a",
            Class::B => "b",
            Class::C => "c",
        }
    }
}

/// Class, the `field.jsonl` key of the speedup that applies (class b only),
/// and the reason, for a unit named without the `ns_per_` prefix.
pub fn classify(unit: &str) -> Option<(Class, Option<&'static str>, &'static str)> {
    Some(match unit {
        "double" => (
            Class::A,
            None,
            "counted group operation: a doubling is one reference addition's work and stays about 1 GAE",
        ),
        "sqrt" => (
            Class::B,
            Some("f_sqrt"),
            "PrimeCurve::sqrt: powmod/Tonelli-Shanks over mulmod (u128 %), the reference addition's arithmetic",
        ),
        "inversion" | "field_inv" => (
            Class::B,
            Some("f_inv_batched"),
            "invmod / Gf2::inv, the reference addition's inversion; independent call sites share one inversion",
        ),
        "row_op" => (
            Class::B,
            Some("f_row_op"),
            "calibrate_row_ops: mulmod/submod (u128 %) modulo the subgroup order",
        ),
        "legendre" => (
            Class::C,
            None,
            "Jacobi symbol: integer remainders and shifts, no field multiplication",
        ),
        "lookup" => (Class::C, None, "hash probe, field-independent"),
        "word_xor" => (Class::C, None, "64-bit XOR, field-independent"),
        "as_solve" => (
            Class::C,
            None,
            "Artin-Schreier solve by a precomputed linear map, field-independent",
        ),
        "s4_pair" => (
            Class::C,
            None,
            "SubspaceOracle::decompose: pclmulqdq Gf2 products with Gf2::batch_inv, already the tuned kernel",
        ),
        "frobenius" => (
            Class::C,
            None,
            "FastCurve::frobenius_k: Gf2::sqr, the tuned addition's own kernel",
        ),
        "canon" => (
            Class::C,
            None,
            "FrobeniusCanon: Gf2 squarings and comparisons, the tuned kernel",
        ),
        "field_mul" | "field_sqr" => (
            Class::C,
            None,
            "Gf2::mul / Gf2::sqr, the kernel the tuned addition calls",
        ),
        _ => return None,
    })
}

/// `ratio_tuned = ratio_old x R / F` (class b), `ratio_old x R` (class c),
/// `ratio_old` (class a).
pub fn tuned_ratio(class: Class, old: f64, r: f64, f: f64) -> f64 {
    match class {
        Class::A => old,
        Class::B => old * r / f,
        Class::C => old * r,
    }
}

fn num(v: &Value, k: &str) -> Result<f64, String> {
    v.get(k)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("missing number `{k}`"))
}

/// A JSONL file's objects keyed by the curve label (`curve` up to ` [`).
fn by_label(text: &str) -> Result<BTreeMap<String, Value>, String> {
    let mut out = BTreeMap::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        let curve = v
            .get("curve")
            .and_then(Value::as_str)
            .ok_or("line without `curve`")?;
        let label = curve.split(" [").next().unwrap_or(curve).to_string();
        out.insert(label, v);
    }
    Ok(out)
}

/// Rows of one legacy table re-expressed in the tuned unit.
pub struct Retabled {
    pub instances: Map<String, Value>,
    pub derivation: Map<String, Value>,
    /// Per unit: the host drifts of every row that has one.
    pub drifts: BTreeMap<String, Vec<f64>>,
}

/// Re-express every row of `legacy["instances"]` (calibration.json or
/// full_prices.json shape).  `sweep` and `field` are keyed by curve label.
pub fn retable_rows(
    legacy: &Value,
    sweep: &BTreeMap<String, Value>,
    field: &BTreeMap<String, Value>,
) -> Result<Retabled, String> {
    let rows = legacy
        .get("instances")
        .and_then(Value::as_object)
        .ok_or("legacy table has no `instances`")?;
    let mut out = Retabled {
        instances: Map::new(),
        derivation: Map::new(),
        drifts: BTreeMap::new(),
    };
    for (key, row) in rows {
        let row = row
            .as_object()
            .ok_or(format!("{key}: row is not an object"))?;
        let sw = sweep.get(key).ok_or(format!("{key}: no sweep line"))?;
        let fl = field.get(key).ok_or(format!("{key}: no field line"))?;
        let r = num(sw, "ratio_reference_over_tuned")?;
        let local = fl.get("host_local_ns").and_then(Value::as_object);
        let mut new_row = Map::new();
        let mut units = Map::new();
        for (name, v) in row {
            let Some(old) = v.as_f64() else {
                new_row.insert(name.clone(), v.clone());
                continue;
            };
            let unit = name.strip_prefix("ns_per_").unwrap_or(name);
            let (class, fkey, why) =
                classify(unit).ok_or(format!("{key}: unit `{name}` has no classification"))?;
            let f_measured = match fkey {
                Some(k) => Some(num(fl, k).map_err(|e| format!("{key}: field.jsonl {e}"))?),
                None => None,
            };
            // A tuned unit never pays more than the reference kernel would:
            // where the tuned kernel measured slower, the reference kernel
            // is the tuned kernel and F = 1.
            let f = f_measured.map(|m| m.max(1.0));
            let new = tuned_ratio(class, old, r, f.unwrap_or(1.0));
            // How far this host's reference ratio is from the legacy one.
            let drift = local.and_then(|l| {
                let add = l.get("add")?.as_f64()?;
                let ns = l.get(unit)?.as_f64()?;
                Some(ns / add / old)
            });
            if let Some(d) = drift {
                out.drifts.entry(unit.to_string()).or_default().push(d);
            }
            let mut d = Map::new();
            d.insert("class".into(), json!(class.letter()));
            d.insert("why".into(), json!(why));
            d.insert("old".into(), json!(old));
            d.insert(
                "R".into(),
                json!(if class == Class::A {
                    Value::Null
                } else {
                    json!(r)
                }),
            );
            d.insert("F".into(), json!(f));
            d.insert("F_measured".into(), json!(f_measured));
            d.insert("new".into(), json!(new));
            d.insert("host_drift".into(), json!(drift));
            if unit == "inversion" || unit == "field_inv" {
                let single = num(fl, "f_inv_single")?.max(1.0);
                d.insert(
                    "new_if_inversions_not_batched".into(),
                    json!(tuned_ratio(class, old, r, single)),
                );
            }
            if unit == "field_mul" || unit == "field_sqr" {
                let k = if unit == "field_mul" {
                    "f_mul_lanes"
                } else {
                    "f_sqr_lanes"
                };
                if let Some(fm) = fl.get(k).and_then(Value::as_f64) {
                    d.insert(
                        "new_if_lane_throughput".into(),
                        json!(tuned_ratio(Class::B, old, r, fm)),
                    );
                }
            }
            units.insert(name.clone(), Value::Object(d));
            new_row.insert(name.clone(), json!(new));
        }
        out.derivation.insert(
            key.clone(),
            json!({
                "R": r,
                "ns_per_add_reference": sw.get("ns_per_add_reference"),
                "ns_per_add_tuned": sw.get("ns_per_add_tuned"),
                "units": units,
            }),
        );
        out.instances.insert(key.clone(), Value::Object(new_row));
    }
    Ok(out)
}

fn summarise(v: &[f64]) -> Value {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    json!({"rows": s.len(), "min": s[0], "median": s[s.len() / 2], "max": s[s.len() - 1]})
}

fn sensitivity(drifts: &BTreeMap<String, Vec<f64>>) -> Value {
    let mut m = Map::new();
    for (u, v) in drifts {
        if !v.is_empty() {
            m.insert(u.clone(), summarise(v));
        }
    }
    Value::Object(m)
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))
}

fn source(path: &str, text: &str) -> Value {
    json!({"path": path, "sha256": sha256_hex(text.as_bytes())})
}

fn flag(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn write_json(path: &str, v: &Value) -> Result<(), String> {
    let mut s = serde_json::to_string_pretty(v).map_err(|e| e.to_string())?;
    s.push('\n');
    std::fs::write(path, s).map_err(|e| format!("{path}: {e}"))
}

const ASSUMPTION: &str = "Legacy ratios were measured on another host (calibration.json: Intel Xeon @ 2.10 GHz; the icv1-f2m23-t5197-69e76b73 row: Apple M4 Pro under load). The tuned ratio multiplies them by this host's R = ns_add_reference / ns_add_tuned and by the class (b) speedup F measured here, which assumes that (unit time) / (reference addition time) carries across hosts; derivation.<row>.units.<unit>.host_drift is this host's own ratio divided by the legacy one, and `sensitivity` summarises it.";

pub fn retable_cli(args: &[String]) -> i32 {
    match retable(args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("retable: {e}");
            1
        }
    }
}

fn retable(args: &[String]) -> Result<(), String> {
    let cal_path = flag(args, "--calibration", "docs/ic/calibration.json");
    let full_path = flag(
        args,
        "--full-prices",
        "src/cryptanalysis/ecbench/full_prices.json",
    );
    let sweep_path = flag(
        args,
        "--sweep",
        "research/tuned_gae_20261020/sweep/gae_reference.jsonl",
    );
    let field_path = flag(
        args,
        "--field",
        "research/tuned_gae_20261020/sweep/field.jsonl",
    );
    let out_cal = flag(args, "--out-calibration", "docs/ic/calibration_tuned.json");
    let out_full = flag(
        args,
        "--out-full-prices",
        "src/cryptanalysis/ecbench/full_prices_tuned.json",
    );
    let (cal_t, full_t, sweep_t, field_t) = (
        read(&cal_path)?,
        read(&full_path)?,
        read(&sweep_path)?,
        read(&field_path)?,
    );
    let cal: Value = serde_json::from_str(&cal_t).map_err(|e| e.to_string())?;
    let full: Value = serde_json::from_str(&full_t).map_err(|e| e.to_string())?;
    let sweep = by_label(&sweep_t)?;
    let field = by_label(&field_t)?;
    let host = sweep
        .values()
        .next()
        .and_then(|v| v.get("host_cpu_model").cloned())
        .unwrap_or(Value::Null);
    let sources = json!({
        "legacy_calibration": source(&cal_path, &cal_t),
        "legacy_full_prices": source(&full_path, &full_t),
        "sweep": source(&sweep_path, &sweep_t),
        "field": source(&field_path, &field_t),
    });

    let c = retable_rows(&cal, &sweep, &field)?;
    let doc = json!({
        "gae_definition": GAE_DEFINITION,
        "what": "Calibration ratios of the boundary ledger re-expressed in the tuned unit: nanoseconds per native operation divided by nanoseconds per tuned batched affine addition. Same schema as calibration.json; not read by Calibration::pin until the switch in research/tuned_gae_20261020/SWITCH_PLAN.md is made.",
        "unit": "ns_per_<native unit> / ns_per_tuned_add, dimensionless (double stays a counted group operation)",
        "method": "ratio_new = ratio_old x R / F; R = reference/tuned addition on this host (sweep), F = measured field speedup for class (b) units, 1 for class (c), class (a) unchanged. Generated by `gae-reference retable`.",
        "host": host,
        "legacy_host": cal.get("host"),
        "assumption": ASSUMPTION,
        "flags": {"icv1-f2m23-t5197-69e76b73": "legacy ratios from an Apple M4 Pro under load 30-150, not the Xeon of every other row"},
        "sources": sources,
        "fallback": cal.get("fallback"),
        "regenerate": "gae-reference field --from-calibration > research/tuned_gae_20261020/sweep/field.jsonl; gae-reference retable",
        "sensitivity": sensitivity(&c.drifts),
        "instances": c.instances,
        "derivation": c.derivation,
    });
    write_json(&out_cal, &doc)?;

    let f = retable_rows(&full, &sweep, &field)?;
    let fdoc = json!({
        "schema": "ecbench.full_prices/v1",
        "gae_definition": GAE_DEFINITION,
        "what": "Ratios to one tuned batched affine addition of every native unit the *_full ecbench methods price. Same schema as full_prices.json; not read by full_price.rs until the switch in research/tuned_gae_20261020/SWITCH_PLAN.md is made.",
        "unit": "ns per native unit / ns per tuned affine addition on the same curve, dimensionless",
        "method": "ratio_new = ratio_old x R / F, as docs/ic/calibration_tuned.json. Generated by `gae-reference retable`.",
        "host": host,
        "assumption": ASSUMPTION,
        "sources": sources,
        "sensitivity": sensitivity(&f.drifts),
        "instances": f.instances,
        "derivation": f.derivation,
    });
    write_json(&out_full, &fdoc)?;
    eprintln!(
        "wrote {out_cal} ({} rows) and {out_full} ({} rows)",
        c.instances.len(),
        f.instances.len()
    );
    Ok(())
}

// ── Re-pricing the ledger ───────────────────────────────────────────────

/// Counter of a ledger phase and the table unit that prices it
/// (`ic_boundary::price_phase`).
const COUNTERS: &[(&str, &str)] = &[
    ("sqrt_solves", "sqrt"),
    ("as_solves", "as_solve"),
    ("s4_pairs", "s4_pair"),
    ("lookups", "lookup"),
    ("row_ops", "row_op"),
    ("legendre_symbols", "legendre"),
    ("inversions", "inversion"),
    ("frobenius_maps", "frobenius"),
    ("canonicalisations", "canon"),
    ("target_guard_probes", "lookup"),
];

const PHASES: &[&str] = &["factor_base", "relations", "linear_algebra", "verify"];

/// The ratio a table prices `unit` at for one ledger instance: the table's
/// entry, else the factor the run measured (`price_phase` keeps those), else
/// `None` (unpriced).
fn ratio_of(row: &Map<String, Value>, calib: &Value, unit: &str) -> Option<f64> {
    let name = format!("ns_per_{unit}");
    if let Some(v) = row.get(&name).and_then(Value::as_f64) {
        return Some(v);
    }
    let ns = calib.get(&name)?.as_f64()?;
    let add = calib.get("ns_per_add")?.as_f64()?;
    Some(ns / add)
}

/// One variant's phases priced by a table: total GAE and, per unit, the GAE
/// that unit contributed.
fn price_variant(
    v: &Value,
    row: &Map<String, Value>,
    calib: &Value,
) -> (f64, BTreeMap<String, (f64, f64)>) {
    let mut total = 0.0;
    let mut by_unit: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    for ph in PHASES {
        let phase = &v[*ph];
        let ops = &phase["group_ops"];
        total += ops["adds"].as_f64().unwrap_or(0.0) + ops["doubles"].as_f64().unwrap_or(0.0);
        for (counter, unit) in COUNTERS {
            let count = phase["native"][*counter].as_f64().unwrap_or(0.0);
            if count == 0.0 {
                continue;
            }
            let g = ratio_of(row, calib, unit).map_or(0.0, |r| count * r);
            total += g;
            let e = by_unit.entry((*unit).to_string()).or_default();
            e.0 += count;
            e.1 += g;
        }
    }
    (total, by_unit)
}

/// Re-price every prime and char2 instance of the ledger with the legacy and
/// the tuned table; `tolerance` is the relative error allowed between the
/// legacy recomputation and the recorded `s`.
pub fn reprice(
    ledger: &Value,
    matched: &Value,
    legacy: &Value,
    tuned: &Value,
    tolerance: f64,
) -> Result<Value, String> {
    let rho: BTreeMap<String, f64> = matched["instances"]
        .as_array()
        .ok_or("matched rho file has no instances")?
        .iter()
        .filter_map(|i| {
            Some((
                i["instance"].as_str()?.to_string(),
                i["matched_reference"]["mean_s"].as_f64()?,
            ))
        })
        .collect();
    let table = |t: &Value, key: &str| -> Result<Map<String, Value>, String> {
        t["instances"][key]
            .as_object()
            .cloned()
            .ok_or(format!("no row `{key}` in a table"))
    };
    let mut rows = Vec::new();
    let mut mismatches = Vec::new();
    for inst in ledger["ledger"]["instances"]
        .as_array()
        .ok_or("ledger has no instances")?
    {
        let regime = inst["regime"].as_str().unwrap_or("");
        if regime != "prime" && regime != "char2" {
            continue;
        }
        let name = inst["curve"]["name"]
            .as_str()
            .ok_or("instance without name")?;
        let key = format!("{regime}/{name}");
        let (lrow, trow) = (table(legacy, &key)?, table(tuned, &key)?);
        let calib = &inst["calibration"];
        let sqrt_r = inst["r"].as_f64().ok_or("no r")?.sqrt();
        let rho_s = *rho.get(name).ok_or(format!("{name}: no matched rho"))?;

        struct Acc {
            rec: Vec<f64>,
            leg: Vec<f64>,
            tun: Vec<f64>,
            by_unit: BTreeMap<String, (f64, f64, f64)>,
            n: usize,
        }
        let mut variants: BTreeMap<String, Acc> = BTreeMap::new();
        for v in inst["variants"].as_array().ok_or("no variants")? {
            let vname = v["name"]
                .as_str()
                .ok_or("variant without name")?
                .to_string();
            let (lg, lu) = price_variant(v, &lrow, calib);
            let (tg, tu) = price_variant(v, &trow, calib);
            let a = variants.entry(vname).or_insert(Acc {
                rec: vec![],
                leg: vec![],
                tun: vec![],
                by_unit: BTreeMap::new(),
                n: 0,
            });
            a.rec.push(num(v, "s")?);
            a.leg.push(lg / sqrt_r);
            a.tun.push(tg / sqrt_r);
            a.n += 1;
            for (u, (c, g)) in &lu {
                let e = a.by_unit.entry(u.clone()).or_default();
                e.0 += c;
                e.1 += g;
            }
            for (u, (_, g)) in &tu {
                a.by_unit.entry(u.clone()).or_default().2 += g;
            }
        }
        let mean = |x: &[f64]| x.iter().sum::<f64>() / x.len() as f64;
        let mut recipes = Vec::new();
        for (vname, a) in &variants {
            let rel = a
                .rec
                .iter()
                .zip(&a.leg)
                .map(|(r, l)| ((l - r) / r).abs())
                .fold(0.0, f64::max);
            let reproduced = rel <= tolerance;
            if !reproduced {
                mismatches.push(json!({
                    "instance": name, "variant": vname,
                    "recorded_s_mean": mean(&a.rec), "recomputed_s_mean": mean(&a.leg),
                    "max_relative_error": rel,
                }));
            }
            let n = a.n as f64;
            let mut drivers: Vec<Value> = a
                .by_unit
                .iter()
                .map(|(u, (c, lg, tg))| {
                    json!({
                        "unit": u,
                        "count_mean": c / n,
                        "legacy_s_part": lg / n / sqrt_r,
                        "tuned_s_part": tg / n / sqrt_r,
                        "delta_s": (tg - lg) / n / sqrt_r,
                    })
                })
                .collect();
            drivers.sort_by(|x, y| {
                y["delta_s"]
                    .as_f64()
                    .unwrap_or(0.0)
                    .abs()
                    .partial_cmp(&x["delta_s"].as_f64().unwrap_or(0.0).abs())
                    .expect("finite")
            });
            recipes.push((
                vname.clone(),
                json!({
                    "variant": vname,
                    "repetitions": a.n,
                    "legacy_s_recorded": mean(&a.rec),
                    "legacy_s_recomputed": mean(&a.leg),
                    "legacy_reproduced": reproduced,
                    "legacy_max_relative_error": rel,
                    "tuned_s": mean(&a.tun),
                    "tuned_over_legacy": mean(&a.tun) / mean(&a.leg),
                    "legacy_gap_to_rho": mean(&a.rec) / rho_s,
                    "tuned_gap_to_rho": mean(&a.tun) / rho_s,
                    "drivers": drivers,
                }),
            ));
        }
        let best = |f: &dyn Fn(&Value) -> f64| -> Value {
            recipes
                .iter()
                .min_by(|a, b| f(&a.1).partial_cmp(&f(&b.1)).expect("finite"))
                .map(|r| r.1.clone())
                .unwrap_or(Value::Null)
        };
        let best_legacy = best(&|r| r["legacy_s_recorded"].as_f64().unwrap_or(f64::MAX));
        let best_tuned = best(&|r| r["tuned_s"].as_f64().unwrap_or(f64::MAX));
        rows.push(json!({
            "instance": name,
            "regime": regime,
            "table_row": key,
            "r": inst["r"],
            "log2_r": inst["log2_r"],
            "matched_rho_s": rho_s,
            "matched_rho_s_tuned": rho_s,
            "legacy_best": best_legacy,
            "tuned_best": best_tuned,
            "recipes": recipes.into_iter().map(|r| r.1).collect::<Vec<_>>(),
        }));
    }
    Ok(json!({
        "gae_definition": GAE_DEFINITION,
        "reproduced_all": mismatches.is_empty(),
        "tolerance": tolerance,
        "mismatches": mismatches,
        "instances": rows,
    }))
}

pub fn reprice_cli(args: &[String]) -> i32 {
    match reprice_run(args) {
        Ok(ok) => i32::from(!ok) * 3,
        Err(e) => {
            eprintln!("reprice-ledger: {e}");
            1
        }
    }
}

/// Returns whether the legacy recomputation reproduced every recorded S.
fn reprice_run(args: &[String]) -> Result<bool, String> {
    let ledger_p = flag(
        args,
        "--ledger",
        "docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json",
    );
    let matched_p = flag(
        args,
        "--matched-rho",
        "research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json",
    );
    let legacy_p = flag(args, "--legacy", "docs/ic/calibration.json");
    let tuned_p = flag(args, "--tuned", "docs/ic/calibration_tuned.json");
    let out_p = flag(
        args,
        "--out",
        "research/tuned_gae_20261020/reprice/table_b.json",
    );
    let tol: f64 = flag(args, "--tolerance", "1e-9")
        .parse()
        .map_err(|e| format!("--tolerance: {e}"))?;
    let (lt, mt, lg, tn) = (
        read(&ledger_p)?,
        read(&matched_p)?,
        read(&legacy_p)?,
        read(&tuned_p)?,
    );
    let parse = |t: &str| serde_json::from_str::<Value>(t).map_err(|e| e.to_string());
    let mut out = reprice(&parse(&lt)?, &parse(&mt)?, &parse(&lg)?, &parse(&tn)?, tol)?;
    out["sources"] = json!({
        "ledger": source(&ledger_p, &lt),
        "matched_rho": source(&matched_p, &mt),
        "legacy_calibration": source(&legacy_p, &lg),
        "tuned_calibration": source(&tuned_p, &tn),
    });
    if let Some(parent) = std::path::Path::new(&out_p).parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json(&out_p, &out)?;
    let ok = out["reproduced_all"].as_bool().unwrap_or(false);
    eprintln!(
        "wrote {out_p}; legacy S reproduced for every recipe: {ok} ({} mismatches)",
        out["mismatches"].as_array().map_or(0, Vec::len)
    );
    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_tables() -> (Value, BTreeMap<String, Value>, BTreeMap<String, Value>) {
        let legacy = json!({"instances": {"prime/toy": {
            "ns_per_double": 1.1, "ns_per_sqrt": 0.6, "ns_per_lookup": 0.02, "ns_per_legendre": 0.5,
            "ns_per_row_op": 0.02}}});
        let sweep = by_label(
            r#"{"curve":"prime/toy [x]","ratio_reference_over_tuned":4.0,"ns_per_add_reference":200.0,"ns_per_add_tuned":50.0}"#,
        )
        .unwrap();
        let field = by_label(
            r#"{"curve":"prime/toy [x]","f_sqrt":2.0,"f_inv_batched":6.0,"f_inv_single":1.0,"f_row_op":0.8,"host_local_ns":{"add":200.0,"sqrt":130.0}}"#,
        )
        .unwrap();
        (legacy, sweep, field)
    }

    #[test]
    fn retable_scales_each_class_by_its_rule() {
        let (legacy, sweep, field) = toy_tables();
        let t = retable_rows(&legacy, &sweep, &field).unwrap();
        let row = t.instances["prime/toy"].as_object().unwrap();
        // (a) the doubling is not scaled.
        assert_eq!(row["ns_per_double"].as_f64().unwrap(), 1.1);
        // (b) sqrt: 0.6 x R / F = 0.6 x 4 / 2.
        assert!((row["ns_per_sqrt"].as_f64().unwrap() - 1.2).abs() < 1e-12);
        // (b) with a tuned kernel that measured slower: F clamps to 1.
        assert!((row["ns_per_row_op"].as_f64().unwrap() - 0.08).abs() < 1e-12);
        let d = &t.derivation["prime/toy"]["units"]["ns_per_row_op"];
        assert_eq!(d["F"].as_f64(), Some(1.0));
        assert_eq!(d["F_measured"].as_f64(), Some(0.8));
        // (c) lookup and Legendre: x R.
        assert!((row["ns_per_lookup"].as_f64().unwrap() - 0.08).abs() < 1e-12);
        assert!((row["ns_per_legendre"].as_f64().unwrap() - 2.0).abs() < 1e-12);
        let d = &t.derivation["prime/toy"]["units"]["ns_per_sqrt"];
        assert_eq!(d["class"], "b");
        assert_eq!(d["R"].as_f64(), Some(4.0));
        assert_eq!(d["F"].as_f64(), Some(2.0));
        // This host's ratio 130/200 against the legacy 0.6.
        assert!((d["host_drift"].as_f64().unwrap() - 0.65 / 0.6).abs() < 1e-12);
        assert_eq!(
            t.derivation["prime/toy"]["units"]["ns_per_double"]["class"],
            "a"
        );
    }

    #[test]
    fn every_table_unit_has_a_class() {
        for u in [
            "double",
            "sqrt",
            "legendre",
            "inversion",
            "row_op",
            "lookup",
            "word_xor",
            "as_solve",
            "s4_pair",
            "frobenius",
            "canon",
            "field_mul",
            "field_sqr",
            "field_inv",
        ] {
            assert!(classify(u).is_some(), "{u}");
        }
        assert!(classify("nonsense").is_none());
    }

    /// One prime instance, two reps of one variant: GAE = adds + doubles +
    /// sqrt_solves x 0.6 + row_ops x 0.02 (+ lookups at the run's measured
    /// factor, which neither table carries).
    fn toy_ledger(recorded_s: f64) -> (Value, Value, Value, Value) {
        let phase = |sq: u64| {
            json!({"group_ops":{"adds":10,"doubles":5,"scalar_mults":0},
                   "native":{"sqrt_solves":sq,"row_ops":100,"lookups":10}})
        };
        let empty = json!({"group_ops":{"adds":0,"doubles":0,"scalar_mults":0},"native":{}});
        let var = |sq| {
            json!({"name":"v","s":recorded_s,"factor_base":phase(sq),"relations":empty,
                   "linear_algebra":empty,"verify":empty})
        };
        let ledger = json!({"ledger":{"instances":[{
            "regime":"prime","curve":{"name":"toy"},"r":100.0,"log2_r":6.6,
            "calibration":{"ns_per_add":200.0,"ns_per_lookup":10.0},
            "variants":[var(20), var(20)]}]}});
        let matched = json!({"instances":[{"instance":"toy","matched_reference":{"mean_s":2.0}}]});
        let legacy = json!({"instances":{"prime/toy":{"ns_per_sqrt":0.6,"ns_per_row_op":0.02}}});
        let tuned = json!({"instances":{"prime/toy":{"ns_per_sqrt":1.2,"ns_per_row_op":0.08}}});
        (ledger, matched, legacy, tuned)
    }

    #[test]
    fn legacy_reproduction_is_asserted_and_tuned_reprices() {
        // GAE = 15 + 20 x 0.6 + 100 x 0.02 + 10 x (10 / 200) = 29.5; S = 2.95.
        let (ledger, matched, legacy, tuned) = toy_ledger(2.95);
        let out = reprice(&ledger, &matched, &legacy, &tuned, 1e-9).unwrap();
        assert_eq!(out["reproduced_all"], true);
        let best = &out["instances"][0]["legacy_best"];
        assert!((best["legacy_s_recomputed"].as_f64().unwrap() - 2.95).abs() < 1e-12);
        // Tuned: 15 + 24 + 8 + 0.5 = 47.5; S = 4.75; matched rho unchanged.
        assert!((best["tuned_s"].as_f64().unwrap() - 4.75).abs() < 1e-12);
        assert_eq!(
            out["instances"][0]["matched_rho_s_tuned"].as_f64(),
            Some(2.0)
        );
        assert!((best["tuned_gap_to_rho"].as_f64().unwrap() - 2.375).abs() < 1e-12);
        let drivers = best["drivers"].as_array().unwrap();
        assert_eq!(drivers[0]["unit"], "sqrt");
        // A recorded S that the table cannot produce is reported, not hidden.
        let (ledger, ..) = toy_ledger(3.0);
        let out = reprice(&ledger, &matched, &legacy, &tuned, 1e-9).unwrap();
        assert_eq!(out["reproduced_all"], false);
        assert_eq!(out["mismatches"].as_array().unwrap().len(), 1);
    }
}
