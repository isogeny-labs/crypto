//! Enumerate rational assignments of supplied affine/quadratic basis equations.
//! Completeness is RELATIVE TO THOSE EQUATIONS. No ideal-equivalence certificate
//! is produced. Unsupported residuals and resource caps are unknown, not UNSAT.
use super::*;
use std::collections::BTreeSet;

fn quadratic_roots(f: F, a: u64, b: u64, c: u64) -> Vec<u64> {
    if a == 0 {
        return if b == 0 {
            vec![]
        } else {
            vec![f.mul(f.neg(c), f.inv(b))]
        };
    }
    let disc = f.sub(f.mul(b, b), f.mul(4, f.mul(a, c)));
    let Some(s) = f.sqrt(disc) else { return vec![] };
    let inverse = f.inv(f.mul(2, a));
    let mut roots = vec![
        f.mul(f.add(f.neg(b), s), inverse),
        f.mul(f.sub(f.neg(b), s), inverse),
    ];
    roots.sort_unstable();
    roots.dedup();
    roots
}

struct Limits {
    nodes: usize,
    solutions: usize,
}

fn visit(
    f: F,
    n: usize,
    eqs: Vec<MP>,
    active: Vec<usize>,
    limit: &mut Limits,
) -> Result<Vec<Vec<u64>>, String> {
    if limit.nodes == 0 {
        return Err("node limit".into());
    }
    limit.nodes -= 1;
    if eqs.iter().any(|p| p.deg() == 0 && !p.is_zero()) {
        return Ok(vec![]);
    }
    let eqs: Vec<MP> = eqs.into_iter().filter(|p| !p.is_zero()).collect();
    if active.is_empty() {
        return Ok(vec![vec![0; n]]);
    }
    if eqs.iter().any(|p| p.t.len() > 100_000) {
        return Err("term limit".into());
    }
    for (k, p) in eqs.iter().enumerate().filter(|(_, p)| p.deg() == 1) {
        for &v in &active {
            if let Some(rhs) = reduction::pivot(p, f, v) {
                let next = eqs
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != k)
                    .map(|(_, p)| reduction::substitute(p, f, v, &rhs))
                    .collect();
                let mut solutions = visit(
                    f,
                    n,
                    next,
                    active.iter().copied().filter(|&w| w != v).collect(),
                    limit,
                )?;
                for point in &mut solutions {
                    point[v] = rhs.eval(f, point);
                }
                return Ok(solutions);
            }
        }
    }
    for p in &eqs {
        let vars: BTreeSet<usize> =
            p.t.keys()
                .flat_map(|e| {
                    e.iter()
                        .enumerate()
                        .filter_map(|(i, &k)| (k > 0).then_some(i))
                })
                .collect();
        if vars.len() != 1 || p.deg() > 2 {
            continue;
        }
        let v = *vars.first().unwrap();
        let mut coeff = [0; 3];
        for (e, &c) in &p.t {
            coeff[e[v] as usize] = c;
        }
        let roots = quadratic_roots(f, coeff[2], coeff[1], coeff[0]);
        let mut result = vec![];
        for root in roots {
            let rhs = MP::konst(n, root);
            let next = eqs
                .iter()
                .map(|p| reduction::substitute(p, f, v, &rhs))
                .collect();
            let mut branch = visit(
                f,
                n,
                next,
                active.iter().copied().filter(|&w| w != v).collect(),
                limit,
            )?;
            for p in &mut branch {
                p[v] = root;
            }
            if result.len() + branch.len() > limit.solutions {
                return Err("solution limit".into());
            }
            result.extend(branch);
        }
        return Ok(result);
    }
    Err("unsupported coupled/higher-degree residual or free variable".into())
}

fn enumerate(
    f: F,
    n: usize,
    eqs: &[MP],
    nodes: usize,
    solutions: usize,
) -> Result<Vec<Vec<u64>>, String> {
    if eqs.iter().any(|p| p.n != n) {
        return Err("basis dimension mismatch".into());
    }
    let mut result = visit(
        f,
        n,
        eqs.to_vec(),
        (0..n).collect(),
        &mut Limits { nodes, solutions },
    )?;
    result.sort();
    result.dedup();
    if result.len() > solutions {
        return Err("solution limit".into());
    }
    if result
        .iter()
        .any(|point| eqs.iter().any(|p| p.eval(f, point) != 0))
    {
        return Err("assignment fails supplied basis".into());
    }
    Ok(result)
}

fn collect(metadata: &Value, text: &str, gb: &str) -> Result<Value, String> {
    extraction::validate_query(metadata)?;
    let summary = &metadata["summary"];
    if !matches!(summary["system"].as_str(), Some("rr" | "f")) {
        return Err("collect supports RR and F only".into());
    }
    if summary["sha256"] != sha256_hex(text.as_bytes()) {
        return Err("input digest mismatch".into());
    }
    let mut lines = text.lines();
    let names: Vec<String> = lines
        .next()
        .ok_or("missing names")?
        .split(',')
        .map(String::from)
        .collect();
    if names.is_empty()
        || names.iter().any(String::is_empty)
        || names.iter().collect::<BTreeSet<_>>().len() != names.len()
    {
        return Err("invalid input variable order".into());
    }
    let p: u64 = lines
        .next()
        .ok_or("missing modulus")?
        .parse()
        .map_err(|_| "invalid modulus")?;
    if p <= 3 || p >= 1u64 << 32 || !is_prime(p) || summary["instance"]["p"] != p {
        return Err("input modulus mismatch".into());
    }
    let characteristic = gb
        .lines()
        .filter_map(|s| s.strip_prefix("#field characteristic:"))
        .map(str::trim)
        .collect::<Vec<_>>();
    let order = gb
        .lines()
        .filter_map(|s| s.strip_prefix("#variable order:"))
        .map(str::trim)
        .collect::<Vec<_>>();
    if characteristic != vec![p.to_string().as_str()] || order != vec![names.join(", ").as_str()] {
        return Err("missing or mismatched basis field/variable header".into());
    }
    let f = F::new(p);
    let basis = parse_gb(gb, &names, f);
    let binding = json!({"input_sha256":summary["sha256"],"basis_sha256":sha256_hex(gb.as_bytes()),
        "input_completeness":"not certified: supplied basis equivalence to input requires trusted provenance and independent qualification"});
    let assignments = match enumerate(f, names.len(), &basis, 10_000, 4096) {
        Ok(v) => v,
        Err(reason) => {
            return Ok(
                json!({"schema":"p1tool.collect/v1","status":"unknown","reason":reason,
            "binding":binding,"assignments":null,"decompositions":null,"standing":"producer-unverified"}),
            )
        }
    };
    let mut outputs = BTreeSet::new();
    let mut errors = vec![];
    for (index, point) in assignments.iter().enumerate() {
        match extraction::checked_extract(metadata, text, point) {
            Ok(value) => {
                let points: Vec<Vec<(u64, u64)>> =
                    serde_json::from_value(value["decompositions"].clone())
                        .map_err(|e| e.to_string())?;
                outputs.extend(points);
            }
            Err(reason) => errors.push(json!({"assignment_index":index,"reason":reason})),
        }
    }
    Ok(
        json!({"schema":"p1tool.collect/v1","status":"enumerated_supplied_basis",
        "normalization_status":if errors.is_empty(){"ok"}else{"unknown"},
        "binding":binding,"variable_order":names,"assignments":assignments,
        "decompositions":outputs.into_iter().collect::<Vec<_>>(),"extraction_errors":errors,
        "standing":"producer-unverified"}),
    )
}

pub(super) fn cmd_collect(args: &HashMap<String, String>) {
    let read = |key: &str| {
        std::fs::read_to_string(
            args.get(key)
                .unwrap_or_else(|| die(&format!("missing --{key}"))),
        )
        .unwrap_or_else(|e| die(&e.to_string()))
    };
    let metadata: Value =
        serde_json::from_str(&read("metadata")).unwrap_or_else(|e| die(&e.to_string()));
    let result = collect(&metadata, &read("system-file"), &read("gb")).unwrap_or_else(|e| die(&e));
    println!("{result}");
    if result["status"] == "unknown" || result["normalization_status"] != "ok" {
        exit(3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn affine_quadratic_sets_match_exhaustive_reference() {
        let f = F::new(7);
        let n = 2;
        let x = MP::var(n, 0);
        let y = MP::var(n, 1);
        for a in 1..7 {
            for b in 0..7 {
                for c in 0..7 {
                    let eqs = vec![
                        x.sub(f, &y.scale(f, 2)).sub(f, &MP::konst(n, 3)),
                        y.mul(f, &y)
                            .scale(f, a)
                            .add(f, &y.scale(f, b))
                            .add(f, &MP::konst(n, c)),
                    ];
                    let mut reference = vec![];
                    for xx in 0..7 {
                        for yy in 0..7 {
                            let point = vec![xx, yy];
                            if eqs.iter().all(|p| p.eval(f, &point) == 0) {
                                reference.push(point);
                            }
                        }
                    }
                    assert_eq!(enumerate(f, n, &eqs, 100, 10).unwrap(), reference);
                }
            }
        }
    }
    #[test]
    fn unsat_unknown_and_caps_stay_distinct() {
        let f = F::new(7);
        let x = MP::var(2, 0);
        let y = MP::var(2, 1);
        assert!(enumerate(f, 2, &[MP::konst(2, 1)], 100, 10)
            .unwrap()
            .is_empty());
        assert!(enumerate(f, 2, &[], 100, 10).is_err());
        let coupled = vec![x.mul(f, &x).sub(f, &y), y.mul(f, &y).sub(f, &x)];
        assert!(enumerate(f, 2, &coupled, 100, 10).is_err());
        let partial = vec![
            x.mul(f, &x).sub(f, &MP::konst(2, 1)),
            x.add(f, &MP::konst(2, 1)).mul(f, &y),
        ];
        assert!(enumerate(f, 2, &partial, 100, 10).is_err()); // one supported branch, one free branch
        let x = MP::var(1, 0);
        let cubic = x.mul(f, &x).mul(f, &x);
        assert!(enumerate(f, 1, &[cubic], 100, 10).is_err());
        let quadratic = x.mul(f, &x).sub(f, &MP::konst(1, 1));
        assert!(enumerate(f, 1, &[quadratic.clone()], 100, 1).is_err());
        assert!(enumerate(f, 1, &[quadratic], 1, 10).is_err());
    }
    #[test]
    fn archived_two_solution_basis_is_collected_without_planted_coordinates() {
        let text = include_str!("../../calibration/round2/dickson_smoke_f/p1151_m3_t4_s1_f.ms");
        let gb = include_str!("../../calibration/round2/dickson_smoke_f/p1151_m3_t4_s1_f.gb");
        let mut metadata: Value = serde_json::from_str(include_str!(
            "../../calibration/round2/dickson_smoke_f/p1151_m3_t4_s1_f.ms.plant.json"
        ))
        .unwrap();
        metadata.as_object_mut().unwrap().remove("planted");
        let instance = metadata["summary"]["instance"].as_object_mut().unwrap();
        for key in ["w", "x", "y", "tree", "Y"] {
            instance.remove(key);
        }
        let tower = instance["info"]["tower"].clone();
        instance.insert("info".into(), json!({"tower":tower}));
        metadata["summary"]
            .as_object_mut()
            .unwrap()
            .remove("planted_check");
        let result = collect(&metadata, text, gb).unwrap();
        assert_eq!(result["status"], "enumerated_supplied_basis");
        assert_eq!(result["normalization_status"], "ok");
        assert_eq!(result["assignments"].as_array().unwrap().len(), 2);
        assert_eq!(result["decompositions"].as_array().unwrap().len(), 2);
        assert!(result["binding"]["input_completeness"]
            .as_str()
            .unwrap()
            .starts_with("not certified"));
        let bad = gb.replace("#field characteristic: 1151", "#field characteristic: 1279");
        assert!(collect(&metadata, text, &bad).is_err());
        let bad = gb.replace("#variable order:", "#variable order: broken,");
        assert!(collect(&metadata, text, &bad).is_err());
        assert!(collect(&metadata, &format!("{text}\n"), gb).is_err());
        let prefix = gb.split('[').next().unwrap();
        let forged = format!("{prefix}[1]:");
        let result = collect(&metadata, text, &forged).unwrap();
        assert_eq!(result["assignments"].as_array().unwrap().len(), 0);
        assert!(result["binding"]["input_completeness"]
            .as_str()
            .unwrap()
            .starts_with("not certified"));
        let mut bad_metadata = metadata.clone();
        bad_metadata["summary"]["system"] = json!("t");
        assert!(collect(&bad_metadata, text, &forged).is_err());
        bad_metadata["summary"]
            .as_object_mut()
            .unwrap()
            .remove("system");
        assert!(collect(&bad_metadata, text, &forged).is_err());
    }
}
