//! Public-input native model adapter. Only checked roots become relations.
pub mod backend;
pub mod controls;
pub mod model;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use torus_coset_circuit::{index::IndexTable, Spec};
use torus_quotient_query::{arithmetic::Engine, query::Query};
use torus_s4::Point;
use torus_spectrum_compiler::{
    ir::{Gate, Program},
    Compiled,
};
pub type Result<T> = std::result::Result<T, String>;
pub fn inputs(p: &Program) -> Result<Vec<(usize, usize)>> {
    p.validate()?;
    let xs: Vec<_> = p
        .gates
        .iter()
        .enumerate()
        .filter_map(|(w, g)| {
            if let Gate::Input(i) = g {
                Some((w, *i))
            } else {
                None
            }
        })
        .collect();
    if xs.len() != p.inputs {
        return Err("semantic layout mismatch".into());
    }
    Ok(xs)
}
/// Data supplied here is a candidate backend response, not an extension oracle.
pub fn check_model(c: &Compiled, text: &str) -> Result<Value> {
    let semantic =
        model::parse(text, &c.spec.p, &inputs(&c.program)?).map_err(|e| format!("model: {e}"))?;
    let values = c
        .program
        .evaluate(&semantic)
        .map_err(|e| format!("model: {e}"))?;
    c.program
        .check(&values)
        .map_err(|e| format!("model: {e}"))?;
    let mut cs = vec![BigUint::zero(); 6];
    cs[if c.r.is_some() { 4 } else { 5 }] = BigUint::one();
    for (i, slot) in c.coefficient_slots.iter().enumerate() {
        cs[*slot] = semantic[i].clone();
    }
    let labels = c
        .labels
        .iter()
        .map(|w| values[*w].to_u64().ok_or("label does not fit u64".into()))
        .collect::<Result<Vec<_>>>()?;
    if labels.iter().any(|j| *j == 0 || *j > c.spec.k) {
        return Err("noncanonical recovered label".into());
    }
    let e = Engine::new(c.spec.p.clone(), 100_000_000)?;
    let q = Query::new(&e, &c.spec, c.r.clone(), c.target.clone(), cs.clone())?;
    let predicate = torus_section_radical::predicate(&q)?;
    if predicate["accept"] != true {
        return Err("original support predicate rejected model".into());
    }
    let table = IndexTable::new(&c.spec)?;
    let original = q.extract(&table)?;
    if original["original_replay_checked"] != true {
        return Err("missing original indexed/group replay".into());
    }
    Ok(
        json!({"kind":"checked_root","semantic_inputs":semantic.iter().map(ToString::to_string).collect::<Vec<_>>(),"coefficients":cs.iter().map(ToString::to_string).collect::<Vec<_>>(),"labels":labels,"full_gate_check":true,"original":original,"scope":"valid original indexed relation; no target scalar recovery or complexity inference"}),
    )
}
/// Query receives only public data and limits. No known unknowns are arguments.
pub fn solve(
    spec: &Spec,
    r: Point,
    target: Point,
    exe: &std::path::Path,
    limits: &backend::Limits,
) -> Result<Value> {
    limits.validate()?;
    let c = torus_spectrum_compiler::compile(spec, r, target, limits.max_nodes)?;
    let run = backend::run(exe, &c.program, limits)?;
    let mut result = match run.status.as_deref() {
        Some("sat") if run.refusal.is_none() => match check_model(&c, &run.model) {
            Ok(root) => root,
            Err(reason) => {
                json!({"kind":if reason.starts_with("model:"){ "invalid_model" }else{ "projection_refusal" },"reason":reason,"scope":"one candidate only; no target absence or solver-hardness inference"})
            }
        },
        Some("unsat" | "unknown") if run.refusal.is_none() => {
            json!({"kind":"no_root_observation","solver_status":run.status,"scope":"unverified backend observation, not target absence or a route disproof"})
        }
        _ => json!({"kind":"resource_or_backend_refusal","reason":run.refusal}),
    };
    result["backend"] = run.json();
    result["program"] = c.program.stats();
    Ok(result)
}
