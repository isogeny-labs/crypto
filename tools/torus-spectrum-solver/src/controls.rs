//! Synthetic complete-model checks are explicitly separate from solver discovery.
use crate::{
    backend::{self, Limits},
    check_model, inputs, model, Result,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{io::Write, path::Path};
use torus_quotient_query::controls::spec;
use torus_s4::{Curve, Field, Point};
use torus_spectrum_compiler::ir::E;
fn number(v: &Value) -> Result<BigUint> {
    BigUint::parse_bytes(v.as_str().ok_or("bad fixture number")?.as_bytes(), 10)
        .ok_or("bad decimal".into())
}
fn tuple(v: &Value) -> Result<Vec<BigUint>> {
    v.as_array()
        .ok_or("bad tuple")?
        .iter()
        .map(number)
        .collect()
}
fn point(v: &Value) -> Result<Point> {
    if v.is_null() {
        Ok(None)
    } else {
        let xs = tuple(v)?;
        if xs.len() != 2 {
            return Err("bad fixture point".into());
        }
        Ok(Some((xs[0].clone(), xs[1].clone())))
    }
}
pub fn synthetic(out: &mut impl Write) -> Result<()> {
    let fixtures = torus_spectrum_compiler::controls::fixtures()?;
    let mut n = 0;
    for fixture in fixtures["cases"].as_array().ok_or("no fixtures")? {
        let name = fixture["case"].as_str().ok_or("bad case")?;
        let actual = name.starts_with("original-actual");
        let k = if name.ends_with("k20633") {
            20633
        } else if actual {
            31
        } else {
            8
        };
        let s = spec(actual, k)?;
        let r = point(&fixture["value"]["R"])?;
        let curve = Curve {
            f: Field::new(s.p.clone())?,
            a: s.curve_a.clone().ok_or("no A")?,
        };
        let target = curve.times(&r, 8)?;
        let c = torus_spectrum_compiler::compile(&s, r, target, 200_000)?;
        let cs = tuple(&fixture["value"]["coefficients"])?;
        let labels = fixture["value"]["labels"]
            .as_array()
            .ok_or("bad labels")?
            .iter()
            .map(|v| v[0].as_u64().ok_or("bad j".into()))
            .collect::<Result<Vec<_>>>()?;
        let values = c.extend(&cs, &labels)?;
        let expected = inputs(&c.program)?;
        let pairs: Vec<_> = expected
            .iter()
            .rev()
            .map(|(w, _)| format!("(w{w} #f{}m{})", values[*w], s.p))
            .collect();
        let text = format!("({})", pairs.join("\n"));
        let checked = check_model(&c, &text)?;
        let mut semantic = model::parse(&text, &s.p, &expected)?;
        semantic[5] = 0u32.into();
        let bad = format!(
            "({})",
            expected
                .iter()
                .map(|(w, i)| format!("(w{w} #f{}m{})", semantic[*i], s.p))
                .collect::<Vec<_>>()
                .join(" ")
        );
        if check_model(&c, &bad).is_ok() {
            return Err("zero-inverse full model accepted".into());
        }
        writeln!(out,"{}",json!({"case":name,"kind":"synthetic_model_replay","model_text":text,"checked":checked,"zero_guard_model_rejected":true,"scope":"supplied synthetic model, not backend root discovery"})).map_err(|e|e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
        n += 1;
    }
    writeln!(out, "{}", json!({"kind":"synthetic_complete","cases":n})).map_err(|e| e.to_string())
}
pub fn backend_controls(exe: &Path, out: &mut impl Write) -> Result<()> {
    for inconsistent in [false, true] {
        let e = E::new(101u32.into(), 100)?;
        let x = e.input();
        let y = e.input();
        e.equal(e.mul(x, x), e.c(9));
        e.equal(e.add(x, y), e.c(7));
        if inconsistent {
            e.equal(x, e.c(4));
        }
        let p = e.finish()?;
        let run = backend::run(exe, &p, &Limits::default())?;
        if run.refusal.is_some() {
            return Err(format!("small backend control refusal: {:?}", run.refusal));
        }
        if inconsistent {
            if run.status.as_deref() != Some("unsat") || run.get_value_sent {
                return Err("inconsistent control status/protocol".into());
            }
        } else {
            if run.status.as_deref() != Some("sat") || !run.get_value_sent {
                return Err("small SAT status/protocol".into());
            }
            let semantic = model::parse(&run.model, &p.f.p, &inputs(&p)?)?;
            let v = p.evaluate(&semantic)?;
            p.check(&v)?;
        }
        writeln!(out,"{}",json!({"kind":"two_phase_backend_control","inconsistent":inconsistent,"backend":run.json()})).map_err(|e|e.to_string())?;
    }
    // Deterministic output-cap refusal using the same trusted binary, not a fake solver.
    let e = E::new(101u32.into(), 100)?;
    for _ in 0..10 {
        let x = e.input();
        e.equal(x, e.c(3));
    }
    let p = e.finish()?;
    let limits = Limits {
        stdout_bytes: 64,
        ..Limits::default()
    };
    let run = backend::run(exe, &p, &limits)?;
    if !run.stdout_truncated || run.refusal.is_none() {
        return Err("backend output cap not enforced".into());
    }
    writeln!(
        out,
        "{}",
        json!({"kind":"backend_output_cap_control","backend":run.json()})
    )
    .map_err(|e| e.to_string())
}
/// Exactly one fixed known-consistent public-target integration, no retry/cohort.
pub fn curve_control(exe: &Path, out: &mut impl Write) -> Result<()> {
    let s = spec(false, 8)?;
    let r = Some((89u32.into(), 27u32.into()));
    let curve = Curve {
        f: Field::new(s.p.clone())?,
        a: s.curve_a.clone().ok_or("no A")?,
    };
    let target = curve.times(&r, 8)?;
    // No fixture, coefficients, labels, signs or witnesses enter solve.
    let outcome = crate::solve(&s, r, target, exe, &Limits::default())?;
    if outcome["kind"] == "invalid_model"
        || outcome["kind"] == "no_root_observation" && outcome["backend"]["status"] == "unsat"
    {
        return Err(format!(
            "known-consistent curve control rejected: {outcome}"
        ));
    }
    writeln!(out,"{}",json!({"kind":"fixed_public_curve_integration","outcome":outcome,"scope":"one known-consistent development target; cap/unknown is incomplete, not hardness or absence"})).map_err(|e|e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use torus_spectrum_compiler::ir::Gate;
    #[test]
    fn supplied_model_grammar_and_original_projection() -> Result<()> {
        synthetic(&mut std::io::sink())
    }
    #[test]
    fn unknowns_are_semantic_inputs_only() -> Result<()> {
        let e = E::new(101u32.into(), 100)?;
        let x = e.input();
        e.equal(e.mul(x, x), e.c(9));
        let p = e.finish()?;
        let layout = inputs(&p)?;
        assert_eq!(layout.len(), 1);
        assert!(matches!(p.gates[layout[0].0], Gate::Input(0)));
        Ok(())
    }
}
