//! Supplied extension controls remain separate from the public unknown query.
use crate::{
    check_canonical, compile, export, export_asserted, model, process, PointQuery, Result,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{
    io::{Read, Write},
    path::Path,
};
use torus_quotient_query::controls::spec;
use torus_s4::{Curve, Field, Point, Poly, System};
fn number(v: &Value) -> Result<BigUint> {
    v.as_str()
        .ok_or("bad integer")?
        .parse()
        .map_err(|_| "bad decimal".into())
}
fn point(v: &Value) -> Result<Point> {
    if v.is_null() {
        Ok(None)
    } else {
        let a = v.as_array().ok_or("bad point")?;
        if a.len() != 2 {
            return Err("bad point length".into());
        }
        Ok(Some((number(&a[0])?, number(&a[1])?)))
    }
}
fn fixtures() -> Result<Vec<Value>> {
    let mut raw = String::new();
    flate2::read::GzDecoder::new(&include_bytes!("../FIXTURES.jsonl.gz")[..])
        .take(1 << 20)
        .read_to_string(&mut raw)
        .map_err(|e| e.to_string())?;
    raw.lines()
        .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
        .collect()
}
fn load(v: &Value) -> Result<(PointQuery, Vec<BigUint>)> {
    let name = v["case"].as_str().ok_or("no case")?;
    let actual = name.starts_with("original-actual");
    let k = if name.ends_with("k20633") {
        20633
    } else if actual {
        31
    } else {
        8
    };
    let s = spec(actual, k)?;
    let old = &v["checked"]["original"];
    let r = point(&old["public_R"])?;
    let target = point(&old["public_Q"])?;
    let c = compile(&s, r, target)?;
    let values = old["original_witness"]
        .as_array()
        .ok_or("no witness")?
        .iter()
        .map(number)
        .collect::<Result<Vec<_>>>()?;
    Ok((c, values))
}
fn text(p: &BigUint, v: &[BigUint]) -> String {
    format!(
        "({})",
        v.iter()
            .enumerate()
            .rev()
            .map(|(i, x)| format!("(w{i} #f{x}m{p})"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
pub fn synthetic(out: &mut impl Write) -> Result<()> {
    let mut n = 0;
    for record in fixtures()? {
        if record.get("case").is_none() {
            continue;
        }
        let (c, values) = load(&record)?;
        let model = text(&c.spec.p, &values);
        let checked = c.check_model(&model)?;
        let pure = export(&c.circuit.system)?;
        if pure.script.matches("(declare-const ").count() != c.circuit.system.variables.len()
            || pure.script.matches("(assert ").count() != c.circuit.system.equations.len()
        {
            return Err("SMT original row/variable count mismatch".into());
        }
        let mut mutations = 0;
        for i in 0..values.len() {
            if i < 16 || i % 31 == 0 || i + 1 == values.len() {
                let mut bad = values.clone();
                bad[i] = (&bad[i] + 1u32) % &c.spec.p;
                if c.check_values(&bad).is_ok() {
                    return Err(format!("one-wire mutation accepted {i}"));
                }
                mutations += 1;
            }
        }
        let mut bad = values.clone();
        bad[0] += &c.spec.p;
        if c.check_values(&bad).is_ok() || c.check_values(&values[..values.len() - 1]).is_ok() {
            return Err("canonical/length boundary accepted".into());
        }
        writeln!(out,"{}",json!({"case":record["case"],"kind":"synthetic_original_point_replay","model_text":model,"checked":checked,"original_stats":c.circuit.stats(),"declared_variables":pure.variables,"original_equations_exported":c.circuit.system.equations.len(),"wire_mutations_rejected":mutations,"canonical_and_length_refused":true,"scope":"supplied complete model; not unknown-root discovery"})).map_err(|e|e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
        n += 1;
    }
    writeln!(out, "{}", json!({"kind":"synthetic_complete","cases":n})).map_err(|e| e.to_string())
}
pub fn backend_controls(exe: &Path, out: &mut impl Write) -> Result<()> {
    for inconsistent in [false, true] {
        let p = BigUint::from(101u32);
        let x = Poly::var(&p, 0);
        let y = Poly::var(&p, 1);
        let mut system = System {
            p: p.clone(),
            variables: vec!["x".into(), "y".into()],
            equations: vec![
                &(&x * &x) - &Poly::small(&p, 9),
                &(&x + &y) - &Poly::small(&p, 7),
            ],
        };
        if inconsistent {
            system.equations.push(&x - &Poly::small(&p, 4));
        }
        let q = export(&system)?;
        let run = process::run(exe, &q, &process::Limits::default())?;
        if run.refusal.is_some() {
            return Err(format!("small system refusal: {:?}", run.refusal));
        }
        if inconsistent {
            if run.status.as_deref() != Some("unsat") || run.get_value_sent {
                return Err("UNSAT protocol mismatch".into());
            }
        } else {
            if run.status.as_deref() != Some("sat") {
                return Err("SAT protocol mismatch".into());
            }
            let v = model::parse(&run.model, &p, &[(0, 0), (1, 1)])?;
            check_canonical(&system, &v)?;
            system.check(&v)?;
        }
        writeln!(out,"{}",json!({"kind":"native_small_sparse_control","inconsistent":inconsistent,"backend":run.json()})).map_err(|e|e.to_string())?;
    }
    for record in fixtures()? {
        let name = record["case"].as_str().unwrap_or("");
        if !matches!(name, "toy-signed-support-1" | "original-actual-k20633") {
            continue;
        }
        let (c, known) = load(&record)?;
        let q = export_asserted(&c.circuit.system, &known)?;
        let run = process::run(exe, &q, &process::Limits::default())?;
        if run.refusal.is_some() || run.status.as_deref() != Some("sat") {
            return Err(format!(
                "asserted original export refusal for {name}: {}",
                run.json()
            ));
        }
        let checked = c.check_model(&run.model)?;
        writeln!(out,"{}",json!({"kind":"asserted_original_export_control","case":name,"backend":run.json(),"checked":checked,"scope":"all original values asserted; export extension only, not discovery"})).map_err(|e|e.to_string())?;
    }
    Ok(())
}
pub fn public_control(exe: &Path, out: &mut impl Write) -> Result<()> {
    let s = spec(false, 8)?;
    let r = Some((89u32.into(), 27u32.into()));
    let curve = Curve {
        f: Field::new(s.p.clone())?,
        a: s.curve_a.clone().ok_or("no A")?,
    };
    let target = curve.times(&r, 8)?;
    let outcome = crate::solve(&s, r, target, exe, &process::Limits::default())?;
    if outcome["kind"] == "model_or_projection_refusal" || outcome["backend"]["status"] == "unsat" {
        return Err(format!("known-consistent point target rejected: {outcome}"));
    }
    writeln!(out,"{}",json!({"kind":"single_public_point_query","outcome":outcome,"scope":"fixed known-consistent development target; no timing/rate or exponent comparison"})).map_err(|e|e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate_system;
    #[test]
    fn original_model_projection() -> Result<()> {
        synthetic(&mut std::io::sink())
    }
    #[test]
    fn malformed_sparse_system_and_noncanonical_refuse() -> Result<()> {
        let p = 101u32.into();
        let mut s = System {
            p,
            variables: vec!["x".into()],
            equations: vec![],
        };
        s.equations.push(Poly::var(&s.p, 1));
        assert!(validate_system(&s).is_err());
        s.equations.clear();
        s.equations.push(Poly::var(&s.p, 0));
        assert!(check_canonical(&s, &[101u32.into()]).is_err());
        s.equations[0].terms.insert(vec![], 101u32.into());
        assert!(export(&s).is_err());
        Ok(())
    }
}
