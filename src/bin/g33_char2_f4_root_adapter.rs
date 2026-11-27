//! Root-complete splitter using the characteristic-two F4 decision port.
//! Only a finished search emits roots; caps produce an INCOMPLETE receipt.
use crypto_lib::cryptanalysis::f4_gf2::{decide_with, KernelOptions, MacaulayCaps};
use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly, Mask};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;

#[derive(Clone, Copy)]
struct Caps {
    roots: usize,
    nodes: usize,
}

#[derive(Default)]
struct Stats {
    nodes: usize,
    f4_calls: usize,
    oversize: usize,
    refuted: usize,
}

fn specialize(polys: &[F2BoolPoly], variable: u32, value: bool) -> Vec<F2BoolPoly> {
    polys
        .iter()
        .map(|p| p.substitute(variable, value))
        .filter(|p| !p.is_zero())
        .collect()
}

fn walk(
    polys: Vec<F2BoolPoly>,
    original: &[F2BoolPoly],
    mut assignment: Vec<Option<bool>>,
    caps: Caps,
    stats: &mut Stats,
    roots: &mut Vec<Mask>,
) -> Result<(), &'static str> {
    if stats.nodes >= caps.nodes {
        return Err("node cap");
    }
    stats.nodes += 1;
    let mut polys = polys;
    loop {
        if polys
            .iter()
            .any(|p| p.terms.len() == 1 && p.terms[0].mask == 0)
        {
            stats.refuted += 1;
            return Ok(());
        }
        if assignment.iter().all(Option::is_some) {
            let point = assignment
                .iter()
                .enumerate()
                .fold(0 as Mask, |v, (i, bit)| {
                    v | (((*bit == Some(true)) as Mask) << i)
                });
            if original.iter().all(|p| p.eval(point) == 0) {
                if roots.len() >= caps.roots {
                    return Err("root cap");
                }
                roots.push(point);
            }
            return Ok(());
        }
        if polys.is_empty() {
            break;
        }
        stats.f4_calls += 1;
        let (decision, counters) = decide_with(
            &polys,
            assignment.len(),
            4,
            MacaulayCaps {
                max_rows: 20_000,
                max_cols: 40_000,
            },
            KernelOptions { f5: true },
        );
        if counters.oversize {
            stats.oversize += 1;
        }
        let Some(decision) = decision else { break };
        if decision.refuted {
            stats.refuted += 1;
            return Ok(());
        }
        let mut changed = false;
        for (variable, value) in decision.forced {
            let index = variable as usize;
            if index >= assignment.len() {
                return Err("invalid forced variable");
            }
            match assignment[index] {
                Some(old) if old != value => return Ok(()),
                Some(_) => {}
                None => {
                    assignment[index] = Some(value);
                    polys = specialize(&polys, variable, value);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let variable = assignment
        .iter()
        .position(Option::is_none)
        .expect("free variable");
    for value in [false, true] {
        let mut next = assignment.clone();
        next[variable] = Some(value);
        walk(
            specialize(&polys, variable as u32, value),
            original,
            next,
            caps,
            stats,
            roots,
        )?;
    }
    Ok(())
}

fn solve(polys: &[F2BoolPoly], n_vars: usize, caps: Caps) -> (Value, Vec<Mask>) {
    let mut stats = Stats::default();
    let mut roots = Vec::new();
    let outcome = walk(
        polys.to_vec(),
        polys,
        vec![None; n_vars],
        caps,
        &mut stats,
        &mut roots,
    );
    roots.sort_unstable();
    let complete = outcome.is_ok();
    let report = json!({"schema":"g33-char2-f4-port-roots/v1",
        "status":if complete {"COMPLETE"} else {"INCOMPLETE"},
        "reason":outcome.err(),
        "roots":if complete {Some(&roots)} else {None},
        "roots_sha256":if complete {Some(hex::encode(Sha256::digest(
            serde_json::to_vec(&roots).expect("roots JSON"))))} else {None},
        "caps":{"max_roots":caps.roots,"max_nodes":caps.nodes,
                "degree":4,"max_rows":20_000,"max_cols":40_000,"f5":true},
        "stats":{"nodes":stats.nodes,"f4_calls":stats.f4_calls,
                 "oversize":stats.oversize,"refuted":stats.refuted}});
    (report, roots)
}

fn uint(value: &Value, key: &str) -> Result<u64, String> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("invalid {key}"))
}

fn manifest(path: &str, draw: u64) -> Result<(Vec<F2BoolPoly>, usize, String, String), String> {
    let raw = fs::read(path).map_err(|e| e.to_string())?;
    let input_hash = hex::encode(Sha256::digest(&raw));
    let data: Value = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
    if data["schema"] != "G33-002-char2-panel-v1"
        || data["n"] != 15
        || data["field_modulus"] != "z^15+z+1"
        || data["curve"] != "y^2+xy=x^3+x^2+1"
        || data["source_api"] != "koblitz_groebner::build_decomposition_system"
    {
        return Err("manifest identity differs".into());
    }
    let m = uint(&data, "m")? as usize;
    let ell = uint(&data, "ell")? as usize;
    let cell = data["cell"].as_str().ok_or("invalid cell")?;
    if !matches!(
        (cell, m, ell),
        ("C1", 3, 5) | ("C2", 3, 6) | ("C3", 4, 6) | ("C4", 4, 7)
    ) {
        return Err("cell dimensions differ".into());
    }
    let draws = data["draws"].as_array().ok_or("invalid draws")?;
    if draws.len() != 4 {
        return Err("draw count differs".into());
    }
    let matching: Vec<_> = draws.iter().filter(|d| d["draw"] == draw).collect();
    if draw > 3 || matching.len() != 1 {
        return Err("missing or duplicate draw".into());
    }
    let entry = matching[0];
    let basis = entry["basis_masks"].as_array().ok_or("invalid basis")?;
    if basis.len() != ell
        || basis
            .iter()
            .any(|v| v.as_u64().is_none_or(|x| x == 0 || x >= 1 << 15))
    {
        return Err("basis masks differ".into());
    }
    let mut pivots = [0u64; 15];
    for value in basis {
        let mut row = value.as_u64().expect("checked basis mask");
        let mut independent = false;
        for i in (0..15).rev() {
            if row & (1 << i) == 0 {
                continue;
            }
            if pivots[i] == 0 {
                pivots[i] = row;
                independent = true;
                break;
            }
            row ^= pivots[i];
        }
        if !independent {
            return Err("basis masks are dependent".into());
        }
    }
    let target = entry["target"].as_object().ok_or("invalid target")?;
    if target.len() != 2
        || target
            .get("x")
            .and_then(Value::as_u64)
            .is_none_or(|x| x >= 1 << 15)
        || target
            .get("y")
            .and_then(Value::as_u64)
            .is_none_or(|y| y >= 1 << 15)
    {
        return Err("target encoding differs".into());
    }
    let n_vars = uint(entry, "n_vars")? as usize;
    if n_vars != m * ell + (m - 2) * 15 || n_vars >= 64 {
        return Err("variable layout differs".into());
    }
    let rows = entry["equations"].as_array().ok_or("invalid equations")?;
    if rows.len() != (m - 1) * 15 {
        return Err("equation count differs".into());
    }
    let arrays: Vec<Vec<u64>> = rows
        .iter()
        .map(|row| {
            let terms = row.as_array().ok_or("invalid equation row")?;
            let masks: Vec<u64> = terms
                .iter()
                .map(|x| x.as_u64().ok_or("invalid mask"))
                .collect::<Result<_, _>>()?;
            if masks.windows(2).any(|w| w[0] >= w[1]) || masks.iter().any(|&x| x >= 1u64 << n_vars)
            {
                return Err("mask order or width differs");
            }
            Ok(masks)
        })
        .collect::<Result<_, _>>()?;
    let equation_hash = hex::encode(Sha256::digest(
        serde_json::to_vec(&arrays).map_err(|e| e.to_string())?,
    ));
    if entry["equations_sha256"] != equation_hash {
        return Err("equation digest differs".into());
    }
    let polys = arrays
        .into_iter()
        .map(|row| {
            F2BoolPoly::from_monos(
                row.into_iter()
                    .map(|mask| F2BoolMono::from_mask(mask as Mask))
                    .collect(),
                n_vars,
            )
        })
        .collect();
    Ok((polys, n_vars, input_hash, equation_hash))
}

fn selftest() {
    let p = |row: &[u64]| {
        F2BoolPoly::from_monos(
            row.iter()
                .map(|&x| F2BoolMono::from_mask(x as Mask))
                .collect(),
            3,
        )
    };
    let cases = [
        vec![p(&[1, 2]), p(&[0, 6])],
        vec![p(&[1]), p(&[0, 1])],
        vec![p(&[1])],
    ];
    for polys in &cases {
        let (report, roots) = solve(
            polys,
            3,
            Caps {
                roots: 8,
                nodes: 100,
            },
        );
        let direct: Vec<_> = (0..8)
            .filter(|&v| polys.iter().all(|p| p.eval(v) == 0))
            .collect();
        assert_eq!(report["status"], "COMPLETE");
        assert_eq!(roots, direct);
    }
    let (root_cap, _) = solve(
        &cases[2],
        3,
        Caps {
            roots: 3,
            nodes: 100,
        },
    );
    assert_eq!(root_cap["status"], "INCOMPLETE");
    assert!(root_cap["roots"].is_null());
    let (node_cap, _) = solve(&cases[2], 3, Caps { roots: 8, nodes: 1 });
    assert_eq!(node_cap["status"], "INCOMPLETE");
    assert!(node_cap["roots"].is_null());
    let mut state = 0x5eed_u64;
    for _ in 0..64 {
        let mut polys = Vec::new();
        for _ in 0..3 {
            let mut row = Vec::new();
            for mask in 0..16 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                if state >> 61 == 0 {
                    row.push(F2BoolMono::from_mask(mask));
                }
            }
            polys.push(F2BoolPoly::from_monos(row, 4));
        }
        let (report, roots) = solve(
            &polys,
            4,
            Caps {
                roots: 16,
                nodes: 1000,
            },
        );
        let direct: Vec<_> = (0..16)
            .filter(|&point| polys.iter().all(|poly| poly.eval(point) == 0))
            .collect();
        assert_eq!(report["status"], "COMPLETE");
        assert_eq!(roots, direct);
    }
    println!("G33 char2 F4 port all-root noncohort controls passed");
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--selftest"] {
        selftest();
        return;
    }
    let result = (|| -> Result<Value, String> {
        if args.len() != 8
            || args[0] != "--manifest"
            || args[2] != "--draw"
            || args[4] != "--max-roots"
            || args[6] != "--max-nodes"
        {
            return Err("usage: g33_char2_f4_root_adapter --selftest | --manifest FILE --draw N --max-roots K --max-nodes K".into());
        }
        let draw = args[3].parse::<u64>().map_err(|e| e.to_string())?;
        let roots = args[5].parse::<usize>().map_err(|e| e.to_string())?;
        let nodes = args[7].parse::<usize>().map_err(|e| e.to_string())?;
        if roots == 0 || nodes == 0 {
            return Err("caps must be positive".into());
        }
        let (polys, n_vars, input_hash, equation_hash) = manifest(&args[1], draw)?;
        let (mut report, _) = solve(&polys, n_vars, Caps { roots, nodes });
        report["input_sha256"] = json!(input_hash);
        report["equations_sha256"] = json!(equation_hash);
        report["draw"] = json!(draw);
        Ok(report)
    })();
    match result {
        Ok(report) => {
            println!("{report}");
            if report["status"] != "COMPLETE" {
                std::process::exit(3);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
