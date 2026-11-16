//! The frozen protocol set, its deterministic file list and its replay.

use std::fs;
use std::path::Path;
use std::time::Instant;

use num_bigint::BigUint;
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::curve_traits::{read_hints, Factoring};
use crypto_lib::cryptanalysis::jacobian_transfer::cost::CostModel;

use crate::certificate::{Certificate, SCHEMA, TOOL};
use crate::controls::{
    hec_ic_control, subfield_subgroup_instance, synthetic_ghs_control, EXTENDED_PLAN, PROTOCOL_PLAN,
};
use crate::ctx::Ctx;
use crate::spec::{load_registered, load_registered_from, toy_koblitz, Instance};
use crate::{cover, ghs, isogeny, pairing, Common};

/// Registered curves of the protocol (names resolved by the registry's
/// `standard_names`; the certificate records the ICV1 slug).
const REGISTERED: [(&str, &str); 6] = [
    ("sect113r1", "sect113r1"),
    ("ECC2K-130", "ecc2k-130"),
    ("K-163", "k-163"),
    ("B-163", "b-163"),
    ("P-192", "p-192"),
    ("secp256k1", "secp256k1"),
];

/// Registered pairing-friendly curves audited as end-to-end controls.
///
/// Positive controls (role `control`): the MNT k = 3 example (a curve designed
/// so that the pairing route is the attack; `class-weak` is expected) and
/// BLS12-381 (k = 12, where the formula band straddles rho; no weakness).
///
/// Boundary controls (role `boundary-control`), added with the margin rule of
/// Amendment 3: they sit on both sides of the two lines that decided a
/// verdict.  BN254 is the deployed k = 12 curve the review named (price above
/// rho); BN318 read `class-weak` on a -3-bit sign before the margin (inside
/// the margin: no weakness); BLS12-638 is just inside the 8-bit margin; BN350
/// is the smallest curve past it; BN446 is the registry curve nearest to
/// BN462 (the review's other example; there is no BN462 in the registry).
///
/// `(lookup, display name, directory, trait record read from the committed
/// docs/curves/traits.json, role)`.
const CONTROL_CURVES: [(&str, &str, &str, bool, &str); 7] = [
    // The live curve_traits::compute of mnt/mnt1 takes about six minutes (not
    // in this tool's code), so its trait record is read from the committed
    // docs/curves/traits.json, with order and trace checked against the instance.
    ("mnt/mnt1", "mnt/mnt1", "control-mnt1", true, "control"),
    (
        "icv1-fp381-tm15132376222941642751-961de9ba",
        "BLS12-381",
        "control-bls12-381",
        false,
        "control",
    ),
    (
        "bn/bn254",
        "BN254",
        "boundary-bn254",
        false,
        "boundary-control",
    ),
    (
        "bn/bn318",
        "BN318",
        "boundary-bn318",
        false,
        "boundary-control",
    ),
    (
        "bls/BLS12-638",
        "BLS12-638",
        "boundary-bls12-638",
        false,
        "boundary-control",
    ),
    (
        "bn/bn350",
        "BN350",
        "boundary-bn350",
        false,
        "boundary-control",
    ),
    (
        "bn/bn446",
        "BN446",
        "boundary-bn446",
        false,
        "boundary-control",
    ),
];

/// `(name, a, modulus)` of the toy Koblitz curves.
const TOYS: [(&str, u8, u64); 4] = [
    ("toy-k1-n15", 1, 0x8003),
    ("toy-k0-n15", 0, 0x8003),
    ("toy-k1-n21", 1, 0x20_0005),
    ("toy-k1-n35", 1, 0x8_0000_0005),
];

pub fn preset_lines() -> Vec<String> {
    let mut v = vec!["registered (docs/curves/registry.json):".to_string()];
    v.extend(REGISTERED.iter().map(|(n, _)| format!("  {n}")));
    v.push("toy Koblitz curves y^2+xy = x^3 + a x^2 + 1 (unregistered):".into());
    v.extend(
        TOYS.iter()
            .map(|(n, a, m)| format!("  {n}  a = {a}  modulus 0x{m:x}")),
    );
    v
}

pub struct Env {
    pub model: CostModel,
    pub fx: Factoring,
    pub hints: usize,
    pub seed: u64,
    pub transfers: Option<Value>,
    pub covers: Option<Value>,
    pub traits: Option<Value>,
}

pub fn build_env(common: &Common) -> Result<Env, String> {
    let hints = match fs::read_to_string(&common.hints) {
        Ok(text) => read_hints(&text)?,
        Err(_) => Vec::new(),
    };
    let n_hints = hints.len();
    let read_json = |p: &Path| -> Option<Value> {
        fs::read_to_string(p)
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
    };
    Ok(Env {
        model: CostModel::default(),
        fx: Factoring::new(common.budget, hints),
        hints: n_hints,
        seed: common.seed,
        transfers: read_json(&common.transfers),
        covers: read_json(&common.covers),
        traits: read_json(&common.traits),
    })
}

fn sanitize(id: &str) -> String {
    id.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn sha_hex(bytes: &[u8]) -> String {
    hex::encode(crypto_lib::hash::sha256(bytes))
}

fn pretty(v: &Value) -> String {
    let mut s = serde_json::to_string_pretty(v).expect("serializable");
    s.push('\n');
    s
}

/// Compare this certificate's route totals with the repository's
/// `jacobian-transfers.json` and attach the catalog cover row.
fn annotate(env: &Env, slug: &str, cert: &mut Value) {
    let id = cert["candidate"]["id"].as_str().unwrap_or("").to_string();
    if let Some(tf) = &env.transfers {
        let curve = tf["curves"]
            .as_array()
            .and_then(|a| a.iter().find(|c| c["slug"] == slug));
        if let Some(curve) = curve {
            // The cross-check compares the frozen formulas, so it uses the total
            // without any equivalence-class discount this tool adds on top
            // (Amendment 3); the two coincide when no discount applies.
            let frozen = cert["costs"]["log2_total_frozen_model"].as_f64();
            let reported = cert["costs"]["log2_total"].as_f64();
            let mine = frozen.or(reported);
            let route = if let Some(l) = id
                .strip_prefix("ghs:l=")
                .and_then(|s| s.parse::<u64>().ok())
            {
                let magic = cert["invariants"]["magic_number"].as_u64().unwrap_or(0);
                (magic >= 2)
                    .then(|| {
                        curve["routes"].as_array().and_then(|rs| {
                            rs.iter().find(|r| {
                                r["kind"] == "ghs_weil_descent"
                                    && r["invariants"]["base_degree"] == l
                            })
                        })
                    })
                    .flatten()
            } else if id.starts_with("cover:") && id != "cover:genus_sweep" {
                curve["routes"]
                    .as_array()
                    .and_then(|rs| rs.iter().find(|r| r["kind"] == "same_field_cover"))
            } else {
                None
            };
            if let (Some(route), Some(mine)) = (route, mine) {
                let theirs = route["cost"]["log2_total"].as_f64();
                if let Some(theirs) = theirs {
                    let delta = mine - theirs;
                    cert["invariants"]["crosscheck_jacobian_transfers"] = json!({
                        "file": "docs/curves/jacobian-transfers.json",
                        "route_kind": route["kind"],
                        "repo_log2_total": theirs,
                        "this_log2_total": mine,
                        "reported_total_is_lower_by_class_discount_bits": frozen.zip(reported).map(|(f, r)| ((f - r) * 1000.0).round() / 1000.0),
                        "delta_bits": (delta * 1000.0).round() / 1000.0,
                        "within_0_002_bits": delta.abs() <= 0.002,
                        "note": "a wiring check of the frozen formulas, not independent validation: this tool calls the repository's cost functions, so agreement shows the inputs were passed through, not that the formulas are right. The repository total is search + transfer + destination; this total also carries construction and one verification, both negligible against the destination at these sizes",
                    });
                }
            }
        }
    }
    if id.starts_with("cover:") && id != "cover:genus_sweep" {
        if let Some(cv) = &env.covers {
            let row = cv["curves"]
                .as_array()
                .and_then(|a| a.iter().find(|c| c["slug"] == slug));
            if let Some(row) = row {
                cert["invariants"]["catalog_cover_row"] = json!({
                    "file": "docs/curves/covers.json",
                    "status": row["status"],
                    "exists": row["exists"],
                    "subgroup_transfer": row["subgroup_transfer"],
                    "subfield_descent": row["subfield_descent"],
                });
            }
        }
    }
}

pub struct CurveRun {
    pub dir: String,
    pub files: Vec<(String, String)>,
    pub summary: Value,
}

/// Run every candidate family on one instance.
pub fn audit_instance(
    env: &Env,
    dir: &str,
    inst: &Instance,
    with_controls: bool,
    role: &str,
    committed_traits: bool,
) -> Result<CurveRun, String> {
    let committed = if committed_traits {
        let rec = env
            .traits
            .as_ref()
            .and_then(|t| t["curves"].as_array())
            .and_then(|a| a.iter().find(|c| c["slug"] == inst.rc.slug.as_str()))
            .ok_or_else(|| {
                format!(
                    "{}: no committed trait record (docs/curves/traits.json)",
                    inst.name
                )
            })?;
        Some(serde_json::from_value(rec.clone()).map_err(|e| format!("trait record: {e}"))?)
    } else {
        None
    };
    let ctx = Ctx::with_traits(inst, &env.model, &env.fx, env.seed, committed)?;
    let mut certs: Vec<Certificate> = Vec::new();
    let trace = std::env::var_os("TRANSFER_AUDIT_TRACE").is_some();
    let t0 = Instant::now();
    certs.extend(ghs::candidates(&ctx)?);
    if trace {
        eprintln!("trace {}: ghs {:?}", inst.name, t0.elapsed());
    }
    certs.extend(isogeny::candidates(&ctx)?);
    if trace {
        eprintln!("trace {}: isogeny {:?}", inst.name, t0.elapsed());
    }
    certs.extend(cover::candidates(&ctx)?);
    if trace {
        eprintln!("trace {}: cover {:?}", inst.name, t0.elapsed());
    }
    certs.extend(pairing::candidates(&ctx)?);
    if trace {
        eprintln!("trace {}: pairing {:?}", inst.name, t0.elapsed());
    }
    let mut files = Vec::new();
    let mut rows = Vec::new();
    let mut strongest = crate::certificate::Verdict::NoWeaknessFoundInScope;
    let emit = |cert: &Certificate,
                sub: &str,
                files: &mut Vec<(String, String)>|
     -> Result<Value, String> {
        let mut v = serde_json::to_value(cert).map_err(|e| e.to_string())?;
        annotate(env, &inst.rc.slug, &mut v);
        let name = format!(
            "{dir}/{sub}{}.json",
            sanitize(v["candidate"]["id"].as_str().unwrap_or("candidate"))
        );
        files.push((name.clone(), pretty(&v)));
        Ok(json!({
            "id": v["candidate"]["id"],
            "transfer_type": v["candidate"]["transfer_type"],
            "label": v["candidate"]["label"],
            "verdict": v["verdict"],
            "lead_class": v["lead_class"],
            "evaluation": v["evaluation"]["status"],
            "subgroup_status": v["subgroup_preservation"]["status"],
            "carried": v["carried"],
            "construction_status": v["map"]["construction_status"],
            "destination_solver": v["costs"]["destination_solver"],
            "log2_total": v["costs"]["log2_total"],
            "total_is_lower_bound": v["costs"]["total_is_lower_bound"],
            "log2_baseline": v["costs"]["log2_baseline"],
            "log2_cost_ratio": v["costs"]["log2_cost_ratio"],
            "repo_ratio_log2": v["costs"]["repo_ratio_log2"],
            "crosscheck_within_0_002_bits": v["invariants"]["crosscheck_jacobian_transfers"]["within_0_002_bits"],
            "file": name,
        }))
    };
    for cert in &certs {
        if cert.verdict.strength() > strongest.strength() {
            strongest = cert.verdict;
        }
        rows.push(emit(cert, "", &mut files)?);
    }
    let mut control_rows = Vec::new();
    if with_controls {
        if let Some(n) = inst.binary().map(|c| c.m) {
            for l in (2..n).filter(|l| n % l == 0) {
                let Some(sub) = subfield_subgroup_instance(&ctx, l) else {
                    continue;
                };
                let sctx = Ctx::new(&sub, &env.model, &env.fx, env.seed)?;
                for c in ghs::candidates(&sctx)? {
                    if c.candidate["id"] == format!("ghs:l={l}") {
                        control_rows.push(emit(&c, "controls/subfield_subgroup_", &mut files)?);
                    }
                }
            }
        }
    }
    let base = &ctx.base;
    let count_status = |s: &str| rows.iter().filter(|r| r["evaluation"] == s).count();
    let not_evaluable: Vec<Value> = rows
        .iter()
        .filter(|r| r["evaluation"] == "not_evaluable")
        .map(|r| r["id"].clone())
        .collect();
    let summary = json!({
        "schema": "transfer-audit/summary/v2",
        "directory": dir,
        "role": role,
        "tool": TOOL,
        "certificate_schema": SCHEMA,
        "curve": {
            "name": inst.name,
            "slug": inst.rc.slug,
            "registered": inst.registered(),
            "family": if inst.is_binary() { "binary" } else { "prime" },
            "field_bits": inst.field_bits(),
            "r_log2": (inst.log2_r() * 1000.0).round() / 1000.0,
            "cofactor": inst.cofactor.to_string(),
        },
        "baseline": {
            "log2_total": (base.log2_total * 1000.0).round() / 1000.0,
            "equivalence_class_size": base.class_size,
            "reason": base.class_reason,
        },
        "verdict": strongest,
        "scope": [
            "A. Weil/GHS descent: every divisor l of n with l < n (binary fields); trace branch executed, m >= 2 not constructible in the repository",
            "B. isogeny moves: class invariants and local volcano data for l <= 31, GHS isogeny-extension heuristic, explicit degree-5 replay for sect113r1; no destination representative with a cheaper attack identified",
            "C. same-field covers: genus 3 (binary, executed) or genus 2 (prime, arithmetic), genus sweep g = 2..8 under the frozen model",
            "D. pairing: embedding degree exact or lower-bounded, L[1/3] price with the smallest published constant and o(1) dropped",
            "not covered: (2,2)-gluing, non-GHS descents, Weil restriction of non-rational maps, quantum attacks, implementation attacks, multi-target and amortized settings",
        ],
        "evaluation": {
            "evaluated": count_status("evaluated"),
            "excluded": count_status("excluded"),
            "not_evaluable": count_status("not_evaluable"),
            "not_evaluable_candidates": not_evaluable,
            "reading": "`no weakness found in scope` is the verdict of all three kinds of candidate. Evaluated: priced and compared with the matched baseline. Excluded: cannot yield a weakness (the subgroup is not carried, or the priced total, a lower bound where phases are unpriced, is not below the baseline). Not evaluable: the route may exist but its map or price could not be constructed far enough to decide, so the verdict is vacuous for it.",
        },
        "candidates": rows,
        "controls": control_rows,
        "environment": {
            "factor_budget": env.fx.budget,
            "factor_hints_loaded": env.hints,
            "cost_model": "jacobian_transfer::cost::CostModel::default()",
            "seed": env.seed,
        },
    });
    files.push((format!("{dir}/summary.json"), pretty(&summary)));
    Ok(CurveRun {
        dir: dir.to_string(),
        files,
        summary,
    })
}

fn toy_instances() -> Result<Vec<(String, Instance)>, String> {
    TOYS.iter()
        .map(|(name, a, m)| {
            toy_koblitz(name, *a, &BigUint::from(*m), 20261011).map(|i| (name.to_string(), i))
        })
        .collect()
}

/// All deterministic files of the suite, as `(relative path, content)`,
/// with per-curve timings.
pub fn build_suite(
    env: &Env,
    registry: &Path,
) -> Result<(Vec<(String, String)>, Vec<Value>), String> {
    let registry_text = fs::read_to_string(registry)
        .map_err(|e| format!("cannot read {}: {e}", registry.display()))?;
    let mut files: Vec<(String, String)> = Vec::new();
    let mut summaries = Vec::new();
    let mut timings = Vec::new();
    let mut jobs: Vec<(String, Instance, bool, &str, bool)> = Vec::new();
    for (name, dir) in REGISTERED {
        let inst = load_registered(&registry_text, name)?;
        jobs.push((dir.to_string(), inst, false, "audit", false));
    }
    for (name, inst) in toy_instances()? {
        jobs.push((name, inst, true, "audit", false));
    }
    for (lookup, display, dir, committed, role) in CONTROL_CURVES {
        let mut inst = load_registered(&registry_text, lookup)?;
        inst.name = display.to_string();
        jobs.push((dir.to_string(), inst, false, role, committed));
    }
    for (dir, inst, controls, role, committed) in jobs {
        let t0 = Instant::now();
        let run = audit_instance(env, &dir, &inst, controls, role, committed)
            .map_err(|e| format!("{}: {e}", inst.name))?;
        timings.push(
            json!({"curve": inst.name, "dir": run.dir, "wall_ms": t0.elapsed().as_millis() as u64}),
        );
        files.extend(run.files);
        summaries.push(run.summary);
    }
    let control = synthetic_ghs_control(&env.model);
    files.push((
        "controls/synthetic_formula_advantage.json".into(),
        pretty(&control),
    ));
    let listing: Vec<Value> = files
        .iter()
        .map(|(p, c)| json!({"path": p, "sha256": sha_hex(c.as_bytes()), "bytes": c.len()}))
        .collect();
    let index = json!({
        "schema": "transfer-audit/index/v1",
        "tool": TOOL,
        "seed": env.seed,
        "factor_budget": env.fx.budget,
        "factor_hints_loaded": env.hints,
        "cost_model": serde_json::to_value(&env.model).map_err(|e| e.to_string())?,
        "curves": summaries.iter().map(|s| json!({
            "name": s["curve"]["name"],
            "slug": s["curve"]["slug"],
            "verdict": s["verdict"],
            "summary": format!("{}/summary.json", s["directory"].as_str().unwrap_or("")),
        })).collect::<Vec<_>>(),
        "files": listing,
        "nondeterministic_files_excluded": ["run-log.json", "controls/hec_ic_control.json", "controls/hec_ic_control_extended.json"],
    });
    files.push(("index.json".into(), pretty(&index)));
    Ok((files, timings))
}

fn write_all(out: &Path, files: &[(String, String)]) -> Result<(), String> {
    for (rel, content) in files {
        let p = out.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::write(&p, content).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    Ok(())
}

pub fn run_suite(
    out: &Path,
    registry: &Path,
    common: &Common,
    measure_ic: bool,
    measure_ic_extended: bool,
) -> Result<(), String> {
    let env = build_env(common)?;
    let t0 = Instant::now();
    let (files, timings) = build_suite(&env, registry)?;
    write_all(out, &files)?;
    let mut log = json!({
        "schema": "transfer-audit/run-log/v1",
        "note": "timings and measured controls are nondeterministic and excluded from `check`",
        "suite_wall_ms": t0.elapsed().as_millis() as u64,
        "per_curve": timings,
    });
    if measure_ic {
        let t1 = Instant::now();
        let c = hec_ic_control(
            &PROTOCOL_PLAN,
            "control:hec_index_calculus_vs_rho_toy",
            false,
        );
        write_all(out, &[("controls/hec_ic_control.json".into(), pretty(&c))])?;
        log["hec_ic_control_wall_ms"] = json!(t1.elapsed().as_millis() as u64);
        log["hec_ic_control_pass"] = c["pass"].clone();
    }
    if measure_ic_extended {
        let t1 = Instant::now();
        let c = hec_ic_control(
            &EXTENDED_PLAN,
            "control:hec_index_calculus_vs_rho_toy_extended",
            true,
        );
        write_all(
            out,
            &[("controls/hec_ic_control_extended.json".into(), pretty(&c))],
        )?;
        log["hec_ic_control_extended_wall_ms"] = json!(t1.elapsed().as_millis() as u64);
        log["hec_ic_control_extended_pass"] = c["pass"].clone();
    }
    write_all(out, &[("run-log.json".into(), pretty(&log))])?;
    println!("wrote {} files under {}", files.len(), out.display());
    print_table(&files);
    Ok(())
}

fn print_table(files: &[(String, String)]) {
    for (path, content) in files {
        if !path.ends_with("summary.json") {
            continue;
        }
        let Ok(v) = serde_json::from_str::<Value>(content) else {
            continue;
        };
        println!(
            "\n{}  [{}]",
            v["curve"]["name"].as_str().unwrap_or("?"),
            v["verdict"].as_str().unwrap_or("?")
        );
        for c in v["candidates"].as_array().into_iter().flatten() {
            println!(
                "  {:<34} subgroup={:<13} log2(route/rho)={:<11} {} [{}]",
                c["id"].as_str().unwrap_or("?"),
                c["subgroup_status"].as_str().unwrap_or("?"),
                if c["carried"].as_bool() == Some(false) {
                    "not carried".to_string()
                } else {
                    c["log2_cost_ratio"]
                        .as_f64()
                        .map_or("-".into(), |x| format!("{x:.1}"))
                },
                c["verdict"].as_str().unwrap_or("?"),
                c["evaluation"].as_str().unwrap_or("?"),
            );
        }
    }
}

pub fn audit_one(
    curve: &str,
    registry: &Path,
    common: &Common,
    out: Option<&Path>,
) -> Result<(), String> {
    let env = build_env(common)?;
    let text = fs::read_to_string(registry)
        .map_err(|e| format!("cannot read {}: {e}", registry.display()))?;
    let inst = load_registered(&text, curve)?;
    let dir = sanitize(&curve.to_ascii_lowercase());
    let run = audit_instance(&env, &dir, &inst, false, "audit", false)?;
    if let Some(out) = out {
        write_all(out, &run.files)?;
    }
    print_table(&run.files);
    Ok(())
}

pub fn check_suite(certs: &Path, registry: &Path, common: &Common) -> Result<(), String> {
    let env = build_env(common)?;
    let (files, _) = build_suite(&env, registry)?;
    let mut bad = Vec::new();
    for (rel, content) in &files {
        match fs::read_to_string(certs.join(rel)) {
            Ok(on_disk) if on_disk == *content => {}
            Ok(_) => bad.push(format!("differs: {rel}")),
            Err(_) => bad.push(format!("missing: {rel}")),
        }
    }
    if bad.is_empty() {
        println!("check: {} files regenerated byte-identically", files.len());
        Ok(())
    } else {
        Err(format!("check failed:\n  {}", bad.join("\n  ")))
    }
}

// ── Registry sweep (pairing calibration) ─────────────────────────────

/// One curve of [`sweep_registry`].
pub struct SweepRow {
    pub slug: String,
    pub alias: String,
    pub embedding_degree: u64,
    pub field_bits: f64,
    pub log2_r: f64,
    /// `log2(route/rho)` at the conservative end of the price band.
    pub conservative_ratio: Option<f64>,
    /// The same at the attacker-favourable end.
    pub favourable_ratio: Option<f64>,
    pub lead: String,
    /// The curve's verdict: the strongest over every candidate.
    pub verdict: String,
    /// The pairing candidate's evaluation status.
    pub evaluation: String,
}

/// Audit every prime-field registry curve that has a generator and whose
/// committed trait record gives an embedding degree of at most `k_max`:
/// the curves for which the pairing route is not out of reach by size alone.
/// The trait records are read from `docs/curves/traits.json` (`env.traits`),
/// which `curve-traits check` keeps equal to the live computation; the
/// verdicts are those `audit --curve` gives, without its minutes-long live
/// trait computation for `mnt/mnt1`.
pub fn sweep_registry(env: &Env, registry: &Value, k_max: u64) -> Result<Vec<SweepRow>, String> {
    let traits = env
        .traits
        .as_ref()
        .and_then(|t| t["curves"].as_array())
        .ok_or("the sweep needs docs/curves/traits.json")?;
    let reg = registry["curves"]
        .as_array()
        .ok_or("registry has no curves")?;
    let with_generator: std::collections::BTreeMap<&str, &Value> = reg
        .iter()
        .filter(|c| {
            c["representations"]
                .as_array()
                .is_some_and(|r| !r.is_empty())
        })
        .filter_map(|c| c["slug"].as_str().map(|s| (s, c)))
        .collect();
    let mut chosen: Vec<(&str, u64)> = traits
        .iter()
        .filter(|t| t["family"] == "prime")
        .filter_map(|t| {
            let k: u64 = t["embedding"]["degree"].as_str()?.parse().ok()?;
            let slug = t["slug"].as_str()?;
            (k <= k_max && with_generator.contains_key(slug)).then_some((slug, k))
        })
        .collect();
    chosen.sort_unstable();
    let mut rows = Vec::new();
    for (slug, k) in chosen {
        let inst = load_registered_from(registry, slug)?;
        let run = audit_instance(env, &sanitize(slug), &inst, false, "audit", true)
            .map_err(|e| format!("{slug}: {e}"))?;
        let cand = run.summary["candidates"]
            .as_array()
            .and_then(|a| a.iter().find(|c| c["id"] == "pairing:tate_mov_frey_ruck"))
            .ok_or_else(|| format!("{slug}: no pairing candidate"))?;
        let favourable_ratio = run
            .files
            .iter()
            .find(|(p, _)| p.ends_with("pairing_tate_mov_frey_ruck.json"))
            .and_then(|(_, c)| serde_json::from_str::<Value>(c).ok())
            .and_then(|v| v["invariants"]["price_band"]["favourable_log2_cost_ratio"].as_f64());
        let alias = with_generator[slug]["aliases"][0]
            .as_str()
            .unwrap_or(slug)
            .to_string();
        rows.push(SweepRow {
            slug: slug.to_string(),
            alias,
            embedding_degree: k,
            field_bits: inst.field_bits(),
            log2_r: inst.log2_r(),
            conservative_ratio: cand["log2_cost_ratio"].as_f64(),
            favourable_ratio,
            lead: cand["lead_class"].as_str().unwrap_or("?").to_string(),
            verdict: run.summary["verdict"].as_str().unwrap_or("?").to_string(),
            evaluation: cand["evaluation"].as_str().unwrap_or("?").to_string(),
        });
    }
    rows.sort_by(|a, b| {
        a.field_bits
            .partial_cmp(&b.field_bits)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.slug.cmp(&b.slug))
    });
    Ok(rows)
}

/// `transfer-audit sweep`: print [`sweep_registry`] as a table.
pub fn print_sweep(registry: &Path, common: &Common, k_max: u64) -> Result<(), String> {
    let env = build_env(common)?;
    let text = fs::read_to_string(registry)
        .map_err(|e| format!("cannot read {}: {e}", registry.display()))?;
    let doc: Value = serde_json::from_str(&text).map_err(|e| format!("registry JSON: {e}"))?;
    let rows = sweep_registry(&env, &doc, k_max)?;
    println!(
        "# prime-field registry curves with a generator and embedding degree <= {k_max}: {} curves; trait records from {}",
        rows.len(),
        common.traits.display()
    );
    println!("# ratio = log2(pairing route / matched rho); verdict = the curve's strongest verdict; margin {} bits", env.model.advantage_margin_bits);
    println!("alias\tk\tfield_bits\tlog2_r\tconservative_ratio\tfavourable_ratio\tlead_class\tverdict\tpairing_evaluation");
    let f = |x: Option<f64>| x.map_or("-".to_string(), |v| format!("{v:.1}"));
    for r in &rows {
        println!(
            "{}\t{}\t{:.0}\t{:.1}\t{}\t{}\t{}\t{}\t{}",
            r.alias,
            r.embedding_degree,
            r.field_bits,
            r.log2_r,
            f(r.conservative_ratio),
            f(r.favourable_ratio),
            r.lead,
            r.verdict,
            r.evaluation
        );
    }
    let weak: Vec<&str> = rows
        .iter()
        .filter(|r| r.verdict == "class-weak")
        .map(|r| r.alias.as_str())
        .collect();
    println!(
        "# class-weak: {} of {}: {}",
        weak.len(),
        rows.len(),
        weak.join(" ")
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certificate::Evidence;
    use crate::subgroup::{check_subgroup_preservation, BinaryEc, SubgroupInputs};
    use crypto_lib::binary_ecc::curve::scalar_mul;
    use crypto_lib::binary_ecc::BinaryPoint;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn env() -> Env {
        Env {
            model: CostModel::default(),
            fx: Factoring::new(1 << 16, Vec::new()),
            hints: 0,
            seed: 20261011,
            transfers: None,
            covers: None,
            traits: None,
        }
    }

    /// On the Koblitz toy curve over F_(2^7) (#E = 142 = 2 * 71, r = 71) the
    /// 2-Frobenius is an automorphism of E(F_q) that acts on <G> as an
    /// eigenvalue, so the certificate passes; multiplication by r kills <G>
    /// (kernel meets the subgroup) and the certificate fails.
    #[test]
    fn certificate_passes_a_correct_map_and_fails_a_map_with_kernel_in_the_subgroup() {
        let inst = toy_koblitz("t7", 1, &BigUint::from(0x83u32), 1).unwrap();
        let c = inst.binary().unwrap();
        let g = c.generator.clone();
        let mut rng = StdRng::seed_from_u64(3);
        let samples: Vec<BinaryPoint> = (0..4)
            .map(|_| crate::spec::random_binary_point(c, &mut rng))
            .collect();
        let scalars = [BigUint::from(5u8), BigUint::from(17u8)];
        let group = BinaryEc(c);
        let frob = |p: &BinaryPoint| -> Result<BinaryPoint, String> {
            Ok(match p {
                BinaryPoint::Infinity => BinaryPoint::Infinity,
                BinaryPoint::Affine { x, y } => BinaryPoint::Affine {
                    x: x.square(&c.irreducible),
                    y: y.square(&c.irreducible),
                },
            })
        };
        let good = check_subgroup_preservation(&SubgroupInputs {
            src: &group,
            dst: &group,
            map: &frob,
            g: &g,
            r: &inst.r,
            r_is_prime: true,
            samples: &samples,
            scalars: &scalars,
        });
        assert_eq!(good.status, Evidence::Verified, "{:?}", good.failures);
        assert!(good.image_order_is_r && !good.kernel_meets_subgroup);
        let times_r =
            |p: &BinaryPoint| -> Result<BinaryPoint, String> { Ok(scalar_mul(c, p, &inst.r)) };
        let bad = check_subgroup_preservation(&SubgroupInputs {
            src: &group,
            dst: &group,
            map: &times_r,
            g: &g,
            r: &inst.r,
            r_is_prime: true,
            samples: &samples,
            scalars: &scalars,
        });
        assert_eq!(bad.status, Evidence::Refuted);
        assert!(bad.kernel_meets_subgroup && bad.image_is_identity);
        // The real descent: trace to F_2 has image in E(F_2) of order 2,
        // gcd(71, 2) = 1, so it also kills <G>, in agreement with the arithmetic.
        let e = env();
        let ctx = Ctx::new(&inst, &e.model, &e.fx, e.seed).unwrap();
        let certs = ghs::candidates(&ctx).unwrap();
        let l1 = certs
            .iter()
            .find(|c| c.candidate["id"] == "ghs:l=1")
            .unwrap();
        assert_eq!(l1.subgroup_preservation.status, Evidence::Refuted);
        assert!(l1
            .checks
            .iter()
            .all(|c| c.result != crate::certificate::CheckResult::Fail));
    }

    /// The whole pipeline on a toy curve: deterministic, no failed check,
    /// executed cover pullback verified, every verdict inside the vocabulary
    /// and equal to `no weakness found in scope`, prime degree gives the
    /// out-of-scope certificate.
    #[test]
    fn toy_audit_is_deterministic_and_consistent() {
        let e = env();
        let inst = toy_koblitz("toy-t15", 1, &BigUint::from(0x8003u32), 20261011).unwrap();
        let a = audit_instance(&e, "toy-t15", &inst, true, "audit", false).unwrap();
        let b = audit_instance(&e, "toy-t15", &inst, true, "audit", false).unwrap();
        assert_eq!(a.files, b.files);
        let ids: Vec<String> = a.summary["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["id"].as_str().unwrap().to_string())
            .collect();
        for want in [
            "ghs:l=1",
            "ghs:l=3",
            "ghs:l=5",
            "cover:binary_g3_deg3",
            "cover:genus_sweep",
            "pairing:tate_mov_frey_ruck",
            "isogeny:class_invariants",
        ] {
            assert!(ids.iter().any(|i| i == want), "missing {want}: {ids:?}");
        }
        for (path, content) in &a.files {
            if path.ends_with("summary.json") {
                continue;
            }
            let v: Value = serde_json::from_str(content).unwrap();
            assert_eq!(v["verdict"], "no weakness found in scope", "{path}");
            for chk in v["checks"].as_array().unwrap() {
                assert_ne!(chk["result"], "FAIL", "{path}: {chk}");
            }
        }
        let cover = a.summary["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == "cover:binary_g3_deg3")
            .unwrap();
        assert_eq!(cover["subgroup_status"], "verified");
        // n = 15 is composite: no out-of-scope certificate, towers l = 1, 3, 5.
        assert!(!ids.iter().any(|i| i == "ghs:proper_subfield"));
        // A prime degree gives it.
        let p7 = toy_koblitz("toy-t7", 1, &BigUint::from(0x83u32), 1).unwrap();
        let c7 = audit_instance(&e, "toy-t7", &p7, false, "audit", false).unwrap();
        assert!(c7
            .files
            .iter()
            .any(|(p, _)| p.ends_with("ghs_proper_subfield.json")));
    }

    /// Slugs of the registry curves the sweep audits, and the subset whose
    /// verdict is `class-weak` after the margin rule (Amendment 3).  Both
    /// are measured by this test's own sweep and pinned: a registry change
    /// that adds a pairing-friendly curve, or a model change that moves a
    /// curve across the margin, fails here and needs a human reading of the
    /// new row before the pin is updated.
    // 31 since the registry's mixed-twist IC curves p101 and p1009 gained
    // trait records (#2746); neither is class-weak.
    const PINNED_SWEPT: usize = 31;
    const PINNED_CLASS_WEAK: &[&str] = &[
        "icv1-fp170-tm52213820118540299389901413-869d5d3c", // mnt/mnt1, the positive control
        "icv1-fp350-t35917316178020966140770026648761406645010658995732487-6e1654f7", // bn/bn350, -13.3 bits
        "icv1-fp382-t2353932232174051770881173201777159236219815696568919523335-b80e1e2a", // bn/bn382
        "icv1-fp384-t6277101735386680763835754795021643429033562911502587034007-3daf17f4", // other/Fp384BN
        "icv1-fp414-t154267229207681125708793071161652324726038701923943665681563655-ebea0e02", // bn/bn414
        "icv1-fp446-t10109980000181489923001201093401906549415298505746886291297633566727-278b8869", // bn/bn446, -44.5 bits
        "icv1-fp478-t662567649291894123450065105820326672097321614256273183056335502227537927-6422afbf", // bn/bn478
        "icv1-fp510-t43422033463993573283847164979847511362503473044959842495500254821277109321735-ee4a65f3", // bn/bn510
        "icv1-fp512-t115792089237316195423570985008687840101103879741288409693302319863988063823175-9e4bc1d2", // other/Fp512BN
        "icv1-fp542-t2845711812853053972806387468884004947678235462567670984735634579507964130977906695-f8fcf71f", // bn/bn542
        "icv1-fp574-t186496302581962721809830439849867378820456264884631045871326327118225203759963374092295-5d71cb14", // bn/bn574
        "icv1-fp606-t12222215858006915839141401255556695821975177558367937484927312121315564820050836996405726215-9acd5f3a", // bn/bn606
        "icv1-fp638-t800995136978371572363525747477255032258950408689114271367829691469194143147501961435441086332935-2ce55ebd", // bn/bn638, -112.6 bits
    ];

    /// The calibration test of finding 2: the pairing verdict over the whole
    /// registry list.  Before the margin, 15 of the 29 swept curves read
    /// `class-weak` (the review counted 34 swept curves and 14 verdicts;
    /// the difference in the count is unexplained), among them BN318 on a
    /// -2.987-bit formula difference.  Now a weakness verdict needs the
    /// conservative price to be at least the model margin below rho on E.
    #[test]
    fn registry_sweep_pins_the_class_weak_set() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let read = |rel: &str| fs::read_to_string(root.join(rel)).unwrap();
        let registry: Value = serde_json::from_str(&read("docs/curves/registry.json")).unwrap();
        let traits: Value = serde_json::from_str(&read("docs/curves/traits.json")).unwrap();
        let hints = read_hints(&read("docs/curves/factor-hints.txt")).unwrap();
        let mut e = env();
        e.fx = Factoring::new(1 << 18, hints);
        e.traits = Some(traits);
        let margin = e.model.advantage_margin_bits;
        let rows = sweep_registry(&e, &registry, 64).unwrap();
        assert_eq!(
            rows.len(),
            PINNED_SWEPT,
            "swept curves changed: {:#?}",
            rows.iter().map(|r| &r.alias).collect::<Vec<_>>()
        );
        let mut weak: Vec<&str> = rows
            .iter()
            .filter(|r| r.verdict == "class-weak")
            .map(|r| r.slug.as_str())
            .collect();
        weak.sort_unstable();
        let mut pinned: Vec<&str> = PINNED_CLASS_WEAK.to_vec();
        pinned.sort_unstable();
        assert_eq!(weak, pinned, "the class-weak set moved");
        for r in &rows {
            let ratio = r.conservative_ratio.expect("every swept curve is priced");
            // Only the pairing candidate can be weak on a prime-field curve,
            // and it is weak exactly when it is past the margin.
            assert_eq!(
                r.verdict == "class-weak",
                ratio <= -margin,
                "{}: ratio {ratio}, verdict {}",
                r.alias,
                r.verdict
            );
            // A price below rho by less than the margin stays a recorded lead.
            if ratio < 0.0 && ratio > -margin {
                assert_eq!(r.lead, "formula_marginal", "{}", r.alias);
                assert_eq!(r.verdict, "no weakness found in scope", "{}", r.alias);
            }
        }
        // Named boundary and positive controls.
        let verdict_of = |alias: &str| {
            rows.iter()
                .find(|r| r.alias == alias)
                .unwrap_or_else(|| panic!("{alias} is not in the sweep"))
                .verdict
                .as_str()
        };
        assert_eq!(verdict_of("mnt/mnt1"), "class-weak", "positive control");
        assert_eq!(verdict_of("bn/bn254"), "no weakness found in scope");
        assert_eq!(verdict_of("bls/BLS12-381"), "no weakness found in scope");
        assert_eq!(
            verdict_of("bn/bn318"),
            "no weakness found in scope",
            "the old flip"
        );
        assert_eq!(verdict_of("bls/BLS12-638"), "no weakness found in scope");
        assert_eq!(
            verdict_of("bn/bn350"),
            "class-weak",
            "smallest past the margin"
        );
        assert_eq!(
            verdict_of("bn/bn446"),
            "class-weak",
            "relative to rho on E, not absolute security"
        );
    }
}
