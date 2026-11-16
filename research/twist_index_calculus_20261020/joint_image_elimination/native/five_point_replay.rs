//! Independent point-keyed enumeration; no factor logs or BSGS are read.
use super::*;

#[derive(Clone, Copy)]
struct Triple {
    ids: [u16; 3],
    mask: u8,
}
#[derive(Clone, Copy)]
struct Entry {
    key: u128,
    triple: Triple,
}

fn triple_table<T: Copy>(
    points: &[T],
    add: impl Fn(T, T) -> T,
    neg: impl Fn(T) -> T,
    key: impl Fn(T) -> u128,
) -> Vec<Entry> {
    assert!(points.len() < u16::MAX as usize);
    let l = points.len();
    let mut entries = Vec::with_capacity(l * (l - 1) * (l - 2) / 6 * 4);
    for j in 1..l {
        for i in 0..j {
            for sj in 0..2u8 {
                let pair = add(points[i], if sj == 0 { points[j] } else { neg(points[j]) });
                for k in j + 1..l {
                    for sk in 0..2u8 {
                        entries.push(Entry {
                            key: key(add(pair, if sk == 0 { points[k] } else { neg(points[k]) })),
                            triple: Triple {
                                ids: [i as u16, j as u16, k as u16],
                                mask: sj | (sk << 1),
                            },
                        });
                    }
                }
            }
        }
    }
    entries.sort_unstable_by_key(|e| e.key);
    entries
}
fn collect<T: Copy>(
    points: &[T],
    l0: usize,
    images: Option<&[usize]>,
    table: &[Entry],
    add: impl Fn(T, T) -> T,
    neg: impl Fn(T) -> T,
    key: impl Fn(T) -> u128,
) -> (BTreeSet<Terms>, u64) {
    let mut found = BTreeSet::new();
    let mut queries = 0u64;
    let mut visit = |i: usize, j: usize, si: i8, sj: i8| {
        let a = if si == 1 { points[i] } else { neg(points[i]) };
        let b = if sj == 1 { points[j] } else { neg(points[j]) };
        let wanted = neg(add(a, b));
        for (point, global_sign) in [(wanted, 1i8), (neg(wanted), -1i8)] {
            queries += 1;
            let k = key(point);
            let lo = table.partition_point(|e| e.key < k);
            let hi = table.partition_point(|e| e.key <= k);
            for entry in &table[lo..hi] {
                let t = entry.triple;
                // Pure query pair contains the two least IDs; mixed query
                // contains the least ordinary ID and the unique image.
                let floor = if images.is_some() { i } else { j };
                if floor >= t.ids[0] as usize {
                    continue;
                }
                let mut terms = vec![
                    (i, si),
                    (j, sj),
                    (t.ids[0] as usize, global_sign),
                    (
                        t.ids[1] as usize,
                        if t.mask & 1 == 0 {
                            global_sign
                        } else {
                            -global_sign
                        },
                    ),
                    (
                        t.ids[2] as usize,
                        if t.mask & 2 == 0 {
                            global_sign
                        } else {
                            -global_sign
                        },
                    ),
                ];
                terms.sort();
                assert!(
                    found.insert(terms),
                    "canonical point partition duplicated a physical tuple"
                );
            }
        }
    };
    if let Some(images) = images {
        for &j in images {
            assert!(j >= l0);
            for i in 0..l0 {
                for si in [-1, 1] {
                    for sj in [-1, 1] {
                        visit(i, j, si, sj);
                    }
                }
            }
        }
    } else {
        for j in 1..l0 {
            for i in 0..j {
                for si in [-1, 1] {
                    for sj in [-1, 1] {
                        visit(i, j, si, sj);
                    }
                }
            }
        }
    }
    (found, queries)
}
fn inverse(a: u64, n: u64) -> u64 {
    let (mut r, mut next) = (n as i128, a as i128);
    let (mut t, mut nt) = (0i128, 1i128);
    while next != 0 {
        let q = r / next;
        (r, next) = (next, r - q * next);
        (t, nt) = (nt, t - q * nt);
    }
    assert_eq!(r, 1);
    t.rem_euclid(n as i128) as u64
}
fn mul(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn sub(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 + n as u128 - b as u128) % n as u128) as u64
}
fn eliminate(rows: &[Vec<u64>], cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
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
                    row[j] = sub(mul(a, row[j], n), mul(b, pivot[j], n), n);
                }
            } else {
                pivots.insert(col, row.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            assert!(row[..cols].iter().all(|&v| v == 0));
            assert_eq!(row[cols], 0, "inconsistent point-replay row");
        }
    }
    let rank = pivots.len();
    let solution = if rank == cols {
        let mut x = vec![0; cols];
        for col in (0..cols).rev() {
            let row = &pivots[&col];
            let mut rhs = row[cols];
            for j in col + 1..cols {
                rhs = sub(rhs, mul(row[j], x[j], n), n);
            }
            x[col] = mul(rhs, inverse(row[col], n), n);
        }
        Some(x)
    } else {
        None
    };
    (rank, solution)
}
fn point_key(point: Point, p: u64) -> u128 {
    if point.inf {
        return 0;
    }
    let pack = |a: [u64; 4]| {
        a.iter().rev().fold(0u128, |x, &v| {
            assert!(v < p);
            x * p as u128 + v as u128
        })
    };
    1 + pack(point.x) + (p as u128).pow(4) * pack(point.y)
}
pub(super) fn audit(public: &Path, sha: &str, prior: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(checked_public(public, sha), out);
    let points: Vec<Point> = case.pool.iter().map(|f| f.p.into()).collect();
    let table = triple_table(
        &points[..case.l0],
        |a, b| case.c.add(a, b),
        |p| case.c.neg(p),
        |p| point_key(p, case.p),
    );
    let cols = case.columns(8) - 1;
    let mut arms = Vec::new();
    let mut pass = true;
    for arm in [8usize, 7, 9] {
        let path = prior.join(format!("{}.json", ARMS[arm]));
        let bytes = std::fs::read(&path).unwrap();
        let old: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(old["arm"], ARMS[arm]);
        assert_eq!(old["status"], "complete-oracle-census");
        let expected: BTreeSet<Terms> =
            serde_json::from_value(old["signed_physical_set"].clone()).unwrap();
        assert_eq!(old["signed_physical_relations"], expected.len());
        assert_eq!(
            old["signed_set_sha256"],
            hex::encode(hash::sha256::sha256(
                &serde_json::to_vec(&expected).unwrap()
            ))
        );
        let (actual, queries) = collect(
            &points,
            case.l0,
            if arm == 8 {
                None
            } else {
                Some(&case.peels[arm])
            },
            &table,
            |a, b| case.c.add(a, b),
            |p| case.c.neg(p),
            |p| point_key(p, case.p),
        );
        let mut rows = Vec::new();
        let mut projective = BTreeSet::new();
        let mut zero = 0usize;
        for terms in &actual {
            assert_eq!(terms.len(), 5);
            assert_eq!(terms.iter().map(|t| t.0).collect::<BTreeSet<_>>().len(), 5);
            assert_eq!(
                terms
                    .iter()
                    .map(|t| points[t.0].x)
                    .collect::<BTreeSet<_>>()
                    .len(),
                5
            );
            let sum = terms.iter().fold(INF, |s, &(id, sign)| {
                case.c.add(
                    s,
                    if sign == 1 {
                        points[id]
                    } else {
                        case.c.neg(points[id])
                    },
                )
            });
            assert_eq!(sum, INF);
            let mut row = vec![0; cols + 1];
            for &(id, sign) in terms {
                let f = &case.pool[id];
                assert!(f.col <= cols);
                let coef = if sign == 1 {
                    f.coef
                } else {
                    if f.coef == 0 {
                        0
                    } else {
                        case.n - f.coef
                    }
                };
                if f.col == 0 {
                    row[cols] = sub(row[cols], mul(case.anchor, coef, case.n), case.n);
                } else {
                    row[f.col - 1] =
                        ((row[f.col - 1] as u128 + coef as u128) % case.n as u128) as u64;
                }
            }
            let encoded = case.row(terms, &json!([0, 0]), arm);
            assert_eq!(encoded[cols], 0);
            assert_eq!(row[..cols], encoded[..cols]);
            assert_eq!(row[cols], encoded[cols + 1]);
            if let Some(&first) = row.iter().find(|&&v| v != 0) {
                let inv = inverse(first, case.n);
                projective.insert(row.iter().map(|&v| mul(v, inv, case.n)).collect::<Vec<_>>());
            } else {
                zero += 1;
            }
            rows.push(row);
        }
        let (rank, solution) = eliminate(&rows, cols, case.n);
        let verified = solution
            .as_ref()
            .map(|solution| {
                let mut reps = vec![case.anchor];
                reps.extend(solution);
                case.usable(arm).iter().all(|&id| {
                    let f = &case.pool[id];
                    case.c.mul(case.g, mul(reps[f.col], f.coef, case.n)) == points[id]
                })
            })
            .unwrap_or(false);
        let missing: Vec<_> = expected.difference(&actual).cloned().collect();
        let extra: Vec<_> = actual.difference(&expected).cloned().collect();
        let ok = missing.is_empty()
            && extra.is_empty()
            && old["factor_log_rank"] == rank
            && old["zero_augmented_rows"] == zero
            && old["unique_projective_augmented_rows"] == projective.len()
            && old["factor_log_solution_point_verified"] == verified
            && verified == (rank == cols);
        pass &= ok;
        write(
            &out.join(format!("{}.json", ARMS[arm])),
            &json!({"status":if ok {"pass"}else{"mismatch"},
            "arm":ARMS[arm],"prior_file_sha256":hex::encode(hash::sha256::sha256(&bytes)),
            "signed_physical_set":actual,"physical_relations":actual.len(),"missing":missing,"extra":extra,
            "zero_rows":zero,"projective_rows":projective.len(),"factor_log_rank":rank,"factor_columns":cols,
            "all_factor_points_verified":verified,"point_table_queries":queries,"target_coefficient_zero":true}),
        );
        arms.push(json!({"arm":ARMS[arm],"status":if ok {"pass"}else{"mismatch"},"physical_relations":actual.len(),
            "factor_log_rank":rank,"factor_columns":cols,"zero_rows":zero,"projective_rows":projective.len(),"all_factor_points_verified":verified}));
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"five-zero-point-replay/v1","status":if pass {"pass"}else{"mismatch"},
        "p":case.p,"seed":case.seed,"public_sha256":sha,"half_signed_triple_entries":table.len(),
        "entry_bytes":std::mem::size_of::<Entry>(),"arms":arms,
        "scope":"Public-point triple table with a unique partition, direct row reconstruction and separate cross-multiplication rank/back-substitution. Shared public field/curve arithmetic and orbit construction. No scalar logs, BSGS, polynomial collector, target recovery or timing claim."}),
    );
    pass
}
pub(super) fn summarize(run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    let mut records = Vec::new();
    let mut missing = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let id = format!("p{p}-seed{seed}");
            let path = run.join("cases").join(&id).join("summary.json");
            if !path.is_file() {
                missing.push(id);
                continue;
            }
            let bytes = std::fs::read(&path).unwrap();
            let v: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(v["schema"], "five-zero-point-replay/v1");
            assert_eq!(v["p"], p);
            assert_eq!(v["seed"], seed);
            records.push(
                json!({"id":id,"sha256":hex::encode(hash::sha256::sha256(&bytes)),"record":v}),
            );
        }
    }
    let pass = missing.is_empty()
        && records.len() == 21
        && records.iter().all(|r| r["record"]["status"] == "pass");
    write(
        out,
        &json!({"schema":"five-zero-point-replay-summary/v1","status":if pass {"pass"}else{"partial"},
        "completed_cases":records.len(),"missing":missing,"records":records,"scope":"Independent enumeration and matrix replay of the 21 saved oracle censuses; no polynomial or complete-cost claim."}),
    );
    pass
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_point_partition_matches_direct_five_term_enumeration() {
        let points = [1u64, 3, 7, 11, 22, 29, 44, 48];
        let table = triple_table(
            &points[..6],
            |a, b| (a + b) % 101,
            |a| (101 - a) % 101,
            |a| a as u128,
        );
        for images in [None, Some([6usize, 7].as_slice())] {
            let actual = collect(
                &points,
                6,
                images,
                &table,
                |a, b| (a + b) % 101,
                |a| (101 - a) % 101,
                |a| a as u128,
            )
            .0;
            let mut expected = BTreeSet::new();
            for e in 4..points.len() {
                for d in 3..e {
                    for c in 2..d {
                        for b in 1..c {
                            for a in 0..b {
                                let ids = [a, b, c, d, e];
                                let count = ids.iter().filter(|&&id| id >= 6).count();
                                if count != if images.is_some() { 1 } else { 0 } {
                                    continue;
                                }
                                for mask in 0..32 {
                                    let terms: Terms = ids
                                        .iter()
                                        .enumerate()
                                        .map(|(j, &id)| {
                                            (id, if mask & (1 << j) == 0 { 1 } else { -1 })
                                        })
                                        .collect();
                                    let sum: i128 = terms
                                        .iter()
                                        .map(|&(id, s)| points[id] as i128 * s as i128)
                                        .sum();
                                    if sum.rem_euclid(101) == 0 {
                                        expected.insert(terms);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            assert!(!expected.is_empty());
            assert_eq!(actual, expected);
        }
    }
    #[test]
    fn cross_elimination_distinguishes_rank_and_rejects_inconsistency() {
        let rows = vec![vec![1, 2, 63], vec![3, 4, 143], vec![4, 6, 206]];
        assert_eq!(eliminate(&rows, 2, 1_873_633), (2, Some(vec![17, 23])));
        assert_eq!(
            eliminate(&[vec![1, 2, 63], vec![2, 4, 126]], 2, 1_873_633),
            (1, None)
        );
        let mut bad = rows;
        bad[2][2] += 1;
        assert!(std::panic::catch_unwind(|| eliminate(&bad, 2, 1_873_633)).is_err());
    }
}
