//! Separate actual-engine rollback control, outside the frozen core140 context.
use num_bigint::BigUint;
use serde_json::json;
use std::{env, fs, path::Path};
use torus_affine_closure::{check as original, sha256};
use torus_monomial_cancellation::{
    self as solver, Answer, DynamicLimits, Event, Limits, Mode, Schedule,
};
use torus_s4::{Poly, System};
fn run(out: &Path) -> Result<(), String> {
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let save = |name: &str, value: &serde_json::Value| {
        fs::write(
            out.join(name),
            serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    };
    save(
        "pre-call.json",
        &json!({"schema":"torus.monomial-engine-rollback.precall/v1",
        "standing":"producer unsigned/unverified","scope":"one-system native correctness contract; not core35 or timing",
        "source_sha256":sha256(include_bytes!("main.rs")),"cargo_toml_sha256":sha256(include_bytes!("../Cargo.toml")),
        "cargo_lock_sha256":sha256(include_bytes!("../Cargo.lock")),
        "binary_sha256":sha256(&fs::read(env::current_exe().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?)}),
    )?;
    let p = BigUint::from(29u32);
    let x = |v| Poly::var(&p, v);
    let c = |n| Poly::small(&p, n);
    let s = System {
        p: p.clone(),
        variables: vec!["z".into(), "u".into(), "v".into()],
        equations: vec![
            &(&x(0) * &x(0)) - &x(0),
            &(&(&x(0) * &x(1)) + &(&x(1) + &x(2))) - &c(1),
            &(&(&x(0) * &x(2)) + &(&x(1) + &x(2))) - &c(2),
        ],
    };
    save(
        "public-input.json",
        &json!({"system":s.canonical_json(),"roles":[0],"limits":Limits::default(),"dynamic_limits":DynamicLimits::default()}),
    )?;
    let off = solver::solve(
        &s,
        &[0],
        Schedule::Worklist,
        Limits::default(),
        Mode::Off,
        DynamicLimits::default(),
    );
    save(
        "disabled.json",
        &serde_json::to_value(&off).map_err(|e| e.to_string())?,
    )?;
    let on = solver::solve(
        &s,
        &[0],
        Schedule::Worklist,
        Limits::default(),
        Mode::MonomialCancellation,
        DynamicLimits::default(),
    );
    save(
        "enabled.json",
        &serde_json::to_value(&on).map_err(|e| e.to_string())?,
    )?;
    solver::check::verify(&s, &off)?;
    solver::check::verify(&s, &on)?;
    let Answer::Root(strings) = &on.answer else {
        return Err("runtime contract root missing".into());
    };
    let values = strings
        .iter()
        .map(|s| original::decimal(s))
        .collect::<Result<Vec<_>, _>>()?;
    original::assignment(&s, &values)?;
    if on.counts.undos == 0 || on.dynamic_counts.drops == 0 || on.dynamic_counts.activations == 0 {
        return Err("actual engine rollback/drop path missing".into());
    }
    let undo = on
        .events
        .iter()
        .position(|e| matches!(e, Event::Undo { .. }))
        .ok_or("actual Undo absent")?;
    let drop = on.events[..undo]
        .iter()
        .rposition(|e| matches!(e, Event::Drop { .. }))
        .ok_or("actual Drop absent")?;
    let mut bad = on.clone();
    bad.events.remove(drop);
    save(
        "bad-missing-drop.json",
        &serde_json::to_value(&bad).map_err(|e| e.to_string())?,
    )?;
    if solver::check::verify(&s, &bad).is_ok() {
        return Err("missing Drop accepted".into());
    }
    let mut bad = on.clone();
    bad.events[undo] = Event::Undo { variable: 999 };
    save(
        "bad-undo.json",
        &serde_json::to_value(&bad).map_err(|e| e.to_string())?,
    )?;
    if solver::check::verify(&s, &bad).is_ok() {
        return Err("wrong Undo accepted".into());
    }
    let mut truth = Vec::new();
    let mut roots = Vec::new();
    for mut k in 0..29u64.pow(3) {
        let mut v = Vec::new();
        for _ in 0..3 {
            v.push(BigUint::from(k % 29));
            k /= 29;
        }
        let valid = original::assignment(&s, &v).is_ok();
        truth.push(u8::from(valid));
        if valid {
            roots.push(v.iter().map(ToString::to_string).collect::<Vec<_>>());
        }
    }
    if !roots.contains(strings) {
        return Err("root differs from exhaustive original truth set".into());
    }
    fs::write(out.join("truth.bin"), &truth).map_err(|e| e.to_string())?;
    save(
        "results.json",
        &json!({"schema":"torus.monomial-engine-rollback.results/v1","standing":"producer unsigned/unverified",
        "scope":"separate actual-engine correctness contract; not core35, curve or performance result",
        "before_answer":off.answer,"before_nodes":off.counts.nodes,"after_answer":on.answer,"after_nodes":on.counts.nodes,
        "actual_undos":on.counts.undos,"actual_activations":on.dynamic_counts.activations,"actual_drops":on.dynamic_counts.drops,
        "trace_reader_accepts":true,"bad_missing_drop_refused":true,"bad_undo_refused":true,
        "truth_assignments":truth.len(),"truth_sha256":sha256(&truth),"all_original_roots":roots,
        "accounting_complete":false}),
    )?;
    Ok(())
}
fn main() {
    let Some(arg) = env::args().nth(1) else {
        eprintln!("expected NEW output directory");
        std::process::exit(2)
    };
    let out = Path::new(&arg);
    if let Err(e) = run(out) {
        if out.is_dir() {
            let _ = fs::write(out.join("failure.txt"), &e);
        }
        eprintln!("{e}");
        std::process::exit(1)
    }
}
