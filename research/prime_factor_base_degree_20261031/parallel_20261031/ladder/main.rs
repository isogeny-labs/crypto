//! Deterministic, bounded construction and exact reference exporter.
//! Running the scientific ladder requires an externally supplied admission receipt.
mod core;
use core::*;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Spec {
    schema: String,
    seed: u64,
    primes: Vec<u64>,
    degrees: Vec<usize>,
    curve_attempt_cap: u64,
    graph_attempt_cap: u64,
    soft_seconds: u64,
    curve_selection: String,
    graph_selection: String,
    controls: Vec<String>,
    targets: String,
    stop_rule: String,
    scientific_run_admission: String,
}
fn write_once(path: &Path, value: &Value) -> Result<(String, u64), Box<dyn std::error::Error>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(&bytes)?;
    Ok((digest(&bytes), bytes.len() as u64))
}
fn word(bytes: &[u8]) -> u64 {
    u64::from_be_bytes(bytes[..8].try_into().unwrap())
}
fn source_hash() -> String {
    digest(
        [
            include_str!("main.rs"),
            include_str!("core.rs"),
            include_str!("Cargo.toml"),
            include_str!("Cargo.lock"),
            include_str!("spec.json"),
        ]
        .join("\n")
        .as_bytes(),
    )
}
fn validate_spec(s: &Spec) -> Result<(), String> {
    if s.schema!="secant-graph-ladder-spec/v1"||s.seed!=315||s.primes!=[83,101,127]||s.degrees!=[3,4,5]||s.curve_attempt_cap!=256||s.graph_attempt_cap!=512||s.soft_seconds!=120
        ||s.curve_selection!="first_hash_proposal_nonsingular_nonexceptional_nonanomalous_prime_full_order_ge23"
        ||s.graph_selection!="first_hash_interpolation_d_plus_one_distinct_signed_points_with_exact_2d_rational_roots"
        ||s.controls!=["low-x","hash-x"]||s.targets!="all_affine_curve_points_both_signs"
        ||s.stop_rule!="first_success_per_cell_or_frozen_cap_or_global_deadline"
        ||s.scientific_run_admission!="parent_must_verify_checkpoint_and_backend_or_explicit_owner_override"{return Err("spec differs from frozen v1 ladder".into());}
    Ok(())
}
fn validate_admission(receipt: &Value, spec_hash: &str) -> Result<(), String> {
    let allowed = [
        "schema",
        "workload",
        "source_revision",
        "protocol_sha256",
        "backend",
        "checkpoint_gate_passed",
        "checkpoint_override",
        "backend_override",
        "user_authorization_received",
        "max_seconds",
        "address_space_bytes",
        "authorization_reference",
    ];
    if receipt
        .as_object()
        .is_none_or(|object| object.keys().any(|key| !allowed.contains(&key.as_str())))
    {
        return Err("unknown admission field".into());
    }
    let revision = receipt["source_revision"]
        .as_str()
        .ok_or("admission lacks source revision")?;
    let reference = receipt["authorization_reference"]
        .as_str()
        .ok_or("admission lacks authorization reference")?;
    if receipt["schema"] != "secant-graph-run-admission/v1"
        || receipt["workload"] != "secant_graph_d3_d4_d5_ladder"
        || receipt["protocol_sha256"] != spec_hash
        || receipt["backend"] != "local"
        || receipt["checkpoint_gate_passed"] != false
        || receipt["checkpoint_override"] != true
        || receipt["backend_override"] != true
        || receipt["user_authorization_received"] != true
        || receipt["max_seconds"] != 180
        || receipt["address_space_bytes"] != 2147483648u64
        || revision.len() != 40
        || !revision.bytes().all(|b| b.is_ascii_hexdigit())
        || reference.trim().is_empty()
    {
        return Err("missing/mismatched run admission".into());
    }
    Ok(())
}
fn append_attempt(path: &Path, value: &Value) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("attempt journal open");
    let mut bytes = serde_json::to_vec(value).expect("attempt serialization");
    bytes.push(b'\n');
    file.write_all(&bytes).expect("attempt journal write");
}
fn select_curve(
    s: &Spec,
    p: u64,
    start: Instant,
    journal: &Path,
) -> (Option<Curve>, Vec<Value>, Ops, String) {
    let mut f = Field::new(p);
    let mut attempts = Vec::new();
    for attempt in 0..s.curve_attempt_cap {
        if start.elapsed().as_secs() >= s.soft_seconds {
            return (None, attempts, f.ops, "global_deadline".into());
        }
        let before = f.ops.clone();
        let h = f.hash("secant-graph-ladder/v1/curve", &[s.seed, p, attempt]);
        let a = 1 + word(&h[..8]) % (p - 1);
        let b = 1 + word(&h[8..16]) % (p - 1);
        let aa = f.mul(a, a);
        let aaa = f.mul(aa, a);
        let a4 = f.mul(4, aaa);
        let bb = f.mul(b, b);
        let b27 = f.mul(27, bb);
        let disc = f.add(a4, b27);
        if disc == 0 {
            attempts.push(json!({"attempt":attempt,"a":a,"b":b,"status":"singular","operations":f.ops.delta(&before)}));
            append_attempt(journal, attempts.last().unwrap());
            continue;
        }
        let pts = curve_points(&mut f, a, b);
        let order = pts.len() as u64 + 1;
        let prime = f.prime(order);
        let status = if !prime {
            "composite_full_order"
        } else if order == p {
            "anomalous"
        } else if order < 23 {
            "too_few_base_abscissae"
        } else {
            "accepted"
        };
        if status != "accepted" {
            attempts.push(json!({"attempt":attempt,"a":a,"b":b,"group_order":order,"status":status,"operations":f.ops.delta(&before)}));
            append_attempt(journal, attempts.last().unwrap());
            continue;
        }
        let curve = identity(&mut f, a, b, order, pts);
        let j = curve
            .icv1
            .split(':')
            .nth(4)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        if j == 0 || j == 1728 % p {
            attempts.push(json!({"attempt":attempt,"a":a,"b":b,"group_order":order,"status":"exceptional_j","operations":f.ops.delta(&before)}));
            append_attempt(journal, attempts.last().unwrap());
            continue;
        }
        attempts.push(json!({"attempt":attempt,"a":a,"b":b,"group_order":order,"status":"accepted","operations":f.ops.delta(&before)}));
        append_attempt(journal, attempts.last().unwrap());
        return (Some(curve), attempts, f.ops, "accepted".into());
    }
    (None, attempts, f.ops, "curve_attempt_cap".into())
}
fn abscissae(curve: &Curve) -> Vec<u64> {
    curve
        .points
        .iter()
        .map(|p| p.0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
fn low_ordinate(curve: &Curve, x: u64) -> u64 {
    curve.points.iter().find(|p| p.0 == x).unwrap().1
}
fn propose(
    f: &mut Field,
    s: &Spec,
    curve: &Curve,
    d: usize,
    attempt: u64,
) -> (Vec<u64>, Vec<Point>) {
    let mut order: Vec<_> = abscissae(curve)
        .into_iter()
        .map(|x| {
            let h = f.hash(
                "secant-graph-ladder/v1/forced",
                &[s.seed, curve.p, curve.a, curve.b, d as u64, attempt, x],
            );
            (h, x)
        })
        .collect();
    order.sort_unstable();
    let forced: Vec<Point> = order
        .into_iter()
        .take(d + 1)
        .map(|(hash, x)| {
            let y = low_ordinate(curve, x);
            let y = if hash[31] & 1 == 0 { y } else { curve.p - y };
            (x, y)
        })
        .collect();
    let h = interpolate(f, &forced).expect("distinct hash-selected interpolation nodes");
    (h, forced)
}
struct GraphCandidate {
    h: Vec<u64>,
    xs: Vec<u64>,
    forced: Vec<Point>,
    validation_operations: Ops,
}
type GraphSearchResult = (Option<GraphCandidate>, Value, Ops, String);
fn graph_search(
    s: &Spec,
    curve: &Curve,
    d: usize,
    start: Instant,
    journal: &Path,
) -> GraphSearchResult {
    let mut f = Field::new(curve.p);
    let mut attempts = Vec::new();
    let mut seen = BTreeMap::new();
    for attempt in 0..s.graph_attempt_cap {
        if start.elapsed().as_secs() >= s.soft_seconds {
            return (None, json!(attempts), f.ops, "global_deadline".into());
        }
        let before = f.ops.clone();
        let (h, forced) = propose(&mut f, s, curve, d, attempt);
        for (x, y) in &forced {
            if f.eval(&h, *x) != *y {
                append_attempt(
                    &journal.with_extension("failure.jsonl"),
                    &json!({"schema":"secant-graph-construction-failure/v1","curve":curve,"d":d,"attempt":attempt,"h_low_to_high":h,"forced_points":forced,"reason":"interpolation_identity_failed"}),
                );
                panic!("interpolation identity failed; exact context journaled");
            }
        }
        let degree = h.len().saturating_sub(1);
        let hash = digest(&serde_json::to_vec(&h).unwrap());
        f.ops.hash_calls += 1;
        if degree != d {
            attempts.push(json!([
                attempt,
                degree,
                null,
                "degree_drop",
                &hash[..16],
                f.ops.delta(&before)
            ]));
            append_attempt(journal, attempts.last().unwrap());
            continue;
        }
        if let Some(root_count) = seen.get(&h) {
            attempts.push(json!([
                attempt,
                degree,
                root_count,
                "duplicate_polynomial",
                &hash[..16],
                f.ops.delta(&before)
            ]));
            append_attempt(journal, attempts.last().unwrap());
            continue;
        }
        let roots: Vec<u64> = (0..curve.p)
            .filter(|x| {
                let y = f.eval(&h, *x);
                let yy = f.mul(y, y);
                let right = rhs(&mut f, curve.a, curve.b, *x);
                yy == right
            })
            .collect();
        if roots.len() > 2 * d || roots.len() < d + 1 {
            append_attempt(
                &journal.with_extension("failure.jsonl"),
                &json!({"schema":"secant-graph-construction-failure/v1","curve":curve,"d":d,"attempt":attempt,"h_low_to_high":h,"forced_points":forced,"roots":roots,"reason":"polynomial_root_bound_or_forced_membership_failed"}),
            );
            panic!("polynomial root invariant failed; exact context journaled");
        }
        seen.insert(h.clone(), roots.len());
        let status = if roots.len() == 2 * d {
            "accepted"
        } else {
            "residual_nonsplit_or_repeated"
        };
        attempts.push(json!([
            attempt,
            degree,
            roots.len(),
            status,
            &hash[..16],
            f.ops.delta(&before)
        ]));
        append_attempt(journal, attempts.last().unwrap());
        if status == "accepted" {
            let before_validation = f.ops.clone();
            let mut norm = univariate_product(&mut f, &h, &h);
            for (i, c) in [curve.b, curve.a, 0, 1].into_iter().enumerate() {
                norm[i] = f.sub(norm[i], c);
            }
            let mut expected = membership(&mut f, &roots);
            let scale = f.mul(*h.last().unwrap(), *h.last().unwrap());
            for c in &mut expected {
                *c = f.mul(*c, scale);
            }
            if trim(norm.clone()) != trim(expected.clone()) {
                append_attempt(
                    &journal.with_extension("failure.jsonl"),
                    &json!({"schema":"secant-graph-construction-failure/v1","curve":curve,"d":d,"attempt":attempt,"h_low_to_high":h,"roots":roots,"norm_coefficients":norm,"expected_coefficients":expected,"operations":f.ops}),
                );
                panic!("graph factorization identity failed; exact context journaled");
            }
            let validation_operations = f.ops.delta(&before_validation);
            return (
                Some(GraphCandidate {
                    h,
                    xs: roots,
                    forced,
                    validation_operations,
                }),
                json!(attempts),
                f.ops,
                "accepted".into(),
            );
        }
    }
    (None, json!(attempts), f.ops, "graph_attempt_cap".into())
}
fn control(f: &mut Field, s: &Spec, curve: &Curve, d: usize, name: &str) -> (Vec<u64>, Vec<u64>) {
    let mut xs = abscissae(curve);
    if name == "hash-x" {
        let mut ordered: Vec<_> = xs
            .into_iter()
            .map(|x| {
                (
                    f.hash(
                        "secant-graph-ladder/v1/control-x",
                        &[s.seed, curve.p, curve.a, curve.b, d as u64, x],
                    ),
                    x,
                )
            })
            .collect();
        ordered.sort_unstable();
        xs = ordered.into_iter().map(|(_, x)| x).collect();
    }
    xs.truncate(2 * d);
    xs.sort_unstable();
    let hpoints: Vec<_> = xs
        .iter()
        .map(|x| {
            let hash = f.hash(
                &format!("secant-graph-ladder/v1/control-y/{name}"),
                &[s.seed, curve.p, curve.a, curve.b, d as u64, *x],
            );
            let y = low_ordinate(curve, *x);
            (*x, if hash[31] & 1 == 0 { y } else { curve.p - y })
        })
        .collect();
    let h = interpolate(f, &hpoints).unwrap();
    (h, xs)
}
fn slope(f: &mut Field, curve: &Curve, p: Point, q: Point) -> u64 {
    let (n, d) = if p.0 == q.0 {
        let xx = f.mul(p.0, p.0);
        let triple = f.mul(3, xx);
        (f.add(triple, curve.a), f.mul(2, p.1))
    } else {
        (f.sub(q.1, p.1), f.sub(q.0, p.0))
    };
    let inv = f.inv(d);
    f.mul(n, inv)
}
struct Arm {
    name: String,
    h: Vec<u64>,
    xs: Vec<u64>,
    points: Vec<Point>,
    membership: Vec<u64>,
    dump: Value,
    complete: bool,
}
fn arm(f: &mut Field, curve: &Curve, name: &str, h: Vec<u64>, xs: Vec<u64>, complete: bool) -> Arm {
    let points = signed_points(f, &xs, &h);
    let membership = membership(f, &xs);
    f.ops.hash_calls += 2;
    let dump = dump(curve, &xs, &points);
    Arm {
        name: name.into(),
        h,
        xs,
        points,
        membership,
        dump,
        complete,
    }
}
fn expected(f: &mut Field, curve: &Curve, arm: &Arm) -> BTreeMap<Point, Vec<[u64; 3]>> {
    let mut out: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for (i, p) in arm.points.iter().enumerate() {
        for (j, q) in arm.points.iter().enumerate() {
            if let Some(r) = point_add(f, curve, Some(*p), Some(*q)) {
                let m = slope(f, curve, *p, *q);
                out.entry(r).or_default().push([i as u64, j as u64, m]);
            }
        }
    }
    out
}
fn evaluate_counted(f: &mut Field, poly: &[Term], point: &[u64]) -> u64 {
    f.ops.field_adds += poly.len() as u64;
    f.ops.field_muls += poly
        .iter()
        .map(|t| t.exponents.iter().map(|e| u64::from(*e)).sum::<u64>())
        .sum::<u64>();
    evaluate(poly, point, f.p)
}
fn export_case(
    s: &Spec,
    curve: &Curve,
    d: usize,
    candidate: GraphCandidate,
    output: &Path,
    attempts: Value,
    construction: Ops,
) -> Result<Value, Box<dyn std::error::Error>> {
    let GraphCandidate {
        h,
        xs,
        forced,
        validation_operations,
    } = candidate;
    fs::create_dir(output)?;
    let mut f = Field::new(curve.p);
    let mut arms = vec![arm(&mut f, curve, "candidate", h, xs, true)];
    for name in &s.controls {
        let (h, xs) = control(&mut f, s, curve, d, name);
        arms.push(arm(&mut f, curve, name, h, xs, false));
    }
    let base_ops = f.ops.clone();
    let mut references = Vec::new();
    let mut systems = Vec::new();
    let mut arm_records = Vec::new();
    for arm in &arms {
        let before = f.ops.clone();
        let refs = expected(&mut f, curve, arm);
        let reference_ops = f.ops.delta(&before);
        let mut relation_count = 0;
        let mut tangent_count = 0;
        let mut unordered_count = 0;
        let mut sat_targets = 0;
        let mut all_targets = Vec::new();
        for (target_index, target) in curve.points.iter().copied().enumerate() {
            let pairs = refs.get(&target).cloned().unwrap_or_default();
            relation_count += pairs.len();
            tangent_count += pairs.iter().filter(|p| p[0] == p[1]).count();
            let target_unordered = pairs
                .iter()
                .map(|pair| [pair[0].min(pair[1]), pair[0].max(pair[1])])
                .collect::<BTreeSet<_>>()
                .len();
            unordered_count += target_unordered;
            if !pairs.is_empty() {
                sat_targets += 1;
            }
            let dense_roots: Vec<_> = pairs
                .iter()
                .map(|pair| {
                    vec![
                        arm.points[pair[0] as usize].0,
                        arm.points[pair[1] as usize].0,
                    ]
                })
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let fb = arm.dump["factor_base"]["fb_id"].clone();
            let role = if arm.complete { "candidate" } else { "control" };
            let base_record = json!({"p":curve.p,"a":curve.a,"b":curve.b,"group_order":curve.group_order,"generator":curve.generator});
            systems.push(json!({"id":format!("{}-d{d}-{}-t{target_index}-dense",curve.slug,arm.name),"p":curve.p,"variables":["x","z"],"polynomials":dense(curve,&arm.membership,target.0),"curve":base_record,"target":target,"base_x":arm.xs,"base_points":arm.points,"h_low_to_high":arm.h,"arm":format!("{role}-dense"),"control_name":arm.name,"d":d,"fb_id":fb,"curve_slug":curve.slug,"target_index":target_index,"signs":null,"expected_roots":dense_roots}));
            for signs in [[1, 1], [1, -1], [-1, 1], [-1, -1]] {
                let mut roots = Vec::new();
                for pair in &pairs {
                    let p = arm.points[pair[0] as usize];
                    let q = arm.points[pair[1] as usize];
                    let hp = f.eval(&arm.h, p.0);
                    let hq = f.eval(&arm.h, q.0);
                    let yp = if signs[0] == 1 { hp } else { curve.p - hp };
                    let yq = if signs[1] == 1 { hq } else { curve.p - hq };
                    if p.1 == yp && q.1 == yq {
                        roots.push(vec![p.0, q.0, pair[2]]);
                    }
                }
                roots.sort();
                roots.dedup();
                let polynomials =
                    secant(curve, &arm.h, &arm.membership, target, signs, arm.complete);
                for root in &roots {
                    for (generator_index, polynomial) in polynomials.iter().enumerate() {
                        let value = evaluate_counted(&mut f, polynomial, root);
                        if value != 0 {
                            write_once(
                                &output.join("FAILURE.json"),
                                &json!({"schema":"secant-graph-export-failure/v1","curve":curve,"d":d,"arm":arm.name,"h_low_to_high":arm.h,"base_x":arm.xs,"target":target,"signs":signs,"root":root,"generator_index":generator_index,"polynomial":polynomial,"value":value,"operations":f.ops,"source_sha256":source_hash()}),
                            )?;
                            return Err("expected group root failed exported constraint; exact context retained".into());
                        }
                    }
                }
                systems.push(json!({"id":format!("{}-d{d}-{}-t{target_index}-secant-{}-{}",curve.slug,arm.name,signs[0],signs[1]),"p":curve.p,"variables":["x","z","m"],"polynomials":polynomials,"curve":base_record,"target":target,"base_x":arm.xs,"base_points":arm.points,"h_low_to_high":arm.h,"arm":format!("{role}-secant"),"control_name":arm.name,"d":d,"fb_id":fb,"curve_slug":curve.slug,"target_index":target_index,"signs":signs,"expected_roots":roots}));
            }
            let negative = (target.0, curve.p - target.1);
            if pairs.len() != refs.get(&negative).map_or(0, Vec::len) {
                write_once(
                    &output.join("FAILURE.json"),
                    &json!({"schema":"secant-graph-reference-failure/v1","curve":curve,"d":d,"arm":arm.name,"target":target,"negative_target":negative,"pairs":pairs,"negative_pairs":refs.get(&negative),"reason":"negation_target_count_mismatch"}),
                )?;
                return Err("reference negation symmetry failed; context retained".into());
            }
            all_targets.push(json!({"target":target,"ordered_pairs_with_slope":pairs,"distinct_unordered_pairs":target_unordered}));
        }
        let k = arm.points.len();
        if relation_count != k * k - k || tangent_count != k || unordered_count != k * k / 2 {
            write_once(
                &output.join("FAILURE.json"),
                &json!({"schema":"secant-graph-reference-failure/v1","curve":curve,"d":d,"arm":arm.name,"signed_base":arm.points,"finite_ordered_count":relation_count,"tangent_count":tangent_count,"unordered_count":unordered_count,"references":all_targets,"reason":"complete_relation_totals_mismatch"}),
            )?;
            return Err("complete relation invariant failed; context retained".into());
        }
        let inverse_pairs: Vec<_> = arm
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let j = arm
                    .points
                    .iter()
                    .position(|q| q.0 == p.0 && q.1 == curve.p - p.1)
                    .unwrap();
                [i, j]
            })
            .collect();
        assert_eq!(inverse_pairs.len(), k);
        references.push(json!({"name":arm.name,"points":arm.points,"targets":all_targets,"infinity_inverse_pairs":inverse_pairs,"finite_ordered_relation_total":relation_count,"tangent_ordered_relations":tangent_count,"all_target_negation_symmetry_checked":true}));
        arm_records.push(json!({"name":arm.name,"h_low_to_high":arm.h,"oriented_interpolant_degree":arm.h.len()-1,"x_values":arm.xs,"complete_graph":arm.complete,"factor_base_dump":arm.dump,"same_full_signed_point_set_as_candidate":arm.points==arms[0].points,"finite_target_count":curve.points.len(),"sat_finite_target_count":sat_targets,"finite_ordered_relation_total":relation_count,"mean_ordered_yield_numerator":relation_count,"mean_ordered_yield_denominator":curve.points.len(),"tangent_relation_total":tangent_count,"finite_distinct_unordered_relation_total":unordered_count,"identity_inverse_relation_total":k,"derived_dense_homogeneous_degree":arm.xs.len()+1,"derived_secant_homogeneous_degree":if arm.h.len()>=3{Some(arm.h.len())}else{None},"reference_group_operations":reference_ops,"independent_replay_status":"producer_checks_only_external_runner_required"}));
    }
    let ops = f.ops.delta(&base_ops);
    let mut written = 0;
    let (attempt_hash, n) = write_once(&output.join("attempts.json"), &attempts)?;
    written += n;
    let (references_hash, n) = write_once(
        &output.join("references.json"),
        &json!({"schema":"secant-graph-pair-references/v1","curve":curve,"arms":references}),
    )?;
    written += n;
    let (systems_hash, n) = write_once(
        &output.join("systems.json"),
        &json!({"schema":"secant-graph-systems/v1","systems":systems}),
    )?;
    written += n;
    let case = json!({"schema":"secant-graph-ladder-case/v1","curve":curve,"d":d,"status":"accepted","forced_interpolation_points":forced,"construction_operations":construction,"post_attempt_factorization_validation_operations_subset":validation_operations,"control_and_base_operations":base_ops,"reference_and_root_verification_operations":ops,"polynomial_coefficient_export_operations":null,"artifact_hash_operations":null,"attempt_log_sha256":attempt_hash,"references_sha256":references_hash,"systems_sha256":systems_hash,"systems_count":systems.len(),"serialized_payload_bytes_before_case":written,"arms":arm_records,"homogeneous_degree_kind":"derived_from_generator_top_ideals","actual_f4_degree":null,"rank_independent_rows":null,"timing_qualification":"unqualified_construction_process_not_f4_performance"});
    let (_, n) = write_once(&output.join("case.json"), &case)?;
    written += n;
    Ok(
        json!({"status":"accepted","curve":curve.slug,"d":d,"directory":output.file_name().unwrap().to_string_lossy(),"systems_count":systems.len(),"serialized_payload_bytes":written,"systems_sha256":systems_hash,"attempts":attempts.as_array().unwrap().len(),"construction_operations":construction}),
    )
}
fn run(
    spec_path: &str,
    out_path: &str,
    admission_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec_bytes = fs::read(spec_path)?;
    let spec: Spec = serde_json::from_slice(&spec_bytes)?;
    validate_spec(&spec)?;
    let spec_hash = digest(&spec_bytes);
    let admission_bytes = fs::read(admission_path)?;
    let admission: Value = serde_json::from_slice(&admission_bytes)?;
    validate_admission(&admission, &spec_hash)?;
    let binary_hash = digest(&fs::read(std::env::current_exe()?)?);
    let out = Path::new(out_path);
    if out.exists() {
        return Err("refuse existing output directory".into());
    }
    fs::create_dir(out)?;
    write_once(
        &out.join("RUNNING.json"),
        &json!({"schema":"secant-graph-ladder-run/v1","protocol_sha256":spec_hash,"source_sha256":source_hash(),"binary_sha256":binary_hash,"admission":admission,"status":"running"}),
    )?;
    let start = Instant::now();
    let mut cells = Vec::new();
    let mut curves = Vec::new();
    for p in &spec.primes {
        let (curve, curve_attempts, curve_ops, status) = select_curve(
            &spec,
            *p,
            start,
            &out.join(format!("curve-search-p{p}.jsonl")),
        );
        write_once(
            &out.join(format!("curve-search-p{p}.json")),
            &json!({"p":p,"attempts":curve_attempts,"operations":curve_ops,"status":status}),
        )?;
        let Some(curve) = curve else {
            for d in &spec.degrees {
                cells.push(
                    json!({"p":p,"d":d,"status":"curve_selection_unavailable","reason":status}),
                );
            }
            continue;
        };
        curves.push(json!({"curve":curve,"selection_operations":curve_ops}));
        for d in &spec.degrees {
            let (candidate, attempts, operations, status) = graph_search(
                &spec,
                &curve,
                *d,
                start,
                &out.join(format!("{}-d{d}-active-attempts.jsonl", curve.slug)),
            );
            let name = format!("{}-d{d}", curve.slug);
            if let Some(candidate) = candidate {
                let cell = export_case(
                    &spec,
                    &curve,
                    *d,
                    candidate,
                    &out.join(&name),
                    attempts,
                    operations,
                )?;
                append_attempt(&out.join("cell-completions.jsonl"), &cell);
                cells.push(cell);
            } else {
                let file = format!("{name}-failed-attempts.json");
                let (hash, bytes) = write_once(&out.join(&file), &attempts)?;
                cells.push(json!({"p":p,"curve":curve.slug,"d":d,"status":status,"attempt_log":file,"attempt_log_sha256":hash,"serialized_payload_bytes":bytes,"operations":operations}));
            }
        }
    }
    let accepted = cells.iter().filter(|c| c["status"] == "accepted").count();
    let manifest = json!({"schema":"secant-graph-ladder-run/v1","protocol_sha256":spec_hash,"source_sha256":source_hash(),"source_revision":admission["source_revision"],"binary_sha256":binary_hash,"admission":admission,"requested_fields":spec.primes,"requested_degrees":spec.degrees,"requested_cells":9,"accepted_cells":accepted,"curves":curves,"cells":cells,"elapsed_wall_millis_unqualified":start.elapsed().as_millis(),"reference_work_included":true,"all_scientific_attempts_retained":true,"all_target_policy":"all_finite_points_both_signs_plus_identity_pair_record","f4_run_performed":false,"rss_bytes":null,"physical_disk_traffic_bytes":null,"input_bytes":spec_bytes.len()+admission_bytes.len(),"outcome":"construction_and_exact_reference_export_only"});
    write_once(&out.join("manifest.json"), &manifest)?;
    println!(
        "{}",
        json!({"accepted_cells":accepted,"requested_cells":9,"manifest":out.join("manifest.json")})
    );
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 6 && args[1] == "run" && args[4] == "--admission-receipt" {
        run(&args[2], &args[3], &args[5])
    } else {
        Err("usage: secant-graph-ladder run SPEC.json FRESH-OUTPUT --admission-receipt RECEIPT.json".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_spec_parses_and_rejects_silent_degree_drop() {
        let mut s: Spec = serde_json::from_str(include_str!("spec.json")).unwrap();
        validate_spec(&s).unwrap();
        s.degrees.pop();
        assert!(validate_spec(&s).is_err());
    }
    #[test]
    fn missing_admission_is_rejected() {
        assert!(validate_admission(&json!({}), "abc").is_err());
    }
    #[test]
    fn root_verification_counter_matches_naive_evaluator() {
        let mut f = Field::new(17);
        let poly = vec![Term {
            exponents: vec![2, 1, 0],
            coefficient: 2,
        }];
        assert_eq!(evaluate_counted(&mut f, &poly, &[3, 4, 0]), 4);
        assert_eq!(f.ops.field_adds, 1);
        assert_eq!(f.ops.field_muls, 3);
    }
}
