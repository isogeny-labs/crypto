//! Complete original-prefix collector, with an independent post-run group oracle.
use super::*;
type Terms = Vec<(usize, i8)>;
type Key = (E, E, bool);
fn key(p: Point) -> Key {
    (p.x, p.y, p.inf)
}
fn signed(c: &Curve, p: Point, s: i8) -> Point {
    if s == 1 {
        p
    } else {
        c.neg(p)
    }
}

fn oracle_pairs(c: &Curve, base: &[Point], l0: usize) -> BTreeMap<Key, Vec<Terms>> {
    let mut pairs = BTreeMap::<Key, Vec<Terms>>::new();
    for j in 1..l0 {
        for i in 0..j {
            for sj in [-1, 1] {
                for si in [-1, 1] {
                    let sum = c.add(signed(c, base[j], sj), signed(c, base[i], si));
                    pairs
                        .entry(key(sum))
                        .or_default()
                        .push(vec![(i, si), (j, sj)]);
                }
            }
        }
    }
    pairs
}
fn oracle(
    c: &Curve,
    base: &[Point],
    l0: usize,
    pairs: &BTreeMap<Key, Vec<Terms>>,
    r: Point,
) -> BTreeSet<Terms> {
    let mut out = BTreeSet::new();
    for j in l0..base.len() {
        for sj in [-1, 1] {
            let u = c.add(r, c.neg(signed(c, base[j], sj)));
            for i in 0..l0 {
                for si in [-1, 1] {
                    let needed = c.add(u, c.neg(signed(c, base[i], si)));
                    if let Some(first) = pairs.get(&key(needed)) {
                        for pair in first {
                            if pair.iter().any(|&(id, _)| id == i) {
                                continue;
                            }
                            let mut terms = pair.clone();
                            terms.extend([(i, si), (j, sj)]);
                            terms.sort();
                            out.insert(terms);
                        }
                    }
                }
            }
        }
    }
    out
}
fn rank_solution(rows: &[Vec<u64>], cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    let mut a = rows.to_vec();
    let mut rank = 0;
    for col in 0..cols {
        let Some(pivot) = (rank..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(rank, pivot);
        let inv = fpow(a[rank][col], n - 2, n);
        for j in col..=cols {
            a[rank][j] = a[rank][j] * inv % n;
        }
        for i in 0..a.len() {
            if i != rank {
                let factor = a[i][col];
                for j in col..=cols {
                    a[i][j] = (a[i][j] + n - factor * a[rank][j] % n) % n;
                }
            }
        }
        rank += 1;
        if rank == a.len() {
            break;
        }
    }
    for row in a.iter().skip(rank) {
        assert_eq!(row[cols], 0, "inconsistent candidate relation");
    }
    (
        rank,
        if rank == cols {
            Some(a.iter().take(cols).map(|r| r[cols]).collect())
        } else {
            None
        },
    )
}
fn relation_row(terms: &Terms, raw: &Value, item: &Value, cols: usize, n: u64) -> Vec<u64> {
    let mut sum = vec![0; cols];
    for &(id, s) in terms {
        let col = raw["orbit_column"][id].as_u64().unwrap() as usize;
        let v = raw["orbit_coefficient"][id].as_u64().unwrap();
        sum[col] = (sum[col] + if s == 1 { v } else { n - v }) % n;
    }
    let mut row = sum[1..].to_vec();
    row.push((n - item["b"].as_u64().unwrap()) % n);
    row.push(
        (item["a"].as_u64().unwrap() + n - raw["anchor_log"].as_u64().unwrap() * sum[0] % n) % n,
    );
    row
}
fn write_json(p: &Path, v: &Value) {
    std::fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}

pub(super) fn run(fixture_path: &Path, expected_sha: &str, root: &Path, out: &Path) {
    let bytes = std::fs::read(fixture_path).unwrap();
    assert_eq!(
        hex::encode(hash::sha256::sha256(&bytes)),
        expected_sha,
        "profile digest mismatch before solving"
    );
    let profile: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(profile["status"], "pass");
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let (_, h) = integer::derive();
    let mut results = Vec::new();
    for run in profile["runs"].as_array().unwrap() {
        let p = run["p"].as_u64().unwrap();
        let seed = run["seed"].as_u64().unwrap();
        let raw_bytes =
            std::fs::read(root.join(format!("quartic_twist_k1_scale_p{p}_seed{seed}_v1.json")))
                .unwrap();
        assert_eq!(
            hex::encode(hash::sha256::sha256(&raw_bytes)),
            run["input_sha256"].as_str().unwrap(),
            "frozen raw input changed"
        );
        let raw: Value = serde_json::from_slice(&raw_bytes).unwrap();
        let f = Field::new(p);
        let c = Curve {
            f,
            b: [0, 0, 0, raw["b0"].as_u64().unwrap()],
        };
        let g = Point {
            x: element(&raw["generator_x"]),
            y: element(&raw["generator_y"]),
            inf: false,
        };
        let q = Point {
            x: element(&raw["target_x"]),
            y: element(&raw["target_y"]),
            inf: false,
        };
        let base: Vec<_> = raw["base_coordinates"]
            .as_array()
            .unwrap()
            .iter()
            .map(point)
            .collect();
        assert!(base.iter().copied().all(|p| c.on_curve(p)));
        assert!(c.on_curve(g) && c.on_curve(q));
        let l0 = base.iter().take_while(|pt| pt.x[2] == 0).count();
        let n = raw["n"].as_u64().unwrap();
        let cols = raw["folded_columns"].as_u64().unwrap() as usize;
        let reps: Vec<_> = raw["orbit_representatives"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as usize)
            .collect();
        assert_eq!(reps.len(), cols);
        assert_eq!(reps[0], 0);
        assert_eq!(c.mul(g, raw["anchor_log"].as_u64().unwrap()), base[0]);
        for (id, &pt) in base.iter().enumerate() {
            let col = raw["orbit_column"][id].as_u64().unwrap() as usize;
            assert_eq!(
                c.mul(
                    base[reps[col]],
                    raw["orbit_coefficient"][id].as_u64().unwrap()
                ),
                pt
            );
        }
        let dir = out.join(format!("p{p}-seed{seed}"));
        std::fs::create_dir_all(&dir).unwrap();
        let mut traces = Vec::new();
        let mut sets = Vec::new();
        let mut residuals = Vec::new();
        let mut rows = Vec::new();
        let mut first_full = None;
        let mut total_systems = 0;
        let mut total_muls = 0;
        for (qi, item) in raw["trace"].as_array().unwrap().iter().enumerate() {
            let r = c.add(
                c.mul(g, item["a"].as_u64().unwrap()),
                c.mul(q, item["b"].as_u64().unwrap()),
            );
            residuals.push(r);
            let mut got = BTreeSet::new();
            let mut systems = 0;
            let mut infinity = 0;
            let mut muls = 0;
            let mut undetermined = Vec::new();
            let mut split_failures = 0;
            let mut foreign = 0;
            let mut field_roots = 0;
            for id in l0..base.len() {
                for sign in [-1, 1] {
                    let u = c.add(r, c.neg(signed(&c, base[id], sign)));
                    if u.inf {
                        infinity += 1;
                        continue;
                    }
                    systems += 1;
                    let equations = restrict_exact(f, c.b, &h, u.x);
                    let input: Vec<_> = equations
                        .iter()
                        .map(|poly| {
                            f4_fp::normalise(
                                &poly
                                    .iter()
                                    .map(|(e, &v)| (e.iter().map(|&x| x as u32).collect(), v))
                                    .collect::<Vec<_>>(),
                                p,
                                f4_fp::Ordering::Grevlex,
                            )
                        })
                        .collect();
                    let opts = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 12)
                        .with_budget(Duration::from_secs(1200));
                    let solved = f4_fp::solve(&input, 3, p, &opts);
                    muls += solved.field_ops;
                    let roots = match solved.verdict {
                        f4_fp::Verdict::Solutions(s) => s,
                        f4_fp::Verdict::Inconsistent => Vec::new(),
                        f4_fp::Verdict::Undetermined => {
                            undetermined.push(json!({"peel_id":id,"sign":sign,"equations":equations.iter().map(|poly|poly.iter().map(|(e,c)|json!([e,c])).collect::<Vec<_>>()).collect::<Vec<_>>() }));
                            Vec::new()
                        }
                    };
                    field_roots += roots.len();
                    for e in roots {
                        let xs: Vec<_> = (0..p)
                            .filter(|&x| (((x + p - e[0]) * x + e[1]) % p * x + p - e[2]) % p == 0)
                            .collect();
                        if xs.len() != 3 {
                            split_failures += 1;
                            continue;
                        }
                        let ids: Option<Vec<_>> = xs
                            .iter()
                            .map(|&x| base[..l0].iter().position(|pt| pt.x == f.scalar(x)))
                            .collect();
                        let Some(ids) = ids else {
                            foreign += 1;
                            continue;
                        };
                        for mask in 0..8 {
                            let mut sum = signed(&c, base[id], sign);
                            let mut terms = vec![(id, sign)];
                            for (j, &k) in ids.iter().enumerate() {
                                let s = if mask & (1 << j) == 0 { 1 } else { -1 };
                                sum = c.add(sum, signed(&c, base[k], s));
                                terms.push((k, s));
                            }
                            if sum == r {
                                terms.sort();
                                got.insert(terms);
                            }
                        }
                    }
                }
            }
            for terms in &got {
                let row = relation_row(terms, &raw, item, cols, n);
                if rows.len() < cols {
                    let mut next = rows.clone();
                    next.push(row);
                    if rank_solution(&next, cols, n).0 > rows.len() {
                        rows = next;
                    }
                }
            }
            let rank = rank_solution(&rows, cols, n).0;
            if rank == cols && first_full.is_none() {
                first_full = Some(qi);
            }
            let record = json!({"query_index":qi,"a":item["a"],"b":item["b"],"signed_relations":got,"systems":systems,"infinity_peels":infinity,
                "undetermined":undetermined,"field_roots":field_roots,"split_failures":split_failures,"foreign":foreign,
                "f4_row_reduction_fp_muls":muls,"rank_after":rank});
            write_json(&dir.join(format!("q{qi:04}.json")), &record);
            total_systems += systems;
            total_muls += muls;
            traces.push(record);
            sets.push(got);
            eprintln!("p={p} seed={seed} query={qi} systems={systems} rank={rank}/{cols}");
        }
        // Reference group sums and all reference hits are constructed only now.
        let pairs = oracle_pairs(&c, &base, l0);
        let mut differences = Vec::new();
        let mut oracle_total = 0;
        let mut candidate_total = 0;
        let mut old_total = 0;
        for (qi, (&r, got)) in residuals.iter().zip(&sets).enumerate() {
            let expected = oracle(&c, &base, l0, &pairs, r);
            oracle_total += expected.len();
            candidate_total += got.len();
            old_total += raw["trace"][qi]["patterns"][0]["signed_relations"]
                .as_array()
                .unwrap()
                .len();
            if *got != expected {
                differences.push(json!({"query_index":qi,"missing":expected.difference(got).collect::<Vec<_>>(),"extra":got.difference(&expected).collect::<Vec<_>>() }));
            }
        }
        let (rank, solution) = rank_solution(&rows, cols, n);
        let mut verified = false;
        let recovered = solution.as_ref().map(|v| v[cols - 1]);
        if let Some(logs) = &solution {
            let mut rep_logs = vec![raw["anchor_log"].as_u64().unwrap()];
            rep_logs.extend(&logs[..cols - 1]);
            verified = base.iter().enumerate().all(|(id, &pt)| {
                c.mul(
                    g,
                    rep_logs[raw["orbit_column"][id].as_u64().unwrap() as usize]
                        * raw["orbit_coefficient"][id].as_u64().unwrap()
                        % n,
                ) == pt
            }) && c.mul(g, logs[cols - 1]) == q;
        }
        let undetermined: usize = traces
            .iter()
            .map(|v| v["undetermined"].as_array().unwrap().len())
            .sum();
        let pass = differences.is_empty() && undetermined == 0 && rank == cols && verified;
        let result = json!({"p":p,"seed":seed,"status":if pass{"pass"}else{"partial"},"queries":traces.len(),"systems":total_systems,
            "signed_relations":candidate_total,"oracle_relations":oracle_total,"old_baseline_relations":old_total,
            "differences":differences,"undetermined":undetermined,"anchored_rank":rank,"columns":cols,"first_full_rank_query":first_full,
            "recovered":recovered,"target_and_all_factor_logs_verified":verified,"f4_row_reduction_fp_muls":total_muls,"input_sha256":run["input_sha256"]});
        write_json(&dir.join("summary.json"), &result);
        results.push(result);
    }
    let pass = results.iter().all(|v| v["status"] == "pass");
    let record = json!({"schema":"quartic-twist-s4-f4-prefix-collector/v1","status":if pass{"pass"}else{"partial"},"profile_sha256":expected_sha,"runs":results});
    write_json(&out.join("summary.json"), &record);
    assert!(pass, "prefix discrepancy retained");
}
