//! Oracle diagnostics only. Never called by a polynomial candidate.
use super::*;
use crate::fx_hash::FxMap;
const AUDIT_ARMS: [usize; 4] = [0, 7, 9, 8];

fn pairs(case: &Case, logs: &[u64]) -> FxMap<u64, Vec<[(usize, i8); 2]>> {
    let mut result = FxMap::<u64, Vec<[(usize, i8); 2]>>::default();
    for j in 1..case.l0 {
        for i in 0..j {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let a = if si == 1 {
                        logs[i]
                    } else {
                        negn(logs[i], case.n)
                    };
                    let b = if sj == 1 {
                        logs[j]
                    } else {
                        negn(logs[j], case.n)
                    };
                    result
                        .entry((a + b) % case.n)
                        .or_default()
                        .push([(i, si), (j, sj)]);
                }
            }
        }
    }
    result
}
fn zero_arm(
    case: &Case,
    logs: &[u64],
    arm: usize,
    pairs: &FxMap<u64, Vec<[(usize, i8); 2]>>,
) -> Value {
    let terms = reference_sets(case, logs, &case.peels[arm], 0, pairs);
    let query = json!([0, 0]);
    let cols = case.columns(arm);
    let mut rows = Vec::new();
    let mut unique = BTreeSet::new();
    let mut zero = 0;
    for relation in &terms {
        case.verify_terms(relation, INF);
        let r = case.row(relation, &query, arm);
        assert_eq!(
            r[cols - 1],
            0,
            "zero residual acquired a target coefficient"
        );
        match canonical_row(r.clone(), case.n) {
            Some(r) => {
                unique.insert(r);
            }
            None => zero += 1,
        }
        rows.push(r);
    }
    let (rank, solution) = independent_rref(&rows, cols, case.n);
    assert!(
        solution.is_none(),
        "zero-only rows cannot recover the target"
    );
    assert!(rank <= cols - 1);
    json!({"arm":ARMS[arm],"signed_physical_relations":terms.len(),
        "signed_physical_set":terms,"zero_augmented_rows":zero,
        "unique_projective_augmented_rows":unique.len(),"rank":rank,
        "factor_log_unknowns":cols-1,"target_column_zero":true,
        "target_recovery":false,"status":"pass"})
}

pub(super) fn zero(public: &Path, sha: &str, out: &Path) {
    let raw = checked_public(public, sha);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let logs = reference_logs(&case);
    let p = pairs(&case, &logs);
    let mut arms = Vec::new();
    for arm in AUDIT_ARMS {
        let result = zero_arm(&case, &logs, arm, &p);
        write(&out.join(format!("{}.json", ARMS[arm])), &result);
        arms.push(result);
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-zero-oracle/v1",
        "p":case.p,"seed":case.seed,"n":case.n,"l0":case.l0,
        "public_input_sha256":sha,"status":"pass","arms":arms,
        "scope":"Complete group/BSGS zero-fiber diagnostic and independent RREF. Not a polynomial collector, precomputation recovery or DLP cost."}),
    );
    println!(
        "{}",
        json!({"p":case.p,"seed":case.seed,"status":"pass","arms":arms.iter().map(|a|json!({"arm":a["arm"],"relations":a["signed_physical_relations"],"rank":a["rank"],"factor_unknowns":a["factor_log_unknowns"]})).collect::<Vec<_>>() })
    );
}

fn add_tuple(hist: &mut [u32], n: u64, logs: [u64; 4]) {
    for mask in 0..16 {
        let mut sum = 0;
        for (i, log) in logs.into_iter().enumerate() {
            sum =
                (sum + if mask & (1 << i) == 0 {
                    log
                } else {
                    negn(log, n)
                }) % n;
        }
        hist[sum as usize] = hist[sum as usize].checked_add(1).unwrap();
    }
}
fn mixed_hist(n: u64, ordinary: &[u64], image: &[u64]) -> Vec<u32> {
    assert!(
        n <= 10_000_000,
        "full histogram deliberately restricted to the p37 control"
    );
    let mut h = vec![0; n as usize];
    for k in 2..ordinary.len() {
        for j in 1..k {
            for i in 0..j {
                for &fourth in image {
                    add_tuple(&mut h, n, [ordinary[i], ordinary[j], ordinary[k], fourth]);
                }
            }
        }
    }
    h
}
fn pure_hist(n: u64, ordinary: &[u64]) -> Vec<u32> {
    assert!(n <= 10_000_000);
    let mut h = vec![0; n as usize];
    for l in 3..ordinary.len() {
        for k in 2..l {
            for j in 1..k {
                for i in 0..j {
                    add_tuple(
                        &mut h,
                        n,
                        [ordinary[i], ordinary[j], ordinary[k], ordinary[l]],
                    );
                }
            }
        }
    }
    h
}
fn choose3(l: u64) -> u64 {
    if l < 3 {
        0
    } else {
        l * (l - 1) * (l - 2) / 6
    }
}
fn choose4(l: u64) -> u64 {
    if l < 4 {
        0
    } else {
        l * (l - 1) * (l - 2) * (l - 3) / 24
    }
}

pub(super) fn population(public: &Path, sha: &str, out: &Path) {
    use std::io::Write;
    let raw = checked_public(public, sha);
    assert_eq!(
        raw["p"], 37,
        "full-residual histogram is preregistered only for p37"
    );
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let logs = reference_logs(&case);
    let pairs = pairs(&case, &logs);
    let l = case.l0 as u64;
    let mut results = Vec::new();
    let mut mixed_masses = Vec::new();
    for arm in AUDIT_ARMS {
        let (hist, expected) = if arm == 8 {
            (pure_hist(case.n, &logs[..case.l0]), 16 * choose4(l))
        } else {
            let image: Vec<_> = case.peels[arm]
                .iter()
                .map(|&id| {
                    assert!(
                        id >= case.l0,
                        "mixed image overlaps the ordinary physical base"
                    );
                    logs[id]
                })
                .collect();
            if arm == 7 || arm == 9 {
                assert_eq!(image.len(), case.l0);
            }
            (
                mixed_hist(case.n, &logs[..case.l0], &image),
                16 * choose3(l) * image.len() as u64,
            )
        };
        let mut frequencies = BTreeMap::<u32, u64>::new();
        let mut mass = 0u64;
        let mut square_mass = 0u128;
        for (r, &c) in hist.iter().enumerate() {
            assert_eq!(
                c,
                hist[(case.n as usize - r) % case.n as usize],
                "global sign symmetry failed"
            );
            *frequencies.entry(c).or_default() += 1;
            mass += c as u64;
            square_mass += (c as u128) * (c as u128);
        }
        assert_eq!(
            mass, expected,
            "candidate domain and full residual mass disagree"
        );
        if arm == 7 || arm == 9 {
            mixed_masses.push(mass);
        }
        let zero = zero_arm(&case, &logs, arm, &pairs);
        assert_eq!(
            hist[0] as u64,
            zero["signed_physical_relations"].as_u64().unwrap(),
            "independent pair oracle disagrees with exhaustive zero fiber"
        );
        let file = out.join(format!("{}.u32le", ARMS[arm]));
        let mut buffer = std::io::BufWriter::new(std::fs::File::create(&file).unwrap());
        for &count in &hist {
            buffer.write_all(&count.to_le_bytes()).unwrap();
        }
        buffer.flush().unwrap();
        drop(buffer);
        let bytes = std::fs::read(&file).unwrap();
        let variance_numerator = case.n as u128 * square_mass - (mass as u128) * (mass as u128);
        results.push(json!({"arm":ARMS[arm],"residual_population":case.n,
            "signed_tuple_mass":mass,"combinatorial_mass":expected,
            "mean":{"numerator":mass,"denominator":case.n},
            "support":case.n-frequencies.get(&0).copied().unwrap_or(0),
            "second_moment_sum":square_mass,
            "variance":{"numerator":variance_numerator,"denominator":(case.n as u128)*(case.n as u128)},
            "frequency_of_yield":frequencies,"zero":zero,
            "histogram":{"path":file.file_name().unwrap().to_str().unwrap(),"encoding":"u32 little endian; scalar residual labels 0..n-1","sha256":hex::encode(hash::sha256::sha256(&bytes)),"bytes":bytes.len()},"status":"pass"}));
    }
    assert_eq!(mixed_masses.len(), 2);
    assert_eq!(mixed_masses[0], mixed_masses[1]);
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-uniform-population/v1",
        "p":case.p,"seed":case.seed,"n":case.n,"l0":case.l0,
        "public_input_sha256":sha,"status":"pass","arms":results,
        "scope":"Entire finite residual population under a separate BSGS scalar oracle, including zero; not polynomial yield, blind recovery or a solver price."}),
    );
    println!(
        "{}",
        json!({"p":case.p,"seed":case.seed,"status":"pass","equal_mixed_mass":mixed_masses[0],"residuals":case.n})
    );
}

pub(super) fn summarize(run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    let mut zero = Vec::new();
    let mut population = Vec::new();
    let mut missing = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let file = run
                .join("zero")
                .join(format!("p{p}-seed{seed}/summary.json"));
            if !file.is_file() {
                missing.push(json!({"kind":"zero","p":p,"seed":seed}));
                continue;
            }
            let bytes = std::fs::read(&file).unwrap();
            let v: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(v["status"], "pass");
            assert_eq!(v["p"], p);
            assert_eq!(v["seed"], seed);
            for a in v["arms"].as_array().unwrap() {
                zero.push(json!({"p":p,"seed":seed,"arm":a["arm"],
                "signed_physical_relations":a["signed_physical_relations"],
                "zero_augmented_rows":a["zero_augmented_rows"],
                "unique_projective_augmented_rows":a["unique_projective_augmented_rows"],
                "rank":a["rank"],"factor_log_unknowns":a["factor_log_unknowns"],
                "source":file,"source_sha256":hex::encode(hash::sha256::sha256(&bytes))}));
            }
        }
    }
    for seed in 0..3 {
        let file = run
            .join("population")
            .join(format!("p37-seed{seed}/summary.json"));
        if !file.is_file() {
            missing.push(json!({"kind":"population","p":37,"seed":seed}));
            continue;
        }
        let bytes = std::fs::read(&file).unwrap();
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["status"], "pass");
        population.push(json!({"p":37,"seed":seed,"n":v["n"],"l0":v["l0"],"arms":v["arms"],"source":file,"source_sha256":hex::encode(hash::sha256::sha256(&bytes))}));
    }
    let pass = missing.is_empty();
    write(
        out,
        &json!({"schema":"joint-image-population-zero-summary/v1",
        "status":if pass {"pass"} else {"partial"},"missing":missing,
        "zero_cases_requested":21,"population_cases_requested":3,
        "zero_arm_rows":zero,"population_cases":population,
        "scope":"Oracle population and zero-rank diagnostics. No polynomial recovery or complete-cost claim."}),
    );
    pass
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_zero_fiber_uses_public_query_encoding_and_has_no_target_column() {
        let tape =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../quartic_twist_mixed_v1.ndjson");
        let scratch =
            std::env::temp_dir().join(format!("joint-zero-row-check-{}", std::process::id()));
        assert!(!scratch.exists());
        std::fs::create_dir(&scratch).unwrap();
        let case = Case::new(public_record(&load(&tape, INPUT_SHA, 37, 0)), &scratch);
        let logs = reference_logs(&case);
        let table = pairs(&case, &logs);
        for arm in AUDIT_ARMS {
            let result = zero_arm(&case, &logs, arm, &table);
            assert_eq!(result["status"], "pass");
            assert_eq!(result["target_recovery"], false);
            assert_eq!(result["target_column_zero"], true);
        }
        std::fs::remove_dir_all(scratch).unwrap();
    }
    #[test]
    fn equal_disjoint_image_domains_have_equal_mass_but_different_histograms() {
        let ordinary = [1, 3, 7, 11];
        let first = [17, 51, 18, 86];
        let second = [2, 6, 14, 22];
        for image in [&first, &second] {
            assert!(image
                .iter()
                .all(|x| ordinary.iter().all(|y| x != y && *x != 101 - *y)));
        }
        let a = mixed_hist(101, &ordinary, &first);
        let b = mixed_hist(101, &ordinary, &second);
        assert_eq!(a.iter().map(|&c| c as u64).sum::<u64>(), 256);
        assert_eq!(b.iter().map(|&c| c as u64).sum::<u64>(), 256);
        assert_ne!(a, b);
        for r in 0..101 {
            assert_eq!(a[r], a[(101 - r) % 101]);
            assert_eq!(b[r], b[(101 - r) % 101]);
        }
        assert_eq!(
            pure_hist(101, &ordinary)
                .iter()
                .map(|&c| c as u64)
                .sum::<u64>(),
            16
        );
    }
}
