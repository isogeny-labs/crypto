//! Signed multiplicity replay and independent public-point completeness control.
#![allow(dead_code)]
include!("fixture.rs");
use std::time::Instant;

fn budget(start: Instant) {
    assert!(start.elapsed().as_secs() < 1180, "cooperative phase budget");
}
fn normalize(terms: impl IntoIterator<Item = (usize, i8)>) -> Terms {
    let mut weights = BTreeMap::<usize, i16>::new();
    for (id, w) in terms {
        *weights.entry(id).or_default() += i16::from(w);
    }
    weights
        .into_iter()
        .filter_map(|(id, w)| {
            if w == 0 {
                None
            } else {
                Some((id, i8::try_from(w).unwrap()))
            }
        })
        .collect()
}
fn image_start(case: &Case, double: bool) -> usize {
    case.original_len * if double { 2 } else { 1 }
}
fn weighted_physical(case: &Case, terms: &Terms, double: bool) -> bool {
    let start = image_start(case, double);
    let ordinary: u32 = terms
        .iter()
        .filter(|t| t.0 < case.l0)
        .map(|t| u32::from(t.1.unsigned_abs()))
        .sum();
    let images: Vec<_> = terms.iter().filter(|t| t.0 >= case.l0).collect();
    if *terms != normalize(terms.iter().copied())
        || ordinary > 4
        || ordinary % 2 != 0
        || images.len() != 1
        || images[0].0 < start
        || images[0].0 >= start + case.l0
        || images[0].1.unsigned_abs() != 1
        || terms.iter().any(|t| t.0 >= case.pool.len() || t.1 == 0)
    {
        return false;
    }
    terms.iter().fold(O, |sum, &(id, w)| {
        let pt = if w > 0 {
            case.pool[id].point
        } else {
            case.c.neg(case.pool[id].point)
        };
        (0..w.unsigned_abs()).fold(sum, |s, _| case.c.add(s, pt))
    }) == O
}
fn transport(case: &Case, terms: &Terms, map: &[(usize, i8)]) -> Terms {
    normalize(terms.iter().map(|&(id, w)| {
        let (to, sign) = map[id];
        (to, sign * w)
    }))
}
fn weighted_row(case: &Case, terms: &Terms) -> Vec<u64> {
    let mut v = vec![0; case.active];
    for &(id, w) in terms {
        let f = &case.pool[id];
        assert!(f.col < case.active);
        let value = mm(u64::from(w.unsigned_abs()), f.coefficient, case.n);
        v[f.col] = (v[f.col] + if w > 0 { value } else { neg(value, case.n) }) % case.n;
    }
    let mut row = v[1..].to_vec();
    row.push(neg(mm(case.anchor, v[0], case.n), case.n));
    row
}
fn fraction_free(rows: &[Vec<u64>], cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    let mut pivots = BTreeMap::<usize, Vec<u64>>::new();
    for original in rows {
        assert_eq!(original.len(), cols + 1);
        let mut row = original.clone();
        let mut inserted = false;
        for col in 0..cols {
            if row[col] == 0 {
                continue;
            }
            if let Some(pivot) = pivots.get(&col) {
                let (a, b) = (pivot[col], row[col]);
                for j in col..=cols {
                    row[j] = (mm(a, row[j], n) + n - mm(b, pivot[j], n)) % n;
                }
            } else {
                pivots.insert(col, row.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            assert!(row.iter().all(|&x| x == 0), "inconsistent weighted row");
        }
    }
    let solution = if pivots.len() == cols {
        let mut x = vec![0; cols];
        for col in (0..cols).rev() {
            let row = &pivots[&col];
            let mut rhs = row[cols];
            for j in col + 1..cols {
                rhs = (rhs + n - mm(row[j], x[j], n)) % n;
            }
            x[col] = mm(rhs, pow(row[col], n - 2, n), n);
        }
        Some(x)
    } else {
        None
    };
    (pivots.len(), solution)
}
fn matrix(case: &Case, set: &BTreeSet<Terms>, independent: bool) -> Value {
    let rows: Vec<_> = set.iter().map(|t| weighted_row(case, t)).collect();
    let zero = rows.iter().filter(|r| r.iter().all(|&v| v == 0)).count();
    let projective: BTreeSet<_> = rows.iter().filter_map(|r| canonical(r, case.n)).collect();
    let (rank, solution) = if independent {
        fraction_free(&rows, case.active - 1, case.n)
    } else {
        rref(rows, case.active - 1, case.n)
    };
    let point_checked = solution
        .as_ref()
        .map(|x| {
            let mut logs = vec![case.anchor];
            logs.extend(x);
            case.pool
                .iter()
                .filter(|f| f.col < case.active)
                .all(|f| case.c.mul(case.g, mm(logs[f.col], f.coefficient, case.n)) == f.point)
        })
        .unwrap_or(false);
    assert_eq!(point_checked, rank == case.active - 1);
    json!({"rank":rank,"factor_columns":case.active-1,"projective_rows":projective.len(),"projective_augmented_rows":projective,"zero_rows":zero,"factor_logs":solution,"full_rank_point_verified":point_checked})
}
fn cubic_key([a, b, c]: [u64; 3], p: u64) -> [u64; 3] {
    [
        (a + b + c) % p,
        (mm(a, b, p) + mm(a, c, p) + mm(b, c, p)) % p,
        mm(mm(a, b, p), c, p),
    ]
}
fn cubic_map(case: &Case, start: Instant) -> BTreeMap<[u64; 3], [usize; 3]> {
    let p = case.c.f.p;
    let mut map = BTreeMap::new();
    for i in 0..case.l0 {
        budget(start);
        let a = case.pool[i].point.x[0];
        for j in i..case.l0 {
            let b = case.pool[j].point.x[0];
            for k in j..case.l0 {
                let c = case.pool[k].point.x[0];
                let e = cubic_key([a, b, c], p);
                assert!(
                    map.insert(e, [i, j, k]).is_none(),
                    "distinct cubic factorizations"
                );
            }
        }
    }
    assert_eq!(map.len(), case.l0 * (case.l0 + 1) * (case.l0 + 2) / 6);
    map
}
fn horner_equation(eq: &Value, root: [u64; 3], p: u64) -> u64 {
    let terms = eq.as_array().unwrap();
    let mut sizes = [1usize; 3];
    for term in terms {
        let e: [usize; 3] = serde_json::from_value(term[0].clone()).unwrap();
        for j in 0..3 {
            sizes[j] = sizes[j].max(e[j] + 1);
        }
    }
    let mut dense = vec![vec![vec![0u64; sizes[2]]; sizes[1]]; sizes[0]];
    for term in terms {
        let e: [usize; 3] = serde_json::from_value(term[0].clone()).unwrap();
        dense[e[0]][e[1]][e[2]] = (dense[e[0]][e[1]][e[2]] + u(&term[1])) % p;
    }
    let mut value = 0;
    for plane in dense.iter().rev() {
        let mut a = 0;
        for row in plane.iter().rev() {
            let mut b = 0;
            for &coefficient in row.iter().rev() {
                b = (mm(b, root[2], p) + coefficient) % p;
            }
            a = (mm(a, root[1], p) + b) % p;
        }
        value = (mm(value, root[0], p) + a) % p;
    }
    value
}
fn q_value(root: [u64; 3], a: u64, p: u64) -> u64 {
    (pow(a, 3, p) + p - mm(root[0], mm(a, a, p), p) + mm(root[1], a, p) + p - root[2]) % p
}
fn catalog(path: &Path) -> Value {
    let doc = load(path, WORKLOAD_SHA);
    let rows = doc["workloads"].as_array().unwrap();
    assert_eq!(rows.len(), 42);
    let expected: BTreeSet<_> = PRIMES
        .iter()
        .flat_map(|&p| {
            (0..3).flat_map(move |seed| {
                ["phi-l0-active", "double-l0-active"].map(move |a| (p, seed, a.to_string()))
            })
        })
        .collect();
    let actual: BTreeSet<_> = rows
        .iter()
        .map(|r| (u(&r["p"]), u(&r["seed"]), s(&r["arm"]).to_string()))
        .collect();
    assert_eq!(actual, expected);
    doc
}
fn find_row<'a>(doc: &'a Value, id: &str) -> &'a Value {
    doc["workloads"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == id)
        .unwrap()
}
fn decode(row: &Value, out: &Path) {
    let started = Instant::now();
    assert!(!out.exists());
    let raw = load(Path::new(s(&row["public_path"])), s(&row["public_sha256"]));
    let reference = load(
        Path::new(s(&row["reference_path"])),
        s(&row["reference_sha256"]),
    );
    let oldaudit = load(Path::new(s(&row["audit_path"])), s(&row["audit_sha256"]));
    assert_eq!(oldaudit["status"], "pass");
    assert_eq!(oldaudit["candidate_sha256"], row["reference_sha256"]);
    assert_eq!(reference["public_input_sha256"], row["public_sha256"]);
    assert_eq!(reference["status"], "complete-polynomial-enumeration");
    let case = Case::new(&raw);
    let p = case.c.f.p;
    let double = row["arm"] == "double-l0-active";
    let cubes = cubic_map(&case, started);
    let old: BTreeSet<Terms> =
        serde_json::from_value(reference["signed_physical_set"].clone()).unwrap();
    let mut all = BTreeSet::new();
    let mut fp_only = BTreeSet::new();
    let mut non_fp = BTreeSet::new();
    let mut systems = Vec::new();
    let mut witnesses = BTreeMap::<Terms, Value>::new();
    let mut totals = BTreeMap::<&str, u64>::new();
    for (index, record) in reference["systems"].as_array().unwrap().iter().enumerate() {
        budget(started);
        let pair: (usize, i8, usize, i8) = serde_json::from_value(record["pair"].clone()).unwrap();
        let image = image_start(&case, double);
        assert!(pair.0 < case.l0 && pair.2 >= image && pair.2 < image + case.l0);
        assert!([-1, 1].contains(&pair.1) && [-1, 1].contains(&pair.3));
        let residual = case.c.neg(case.c.add(
            case.c.signed(case.pool[pair.0].point, pair.1),
            case.c.signed(case.pool[pair.2].point, pair.3),
        ));
        assert_eq!(
            residual,
            serde_json::from_value::<Point>(record["remaining_point"].clone()).unwrap()
        );
        assert!(!residual.inf);
        let finite_fp = reject(residual);
        let roots: Vec<[u64; 3]> = serde_json::from_value(record["field_roots"].clone()).unwrap();
        assert!(roots.windows(2).all(|w| w[0] < w[1]));
        let eqs = record["equations"].as_array().unwrap();
        assert_eq!(eqs.len(), 4);
        let mut accepted = BTreeSet::new();
        let mut split = 0u64;
        let mut hits = 0u64;
        for &root in &roots {
            assert!(eqs.iter().all(|e| eval(e, root, p) == 0));
            let Some(&ids) = cubes.get(&root) else {
                continue;
            };
            split += 1;
            for mask in 0..8u8 {
                let signs: [i8; 3] =
                    std::array::from_fn(|j| if mask & (1 << j) == 0 { 1 } else { -1 });
                let sum = (0..3).fold(O, |s, j| {
                    case.c
                        .add(s, case.c.signed(case.pool[ids[j]].point, signs[j]))
                });
                if sum != residual {
                    continue;
                }
                hits += 1;
                let terms = normalize(
                    [(pair.0, pair.1), (pair.2, pair.3)]
                        .into_iter()
                        .chain((0..3).map(|j| (ids[j], signs[j]))),
                );
                assert!(weighted_physical(&case, &terms, double));
                accepted.insert(terms.clone());
                witnesses.entry(terms).or_insert_with(||json!({"system_index":index,"root":root,"triple_ids":ids,"sign_mask":mask,"pair":pair}));
            }
        }
        let candidate = if finite_fp {
            Some([residual.x[0], 0, 0])
        } else {
            None
        };
        let q_witness=candidate.map(|root| {
            let values:Vec<_>=eqs.iter().map(|eq|eval(eq,root,p)).collect();
            let q=q_value(root,case.pool[pair.0].point.x[0],p);
            json!({"root":root,"equation_values":values,"q":q,"common_zero_q_nonzero":values.iter().all(|&v|v==0)&&q!=0})
        });
        for terms in &accepted {
            for map in &case.actions {
                let image = transport(&case, terms, map);
                assert!(weighted_physical(&case, &image, double));
                if finite_fp {
                    fp_only.insert(image.clone());
                } else {
                    non_fp.insert(image.clone());
                }
                all.insert(image);
            }
        }
        for (k, v) in [
            ("systems", 1),
            ("saved_field_roots", roots.len() as u64),
            ("split_ordinary_multiset_roots", split),
            ("sign_assignments_checked", split * 8),
            ("zero_sum_presentations", hits),
            ("accepted_representatives", accepted.len() as u64),
            ("finite_fp_systems", u64::from(finite_fp)),
            (
                "q_nonmembership_witnesses",
                u64::from(
                    q_witness
                        .as_ref()
                        .is_some_and(|w| w["common_zero_q_nonzero"] == true),
                ),
            ),
        ] {
            *totals.entry(k).or_default() += v;
        }
        systems.push(json!({"index":index,"pair":pair,"finite_fp":finite_fp,"root_count":roots.len(),"split_multiset_roots":split,"zero_sum_presentations":hits,"accepted_representatives":accepted,"q_witness":q_witness}));
    }
    let distinct: BTreeSet<_> = all
        .iter()
        .filter(|t| case.physical(t, double))
        .cloned()
        .collect();
    assert_eq!(distinct, old, "old five-distinct subset changed");
    let oldmatrix = matrix(&case, &old, false);
    let expanded = matrix(&case, &all, false);
    assert_eq!(oldmatrix["rank"], reference["factor_log_rank"]);
    assert_eq!(
        oldmatrix["projective_rows"],
        reference["unique_projective_augmented_rows"]
    );
    assert_eq!(oldmatrix["factor_logs"], reference["factor_log_solution"]);
    let fresh: BTreeSet<_> = all.difference(&old).cloned().collect();
    assert!(old.is_subset(&non_fp), "distinct-family pruning regression");
    let lost_if_pruned: BTreeSet<_> = all.difference(&non_fp).cloned().collect();
    let filtered_matrix = matrix(&case, &non_fp, false);
    let fp_matrix = matrix(&case, &fp_only, false);
    let witnesses: Vec<_> = witnesses
        .into_iter()
        .map(|(terms, witness)| json!({"terms":terms,"witness":witness}))
        .collect();
    std::fs::create_dir_all(out).unwrap();
    write(
        &out.join("candidate.json"),
        &json!({"schema":"twist-multiset-root-audit/v1","id":row["id"],"source_bindings":row,"status":"decoded-pending-independent-completeness","counts":totals,"old_physical_relations":old.len(),"expanded_physical_relations":all.len(),"new_physical_relations":fresh.len(),"finite_fp_physical_relations":fp_only.len(),"physical_set":all,"physical_set_sha256":digest(&serde_json::to_vec(&all).unwrap()),"new_physical_set":fresh,"finite_fp_physical_set":fp_only,"non_fp_physical_set":non_fp,"physical_lost_if_finite_fp_pruned":lost_if_pruned,"old_matrix":oldmatrix,"expanded_matrix":expanded,"filtered_matrix":filtered_matrix,"finite_fp_matrix":fp_matrix,"systems":systems,"representative_witnesses":witnesses,"admitted_timing":false,"complete_cost":null,"scope":"Saved-root decoding with repeated factors and signed cancellation; independent public-point enumeration still required. Finite-Fp-seen and non-Fp-seen sets can overlap; only the explicit difference is lost under counterfactual pruning. No F4 rerun, target solve or complete-cost gain."}),
    );
}

fn point_key(pt: Point, p: u64) -> u128 {
    if pt.inf {
        return 0;
    }
    let pack = |v: E| {
        v.iter().rev().fold(0u128, |a, &b| {
            assert!(b < p);
            a * p as u128 + b as u128
        })
    };
    1 + pack(pt.x) + (p as u128).pow(4) * pack(pt.y)
}
struct PairTable<T> {
    entries: Vec<(Terms, T)>,
    by_point: BTreeMap<u128, Vec<usize>>,
}
fn pair_table<T: Copy>(
    points: &[T],
    add: impl Fn(T, T) -> T,
    negate: impl Fn(T) -> T,
    key: impl Fn(T) -> u128,
) -> PairTable<T> {
    let signed: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(id, &pt)| [(id, 1, pt), (id, -1, negate(pt))])
        .collect();
    let mut unique = BTreeMap::<Terms, T>::new();
    for (i, &(a, sa, pa)) in signed.iter().enumerate() {
        for &(b, sb, pb) in &signed[i..] {
            let terms = normalize([(a, sa), (b, sb)]);
            let sum = add(pa, pb);
            if let Some(old) = unique.insert(terms, sum) {
                assert_eq!(key(old), key(sum));
            }
        }
    }
    assert_eq!(unique.len(), 2 * points.len() * points.len() + 1);
    let entries: Vec<_> = unique.into_iter().collect();
    let mut by_point = BTreeMap::<u128, Vec<usize>>::new();
    for (i, (_, pt)) in entries.iter().enumerate() {
        by_point.entry(key(*pt)).or_default().push(i);
    }
    PairTable { entries, by_point }
}
fn join_image<T: Copy>(
    table: &PairTable<T>,
    image: (usize, i8, T),
    add: impl Fn(T, T) -> T,
    negate: impl Fn(T) -> T,
    key: impl Fn(T) -> u128,
    start: Instant,
) -> BTreeSet<Terms> {
    let mut out = BTreeSet::new();
    for (i, (left, sum)) in table.entries.iter().enumerate() {
        if i % 1024 == 0 {
            budget(start);
        }
        let need = key(negate(add(image.2, *sum)));
        if let Some(rights) = table.by_point.get(&need) {
            for &j in rights {
                if i <= j {
                    out.insert(normalize(
                        left.iter()
                            .copied()
                            .chain(table.entries[j].0.iter().copied())
                            .chain([(image.0, image.1)]),
                    ));
                }
            }
        }
    }
    out
}
fn point_complete(case: &Case, double: bool, start: Instant) -> (BTreeSet<Terms>, Value) {
    let points: Vec<_> = case.pool[..case.l0].iter().map(|f| f.point).collect();
    let table = pair_table(
        &points,
        |a, b| case.c.add(a, b),
        |a| case.c.neg(a),
        |a| point_key(a, case.c.f.p),
    );
    let first = image_start(case, double);
    let mut seen = BTreeSet::new();
    let mut all = BTreeSet::new();
    let mut reps = 0u64;
    let mut seed_count = 0u64;
    for id in first..first + case.l0 {
        for sign in [-1, 1] {
            if seen.contains(&(id, sign)) {
                continue;
            }
            reps += 1;
            for map in &case.actions {
                let to = map[id];
                assert!(to.0 >= first && to.0 < first + case.l0);
                seen.insert((to.0, sign * to.1));
            }
            let seeds = join_image(
                &table,
                (id, sign, case.c.signed(case.pool[id].point, sign)),
                |a, b| case.c.add(a, b),
                |a| case.c.neg(a),
                |a| point_key(a, case.c.f.p),
                start,
            );
            seed_count += seeds.len() as u64;
            for terms in seeds {
                assert!(weighted_physical(case, &terms, double));
                for map in &case.actions {
                    let image = transport(case, &terms, map);
                    assert!(weighted_physical(case, &image, double));
                    all.insert(image);
                }
            }
        }
    }
    assert_eq!(seen.len(), 2 * case.l0);
    assert_eq!(reps as usize * 24, seen.len());
    (
        all,
        json!({"unique_signed_ordinary_pair_vectors":table.entries.len(),"signed_image_orbits":reps,"signed_image_points_covered":seen.len(),"orbit_seed_weighted_relations":seed_count,"pair_queries":table.entries.len() as u64*reps}),
    )
}
fn audit(row: &Value, candidate: &Path, out: &Path) {
    let started = Instant::now();
    assert!(!out.exists());
    let bytes = std::fs::read(candidate.join("candidate.json")).unwrap();
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["source_bindings"], *row);
    let raw = load(Path::new(s(&row["public_path"])), s(&row["public_sha256"]));
    let case = Case::new(&raw);
    let double = row["arm"] == "double-l0-active";
    let (actual, control) = point_complete(&case, double, started);
    let expected: BTreeSet<Terms> = serde_json::from_value(v["physical_set"].clone()).unwrap();
    assert_eq!(actual, expected, "complete weighted physical set mismatch");
    assert_eq!(actual.len(), v["physical_set"].as_array().unwrap().len());
    assert_eq!(
        digest(&serde_json::to_vec(&actual).unwrap()),
        s(&v["physical_set_sha256"])
    );
    let rebuilt = matrix(&case, &actual, true);
    assert_eq!(rebuilt, v["expanded_matrix"]);
    for (set_key, matrix_key) in [
        ("non_fp_physical_set", "filtered_matrix"),
        ("finite_fp_physical_set", "finite_fp_matrix"),
    ] {
        let set: BTreeSet<Terms> = serde_json::from_value(v[set_key].clone()).unwrap();
        assert!(set.is_subset(&actual));
        assert_eq!(matrix(&case, &set, true), v[matrix_key]);
    }
    let reference = load(
        Path::new(s(&row["reference_path"])),
        s(&row["reference_sha256"]),
    );
    let records = reference["systems"].as_array().unwrap();
    let decoded = v["systems"].as_array().unwrap();
    assert_eq!(records.len(), decoded.len());
    let mut q_nonzero = 0u64;
    for (record, reported) in records.iter().zip(decoded) {
        budget(started);
        let pair: (usize, i8, usize, i8) = serde_json::from_value(record["pair"].clone()).unwrap();
        let point = case.c.neg(case.c.add(
            case.c.signed(case.pool[pair.0].point, pair.1),
            case.c.signed(case.pool[pair.2].point, pair.3),
        ));
        if !reject(point) {
            assert!(reported["q_witness"].is_null());
            continue;
        }
        let root = [point.x[0], 0, 0];
        let values: Vec<_> = record["equations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|eq| horner_equation(eq, root, case.c.f.p))
            .collect();
        let p = case.c.f.p;
        let a = case.pool[pair.0].point.x[0];
        let q = mm(mm(a, a, p), (a + p - root[0]) % p, p);
        let found = values.iter().all(|&x| x == 0) && q != 0;
        assert_eq!(
            reported["q_witness"],
            json!({"root":root,"equation_values":values,"q":q,"common_zero_q_nonzero":found})
        );
        q_nonzero += u64::from(found);
    }
    assert_eq!(v["counts"]["q_nonmembership_witnesses"], q_nonzero);
    std::fs::create_dir_all(out).unwrap();
    write(
        &out.join("audit.json"),
        &json!({"id":row["id"],"status":"pass","candidate_sha256":digest(&bytes),"public_sha256":row["public_sha256"],"weighted_physical_set_sha256":v["physical_set_sha256"],"weighted_physical_relations":actual.len(),"matrix":rebuilt,"point_enumeration":control,"q_nonmembership_witnesses_horner_checked":q_nonzero,"scope":"Complete four-signed-ordinary-slot plus one-image public-point enumeration, multiplicities and cancellation included; no saved polynomial root enters enumeration. Independent fraction-free rank/solution and Horner q-witness checks. Field/group kernel, normalization and checked scalar actions are shared."}),
    );
}
fn summarize(workload: &Path, run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    let doc = catalog(workload);
    let mut cases = Vec::new();
    let mut totals = BTreeMap::<String, BTreeMap<String, u64>>::new();
    let mut complete = true;
    for row in doc["workloads"].as_array().unwrap() {
        let id = s(&row["id"]);
        let arm = s(&row["arm"]);
        let mut exits = BTreeMap::<String, String>::new();
        for phase in ["first", "audit", "aa", "parity"] {
            let path = run.join("metadata").join(format!("{id}-{phase}.exit"));
            exits.insert(
                phase.to_string(),
                std::fs::read_to_string(path)
                    .unwrap_or_else(|_| "missing".into())
                    .trim()
                    .to_string(),
            );
        }
        let t = totals.entry(arm.to_string()).or_default();
        *t.entry("registered_cases".into()).or_default() += 1;
        if exits.values().any(|v| v != "0") {
            complete = false;
            cases.push(json!({"id":id,"p":row["p"],"seed":row["seed"],"arm":arm,"status":"failed-or-unverified","phase_exits":exits,"numerical_result":null}));
            continue;
        }
        let root = run.join("cases").join(id);
        let bytes = std::fs::read(root.join("first/candidate.json")).unwrap();
        let aa = std::fs::read(root.join("aa/candidate.json")).unwrap();
        assert_eq!(bytes, aa);
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["source_bindings"], *row);
        let audit: Value =
            serde_json::from_slice(&std::fs::read(root.join("audit/audit.json")).unwrap()).unwrap();
        let parity: Value =
            serde_json::from_slice(&std::fs::read(root.join("parity.json")).unwrap()).unwrap();
        assert_eq!(audit["status"], "pass");
        assert_eq!(audit["candidate_sha256"], digest(&bytes));
        assert_eq!(parity["status"], "pass");
        assert_eq!(parity["first_sha256"], digest(&bytes));
        assert_eq!(parity["second_sha256"], digest(&bytes));
        let oldrank = u(&v["old_matrix"]["rank"]);
        let newrank = u(&v["expanded_matrix"]["rank"]);
        assert!(newrank >= oldrank);
        let oldproj: BTreeSet<Vec<u64>> =
            serde_json::from_value(v["old_matrix"]["projective_augmented_rows"].clone()).unwrap();
        let newproj: BTreeSet<Vec<u64>> =
            serde_json::from_value(v["expanded_matrix"]["projective_augmented_rows"].clone())
                .unwrap();
        assert!(oldproj.is_subset(&newproj));
        let lost = v["physical_lost_if_finite_fp_pruned"]
            .as_array()
            .unwrap()
            .len();
        let newzero = u(&v["expanded_matrix"]["zero_rows"]) - u(&v["old_matrix"]["zero_rows"]);
        let newprojective = newproj.difference(&oldproj).count();
        let mut brief = v.as_object().unwrap().clone();
        for key in [
            "physical_set",
            "new_physical_set",
            "finite_fp_physical_set",
            "non_fp_physical_set",
            "physical_lost_if_finite_fp_pruned",
            "systems",
            "representative_witnesses",
        ] {
            brief.remove(key);
        }
        brief.insert("status".into(), json!("verified-first-aa-point-audit"));
        brief.insert("p".into(), row["p"].clone());
        brief.insert("seed".into(), row["seed"].clone());
        brief.insert("arm".into(), json!(arm));
        brief.insert("phase_exits".into(), json!(exits));
        brief.insert("new_zero_rows".into(), json!(newzero));
        brief.insert("new_projective_rows".into(), json!(newprojective));
        brief.insert(
            "physical_lost_if_finite_fp_pruned_count".into(),
            json!(lost),
        );
        brief.insert("point_audit".into(), audit);
        *t.entry("verified_cases".into()).or_default() += 1;
        for (k, value) in [
            ("old_physical_relations", u(&v["old_physical_relations"])),
            (
                "expanded_physical_relations",
                u(&v["expanded_physical_relations"]),
            ),
            ("new_physical_relations", u(&v["new_physical_relations"])),
            ("new_zero_rows", newzero),
            ("new_projective_rows", newprojective as u64),
            ("physical_lost_if_finite_fp_pruned", lost as u64),
            ("rank_increase_cases", u64::from(newrank > oldrank)),
            (
                "full_rank_cases",
                u64::from(v["expanded_matrix"]["full_rank_point_verified"] == true),
            ),
        ] {
            *t.entry(k.to_string()).or_default() += value;
        }
        for (k, value) in v["counts"].as_object().unwrap() {
            *t.entry(k.clone()).or_default() += u(value);
        }
        cases.push(Value::Object(brief));
    }
    assert_eq!(cases.len(), 42);
    std::fs::create_dir_all(out).unwrap();
    let result = json!({"schema":"twist-multiset-panel/v1","status":if complete {"complete"} else {"complete-with-failures-or-missing"},"workload_sha256":WORKLOAD_SHA,"registered_case_arms":42,"cases":cases,"verified_case_local_arm_totals":totals,"totals_scope":"Sum of verified case-local values across public presentations; not global relation deduplication, global rank, independent secret samples or complete cost. Failed/missing positions remain listed.","admitted_timing":false,"complete_cost":null});
    write(&out.join("summary.json"), &result);
    complete
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert!(a.len() >= 2);
    match a[1].as_str() {
        "decode" => {
            assert_eq!(a.len(), 5);
            let doc = catalog(Path::new(&a[2]));
            decode(find_row(&doc, &a[3]), Path::new(&a[4]));
        }
        "audit" => {
            assert_eq!(a.len(), 6);
            let doc = catalog(Path::new(&a[2]));
            audit(find_row(&doc, &a[3]), Path::new(&a[4]), Path::new(&a[5]));
        }
        "parity" => {
            assert_eq!(a.len(), 5);
            let first = std::fs::read(Path::new(&a[2]).join("candidate.json")).unwrap();
            let second = std::fs::read(Path::new(&a[3]).join("candidate.json")).unwrap();
            assert_eq!(first, second);
            write(
                Path::new(&a[4]),
                &json!({"status":"pass","first_sha256":digest(&first),"second_sha256":digest(&second)}),
            );
        }
        "summarize" => {
            assert_eq!(a.len(), 5);
            if !summarize(Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4])) {
                std::process::exit(2);
            }
        }
        _ => panic!("unknown command"),
    }
}

#[cfg(test)]
mod multiset_tests {
    use super::*;
    #[test]
    fn signed_cancellation_and_repetition_are_canonical() {
        assert_eq!(normalize([(1, 1), (1, -1), (2, 1), (2, 1)]), vec![(2, 2)]);
        assert_eq!(normalize([(1, -1); 4]), vec![(1, -4)]);
    }
    #[test]
    fn repeated_cubic_factors_match_the_full_polynomial() {
        assert_eq!(cubic_key([1, 1, 2], 37), [4, 5, 2]);
        assert_eq!(cubic_key([1, 2, 5], 37), [8, 17, 10]);
        for xs in [[1, 1, 2], [1, 2, 5], [2, 2, 2]] {
            let key = cubic_key(xs, 37);
            for x in 0..37 {
                let product = xs.into_iter().fold(1, |v, a| mm(v, (x + 37 - a) % 37, 37));
                assert_eq!(q_value(key, x, 37), product);
            }
        }
    }
    #[test]
    fn weighted_column_fold_does_not_treat_two_as_minus_one() {
        let c = Case {
            c: Curve {
                f: Field { p: 37, d: 2 },
                b: [0, 0, 0, 2],
            },
            pool: vec![
                Factor {
                    point: O,
                    col: 0,
                    coefficient: 3,
                },
                Factor {
                    point: O,
                    col: 1,
                    coefficient: 5,
                },
            ],
            n: 37,
            original_len: 2,
            l0: 2,
            active: 2,
            anchor: 1,
            g: O,
            actions: vec![],
        };
        assert_eq!(weighted_row(&c, &vec![(1, 2)]), vec![10, 0]);
        assert_eq!(weighted_row(&c, &vec![(0, -2), (1, 3)]), vec![15, 6]);
    }
    #[test]
    fn fraction_free_matches_rref_for_rank_and_solution() {
        for rows in [
            vec![vec![1, 2, 3], vec![0, 1, 4]],
            vec![vec![1, 2, 3], vec![2, 4, 6]],
            vec![vec![0, 0, 0]],
        ] {
            assert_eq!(fraction_free(&rows, 2, 37), rref(rows, 2, 37));
        }
    }
    #[test]
    fn complete_pair_join_matches_four_slot_direct_enumeration() {
        let add = |a: u64, b: u64| (a + b) % 11;
        let negate = |a: u64| (11 - a) % 11;
        let key = |a: u64| a as u128;
        let points = [1, 2];
        let table = pair_table(&points, add, negate, key);
        assert_eq!(table.entries.len(), 9);
        let mut joined = BTreeSet::new();
        let mut direct = BTreeSet::new();
        for sign in [-1, 1] {
            let image = if sign == 1 { 3 } else { 8 };
            joined.extend(join_image(
                &table,
                (2, sign, image),
                add,
                negate,
                key,
                Instant::now(),
            ));
            for mask in 0..256usize {
                let mut value = mask;
                let mut terms = vec![(2, sign)];
                let mut sum = image;
                for _ in 0..4 {
                    let t = value % 4;
                    value /= 4;
                    let id = t / 2;
                    let s = if t % 2 == 0 { 1 } else { -1 };
                    terms.push((id, s));
                    sum = add(
                        sum,
                        if s == 1 {
                            points[id]
                        } else {
                            negate(points[id])
                        },
                    );
                }
                if sum == 0 {
                    direct.insert(normalize(terms));
                }
            }
        }
        assert_eq!(joined, direct);
        assert!(!joined.is_empty());
    }
    #[test]
    fn q_witness_requires_nonzero_evaluation() {
        assert_eq!(q_value([5, 0, 0], 3, 37), 19);
        assert_eq!(q_value([5, 0, 0], 5, 37), 0);
        let e = json!([[[1, 0, 0], 1], [[0, 0, 0], 32]]);
        assert_eq!(eval(&e, [5, 0, 0], 37), 0);
        assert_ne!(eval(&e, [6, 0, 0], 37), 0);
        assert_eq!(horner_equation(&e, [5, 0, 0], 37), 0);
        let mixed = json!([[[2, 1, 0], 3], [[0, 0, 2], 5], [[0, 0, 0], 8]]);
        assert_eq!(horner_equation(&mixed, [2, 3, 4], 37), 13);
        assert_eq!(
            horner_equation(&mixed, [2, 3, 4], 37),
            eval(&mixed, [2, 3, 4], 37)
        );
    }
}
