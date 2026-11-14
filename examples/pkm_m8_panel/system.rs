//! The raw-presentation polynomial system of one target and its exports.
//!
//! Variables `y{i}_{j}` (block `i < m`, tower level `j < t`, index `i*t+j`) then the
//! chain unknowns `u{k}` (`k < m-2`).  Equations, in toms.py's order: the Semaev S3
//! chain (`S3(x0,x1,u0)`, `S3(u_{k-1},x_{k+1},u_k)`, `S3(u_{m-3},x_{m-1},xR)`;
//! `S3(x0,x1,xR)` at m = 2) with `x_i = g*y_{i,0}`, then per block
//! `y_{i,j+1} - y_{i,j}^2` and `1 - y_{i,t-1}^2`.  The chain polynomials come from
//! `pkm_oracle_program/algebra.rs` (`plain_system`, chain arm).
use crate::algebra::{plain_system, Instance, Pol};
use crate::util::*;
use serde_json::{json, Value};
use std::path::Path;

pub struct Spec {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub g: u64,
    pub zeta: u64,
    pub m: usize,
    pub t: usize,
    pub x_r: u64,
}

pub fn nvars(m: usize, t: usize) -> usize {
    m * t + m.saturating_sub(2)
}

pub fn var_names(m: usize, t: usize) -> Vec<String> {
    let mut v: Vec<String> = (0..m)
        .flat_map(|i| (0..t).map(move |j| format!("y{i}_{j}")))
        .collect();
    v.extend((0..m.saturating_sub(2)).map(|k| format!("u{k}")));
    v
}

/// Equations as sparse terms over all variables, each sorted by descending
/// exponent vector (the order toms.py writes), plus the number of chain polynomials.
pub fn build(s: &Spec) -> (Vec<Vec<(Vec<u32>, u64)>>, usize) {
    let inst = Instance {
        p: s.p,
        a: s.a,
        b: s.b,
        g: s.g,
        zeta: s.zeta,
        t: s.t,
    };
    let (pols, n) = plain_system(&inst, s.m, s.x_r, true);
    let polys = pols.iter().map(sorted_terms).collect();
    (polys, n)
}

pub fn sorted_terms(f: &Pol) -> Vec<(Vec<u32>, u64)> {
    let mut t: Vec<(Vec<u32>, u64)> = f.t.iter().map(|(e, &c)| (e.clone(), c)).collect();
    t.sort_by(|x, y| y.0.cmp(&x.0));
    t
}

/// toms.py term text: `c*y0_0^e*...*u0^e` (constant: just `c`).
pub fn term_text(e: &[u32], c: u64, names: &[String]) -> String {
    let mut s = c.to_string();
    for (k, &x) in e.iter().enumerate() {
        if x > 0 {
            s.push('*');
            s.push_str(&names[k]);
            if x > 1 {
                s.push_str(&format!("^{x}"));
            }
        }
    }
    s
}

/// The `.ms` text in the format of L1-pkm/scripts/toms.py.
pub fn ms_text(polys: &[Vec<(Vec<u32>, u64)>], m: usize, t: usize, p: u64) -> String {
    let names = var_names(m, t);
    let mut out = names.join(",");
    out.push('\n');
    out.push_str(&p.to_string());
    out.push('\n');
    let chain = polys.len() - m * t;
    let mut eqs: Vec<String> = Vec::new();
    for f in &polys[..chain] {
        eqs.push(
            f.iter()
                .map(|(e, c)| term_text(e, *c, &names))
                .collect::<Vec<_>>()
                .join("+"),
        );
    }
    for i in 0..m {
        for j in 0..t.saturating_sub(1) {
            eqs.push(format!("1*y{i}_{}+{}*y{i}_{j}^2", j + 1, p - 1));
        }
        eqs.push(format!("1+{}*y{i}_{}^2", p - 1, t - 1));
    }
    out.push_str(&eqs.join(",\n"));
    out.push('\n');
    out
}

pub fn system_json(s: &Spec, polys: &[Vec<(Vec<u32>, u64)>], chain: usize) -> Value {
    json!({
        "schema": "pkm.m8-panel.system/v1",
        "p": s.p, "a": s.a, "b": s.b, "g": s.g, "zeta": s.zeta,
        "m": s.m, "t": s.t, "x_r": s.x_r,
        "variables": var_names(s.m, s.t),
        "chain_polys": chain,
        "polys": polys.iter().map(|f| f.iter().map(|(e, c)| json!([e, c])).collect::<Vec<_>>()).collect::<Vec<_>>(),
    })
}

pub fn polys_from_json(j: &Value) -> Result<(u64, usize, usize, Vec<Vec<(Vec<u32>, u64)>>), String> {
    let p = u64_of(j, "p")?;
    let m = u64_of(j, "m")? as usize;
    let t = u64_of(j, "t")? as usize;
    let polys = j["polys"]
        .as_array()
        .ok_or("polys")?
        .iter()
        .map(|f| {
            f.as_array()
                .ok_or("poly")?
                .iter()
                .map(|term| {
                    let e: Vec<u32> = u64_vec(&term[0])?.into_iter().map(|x| x as u32).collect();
                    let c = term[1].as_u64().ok_or("coef")?;
                    Ok((e, c))
                })
                .collect::<Result<Vec<_>, String>>()
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((p, m, t, polys))
}

pub fn run_export(args: &[String]) -> Result<(), String> {
    if crate::flag(args, "--l1-replication") {
        return l1_replication(&crate::arg_req(args, "--out")?);
    }
    let cell = crate::cell::Cell::load(Path::new(&crate::arg_req(args, "--cell")?))?;
    let tdir = crate::arg_req(args, "--targets")?;
    let out = crate::arg_req(args, "--out")?;
    let format = crate::arg_opt(args, "--format").unwrap_or_else(|| "both".into());
    let only = crate::arg_opt(args, "--only");
    let tj = read_json(&Path::new(&tdir).join("targets.json"))?;
    if !check_seal(&tj) {
        return Err("targets.json seal mismatch".into());
    }
    let m = u64_of(&tj, "m")? as usize;
    let mut entries = Vec::new();
    for tg in tj["targets"].as_array().ok_or("targets")? {
        let name = tg["name"].as_str().ok_or("name")?;
        if only.as_deref().is_some_and(|o| o != name) {
            continue;
        }
        let spec = Spec {
            p: cell.p,
            a: cell.a,
            b: cell.b,
            g: cell.g,
            zeta: cell.zeta,
            m,
            t: cell.t,
            x_r: u64_of(tg, "x_r")?,
        };
        let (polys, _n) = build(&spec);
        let chain = polys.len() - m * cell.t;
        let mut e = json!({"name": name, "x_r": spec.x_r, "nvars": nvars(m, cell.t), "npolys": polys.len()});
        if format == "ms" || format == "both" {
            let text = ms_text(&polys, m, cell.t, cell.p);
            let path = Path::new(&out).join(format!("{name}.ms"));
            std::fs::create_dir_all(&out).map_err(|x| x.to_string())?;
            std::fs::write(&path, &text).map_err(|x| x.to_string())?;
            e["ms_sha256"] = json!(sha256_hex(text.as_bytes()));
            e["ms_bytes"] = json!(text.len());
        }
        if format == "native" || format == "both" {
            let sj = system_json(&spec, &polys, chain);
            let path = Path::new(&out).join(format!("{name}.system.json"));
            write_json(&path, &sj)?;
            e["system_sha256"] = json!(file_sha256(&path)?);
            e["system_canonical_sha256"] = json!(sha256_hex(canonical(&sj).as_bytes()));
        }
        entries.push(e);
    }
    let man = json!({
        "schema": "pkm.m8-panel.export-manifest/v1",
        "cell_sha256": cell.sha256,
        "targets_sha256": tj["sha256"],
        "m": m, "t": cell.t, "p": cell.p,
        "format_note": "`.ms` follows L1-pkm/scripts/toms.py: variable line y{i}_{j} then u{k}, characteristic, chain polynomials \
(terms sorted by descending (block exponents of y_i_0, u exponents)), then tower equations 1*y_{j+1}+(p-1)*y_j^2 and 1+(p-1)*y_{t-1}^2.",
        "entries": entries,
        "tool": crate::tool_info(),
    });
    write_json(&Path::new(&out).join("export.manifest.json"), &seal(man))?;
    println!("exported {} system(s) to {out}", entries.len());
    Ok(())
}

/// `export --l1-replication`: the L1 decisive cell (m = 8, t = 3, p = 16801793), natively
/// reconstructed, with the target `x_R` gen.py would draw next from the same stream
/// (random target, no algebraic decomposition). Everything here is reconstruction; the
/// committed L1 files hold none of a, b, g, zeta, x_R.
pub fn l1_replication(out: &str) -> Result<(), String> {
    use crate::targets::{alg_info, enumerate_alg};
    let d = crate::cell::derive(3, 8, 0, false)?;
    let cell = &d.cell;
    let (a, b, g, mut py) = crate::cell::l1_reconstruct()?;
    if (a, b, g) != (cell.a, cell.b, cell.g) {
        return Err("L1 reconstruction mismatch".into());
    }
    let m = 8;
    let alg = enumerate_alg(cell, m, 1_000_000_000)?;
    let mut draws = 0u64;
    let x_r = loop {
        let x = py.randrange(1, cell.p);
        draws += 1;
        if jacobi(crate::ec::rhs(cell.p, a, b, x), cell.p) != 1 {
            continue;
        }
        if alg_info(&alg, a, m, x).0.is_empty() {
            break x;
        }
    };
    let spec = Spec { p: cell.p, a, b, g, zeta: cell.zeta, m, t: cell.t, x_r };
    let (polys, _) = build(&spec);
    let chain = polys.len() - m * cell.t;
    let text = ms_text(&polys, m, cell.t, cell.p);
    let dir = Path::new(out).join("l1");
    write_json(&dir.join("cell.json"), &d.json)?;
    std::fs::write(dir.join("l1_t3r8.ms"), &text).map_err(|e| e.to_string())?;
    let sj = system_json(&spec, &polys, chain);
    write_json(&dir.join("l1_t3r8.system.json"), &sj)?;
    let rec = json!({
        "schema": "pkm.m8-panel.l1-replication/v1",
        "status": "reconstructed_unverified",
        "recoverable_from_committed_files": {"p": true, "a": false, "b": false, "g": false, "zeta": false, "x_R": false,
            "note": "only seeds, counts and logs are committed; a, b, g, x_R re-derived by the native CPython random.Random(str) port"},
        "p": cell.p, "a": a, "b": b, "g": g, "zeta": cell.zeta, "x_r": x_r, "x_r_draws": draws,
        "nvars": nvars(m, cell.t), "npolys": polys.len(),
        "expected_from_A3_log": {"variables": 30, "equations": 31, "first_round": "deg 4, 7 selected, 7 pairs, 31 x 99, 4.40%, 7 new, 0 zero"},
        "ms_sha256": sha256_hex(text.as_bytes()),
        "tool": crate::tool_info(),
    });
    write_json(&dir.join("l1.json"), &seal(rec))?;
    println!("l1 replication: x_R={x_r}, {} variables, {} equations", nvars(m, cell.t), polys.len());
    Ok(())
}
