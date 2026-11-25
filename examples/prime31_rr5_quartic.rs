//! Prime31 RR5 exact four-leaf algebraic gate.
//! Inputs to discovery are the PUBLIC target and factor-base x coordinates.
//! No group-sum tables or factor-base pair/quadruple enumeration.
//! The native F4 solver's solve() is deliberately not called: it can
//! enumerate all p field values as a fallback. Failure to triangularize
//! is Undetermined, not an empty-query certificate.
//! This first implementation caps B at 8: dense symbolic remainders
//! are not an acceptable B677 complexity claim.

use crypto_lib::cryptanalysis::f4_fp::{f4, normalise, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use crypto_lib::cryptanalysis::semaev_prime_poly::Fp;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const G: PrimePoint = PrimePoint {
    x: 625_037_715,
    y: 2_100_627_251,
    infinity: false,
};
type E = BTreeMap<[u32; 3], u64>;
const MAX_PARAMETER_ROOTS: usize = 4096;

fn c(x: u64) -> E {
    if x == 0 {
        E::new()
    } else {
        BTreeMap::from([([0; 3], x)])
    }
}
fn v(i: usize) -> E {
    let mut e = [0; 3];
    e[i] = 1;
    BTreeMap::from([(e, 1)])
}
fn add(a: &E, b: &E, f: Fp) -> E {
    let mut x = a.clone();
    for (&k, &t) in b {
        let z = f.add(x.get(&k).copied().unwrap_or(0), t);
        if z == 0 {
            x.remove(&k);
        } else {
            x.insert(k, z);
        }
    }
    x
}
fn scale(a: &E, k: u64, f: Fp) -> E {
    a.iter()
        .filter_map(|(&e, &z)| {
            let q = f.mul(z, k);
            (q != 0).then_some((e, q))
        })
        .collect()
}
fn sub(a: &E, b: &E, f: Fp) -> E {
    add(a, &scale(b, f.neg(1), f), f)
}
fn mul(a: &E, b: &E, f: Fp) -> Result<E, String> {
    let mut r = E::new();
    for (&x, &u) in a {
        for (&y, &w) in b {
            let z = [x[0] + y[0], x[1] + y[1], x[2] + y[2]];
            let s = f.add(r.get(&z).copied().unwrap_or(0), f.mul(u, w));
            if s == 0 {
                r.remove(&z);
            } else {
                r.insert(z, s);
            }
            if r.len() > 20_000 {
                return Err("dense three-variable monomial cap".into());
            }
        }
    }
    Ok(r)
}
fn ev(a: &E, sol: [u64; 3], f: Fp) -> u64 {
    a.iter().fold(0, |z, (e, &k)| {
        let mut t = k;
        for i in 0..3 {
            t = f.mul(t, f.pow(sol[i], e[i] as u64));
        }
        f.add(z, t)
    })
}
fn group() -> PrimeCurve {
    PrimeCurve { p: P, a: A, b: B }
}
fn scalar(curve: &PrimeCurve, mut p: PrimePoint, mut n: u64) -> PrimePoint {
    let mut acc = PrimePoint::INFINITY;
    let mut ops = GroupOps::default();
    while n > 0 {
        if n & 1 != 0 {
            acc = curve.add(&mut ops, acc, p);
        }
        n >>= 1;
        if n != 0 {
            p = curve.double(&mut ops, p);
        }
    }
    acc
}
fn base4() -> Vec<PrimePoint> {
    let curve = group();
    (1..=4).map(|k| scalar(&curve, G, k)).collect()
}
fn polynomial(xs: &[u64], f: Fp) -> Vec<u64> {
    let mut h = vec![1];
    for &x in xs {
        let mut nxt = vec![0; h.len() + 1];
        for (i, &z) in h.iter().enumerate() {
            nxt[i] = f.sub(nxt[i], f.mul(x, z));
            nxt[i + 1] = f.add(nxt[i + 1], z);
        }
        h = nxt;
    }
    h
}
fn quartic(r: PrimePoint, f: Fp) -> Result<[E; 5], String> {
    if r.infinity {
        return Err("RR5 finite R chart only".into());
    }
    // F=(x-u)(alpha+beta*x)+(y-v)(x+t), F(R)=0.
    // D=[v(x+t)-(x-u)(alpha+beta*x)]^2-fcurve(x)(x+t)^2
    //  = -(x-u)*M(x), M monic quartic.
    let (alpha, beta, t) = (v(0), v(1), v(2));
    let z1 = add(&sub(&c(r.y), &alpha, f), &scale(&beta, r.x, f), f);
    let z0 = add(&scale(&t, r.y, f), &scale(&alpha, r.x, f), f);
    let d4 = sub(&mul(&beta, &beta, f)?, &scale(&t, 2, f), f);
    let d3 = sub(
        &sub(
            &scale(&mul(&beta, &z1, f)?, f.neg(2), f),
            &mul(&t, &t, f)?,
            f,
        ),
        &c(A),
        f,
    );
    let d2 = sub(
        &sub(
            &sub(&mul(&z1, &z1, f)?, &scale(&mul(&beta, &z0, f)?, 2, f), f),
            &scale(&t, f.mul(2, A), f),
            f,
        ),
        &c(B),
        f,
    );
    let d1 = sub(
        &sub(
            &scale(&mul(&z1, &z0, f)?, 2, f),
            &scale(&mul(&t, &t, f)?, A, f),
            f,
        ),
        &scale(&t, f.mul(2, B), f),
        f,
    );
    let h4 = c(f.neg(1));
    let h3 = add(&d4, &scale(&h4, r.x, f), f);
    let h2 = add(&d3, &scale(&h3, r.x, f), f);
    let h1 = add(&d2, &scale(&h2, r.x, f), f);
    let h0 = add(&d1, &scale(&h1, r.x, f), f);
    Ok([
        scale(&h0, f.neg(1), f),
        scale(&h1, f.neg(1), f),
        scale(&h2, f.neg(1), f),
        scale(&h3, f.neg(1), f),
        c(1),
    ])
}
fn validate_chart(q: PrimePoint, base: &[PrimePoint]) -> Result<(), String> {
    let curve = group();
    if q.infinity {
        return Err("unsupported chart: identity target".into());
    }
    if q.x >= P || q.y >= P || !curve.is_on_curve(q) {
        return Err("invalid canonical public target".into());
    }
    if !(4..=8).contains(&base.len()) {
        return Err("supported development base size is 4..=8".into());
    }
    if base
        .iter()
        .any(|&pt| pt.infinity || pt.x >= P || pt.y >= P || pt.y == 0 || !curve.is_on_curve(pt))
    {
        return Err("base needs canonical nonidentity nonzero-y points".into());
    }
    let mut xs = base.iter().map(|pt| pt.x).collect::<Vec<_>>();
    xs.sort_unstable();
    if xs.windows(2).any(|p| p[0] == p[1]) {
        return Err("base needs distinct abscissae".into());
    }
    if xs.binary_search(&q.x).is_ok() {
        return Err("unsupported chart: target abscissa belongs to the base".into());
    }
    Ok(())
}
fn system(q: PrimePoint, base: &[PrimePoint]) -> Result<Vec<Poly>, String> {
    validate_chart(q, base)?;
    let curve = group();
    let f = Fp::new(P);
    let xs = base.iter().map(|pt| pt.x).collect::<Vec<_>>();
    let m = quartic(curve.neg(q), f)?;
    let h = polynomial(&xs, f);
    let mut t = h.into_iter().map(c).collect::<Vec<_>>();
    for i in (4..t.len()).rev() {
        let a = t[i].clone();
        for j in 0..5 {
            t[i - 4 + j] = sub(&t[i - 4 + j], &mul(&a, &m[j], f)?, f);
        }
    }
    Ok(t.iter()
        .take(4)
        .map(|x| {
            let poly: Poly = x.iter().map(|(e, &k)| (e.to_vec(), k)).collect();
            normalise(&poly, P, Ordering::Lex)
        })
        .collect())
}
fn poly_eval(poly: &Poly, z: [u64; 3], f: Fp) -> u64 {
    poly.iter().fold(0, |sum, (e, a)| {
        let t = e
            .iter()
            .enumerate()
            .fold(*a, |v, (i, &n)| f.mul(v, f.pow(z[i], n as u64)));
        f.add(sum, t)
    })
}
/// Return a (variable,dense coefficients) univariate consequence of a
/// completed F4 basis after substituting earlier roots. There is NO
/// enumeration of an unconstrained field coordinate.
fn find_uni(basis: &[Poly], a: &[Option<u64>; 3], f: Fp) -> Result<Option<(usize, Vec<u64>)>, ()> {
    let mut best: Option<(usize, Vec<u64>)> = None;
    for g in basis {
        // Specialization can kill terms or make distinct monomials cancel.
        // Determine the support only after collecting the specialized terms.
        let mut specialized = E::new();
        for (e, coeff) in g {
            let mut t = *coeff;
            let mut exponent = [e[0], e[1], e[2]];
            for i in 0..3 {
                if let Some(x) = a[i] {
                    t = f.mul(t, f.pow(x, e[i] as u64));
                    exponent[i] = 0;
                }
            }
            let value = f.add(specialized.get(&exponent).copied().unwrap_or(0), t);
            if value == 0 {
                specialized.remove(&exponent);
            } else {
                specialized.insert(exponent, value);
            }
        }
        let used = specialized
            .keys()
            .flat_map(|e| (0..3).filter(move |&i| e[i] != 0))
            .collect::<BTreeSet<_>>();
        if used.is_empty() {
            if !specialized.is_empty() {
                return Err(());
            }
            continue;
        }
        if used.len() != 1 {
            continue;
        }
        let i = *used.iter().next().unwrap();
        let degree = specialized.keys().map(|e| e[i]).max().unwrap() as usize;
        let mut d = vec![0u64; degree + 1];
        for (e, t) in specialized {
            d[e[i] as usize] = t;
        }
        while d.last() == Some(&0) {
            d.pop();
        }
        if d.len() == 1 && d[0] != 0 {
            return Err(());
        }
        if d.len() > 1 && best.as_ref().is_none_or(|(_, old)| d.len() < old.len()) {
            best = Some((i, d));
        }
    }
    Ok(best)
}
fn triangular(
    basis: &[Poly],
    input: &[Poly],
    a: &mut [Option<u64>; 3],
    roots: &mut BTreeSet<[u64; 3]>,
    muls: &mut u64,
    deadline: Instant,
    max_roots: usize,
) -> Result<(), String> {
    if Instant::now() >= deadline {
        return Err("triangular deadline".into());
    }
    if a.iter().all(Option::is_some) {
        let z = [a[0].unwrap(), a[1].unwrap(), a[2].unwrap()];
        if input.iter().all(|g| poly_eval(g, z, Fp::new(P)) == 0) {
            if !roots.contains(&z) && roots.len() >= max_roots {
                return Err("parameter output cap".into());
            }
            roots.insert(z);
        }
        return Ok(());
    }
    let (i, g) = match find_uni(basis, a, Fp::new(P)) {
        Err(()) => return Ok(()),
        Ok(Some(t)) => t,
        Ok(None) => return Err("no triangular polynomial: refuse field scan".into()),
    };
    let values = poly_roots_fp_counted_until(&g, P, 0x52523546524F4D, &mut *muls, Some(deadline))
        .ok_or("rootfactor deadline")?;
    if values.len() > 10000 {
        return Err("root branch cap".into());
    }
    for x in values {
        a[i] = Some(x);
        let r = triangular(basis, input, a, roots, muls, deadline, max_roots);
        a[i] = None;
        r?;
    }
    Ok(())
}
fn extract(input: &[Poly], deadline: Instant) -> Result<(Vec<[u64; 3]>, u64, u64), String> {
    if Instant::now() >= deadline {
        return Err("extraction deadline".into());
    }
    let mut opts = F4Options::new(Ordering::Lex, 12);
    opts.deadline = Some(deadline);
    let rep = f4(input, 3, P, &opts);
    if rep.inconsistent {
        return Ok((vec![], rep.field_ops, 0));
    }
    if rep.timed_out || rep.pairs_above_bound > 0 {
        return Err(format!(
            "F4 censored: timed_out={} pairs_above_bound={}",
            rep.timed_out, rep.pairs_above_bound
        ));
    }
    let mut roots = BTreeSet::new();
    let mut a = [None; 3];
    let mut ops = 0;
    triangular(
        &rep.basis,
        input,
        &mut a,
        &mut roots,
        &mut ops,
        deadline,
        MAX_PARAMETER_ROOTS,
    )?;
    Ok((roots.into_iter().collect(), rep.field_ops, ops))
}
/// Recover signed leaves from every completed algebraic coefficient solution.
fn signed_lift(
    q: PrimePoint,
    base: &[PrimePoint],
    params: [u64; 3],
    root_muls: &mut u64,
    deadline: Instant,
) -> Result<Vec<(usize, i8)>, String> {
    validate_chart(q, base)?;
    if Instant::now() >= deadline {
        return Err("signed lift deadline".into());
    }
    if params.iter().any(|&x| x >= P) {
        return Err("noncanonical coefficient root".into());
    }
    let curve = group();
    let f = Fp::new(P);
    let r = curve.neg(q);
    let m = quartic(r, f)?;
    let coeff = m.iter().map(|p| ev(p, params, f)).collect::<Vec<_>>();
    if coeff[4] != 1 {
        return Err("quartic is not monic".into());
    }
    let roots =
        poly_roots_fp_counted_until(&coeff, P, 0x464F55524C454146, root_muls, Some(deadline))
            .ok_or("quartic root-factor deadline")?;
    if roots.len() != 4 {
        return Err("quartic does not have four distinct rational roots".into());
    }
    let mut sum = PrimePoint::INFINITY;
    let mut steps = GroupOps::default();
    let mut row = Vec::new();
    for x in roots {
        if Instant::now() >= deadline {
            return Err("signed lift deadline".into());
        }
        let i = base
            .iter()
            .position(|p| p.x == x)
            .ok_or("quartic root outside factor base")?;
        let den = f.add(x, params[2]);
        if den == 0 || x == r.x {
            return Err("unsupported denominator/target-abscissa chart".into());
        }
        let u = f.add(params[0], f.mul(params[1], x));
        let y = f.sub(r.y, f.mul(f.mul(f.sub(x, r.x), u), f.inv(den)));
        let p = PrimePoint::affine(x, y);
        if !curve.is_on_curve(p) {
            return Err("lifted point is off curve".into());
        }
        let sign = if p == base[i] {
            1
        } else if p == curve.neg(base[i]) {
            -1
        } else {
            return Err("lift is not a signed factor-base point".into());
        };
        row.push((i, sign));
        sum = curve.add(&mut steps, sum, p);
    }
    if sum != q {
        return Err("signed point-sum replay failed".into());
    }
    row.sort_unstable();
    Ok(row)
}
/// A complete coefficient extraction is not a complete row extraction if any
/// candidate fails to lift. Preserve that failure rather than filtering it out.
fn lift_all(
    q: PrimePoint,
    base: &[PrimePoint],
    input: &[Poly],
    params: Vec<[u64; 3]>,
    root_muls: &mut u64,
    deadline: Instant,
) -> Result<BTreeSet<Vec<(usize, i8)>>, String> {
    validate_chart(q, base)?;
    if Instant::now() >= deadline {
        return Err("signed row extraction deadline".into());
    }
    let mut rows = BTreeSet::new();
    for p in params {
        if Instant::now() >= deadline {
            return Err("signed row extraction deadline".into());
        }
        if !input.iter().all(|g| poly_eval(g, p, Fp::new(P)) == 0) {
            return Err("coefficient assignment failed original-input replay".into());
        }
        rows.insert(signed_lift(q, base, p, root_muls, deadline)?);
    }
    Ok(rows)
}
/// Sparse signed factors become an exact RHS-bearing row, with the unknown
/// target log in the final variable: SUM(sign_i * log(P_i)) - log(Q) = k.
/// For query Q+[k]G supply k; this pilot's k=0. No known factor logs enter.
const N: u64 = 2_147_446_831;
struct RelationRank {
    width: usize,
    base: Vec<PrimePoint>,
    target: PrimePoint,
    pivots: BTreeMap<usize, Vec<u64>>,
}
impl RelationRank {
    fn new(base: &[PrimePoint], target: PrimePoint) -> Result<Self, String> {
        validate_chart(target, base)?;
        Ok(Self {
            width: base.len() + 1,
            base: base.to_vec(),
            target,
            pivots: BTreeMap::new(),
        })
    }
    fn ingest(&mut self, terms: &[(usize, i8)], shift: u64) -> Result<bool, String> {
        // Bind every row to this accumulator's actual public equation. Rank
        // alone does not authenticate a sign, target, base, or claimed RHS.
        if terms.len() != 4 {
            return Err("RR5 row needs exactly four leaves".into());
        }
        let curve = group();
        let mut group_ops = GroupOps::default();
        let mut sum = PrimePoint::INFINITY;
        let mut used = BTreeSet::new();
        for &(i, sign) in terms {
            if i >= self.base.len() || ![-1, 1].contains(&sign) || !used.insert(i) {
                return Err("malformed distinct signed coefficient".into());
            }
            let point = if sign == 1 {
                self.base[i]
            } else {
                curve.neg(self.base[i])
            };
            sum = curve.add(&mut group_ops, sum, point);
        }
        let shifted = curve.add(&mut group_ops, self.target, scalar(&curve, G, shift % N));
        if sum != shifted {
            return Err("public RHS point equation failed".into());
        }
        let f = Fp::new(N);
        let mut row = vec![0; self.width + 1];
        for &(i, sign) in terms {
            if i >= self.width - 1 || ![-1, 1].contains(&sign) {
                return Err("malformed signed coefficient".into());
            }
            row[i] = f.add(row[i], if sign > 0 { 1 } else { f.neg(1) });
        }
        row[self.width - 1] = f.neg(1);
        row[self.width] = shift % N;
        for (&j, pivot) in &self.pivots {
            let t = row[j];
            if t == 0 {
                continue;
            }
            for k in j..=self.width {
                row[k] = f.sub(row[k], f.mul(t, pivot[k]));
            }
        }
        let Some(j) = (0..self.width).find(|&j| row[j] != 0) else {
            if row[self.width] != 0 {
                return Err("inconsistent genuine RHS".into());
            }
            return Ok(false);
        };
        let inv = f.inv(row[j]);
        for k in j..=self.width {
            row[k] = f.mul(row[k], inv);
        }
        self.pivots.insert(j, row);
        Ok(true)
    }
    fn rank(&self) -> usize {
        self.pivots.len()
    }
}

fn main() {
    // Known-answer pilot exercises original prime-order curve only;
    // G multiples must not be used for production index-calculus timing.
    let base = base4();
    let c = group();
    let q = scalar(&c, G, 10);
    let deadline = Instant::now() + Duration::from_secs(5);
    let input = system(q, &base).expect("public system");
    match extract(&input, deadline) {
        Ok((params, f4_mul, mut root_mul)) => {
            match lift_all(q,&base,&input,params,&mut root_mul,deadline){
                Ok(rows)=>{
                    let mut rank=RelationRank::new(&base,q).expect("public relation instance");
                    for row in &rows { rank.ingest(row,0).expect("verified relation rank"); }
                    let status=if rows.is_empty(){"exact_empty_chart"}else{"exact_rows_chart"};
                    println!("PRIME31_RR5 p={P} B=4 status={status} algebraic_complete=true signed_rows={} independent_rank={} f4_reduction_muls={f4_mul} root_factor_muls={root_mul} phase_accounting_complete=false (development, no cold ECDLP)",rows.len(),rank.rank());
                }
                Err(reason)=>println!("PRIME31_RR5 p={P} B=4 status=undetermined algebraic_complete=false reason={reason}"),
            }
        }
        Err(reason) => println!(
            "PRIME31_RR5 p={P} B=4 status=undetermined algebraic_complete=false reason={reason}"
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // Test-only certificate from an independent three-by-three interpolation,
    // not a supplied hint to the public algebraic extractor.
    fn witness_parameters(points: &[PrimePoint], q: PrimePoint) -> [u64; 3] {
        let curve = group();
        let f = Fp::new(P);
        let r = curve.neg(q);
        let mut matrix = [[0u64; 4]; 3];
        for (i, p) in points.iter().take(3).enumerate() {
            let dx = f.sub(p.x, r.x);
            let dy = f.sub(p.y, r.y);
            matrix[i] = [dx, f.mul(p.x, dx), dy, f.neg(f.mul(p.x, dy))];
        }
        for j in 0..3 {
            let pivot = (j..3).find(|&k| matrix[k][j] != 0).unwrap();
            matrix.swap(j, pivot);
            let inv = f.inv(matrix[j][j]);
            for k in j..4 {
                matrix[j][k] = f.mul(matrix[j][k], inv);
            }
            for i in 0..3 {
                if i == j {
                    continue;
                }
                let z = matrix[i][j];
                for k in j..4 {
                    matrix[i][k] = f.sub(matrix[i][k], f.mul(z, matrix[j][k]));
                }
            }
        }
        [matrix[0][3], matrix[1][3], matrix[2][3]]
    }
    #[test]
    fn symbolic_quartic_has_target_and_witness_roots() {
        let curve = group();
        let f = Fp::new(P);
        let base = base4();
        let q = scalar(&curve, G, 10);
        let r = curve.neg(q);
        let z = witness_parameters(&base, q);
        let m = quartic(r, f).unwrap();
        let co = m.iter().map(|v| ev(v, z, f)).collect::<Vec<_>>();
        assert_eq!(
            co,
            polynomial(&base.iter().map(|v| v.x).collect::<Vec<_>>(), f)
        );
        let s = system(q, &base).unwrap();
        assert!(s.iter().all(|p| poly_eval(p, z, f) == 0));
        let row =
            signed_lift(q, &base, z, &mut 0, Instant::now() + Duration::from_secs(5)).unwrap();
        assert_eq!(row, vec![(0, 1), (1, 1), (2, 1), (3, 1)]);
        let mut rank = RelationRank::new(&base, q).unwrap();
        assert_eq!(rank.ingest(&row, 0), Ok(true));
        assert_eq!(rank.ingest(&row, 0), Ok(false));
        assert_eq!(rank.rank(), 1);
    }
    #[test]
    fn poisoning_changes_public_equations() {
        let curve = group();
        let base = base4();
        let q = scalar(&curve, G, 10);
        let wrong = scalar(&curve, G, 17);
        let z = witness_parameters(&base, q);
        let wrong_input = system(wrong, &base).unwrap();
        assert_ne!(system(q, &base).unwrap(), wrong_input);
        assert!(wrong_input.iter().any(|p| poly_eval(p, z, Fp::new(P)) != 0));
        assert!(lift_all(
            wrong,
            &base,
            &wrong_input,
            vec![z],
            &mut 0,
            Instant::now() + Duration::from_secs(5)
        )
        .is_err());
        assert!(system(PrimePoint::INFINITY, &base).is_err());
    }
    #[test]
    fn target_in_base_is_unsupported_even_when_a_signed_relation_exists() {
        let curve = group();
        let base = base4();
        let q = base[1];
        let mut ops = GroupOps::default();
        let mut sum = PrimePoint::INFINITY;
        for &(i, sign) in &[(0, 1), (1, 1), (2, 1), (3, -1)] {
            let point = if sign == 1 {
                base[i]
            } else {
                curve.neg(base[i])
            };
            sum = curve.add(&mut ops, sum, point);
        }
        assert_eq!(sum, q); // G+2G+3G-4G=2G: this is not an empty target.
        assert!(system(q, &base).unwrap_err().contains("unsupported chart"));
        assert!(system(curve.neg(q), &base)
            .unwrap_err()
            .contains("unsupported chart"));
    }
    #[test]
    fn noncanonical_invalid_and_duplicate_base_inputs_are_rejected() {
        let curve = group();
        let base = base4();
        let q = scalar(&curve, G, 10);
        let mut shifted = q;
        shifted.x += P;
        assert!(system(shifted, &base).is_err());
        shifted = q;
        shifted.y += P;
        assert!(system(shifted, &base).is_err());
        let mut bad = base.clone();
        bad[0].x += P;
        assert!(system(q, &bad).is_err());
        bad = base.clone();
        bad[3] = curve.neg(base[0]);
        assert!(system(q, &bad).is_err());
        bad = base.clone();
        bad[0] = PrimePoint::INFINITY;
        assert!(system(q, &bad).is_err());
        assert!(system(q, &base[..3]).is_err());
    }
    #[test]
    fn specialized_zero_terms_and_cancellation_expose_univariates() {
        let f = Fp::new(P);
        // At x0=0, x0*x1+x2-1 specializes to x2-1.
        let killed: Poly = vec![
            (vec![1, 1, 0], 1),
            (vec![0, 0, 1], 1),
            (vec![0, 0, 0], P - 1),
        ];
        assert_eq!(
            find_uni(&[killed], &[Some(0), None, None], f),
            Ok(Some((2, vec![P - 1, 1])))
        );
        // At x0=1, x0*x1-x1+x2-1 has the same specialization.
        let cancelled: Poly = vec![
            (vec![1, 1, 0], 1),
            (vec![0, 1, 0], P - 1),
            (vec![0, 0, 1], 1),
            (vec![0, 0, 0], P - 1),
        ];
        assert_eq!(
            find_uni(&[cancelled], &[Some(1), None, None], f),
            Ok(Some((2, vec![P - 1, 1])))
        );
        let contradiction: Poly = vec![
            (vec![1, 1, 0], 1),
            (vec![0, 1, 0], P - 1),
            (vec![0, 0, 0], 1),
        ];
        assert_eq!(
            find_uni(&[contradiction], &[Some(1), None, None], f),
            Err(())
        );
    }
    #[test]
    fn incomplete_branches_and_limits_never_become_empty_results() {
        let future = Instant::now() + Duration::from_secs(5);
        let mut assignments = [None; 3];
        let mut roots = BTreeSet::new();
        let err =
            triangular(&[], &[], &mut assignments, &mut roots, &mut 0, future, 1).unwrap_err();
        assert!(err.contains("refuse field scan"));
        let mut complete = [Some(1), Some(2), Some(3)];
        assert!(
            triangular(&[], &[], &mut complete, &mut roots, &mut 0, future, 0)
                .unwrap_err()
                .contains("output cap")
        );
        assert!(roots.is_empty());
        assert!(extract(&[], Instant::now())
            .unwrap_err()
            .contains("deadline"));
        let base = base4();
        let q = scalar(&group(), G, 10);
        let z = witness_parameters(&base, q);
        assert!(signed_lift(q, &base, z, &mut 0, Instant::now())
            .unwrap_err()
            .contains("deadline"));
    }
    #[test]
    fn genuine_rhs_rank_rejects_sign_and_rhs_poisoning_before_mutation() {
        let base = base4();
        let q = scalar(&group(), G, 10);
        let mut rank = RelationRank::new(&base, q).unwrap();
        let row = vec![(0, 1), (1, 1), (2, 1), (3, 1)];
        assert_eq!(rank.ingest(&row, 0), Ok(true));
        assert!(rank.ingest(&row, 1).is_err());
        let mut poison = row.clone();
        poison[0].1 = -1;
        assert!(rank.ingest(&poison, 0).is_err());
        let mut duplicate = row.clone();
        duplicate[3].0 = 0;
        assert!(rank.ingest(&duplicate, 0).is_err());
        assert_eq!(rank.rank(), 1);
        assert_eq!(rank.ingest(&row, 0), Ok(false));
        // Sixteen genuinely different signed rows span only five coefficient
        // dimensions. The test oracle knows fixture scalars; the rank adapter
        // receives no factor-base logarithms and replays public point equations.
        let mut independent = 1;
        for mask in 0..16 {
            let terms: Vec<(usize, i8)> = (0..4)
                .map(|i| (i, if mask & (1 << i) == 0 { 1 } else { -1 }))
                .collect();
            let sum = terms
                .iter()
                .map(|&(i, s)| (i as i64 + 1) * s as i64)
                .sum::<i64>();
            let shift = (sum - 10).rem_euclid(N as i64) as u64;
            independent += usize::from(rank.ingest(&terms, shift).unwrap());
        }
        assert_eq!(independent, 5);
        assert_eq!(rank.rank(), 5);
    }
    #[test]
    fn public_b4_solver_matches_test_only_signed_scalar_oracle() {
        let base = base4();
        let curve = group();
        for scalar_target in [10, N - 10, 17] {
            let q = scalar(&curve, G, scalar_target);
            let input = system(q, &base).unwrap();
            let deadline = Instant::now() + Duration::from_secs(5);
            let (params, _, mut root_muls) = extract(&input, deadline)
                .expect("B4 algebraic extraction must finish for this correctness gate");
            let actual = lift_all(q, &base, &input, params, &mut root_muls, deadline)
                .expect("every algebraic assignment must lift and replay");
            let mut expected = BTreeSet::new();
            for mask in 0..16 {
                let terms: Vec<(usize, i8)> = (0..4)
                    .map(|i| (i, if mask & (1 << i) == 0 { 1 } else { -1 }))
                    .collect();
                let sum = terms
                    .iter()
                    .map(|&(i, s)| (i as i64 + 1) * s as i64)
                    .sum::<i64>();
                if sum.rem_euclid(N as i64) as u64 == scalar_target {
                    expected.insert(terms);
                }
            }
            assert_eq!(actual, expected, "public target scalar {scalar_target}");
        }
    }
}
