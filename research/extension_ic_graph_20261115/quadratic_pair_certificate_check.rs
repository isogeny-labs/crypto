//! Independent u128 coefficient arithmetic for principal cofactor certificates.
//! No producer constructor, field implementation or graph algorithm is called.
use serde::Deserialize;
use serde_json::{json, Value};
type E = [u64; 2];
type P = Vec<E>;
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
pub(crate) fn fpow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = ((r as u128 * a as u128) % p as u128) as u64;
        }
        a = ((a as u128 * a as u128) % p as u128) as u64;
        e >>= 1;
    }
    r
}
pub(crate) fn prime(p: u64) -> bool {
    p >= 2
        && (2..)
            .take_while(|d| d * d <= p)
            .all(|d| !p.is_multiple_of(d))
}
#[derive(Clone, Copy)]
struct F {
    p: u64,
    w: u64,
}
impl F {
    fn add(self, a: E, b: E) -> E {
        [(a[0] + b[0]) % self.p, (a[1] + b[1]) % self.p]
    }
    fn sub(self, a: E, b: E) -> E {
        [
            (a[0] + self.p - b[0]) % self.p,
            (a[1] + self.p - b[1]) % self.p,
        ]
    }
    fn mul(self, a: E, b: E) -> E {
        let p = self.p as u128;
        [
            ((a[0] as u128 * b[0] as u128 + self.w as u128 * a[1] as u128 * b[1] as u128) % p)
                as u64,
            ((a[0] as u128 * b[1] as u128 + a[1] as u128 * b[0] as u128) % p) as u64,
        ]
    }
    fn inv(self, a: E) -> Result<E, String> {
        let d = (a[0] as u128 * a[0] as u128 + self.p as u128 * self.p as u128
            - self.w as u128 * a[1] as u128 * a[1] as u128 % (self.p as u128))
            % self.p as u128;
        ensure(d != 0, "inverse of zero")?;
        let r = fpow(d as u64, self.p - 2, self.p);
        Ok([
            ((a[0] as u128 * r as u128) % self.p as u128) as u64,
            ((self.p - a[1]) as u128 * r as u128 % self.p as u128) as u64,
        ])
    }
}
fn trim(mut a: P) -> P {
    while a.last() == Some(&[0, 0]) {
        a.pop();
    }
    a
}
fn add(f: F, a: &[E], b: &[E]) -> P {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| f.add(*a.get(i).unwrap_or(&[0, 0]), *b.get(i).unwrap_or(&[0, 0])))
            .collect(),
    )
}
fn sub(f: F, a: &[E], b: &[E]) -> P {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| f.sub(*a.get(i).unwrap_or(&[0, 0]), *b.get(i).unwrap_or(&[0, 0])))
            .collect(),
    )
}
fn mul(f: F, a: &[E], b: &[E]) -> P {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![[0, 0]; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = f.add(r[i + j], f.mul(x, y));
        }
    }
    trim(r)
}
fn rem(f: F, a: &[E], b: &[E]) -> Result<P, String> {
    ensure(
        !b.is_empty() && b.last() != Some(&[0, 0]),
        "nonzero polynomial divisor",
    )?;
    let inverse = f.inv(*b.last().unwrap())?;
    let mut r = trim(a.to_vec());
    while r.len() >= b.len() {
        let shift = r.len() - b.len();
        let c = f.mul(*r.last().unwrap(), inverse);
        for (j, &v) in b.iter().enumerate() {
            r[shift + j] = f.sub(r[shift + j], f.mul(c, v));
        }
        r = trim(r);
    }
    Ok(r)
}
fn gcd(f: F, a: &[E], b: &[E]) -> Result<P, String> {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !b.is_empty() {
        let r = rem(f, &a, &b)?;
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let inv = f.inv(lc)?;
        a = a.into_iter().map(|x| f.mul(x, inv)).collect();
    }
    Ok(a)
}
fn eval(f: F, a: &[E], x: u64) -> E {
    a.iter()
        .rev()
        .fold([0, 0], |v, &c| f.add(f.mul(v, [x, 0]), c))
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Remainder {
    key: [u64; 6],
    coefficient: i8,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Certificate {
    terms: Vec<(usize, i8)>,
    remainders: Vec<Remainder>,
    a: P,
    c: P,
    norm: Vec<u64>,
}
pub struct Domain {
    f: F,
    h: P,
    base: Vec<(u64, E)>,
}
#[derive(Deserialize)]
struct LineInput {
    a0: Vec<u64>,
    a1: Vec<u64>,
    b: P,
    s: u64,
}
pub struct LineProof {
    pub a: P,
    pub c: P,
    pub norm: Vec<u64>,
    pub tail: Vec<u64>,
    pub terms: Vec<(usize, i8)>,
}
impl Domain {
    pub fn new(p: u64, w: u64, h: P, base: Vec<(u64, E)>) -> Result<Self, String> {
        ensure(
            (5..=1511).contains(&p) && prime(p) && w < p && fpow(w, (p - 1) / 2, p) == p - 1,
            "admitted odd field",
        )?;
        let f = F { p, w };
        ensure(
            h.len() == 8 && h[7] == [1, 0] && h.iter().flatten().all(|&x| x < p),
            "monic canonical genus-three curve",
        )?;
        let derivative: P = h
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &x)| f.mul(x, [i as u64 % p, 0]))
            .collect();
        ensure(
            gcd(f, &h, &derivative)? == vec![[1, 0]],
            "curve nonsingularity",
        )?;
        ensure(
            base.windows(2).all(|w| w[0].0 < w[1].0),
            "ordered distinct base",
        )?;
        for &(x, y) in &base {
            ensure(
                x < p && y != [0, 0] && y.iter().all(|&v| v < p) && f.mul(y, y) == eval(f, &h, x),
                "base ordinate",
            )?;
        }
        Ok(Self { f, h, base })
    }
    /// Reconstruct the line, norm and every small root with independent
    /// coefficient arithmetic. The base scan is exhaustive, not a root oracle.
    pub fn line(&self, raw: &Value) -> Result<LineProof, String> {
        let line: LineInput = serde_json::from_value(raw.clone()).map_err(|e| e.to_string())?;
        let f = self.f;
        let p = f.p;
        ensure(
            line.s > 0
                && line.s < p
                && !line.a0.is_empty()
                && line.a0.len() <= 5
                && line.a0.last() == Some(&1)
                && line.a1.len() <= 5
                && line.b.len() == 2
                && line.a0.iter().chain(&line.a1).all(|&x| x < p)
                && line.b.iter().flatten().all(|&x| x < p),
            "line shape/canonicality",
        )?;
        let b2h = mul(f, &mul(f, &line.b, &line.b), &self.h);
        let twice = trim(
            mul(f, &embed(&line.a0), &embed(&line.a1))
                .into_iter()
                .map(|x| f.mul([2, 0], x))
                .collect(),
        );
        let imaginary = trim(b2h.iter().map(|x| [x[1], 0]).collect());
        ensure(twice == imaginary, "independent line imaginary constraint")?;
        let root = |n| (0..p).find(|&x| (x as u128 * x as u128 % p as u128) as u64 == n);
        let sq = if let Some(r) = root(line.s) {
            [r, 0]
        } else {
            let n = (line.s as u128 * fpow(f.w, p - 2, p) as u128 % p as u128) as u64;
            [0, root(n).ok_or("independent quadratic sqrt")?]
        };
        let a = trim(
            (0..5)
                .map(|i| {
                    [
                        *line.a0.get(i).unwrap_or(&0),
                        (line.s as u128 * *line.a1.get(i).unwrap_or(&0) as u128 % p as u128) as u64,
                    ]
                })
                .collect(),
        );
        let c = trim(line.b.iter().map(|&x| f.mul(sq, x)).collect());
        let norm_e = sub(f, &mul(f, &a, &a), &mul(f, &mul(f, &c, &c), &self.h));
        ensure(norm_e.len() == 10, "independent degree-nine line norm")?;
        let norm = fp_coefficients(&norm_e)?;
        let fp_norm = embed(&norm);
        let mut tail = norm_e;
        let mut terms = Vec::new();
        for (i, &(x, y)) in self.base.iter().enumerate() {
            if eval(f, &fp_norm, x) != [0, 0] {
                continue;
            }
            ensure(tail.len() >= 2, "independent root multiplicity")?;
            let mut q = vec![[0, 0]; tail.len() - 1];
            for j in (1..tail.len()).rev() {
                q[j - 1] = tail[j];
                tail[j - 1] = f.add(tail[j - 1], f.mul([x, 0], q[j - 1]));
            }
            ensure(tail[0] == [0, 0], "independent exact root division")?;
            tail = trim(q);
            let den = eval(f, &c, x);
            let ordinate = f.sub([0, 0], f.mul(eval(f, &a, x), f.inv(den)?));
            let sign = if ordinate == y {
                1
            } else {
                ensure(ordinate == f.sub([0, 0], y), "independent line ordinate")?;
                -1
            };
            terms.push((i, sign));
        }
        Ok(LineProof {
            a,
            c,
            norm,
            tail: fp_coefficients(&tail)?,
            terms,
        })
    }
    /// Qualifier-only reconstruction. Solves the two coefficient equations for
    /// C inverse modulo U directly, then checks the complete principal identity.
    /// No producer certificate constructor or low-degree inverse is invoked.
    pub fn candidate(
        &self,
        line: &LineProof,
        split: Option<&[Vec<u64>; 2]>,
    ) -> Result<Value, String> {
        let f = self.f;
        let p = f.p;
        let factors = match line.tail.len() {
            1 => Vec::new(),
            3 => {
                let inv = fpow(line.tail[2], p - 2, p);
                vec![line
                    .tail
                    .iter()
                    .map(|&x| (x as u128 * inv as u128 % p as u128) as u64)
                    .collect()]
            }
            5 => split.ok_or("no admitted quartic split")?.to_vec(),
            _ => return Err("independent candidate support arity".into()),
        };
        let mut classes = Vec::new();
        for u in &factors {
            ensure(
                u.len() == 3 && u[2] == 1 && u.iter().all(|&x| x < p),
                "independent candidate factor",
            )?;
            ensure(
                line.c.len() == 2,
                "independent candidate denominator degree",
            )?;
            let c0 = line.c[0];
            let c1 = line.c[1];
            let a = f.sub(c0, f.mul(c1, [u[1], 0]));
            let determinant = f.add(f.mul(c0, a), f.mul(f.mul(c1, c1), [u[0], 0]));
            let det_inv = f.inv(determinant)?;
            let inverse = vec![f.mul(a, det_inv), f.sub([0, 0], f.mul(c1, det_inv))];
            let modulus = embed(u);
            let v: P = rem(f, &mul(f, &line.a, &inverse), &modulus)?
                .into_iter()
                .map(|x| f.sub([0, 0], x))
                .collect();
            let v0 = *v.first().unwrap_or(&[0, 0]);
            let v1 = *v.get(1).unwrap_or(&[0, 0]);
            let n0 = f.sub([0, 0], v0);
            let n1 = f.sub([0, 0], v1);
            let key = [u[0], u[1], v0[0], v0[1], v1[0], v1[1]];
            let negative = [u[0], u[1], n0[0], n0[1], n1[0], n1[1]];
            let (key, coefficient) = if key <= negative {
                (key, 1i8)
            } else {
                (negative, -1i8)
            };
            classes.push(json!({"key":key,"coefficient":coefficient}));
        }
        let raw =
            json!({"terms":line.terms,"remainders":classes,"a":line.a,"c":line.c,"norm":line.norm});
        self.check(&raw)?;
        Ok(raw)
    }
    pub fn check(&self, raw: &Value) -> Result<Value, String> {
        let row: Certificate = serde_json::from_value(raw.clone()).map_err(|e| e.to_string())?;
        let f = self.f;
        let p = f.p;
        ensure(
            row.remainders.len() <= 2
                && row.terms.len() + 2 * row.remainders.len() == 9
                && row.terms.windows(2).all(|w| w[0].0 < w[1].0)
                && row.a.len() <= 5
                && row.c.len() == 2
                && row.c[1] != [0, 0]
                && row.a.iter().chain(&row.c).flatten().all(|&x| x < p)
                && row.norm.len() == 10
                && row.norm[9] != 0
                && row.norm.iter().all(|&x| x < p),
            "certificate shape",
        )?;
        let norm: P = row.norm.iter().map(|&x| [x, 0]).collect();
        ensure(
            sub(
                f,
                &mul(f, &row.a, &row.a),
                &mul(f, &mul(f, &row.c, &row.c), &self.h),
            ) == norm,
            "independent norm identity",
        )?;
        let mut product = vec![[1, 0]];
        for &(index, sign) in &row.terms {
            let &(x, y) = self.base.get(index).ok_or("base index")?;
            ensure([-1, 1].contains(&sign), "unit small sign")?;
            let y = if sign == 1 { y } else { f.sub([0, 0], y) };
            let den = eval(f, &row.c, x);
            ensure(
                den != [0, 0] && f.add(eval(f, &row.a, x), f.mul(den, y)) == [0, 0],
                "small principal ordinate",
            )?;
            product = mul(f, &product, &[[if x == 0 { 0 } else { p - x }, 0], [1, 0]]);
        }
        let mut previous_u: Option<[u64; 2]> = None;
        for q in &row.remainders {
            ensure(
                q.key.iter().all(|&x| x < p) && [-1, 1].contains(&q.coefficient),
                "class canonical coefficients",
            )?;
            let key_u = [q.key[0], q.key[1]];
            ensure(
                previous_u.is_none_or(|old| old < key_u),
                "distinct ordered quadratic polynomials",
            )?;
            previous_u = Some(key_u);
            let d = (q.key[1] as u128 * q.key[1] as u128 + p as u128 * 4 - 4 * q.key[0] as u128)
                % p as u128;
            ensure(
                d != 0 && fpow(d as u64, (p - 1) / 2, p) == p - 1,
                "independent Fp irreducibility",
            )?;
            let u = vec![[q.key[0], 0], [q.key[1], 0], [1, 0]];
            let v = trim(vec![[q.key[2], q.key[3]], [q.key[4], q.key[5]]]);
            let neg0 = f.sub([0, 0], [q.key[2], q.key[3]]);
            let neg1 = f.sub([0, 0], [q.key[4], q.key[5]]);
            let negative = [q.key[0], q.key[1], neg0[0], neg0[1], neg1[0], neg1[1]];
            ensure(q.key <= negative, "canonical signed class orientation")?;
            ensure(
                gcd(f, &row.c, &u)? == vec![[1, 0]],
                "cofactor denominator coprimality",
            )?;
            let actual = if q.coefficient == 1 {
                v.clone()
            } else {
                v.iter().map(|&x| f.sub([0, 0], x)).collect()
            };
            ensure(
                rem(f, &add(f, &row.a, &mul(f, &row.c, &actual)), &u)?.is_empty(),
                "cofactor principal ordinate",
            )?;
            ensure(
                rem(f, &sub(f, &mul(f, &v, &v), &self.h), &u)?.is_empty(),
                "independent Mumford validity",
            )?;
            product = mul(f, &product, &u);
        }
        let scaled: P = product
            .into_iter()
            .map(|x| f.mul(x, [row.norm[9], 0]))
            .collect();
        ensure(
            scaled == norm,
            "independent full distinct-support factorization",
        )?;
        Ok(
            json!({"schema":"cover.quadratic-pair.principal-check/v1","small_terms":row.terms.len(),"quadratic_classes":row.remainders.len(),"independent_u128_norm_and_ordinates":true,"integer_signs_checked":true,"rank_claim":false,"geometric_source_measurement":false}),
        )
    }
}

fn polynomial_pow(f: F, a: &[E], mut exponent: u64, modulus: &[E]) -> Result<P, String> {
    let mut result = vec![[1, 0]];
    let mut base = rem(f, a, modulus)?;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = rem(f, &mul(f, &result, &base), modulus)?;
        }
        base = rem(f, &mul(f, &base, &base), modulus)?;
        exponent >>= 1;
    }
    Ok(result)
}
fn embed(a: &[u64]) -> P {
    a.iter().map(|&x| [x, 0]).collect()
}
fn fp_coefficients(a: &[E]) -> Result<Vec<u64>, String> {
    ensure(a.iter().all(|x| x[1] == 0), "Fp polynomial")?;
    Ok(a.iter().map(|x| x[0]).collect())
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Attempt {
    polynomial: [u64; 4],
    gcd: Vec<u64>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SplitReport {
    p: u64,
    max_attempts: u8,
    normalized: Vec<u64>,
    squarefree_gcd: Vec<u64>,
    linear_gcd: Option<Vec<u64>>,
    frobenius_p: Option<Vec<u64>>,
    frobenius_p2: Option<Vec<u64>>,
    attempts: Vec<Attempt>,
    factors: Option<[Vec<u64>; 2]>,
    refusal: Option<String>,
}
/// Independently replay every structural prefix and every successful/refused EDF
/// attempt. The expected cap comes from the external registration, not the trace.
pub fn check_split(raw: &Value, tail: &[u64], expected_cap: u8) -> Result<Value, String> {
    let r: SplitReport = serde_json::from_value(raw.clone()).map_err(|e| e.to_string())?;
    let p = r.p;
    ensure(
        (5..=1511).contains(&p)
            && prime(p)
            && tail.len() == 5
            && tail[4] != 0
            && tail.iter().all(|&x| x < p)
            && (1..=16).contains(&expected_cap)
            && r.max_attempts == expected_cap
            && r.attempts.len() <= expected_cap as usize,
        "split registration",
    )?;
    // All coefficients stay in the real Fp subalgebra; no extension-field
    // representation or producer polynomial routine enters this replay.
    let f = F { p, w: 0 };
    let inverse = fpow(tail[4], p - 2, p);
    let normalized: Vec<u64> = tail
        .iter()
        .map(|&x| ((x as u128 * inverse as u128) % p as u128) as u64)
        .collect();
    ensure(r.normalized == normalized, "split normalized input")?;
    let n = embed(&normalized);
    let derivative: P = n
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &x)| f.mul(x, [i as u64 % p, 0]))
        .collect();
    let squarefree = fp_coefficients(&gcd(f, &n, &derivative)?)?;
    ensure(r.squarefree_gcd == squarefree, "squarefree gcd trace")?;
    if squarefree != vec![1] {
        ensure(
            r.frobenius_p.is_none()
                && r.linear_gcd.is_none()
                && r.frobenius_p2.is_none()
                && r.attempts.is_empty()
                && r.factors.is_none()
                && r.refusal.as_deref() == Some("repeated quartic factor"),
            "repeated-factor stop",
        )?;
        return Ok(
            json!({"structural_refusal":"repeated","attempts":0,"independent_u128_trace":true}),
        );
    }
    let x = vec![[0, 0], [1, 0]];
    let xp = polynomial_pow(f, &x, p, &n)?;
    let linear = fp_coefficients(&gcd(f, &n, &sub(f, &xp, &x))?)?;
    ensure(
        r.frobenius_p == Some(fp_coefficients(&xp)?) && r.linear_gcd == Some(linear.clone()),
        "first Frobenius/linear gcd",
    )?;
    if linear != vec![1] {
        ensure(
            r.frobenius_p2.is_none()
                && r.attempts.is_empty()
                && r.factors.is_none()
                && r.refusal.as_deref() == Some("quartic has Fp linear support"),
            "linear-factor stop",
        )?;
        return Ok(
            json!({"structural_refusal":"linear","attempts":0,"independent_u128_trace":true}),
        );
    }
    // Direct exponent p² is intentionally different from producer's successive
    // Frobenius exponentiation of the already-computed x^p.
    let xp2 = polynomial_pow(f, &x, p * p, &n)?;
    ensure(
        r.frobenius_p2 == Some(fp_coefficients(&xp2)?),
        "second Frobenius trace",
    )?;
    if xp2 != x {
        ensure(
            r.attempts.is_empty()
                && r.factors.is_none()
                && r.refusal.as_deref() == Some("quartic is not a product of degree-two factors"),
            "degree-four stop",
        )?;
        return Ok(
            json!({"structural_refusal":"degree_four","attempts":0,"independent_u128_trace":true}),
        );
    }
    for (i, attempt) in r.attempts.iter().enumerate() {
        ensure(
            attempt.polynomial.iter().all(|&x| x < p),
            "canonical split polynomial",
        )?;
        let powered = polynomial_pow(f, &embed(&attempt.polynomial), (p * p - 1) / 2, &n)?;
        let actual = gcd(f, &n, &sub(f, &powered, &[[1, 0]]))?;
        ensure(
            attempt.gcd == fp_coefficients(&actual)?,
            "EDF gcd recurrence",
        )?;
        if actual.len() != 3 {
            continue;
        }
        let mut quotient = n.clone();
        let divisor = actual.clone();
        let mut result = vec![[0, 0]; 3];
        while quotient.len() >= divisor.len() {
            let shift = quotient.len() - divisor.len();
            let c = *quotient.last().unwrap();
            result[shift] = c;
            for (j, &v) in divisor.iter().enumerate() {
                quotient[shift + j] = f.sub(quotient[shift + j], f.mul(c, v));
            }
            quotient = trim(quotient);
        }
        ensure(quotient.is_empty(), "exact split quotient")?;
        let mut expected = [fp_coefficients(&actual)?, fp_coefficients(&trim(result))?];
        expected.sort();
        ensure(
            r.factors == Some(expected.clone()) && r.refusal.is_none() && i + 1 == r.attempts.len(),
            "first successful split endpoint",
        )?;
        ensure(
            expected[0] != expected[1] && mul(f, &embed(&expected[0]), &embed(&expected[1])) == n,
            "distinct complete factors",
        )?;
        for u in &expected {
            ensure(
                u.len() == 3 && u[2] == 1 && u.iter().all(|&x| x < p),
                "monic quadratic",
            )?;
            let d = (u[1] as u128 * u[1] as u128 + 4 * p as u128 - 4 * u[0] as u128) % p as u128;
            ensure(
                d != 0 && fpow(d as u64, (p - 1) / 2, p) == p - 1,
                "independent split irreducibility",
            )?;
        }
        return Ok(
            json!({"attempts":r.attempts.len(),"admitted_factors":expected,"independent_u128_trace":true}),
        );
    }
    ensure(
        r.attempts.len() == expected_cap as usize
            && r.factors.is_none()
            && r.refusal.as_deref() == Some("quartic split attempt cap"),
        "retained attempt-cap refusal",
    )?;
    Ok(
        json!({"attempts":r.attempts.len(),"attempt_cap_refusal":true,"independent_u128_trace":true}),
    )
}
