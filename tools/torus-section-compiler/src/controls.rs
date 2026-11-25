//! Fixed known-answer controls. Labels/coefficient tuples enter only extension/replay.
use crate::{compile, Compiled, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};
use torus_coset_circuit::index::IndexTable;
use torus_quotient_query::{arithmetic::Engine, controls::spec, query::Query};
use torus_s4::{Curve, Field, Point};
const CAP: usize = 16_000_000;
pub fn fixtures() -> Result<Value> {
    serde_json::from_str(include_str!("../FIXTURES.json")).map_err(|e| e.to_string())
}
fn number(v: &Value) -> Result<BigUint> {
    v.as_str()
        .and_then(|s| BigUint::parse_bytes(s.as_bytes(), 10))
        .ok_or("bad fixture integer".into())
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
fn load(v: &Value) -> Result<(Compiled, Vec<BigUint>)> {
    let name = v["case"].as_str().ok_or("bad fixture case")?;
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
        f: Field::new(s.p.clone())?,
        a: s.curve_a.clone().unwrap(),
    };
    let q = curve.times(&r, 8)?;
    // No coefficients, labels or sign bits are passed to compilation.
    let c = compile(&s, r, q, CAP)?;
    let cs = tuple(&v["value"]["coefficients"])?;
    Ok((c, cs))
}
struct Hashed<W> {
    inner: W,
    hash: Sha256,
    bytes: u64,
}
impl<W: Write> Write for Hashed<W> {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        let n = self.inner.write(b)?;
        self.hash.update(&b[..n]);
        self.bytes += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
fn export(
    path: &Path,
    write: impl FnOnce(&mut Hashed<BufWriter<std::fs::File>>) -> Result<()>,
) -> Result<Value> {
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    let mut out = Hashed {
        inner: BufWriter::new(file),
        hash: Sha256::new(),
        bytes: 0,
    };
    write(&mut out)?;
    out.flush().map_err(|e| e.to_string())?;
    Ok(
        json!({"path":path.to_string_lossy(),"bytes":out.bytes,"sha256":format!("{:x}",out.hash.finalize()),"standing":"local producer bytes; no trusted-store receipt"}),
    )
}
fn replay(c: &Compiled, cs: &[BigUint]) -> Result<Value> {
    let e = Engine::new(c.spec.p.clone(), 100_000_000)?;
    let q = Query::new(&e, &c.spec, c.r.clone(), c.target.clone(), cs.to_vec())?;
    let reduced = torus_section_radical::predicate(&q)?;
    if reduced["accept"] != true {
        return Err("projected tuple fails frozen reduced predicate".into());
    }
    let original = q.extract(&IndexTable::new(&c.spec)?)?;
    Ok(json!({"frozen_predicate":reduced,"original":original}))
}
fn checked(v: &Value, dir: &Path, mutations: bool) -> Result<Value> {
    let (c, cs) = load(v)?;
    let mut values = c.extend(&cs)?;
    c.program.check(&values)?;
    if c.project(&values)? != cs {
        return Err("coefficient projection mismatch".into());
    }
    let old = replay(&c, &cs)?;
    let expected: BTreeSet<_> = v["value"]["labels"]
        .as_array()
        .ok_or("missing known labels")?
        .iter()
        .map(|x| (x[0].as_u64().unwrap(), x[1].as_u64().unwrap()))
        .collect();
    let got: BTreeSet<_> = old["original"]["row"]
        .as_array()
        .ok_or("missing recovered row")?
        .iter()
        .map(|x| (x["j"].as_u64().unwrap(), x["t"].as_u64().unwrap()))
        .collect();
    if expected.len() != 6 || got != expected {
        return Err("recovered original raw labels differ".into());
    }
    let numeric_s = &old["frozen_predicate"]["S"];
    for (i, w) in c.s.iter().enumerate() {
        if values[w.0 as usize].to_string()
            != numeric_s[i][0].as_str().ok_or("missing numeric norm")?
        {
            return Err("norm compiler mismatch".into());
        }
    }
    let numeric_h = &old["frozen_predicate"]["H"];
    for i in 0..6 {
        if values[c.h[2 * i].0 as usize].to_string()
            != numeric_h[i][0].as_str().ok_or("missing numeric H")?
        {
            return Err("H compiler mismatch".into());
        }
    }
    let mut checks = 0;
    if mutations {
        for j in 0..64 {
            let i = j * (values.len() - 1) / 63;
            let old = values[i].clone();
            values[i] = c.program.f.add(&old, &BigUint::one());
            if c.program.check(&values).is_ok() {
                return Err(format!("single-wire mutation{i} accepted"));
            }
            values[i] = old;
            checks += 1;
        }
        for i in [5, 6] {
            let w = c.input_wires[i].0 as usize;
            let old = values[w].clone();
            values[w] = c.program.f.add(&old, &BigUint::one());
            if c.program.check(&values).is_ok() {
                return Err("inverse-input mutation accepted".into());
            }
            values[w] = old;
            checks += 1;
        }
        if c.program.check(&values[..values.len() - 1]).is_ok() {
            return Err("truncated assignment accepted".into());
        }
        values[0] = c.spec.p.clone();
        if c.program.check(&values).is_ok() {
            return Err("noncanonical assignment accepted".into());
        }
        values[0] = 0u32.into();
        let mut rows = 0;
        for row in c.program.equations() {
            let mut sum = BigUint::zero();
            for (coef, monomial) in row {
                let mut term = coef;
                for id in monomial {
                    term = c.program.f.mul(&term, &values[id as usize]);
                }
                sum = c.program.f.add(&sum, &term);
            }
            if !sum.is_zero() {
                return Err("lowered equation replay failed".into());
            }
            rows += 1;
        }
        if rows != c.program.stats()["equations"].as_u64().unwrap() {
            return Err("equation iterator count mismatch".into());
        }
    }
    let name = v["case"].as_str().unwrap();
    let system = export(&dir.join(format!("{name}.tcg")), |out| c.program.write(out))?;
    let witness = export(&dir.join(format!("{name}.witness")), |out| {
        c.program.write_witness(&values, out)
    })?;
    Ok(
        json!({"R":v["value"]["R"],"coefficients":v["value"]["coefficients"],"known_expected_labels":v["value"]["labels"],
        "public_spec":{"p":c.spec.p.to_string(),"A":c.spec.curve_a.as_ref().map(ToString::to_string),"h":[c.spec.h.a.to_string(),c.spec.h.b.to_string()],"a":c.spec.order_factor,"q":c.spec.q,"k":c.spec.k,"Q":torus_quotient_query::query::point_json(&c.target)},
        "wire_binding":{"coefficient_slots":c.coefficient_slots,"input_wires":c.input_wires.iter().map(|w|w.0).collect::<Vec<_>>(),"S":c.s.iter().map(|w|w.0).collect::<Vec<_>>(),"H":c.h.iter().map(|w|w.0).collect::<Vec<_>>(),"map_remainder":c.map_remainder.0,"torsion_resultant":c.torsion_resultant.0,"guard_prefix":c.guard_prefix},
        "stats":c.program.stats(),"guard_inputs":[values[c.input_wires[5].0 as usize].to_string(),values[c.input_wires[6].0 as usize].to_string()],
        "map_remainder":values[c.map_remainder.0 as usize].to_string(),"torsion_resultant":values[c.torsion_resultant.0 as usize].to_string(),
        "H_coordinates":c.h.iter().map(|w|values[w.0 as usize].to_string()).collect::<Vec<_>>(),
        "system":system,"witness":witness,"single_wire_mutations_rejected":checks,"frozen_replay":old}),
    )
}
fn pencil(v: &Value) -> Result<Value> {
    let (c, mut cs) = load(v)?;
    let mut records = vec![];
    for n in 0..101u32 {
        cs[3] = n.into();
        let e = Engine::new(c.spec.p.clone(), 100_000_000)?;
        let old = Query::new(&e, &c.spec, c.r.clone(), c.target.clone(), cs.clone());
        let old_accept = match &old {
            Ok(q) => torus_section_radical::predicate(q)?["accept"] == true,
            Err(_) => false,
        };
        let extended = c.extend(&cs);
        let accepted = match &extended {
            Ok(v) => c.program.outputs_hold(v),
            Err(_) => false,
        };
        if accepted != old_accept {
            return Err(format!("pencil mismatch at{n}"));
        }
        if accepted {
            let val = extended.as_ref().unwrap();
            c.program.check(val)?;
            if c.project(val)? != cs {
                return Err("pencil projection mismatch".into());
            }
            replay(&c, &cs)?;
        }
        records.push(json!({"c3":n,"accepted":accepted,"compiler_refusal":extended.err(),"numeric_refusal":old.err()}));
    }
    Ok(json!({"count":records.len(),"records":records}))
}
fn torsion() -> Result<Value> {
    let mut domains = vec![];
    for actual in [false, true] {
        let s = spec(actual, if actual { 31 } else { 8 })?;
        let f = Field::new(s.p.clone())?;
        let curve = Curve {
            f: f.clone(),
            a: s.curve_a.clone().unwrap(),
        };
        let mut xs = if actual {
            [
                "1",
                "325606250916557431795983626356110631294008115727848805560023387167927233504",
                "39382357235489614581723060781553021112529911719440698176882885853963445705823",
            ]
            .iter()
            .map(|s| BigUint::parse_bytes(s.as_bytes(), 10).unwrap())
            .collect::<Vec<_>>()
        } else {
            (0..101u32).map(BigUint::from).collect()
        };
        let mut points = vec![];
        for x in xs.drain(..) {
            if let Some(y) = f.sqrt(&curve.rhs(&x))? {
                if y.is_zero() {
                    continue;
                }
                let n = f.square(&f.sub(&f.square(&x), &1u32.into()));
                let d = f.scale(&curve.rhs(&x), 4);
                let j = f.mul(&f.sub(&x, &1u32.into()), &f.sub(&n, &d));
                for yy in [y.clone(), f.sub(&0u32.into(), &y)] {
                    let zero = curve.times(&Some((x.clone(), yy.clone())), 8)?.is_none();
                    if zero != j.is_zero() {
                        return Err("degree5 rational torsion equivalence mismatch".into());
                    }
                    points.push(json!({"x":x.to_string(),"y":yy.to_string(),"J":j.to_string(),"eightfold_identity":zero}));
                }
            }
        }
        domains.push(json!({"actual":actual,"A_plus_2_sqrt":f.sqrt(&f.add(&curve.a,&2u32.into()))?.map(|x|x.to_string()),"A_minus_2_character":f.sub(&curve.a,&2u32.into()).modpow(&((&s.p-1u32)>>1),&s.p).to_string(),"points":points}));
    }
    Ok(json!(domains))
}
fn refusals(v: &Value) -> Result<Value> {
    let (c, cs) = load(v)?;
    let mut bad = c.spec.clone();
    bad.q = 1;
    let mut records = vec![
        compile(&bad, c.r.clone(), c.target.clone(), CAP)
            .err()
            .ok_or("bad q compiled")?,
        compile(&c.spec, c.r.clone(), c.target.clone(), 16)
            .err()
            .ok_or("forced node cap accepted")?,
        compile(&c.spec, c.r.clone(), None, CAP)
            .err()
            .ok_or("wrong Q accepted")?,
    ];
    let mut badcs = cs.clone();
    badcs[4] = 2u32.into();
    records.push(
        c.extend(&badcs)
            .err()
            .ok_or("unnormalized tuple accepted")?,
    );
    badcs = cs;
    badcs[0] = c.spec.p.clone();
    records.push(
        c.extend(&badcs)
            .err()
            .ok_or("noncanonical tuple accepted")?,
    );
    let e = crate::arithmetic::Engine::new(c.spec.p.clone(), 100)?;
    let input = e.input();
    records.push(
        e.inv(&input)
            .err()
            .ok_or("unguarded symbolic inverse accepted")?,
    );
    Ok(json!(records))
}
pub fn run(emit: &mut impl FnMut(Value) -> Result<()>, dir: &Path) -> Result<bool> {
    std::fs::create_dir(dir).map_err(|e| e.to_string())?;
    let fixture = fixtures()?;
    let cases = fixture["cases"].as_array().ok_or("missing cases")?;
    let mut all = true;
    let mut arm = |name: &str, result: Result<Value>| -> Result<()> {
        match result {
            Ok(v) => emit(json!({"case":name,"passed":true,"value":v})),
            Err(e) => {
                all = false;
                emit(json!({"case":name,"passed":false,"error":e}))
            }
        }
    };
    arm("rational-torsion", torsion())?;
    arm("malformed-and-cap-refusals", refusals(&cases[0]))?;
    arm("coefficient-pencil-101", pencil(&cases[0]))?;
    for (i, v) in cases.iter().enumerate() {
        arm(v["case"].as_str().unwrap(), checked(v, dir, i == 0))?;
    }
    Ok(all)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rational_torsion_fixed_domains() {
        assert!(torsion().is_ok());
    }
    #[test]
    fn public_compile_extend_project_and_caps() -> Result<()> {
        let f = fixtures()?;
        let v = &f["cases"][0];
        let (c, cs) = load(v)?;
        let values = c.extend(&cs)?;
        c.program.check(&values)?;
        assert_eq!(c.project(&values)?, cs);
        assert!(refusals(v).is_ok());
        Ok(())
    }
}
