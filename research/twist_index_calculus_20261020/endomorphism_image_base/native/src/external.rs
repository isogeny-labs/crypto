//! Identical-input external GB export, completion checks and native timer.
use super::*;
fn read(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn input(v: &Value) -> Vec<f4_fp::Poly> {
    v["equations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|poly| {
            let terms: Vec<(Vec<u32>, u64)> = poly
                .as_array()
                .unwrap()
                .iter()
                .map(|t| {
                    (
                        t[0].as_array()
                            .unwrap()
                            .iter()
                            .map(|x| x.as_u64().unwrap() as u32)
                            .collect(),
                        t[1].as_u64().unwrap(),
                    )
                })
                .collect();
            f4_fp::normalise(&terms, v["p"].as_u64().unwrap(), f4_fp::Ordering::Grevlex)
        })
        .collect()
}
fn render(poly: &Poly) -> String {
    if poly.is_empty() {
        return "0".into();
    }
    poly.iter()
        .map(|(e, c)| {
            let mut term = c.to_string();
            for (i, &v) in e.iter().enumerate() {
                if v != 0 {
                    term.push('*');
                    term.push_str(["x", "y", "z"][i]);
                    if v != 1 {
                        term.push_str(&format!("^{v}"));
                    }
                }
            }
            term
        })
        .collect::<Vec<_>>()
        .join("+")
}
pub(crate) fn export(tape: &Path, sha: &str, out: &Path) {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let (_, h) = integer::derive();
    let mut manifest = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let case = Case::new(load(tape, sha, p, seed), out);
            let residual = case.residual(&case.raw["query_trace"][0]);
            for ai in [0usize, 7, 9] {
                let ids = &case.peels[ai];
                for position in [0, ids.len() / 2, ids.len() - 1] {
                    let id = ids[position];
                    for sign in [-1, 1] {
                        let label = format!(
                            "p{p}-s{seed}-{}-i{position}-{}",
                            ARMS[ai],
                            if sign == 1 { "plus" } else { "minus" }
                        );
                        let u = case.c.add(
                            residual,
                            case.c.neg(signed(&case.c, case.pool[id].p.into(), sign)),
                        );
                        if u.inf {
                            manifest.push(json!({"id":label,"status":"infinity-no-finite-system","p":p,"seed":seed,"arm":ARMS[ai],"peel_id":id,"sign":sign}));
                            continue;
                        }
                        let equations = restrict_exact(case.c.f, case.c.b, &h, u.x);
                        let record = json!({"id":label,"p":p,"seed":seed,"arm":ARMS[ai],"query_index":0,"peel_id":id,"sign":sign,"remaining_x":u.x,"equations":equations.iter().map(|poly|poly.iter().map(|(e,c)|json!([e,c])).collect::<Vec<_>>()).collect::<Vec<_>>()});
                        let json_bytes = serde_json::to_vec_pretty(&record).unwrap();
                        let ms = format!(
                            "x,y,z\n{p}\n{}\n",
                            equations.iter().map(render).collect::<Vec<_>>().join(",\n")
                        );
                        std::fs::write(out.join(format!("{label}.json")), &json_bytes).unwrap();
                        std::fs::write(out.join(format!("{label}.ms")), &ms).unwrap();
                        manifest.push(json!({"id":label,"status":"finite","p":p,"seed":seed,"arm":ARMS[ai],"json_sha256":hex::encode(hash::sha256::sha256(&json_bytes)),"msolve_sha256":hex::encode(hash::sha256::sha256(ms.as_bytes()))}));
                    }
                }
            }
        }
    }
    assert_eq!(manifest.len(), 378);
    write(
        &out.join("manifest.json"),
        &json!({"schema":"quartic-twist-external-inputs/v1","input_sha256":sha,"positions":manifest}),
    );
}

fn canonical(basis: &[f4_fp::Poly], p: u64) -> Vec<f4_fp::Poly> {
    let mut result: Vec<_> = basis
        .iter()
        .map(|poly| {
            let mut poly = f4_fp::normalise(poly, p, f4_fp::Ordering::Grevlex);
            if !poly.is_empty() {
                let inverse = fpow(poly[0].1, p - 2, p);
                for (_, c) in &mut poly {
                    *c = fm(*c, inverse, p);
                }
            }
            poly.sort();
            poly
        })
        .filter(|x| !x.is_empty())
        .collect();
    result.sort();
    result.dedup();
    result
}
fn complete(generators: &[f4_fp::Poly], basis: &[f4_fp::Poly], p: u64) -> bool {
    let order = f4_fp::Ordering::Grevlex;
    if basis.is_empty() {
        return generators.iter().all(|g| g.is_empty());
    }
    if generators
        .iter()
        .any(|g| !f4_fp::reduce(g, basis, p, order).is_empty())
    {
        return false;
    }
    for i in 0..basis.len() {
        for j in 0..i {
            let a = &basis[i];
            let b = &basis[j];
            if a.is_empty() || b.is_empty() {
                return false;
            }
            let lcm: Vec<_> = a[0]
                .0
                .iter()
                .zip(&b[0].0)
                .map(|(a, b)| (*a).max(*b))
                .collect();
            let ma: Vec<_> = lcm.iter().zip(&a[0].0).map(|(a, b)| a - b).collect();
            let mb: Vec<_> = lcm.iter().zip(&b[0].0).map(|(a, b)| a - b).collect();
            let mut s = Vec::new();
            for (e, c) in a {
                s.push((
                    e.iter().zip(&ma).map(|(a, b)| a + b).collect(),
                    fm(*c, fpow(a[0].1, p - 2, p), p),
                ));
            }
            for (e, c) in b {
                let v = fm(*c, fpow(b[0].1, p - 2, p), p);
                s.push((
                    e.iter().zip(&mb).map(|(a, b)| a + b).collect(),
                    if v == 0 { 0 } else { p - v },
                ));
            }
            let s = f4_fp::normalise(&s, p, order);
            if !f4_fp::reduce(&s, basis, p, order).is_empty() {
                return false;
            }
        }
    }
    true
}
pub(crate) fn native(path: &Path, out: &Path) {
    let bytes = std::fs::read(path).unwrap();
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    let p = v["p"].as_u64().unwrap();
    meter::configure(p, 0);
    let equations = input(&v);
    let options =
        f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 12).with_budget(Duration::from_secs(1200));
    let report = f4_fp::f4(&equations, 3, p, &options);
    let checked = complete(&equations, &report.basis, p);
    let status = if !report.timed_out && checked {
        "complete"
    } else {
        "incomplete"
    };
    let record = json!({"schema":"quartic-twist-native-gb/v1","id":v["id"],"p":p,"input_sha256":hex::encode(hash::sha256::sha256(&bytes)),"status":status,"basis":canonical(&report.basis,p),"internal_ms":report.ms,"timed_out":report.timed_out,"pairs_above_bound":report.pairs_above_bound,"s_pairs_and_generators_reduce_to_zero":checked,"row_reduction_fp_muls":report.field_ops});
    write(out, &record);
    assert_eq!(status, "complete");
}

fn parse_msolve(text: &str, p: u64) -> Vec<f4_fp::Poly> {
    let header = format!("#field characteristic: {p}");
    assert!(text.lines().any(|line| line.trim() == header));
    let body: String = text
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("");
    let body = body.trim();
    assert!(body.starts_with('[') && body.ends_with("]:"));
    let body = &body[1..body.len() - 2];
    let mut basis = Vec::new();
    for polynomial in body.split(',').filter(|s| !s.trim().is_empty()) {
        let chars: Vec<_> = polynomial.chars().filter(|c| !c.is_whitespace()).collect();
        let mut pos = 0;
        let mut terms = Vec::new();
        while pos < chars.len() {
            let negative = chars[pos] == '-';
            if chars[pos] == '-' || chars[pos] == '+' {
                pos += 1;
            }
            let mut coefficient = 1u64;
            let mut exponents = vec![0u32; 3];
            let mut had_factor = false;
            while pos < chars.len() && chars[pos] != '+' && chars[pos] != '-' {
                if chars[pos] == '*' {
                    assert!(had_factor);
                    pos += 1;
                    continue;
                }
                if chars[pos].is_ascii_digit() {
                    let mut value = 0u64;
                    while pos < chars.len() && chars[pos].is_ascii_digit() {
                        value = value
                            .checked_mul(10)
                            .unwrap()
                            .checked_add(chars[pos] as u64 - '0' as u64)
                            .unwrap();
                        pos += 1;
                    }
                    coefficient = fm(coefficient, value % p, p);
                    had_factor = true;
                } else {
                    let variable = match chars[pos] {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        other => panic!("unsupported external polynomial token {other}"),
                    };
                    pos += 1;
                    let mut power = 1u32;
                    if pos < chars.len() && chars[pos] == '^' {
                        pos += 1;
                        power = 0;
                        let start = pos;
                        while pos < chars.len() && chars[pos].is_ascii_digit() {
                            power = power
                                .checked_mul(10)
                                .unwrap()
                                .checked_add(chars[pos] as u32 - '0' as u32)
                                .unwrap();
                            pos += 1;
                        }
                        assert!(pos > start);
                    }
                    exponents[variable] = exponents[variable].checked_add(power).unwrap();
                    had_factor = true;
                }
            }
            assert!(had_factor);
            if negative && coefficient != 0 {
                coefficient = p - coefficient;
            }
            terms.push((exponents, coefficient));
        }
        basis.push(f4_fp::normalise(&terms, p, f4_fp::Ordering::Grevlex));
    }
    canonical(&basis, p)
}
fn external_time(text: &str, label: &str) -> f64 {
    let values: Vec<_> = text
        .lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            if words.next() == Some(label) {
                Some(words.next().unwrap().parse::<f64>().unwrap())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(values.len(), 1, "ambiguous external internal timer");
    assert!(values[0] > 0.0, "external timer has insufficient precision");
    values[0]
}
pub(crate) fn benchmark(inputs: &Path, msolve: &Path, out: &Path) {
    use std::process::{Command, Stdio};
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let manifest = read(&inputs.join("manifest.json"));
    let cases = manifest["positions"].as_array().unwrap();
    let binary = std::env::current_exe().unwrap();
    let mut results = Vec::new();
    for pass in [
        "warmup", "first", "aa", "round0", "round1", "round2", "round3", "round4",
    ] {
        let dest = out.join(pass);
        std::fs::create_dir(&dest).unwrap();
        for case in cases {
            if case["status"] != "finite" {
                continue;
            }
            let id = case["id"].as_str().unwrap();
            let source = inputs.join(format!("{id}.json"));
            let ms = inputs.join(format!("{id}.ms"));
            for (path, key) in [(&source, "json_sha256"), (&ms, "msolve_sha256")] {
                assert_eq!(
                    hex::encode(hash::sha256::sha256(&std::fs::read(path).unwrap())),
                    case[key].as_str().unwrap()
                );
            }
            let native_out = dest.join(format!("{id}.native.json"));
            let external_out = dest.join(format!("{id}.msolve.txt"));
            let external_log = dest.join(format!("{id}.msolve.log"));
            let mut statuses = Vec::new();
            let methods = if pass == "round1" || pass == "round3" {
                ["msolve", "native"]
            } else {
                ["native", "msolve"]
            };
            for method in methods {
                let mut command = Command::new("prlimit");
                command.args(["--as=8589934592", "--", "timeout", "1200s"]);
                if method == "native" {
                    command
                        .arg(&binary)
                        .arg("--image-native-gb")
                        .arg(&source)
                        .arg(&native_out);
                } else {
                    command
                        .arg(msolve)
                        .args([
                            "--random-seed",
                            "0",
                            "-t",
                            "1",
                            "-v",
                            "2",
                            "-g",
                            "2",
                            "-r",
                            "1",
                            "-f",
                        ])
                        .arg(&ms)
                        .arg("-o")
                        .arg(&external_out);
                }
                let log = if method == "native" {
                    dest.join(format!("{id}.native.log"))
                } else {
                    external_log.clone()
                };
                let file = std::fs::File::create(&log).unwrap();
                let status = command
                    .stdout(Stdio::from(file.try_clone().unwrap()))
                    .stderr(Stdio::from(file))
                    .status()
                    .unwrap();
                statuses.push(json!({"method":method,"exit_code":status.code()}));
            }
            let checked = std::panic::catch_unwind(|| {
                assert!(statuses.iter().all(|s| s["exit_code"] == 0));
                let native = read(&native_out);
                assert_eq!(native["status"], "complete");
                let p = case["p"].as_u64().unwrap();
                let external = parse_msolve(&std::fs::read_to_string(&external_out).unwrap(), p);
                assert_eq!(
                    serde_json::to_value(&external).unwrap(),
                    native["basis"],
                    "external reduced basis differs at {id}"
                );
                let log = std::fs::read_to_string(&external_log).unwrap();
                let elapsed = external_time(&log, "overall(elapsed)");
                let cpu = external_time(&log, "overall(cpu)");
                json!({"pass":pass,"id":id,"p":p,"seed":case["seed"],"arm":case["arm"],"status":"pass","native_internal_ms":native["internal_ms"],"msolve_internal_ms":elapsed*1000.0,"msolve_internal_cpu_ms":cpu*1000.0,"native_over_msolve":native["internal_ms"].as_f64().unwrap()/(elapsed*1000.0),"basis_equal":true,"basis_sha256":hex::encode(hash::sha256::sha256(&serde_json::to_vec(&external).unwrap()))})
            });
            results.push(match checked {
                Ok(v) => v,
                Err(_) => json!({"pass":pass,"id":id,"status":"failed","exits":statuses}),
            });
        }
    }
    let passed = results.iter().all(|r| r["status"] == "pass");
    let mut by_case = BTreeMap::<String, String>::new();
    for r in &results {
        if r["status"] != "pass" {
            continue;
        }
        let id = r["id"].as_str().unwrap().to_owned();
        let digest = r["basis_sha256"].as_str().unwrap().to_owned();
        if let Some(prior) = by_case.insert(id, digest.clone()) {
            assert_eq!(prior, digest, "external basis changed between passes");
        }
    }
    write(
        &out.join("results.json"),
        &json!({"schema":"quartic-twist-external-benchmark/v1","status":if passed {"pass"} else {"partial"},"records":results,"source_manifest_sha256":hex::encode(hash::sha256::sha256(&std::fs::read(inputs.join("manifest.json")).unwrap())),"boundary":"internal complete reduced GB; source parsing and process startup excluded; same container host and one thread; admission requires outer isolation receipt"}),
    );
    assert!(passed, "external discrepancies retained in results.json");
}

fn stats(mut values: Vec<f64>) -> Value {
    assert!(values.len() == 5 && values.iter().all(|x| x.is_finite() && *x > 0.0));
    values.sort_by(f64::total_cmp);
    json!({"min":values[0],"median":values[2],"max":values[4]})
}
pub(crate) fn summarize(inputs: &Path, job: &Path, out: &Path) {
    assert!(!out.exists());
    let attempt = std::fs::read_to_string(job.join("admitted-attempt.txt")).unwrap();
    let attempt = attempt.trim().parse::<usize>().unwrap();
    let receipt = read(&job.join(format!("attempt{attempt}.isolation.json")));
    assert_eq!(receipt["run"]["exit_status"], 0);
    assert_eq!(receipt["run"]["contended"], false);
    assert!(receipt["left_on_reserved"]["user_threads"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(receipt["reserved_cpus"], receipt["run"]["cpus"]);
    assert_eq!(receipt["reserved_cpus"].as_array().unwrap().len(), 2);
    assert_eq!(receipt["host"]["logical_cpus"], 4);
    assert_eq!(
        receipt["host"]["cpu_model"],
        "Intel(R) Xeon(R) Platinum 8488C"
    );
    let root = job.join(format!("attempt{attempt}"));
    let data = read(&root.join("results.json"));
    let manifest = read(&inputs.join("manifest.json"));
    assert_eq!(data["status"], "pass");
    assert_eq!(
        data["source_manifest_sha256"],
        hex::encode(hash::sha256::sha256(
            &std::fs::read(inputs.join("manifest.json")).unwrap()
        ))
    );
    let records = data["records"].as_array().unwrap();
    let finite: Vec<_> = manifest["positions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["status"] == "finite")
        .collect();
    assert_eq!(records.len(), finite.len() * 8);
    let mut rows = Vec::new();
    let mut nonunit = 0;
    for case in &finite {
        let id = case["id"].as_str().unwrap();
        let p = case["p"].as_u64().unwrap();
        let equations = input(&read(&inputs.join(format!("{id}.json"))));
        let get = |pass: &str| {
            let matches: Vec<_> = records
                .iter()
                .filter(|r| r["id"] == id && r["pass"] == pass)
                .collect();
            assert_eq!(matches.len(), 1);
            matches[0]
        };
        let first = get("first");
        let aa = get("aa");
        let mut nt = Vec::new();
        let mut mt = Vec::new();
        let mut ratios = Vec::new();
        let mut basis_digest = None;
        let mut unit = None;
        for pass in [
            "warmup", "first", "aa", "round0", "round1", "round2", "round3", "round4",
        ] {
            let r = get(pass);
            assert_eq!(r["status"], "pass");
            assert_eq!(r["basis_equal"], true);
            assert_eq!(r["p"], case["p"]);
            assert_eq!(r["seed"], case["seed"]);
            assert_eq!(r["arm"], case["arm"]);
            let actual = read(&root.join(pass).join(format!("{id}.native.json")));
            assert_eq!(actual["status"], "complete");
            let parsed = parse_msolve(
                &std::fs::read_to_string(root.join(pass).join(format!("{id}.msolve.txt"))).unwrap(),
                p,
            );
            assert_eq!(serde_json::to_value(&parsed).unwrap(), actual["basis"]);
            // Canonical serialization sorts terms lexicographically. Division
            // and S-pairs require the declared grevlex leading terms again.
            let ordered: Vec<_> = parsed
                .iter()
                .map(|poly| f4_fp::normalise(poly, p, f4_fp::Ordering::Grevlex))
                .collect();
            assert!(complete(&equations, &ordered, p));
            assert_eq!(
                r["basis_sha256"],
                hex::encode(hash::sha256::sha256(&serde_json::to_vec(&parsed).unwrap()))
            );
            if let Some(d) = &basis_digest {
                assert_eq!(d, &r["basis_sha256"]);
            } else {
                basis_digest = Some(r["basis_sha256"].clone());
            }
            let is_unit = actual["basis"] == json!([[[[0, 0, 0], 1]]]);
            if let Some(u) = unit {
                assert_eq!(u, is_unit);
            } else {
                unit = Some(is_unit);
            }
            let n = r["native_internal_ms"].as_f64().unwrap();
            let m = r["msolve_internal_ms"].as_f64().unwrap();
            assert!(n > 0.0 && m > 0.0);
            if pass.starts_with("round") {
                nt.push(n);
                mt.push(m);
                ratios.push(n / m);
            }
        }
        if unit == Some(false) {
            nonunit += 1;
        }
        rows.push(json!({"id":id,"p":p,"seed":case["seed"],"arm":case["arm"],"unit_ideal":unit.unwrap(),"native_internal_ms":stats(nt),"msolve_internal_ms":stats(mt),"native_over_msolve":stats(ratios),"native_aa_over_first":aa["native_internal_ms"].as_f64().unwrap()/first["native_internal_ms"].as_f64().unwrap(),"msolve_aa_over_first":aa["msolve_internal_ms"].as_f64().unwrap()/first["msolve_internal_ms"].as_f64().unwrap(),"basis_sha256":basis_digest.unwrap()}));
    }
    let mut groups = Vec::new();
    for p in PRIMES {
        for arm in ["original", "phi-l0-active", "double-l0-active"] {
            let subset: Vec<_> = records
                .iter()
                .filter(|r| r["p"] == p && r["arm"] == arm)
                .collect();
            let sum = |pass: &str, key: &str| {
                subset
                    .iter()
                    .filter(|r| r["pass"] == pass)
                    .map(|r| r[key].as_f64().unwrap())
                    .sum::<f64>()
            };
            let mut n = Vec::new();
            let mut m = Vec::new();
            let mut ratio = Vec::new();
            for round in 0..5 {
                let pass = format!("round{round}");
                let a = sum(&pass, "native_internal_ms");
                let b = sum(&pass, "msolve_internal_ms");
                n.push(a);
                m.push(b);
                ratio.push(a / b);
            }
            groups.push(json!({"p":p,"arm":arm,"gb_inputs":subset.len()/8,"native_internal_ms":stats(n),"msolve_internal_ms":stats(m),"native_over_msolve":stats(ratio),"native_aa_over_first":sum("aa","native_internal_ms")/sum("first","native_internal_ms"),"msolve_aa_over_first":sum("aa","msolve_internal_ms")/sum("first","msolve_internal_ms")}));
        }
    }
    std::fs::create_dir_all(out).unwrap();
    write(
        &out.join("summary.json"),
        &json!({"schema":"twist-image-external-summary/v1","status":"pass","input_positions":manifest["positions"].as_array().unwrap().len(),"finite_positions":finite.len(),"nonunit_positions":nonunit,"executions":records.len(),"basis_checks":"all raw canonical bases equal, all S-pairs and input reductions zero; identical digest in all eight passes","source_manifest_sha256":data["source_manifest_sha256"],"host":receipt["host"],"isolation_admitted":true,"rows":rows,"groups":groups,"boundary":data["boundary"],"scope":"sampled complete GB computations, excluding polynomial restriction, roots, group work, matrix, setup and process startup; not an end-to-end solver comparison"}),
    );
    println!(
        "{}",
        json!({"status":"pass","positions":finite.len(),"nonunit_positions":nonunit,"executions":records.len(),"groups":groups})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buchberger_completion_rejects_truncated_generator_set() {
        let o = f4_fp::Ordering::Grevlex;
        let a = f4_fp::normalise(&[(vec![2, 0], 1), (vec![0, 1], 6)], 7, o);
        let b = f4_fp::normalise(&[(vec![1, 1], 1), (vec![0, 0], 6)], 7, o);
        assert!(!complete(
            &[a.clone(), b.clone()],
            &[a.clone(), b.clone()],
            7
        ));
        let report = f4_fp::f4(&[a.clone(), b.clone()], 2, 7, &f4_fp::F4Options::new(o, 12));
        assert!(complete(&[a, b], &report.basis, 7));
    }
    #[test]
    fn external_parser_preserves_mixed_terms_and_signs() {
        let got = parse_msolve("#field characteristic: 37\n[x^2-y,x*y-1,z]:", 37);
        let expected = canonical(
            &[
                vec![(vec![2, 0, 0], 1), (vec![0, 1, 0], 36)],
                vec![(vec![1, 1, 0], 1), (vec![0, 0, 0], 36)],
                vec![(vec![0, 0, 1], 1)],
            ],
            37,
        );
        assert_eq!(got, expected);
        assert_ne!(got, parse_msolve("#field characteristic: 37\n[1]:", 37));
    }
}
