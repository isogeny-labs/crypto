//! Fixed existing known-answer controls; no search, timing or scaling campaign.
use crate::{compile, ir::Gate, Compiled, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};
use torus_coset_circuit::index::IndexTable;
use torus_quotient_query::{arithmetic::Engine, controls::spec, query::Query};
use torus_s4::{Curve, Point};
pub fn fixtures() -> Result<Value> {
    serde_json::from_str(include_str!("../FIXTURES.json")).map_err(|e| e.to_string())
}
fn number(v: &Value) -> Result<BigUint> {
    BigUint::parse_bytes(v.as_str().ok_or("bad integer")?.as_bytes(), 10)
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
        let a = tuple(v)?;
        if a.len() != 2 {
            return Err("bad point".into());
        }
        Ok(Some((a[0].clone(), a[1].clone())))
    }
}
fn load(v: &Value) -> Result<(Compiled, Vec<BigUint>, Vec<u64>)> {
    let name = v["case"].as_str().ok_or("bad case")?;
    let actual = name.starts_with("original-actual");
    let k = if name.ends_with("k20633") {
        20633
    } else if actual {
        31
    } else {
        8
    };
    let s = spec(actual, k)?;
    let r = point(&v["value"]["R"])?;
    let curve = Curve {
        f: torus_s4::Field::new(s.p.clone())?,
        a: s.curve_a.clone().ok_or("missing A")?,
    };
    let q = curve.times(&r, 8)?;
    // Compile before reading any supplied coefficient/label data.
    let c = compile(&s, r, q, 200_000)?;
    let cs = tuple(&v["value"]["coefficients"])?;
    let labels = v["value"]["labels"]
        .as_array()
        .ok_or("bad labels")?
        .iter()
        .map(|x| x[0].as_u64().ok_or("bad j".into()))
        .collect::<Result<_>>()?;
    Ok((c, cs, labels))
}
fn rejected(x: Result<impl Sized>) -> Result<()> {
    if x.is_ok() {
        Err("negative control unexpectedly accepted".into())
    } else {
        Ok(())
    }
}
fn numeric_det(
    f: &torus_s4::Field,
    m: &[Vec<BigUint>],
    row: usize,
    used: &mut Vec<usize>,
) -> BigUint {
    if row == m.len() {
        return BigUint::one();
    }
    let mut sum = BigUint::zero();
    for col in 0..m.len() {
        if used.contains(&col) {
            continue;
        }
        let inversions = used.iter().filter(|j| **j > col).count();
        used.push(col);
        let term = f.mul(&m[row][col], &numeric_det(f, m, row + 1, used));
        used.pop();
        sum = if inversions % 2 == 0 {
            f.add(&sum, &term)
        } else {
            f.sub(&sum, &term)
        };
    }
    sum
}
fn one(v: &Value, dir: Option<&Path>) -> Result<Value> {
    let (mut c, cs, labels) = load(v)?;
    let name = v["case"].as_str().ok_or("bad name")?;
    let values = match c.extend(&cs, &labels) {
        Ok(x) => x,
        Err(e) if e == "inverse of zero" => {
            return Ok(
                json!({"case":name,"kind":"guard_refusal","detail":e,"scope":"supplied tuple only; not target absence","stats":c.program.stats()}),
            )
        }
        Err(e) => return Err(format!("{name}: {e}")),
    };
    let e = Engine::new(c.spec.p.clone(), 100_000_000)?;
    let q = Query::new(&e, &c.spec, c.r.clone(), c.target.clone(), cs.clone())?;
    let pred = torus_section_radical::predicate(&q)?;
    if pred["accept"] != true {
        return Err("independent original predicate mismatch".into());
    }
    for (w, x) in c.s.iter().zip(&q.ring.s) {
        if !x.b.is_zero() || values[*w] != x.a {
            return Err("independent sextic mismatch".into());
        }
    }
    // The existing numeric query stores tau=4*T_q(lambda)^2-2.
    let inv4 = e.f.inv(&4u32.into())?;
    for i in 0..6 {
        let ta = q.ring.tau.get(i).map(|x| x.a.clone()).unwrap_or_default();
        let tb = q.ring.tau.get(i).map(|x| x.b.clone()).unwrap_or_default();
        let real = if i == 0 {
            e.f.add(&ta, &2u32.into())
        } else {
            ta
        };
        if !tb.is_zero() || values[c.g[i]] != e.f.mul(&real, &inv4) {
            return Err("independent g remainder mismatch".into());
        }
    }
    let ng: Vec<_> =
        c.g.iter()
            .map(|w| torus_s4_tree::Fp2 {
                a: values[*w].clone(),
                b: BigUint::zero(),
            })
            .collect();
    let mut matrix = vec![vec![BigUint::zero(); 6]; 6];
    for col in 0..6 {
        let mut b = vec![torus_quotient_query::arithmetic::z(); 6];
        b[col] = torus_s4_tree::Fp2::one();
        for (row, x) in q.ring.mul(&ng, &b)?.into_iter().enumerate() {
            matrix[row][col] = x.a;
        }
    }
    for t in [0u32, 1, 7] {
        let mut tm = matrix.clone();
        for (i, row) in tm.iter_mut().enumerate() {
            for (j, x) in row.iter_mut().enumerate() {
                *x = e.f.sub(&if i == j { t.into() } else { BigUint::zero() }, x);
            }
        }
        let det = numeric_det(&e.f, &tm, 0, &mut vec![]);
        let mut val = BigUint::zero();
        for w in c.characteristic.iter().rev() {
            val = e.f.add(&e.f.mul(&val, &t.into()), &values[*w]);
        }
        if det != val {
            return Err("independent characteristic determinant mismatch".into());
        }
    }
    let table = IndexTable::new(&c.spec)?;
    let replay = q.extract(&table)?;
    if replay["original_replay_checked"] != true {
        return Err("missing original indexed replay".into());
    }
    // All semantic inputs plus a bounded set of internal wires, one at a time.
    let mut mutations = 0;
    for i in 0..values.len() {
        if matches!(c.program.gates[i], Gate::Input(_)) || i < 16 || i % 997 == 0 {
            let mut bad = values.clone();
            bad[i] = c.program.f.add(&bad[i], &BigUint::one());
            rejected(c.program.check(&bad))?;
            mutations += 1;
        }
    }
    rejected(c.program.check(&values[..values.len() - 1]))?;
    let mut bad = values.clone();
    bad[0] = c.spec.p.clone();
    rejected(c.program.check(&bad))?;
    let mut wrong = labels.clone();
    wrong[0] = 0;
    rejected(c.extend(&cs, &wrong))?;
    wrong[0] = c.spec.k + 1;
    rejected(c.extend(&cs, &wrong))?;
    let mut semantic = vec![BigUint::zero(); c.program.inputs];
    for (g, x) in c.program.gates.iter().zip(&values) {
        if let Gate::Input(i) = g {
            semantic[*i] = x.clone();
        }
    }
    // Recompute all dependent wires: these check constraints, not stale wires.
    for i in 5..8 {
        let mut bad = semantic.clone();
        bad[i] = BigUint::zero();
        rejected(c.program.check(&c.program.evaluate(&bad)?))?;
    }
    let mut nonbit = semantic.clone();
    nonbit[8] = 2u32.into();
    rejected(c.program.check(&c.program.evaluate(&nonbit)?))?;
    let mut overflow = semantic.clone();
    for b in 0..c.bit_length {
        overflow[8 + b] = BigUint::from((c.spec.k >> b) & 1);
        overflow[8 + c.bit_length + b] = BigUint::zero();
    }
    rejected(c.program.check(&c.program.evaluate(&overflow)?))?;
    // Relabeling the multiset preserves the equations; strict order is not imposed.
    let mut reverse = labels.clone();
    reverse.reverse();
    c.extend(&cs, &reverse)?;
    rejected(compile(&c.spec, c.r.clone(), c.target.clone(), 16))?;
    let mut malformed = c.spec.clone();
    malformed.k = 0;
    rejected(compile(&malformed, c.r.clone(), c.target.clone(), 200_000))?;
    if c.r.is_some() && c.target.is_some() {
        rejected(compile(&c.spec, c.r.clone(), None, 200_000))?;
    }
    // Validate malformed forward-edge rejection, restoring the exact program.
    let saved = c.program.gates[0].clone();
    c.program.gates[0] = Gate::Add(values.len(), 0);
    rejected(c.program.validate())?;
    c.program.gates[0] = saved;
    let export = if let Some(dir) = dir {
        if name == "original-actual-k20633" || name == "toy-signed-support-207" {
            let path = dir.join(format!("{name}.smt2"));
            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .map_err(|e| e.to_string())?;
            let mut out = BufWriter::new(file);
            c.program.smt(&mut out)?;
            out.flush().map_err(|e| e.to_string())?;
            Some(path.to_string_lossy().to_string())
        } else {
            None
        }
    } else {
        None
    };
    Ok(
        json!({"case":name,"kind":"extension_and_original_replay","stats":c.program.stats(),"bit_length":c.bit_length,"labels":labels,"coefficient_slots":c.coefficient_slots,"mutations_rejected":mutations,"recomputed_guard_bit_range_refusals":5,"independent_determinant_points":[0,1,7],"original":replay,"native_sextic":c.s.iter().map(|w|values[*w].to_string()).collect::<Vec<_>>(),"native_g":c.g.iter().map(|w|values[*w].to_string()).collect::<Vec<_>>(),"native_characteristic":c.characteristic.iter().map(|w|values[*w].to_string()).collect::<Vec<_>>(),"smt_export":export}),
    )
}
pub fn run(dir: Option<&Path>, out: &mut impl Write) -> Result<()> {
    let f = fixtures()?;
    let mut n = 0;
    let mut accepted = 0;
    for v in f["cases"].as_array().ok_or("missing fixture cases")? {
        let r = one(v, dir)?;
        accepted += usize::from(r["kind"] == "extension_and_original_replay");
        writeln!(out, "{r}").map_err(|e| e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
        n += 1;
    }
    if accepted < 2 {
        return Err("insufficient admitted known-answer controls".into());
    }
    writeln!(out,"{}",json!({"kind":"complete","cases":n,"admitted":accepted,"scope":"fixed development correctness; no coefficient root discovery, solving degree or total cold cost"})).map_err(|e|e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_original_extension_replay() -> Result<()> {
        run(None, &mut std::io::sink())
    }
}
