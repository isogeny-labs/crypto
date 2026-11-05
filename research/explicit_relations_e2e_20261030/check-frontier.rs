// Post-run audit of frozen nonrectangular plans; no statistics or reruns.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};

fn check(raw: &str, version: u32) -> Result<(), String> {
    let fields: Vec<u32> = match version {
        3 => (20..=56).step_by(4).collect(),
        4 => vec![48, 52, 56, 58, 60],
        5 | 7 | 8 => vec![52, 56, 58, 60],
        9 | 11 | 12 | 13 => vec![52, 61, 62],
        14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => vec![52, 63],
        _ => return Err("version has no completed frontier contract".into()),
    };
    let mut lines = raw.lines();
    if lines.next() != Some("arm,instance_id,seed,replicate,warmup,status,verification,recoveries,wall_seconds,solver_seconds,verify_seconds,relations,attempts,bases_ms,targets_ms,oracle_ms,linear_ms,replay_ms,tier") {
        return Err("unexpected header".into());
    }
    let mut seen = BTreeSet::new();
    let mut rungs = BTreeMap::<u32, Vec<Vec<&str>>>::new();
    for line in lines {
        let v: Vec<_> = line.split(',').collect();
        if v.len() != 19 {
            return Err("invalid column count".into());
        }
        let integer = |i: usize| {
            v[i].parse::<u32>()
                .map_err(|_| format!("invalid integer column {i}"))
        };
        let (bits, seed, replicate) = (integer(1)?, integer(2)?, integer(3)?);
        if !fields.contains(&bits)
            || ![101, 202].contains(&seed)
            || replicate != u32::from(seed == 202)
        {
            return Err("wrong field, seed or replicate".into());
        }
        if !seen.insert((bits, seed, v[0])) {
            return Err("duplicate execution".into());
        }
        if v[4] != "false" || v[18] != "L0" {
            return Err("warmup or isolation mismatch".into());
        }
        if !["succeeded", "timeout", "limited"].contains(&v[5]) {
            return Err("invalid status".into());
        }
        let ok = v[5] == "succeeded";
        if v[6] != if ok { "verified" } else { "not_run" } || v[7] != if ok { "1" } else { "0" } {
            return Err("status/verification/recovery mismatch".into());
        }
        for i in 8..19 {
            if i == 18 || (!ok && i >= 11 && v[i].is_empty()) {
                continue;
            }
            let x = v[i]
                .parse::<f64>()
                .map_err(|_| format!("invalid numeric column {i}"))?;
            if !x.is_finite() || x < 0.0 {
                return Err("invalid numeric value".into());
            }
        }
        if ok {
            let relations = integer(11)?;
            let attempts = integer(12)?;
            if relations == 0 || attempts < relations {
                return Err("invalid relation accounting".into());
            }
        }
        rungs.entry(bits).or_default().push(v);
    }
    for (index, bits) in fields.iter().enumerate() {
        let rows = rungs
            .get(bits)
            .ok_or("premature truncation without recorded stopping condition")?;
        let arms: &[&str] = match version {
            11 | 13 | 14 | 17 | 18 | 19 | 20 | 21 if *bits == 52 => &["b", "u", "v", "x"],
            11 | 13 | 14 | 20 | 21 => &["v"],
            17 | 18 | 19 => &["u", "v"],
            3 if *bits <= 24 => &["h", "u", "x"],
            3 => &["u", "x"],
            _ if *bits <= 52 => &["b", "u", "x"],
            4 => &["u", "x"],
            _ => &["u"],
        };
        let expected: BTreeSet<_> = arms
            .iter()
            .flat_map(|arm| [(*bits, 101, *arm), (*bits, 202, *arm)])
            .collect();
        let actual: BTreeSet<_> = seen.iter().filter(|key| key.0 == *bits).copied().collect();
        if actual != expected {
            return Err("missing or unexpected prescribed cell".into());
        }
        if rows.iter().any(|r| r[5] != "succeeded") {
            if rungs.len() != index + 1 {
                return Err("execution beyond frozen stopping rule".into());
            }
            return Ok(());
        }
    }
    if rungs.len() != fields.len() {
        return Err("unexpected rung".into());
    }
    Ok(())
}
fn regression_controls(raw: &str, version: u32) {
    let rows: Vec<_> = raw.lines().map(str::to_owned).collect();
    let mut duplicate = rows.clone();
    duplicate.push(rows[1].clone());
    assert_eq!(
        check(&duplicate.join("\n"), version).unwrap_err(),
        "duplicate execution"
    );
    let mut missing = rows.clone();
    missing.remove(1);
    assert!(check(&missing.join("\n"), version).is_err());
    let first = rows[1].split(',').nth(1).unwrap();
    let prefix: Vec<_> = rows
        .iter()
        .enumerate()
        .filter(|(i, s)| *i == 0 || s.split(',').nth(1) == Some(first))
        .map(|(_, s)| s.clone())
        .collect();
    if prefix
        .iter()
        .skip(1)
        .all(|s| s.split(',').nth(5) == Some("succeeded"))
    {
        assert_eq!(
            check(&prefix.join("\n"), version).unwrap_err(),
            "premature truncation without recorded stopping condition"
        );
    } else {
        // An incomplete first rung is a valid terminal record, not truncation.
        check(&prefix.join("\n"), version).unwrap();
    }
    let mut stopped_prefix = prefix.clone();
    let mut limited: Vec<_> = stopped_prefix[1].split(',').map(str::to_owned).collect();
    limited[5] = "limited".into();
    limited[6] = "not_run".into();
    limited[7] = "0".into();
    stopped_prefix[1] = limited.join(",");
    check(&stopped_prefix.join("\n"), version).unwrap();
    // Even one cell beyond that complete but unsuccessful rung is forbidden.
    let mut after_stop = stopped_prefix;
    let mut later: Vec<_> = rows[1].split(',').map(str::to_owned).collect();
    later[1] = match version {
        3 => "24",
        4 => "52",
        5 | 7 | 8 => "56",
        9 | 11 | 12 | 13 => "61",
        14..=21 => "63",
        _ => unreachable!(),
    }
    .into();
    after_stop.push(later.join(","));
    assert_eq!(
        check(&after_stop.join("\n"), version).unwrap_err(),
        "execution beyond frozen stopping rule"
    );
    for (column, replacement) in [(2, "999"), (3, "9"), (4, "true"), (0, "z"), (6, "invalid")] {
        let mut corrupt = rows.clone();
        let mut values: Vec<_> = corrupt[1].split(',').map(str::to_owned).collect();
        values[column] = replacement.into();
        corrupt[1] = values.join(",");
        assert!(check(&corrupt.join("\n"), version).is_err());
    }
}
fn main() {
    let a: Vec<_> = env::args().skip(1).collect();
    assert!(
        !a.is_empty() && a.len() % 2 == 0,
        "usage: check-frontier VERSION ROWS.csv [VERSION ROWS.csv ...]"
    );
    for pair in a.chunks_exact(2) {
        let version = pair[0].parse().unwrap();
        let raw = fs::read_to_string(&pair[1]).unwrap();
        check(&raw, version).unwrap();
        regression_controls(&raw, version);
        println!("PASS v{version} {}: exact cells, seeds, replicates, outcomes and full plan or recorded stop; accepts first-rung limit; rejects duplicate, missing cell, truncated successful prefix, continuation after stop, wrong seed/replicate/warmup/arm/verification",pair[1]);
    }
}
