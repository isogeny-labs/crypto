//! Exact parameter-certificate audit; no decoder or root-discovery algorithm.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    io::{Read, Write},
};
use torus_coset_circuit::{aggregation::TargetCircuit, index::IndexTable, Circuit, WitnessInput};
use torus_quotient_query::controls::spec;
use torus_s4::{Field, Point};
use torus_section_operator::{IndexedPoint, Operator};
pub type Result<T> = std::result::Result<T, String>;
fn mul(a: u128, b: u128) -> Result<u128> {
    a.checked_mul(b).ok_or("integer product overflow".into())
}
fn add(a: u128, b: u128) -> Result<u128> {
    a.checked_add(b).ok_or("integer sum overflow".into())
}
/// This models the two STANDARD sufficient certificate guards. Passing it
/// does not assert a decoder is implemented, applicable or successful.
pub fn parameters(b: u128, m: u128, s: u128, t: u128) -> Result<Value> {
    if m == 0 || s == 0 || s > m || m > b {
        return Err("invalid/nonempty folding parameters".into());
    }
    let n = b / m;
    let tail = b % m;
    let a = (t / m).min(n);
    let l = m - s + 1;
    let nl = mul(n, l)?;
    if mul(n, m)? < 5 || nl < 4 {
        return Err("five-coefficient interpolation degree not admitted".into());
    }
    let d = (nl - 4) / add(s, 1)?;
    let dimensions = add(mul(add(d, 1)?, add(s, 1)?)?, 4)?;
    let roots = mul(a, l)?;
    let pole = add(d, 4)?;
    let floor = mul(a, add(s, 1)?)?;
    Ok(
        json!({"length":b.to_string(),"fold_width":m.to_string(),"interpolation_s":s.to_string(),"message_dimension":5,"degree_offset":4,"complete_blocks":n.to_string(),"discarded_tail":tail.to_string(),"support_budget":t.to_string(),"optimistic_full_agreeing_blocks":a.to_string(),"L":l.to_string(),"D":d.to_string(),"interpolation_dimensions":dimensions.to_string(),"constraints":nl.to_string(),"dimension_guard":dimensions>nl,"available_root_count":roots.to_string(),"root_degree_bound":pole.to_string(),"root_count_guard":roots>pole,"floor_guard":floor>n,"certificate_guards_hold":dimensions>nl&&roots>pole,"scope":"parameter guards only; no algorithm success/failure claim"}),
    )
}
fn number(v: &Value) -> Result<BigUint> {
    v.as_str()
        .ok_or("decimal string")?
        .parse()
        .map_err(|_| "bad decimal".into())
}
fn point(v: &Value) -> Result<Point> {
    if v.is_null() {
        return Ok(None);
    }
    let a = v.as_array().ok_or("point")?;
    if a.len() != 2 {
        return Err("point length".into());
    }
    Ok(Some((number(&a[0])?, number(&a[1])?)))
}
fn fixture() -> Result<Value> {
    let mut text = String::new();
    flate2::read::GzDecoder::new(&include_bytes!("../../torus-point-solver/FIXTURES.jsonl.gz")[..])
        .take(1 << 20)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    text.lines()
        .map(|l| serde_json::from_str::<Value>(l).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .find(|v| v["case"].as_str() == Some("toy-signed-support-1"))
        .ok_or("fixed supplied fixture absent".into())
}
fn dot(f: &Field, a: &[BigUint], b: &[BigUint]) -> BigUint {
    a.iter()
        .zip(b)
        .fold(BigUint::zero(), |s, (x, y)| f.add(&s, &f.mul(x, y)))
}
/// Same line-bundle trivializations as the original numeric section operator.
fn fibre(f: &Field, curve_a: &BigUint, p: &Point, r: &Point) -> Result<Vec<BigUint>> {
    let (x, y) = p.as_ref().ok_or("identity member unsupported")?;
    let (rx, ry) = r.as_ref().ok_or("finite target required")?;
    if x == rx && y == ry {
        return Ok(vec![
            0u32.into(),
            0u32.into(),
            0u32.into(),
            0u32.into(),
            0u32.into(),
            f.scale(ry, 2),
        ]);
    }
    let h = if x == rx {
        if y != &f.sub(&BigUint::zero(), ry) {
            return Err("same-x point is neither target nor its negative".into());
        }
        let derivative = f.add(
            &f.add(&f.scale(&f.square(x), 3), &f.scale(&f.mul(curve_a, x), 2)),
            &BigUint::one(),
        );
        f.mul(&derivative, &f.inv(&f.scale(y, 2))?)
    } else {
        f.mul(&f.add(y, ry), &f.inv(&f.sub(x, rx))?)
    };
    Ok(vec![
        BigUint::one(),
        x.clone(),
        y.clone(),
        f.square(x),
        f.mul(x, y),
        h,
    ])
}
fn emit(out: &mut impl Write, v: Value) -> Result<()> {
    writeln!(out, "{v}").map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())
}
pub fn run(out: &mut impl Write) -> Result<()> {
    let fixed = fixture()?;
    let old = &fixed["checked"]["original"];
    let r = point(&old["public_R"])?;
    let q = point(&old["public_Q"])?;
    let cs = old["coefficients"]
        .as_array()
        .ok_or("coefficients")?
        .iter()
        .map(number)
        .collect::<Result<Vec<_>>>()?;
    if cs.len() != 6 || cs[4] != BigUint::one() {
        return Err("normalized supplied section".into());
    }
    let s = spec(false, 8)?;
    let f = Field::new(s.p.clone())?;
    let curve = torus_s4::Curve {
        f: f.clone(),
        a: s.curve_a.clone().ok_or("A")?,
    };
    let member = Circuit::compile(&s)?;
    let table = IndexTable::new(&s)?;
    let half = f.mul(&curve.a, &f.inv(&2u32.into())?);
    let center = f.sub(&BigUint::zero(), &half);
    let (mut rows, mut zeros, mut geometries, mut excluded) =
        (vec![], vec![], BTreeSet::new(), vec![]);
    for j in 1..=s.k {
        for t in 0..s.q {
            let z =
                s.h.pow(&(BigUint::from(j) + BigUint::from(s.order_factor) * t), &f);
            let delta = f.mul(&half, &f.inv(&z.a)?);
            let mut retained = 0;
            for (branch, x) in [f.add(&center, &delta), f.sub(&center, &delta)]
                .into_iter()
                .enumerate()
            {
                let Some(y) = f.sqrt(&curve.rhs(&x))? else {
                    continue;
                };
                if y.is_zero() {
                    continue;
                }
                let ny = f.sub(&BigUint::zero(), &y);
                let y = y.min(ny);
                for y in [y.clone(), f.sub(&BigUint::zero(), &y)] {
                    let p = Some((x.clone(), y.clone()));
                    if curve.times(&p, 8)?.is_none() {
                        continue;
                    }
                    let input = WitnessInput {
                        z: z.clone(),
                        selector: j - 1,
                        slack: s.k - j,
                        branch: BigUint::from(branch),
                        raw_v: y.clone(),
                    };
                    let witness = member.witness(&input)?;
                    let decoded = table.decode(&s, &member, &witness)?;
                    if decoded.j != j
                        || decoded.t != t
                        || member.image(&witness)? != curve.times(&p, 8)?
                    {
                        return Err("original membership/index/image replay mismatch".into());
                    }
                    if !geometries.insert((x.clone(), y.clone())) {
                        return Err("geometric duplicate: six-position support not automatically applicable".into());
                    }
                    let basis = fibre(&f, &curve.a, &p, &r)?;
                    let value = dot(&f, &basis, &cs);
                    let received = f.sub(&BigUint::zero(), &basis[4]);
                    let free = cs
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != 4)
                        .fold(BigUint::zero(), |a, (i, c)| f.add(&a, &f.mul(c, &basis[i])));
                    if (free == received) != value.is_zero() {
                        return Err("normalized coset agreement mismatch".into());
                    }
                    let index = rows.len();
                    if value.is_zero() {
                        zeros.push((index, input, p.clone()));
                    }
                    rows.push(json!({"position":index,"j":j,"t":t,"branch":branch,"point":[x.to_string(),y.to_string()],"image":member.image(&witness)?.as_ref().map(|(u,v)|[u.to_string(),v.to_string()]),"canonical_coefficient":decoded.coefficient,"member_witness":witness.iter().map(ToString::to_string).collect::<Vec<_>>(),"section_basis":basis.iter().map(ToString::to_string).collect::<Vec<_>>(),"received":received.to_string(),"codeword":free.to_string(),"section_value":value.to_string(),"agreement":value.is_zero()}));
                    retained += 1;
                }
            }
            if retained == 0 {
                excluded
                    .push(json!({"j":j,"t":t,"reason":"no admitted nonidentity eightfold image"}));
            }
        }
    }
    if rows.len() != 44 || zeros.len() != 6 {
        return Err(format!(
            "toy length/support mismatch {}/{}",
            rows.len(),
            zeros.len()
        ));
    }
    let original = TargetCircuit::compile(&s, 6, q.clone(), true)?;
    let inputs = zeros.iter().map(|(_, i, _)| i.clone()).collect::<Vec<_>>();
    let witness = original.witness(&inputs)?;
    let decoded = original.decode_row(&witness, &table)?;
    let mut raw_sum = None;
    for (_, _, p) in &zeros {
        raw_sum = curve.add(&raw_sum, p)?;
    }
    if raw_sum != r {
        return Err("six supplied zeros do not replay raw R".into());
    }
    let op = Operator::new(s.p.clone(), curve.a.clone())?;
    let indexed = zeros
        .iter()
        .map(|(i, _, p)| IndexedPoint {
            column: *i as u64,
            point: p.clone(),
        })
        .collect::<Vec<_>>();
    let scale = f.inv(cs.iter().find(|c| !c.is_zero()).ok_or("zero section")?)?;
    op.verify_section(
        &indexed,
        &r,
        &cs.iter().map(|c| f.mul(c, &scale)).collect::<Vec<_>>(),
    )?;
    let support = zeros.iter().map(|(i, _, _)| *i).collect::<BTreeSet<_>>();
    let mut folding = vec![];
    for m in 1u128..=6 {
        for par in 1..=m {
            let mut a = parameters(rows.len() as u128, m, par, 6)?;
            let actual = (0..rows.len() / (m as usize))
                .filter(|block| {
                    (0..m as usize).all(|offset| support.contains(&(block * m as usize + offset)))
                })
                .count();
            a["actual_full_agreeing_blocks"] = json!(actual);
            if a["floor_guard"] != false || a["certificate_guards_hold"] != false {
                return Err("optimistic toy guarantee unexpectedly applicable".into());
            }
            folding.push(a);
        }
    }
    emit(
        out,
        json!({"kind":"original_toy_support_audit","p":s.p.to_string(),"q":s.q,"k":s.k,"columns":rows.len(),"geometric_duplicates":0,"excluded_raw_labels":excluded,"section_coefficients":cs.iter().map(ToString::to_string).collect::<Vec<_>>(),"public_R":r.as_ref().map(|(x,y)|[x.to_string(),y.to_string()]),"public_Q":q.as_ref().map(|(x,y)|[x.to_string(),y.to_string()]),"matching_positions":support,"full_base":rows,"folds":folding,"original_witness":witness.iter().map(ToString::to_string).collect::<Vec<_>>(),"decoded_rows":decoded.len(),"raw_R_and_original_Q_replay":true,"numeric_section_replay":true,"scope":"known supplied section and complete declared toy base, not new root discovery/decoder failure"}),
    )?;
    let declared = mul(213156431, 20633)?;
    let mut big = vec![];
    for m in 1..=6 {
        for s in 1..=m {
            let a = parameters(declared, m, s, 6)?;
            if a["floor_guard"] != false || a["certificate_guards_hold"] != false {
                return Err("declared-length budget condition unexpectedly applicable".into());
            }
            big.push(a);
        }
    }
    emit(
        out,
        json!({"kind":"declared_length_arithmetic","length":declared.to_string(),"folds":big,"scope":"integer model only; actual base cardinality/support geometry not proved or enumerated"}),
    )?;
    let yes = parameters(6, 1, 1, 6)?;
    let floor_only = parameters(41, 6, 6, 6)?;
    if yes["certificate_guards_hold"] != true
        || floor_only["floor_guard"] != true
        || floor_only["certificate_guards_hold"] != false
    {
        return Err("positive/one-way certificate controls".into());
    }
    emit(
        out,
        json!({"kind":"guard_controls","small_positive_certificate":yes,"floor_positive_full_negative":floor_only,"decoder_implemented":false}),
    )?;
    emit(
        out,
        json!({"kind":"complete","toy_columns":44,"supplied_section_matches":6,"toy_parameter_cases":21,"declared_integer_cases":21,"backend_invocations":0,"new_roots_discovered":0,"conclusion":"Standard parameter certificate not instantiated. No algorithmic lower bound or universal folding impossibility."}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fibre_charts_match_original_operator() -> Result<()> {
        let f = Field::new(101u32.into())?;
        let r = Some((89u32.into(), 27u32.into()));
        let op = Operator::new(101u32.into(), 95u32.into())?;
        for p in [
            r.clone(),
            Some((89u32.into(), 74u32.into())),
            Some((70u32.into(), 41u32.into())),
        ] {
            let terms = (0..6)
                .map(|i| IndexedPoint {
                    column: i,
                    point: p.clone(),
                })
                .collect::<Vec<_>>();
            let old = op.evaluate(&terms, &r)?;
            assert_eq!(fibre(&f, &95u32.into(), &p, &r)?, old.rows[0]);
        }
        Ok(())
    }
    #[test]
    fn integer_bounds_and_overflow() -> Result<()> {
        for b in 1..=80u128 {
            for m in 1..=10 {
                for s in 1..=m {
                    for a in 0..=6 / m {
                        let n = b / m;
                        let tail = b % m;
                        if n < a * (s + 1) {
                            assert!(b < 42, "B={b} m={m} s={s} a={a} tail={tail}");
                        }
                    }
                }
            }
        }
        assert!(parameters(44, 0, 1, 6).is_err());
        assert!(parameters(44, 3, 4, 6).is_err());
        assert!(parameters(u128::MAX, 1, 1, 6).is_err());
        assert_eq!(parameters(6, 1, 1, 6)?["certificate_guards_hold"], true);
        assert_eq!(
            parameters(5, 1, 1, 6)?["optimistic_full_agreeing_blocks"],
            "5"
        );
        assert_eq!(parameters(41, 6, 6, 6)?["floor_guard"], true);
        assert_eq!(parameters(41, 6, 6, 6)?["certificate_guards_hold"], false);
        Ok(())
    }
    #[test]
    fn packed_and_scattered_matches_differ() {
        let packed = BTreeSet::from([0usize, 1, 2, 3, 4, 5]);
        let scattered = BTreeSet::from([0usize, 3, 6, 9, 12, 15]);
        let count = |s: &BTreeSet<usize>| {
            (0..6)
                .filter(|b| (0..3).all(|j| s.contains(&(b * 3 + j))))
                .count()
        };
        assert_eq!(count(&packed), 2);
        assert_eq!(count(&scattered), 0);
    }
}
