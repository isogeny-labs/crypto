//! Independent square-table/Euler/group-translation checks of the 600-box data.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Point = Option<(u64, u64)>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn pow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = r * x % p;
        }
        x = x * x % p;
        n >>= 1;
    }
    r
}
fn inverse(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    let (mut r0, mut r1) = (p as i128, x as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert_eq!(r0, 1);
    t0.rem_euclid(p as i128) as u64
}
fn add(a: Point, b: Point, curve_a: u64, p: u64) -> Point {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some((x, y)), Some((u, v))) => {
            if x == u && (y + v) % p == 0 {
                return None;
            }
            let slope = if x == u {
                (3 * x % p * x + curve_a) % p * inverse(2 * y % p, p) % p
            } else {
                (v + p - y) % p * inverse((u + p - x) % p, p) % p
            };
            let z = (slope * slope % p + 2 * p - x - u) % p;
            Some((z, (slope * ((x + p - z) % p) % p + p - y) % p))
        }
    }
}
fn rhs(x: u64, a: u64, b: u64, p: u64) -> u64 {
    (x * x % p * x + a * x + b) % p
}
fn eval(c: &[Vec<u64>], r: u64, u: u64, p: u64) -> u64 {
    c.iter().rev().fold(0, |a, row| {
        (a * r + row.iter().rev().fold(0, |a, c| (a * u + c) % p)) % p
    })
}
fn degree(c: &[Vec<u64>]) -> usize {
    let mut answer = 0;
    for (i, row) in c.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v != 0 {
                answer = answer.max(i + j);
            }
        }
    }
    answer
}
fn run(fixtures: &Path, original: &Path, run: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh frontier audit required".into());
    }
    let f = read(fixtures)?;
    let rows = read(run.join("ROWS.json"))?;
    let primary = read(run.join("SUMMARY.json"))?;
    let mut counts = [0usize; 6];
    let mut targets = BTreeSet::new();
    let mut traces = vec![];
    for row in rows.as_array().ok_or("rows")? {
        let case = row["case"].as_str().unwrap();
        let file = run.join(format!("{case}.json"));
        if row["record_sha256"] != sha(&file)? {
            return Err("raw case changed".into());
        }
        let r = read(&file)?;
        let fixture = f["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["id"] == r["fixture"])
            .ok_or("fixture")?;
        let p = fixture["p"].as_u64().unwrap();
        let a = fixture["a"].as_u64().unwrap();
        let b = fixture["b"].as_u64().unwrap();
        let d = fixture["d"].as_u64().unwrap();
        let h = r["h"].as_u64().unwrap();
        let k = r["k"].as_u64().unwrap();
        let lambda = r["lambda"].as_u64().unwrap();
        if pow(d, (p - 1) / 2, p) != p - 1 {
            return Err("twist parameter is not a nonsquare".into());
        }
        let mut roots = vec![None; p as usize];
        for y in 0..p {
            roots[(y * y % p) as usize].get_or_insert(y);
        }
        let n = 1
            + (0..p)
                .map(|x| {
                    let v = rhs(x, a, b, p);
                    if v == 0 {
                        1
                    } else if roots[v as usize].is_some() {
                        2
                    } else {
                        0
                    }
                })
                .sum::<u64>();
        if fixture["order"] != n || n % 2 == 0 {
            return Err("source order check failed".into());
        }
        let at = a * d % p * d % p;
        let bt = b * d % p * d % p * d % p;
        let us = (0..k)
            .filter(|u| {
                let v = rhs(d * u % p, at, bt, p);
                v == 0 || pow(v, (p - 1) / 2, p) == 1
            })
            .collect::<BTreeSet<_>>();
        if json!(us) != r["U"] {
            return Err("twist U mismatch".into());
        }
        let mut fibers = BTreeMap::<u64, usize>::new();
        let mut raw = BTreeSet::new();
        for rr in 0..h {
            for u in 0..k {
                let x = (rr + p - lambda * u % p) % p;
                raw.insert(x);
                if us.contains(&u) {
                    *fibers.entry(x).or_default() += 1;
                }
            }
        }
        let base = fibers
            .keys()
            .copied()
            .filter(|x| roots[rhs(*x, a, b, p) as usize].is_some())
            .collect::<BTreeSet<_>>();
        if json!(base) != r["base_x"]
            || json!(fibers.keys().collect::<Vec<_>>()) != r["unfiltered_image"]
            || r["raw_box_image_size"] != raw.len()
        {
            return Err("source base/image mismatch".into());
        }
        let c: Vec<Vec<u64>> = serde_json::from_value(r["mask"].clone())?;
        let ordinary: Vec<Vec<u64>> = serde_json::from_value(r["ordinary_mask"].clone())?;
        if c.len() != h as usize || c.iter().any(|v| v.len() != k as usize) {
            return Err("invalid interpolation dimensions".into());
        }
        for rr in 0..h {
            for u in 0..k {
                let in_base = base.contains(&((rr + p - lambda * u % p) % p));
                if eval(&c, rr, u, p) != (in_base && us.contains(&u)) as u64
                    || eval(&ordinary, rr, u, p) != in_base as u64
                {
                    return Err("mask evaluation differs".into());
                }
            }
        }
        if lambda == h && h * k <= p && raw.len() as u64 != h * k {
            return Err("radix collision".into());
        }
        let t = &r["target"];
        let target = Some((t[0][0].as_u64().unwrap(), t[1][0].as_u64().unwrap()));
        let (tx, ty) = target.unwrap();
        if ty == 0 || ty * ty % p != rhs(tx, a, b, p) {
            return Err("invalid anchored target".into());
        }
        targets.insert((r["fixture"].as_str().unwrap().to_string(), target));
        // Different complete oracle: enumerate P1, then translate T-P1 and look up x2.
        let mut pairs = BTreeMap::<Vec<u64>, Vec<u64>>::new();
        for x in &base {
            let y = roots[rhs(*x, a, b, p) as usize].unwrap();
            for y in [y, (p - y) % p] {
                let p2 = add(target, Some((*x, (p - y) % p)), a, p);
                if let Some((x2, y2)) = p2 {
                    if base.contains(&x2) {
                        let (xs, ys) = if *x <= x2 {
                            (vec![*x, x2], vec![y, y2])
                        } else {
                            (vec![x2, *x], vec![y2, y])
                        };
                        pairs.entry(xs).or_insert(ys);
                    }
                }
            }
        }
        if json!(pairs.keys().collect::<Vec<_>>()) != r["canonical_pairs"] {
            return Err("independent translation oracle differs".into());
        }
        let witnesses = r["explicit_y_witnesses"].as_array().unwrap();
        if witnesses.len() != pairs.len() {
            return Err("witness coverage".into());
        }
        for w in witnesses {
            let x: Vec<u64> = serde_json::from_value(w["x"].clone())?;
            let y: Vec<u64> = serde_json::from_value(w["y"].clone())?;
            if pairs.get(&x) != Some(&y)
                || add(Some((x[0], y[0])), Some((x[1], y[1])), a, p) != target
            {
                return Err("explicit y differs from group translation".into());
            }
        }
        let raw_count = pairs
            .keys()
            .map(|x| (if x[0] == x[1] { 1 } else { 2 }) * fibers[&x[0]] * fibers[&x[1]])
            .sum::<usize>();
        if r["F3_pair_image_solutions"] != raw_count {
            return Err("fiber-weighted solution count differs".into());
        }
        let direct = base.len().max(4);
        let full = (h as usize).max(k as usize).max(degree(&c)).max(4);
        let reduced = (h as usize).max(us.len()).max(4);
        if row["direct_degree"] != direct
            || row["full_degree"] != full
            || row["reduced_U_degree"] != reduced
        {
            return Err("degree aggregate differs".into());
        }
        let old = r["original_case"] == true;
        if old {
            let p = original.join(format!("{case}.json"));
            let o = read(&p)?;
            if r["original_record_sha256"] != sha(&p)?
                || o["mask"] != r["mask"]
                || o["ordinary_mask"] != r["ordinary_mask"]
            {
                return Err("old Lagrange evidence differs".into());
            }
            counts[0] += 1;
        }
        if base.len() >= 2 && full < direct {
            counts[1] += 1;
            if old {
                counts[3] += 1;
            }
        }
        if base.len() >= 2 && reduced < direct {
            counts[2] += 1;
            if old {
                counts[4] += 1;
            }
        }
        counts[5] += pairs.len();
        traces.push(json!({"case":case,"independent_base_and_mask_checked":true,"translation_pair_oracle_equal":true,"raw_solutions":raw_count,"direct_degree":direct,"full_degree":full,"reduced_degree":reduced}));
    }
    let mixed = rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["mixed_counterexample"] == true)
        .count();
    for (key, v) in [
        ("cases", 600),
        ("original_coefficient_checks", counts[0]),
        ("full_mask_degree_reductions", counts[1]),
        ("reduced_U_degree_reductions", counts[2]),
        ("original_270_full_mask_degree_reductions", counts[3]),
        ("original_270_reduced_U_degree_reductions", counts[4]),
        ("explicit_signed_pair_checks", counts[5]),
        ("mixed_prefix_counterexamples", mixed),
    ] {
        if primary[key] != v {
            return Err(format!("aggregate mismatch {key}").into());
        }
    }
    if rows.as_array().unwrap().len() != 600 || counts[0] != 270 {
        return Err("requested grid incomplete".into());
    }
    fs::create_dir_all(out)?;
    let original_gp = fs::read_to_string(run.join("verify.gp"))?;
    if original_gp.matches("S=elladd(E,T,A);").count() != mixed {
        return Err("mixed checker coverage differs".into());
    }
    let insertion="if(!ellisoncurve(E,T)||!ellisoncurve(E,A),error(\"mixed point off curve\"));S=elladd(E,T,A);if(frontier_f3(T[1],A[1],S[1],E[4],E[5])!=0||frontier_f3(S[1],A[1],T[1],E[4],E[5])!=0,error(\"mixed F3 not zero\"));";
    let strict = "frontier_f3(x,y,z,a,b)=(x-y)^2*z^2-2*((x+y)*(x*y+a)+2*b)*z+(x*y-a)^2-4*b*(x+y);\n"
        .to_string() + &original_gp.replace("S=elladd(E,T,A);", insertion);
    fs::write(out.join("verify-strict.gp"), strict)?;
    let report = json!({"schema":"twist.frontier-independent-audit/v1","status":"raw-geometry-and-aggregates-checked-awaiting-strict-PARI","cases":600,"distinct_fixture_target_pairs":targets.len(),"original_coefficient_checks":counts[0],"full_mask_degree_reductions":counts[1],"reduced_U_degree_reductions":counts[2],"original_270_full_mask_degree_reductions":counts[3],"original_270_reduced_U_degree_reductions":counts[4],"explicit_signed_pair_checks":counts[5],"mixed_prefix_counterexamples":mixed,"independent_method":"square tables, Euler tests, extended-Euclid affine addition and T-P1 translation","summary_sha256":sha(run.join("SUMMARY.json"))?,"rows_sha256":sha(run.join("ROWS.json"))?,"strict_replay_script_sha256":sha(out.join("verify-strict.gp"))?,"solver_cost_measured":false,"novelty_established":false});
    fs::write(
        out.join("AUDIT.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    fs::write(
        out.join("CASE_CHECKS.json"),
        serde_json::to_string_pretty(&traces)? + "\n",
    )?;
    println!("{}", report);
    Ok(())
}
fn gate(audit: &Path, prefix: &str) -> Result<()> {
    let a = read(audit.join("AUDIT.json"))?;
    let meta = read(format!("{prefix}.json"))?;
    let stdout = fs::read_to_string(format!("{prefix}.stdout"))?;
    let stderr = fs::read_to_string(format!("{prefix}.stderr"))?;
    if meta["exit_code"] != 0
        || meta["script_sha256"] != a["strict_replay_script_sha256"]
        || meta["script_sha256"] != sha(audit.join("verify-strict.gp"))?
        || meta["stdout_sha256"] != sha(format!("{prefix}.stdout"))?
        || meta["stderr_sha256"] != sha(format!("{prefix}.stderr"))?
    {
        return Err("strict replay receipt differs".into());
    }
    if !stderr
        .lines()
        .all(|l| l.trim().is_empty() || l.contains("Warning: new maximum stack size ="))
    {
        return Err("strict PARI errors".into());
    }
    let marker = stdout
        .lines()
        .find(|l| l.starts_with("ALL_FRONTIER_REPLAY_OK "))
        .ok_or("marker missing")?
        .split_whitespace()
        .collect::<Vec<_>>();
    if marker.len() != 3
        || json!(marker[1].parse::<usize>()?) != a["explicit_signed_pair_checks"]
        || json!(marker[2].parse::<usize>()?) != a["mixed_prefix_counterexamples"]
    {
        return Err("strict coverage differs".into());
    }
    let path = audit.join("ADMISSION.json");
    if path.exists() {
        return Err("fresh admission required".into());
    }
    fs::write(
        path,
        serde_json::to_string_pretty(
            &json!({"schema":"twist.frontier-admission/v1","accepted":true,"audit_sha256":sha(audit.join("AUDIT.json"))?,"CAS_receipt_sha256":sha(format!("{prefix}.json"))?,"explicit_pair_checks":a["explicit_signed_pair_checks"],"mixed_checks":a["mixed_prefix_counterexamples"],"case_count":600,"all_source_and_mixed_points_checked_on_curve":true,"solver_cost_measured":false,"speedup_or_novelty_established":false}),
        )? + "\n",
    )?;
    println!("ALL_FRONTIER_CHECKS_ACCEPTED");
    Ok(())
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("run") if a.len() == 6 => run(
            Path::new(&a[2]),
            Path::new(&a[3]),
            Path::new(&a[4]),
            Path::new(&a[5]),
        ),
        Some("gate") if a.len() == 4 => gate(Path::new(&a[2]), &a[3]),
        _ => {
            Err("usage: run FIXTURES OLD_INCIDENCE RUN FRESH_AUDIT | gate AUDIT CAS_PREFIX".into())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extended_euclid_and_affine_translations() {
        for a in 1..59 {
            assert_eq!(a * inverse(a, 59) % 59, 1);
        }
        let p = Some((1, 29));
        assert_eq!(add(p, Some((1, 30)), 1, 59), None);
        assert_eq!(add(p, None, 1, 59), p);
        let double = add(p, p, 1, 59).unwrap();
        assert_eq!(double.1 * double.1 % 59, rhs(double.0, 1, 13, 59));
    }
}
