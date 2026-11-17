//! Fixed compiler/certificate and public-input controls, never a timed campaign.
use crate::{check, preprocess, sha256, system_hash, Artifact, Limits};
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{fs, io::Read, path::Path};
use torus_coset_circuit::{index::IndexTable, Spec};
use torus_indexed_circuit::Augmented;
use torus_s4::{Point, Poly, System};
use torus_s4_tree::{Fp2, TorusParameters};
use torus_worklist_solver::{solve, Answer, Report, Schedule};
type Result<T> = std::result::Result<T, String>;
pub const PUBLIC_TARGETS: &[u8] = include_bytes!("../fixtures/public-targets.json");
pub const HUGE_ORIGINAL:&[u8]=include_bytes!("../../../research/curve25519_torus_native_20261115/target-circuit-v2/native-v2/six-member-system.json.gz");
pub const HUGE_INDEXED:&[u8]=include_bytes!("../../../research/curve25519_torus_native_20261115/indexed-circuit-v1/native-v1/actual-augmented-system.json");
fn write(path: &Path, v: &Value) -> Result<()> {
    fs::write(
        path,
        serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
pub fn compiled_sources() -> Value {
    json!({"Cargo.toml":sha256(include_bytes!("../Cargo.toml")),"Cargo.lock":sha256(include_bytes!("../Cargo.lock")),
        "src/lib.rs":sha256(include_bytes!("lib.rs")),"src/check.rs":sha256(include_bytes!("check.rs")),
        "src/controls.rs":sha256(include_bytes!("controls.rs")),"src/main.rs":sha256(include_bytes!("main.rs")),
        "fixtures/public-targets.json":sha256(PUBLIC_TARGETS),"huge_original":sha256(HUGE_ORIGINAL),"huge_indexed":sha256(HUGE_INDEXED)})
}
fn source_identity() -> String {
    sha256(compiled_sources().to_string().as_bytes())
}
fn request(id: &str, system: Value, protected: &[usize]) -> Value {
    json!({"schema":"torus.affine-request/v1","id":id,"source_identity_sha256":source_identity(),
        "system":system,"protected":protected,"preprocess_limits":Limits::default(),"solver_limits":torus_worklist_solver::Limits::default()})
}
fn checked_root(report: &Report, original: &System) -> Result<Option<Vec<BigUint>>> {
    if let Answer::Root(strings) = &report.answer {
        let values = strings
            .iter()
            .map(|s| check::decimal(s))
            .collect::<Result<Vec<_>>>()?;
        check::assignment(original, &values)?;
        original.check(&values)?;
        let mut alias = values.clone();
        alias[0] += &original.p;
        if check::assignment(original, &alias).is_ok()
            || check::assignment(original, &values[..values.len() - 1]).is_ok()
        {
            return Err("independent shape/noncanonical refusal failed".into());
        }
        Ok(Some(values))
    } else if matches!(
        report.answer,
        Answer::Stopped(torus_worklist_solver::Stop::Invalid(_))
    ) {
        Err(format!("unexpected solver invalid: {:?}", report.answer))
    } else {
        Ok(None)
    }
}
struct Case {
    original: System,
    output: Option<System>,
    before: Option<Report>,
    after: Option<Report>,
    artifact: Option<Artifact>,
    summary: Value,
}
fn one_case(
    root: &Path,
    id: &str,
    value: Value,
    protected: &[usize],
    run_solver: bool,
) -> Result<Case> {
    eprintln!("fixed affine case {id}; solver={run_solver}");
    let dir = root.join(id);
    fs::create_dir(&dir).map_err(|e| e.to_string())?;
    let req = request(id, value, protected);
    let raw = serde_json::to_vec_pretty(&req).map_err(|e| e.to_string())?;
    fs::write(dir.join("request.json"), &raw).map_err(|e| e.to_string())?;
    write(
        &dir.join("PRE_CALL.json"),
        &json!({"schema":"torus.affine-input-seal/v1","request_sha256":sha256(&raw),
        "request_bytes":raw.len(),"source_identity_sha256":source_identity(),"mode":if run_solver {"fixed-comparator"}else{"static-only-zero-solver-calls"},
        "prepared_before":"parser/compiler/solver invocation; no timing receipt inferred"}),
    )?;
    let retained = fs::read(dir.join("request.json")).map_err(|e| e.to_string())?;
    if retained != raw {
        return Err("request readback mismatch".into());
    }
    let (s, roles) = check::parse_request(&retained)?;
    if roles != protected {
        return Err("public roles changed".into());
    }
    let before = if run_solver {
        let r = solve(
            &s,
            &roles,
            Schedule::Worklist,
            torus_worklist_solver::Limits::default(),
        );
        write(
            &dir.join("before.json"),
            &serde_json::to_value(&r).map_err(|e| e.to_string())?,
        )?;
        checked_root(&r, &s)?;
        Some(r)
    } else {
        None
    };
    let result = preprocess(&s, &roles, Limits::default());
    write(
        &dir.join("preprocess.json"),
        &serde_json::to_value(&result).map_err(|e| e.to_string())?,
    )?;
    // Full certificate is contained in preprocess.json; round-trip before checking.
    let serialized = fs::read(dir.join("preprocess.json")).map_err(|e| e.to_string())?;
    let decoded: Value = serde_json::from_slice(&serialized).map_err(|e| e.to_string())?;
    let artifact = if decoded["artifact"].is_null() {
        None
    } else {
        Some(
            serde_json::from_value::<Artifact>(decoded["artifact"].clone())
                .map_err(|e| e.to_string())?,
        )
    };
    let output = if let Some(a) = &artifact {
        Some(check::verify(&s, &roles, a)?)
    } else {
        None
    };
    let after = if run_solver {
        if let Some(output) = &output {
            let r = solve(
                output,
                &roles,
                Schedule::Worklist,
                torus_worklist_solver::Limits::default(),
            );
            write(
                &dir.join("after.json"),
                &serde_json::to_value(&r).map_err(|e| e.to_string())?,
            )?;
            checked_root(&r, &s)?;
            if let Some(w) = checked_root(&r, output)? {
                check::assignment(&s, &w)?;
            }
            Some(r)
        } else {
            None
        }
    } else {
        None
    };
    let summary = json!({"id":id,"input_sha256":sha256(&raw),"original_system_sha256":system_hash(&s),
        "variables":s.variables.len(),"original_equations":s.equations.len(),"protected":roles,
        "compiler_counts":result.counts,"compiler_stop":result.stop,"certificate_checked":output.is_some(),
        "output_system_sha256":output.as_ref().map(system_hash),"output_equations":output.as_ref().map(|s|s.equations.len()),
        "output_degree":output.as_ref().map(|s|s.equations.iter().map(Poly::degree).max().unwrap_or(0)),
        "solver_invocations":usize::from(before.is_some())+usize::from(after.is_some()),
        "before":before,"after":after,"standing":"producer unsigned/unverified"});
    write(&dir.join("summary.json"), &summary)?;
    Ok(Case {
        original: s,
        output,
        before,
        after,
        artifact,
        summary,
    })
}
fn roles(a: &Augmented, indexed: bool) -> Vec<usize> {
    a.roles
        .iter()
        .flat_map(|r| {
            r.selector_bits
                .iter()
                .copied()
                .chain(if indexed {
                    r.index_bits.clone()
                } else {
                    vec![]
                })
                .chain(r.branch_bit)
                .chain(r.raw_v)
                .chain(if indexed {
                    vec![]
                } else {
                    r.parameter.to_vec()
                })
        })
        .collect()
}
fn point(v: &Value) -> Result<Point> {
    if v.is_null() {
        Ok(None)
    } else {
        Ok(Some((
            check::decimal(v[0].as_str().ok_or("point u")?)?,
            check::decimal(v[1].as_str().ok_or("point v")?)?,
        )))
    }
}
fn curve_case(root: &Path, id: &str, spec: &Spec, target: Point) -> Result<Vec<Value>> {
    let (c, a) = Augmented::target(spec, 2, target, true)?;
    let mut out = vec![];
    for indexed in [false, true] {
        let id = format!("{id}-{}", if indexed { "indexed" } else { "original" });
        let original = if indexed { &a.system } else { &c.system };
        let case = one_case(
            root,
            &id,
            original.canonical_json(),
            &roles(&a, indexed),
            true,
        )?;
        let table = IndexTable::new(spec)?; // replay-only; never an oracle query input.
        let mut replays = vec![];
        for (arm, r) in [("before", &case.before), ("after", &case.after)] {
            if let Some(r) = r {
                if matches!(r.answer, Answer::NoRootObservation) {
                    return Err("known-consistent fixed target reported contradiction".into());
                }
                let row = if let Some(w) = checked_root(r, &case.original)? {
                    let w = if indexed { a.project_and_check(&w)? } else { w };
                    Some(
                        c.decode_row(&w, &table)?
                            .iter()
                            .map(|r| json!({"j":r.j,"t":r.t,"coefficient":r.coefficient}))
                            .collect::<Vec<_>>(),
                    )
                } else {
                    None
                };
                replays.push(json!({"arm":arm,"row":row}));
            }
        }
        write(&root.join(&id).join("row-replay.json"), &json!(replays))?;
        out.push(json!({"case":case.summary,"row_replays":replays}));
    }
    Ok(out)
}
fn poly_system(p: &BigUint, n: usize, equations: Vec<Poly>) -> System {
    System {
        p: p.clone(),
        variables: (0..n).map(|i| format!("x{i}")).collect(),
        equations,
    }
}
fn expect_bad(s: &System, roles: &[usize], a: &Artifact) -> Result<()> {
    if check::verify(s, roles, a).is_ok() {
        return Err("corrupted certificate accepted".into());
    }
    Ok(())
}
fn perturb(s: &str, p: &BigUint) -> Result<String> {
    let mut next = (check::decimal(s)? + 1u32) % p;
    if next == BigUint::from(0u32) {
        next = 1u32.into();
    }
    Ok(next.to_string())
}
fn small_controls(root: &Path) -> Result<Value> {
    let p = 29u32.into();
    let v = |i| Poly::var(&p, i);
    let c = |n| Poly::small(&p, n);
    let coupled = poly_system(
        &p,
        3,
        vec![
            &(&v(0) + &v(1)) - &c(3),
            &(&v(0) - &v(1)) - &c(1),
            &v(2) - &(&v(0) * &v(1)),
        ],
    );
    let conflict = poly_system(
        &p,
        2,
        vec![&(&v(0) + &v(1)) - &c(1), &(&v(0) + &v(1)) - &c(2)],
    );
    let nonlinear = poly_system(&p, 2, vec![&(&v(0) * &v(1)) - &c(1)]);
    let protected = poly_system(&p, 2, vec![&v(0) - &c(2), &v(1) - &v(0)]);
    let mut summaries = vec![];
    let mut sample = None;
    for (id, s, roles) in [
        ("small-coupled", coupled.clone(), vec![]),
        ("small-conflict", conflict, vec![]),
        ("small-nonlinear", nonlinear, vec![]),
        ("small-protected", protected, vec![0]),
    ] {
        let case = one_case(root, id, s.canonical_json(), &roles, true)?;
        let output = case.output.as_ref().ok_or("small certificate was capped")?;
        let mut trace = vec![];
        let mut accepted = 0;
        for mut index in 0..29u64.pow(s.variables.len() as u32) {
            let mut assignment = vec![];
            for _ in &s.variables {
                assignment.push(BigUint::from(index % 29));
                index /= 29;
            }
            let a = check::assignment(&s, &assignment).is_ok();
            let b = check::assignment(output, &assignment).is_ok();
            if a != b {
                return Err("finite original/closure truth mismatch".into());
            }
            accepted += usize::from(a);
            trace.push(u8::from(a) | (u8::from(b) << 1));
        }
        fs::write(root.join(id).join("truth-trace.bin"), &trace).map_err(|e| e.to_string())?;
        summaries.push(json!({"case":case.summary,"exhaustive_assignments":trace.len(),"accepted":accepted,"trace_sha256":sha256(&trace)}));
        if id == "small-coupled" {
            sample = case.artifact;
        }
    }
    let good = sample.ok_or("missing small certificate")?;
    let corrupt_dir = root.join("corruptions");
    fs::create_dir(&corrupt_dir).map_err(|e| e.to_string())?;
    let mut refusal = vec![];
    for kind in 0..8 {
        let mut bad = good.clone();
        match kind {
            0 => {
                bad.basis[0].origins[0].coefficient =
                    perturb(&bad.basis[0].origins[0].coefficient, &p)?
            }
            1 => bad.basis[0].pivot = Some(3),
            2 => {
                bad.basis[0].polynomial[0].coefficient =
                    perturb(&bad.basis[0].polynomial[0].coefficient, &p)?
            }
            3 => bad.rewrites[0].source_equation = 2,
            4 => {
                bad.rewrites[2].residual[0].coefficient =
                    perturb(&bad.rewrites[2].residual[0].coefficient, &p)?
            }
            5 => {
                bad.rewrites[0].quotients[0].polynomial[0].coefficient =
                    perturb(&bad.rewrites[0].quotients[0].polynomial[0].coefficient, &p)?
            }
            6 => bad.rewrites[0].quotients[0].basis = bad.basis.len(),
            _ => bad.protected.push(0),
        }
        expect_bad(&coupled, &[], &bad)?;
        write(
            &corrupt_dir.join(format!("certificate-{kind}.json")),
            &serde_json::to_value(&bad).map_err(|e| e.to_string())?,
        )?;
        refusal.push(json!({"kind":kind,"refused":true}));
    }
    let mut wrong = coupled.clone();
    wrong.variables[0] = "changed-public-name".into();
    expect_bad(&wrong, &[], &good)?;
    let mut malformed = vec![];
    for kind in 0..11 {
        let mut bad = request("malformed", coupled.canonical_json(), &[]);
        match kind {
            0 => bad["system"]["p"] = "28".into(),
            1 => bad["system"]["variables"][1] = "x0".into(),
            2 => bad["system"]["equations"][0][0]["coefficient"] = "29".into(),
            3 => {
                let t = bad["system"]["equations"][0][0].clone();
                bad["system"]["equations"][0]
                    .as_array_mut()
                    .unwrap()
                    .push(t);
            }
            4 => bad["system"]["equations"][0][0]["monomial"] = json!([1, 0]),
            5 => bad["system"]["equations"][0][0]["monomial"] = json!([3]),
            6 => bad["system"]["equations"][0][0]["coefficient"] = "01".into(),
            7 => bad["witness"] = json!([1]),
            8 => bad["protected"] = json!([0, 0]),
            9 => bad["solver_limits"]["events"] = 1u32.into(),
            _ => bad["source_identity_sha256"] = "z".repeat(64).into(),
        }
        let bytes = serde_json::to_vec_pretty(&bad).map_err(|e| e.to_string())?;
        if check::parse_request(&bytes).is_ok() {
            return Err("malformed public request accepted".into());
        }
        fs::write(corrupt_dir.join(format!("request-{kind}.json")), &bytes)
            .map_err(|e| e.to_string())?;
        malformed
            .push(json!({"case":kind,"bytes":bytes.len(),"sha256":sha256(&bytes),"refused":true}));
    }
    let oversized = vec![b' '; (2 << 20) + 1];
    fs::write(corrupt_dir.join("oversized-request.bin"), &oversized).map_err(|e| e.to_string())?;
    if check::parse_request(&oversized).is_ok() {
        return Err("request byte cap accepted".into());
    }
    let caps = [
        Limits {
            variables: 0,
            ..Limits::default()
        },
        Limits {
            equations: 0,
            ..Limits::default()
        },
        Limits {
            input_terms: 0,
            ..Limits::default()
        },
        Limits {
            stored_terms: 0,
            ..Limits::default()
        },
        Limits {
            pairs: 0,
            ..Limits::default()
        },
        Limits {
            events: 0,
            ..Limits::default()
        },
        Limits {
            cumulative_modeled_bytes: 0,
            ..Limits::default()
        },
    ];
    let mut cap_records = vec![];
    for limits in caps {
        let r = preprocess(&coupled, &[], limits.clone());
        if !matches!(r.stop, Some(crate::Stop::Cap(_))) || r.artifact.is_some() {
            return Err("forced preprocessing cap not refused".into());
        }
        cap_records.push(json!({"limits":limits,"report":r}));
    }
    // Unchanged v1 branch semantics: x=0 is unsupported cubic; x=1 succeeds.
    let mut child = poly_system(&p, 2, vec![&(&v(0) * &v(0)) - &v(0), c(0)]);
    for (m, a) in [
        (vec![1, 1, 1], 1u32),
        (vec![], 27),
        (vec![0, 1, 1, 1], 28),
        (vec![0], 1),
        (vec![0, 1, 1], 1),
    ] {
        child.equations[1].terms.insert(m, a.into());
    }
    let child_limits = torus_worklist_solver::Limits {
        degree: 4,
        ..torus_worklist_solver::Limits::default()
    };
    write(
        &root.join("unsupported-child-input.json"),
        &json!({"schema":"declared-v1-generic-interface-control","system":child.canonical_json(),"roles":[0,1],"limits":child_limits}),
    )?;
    let child_report = solve(&child, &[0, 1], Schedule::Worklist, child_limits);
    let w = checked_root(&child_report, &child)?
        .ok_or("unsupported child suppressed successful branch")?;
    if w[0] != 1u32.into() || child_report.counts.undos == 0 {
        return Err("unsupported child not actually undone".into());
    }
    Ok(
        json!({"polynomial":summaries,"corruptions":refusal,"malformed_requests":malformed,"oversized_request":{"bytes":oversized.len(),"sha256":sha256(&oversized),"refused":true},"original_binding_mutation_refused":true,"forced_caps":cap_records,"unsupported_child":child_report}),
    )
}
pub fn run(out: &Path) -> Result<Value> {
    let small = small_controls(out)?;
    let targets: Value = serde_json::from_slice(PUBLIC_TARGETS).map_err(|e| e.to_string())?;
    let toy = Spec {
        p: 101u32.into(),
        h: Fp2 {
            a: 17u32.into(),
            b: 12u32.into(),
        },
        order_factor: 17,
        q: 3,
        k: 8,
        curve_a: Some(95u32.into()),
    };
    let mut toy_cases = vec![];
    let qs = targets["toy_targets"]
        .as_array()
        .ok_or("public targets array")?;
    if qs.len() != 13 {
        return Err("public target count changed".into());
    }
    for (i, q) in qs.iter().enumerate() {
        toy_cases.extend(curve_case(out, &format!("toy-{i:02}"), &toy, point(q)?)?);
    }
    let old = TorusParameters::actual_four()?;
    let actual = Spec {
        p: old.f.p.clone(),
        h: old.h.pow(&213156431u32.into(), &old.f),
        order_factor: 60824497,
        q: 1,
        k: 2,
        curve_a: Some(486662u32.into()),
    };
    let actual_cases = curve_case(
        out,
        "actual-q1",
        &actual,
        point(&targets["actual_q1_target"])?,
    )?;
    let mut original = vec![];
    GzDecoder::new(HUGE_ORIGINAL)
        .read_to_end(&mut original)
        .map_err(|e| e.to_string())?;
    let mut static_cases = vec![];
    for (id, bytes) in [
        ("actual-huge-original", original.as_slice()),
        ("actual-huge-indexed", HUGE_INDEXED),
    ] {
        let s: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        let result = one_case(out, id, s, &[], false)?;
        static_cases.push(result.summary);
    }
    Ok(
        json!({"schema":"torus.affine-controls/v1","standing":"producer unsigned/unverified","small":small,"toy":toy_cases,"actual_q1":actual_cases,"actual_huge_static_only":static_cases,
        "accounting":"partial compiler reservations and old solver diagnostics, no cold/elapsed/live-memory/exponent claim","source_identity_sha256":source_identity()}),
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn fixed_correctness_controls() {
        let out = std::env::temp_dir().join(format!(
            "curve25519-affine-unit-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&out).unwrap();
        if let Err(e) = super::run(&out) {
            panic!("{e}; complete attempt retained at {}", out.display());
        }
    }
}
