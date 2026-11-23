//! Fixed supplied witnesses only; no root search or backend invocation.
use crate::{sparse::Limits, Block, Result};
use num_bigint::BigUint;
use num_traits::Zero;
use serde_json::{json, Value};
use std::io::{Read, Write};
use torus_quotient_query::controls::spec;
use torus_s4::Point;
use torus_section_operator::{IndexedPoint, Operator};
fn number(v: &Value) -> Result<BigUint> {
    v.as_str()
        .ok_or("expected decimal string")?
        .parse()
        .map_err(|_| "bad decimal".into())
}
fn point(v: &Value) -> Result<Point> {
    if v.is_null() {
        return Ok(None);
    }
    let a = v.as_array().ok_or("point array")?;
    if a.len() != 2 {
        return Err("point length".into());
    }
    Ok(Some((number(&a[0])?, number(&a[1])?)))
}
fn fixtures() -> Result<Vec<Value>> {
    let mut text = String::new();
    flate2::read::GzDecoder::new(&include_bytes!("../../torus-point-solver/FIXTURES.jsonl.gz")[..])
        .take(1 << 20)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    text.lines()
        .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
        .collect()
}
pub fn run(out: &mut impl Write) -> Result<()> {
    let (mut cases, mut admitted, mut excluded, mut pivots, mut zeros) = (0, 0, 0, 0, 0);
    for record in fixtures()? {
        let Some(name) = record["case"].as_str() else {
            continue;
        };
        cases += 1;
        let actual = name.starts_with("original-actual");
        let k = if name.ends_with("k20633") {
            20633
        } else if actual {
            31
        } else {
            8
        };
        let s = spec(actual, k)?;
        let old = &record["checked"]["original"];
        let r = point(&old["public_R"])?;
        let q = point(&old["public_Q"])?;
        let values = old["original_witness"]
            .as_array()
            .ok_or("original witness")?
            .iter()
            .map(number)
            .collect::<Result<Vec<_>>>()?;
        let cs = old["coefficients"]
            .as_array()
            .ok_or("original coefficients")?
            .iter()
            .map(number)
            .collect::<Result<Vec<_>>>()?;
        let block = Block::compile(&s, r.clone(), q, Limits::default())?;
        let augmented = match block.extend(&values, &cs) {
            Ok(v) => v,
            Err(e) if e.contains("UNSUPPORTED regular-chart boundary") => {
                excluded += 1;
                writeln!(out,"{}",json!({"case":name,"kind":"excluded_regular_chart","reason":e,"scope":"not absence or whole-route failure"})).map_err(|e|e.to_string())?;
                continue;
            }
            Err(e) => return Err(format!("{name}: {e}")),
        };
        admitted += 1;
        let op = Operator::new(s.p.clone(), s.curve_a.clone().ok_or("A")?)?;
        let points = block.raw_points(&values)?;
        let indexed = points
            .iter()
            .enumerate()
            .map(|(i, p)| IndexedPoint {
                column: i as u64,
                point: p.clone(),
            })
            .collect::<Vec<_>>();
        let scale = op
            .curve
            .f
            .inv(cs.iter().find(|c| !c.is_zero()).ok_or("zero section")?)?;
        let normalized = cs
            .iter()
            .map(|c| op.curve.f.mul(c, &scale))
            .collect::<Vec<_>>();
        op.verify_section(&indexed, &r, &normalized)?;
        let original_replay = block.check_augmented(&augmented)?;
        let mut results = vec![];
        for omit in 0..6 {
            let chart = block.chart(omit)?;
            let lifted = match block.lift(&chart, &augmented) {
                Ok(v) => v,
                Err(e) if e.contains("zero pivot minor") => {
                    zeros += 1;
                    results.push(json!({"omit":omit,"zero_minor_refusal":e,"fallback_augmented_retained":true,"selected_row_identities_checked":5,"system":chart.system.encoded(),"stats":chart.system.stats(),"partial_expansion_pairs":chart.expansion_pairs}));
                    continue;
                }
                Err(e) => return Err(format!("{name}/chart{omit}: {e}")),
            };
            let projected = block.project(&chart, &lifted)?;
            if projected != augmented || block.coefficients(&projected)? != cs {
                return Err("lift/projection disagreement".into());
            }
            let mut bad = lifted.clone();
            let end = bad.len() - 1;
            bad[end] = (&bad[end] + 1u32) % &s.p;
            if block.project(&chart, &bad).is_ok() {
                return Err("pivot mutation accepted".into());
            }
            let mut guard = lifted.clone();
            guard[block.original_count()] = (&guard[block.original_count()] + 1u32) % &s.p;
            if block.project(&chart, &guard).is_ok() {
                return Err("regular inverse mutation accepted".into());
            }
            let mut original = lifted.clone();
            original[0] = (&original[0] + 1u32) % &s.p;
            if block.project(&chart, &original).is_ok() {
                return Err("original mutation accepted".into());
            }
            let mut noncanonical = lifted.clone();
            noncanonical[0] += &s.p;
            if block.project(&chart, &noncanonical).is_ok()
                || block.project(&chart, &lifted[..lifted.len() - 1]).is_ok()
            {
                return Err("canonical/length boundary accepted".into());
            }
            pivots += 1;
            results.push(json!({"omit":omit,"lift_project_equal":true,"coefficient_reconstruction_equal":true,"selected_row_identities_checked":5,"pivot_regular_original_mutations_rejected":true,"canonical_length_refused":true,"stats":chart.system.stats(),"partial_expansion_pairs":chart.expansion_pairs,"system":chart.system.encoded(),"witness":lifted.iter().map(ToString::to_string).collect::<Vec<_>>()}));
        }
        let mut wrong = augmented.clone();
        let last = wrong.len() - 1;
        wrong[last] = (&wrong[last] + 1u32) % &s.p;
        if block.check_augmented(&wrong).is_ok() {
            return Err("coefficient mutation accepted".into());
        }
        writeln!(out,"{}",json!({"case":name,"kind":"regular_section_elimination_control","public_R":r.as_ref().map(|(x,y)|[x.to_string(),y.to_string()]),"augmented_system":block.augmented.encoded(),"augmented_stats":block.augmented.stats(),"augmented_witness":augmented.iter().map(ToString::to_string).collect::<Vec<_>>(),"coefficients":cs.iter().map(ToString::to_string).collect::<Vec<_>>(),"numeric_operator_replay":true,"original_and_raw_R_replay":original_replay,"coefficient_mutation_refused":true,"charts":results,"scope":"supplied witnesses; no root discovery or solver work bound; restricted regular distinct-x/raw-R chart"})).map_err(|e|e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
    }
    writeln!(out,"{}",json!({"kind":"complete","cases":cases,"admitted":admitted,"excluded_chart":excluded,"nonzero_pivot_projections":pivots,"zero_pivot_refusals":zeros,"backend_invocations":0,"new_roots_discovered":0,"full_cost_accounting":false})).map_err(|e|e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::One;
    #[test]
    fn original_and_guard_rows_are_retained() -> Result<()> {
        let s = spec(false, 8)?;
        let r = Some((89u32.into(), 27u32.into()));
        let q = Some((70u32.into(), 41u32.into()));
        let b = Block::compile(&s, r.clone(), q.clone(), Limits::default())?;
        assert_eq!(b.original_count(), 327);
        assert_eq!(b.augmented.variables.len(), 353);
        assert_eq!(b.augmented.equations.len(), 384);
        assert!(b.chart(6).is_err());
        let chart = b.chart(0)?;
        assert_eq!(chart.system.variables.len(), 349);
        assert_eq!(chart.system.equations.len(), 380);
        assert!(Block::compile(&s, None, None, Limits::default()).is_err());
        assert!(Block::compile(
            &s,
            r,
            q,
            Limits {
                terms: 1,
                pairs: 20_000_000,
                degree: 12
            }
        )
        .is_err());
        Ok(())
    }
    #[test]
    fn normalized_known_model_lifts() -> Result<()> {
        let record = fixtures()?
            .into_iter()
            .find(|v| v["case"].as_str() == Some("toy-signed-support-1"))
            .ok_or("fixture")?;
        let old = &record["checked"]["original"];
        let s = spec(false, 8)?;
        let b = Block::compile(
            &s,
            point(&old["public_R"])?,
            point(&old["public_Q"])?,
            Limits::default(),
        )?;
        let v = old["original_witness"]
            .as_array()
            .ok_or("witness")?
            .iter()
            .map(number)
            .collect::<Result<Vec<_>>>()?;
        let cs = old["coefficients"]
            .as_array()
            .ok_or("coefficients")?
            .iter()
            .map(number)
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(cs[4], BigUint::one());
        let a = b.extend(&v, &cs)?;
        let mut found = 0;
        for i in 0..6 {
            let c = b.chart(i)?;
            if let Ok(l) = b.lift(&c, &a) {
                assert_eq!(b.project(&c, &l)?, a);
                found += 1;
            }
        }
        assert!(found > 0);
        let other = Block::compile(
            &spec(false, 7)?,
            point(&old["public_R"])?,
            point(&old["public_Q"])?,
            Limits::default(),
        )?;
        assert!(b
            .lift(&other.chart(0)?, &a)
            .unwrap_err()
            .contains("different public problem"));
        let mut bad = cs;
        bad[4] = 0u32.into();
        assert!(b.extend(&v, &bad).is_err());
        Ok(())
    }
}
