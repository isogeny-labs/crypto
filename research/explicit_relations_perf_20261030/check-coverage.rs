// Post-run coverage audit; frozen analyzers and raw evidence stay unchanged.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};

fn check(raw: &str) -> Result<(), String> {
    let mut lines = raw.lines();
    let header = lines.next().ok_or("missing header")?;
    let columns: Vec<_> = header.split(',').collect();
    if columns.len() != 15
        || columns[0..6]
            != [
                "arm",
                "instance_id",
                "seed",
                "replicate",
                "warmup",
                "verification",
            ]
        || !["normalization_muls", "transfer_bytes"].contains(&columns[6])
    {
        return Err("unexpected header".into());
    }
    let mut seen = BTreeSet::new();
    let mut witnesses = BTreeMap::new();
    for line in lines {
        let v: Vec<_> = line.split(',').collect();
        if v.len() != 15 {
            return Err("invalid column count".into());
        }
        let integer = |i: usize| {
            v[i].parse::<usize>()
                .map_err(|_| format!("invalid integer column {i}"))
        };
        let (instance, seed, round) = (integer(1)?, integer(2)?, integer(3)?);
        if !["u", "v", "w", "x"].contains(&v[0]) || instance >= 3 || round >= 10 {
            return Err("execution outside prescribed grid".into());
        }
        if !seen.insert((v[0], instance, round)) {
            return Err("duplicate execution".into());
        }
        if seed != 111 * (instance + 1) {
            return Err("wrong fixture seed".into());
        }
        if v[4] != if round == 0 { "true" } else { "false" } {
            return Err("wrong warmup flag".into());
        }
        if v[5] != "passed" || v[14] != "L0" {
            return Err("verification or isolation mismatch".into());
        }
        for i in [7, 8, 9, 10, 13] {
            let x = v[i].parse::<f64>().map_err(|_| "invalid timing")?;
            if !x.is_finite() || x < 0.0 {
                return Err("invalid timing".into());
            }
        }
        let pair = (
            v[11].parse::<u64>().map_err(|_| "invalid witness digest")?,
            v[12]
                .parse::<u64>()
                .map_err(|_| "invalid normalization digest")?,
        );
        if let Some(old) = witnesses.insert((instance, round), pair) {
            if old != pair {
                return Err("cross-arm digest mismatch".into());
            }
        }
    }
    for arm in ["u", "v", "w", "x"] {
        for instance in 0..3 {
            for round in 0..10 {
                if !seen.contains(&(arm, instance, round)) {
                    return Err(format!("missing execution {arm}/{instance}/{round}"));
                }
            }
        }
    }
    Ok(())
}

fn regression_controls(raw: &str) {
    let rows: Vec<_> = raw.lines().map(str::to_owned).collect();
    let measured = rows
        .iter()
        .position(|s| s.starts_with("v,0,111,9,"))
        .unwrap();
    let donor = rows
        .iter()
        .position(|s| s.starts_with("v,0,111,8,"))
        .unwrap();
    let mut duplicate = rows.clone();
    duplicate[measured] = rows[donor].clone();
    assert_eq!(
        check(&duplicate.join("\n")).unwrap_err(),
        "duplicate execution"
    );
    let missing: Vec<_> = rows
        .iter()
        .filter(|s| !s.contains(",0,true,passed,"))
        .cloned()
        .collect();
    assert!(check(&missing.join("\n"))
        .unwrap_err()
        .starts_with("missing execution"));
    for (label, column, replacement, expected) in [
        ("seed", 2, "112", "wrong fixture seed"),
        ("warmup", 4, "true", "wrong warmup flag"),
        ("arm", 0, "z", "execution outside prescribed grid"),
        ("digest", 11, "0", "cross-arm digest mismatch"),
    ] {
        let mut corrupt = rows.clone();
        let mut fields: Vec<_> = corrupt[measured].split(',').map(str::to_owned).collect();
        fields[column] = replacement.into();
        corrupt[measured] = fields.join(",");
        assert_eq!(check(&corrupt.join("\n")).unwrap_err(), expected, "{label}");
    }
}
fn main() {
    for path in env::args().skip(1) {
        let raw = fs::read_to_string(&path).unwrap();
        check(&raw).unwrap();
        regression_controls(&raw);
        println!("PASS {path}: exact 120-key grid, 12 warmups, 108 measured, fixed seeds, cross-arm digests; rejects duplicate, missing warmups, wrong seed/flag/arm/digest");
    }
}
