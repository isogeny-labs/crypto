//! Recount every ordered relation using independent polynomial-basis curve arithmetic.
use super::*;
use crypto_lib::binary_ecc::curve::{point_add, point_neg};
use crypto_lib::cryptanalysis::koblitz_index_calculus::points_with_x;
use std::path::Path;

fn lower(p: FastPoint, n: u32) -> BinaryPoint {
    if p.infinity {
        BinaryPoint::Infinity
    } else {
        BinaryPoint::Affine {
            x: element(p.x, n),
            y: element(p.y, n),
        }
    }
}
fn lift(p: BinaryPoint) -> FastPoint {
    match p {
        BinaryPoint::Infinity => FastPoint::INFINITY,
        BinaryPoint::Affine { x, y } => FastPoint::affine(math::word(&x), math::word(&y)),
    }
}
struct Reference<'a>(&'a BinaryCurve);
impl CountedGroup for Reference<'_> {
    type Elt = FastPoint;
    fn identity(&self) -> FastPoint {
        FastPoint::INFINITY
    }
    fn is_identity(&self, p: &FastPoint) -> bool {
        p.infinity
    }
    fn add(&self, ops: &mut GroupOps, p: FastPoint, q: FastPoint) -> FastPoint {
        ops.adds += 1;
        lift(point_add(self.0, &lower(p, self.0.m), &lower(q, self.0.m)))
    }
    fn double(&self, ops: &mut GroupOps, p: FastPoint) -> FastPoint {
        ops.doubles += 1;
        lift(point_add(self.0, &lower(p, self.0.m), &lower(p, self.0.m)))
    }
    fn neg(&self, p: FastPoint) -> FastPoint {
        lift(point_neg(&lower(p, self.0.m)))
    }
    fn key(&self, p: &FastPoint) -> u64 {
        p.pack()
    }
}
pub(super) fn run(model: &str, input: &str) {
    let (inst, record) = read_model(model);
    let curve = BinaryCurve {
        m: inst.n,
        irreducible: inst.irreducible.clone(),
        a: element(inst.a, inst.n),
        b: element(inst.b, inst.n),
        generator: lower(inst.generator, inst.n),
        order: BigUint::from(inst.r),
        cofactor: BigUint::from(inst.cofactor),
    };
    let group = Reference(&curve);
    assert!(curve.is_on_curve(&curve.generator));
    assert!(
        group
            .mul(&mut GroupOps::default(), inst.generator, inst.r)
            .infinity
    );
    let raw = std::fs::read(input).unwrap();
    let input_sha = hex::encode(Sha256::digest(&raw));
    let mut rows = 0;
    let mut seen = BTreeMap::<(Vec<u64>, u64), u64>::new();
    let mut neg = 0;
    let mut sat = 0;
    for line in std::str::from_utf8(&raw).unwrap().lines() {
        let row: Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["schema"], "ic-qualified-query/v1");
        assert_eq!(row["draw"], rows);
        assert_eq!(row["n"], inst.n);
        assert_eq!(row["curve_b"], inst.b);
        assert_eq!(row["field_modulus"], inst.gf.irr);
        assert_eq!(row["curve_record_sha256"], record["record_sha256"]);
        let m = row["m"].as_u64().unwrap() as usize;
        let ell = row["ell"].as_u64().unwrap() as usize;
        let draw = row["draw"].as_u64().unwrap();
        let family = row["family"].as_str().unwrap();
        let h = hash(
            b"ic-qualified-basis/v1\0",
            &[
                202_610_250_003,
                m as u64,
                u64::from(inst.n),
                ell as u64,
                draw,
            ],
            family,
        );
        let seed = u64::from_le_bytes(h[..8].try_into().unwrap());
        assert_eq!(row["seed"], seed);
        let bs = math::make_basis(&inst.gf, ell, family, seed);
        assert_eq!(row["v_basis"], json!(bs));
        let (r, attempt) = target(
            &inst,
            record["record_sha256"].as_str().unwrap(),
            m,
            ell,
            draw,
        );
        assert_eq!(row["target"], point(r));
        assert_eq!(row["x_r"], r.x);
        assert_eq!(row["target_hash_attempt"], attempt);
        assert!(
            curve.is_on_curve(&lower(r, inst.n))
                && group.mul(&mut GroupOps::default(), r, inst.r).infinity
        );
        let mut base = BTreeMap::new();
        for x in span(&bs) {
            for bp in points_with_x(&curve, &element(x, inst.n)) {
                assert!(curve.is_on_curve(&bp));
                let p = lift(bp);
                if group.mul(&mut GroupOps::default(), p, inst.r).infinity {
                    base.insert(p.pack(), p);
                }
            }
        }
        let base: Vec<_> = base.into_values().collect();
        assert_eq!(row["base_points"], base.len());
        let base_json: Vec<_> = base.iter().copied().map(point).collect();
        assert_eq!(
            row["base_points_sha256"],
            id_sha256(&json!(base_json)).unwrap()
        );
        let (count, witness, _ops) = point_count(&group, &base, r, m);
        assert_eq!(row["point_decompositions"], count);
        let witness = witness.map(|w| w.into_iter().map(point).collect::<Vec<_>>());
        assert_eq!(row["point_witness"], json!(witness));
        assert_eq!(row["point_witness_verified"], count > 0);
        let direct = base.contains(&r);
        assert_eq!(row["direct_base_hit"], direct);
        let key = (bs, r.x);
        let duplicate = seen.get(&key).copied();
        seen.entry(key).or_insert(draw);
        assert_eq!(row["duplicate_of_draw"], json!(duplicate));
        let roots = row["solutions"].as_u64().unwrap();
        let selection = if duplicate.is_none()
            && base.len() >= 2
            && !direct
            && roots == 0
            && count == 0
            && neg < 4
        {
            neg += 1;
            Some("negative")
        } else if duplicate.is_none() && roots > 0 && count > 0 && sat < 2 {
            sat += 1;
            Some("positive")
        } else {
            None
        };
        assert_eq!(row["followup_selection"], json!(selection));
        emit(
            &json!({"schema":"ic-qualified-point-replay/v1","draw":draw,"input_sha256":input_sha,
            "point_count":count,"base_points":base.len(),"status":"independent_binary_curve_arithmetic_agrees"}),
        );
        rows += 1;
    }
    assert_eq!(rows, 128);
    emit(
        &json!({"schema":"ic-qualified-point-replay-summary/v1","rows":rows,"input_sha256":input_sha,
        "selected_negative":neg,"selected_positive":sat,"status":"all_128_counts_bases_targets_witnesses_and_selection_agree",
        "scope":"independent field/lift/group arithmetic; same separately unit-controlled group MITM counting logic; finite-chain counts replayed by separate instrument"}),
    );
}
fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
pub(super) fn summary(path: &str) {
    let root = Path::new(path);
    let mut units = vec![];
    let mut targets = BTreeMap::<(u64, u64, u64, u64), Value>::new();
    let mut total = 0;
    let mut boolean_sat = 0;
    let mut point_sat = 0;
    let mut both_sat = 0;
    let mut independent = 0;
    let mut models = vec![];
    for (m, n, ell) in [(3, 22, 8), (3, 25, 9), (4, 17, 6), (4, 18, 6)] {
        let model_path = root.join(format!("model-n{n}/output.jsonl"));
        let (inst, r) = read_model(model_path.to_str().unwrap());
        models.push(r);
        for family in ["prefix", "geometric", "geomtraceu", "random"] {
            let dir = root.join(format!("m{m}-n{n}-ell{ell}-{family}"));
            assert_eq!(read_json(&dir.join("status.json"))["exit_code"], 0);
            assert_eq!(read_json(&dir.join("replay/status.json"))["exit_code"], 0);
            let raw = std::fs::read(dir.join("output.jsonl")).unwrap();
            let sha = hex::encode(Sha256::digest(&raw));
            let rows: Vec<Value> = std::str::from_utf8(&raw)
                .unwrap()
                .lines()
                .map(|l| serde_json::from_str(l).unwrap())
                .collect();
            assert_eq!(rows.len(), 128);
            let replay = std::fs::read_to_string(dir.join("replay/output.jsonl")).unwrap();
            let replay: Vec<Value> = replay
                .lines()
                .map(|l| serde_json::from_str(l).unwrap())
                .collect();
            assert_eq!(replay.len(), 128);
            for (i, row) in rows.iter().enumerate() {
                assert_eq!(replay[i]["input_sha256"], sha);
                assert_eq!(replay[i]["status"], "verified");
                assert_eq!(replay[i]["solutions"], row["solutions"]);
                let key = (m, n, ell, row["draw"].as_u64().unwrap());
                let target = row["target"].clone();
                if let Some(old) = targets.insert(key, target.clone()) {
                    assert_eq!(old, target);
                }
            }
            let b = rows
                .iter()
                .filter(|r| r["solutions"].as_u64().unwrap() > 0)
                .count();
            let p = rows
                .iter()
                .filter(|r| r["point_decompositions"].as_u64().unwrap() > 0)
                .count();
            let both = rows
                .iter()
                .filter(|r| {
                    r["solutions"].as_u64().unwrap() > 0
                        && r["point_decompositions"].as_u64().unwrap() > 0
                })
                .count();
            let selected = |s: &str| {
                rows.iter()
                    .filter(|r| r["followup_selection"] == s)
                    .map(|r| r["draw"].clone())
                    .collect::<Vec<_>>()
            };
            let base_counts: Vec<_> = rows
                .iter()
                .map(|r| r["base_points"].as_u64().unwrap())
                .collect();
            let reference = dir.join("reference");
            let ref_verified = if reference.join("status.json").exists() {
                let status = read_json(&reference.join("status.json"));
                if status["exit_code"] == 0 {
                    let text = std::fs::read_to_string(reference.join("output.jsonl")).unwrap();
                    let last: Value = serde_json::from_str(text.lines().last().unwrap()).unwrap();
                    assert_eq!(last["input_sha256"], sha);
                    assert_eq!(last["rows"], 128);
                    assert_eq!(
                        last["status"],
                        "all_128_counts_bases_targets_witnesses_and_selection_agree"
                    );
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if ref_verified {
                independent += 128;
            }
            units.push(json!({"m":m,"n":n,"ell":ell,"family":family,"rows":128,"input_sha256":sha,
                "boolean_sat":b,"point_sat":p,"both_sat":both,"boolean_only_sat":b-both,"point_only_sat":p-both,
                "empty_bases":base_counts.iter().filter(|&&v|v==0).count(),"base_points_min":base_counts.iter().min(),"base_points_max":base_counts.iter().max(),
                "direct_hits":rows.iter().filter(|r|r["direct_base_hit"]==true).count(),
                "duplicate_queries":rows.iter().filter(|r|!r["duplicate_of_draw"].is_null()).count(),
                "selected_negative_draws":selected("negative"),"selected_positive_draws":selected("positive"),
                "point_reference_rows_verified":if ref_verified {128} else {0},"input":dir.join("output.jsonl")}));
            total += 128;
            boolean_sat += b;
            point_sat += p;
            both_sat += both;
        }
        assert!(inst.r > 2);
    }
    emit(
        &json!({"schema":"ic-qualified-query-summary/v1","status":"all_2048_finite_chain_counts_and_paired_targets_verified",
        "planned":2048,"rows":total,"boolean_sat":boolean_sat,"point_sat":point_sat,"both_sat":both_sat,
        "independent_point_arithmetic_rows_verified":independent,"paired_target_count":targets.len(),"models":models,"units":units,
        "scope":"additional explicitly selected a=0/full-degree-b public hash-to-subgroup controls; not a complete DLP or same-instance speed comparison"}),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reference_recount_matches_fast_arithmetic_on_odd_and_even_fields() {
        for n in [7, 8] {
            let inst = (0..64)
                .filter_map(|s| random_binary_instance(n, 9100 + s, 16))
                .find(|i| i.a == 0 && full_degree(&i.gf, i.b))
                .unwrap();
            validate(&inst);
            let curve = BinaryCurve {
                m: n,
                irreducible: inst.irreducible.clone(),
                a: element(inst.a, n),
                b: element(inst.b, n),
                generator: lower(inst.generator, n),
                order: BigUint::from(inst.r),
                cofactor: BigUint::from(inst.cofactor),
            };
            let reference = Reference(&curve);
            let fast = BinaryGroup(&inst.fast);
            let mut base = vec![];
            for x in 0..(1u64 << n) {
                let mut a: Vec<_> = points_with_x(&curve, &element(x, n))
                    .into_iter()
                    .map(lift)
                    .collect();
                let mut b = inst.points_with_x(x);
                a.sort_by_key(|p| p.pack());
                b.sort_by_key(|p| p.pack());
                assert_eq!(a, b);
                base.extend(
                    a.into_iter()
                        .filter(|&p| reference.mul(&mut GroupOps::default(), p, inst.r).infinity),
                );
            }
            assert!(!base.is_empty());
            base.truncate(8);
            assert_eq!(point_count(&reference, &[], inst.generator, 4).0, 0);
            for m in [3, 4] {
                for scalar in 0..8 {
                    let r = fast.mul(&mut GroupOps::default(), inst.generator, scalar);
                    assert_eq!(
                        point_count(&reference, &base, r, m).0,
                        point_count(&fast, &base, r, m).0
                    );
                }
            }
        }
    }
}
