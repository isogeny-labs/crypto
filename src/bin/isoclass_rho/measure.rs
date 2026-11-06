//! The measurements on a recorded walk: every measured curve under the
//! same paired walk seeds (`class`), many generators of the same `⟨P⟩` with
//! the same secret on chosen curves (`generators`), and instances drawn on
//! one curve carried to every other (`shift`); with isomorphic or
//! re-represented controls, an A/A replica, planted effects of known size
//! (negation fold, `r = 4` walk) and the structural folds a curve admits
//! (automorphisms at `j ∈ {0, 1728}`, Frobenius on a Koblitz crater) as
//! positive controls.  All statistics are computed here.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde_json::{json, Value};

use super::field::{Curve, F2m, Fp, Fq};
use super::poly;
use super::rho::{self, Fold, Folder, Instance, RhoCfg, Solve};
use super::stats;

pub struct MeasureParams {
    pub mode: String,
    pub trials: u64,
    pub generators: usize,
    pub vertices: Option<Vec<usize>>,
    pub max_vertices: usize,
    pub controls: usize,
    pub plants: bool,
    pub folds: bool,
    pub seed: u64,
    pub r: usize,
    pub dp_bits: Option<u32>,
    pub command: String,
}

/// One measured arm: a curve model with an instance, a walk configuration
/// and a fold.
pub struct Arm {
    pub label: String,
    pub kind: String,
    pub vertex: Option<usize>,
    pub source: usize,
    pub generator: Option<(usize, u64)>,
    pub inst: Instance,
    pub cfg: RhoCfg,
    pub folder: Folder,
    pub seed_offset: u64,
    /// For re-represented controls (binary): the field the arm runs in.
    pub alt_field: Option<F2m>,
    pub transport_ms: f64,
    pub meta: Value,
}

fn vget(v: &Value, k: &str) -> u64 {
    v[k].as_u64()
        .unwrap_or_else(|| panic!("walk.json: missing {k}"))
}

/// Parse an instance `[Px, Py, Qx, Qy]`.
fn inst_of(v: &Value) -> Option<((u64, u64), (u64, u64))> {
    let a = v.as_array()?;
    Some((
        (a[0].as_u64()?, a[1].as_u64()?),
        (a[2].as_u64()?, a[3].as_u64()?),
    ))
}

pub fn run_measure<F: Fq>(f: &F, walk: &Value, params: &MeasureParams, out: &Path) {
    let t_all = Instant::now();
    let mut rng = SmallRng::seed_from_u64(params.seed ^ 0x3e45);
    let root = &walk["root"];
    let n = vget(root, "n");
    let k_secret = vget(root, "k");
    let e0 = Curve {
        a: vget(root, "a"),
        b: vget(root, "b"),
    };
    let measured: Vec<usize> = walk["measured"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect();
    let verts = walk["vertices"].as_array().unwrap();
    let n_src = walk["sources"].as_array().unwrap().len();
    let dp_bits = params
        .dp_bits
        .unwrap_or(((63 - n.leading_zeros()) / 2).saturating_sub(6).min(8));
    let base_cfg = RhoCfg {
        r: params.r,
        dp_bits,
        max_run_factor: 40,
    };
    let curve_of = |v: usize| Curve {
        a: vget(&verts[v], "a"),
        b: vget(&verts[v], "b"),
    };
    let inst_at = |v: usize, s: usize| {
        inst_of(&verts[v]["inst"][s]).unwrap_or_else(|| panic!("no instance of source {s} at v{v}"))
    };
    let transport = |v: usize| verts[v]["transport"]["ms"].as_f64().unwrap_or(0.0);
    let meta_of = |v: usize| {
        json!({"levels": verts[v]["levels"], "conductor": verts[v]["conductor"], "dist": verts[v]["dist"],
               "path": verts[v]["path"], "log2_degree": verts[v]["log2_degree"], "j": verts[v]["j"],
               "stratum_class_number": verts[v]["stratum_class_number"],
               "edge": verts[v]["edge"].as_object().map(|e| json!({"ell": e["ell"], "route": e["route"], "field_degree": e["field_degree"]}))})
    };
    let is_prime = !f.char2();
    let is_koblitz = f.char2() && e0.b == 1 && e0.a <= 1;
    let j0 = is_prime
        && (walk["root"]["j"].as_u64() == Some(0)
            || walk["root"]["j"].as_u64()
                == Some(1728 % ((f as &dyn std::any::Any).downcast_ref::<Fp>().unwrap().p)));

    let mut arms: Vec<Arm> = Vec::new();
    let mk = |label: String,
              kind: &str,
              vertex: Option<usize>,
              source: usize,
              curve: Curve,
              inst: ((u64, u64), (u64, u64)),
              cfg: RhoCfg,
              folder: Folder,
              seed_offset: u64,
              transport_ms: f64,
              meta: Value| Arm {
        label,
        kind: kind.into(),
        vertex,
        source,
        generator: None,
        inst: Instance {
            curve,
            p: inst.0,
            q: inst.1,
        },
        cfg,
        folder,
        seed_offset,
        alt_field: None,
        transport_ms,
        meta,
    };

    match params.mode.as_str() {
        "class" => {
            for &v in &measured {
                arms.push(mk(
                    format!("v{v}"),
                    "isogenous",
                    Some(v),
                    0,
                    curve_of(v),
                    inst_at(v, 0),
                    base_cfg,
                    Folder::none(),
                    0,
                    transport(v),
                    meta_of(v),
                ));
            }
        }
        "generators" => {
            let chosen: Vec<usize> = match &params.vertices {
                Some(v) => v.clone(),
                None => spread(&measured, params.max_vertices),
            };
            for &v in &chosen {
                let c = curve_of(v);
                let (p, q) = inst_at(v, 0);
                for g in 0..params.generators {
                    let r = rng.gen_range(1..n);
                    let pg = f.ec_mul(&c, Some(p), r).unwrap();
                    let qg = f.ec_mul(&c, Some(q), r).unwrap();
                    assert_eq!(f.ec_mul(&c, Some(pg), k_secret), Some(qg));
                    let mut arm = mk(
                        format!("v{v}g{g}"),
                        "generator",
                        Some(v),
                        0,
                        c,
                        (pg, qg),
                        base_cfg,
                        Folder::none(),
                        0,
                        transport(v),
                        meta_of(v),
                    );
                    arm.generator = Some((g, r));
                    arm.meta["generator"] = json!({"index": g, "r": r, "r_bits": 64 - r.leading_zeros(), "r_popcount": r.count_ones(), "Px": pg.0});
                    arms.push(arm);
                }
            }
        }
        "shift" => {
            for s in 0..n_src {
                for &v in &measured {
                    let Some(inst) = inst_of(&verts[v]["inst"][s]) else {
                        continue;
                    };
                    let mut arm = mk(
                        format!("s{s}v{v}"),
                        "shift",
                        Some(v),
                        s,
                        curve_of(v),
                        inst,
                        base_cfg,
                        Folder::none(),
                        0,
                        transport(v),
                        meta_of(v),
                    );
                    arm.meta["source_vertex"] = walk["sources"][s]["vertex"].clone();
                    arms.push(arm);
                }
            }
        }
        m => panic!("unknown mode {m}"),
    }
    let root_inst = inst_at(0, 0);
    // controls on the root: isomorphic models (prime) or alternate field
    // representations (binary), an A/A replica, planted effects, folds
    if params.mode != "shift" {
        for c in 0..params.controls {
            if let Some(fp) = (f as &dyn std::any::Any).downcast_ref::<Fp>() {
                let (m, map) = fp.ec_iso_model(&e0, &mut rng);
                let (cp, cq) = (map(root_inst.0), map(root_inst.1));
                assert!(fp.ec_on(&m, Some(cp)) && fp.ec_on(&m, Some(cq)));
                arms.push(mk(
                    format!("iso{c}"),
                    "iso-control",
                    None,
                    0,
                    m,
                    (cp, cq),
                    base_cfg,
                    Folder::none(),
                    0,
                    0.0,
                    json!({"u_model": [m.a, m.b]}),
                ));
            } else if let Some(f2) = (f as &dyn std::any::Any).downcast_ref::<F2m>() {
                // the same curve in F_2[z]/(poly_{c+1}) via a root of poly_0 there
                let alt = F2m::new(f2.m, F2m::irreducible_poly(f2.m, c + 1));
                let poly0: poly::Poly = (0..=f2.m).map(|i| (f2.poly >> i) & 1).collect();
                let rs = poly::roots(&alt, &poly0, &mut rng);
                let root_el = rs[0];
                let emb = |a: u64| f2.embed(&alt, root_el, a);
                let m = Curve {
                    a: emb(e0.a),
                    b: emb(e0.b),
                };
                let cp = (emb(root_inst.0 .0), emb(root_inst.0 .1));
                let cq = (emb(root_inst.1 .0), emb(root_inst.1 .1));
                assert!(alt.ec_on(&m, Some(cp)) && alt.ec_on(&m, Some(cq)));
                assert_eq!(alt.ec_mul(&m, Some(cp), k_secret), Some(cq));
                let mut arm = mk(
                    format!("rep{c}"),
                    "iso-control",
                    None,
                    0,
                    m,
                    (cp, cq),
                    base_cfg,
                    Folder::none(),
                    0,
                    0.0,
                    json!({"alt_poly": format!("{:#x}", alt.poly)}),
                );
                arm.alt_field = Some(alt);
                arms.push(arm);
            }
        }
        if params.plants {
            arms.push(mk(
                "aa".into(),
                "aa-replica",
                Some(0),
                0,
                e0,
                root_inst,
                base_cfg,
                Folder::none(),
                0x9999,
                0.0,
                json!({"note": "root under an independent seed set"}),
            ));
            arms.push(mk("plant-neg".into(), "plant", Some(0), 0, e0, root_inst, base_cfg, Folder { fold: Fold::Neg, ..Folder::none() }, 0, 0.0, json!({"expected_ratio": 1.0 / 2f64.sqrt(), "note": "negation map: expected 1/√2 of the unfolded mean"})));
            let cfg4 = RhoCfg { r: 4, ..base_cfg };
            arms.push(mk("plant-r4".into(), "plant", Some(0), 0, e0, root_inst, cfg4, Folder::none(), 0, 0.0, json!({"expected_ratio": (1.0 / (1.0 - 1.0 / 4.0f64)).sqrt() / (1.0 / (1.0 - 1.0 / params.r as f64)).sqrt(), "note": "4-adding walk: Teske's 1/sqrt(1 − 1/r) relative to the r used"})));
        }
        if params.folds {
            if j0 {
                let fp = (f as &dyn std::any::Any).downcast_ref::<Fp>().unwrap();
                match rho::folder_aut(fp, &e0, root_inst.0, n) {
                    Ok(folder) => {
                        let orbit = folder.orbit as f64;
                        arms.push(mk("fold-aut".into(), "fold", Some(0), 0, e0, root_inst, base_cfg, folder, 0, 0.0, json!({"expected_ratio": 1.0 / orbit.sqrt(), "orbit": orbit, "note": "automorphism-folded walk on the crater"})));
                    }
                    Err(e) => eprintln!("fold aut unavailable: {e}"),
                }
            }
            if is_koblitz {
                let f2 = (f as &dyn std::any::Any).downcast_ref::<F2m>().unwrap();
                for with_neg in [false, true] {
                    match rho::folder_frob(f2, &e0, root_inst.0, n, with_neg) {
                        Ok(folder) => {
                            let orbit = folder.orbit as f64;
                            arms.push(mk(if with_neg { "fold-frobneg".into() } else { "fold-frob".into() }, "fold", Some(0), 0, e0, root_inst, base_cfg, folder, 0, 0.0, json!({"expected_ratio": 1.0 / orbit.sqrt(), "orbit": orbit, "note": "Frobenius-folded walk on the Koblitz crater"})));
                        }
                        Err(e) => eprintln!("fold frob unavailable: {e}"),
                    }
                }
            }
        }
    }

    // jobs: every arm × the same trial seeds, interleaved
    let mut jobs: Vec<(usize, u64)> = (0..arms.len())
        .flat_map(|a| (0..params.trials).map(move |s| (a, s)))
        .collect();
    jobs.shuffle(&mut rng);
    let seed_base = params.seed.wrapping_mul(0x1000_0000_01b3);
    eprintln!(
        "rho: {} arms x {} trials = {} solves (dp_bits {}, r {})",
        arms.len(),
        params.trials,
        jobs.len(),
        dp_bits,
        params.r
    );
    let t_rho = Instant::now();
    let results: Vec<(usize, u64, Solve)> = jobs
        .par_iter()
        .map(|&(a, s)| {
            let arm = &arms[a];
            let seed = seed_base ^ s ^ arm.seed_offset;
            let r = match arm.alt_field {
                Some(alt) => rho::rho(&alt, &arm.inst, n, seed, &arm.cfg, &arm.folder),
                None => rho::rho(f, &arm.inst, n, seed, &arm.cfg, &arm.folder),
            };
            (a, s, r)
        })
        .collect();
    let rho_s = t_rho.elapsed().as_secs_f64();
    let wrong = results.iter().filter(|r| r.2.k != Some(k_secret)).count();
    if wrong > 0 {
        eprintln!("isoclass-rho: {wrong} solves returned no or the wrong logarithm");
        std::process::exit(3);
    }

    // records
    fs::create_dir_all(out).unwrap();
    let mut csv =
        String::from("arm,kind,vertex,source,generator,seed,iters,restarts,dps,escapes,ns\n");
    let mut by_arm: Vec<Vec<Solve>> = vec![Vec::new(); arms.len()];
    let mut sorted: Vec<&(usize, u64, Solve)> = results.iter().collect();
    sorted.sort_by_key(|r| (r.0, r.1));
    for (a, s, r) in &sorted {
        by_arm[*a].push(*r);
        let arm = &arms[*a];
        csv += &format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            arm.label,
            arm.kind,
            arm.vertex.map_or(String::new(), |v| v.to_string()),
            arm.source,
            arm.generator.map_or(String::new(), |g| g.0.to_string()),
            seed_base ^ s ^ arm.seed_offset,
            r.iters,
            r.restarts,
            r.dps,
            r.escapes,
            r.ns
        );
    }
    fs::write(out.join("trials.csv"), csv).unwrap();

    let iters_of = |rs: &[Solve]| rs.iter().map(|r| r.iters as f64).collect::<Vec<f64>>();
    let summarize = |rs: &[Solve]| {
        let it = iters_of(rs);
        let ms: Vec<f64> = rs.iter().map(|r| r.ns as f64 / 1e6).collect();
        let nsop: Vec<f64> = rs.iter().map(|r| r.ns as f64 / r.iters as f64).collect();
        let restarts: Vec<f64> = rs.iter().map(|r| r.restarts as f64).collect();
        let dps: Vec<f64> = rs.iter().map(|r| r.dps as f64).collect();
        json!({"iters": stats::summary(&it), "ms": stats::summary(&ms), "ns_per_op": stats::summary(&nsop),
               "restarts_mean": stats::mean(&restarts), "dps_mean": stats::mean(&dps),
               "escapes_total": rs.iter().map(|r| r.escapes as u64).sum::<u64>()})
    };
    let expected = (std::f64::consts::PI * n as f64 / 2.0).sqrt();
    let r_factor = (1.0 / (1.0 - 1.0 / params.r as f64)).sqrt();

    // pooled reference: every isogenous (or generator / shift) solve
    let main_kind = match params.mode.as_str() {
        "class" => "isogenous",
        "generators" => "generator",
        _ => "shift",
    };
    let main_idx: Vec<usize> = (0..arms.len())
        .filter(|&i| arms[i].kind == main_kind)
        .collect();
    let pooled: Vec<f64> = main_idx
        .iter()
        .flat_map(|&i| iters_of(&by_arm[i]))
        .collect();
    let pooled_mean = stats::mean(&pooled);
    let pooled_var = stats::var(&pooled);
    let m_tests = main_idx.len().max(1);
    let alpha = 0.05;
    let z_crit = z_for_two_sided(alpha / m_tests as f64);

    let arm_json: Vec<Value> = arms
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let it = iters_of(&by_arm[i]);
            let others: Vec<f64> = main_idx.iter().filter(|&&k| k != i).flat_map(|&k| iters_of(&by_arm[k])).collect();
            let tests = if !others.is_empty() && !it.is_empty() {
                let mo = stats::mean(&others);
                let vo = stats::var(&others);
                let vi = stats::var(&it);
                let z = (stats::mean(&it) - mo) / (vi / it.len() as f64 + vo / others.len() as f64).sqrt();
                let (d, pks) = stats::ks(&it, &others);
                let mw = stats::mann_whitney_z(&it, &others);
                json!({"z_welch": z, "p_welch": stats::normal_two_sided(z), "ks_d": d, "ks_p": pks, "mw_z": mw,
                       "flag_bonferroni": a.kind == main_kind && z.abs() > z_crit})
            } else {
                Value::Null
            };
            let s = summarize(&by_arm[i]);
            let mean_it = s["iters"]["mean"].as_f64().unwrap();
            json!({
                "arm": a.label, "kind": a.kind, "vertex": a.vertex, "source": a.source,
                "generator": a.generator.map(|g| g.0),
                "curve": {"a": a.inst.curve.a, "b": a.inst.curve.b},
                "P": [a.inst.p.0, a.inst.p.1], "Q": [a.inst.q.0, a.inst.q.1],
                "fold": a.folder.fold.name(), "r": a.cfg.r, "meta": a.meta,
                "transport_ms": a.transport_ms,
                "e2e_ms_mean": a.transport_ms + s["ms"]["mean"].as_f64().unwrap(),
                "ratio_to_expected": mean_it / expected,
                "ratio_to_pooled": mean_it / pooled_mean,
                "summary": s, "tests": tests,
            })
        })
        .collect();

    // grouped summaries: by conductor stratum, by level per ℓ, by distance, by parent ℓ
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for &i in &main_idx {
        let a = &arms[i];
        let m = &a.meta;
        if let Some(c) = m["conductor"].as_u64() {
            groups.entry(format!("conductor:{c}")).or_default().push(i);
        }
        if let Some(lv) = m["levels"].as_object() {
            for (l, v) in lv {
                groups
                    .entry(format!("ell{l}:level{v}"))
                    .or_default()
                    .push(i);
            }
        }
        if let Some(d) = m["dist"].as_u64() {
            groups.entry(format!("dist:{d}")).or_default().push(i);
        }
        if let Some(e) = m["edge"].as_object() {
            groups
                .entry(format!("parent_ell:{}", e["ell"]))
                .or_default()
                .push(i);
            groups
                .entry(format!("field_degree:{}", e["field_degree"]))
                .or_default()
                .push(i);
        }
        if let Some(s) = m["source_vertex"].as_u64() {
            groups.entry(format!("source:{s}")).or_default().push(i);
        }
        if params.mode == "shift" || params.mode == "generators" {
            if let Some(v) = a.vertex {
                groups.entry(format!("target:{v}")).or_default().push(i);
            }
        }
        if let Some(g) = a.generator {
            groups
                .entry(format!("generator:{}", g.0))
                .or_default()
                .push(i);
        }
    }
    let mut group_json: Vec<Value> = groups
        .iter()
        .map(|(k, idx)| {
            let rs: Vec<Solve> = idx.iter().flat_map(|&i| by_arm[i].iter().copied()).collect();
            let it = iters_of(&rs);
            let others: Vec<f64> = main_idx.iter().filter(|i| !idx.contains(i)).flat_map(|&i| iters_of(&by_arm[i])).collect();
            let z = if others.len() > 1 {
                (stats::mean(&it) - stats::mean(&others)) / (stats::var(&it) / it.len() as f64 + stats::var(&others) / others.len() as f64).sqrt()
            } else {
                0.0
            };
            json!({"group": k, "arms": idx.len(), "solves": rs.len(), "summary": summarize(&rs), "z_vs_rest": z,
                   "ratio_to_expected": stats::mean(&it) / expected})
        })
        .collect();
    group_json.sort_by(|a, b| a["group"].as_str().cmp(&b["group"].as_str()));

    // across-arm tests
    let main_groups: Vec<Vec<f64>> = main_idx.iter().map(|&i| iters_of(&by_arm[i])).collect();
    let (h, df, p_kw) = if main_groups.len() > 1 {
        stats::kruskal_wallis(&main_groups)
    } else {
        (0.0, 0, 1.0)
    };
    let (fstat, p_f, vratio) = if main_groups.len() > 1 {
        stats::anova(&main_groups)
    } else {
        (0.0, 1.0, 1.0)
    };
    let arm_means: Vec<f64> = main_groups.iter().map(|g| stats::mean(g)).collect();
    let expected_sd_of_mean = (pooled_var / params.trials as f64).sqrt();
    let flagged: Vec<String> = arm_json
        .iter()
        .filter(|a| a["tests"]["flag_bonferroni"].as_bool() == Some(true))
        .map(|a| a["arm"].as_str().unwrap().to_string())
        .collect();
    let max_abs_z = arm_json
        .iter()
        .filter(|a| a["kind"] == main_kind)
        .filter_map(|a| a["tests"]["z_welch"].as_f64())
        .fold(0.0f64, |m, z| m.max(z.abs()));

    // per-vertex analyses for the generator / shift modes
    let mut per_vertex: Vec<Value> = Vec::new();
    if params.mode == "generators" || params.mode == "shift" {
        let mut by_v: HashMap<usize, Vec<usize>> = HashMap::new();
        for &i in &main_idx {
            by_v.entry(arms[i].vertex.unwrap()).or_default().push(i);
        }
        let mut keys: Vec<usize> = by_v.keys().copied().collect();
        keys.sort();
        for v in keys {
            let idx = &by_v[&v];
            let gs: Vec<Vec<f64>> = idx.iter().map(|&i| iters_of(&by_arm[i])).collect();
            let (h, df, p) = if gs.len() > 1 {
                stats::kruskal_wallis(&gs)
            } else {
                (0.0, 0, 1.0)
            };
            let (fs, pf, vr) = if gs.len() > 1 {
                stats::anova(&gs)
            } else {
                (0.0, 1.0, 1.0)
            };
            let means: Vec<f64> = gs.iter().map(|g| stats::mean(g)).collect();
            let all: Vec<f64> = gs.iter().flatten().copied().collect();
            // Spearman correlations of the generator means with generator features
            let feats = ["r", "r_bits", "r_popcount", "Px"];
            let corr: serde_json::Map<String, Value> = feats
                .iter()
                .filter_map(|&ft| {
                    let xs: Vec<f64> = idx
                        .iter()
                        .filter_map(|&i| arms[i].meta["generator"][ft].as_u64().map(|x| x as f64))
                        .collect();
                    (xs.len() == means.len() && xs.len() > 2)
                        .then(|| (ft.to_string(), json!(spearman(&xs, &means))))
                })
                .collect();
            per_vertex.push(json!({
                "vertex": v, "arms": idx.len(), "solves": all.len(), "summary": stats::summary(&all),
                "kruskal_wallis": {"H": h, "df": df, "p": p}, "anova": {"F": fs, "p": pf, "variance_ratio": vr},
                "arm_means": stats::summary(&means), "expected_sd_of_arm_mean": (stats::var(&all) / params.trials as f64).sqrt(),
                "spearman_vs_generator_features": corr,
                "max_abs_z": idx.iter().filter_map(|&i| arm_json[i]["tests"]["z_welch"].as_f64()).fold(0.0f64, |m, z| m.max(z.abs())),
            }));
        }
    }
    // the shift matrix: mean iterations by (source, target)
    let mut matrix: Vec<Value> = Vec::new();
    if params.mode == "shift" {
        for &i in &main_idx {
            let a = &arms[i];
            matrix.push(json!({"source": a.source, "source_vertex": a.meta["source_vertex"], "target": a.vertex,
                               "mean": stats::mean(&iters_of(&by_arm[i])), "n": by_arm[i].len(),
                               "mean_ms": stats::mean(&by_arm[i].iter().map(|r| r.ns as f64 / 1e6).collect::<Vec<_>>())}));
        }
        // two-way decomposition: η² of target and of source
        let grand = pooled_mean;
        let mut by_t: HashMap<usize, Vec<f64>> = HashMap::new();
        let mut by_s: HashMap<usize, Vec<f64>> = HashMap::new();
        for &i in &main_idx {
            by_t.entry(arms[i].vertex.unwrap())
                .or_default()
                .extend(iters_of(&by_arm[i]));
            by_s.entry(arms[i].source)
                .or_default()
                .extend(iters_of(&by_arm[i]));
        }
        let sst: f64 = pooled.iter().map(|x| (x - grand).powi(2)).sum();
        let eta = |m: &HashMap<usize, Vec<f64>>| {
            m.values()
                .map(|g| g.len() as f64 * (stats::mean(g) - grand).powi(2))
                .sum::<f64>()
                / sst
        };
        let tgroups: Vec<Vec<f64>> = by_t.values().cloned().collect();
        let sgroups: Vec<Vec<f64>> = by_s.values().cloned().collect();
        let (ht, dft, pt) = stats::kruskal_wallis(&tgroups);
        let (hs, dfs, ps) = if sgroups.len() > 1 {
            stats::kruskal_wallis(&sgroups)
        } else {
            (0.0, 0, 1.0)
        };
        per_vertex.push(json!({"two_way": {"eta2_target": eta(&by_t), "eta2_source": eta(&by_s),
                                            "kw_target": {"H": ht, "df": dft, "p": pt}, "kw_source": {"H": hs, "df": dfs, "p": ps}}}));
    }

    let host = fs::read_to_string("/proc/cpuinfo").ok().and_then(|c| {
        c.lines()
            .find(|l| l.starts_with("model name"))
            .map(|l| l.split(':').nth(1).unwrap().trim().to_string())
    });
    let controls: Vec<Solve> = arms
        .iter()
        .enumerate()
        .filter(|(_, a)| a.kind == "iso-control")
        .flat_map(|(i, _)| by_arm[i].iter().copied())
        .collect();
    let out_json = json!({
        "schema": "isoclass-rho/v1",
        "command": params.command,
        "mode": params.mode,
        "params": {"trials": params.trials, "generators": params.generators, "controls": params.controls, "plants": params.plants,
                   "folds": params.folds, "seed": params.seed, "r": params.r, "dp_bits": dp_bits, "vertices": params.vertices},
        "walk": {"root": walk["root"], "field": walk["field"], "class": walk["class"], "ells": walk["ells"], "walk": walk["walk"]},
        "host": {"cpu": host, "threads": rayon::current_num_threads()},
        "statistical_test_scope": {
            "ks_p": "two-sided continuous-null asymptotic approximation; D consumes ties together, but iteration counts are discrete",
            "rank_tests": "normal/chi-square asymptotic p-values with standard tie-variance corrections",
            "welch": "independent-sample Welch z; arms reuse trial seeds, so cross-arm covariance is not modeled",
        },
        "rho": {"walk": format!("{}-adding (Teske), single walker, distinguished points, no negation map unless folded", params.r),
                "expected_iters_sqrt_pi_n_over_2": expected, "expected_with_r_adding": expected * r_factor,
                "expected_iters_negation_sqrt_pi_n_over_4": (std::f64::consts::PI * n as f64 / 4.0).sqrt(),
                "solves": results.len(), "all_correct": wrong == 0, "wall_s": rho_s, "total_s": t_all.elapsed().as_secs_f64()},
        "pooled": {"kind": main_kind, "arms": main_idx.len(), "summary": summarize(&main_idx.iter().flat_map(|&i| by_arm[i].iter().copied()).collect::<Vec<_>>()),
                   "ratio_to_expected": pooled_mean / expected},
        "controls": if controls.is_empty() { Value::Null } else { summarize(&controls) },
        "tests": {"family_size": m_tests, "alpha": alpha, "z_crit_bonferroni": z_crit, "flagged_arms": flagged, "max_abs_z": max_abs_z,
                  "kruskal_wallis": {"H": h, "df": df, "p": p_kw}, "anova": {"F": fstat, "p": p_f, "variance_ratio": vratio},
                  "arm_means": stats::summary(&arm_means), "expected_sd_of_arm_mean": expected_sd_of_mean,
                  "observed_sd_of_arm_mean": stats::var(&arm_means).sqrt()},
        "arms": arm_json,
        "groups": group_json,
        "per_vertex": per_vertex,
        "shift_matrix": matrix,
    });
    fs::write(
        out.join("results.json"),
        serde_json::to_string_pretty(&out_json).unwrap(),
    )
    .unwrap();

    println!("\n{} mode: {} arms, {} solves, all correct (k = {k_secret}, n = {n}); expected sqrt(pi n/2) = {expected:.0}", params.mode, arms.len(), results.len());
    println!(
        "{:<14}{:<13}{:>12}{:>12}{:>12}{:>9}{:>9}{:>8}{:>7}",
        "arm", "kind", "min it", "mean it", "max it", "mean ms", "ns/op", "ratio", "z"
    );
    for a in &arm_json {
        let s = &a["summary"];
        println!(
            "{:<14}{:<13}{:>12.0}{:>12.0}{:>12.0}{:>9.2}{:>9.1}{:>8.3}{:>7.2}",
            a["arm"].as_str().unwrap(),
            a["kind"].as_str().unwrap(),
            s["iters"]["min"].as_f64().unwrap(),
            s["iters"]["mean"].as_f64().unwrap(),
            s["iters"]["max"].as_f64().unwrap(),
            s["ms"]["mean"].as_f64().unwrap(),
            s["ns_per_op"]["mean"].as_f64().unwrap(),
            a["ratio_to_expected"].as_f64().unwrap(),
            a["tests"]["z_welch"].as_f64().unwrap_or(0.0)
        );
    }
    println!("Kruskal-Wallis H = {h:.2} (df {df}, p = {p_kw:.3}); ANOVA F = {fstat:.3} (p = {p_f:.3}), variance ratio {vratio:.3}; max |z| = {max_abs_z:.2}, Bonferroni z_crit = {z_crit:.2}, flagged: {:?}", out_json["tests"]["flagged_arms"]);
    eprintln!("wrote {}", out.display());
}

fn spread(list: &[usize], k: usize) -> Vec<usize> {
    if list.len() <= k {
        return list.to_vec();
    }
    let step = list.len() as f64 / k as f64;
    (0..k).map(|i| list[(i as f64 * step) as usize]).collect()
}

fn spearman(x: &[f64], y: &[f64]) -> f64 {
    let rank = |v: &[f64]| {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
        let mut r = vec![0f64; v.len()];
        for (rank, &i) in idx.iter().enumerate() {
            r[i] = rank as f64;
        }
        r
    };
    let (rx, ry) = (rank(x), rank(y));
    let (mx, my) = (stats::mean(&rx), stats::mean(&ry));
    let num: f64 = rx.iter().zip(&ry).map(|(a, b)| (a - mx) * (b - my)).sum();
    let den = (rx.iter().map(|a| (a - mx).powi(2)).sum::<f64>()
        * ry.iter().map(|b| (b - my).powi(2)).sum::<f64>())
    .sqrt();
    if den == 0.0 {
        0.0
    } else {
        num / den
    }
}

/// `z` with two-sided normal tail `p`.
fn z_for_two_sided(p: f64) -> f64 {
    let (mut lo, mut hi) = (0.0f64, 10.0f64);
    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if stats::normal_two_sided(mid) > p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}
