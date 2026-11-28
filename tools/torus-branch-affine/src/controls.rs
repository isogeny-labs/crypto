//! Fixed public-input, exact-disabled-baseline and conditional lifetime controls.
use crate::{check, solve, Answer, DynamicLimits, Event, Limits, Mode, Report, Schedule, Stop};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{fs, path::Path};
use torus_affine_closure::{check as independent, sha256};
use torus_coset_circuit::{index::IndexTable, Spec};
use torus_indexed_circuit::Augmented;
use torus_s4::{Point, Poly, System};
use torus_s4_tree::{Fp2, TorusParameters};
type Result<T> = std::result::Result<T, String>;
macro_rules! requests {($($id:literal),*)=>{pub const REQUESTS:&[(&str,&[u8])]=&[$(($id,include_bytes!(concat!("../fixtures/",$id,".json")))),*];}}
requests!(
    "toy-00-original",
    "toy-00-indexed",
    "toy-01-original",
    "toy-01-indexed",
    "toy-02-original",
    "toy-02-indexed",
    "toy-03-original",
    "toy-03-indexed",
    "toy-04-original",
    "toy-04-indexed",
    "toy-05-original",
    "toy-05-indexed",
    "toy-06-original",
    "toy-06-indexed",
    "toy-07-original",
    "toy-07-indexed",
    "toy-08-original",
    "toy-08-indexed",
    "toy-09-original",
    "toy-09-indexed",
    "toy-10-original",
    "toy-10-indexed",
    "toy-11-original",
    "toy-11-indexed",
    "toy-12-original",
    "toy-12-indexed",
    "actual-q1-original",
    "actual-q1-indexed"
);
pub fn compiled_sources() -> Value {
    json!({"Cargo.toml":sha256(include_bytes!("../Cargo.toml")),"Cargo.lock":sha256(include_bytes!("../Cargo.lock")),
    "src/lib.rs":sha256(include_bytes!("lib.rs")),"src/check.rs":sha256(include_bytes!("check.rs")),
    "src/controls.rs":sha256(include_bytes!("controls.rs")),"src/main.rs":sha256(include_bytes!("main.rs")),
    "unchanged_worklist_template":sha256(include_bytes!("../../torus-worklist-solver/src/lib.rs")),
    "requests":REQUESTS.iter().map(|(id,b)|(id.to_string(),json!(sha256(b)))).collect::<serde_json::Map<_,_>>()})
}
fn write(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    fs::write(
        path,
        serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
fn root(report: &Report, system: &System) -> Result<Option<Vec<BigUint>>> {
    if let Answer::Root(strings) = &report.answer {
        let values = strings
            .iter()
            .map(|s| independent::decimal(s))
            .collect::<Result<Vec<_>>>()?;
        independent::assignment(system, &values)?;
        system.check(&values)?;
        let mut bad = values.clone();
        bad[0] += &system.p;
        if independent::assignment(system, &bad).is_ok()
            || independent::assignment(system, &values[..values.len() - 1]).is_ok()
        {
            return Err("root shape/noncanonical refusal".into());
        }
        Ok(Some(values))
    } else if matches!(report.answer, Answer::Stopped(Stop::Invalid(_))) {
        Err(format!("unexpected Invalid: {:?}", report.answer))
    } else {
        Ok(None)
    }
}
fn one(out: &Path, id: &str, s: &System, roles: &[usize]) -> Result<(Report, Report, Value)> {
    eprintln!("fixed branch-affine case {id}");
    let dir = out.join(id);
    fs::create_dir(&dir).map_err(|e| e.to_string())?;
    write(
        &dir.join("public-input.json"),
        &json!({"system":s.canonical_json(),"roles":roles,"limits":Limits::default(),"dynamic_limits":DynamicLimits::default()}),
    )?;
    let old = torus_worklist_solver::solve(
        s,
        roles,
        torus_worklist_solver::Schedule::Worklist,
        torus_worklist_solver::Limits::default(),
    );
    let off = solve(
        s,
        roles,
        Schedule::Worklist,
        Limits::default(),
        Mode::Off,
        DynamicLimits::default(),
    );
    let ov = serde_json::to_value(&old).map_err(|e| e.to_string())?;
    let fv = serde_json::to_value(&off).map_err(|e| e.to_string())?;
    for field in ["answer", "counts", "decisions", "schedule"] {
        if ov[field] != fv[field] {
            return Err(format!("disabled fork differs in {field} for {id}"));
        }
    }
    write(&dir.join("old-v1.json"), &old)?;
    write(&dir.join("disabled.json"), &off)?;
    let on = solve(
        s,
        roles,
        Schedule::Worklist,
        Limits::default(),
        Mode::BranchAffine,
        DynamicLimits::default(),
    );
    write(&dir.join("enabled.json"), &on)?;
    check::verify(s, roles, &off)?;
    check::verify(s, roles, &on)?;
    root(&off, s)?;
    root(&on, s)?;
    let summary = json!({"id":id,"system_sha256":torus_affine_closure::system_hash(s),"disabled_equals_old_answer_decisions_counts":true,
        "before_answer":off.answer,"after_answer":on.answer,"before_counts":off.counts,"after_counts":on.counts,
        "dynamic_counts":on.dynamic_counts,"trace_events":on.events.len(),"conditional_coefficients_lifetimes_checked":true,
        "accounting_complete":false,"standing":"producer unsigned/unverified"});
    write(&dir.join("summary.json"), &summary)?;
    Ok((off, on, summary))
}
fn point(v: &Value) -> Result<Point> {
    if v.is_null() {
        Ok(None)
    } else {
        Ok(Some((
            independent::decimal(v[0].as_str().ok_or("point u")?)?,
            independent::decimal(v[1].as_str().ok_or("point v")?)?,
        )))
    }
}
fn curves(out: &Path) -> Result<Vec<Value>> {
    let targets: Value = serde_json::from_slice(torus_affine_closure::controls::PUBLIC_TARGETS)
        .map_err(|e| e.to_string())?;
    let old = TorusParameters::actual_four()?;
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
    let actual = Spec {
        p: old.f.p.clone(),
        h: old.h.pow(&213156431u32.into(), &old.f),
        order_factor: 60824497,
        q: 1,
        k: 2,
        curve_a: Some(486662u32.into()),
    };
    let mut results = vec![];
    for &(id, bytes) in REQUESTS {
        let (s, roles) = independent::parse_request(bytes)?;
        let dir = out.join(format!("sealed-{id}.json"));
        fs::write(&dir, bytes).map_err(|e| e.to_string())?;
        if fs::read(&dir).map_err(|e| e.to_string())? != bytes {
            return Err("sealed request readback".into());
        }
        let (spec, target) = if id.starts_with("toy-") {
            let at = id[4..6].parse::<usize>().map_err(|e| e.to_string())?;
            (&toy, point(&targets["toy_targets"][at])?)
        } else {
            (&actual, point(&targets["actual_q1_target"])?)
        };
        let (c, a) = Augmented::target(spec, 2, target, true)?;
        let indexed = id.ends_with("indexed");
        let expected = if indexed { &a.system } else { &c.system };
        if expected.canonical_json() != s.canonical_json() {
            return Err("fixed public circuit identity differs".into());
        }
        let (off, on, summary) = one(out, id, &s, &roles)?;
        let table = IndexTable::new(spec)?; // Replay-only, never a query argument.
        let mut replay = vec![];
        for (mode, report) in [("disabled", &off), ("enabled", &on)] {
            if matches!(report.answer, Answer::NoRootObservation) {
                return Err("known-consistent curve target declared contradiction".into());
            }
            let row = if let Some(w) = root(report, &s)? {
                let projected = if indexed { a.project_and_check(&w)? } else { w };
                Some(
                    c.decode_row(&projected, &table)?
                        .iter()
                        .map(|r| json!({"j":r.j,"t":r.t,"coefficient":r.coefficient}))
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            };
            replay.push(json!({"mode":mode,"row":row}));
        }
        write(&out.join(id).join("row-replay.json"), &replay)?;
        results.push(json!({"case":summary,"replay":replay}));
    }
    Ok(results)
}
fn system(p: &BigUint, n: usize, equations: Vec<Poly>) -> System {
    System {
        p: p.clone(),
        variables: (0..n).map(|i| format!("x{i}")).collect(),
        equations,
    }
}
fn small(out: &Path) -> Result<Value> {
    let p = 29u32.into();
    let x = |i| Poly::var(&p, i);
    let c = |n| Poly::small(&p, n);
    let bool_z = &(&x(0) * &x(0)) - &x(0);
    let one_minus_z = &c(1) - &x(0);
    let conditional = |contradiction| {
        system(
            &p,
            3,
            vec![
                bool_z.clone(),
                &one_minus_z * &(&(&x(1) + &x(2)) - &c(1)),
                if contradiction {
                    &one_minus_z * &(&(&x(1) + &x(2)) - &c(2))
                } else {
                    &one_minus_z * &(&(&x(1) - &x(2)) - &c(1))
                },
                &x(0) * &(&x(1) - &c(2)),
                &x(0) * &(&x(2) - &c(3)),
            ],
        )
    };
    let coupled = system(
        &p,
        2,
        vec![&(&x(0) + &x(1)) - &c(3), &(&x(0) - &x(1)) - &c(1)],
    );
    let rollback = conditional(true);
    let mut sample = None;
    let mut summaries = vec![];
    for (id, s, roles) in [
        ("coupled", coupled.clone(), vec![]),
        ("conditional-root", conditional(false), vec![0]),
        ("rollback", rollback.clone(), vec![0]),
        ("protected", coupled, vec![0]),
        (
            "nonlinear",
            system(&p, 2, vec![&(&x(0) * &x(1)) - &c(1)]),
            vec![],
        ),
    ] {
        let (before, after, summary) = one(out, id, &s, &roles)?;
        if id == "conditional-root" && root(&after, &s)?.is_none() {
            return Err("conditional root missing".into());
        }
        if id == "rollback"
            && (root(&after, &s)?.is_none()
                || after.counts.undos == 0
                || after.dynamic_counts.drops == 0)
        {
            return Err("rollback/drop control missing".into());
        }
        let mut accepted = 0;
        let mut truth = vec![];
        let mut activations = vec![];
        for event in &after.events {
            if let Event::Activate {
                premise,
                certificate,
                ..
            } = event
            {
                activations.push((premise, check::admit(&s, &roles, premise, certificate)?));
            }
        }
        let mut checked_extensions = 0;
        for mut k in 0..29u64.pow(s.variables.len() as u32) {
            let mut values = vec![];
            for _ in &s.variables {
                values.push(BigUint::from(k % 29));
                k /= 29;
            }
            let ok = independent::assignment(&s, &values).is_ok();
            accepted += usize::from(ok);
            truth.push(u8::from(ok));
            for (premise, output) in &activations {
                if premise.iter().all(|(v, t)| values[*v].to_string() == *t) {
                    checked_extensions += 1;
                    if independent::assignment(output, &values).is_ok() != ok {
                        return Err("conditional truth mismatch".into());
                    }
                }
            }
        }
        fs::write(out.join(id).join("truth.bin"), &truth).map_err(|e| e.to_string())?;
        summaries.push(json!({"case":summary,"truth_assignments":truth.len(),"original_roots":accepted,"conditional_assignments_checked":checked_extensions,"truth_sha256":sha256(&truth)}));
        if id == "rollback" {
            sample = Some(after);
        }
        let _ = before;
    }
    let good = sample.ok_or("rollback trace missing")?;
    let mut mutations = vec![];
    for kind in 0..8 {
        let mut bad = Report {
            answer: good.answer.clone(),
            counts: good.counts.clone(),
            decisions: good.decisions.clone(),
            schedule: good.schedule,
            mode: good.mode,
            dynamic_counts: good.dynamic_counts.clone(),
            events: good.events.clone(),
            accounting_complete: false,
        };
        match kind {
            0 => {
                let at = bad
                    .events
                    .iter()
                    .position(|e| matches!(e,Event::Activate{premise,..}if !premise.is_empty()))
                    .ok_or("premise activation")?;
                if let Event::Activate { premise, .. } = &mut bad.events[at] {
                    premise[0].1 = "1".into();
                }
            }
            1 => {
                let at=bad.events.iter().position(|e|matches!(e,Event::Activate{certificate,..}if !certificate.basis.is_empty())).ok_or("basis activation")?;
                if let Event::Activate { certificate, .. } = &mut bad.events[at] {
                    certificate.basis[0].origins[0].coefficient = "2".into();
                }
            }
            2 => {
                let undo = bad
                    .events
                    .iter()
                    .position(|e| matches!(e, Event::Undo { .. }))
                    .ok_or("undo")?;
                let drop = bad.events[..undo]
                    .iter()
                    .rposition(|e| matches!(e, Event::Drop { .. }))
                    .ok_or("drop")?;
                bad.events.remove(drop);
            }
            3 => {
                let at = bad
                    .events
                    .iter()
                    .position(|e| matches!(e, Event::Undo { .. }))
                    .ok_or("undo")?;
                bad.events[at] = Event::Undo { variable: 999 };
            }
            4 => {
                let at = bad
                    .events
                    .iter()
                    .position(|e| matches!(e, Event::Assign { .. }))
                    .ok_or("assign")?;
                bad.events.insert(at, bad.events[at].clone());
            }
            5 => {
                if let Some(Event::Activate { output_sha256, .. }) = bad
                    .events
                    .iter_mut()
                    .find(|e| matches!(e, Event::Activate { .. }))
                {
                    *output_sha256 = "0".repeat(64);
                }
            }
            6 => {
                if let Some(Event::Activate { certificate, .. }) = bad
                    .events
                    .iter_mut()
                    .find(|e| matches!(e, Event::Activate { .. }))
                {
                    certificate.original_system_sha256 = "0".repeat(64);
                }
            }
            _ => {
                if let Answer::Root(v) = &mut bad.answer {
                    v[0] = "2".into();
                }
            }
        }
        write(&out.join(format!("bad-trace-{kind}.json")), &bad)?;
        if check::verify(&rollback, &[0], &bad).is_ok() {
            return Err(format!("mutated trace {kind} accepted"));
        }
        mutations.push(json!({"kind":kind,"refused":true}));
    }
    let mut caps = vec![];
    for kind in 0..7 {
        let mut limits = Limits::default();
        let mut dynamic = DynamicLimits::default();
        match kind {
            0 => dynamic.passes = 0,
            1 => dynamic.trace_events = 0,
            2 => limits.nodes = 0,
            3 => limits.depth = 0,
            4 => limits.events = 1,
            5 => limits.modeled_reservations = 1,
            _ => limits.equations = 2,
        }
        let s = if kind == 6 {
            system(
                &p,
                2,
                vec![&(&x(0) + &x(1)) - &c(3), &(&x(0) - &x(1)) - &c(1)],
            )
        } else {
            conditional(false)
        };
        let r = solve(
            &s,
            &[0],
            Schedule::Worklist,
            limits,
            Mode::BranchAffine,
            dynamic,
        );
        if !matches!(r.answer, Answer::Stopped(Stop::Cap(_))) {
            return Err(format!("forced cap {kind} not observed: {:?}", r.answer));
        }
        check::verify(&s, &[0], &r)?;
        write(&out.join(format!("cap-{kind}.json")), &r)?;
        caps.push(json!({"kind":kind,"answer":r.answer}));
    }
    let mut malformed = vec![];
    for kind in 0..4 {
        let mut v: Value = serde_json::from_slice(REQUESTS[0].1).map_err(|e| e.to_string())?;
        match kind {
            0 => v["witness"] = json!([1]),
            1 => v["protected"] = json!([0, 0]),
            2 => v["system"]["p"] = "28".into(),
            _ => v["system"]["equations"][0][0]["coefficient"] = "01".into(),
        }
        let bytes = serde_json::to_vec_pretty(&v).map_err(|e| e.to_string())?;
        fs::write(out.join(format!("malformed-{kind}.json")), &bytes).map_err(|e| e.to_string())?;
        if independent::parse_request(&bytes).is_ok() {
            return Err("malformed request accepted".into());
        }
        malformed.push(json!({"kind":kind,"refused":true}));
    }
    Ok(
        json!({"systems":summaries,"trace_mutations":mutations,"forced_caps":caps,"malformed":malformed}),
    )
}
pub fn run(out: &Path) -> Result<Value> {
    let small = small(out)?;
    let curves = curves(out)?;
    Ok(
        json!({"schema":"torus.branch-affine-controls/v1","standing":"producer unsigned/unverified",
        "small":small,"curves":curves,"compiled_sources":compiled_sources(),"accounting":"partial diagnostic/reservation counters; coefficient checker internals/cold cost/live allocation unpriced"}),
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn fixed_controls() {
        let p = std::env::temp_dir().join(format!(
            "curve25519-branch-affine-unit-{}",
            std::process::id()
        ));
        std::fs::create_dir(&p).unwrap();
        match super::run(&p) {
            Ok(r) => std::fs::write(
                p.join("controls.json"),
                serde_json::to_vec_pretty(&r).unwrap(),
            )
            .unwrap(),
            Err(e) => {
                std::fs::write(p.join("failure.txt"), &e).unwrap();
                panic!("{e}; retained {}", p.display());
            }
        }
    }
}
