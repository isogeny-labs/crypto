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
use crate::spec::{load_registered, toy_koblitz, Instance};
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

/// Registered pairing-friendly curves audited as end-to-end controls: the
/// MNT k = 3 example (a curve designed so that the pairing route is the
/// attack) and BLS12-381 (k = 12, where the formula band straddles rho).
/// `(lookup, display name, directory)`.
const CONTROL_CURVES: [(&str, &str, &str, bool); 2] = [
    // The live curve_traits::compute of mnt/mnt1 takes about six minutes (not
    // in this tool's code), so its trait record is read from the committed
    // docs/curves/traits.json, with order and trace checked against the instance.
    ("mnt/mnt1", "mnt/mnt1", "control-mnt1", true),
    (
        "icv1-fp381-tm15132376222941642751-961de9ba",
        "BLS12-381",
        "control-bls12-381",
        false,
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
            let mine = cert["costs"]["log2_total"].as_f64();
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
                        "delta_bits": (delta * 1000.0).round() / 1000.0,
                        "within_0_002_bits": delta.abs() <= 0.002,
                        "note": "the repository total is search + transfer + destination; this total also carries construction and one verification, both negligible against the destination at these sizes",
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
    let summary = json!({
        "schema": "transfer-audit/summary/v1",
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
    for (lookup, display, dir, committed) in CONTROL_CURVES {
        let mut inst = load_registered(&registry_text, lookup)?;
        inst.name = display.to_string();
        jobs.push((dir.to_string(), inst, false, "control", committed));
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
                "  {:<34} subgroup={:<13} log2(route/rho)={:<11} {}",
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
}
