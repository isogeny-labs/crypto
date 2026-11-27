//! Fixed finite interpolation templates on original coset rows. No decoder.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{collections::BTreeSet, io::Write};
use torus_coset_circuit::{index::IndexTable, Circuit, WitnessInput};
use torus_quotient_query::controls::spec;
use torus_s4::{Curve, Field};
use torus_s4_tree::Fp2;
pub type Result<T> = std::result::Result<T, String>;
type Matrix = Vec<Vec<Fp2>>;
const PREFIX: usize = 108;
fn zero() -> Fp2 {
    fp(0)
}
fn fp(n: u64) -> Fp2 {
    Fp2 {
        a: n.into(),
        b: 0u32.into(),
    }
}
fn is_zero(a: &Fp2) -> bool {
    a.a.is_zero() && a.b.is_zero()
}
fn sub(f: &Field, a: &Fp2, b: &Fp2) -> Fp2 {
    Fp2 {
        a: f.sub(&a.a, &b.a),
        b: f.sub(&a.b, &b.b),
    }
}
fn encoded(a: &Fp2) -> Value {
    json!([a.a.to_string(), a.b.to_string()])
}
fn encoded_matrix(a: &[Vec<Fp2>]) -> Value {
    json!(a
        .iter()
        .map(|r| r.iter().map(encoded).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn dimensions(f: &Field, a: &[Vec<Fp2>]) -> Result<(usize, usize)> {
    if a.is_empty() || a.len() > PREFIX || a[0].is_empty() || a[0].len() > 18 {
        return Err("matrix resource/dimension admission".into());
    }
    let n = a[0].len();
    if a.iter()
        .any(|r| r.len() != n || r.iter().any(|v| v.a >= f.p || v.b >= f.p))
    {
        return Err("ragged/noncanonical matrix".into());
    }
    Ok((a.len(), n))
}
struct Reduced {
    a: Matrix,
    pivots: Vec<usize>,
    rows: Vec<usize>,
}
fn reduce(f: &Field, a: &[Vec<Fp2>]) -> Result<Reduced> {
    let (m, n) = dimensions(f, a)?;
    let mut a = a.to_vec();
    let mut ids: Vec<_> = (0..m).collect();
    let mut pivots = vec![];
    for c in 0..n {
        let k = pivots.len();
        let Some(r) = (k..m).find(|&r| !is_zero(&a[r][c])) else {
            continue;
        };
        a.swap(k, r);
        ids.swap(k, r);
        let inv = a[k][c].inverse(f)?;
        for item in a[k].iter_mut().skip(c) {
            *item = item.mul(&inv, f);
        }
        let pivot = a[k].clone();
        for row in a.iter_mut().skip(k + 1) {
            let factor = row[c].clone();
            for j in c..n {
                row[j] = sub(f, &row[j], &factor.mul(&pivot[j], f));
            }
        }
        pivots.push(c);
        if pivots.len() == m {
            break;
        }
    }
    ids.truncate(pivots.len());
    Ok(Reduced {
        a,
        pivots,
        rows: ids,
    })
}
fn kernel(f: &Field, r: &Reduced, n: usize) -> Result<Vec<Fp2>> {
    let free = (0..n)
        .find(|c| !r.pivots.contains(c))
        .ok_or("full rank has no requested kernel")?;
    let mut v = vec![zero(); n];
    v[free] = Fp2::one();
    for (i, &c) in r.pivots.iter().enumerate().rev() {
        let mut sum = zero();
        for (j, x) in v.iter().enumerate().skip(c + 1) {
            sum = sub(f, &sum, &r.a[i][j].mul(x, f));
        }
        v[c] = sum;
    }
    Ok(v)
}
fn check_kernel(f: &Field, a: &[Vec<Fp2>], v: &[Fp2]) -> Result<()> {
    let (_, n) = dimensions(f, a)?;
    if v.len() != n || v.iter().all(is_zero) || v.iter().any(|x| x.a >= f.p || x.b >= f.p) {
        return Err("invalid kernel witness".into());
    }
    for r in a {
        let sum = r
            .iter()
            .zip(v)
            .fold(zero(), |s, (x, y)| sub(f, &s, &x.mul(y, f)));
        if !is_zero(&sum) {
            return Err("kernel residual".into());
        }
    }
    Ok(())
}
fn determinant_lu(f: &Field, a: &[Vec<Fp2>]) -> Result<Fp2> {
    let (m, n) = dimensions(f, a)?;
    if m != n {
        return Err("square minor required".into());
    }
    let mut a = a.to_vec();
    let mut det = Fp2::one();
    for k in 0..n {
        let Some(r) = (k..n).find(|&r| !is_zero(&a[r][k])) else {
            return Ok(zero());
        };
        if r != k {
            a.swap(k, r);
            det = det.neg(f)
        }
        let p = a[k][k].clone();
        det = det.mul(&p, f);
        let inv = p.inverse(f)?;
        let row = a[k].clone();
        for target in a.iter_mut().skip(k + 1) {
            let ratio = target[k].mul(&inv, f);
            for j in k..n {
                target[j] = sub(f, &target[j], &ratio.mul(&row[j], f));
            }
        }
    }
    Ok(det)
}
/// Separate pivot/minor recurrence; division is in the declared extension field.
fn determinant_bareiss(f: &Field, a: &[Vec<Fp2>]) -> Result<Fp2> {
    let (m, n) = dimensions(f, a)?;
    if m != n {
        return Err("square minor required".into());
    }
    let mut b = a.to_vec();
    let mut previous = Fp2::one();
    let mut negative = false;
    for k in 0..n - 1 {
        let Some(r) = (k..n).find(|&r| !is_zero(&b[r][k])) else {
            return Ok(zero());
        };
        if r != k {
            b.swap(k, r);
            negative = !negative
        }
        let pivot = b[k][k].clone();
        let row = b[k].clone();
        let inv = previous.inverse(f)?;
        for target in b.iter_mut().skip(k + 1) {
            let left = target[k].clone();
            for j in k + 1..n {
                target[j] = sub(f, &pivot.mul(&target[j], f), &left.mul(&row[j], f)).mul(&inv, f);
            }
            target[k] = zero();
        }
        previous = pivot;
    }
    let d = b[n - 1][n - 1].clone();
    Ok(if negative { d.neg(f) } else { d })
}
fn verify_minor(f: &Field, a: &[Vec<Fp2>], rows: &[usize], det: &Fp2, inv: &Fp2) -> Result<Matrix> {
    let (m, n) = dimensions(f, a)?;
    if rows.len() != n
        || rows.iter().any(|&i| i >= m)
        || rows.iter().collect::<BTreeSet<_>>().len() != n
        || det.a >= f.p
        || det.b >= f.p
        || inv.a >= f.p
        || inv.b >= f.p
        || is_zero(det)
    {
        return Err("malformed full-rank certificate".into());
    }
    let minor: Matrix = rows.iter().map(|&i| a[i].clone()).collect();
    if determinant_bareiss(f, &minor)? != *det || det.mul(inv, f) != Fp2::one() {
        return Err("minor determinant/inverse mismatch".into());
    }
    Ok(minor)
}
fn interpolation(
    f: &Field,
    points: &[(Fp2, BigUint)],
    m: usize,
    s: usize,
    d: usize,
    negative: bool,
) -> Result<(Matrix, Vec<usize>)> {
    if m == 0 || m > 6 || s == 0 || s > m || d > 1 || points.len() > PREFIX || points.len() < m {
        return Err("interpolation parameter admission".into());
    }
    let mut rows = vec![];
    let mut starts = vec![];
    for block in 0..points.len() / m {
        for offset in 0..=m - s {
            let t = block * m + offset;
            let x = &points[t].0;
            let mut powers = vec![Fp2::one()];
            for j in 1..=d + 4 {
                powers.push(powers[j - 1].mul(x, f));
            }
            let mut row = powers.clone();
            for (_, r) in points.iter().skip(t).take(s) {
                let mut y = Fp2 {
                    a: r.clone(),
                    b: 0u32.into(),
                };
                if negative {
                    y = y.neg(f)
                }
                row.extend(powers.iter().take(d + 1).map(|p| p.mul(&y, f)));
            }
            rows.push(row);
            starts.push(t);
        }
    }
    dimensions(f, &rows)?;
    Ok((rows, starts))
}
fn emit(out: &mut impl Write, v: Value) -> Result<()> {
    writeln!(out, "{v}").map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())
}
fn small_controls() -> Result<Value> {
    let f = Field::new(101u32.into())?;
    let swap = vec![vec![fp(0), fp(1)], vec![fp(1), fp(0)]];
    if determinant_lu(&f, &swap)? != fp(100) || determinant_bareiss(&f, &swap)? != fp(100) {
        return Err("row swap determinant control".into());
    }
    let points: Vec<_> = (1..=12)
        .map(|i| (fp(i), BigUint::from(i * i % 101)))
        .collect();
    let (a, _) = interpolation(&f, &points, 1, 1, 0, false)?;
    let r = reduce(&f, &a)?;
    let v = kernel(&f, &r, 6)?;
    check_kernel(&f, &a, &v)?;
    if r.pivots.len() != 5 {
        return Err("polynomial-stream known rank".into());
    }
    let explicit = vec![fp(0), fp(0), fp(100), fp(0), fp(0), fp(1)];
    check_kernel(&f, &a, &explicit)?;
    let det = determinant_lu(&f, &swap)?;
    let inv = det.inverse(&f)?;
    verify_minor(&f, &swap, &[0, 1], &det, &inv)?;
    let mut corrupt = det.clone();
    corrupt.a = f.add(&corrupt.a, &1u32.into());
    let refusals = [
        verify_minor(&f, &swap, &[0, 1], &corrupt, &inv).is_err(),
        verify_minor(&f, &swap, &[0, 0], &det, &inv).is_err(),
        verify_minor(&f, &swap, &[0, 2], &det, &inv).is_err(),
        verify_minor(&f, &swap, &[0], &det, &inv).is_err(),
        verify_minor(&f, &swap, &[0, 1], &det, &Fp2::one()).is_err(),
        interpolation(&f, &points, 0, 1, 0, false).is_err(),
        interpolation(&f, &points, 1, 2, 0, false).is_err(),
        interpolation(&f, &points, 1, 1, 2, false).is_err(),
    ];
    if refusals.iter().any(|x| !*x) {
        return Err("negative control accepted".into());
    }
    Ok(
        json!({"row_swap_determinant":encoded(&det),"polynomial_stream_rank":r.pivots.len(),"polynomial_stream_columns":6,"explicit_kernel":explicit.iter().map(encoded).collect::<Vec<_>>(),"computed_kernel":v.iter().map(encoded).collect::<Vec<_>>(),"negative_controls":refusals.len(),"all_refused":true}),
    )
}
pub fn run(out: &mut impl Write) -> Result<()> {
    emit(
        out,
        json!({"kind":"small_controls","checks":small_controls()?}),
    )?;
    let s = spec(true, 20633)?;
    let f = Field::new(s.p.clone())?;
    let curve = Curve {
        f: f.clone(),
        a: s.curve_a.clone().ok_or("A")?,
    };
    let gamma = s.h.pow(&s.order_factor.into(), &f);
    if gamma == Fp2::one() || gamma.pow(&s.q.into(), &f) != Fp2::one() {
        return Err("original coset step order control".into());
    }
    let member = Circuit::compile(&s)?;
    let table = IndexTable::new(&s)?;
    emit(
        out,
        json!({"kind":"public_spec","p":s.p.to_string(),"A":curve.a.to_string(),"q":s.q,"K":s.k,"a":s.order_factor,"h":encoded(&s.h),"gamma":encoded(&gamma),"j":1,"prefix":PREFIX,"index_table":table.stats(),"field":"Fp2=u^2-2","polynomial_message_bridge":false}),
    )?;
    let half = f.mul(&curve.a, &f.inv(&2u32.into())?);
    let center = f.sub(&0u32.into(), &half);
    let mut z = s.h.clone();
    let mut points = vec![];
    let mut seen = BTreeSet::new();
    for t in 0..PREFIX {
        if z.norm(&f) != BigUint::one()
            || z != s
                .h
                .pow(&(BigUint::one() + BigUint::from(s.order_factor) * t), &f)
            || !seen.insert((z.a.clone(), z.b.clone()))
        {
            return Err("original natural-index step replay".into());
        }
        let delta = f.mul(&half, &f.inv(&z.a)?);
        let mut candidates = vec![];
        for (branch, x) in [f.add(&center, &delta), f.sub(&center, &delta)]
            .into_iter()
            .enumerate()
        {
            if let Some(v) = f.sqrt(&curve.rhs(&x))? {
                if !v.is_zero() {
                    let nv = f.sub(&0u32.into(), &v);
                    candidates.push((branch, x, v.min(nv)));
                }
            }
        }
        if candidates.len() != 1 {
            return Err(format!("original raw branch not unique at t={t}"));
        }
        let (branch, x, y) = candidates.remove(0);
        let mut orientations = vec![];
        for (negative, v) in [(false, y.clone()), (true, f.sub(&0u32.into(), &y))] {
            let p = Some((x.clone(), v.clone()));
            let image = curve.times(&p, 8)?;
            if !curve.contains(&p) || image.is_none() {
                return Err("raw point/cofactor exclusion".into());
            }
            let input = WitnessInput {
                z: z.clone(),
                selector: 0,
                slack: s.k - 1,
                branch: BigUint::from(branch),
                raw_v: v.clone(),
            };
            let witness = member.witness(&input)?;
            let indexed = table.decode(&s, &member, &witness)?;
            if indexed.j != 1
                || indexed.t != t as u64
                || indexed.coefficient != if negative { -1 } else { 1 }
                || member.image(&witness)? != image
            {
                return Err("original member/index/image replay".into());
            }
            orientations.push(json!({"negative":negative,"point":[x.to_string(),v.to_string()],"image":image.map(|(u,v)|[u.to_string(),v.to_string()]),"indexed_coefficient":indexed.coefficient,"full_member_witness":witness.iter().map(ToString::to_string).collect::<Vec<_>>()}));
        }
        let received = f.sub(&0u32.into(), &f.mul(&x, &y));
        emit(
            out,
            json!({"kind":"original_point","t":t,"j":1,"z":encoded(&z),"branch":branch,"received_positive":received.to_string(),"orientations":orientations}),
        )?;
        points.push((z.clone(), received));
        z = z.mul(&gamma, &f);
    }
    let (mut full, mut deficient, mut mutation_refusals) = (0, 0, 0);
    for m in 1..=6 {
        for ss in 1..=m {
            for d in 0..=1 {
                let mut positive_det: Option<Fp2> = None;
                for negative in [false, true] {
                    let (matrix, starts) = interpolation(&f, &points, m, ss, d, negative)?;
                    let (_, n) = dimensions(&f, &matrix)?;
                    let reduced = reduce(&f, &matrix)?;
                    if reduced.pivots.len() == n {
                        let rows = &reduced.rows;
                        let minor: Matrix = rows.iter().map(|&i| matrix[i].clone()).collect();
                        let det = determinant_lu(&f, &minor)?;
                        let inv = det.inverse(&f)?;
                        verify_minor(&f, &matrix, rows, &det, &inv)?;
                        if negative {
                            let expected = positive_det
                                .as_ref()
                                .ok_or("positive determinant missing")?;
                            let expected = if ss * (d + 1) % 2 == 1 {
                                expected.neg(&f)
                            } else {
                                expected.clone()
                            };
                            if det != expected {
                                return Err("global-sign determinant parity".into());
                            }
                        } else {
                            positive_det = Some(det.clone());
                        }
                        let mut bad = det.clone();
                        bad.a = f.add(&bad.a, &1u32.into());
                        if verify_minor(&f, &matrix, rows, &bad, &inv).is_ok() {
                            return Err("corrupt determinant accepted".into());
                        }
                        mutation_refusals += 1;
                        emit(
                            out,
                            json!({"kind":"rank_case","m":m,"s":ss,"D":d,"negative":negative,"columns":n,"declared_rows":matrix.len(),"all_starts":starts,"rank":n,"full_column_rank":true,"selected_original_row_indices":rows,"selected_start_indices":rows.iter().map(|&i|starts[i]).collect::<Vec<_>>(),"matrix":encoded_matrix(&matrix),"minor":encoded_matrix(&minor),"determinant":encoded(&det),"determinant_inverse":encoded(&inv),"bareiss_replay":true,"corruption_refused":true}),
                        )?;
                        full += 1;
                    } else {
                        let v = kernel(&f, &reduced, n)?;
                        check_kernel(&f, &matrix, &v)?;
                        emit(
                            out,
                            json!({"kind":"rank_case","m":m,"s":ss,"D":d,"negative":negative,"columns":n,"declared_rows":matrix.len(),"all_starts":starts,"rank":reduced.pivots.len(),"full_column_rank":false,"matrix":encoded_matrix(&matrix),"kernel":v.iter().map(encoded).collect::<Vec<_>>(),"all_residuals_zero":true,"scope":"finite prefix only"}),
                        )?;
                        deficient += 1;
                    }
                }
            }
        }
    }
    if full + deficient != 84 {
        return Err("case coverage".into());
    }
    emit(
        out,
        json!({"kind":"complete","original_parameters":PREFIX,"original_signed_member_checks":2*PREFIX,"cases":84,"full_rank":full,"rank_deficient":deficient,"corrupt_determinant_refusals":mutation_refusals,"decoder_calls":0,"new_roots":0,"scope":"defined finite natural-order interpolation templates only"}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_controls() -> Result<()> {
        small_controls()?;
        Ok(())
    }
    #[test]
    fn zero_singular_and_noncanonical() -> Result<()> {
        let f = Field::new(101u32.into())?;
        let a = vec![vec![zero(); 3]; 3];
        let r = reduce(&f, &a)?;
        assert!(r.pivots.is_empty());
        check_kernel(&f, &a, &kernel(&f, &r, 3)?)?;
        assert_eq!(determinant_lu(&f, &a)?, zero());
        assert_eq!(determinant_bareiss(&f, &a)?, zero());
        let malformed = vec![vec![fp(101)]];
        assert!(reduce(&f, &malformed).is_err());
        assert!(determinant_bareiss(&f, &[vec![fp(1), fp(2)]]).is_err());
        Ok(())
    }
    #[test]
    fn three_by_three_and_extension() -> Result<()> {
        let f = Field::new(101u32.into())?;
        let a = vec![
            vec![fp(2), fp(3), fp(5)],
            vec![fp(7), fp(11), fp(13)],
            vec![fp(17), fp(19), fp(23)],
        ];
        // Direct cofactor expansion: -78 mod101 =23.
        assert_eq!(determinant_lu(&f, &a)?, fp(23));
        assert_eq!(determinant_bareiss(&f, &a)?, fp(23));
        let u = Fp2 {
            a: 0u32.into(),
            b: 1u32.into(),
        };
        let b = vec![vec![u.clone(), fp(1)], vec![fp(1), u]];
        assert_eq!(determinant_lu(&f, &b)?, fp(1));
        assert_eq!(determinant_bareiss(&f, &b)?, fp(1));
        Ok(())
    }
}
