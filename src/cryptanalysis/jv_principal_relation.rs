//! Checked principal-function certificates for degree-nine relation remainders.
//! All field arithmetic uses the existing native library; no discrete log enters.
use crate::cryptanalysis::{
    jv_bitops as work,
    jv_cover::{BaseEl, Div, Fld, Fq, Hyp, E2},
};
use serde::{Deserialize, Serialize};

pub type Poly = Vec<E2>;
pub fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&E2::ZERO) {
        a.pop();
    }
    a
}
pub fn embed(a: &[u64]) -> Poly {
    a.iter().map(|&x| E2([x, 0])).collect()
}
pub fn encode(a: &[E2]) -> Vec<[u64; 2]> {
    a.iter().map(|x| x.0).collect()
}
#[allow(dead_code)] // Shared helper used by the independent copied qualifier.
pub fn decode(a: &[[u64; 2]]) -> Poly {
    a.iter().copied().map(E2).collect()
}
pub fn sub(f: &Fq, a: &[E2], b: &[E2]) -> Poly {
    work::words((a.len() + b.len()) as u64);
    trim(
        (0..a.len().max(b.len()))
            .map(|i| f.sub(a.get(i).unwrap_or(&E2::ZERO), b.get(i).unwrap_or(&E2::ZERO)))
            .collect(),
    )
}
pub fn mul(f: &Fq, a: &[E2], b: &[E2]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![E2::ZERO; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = f.add(&out[i + j], &f.mul(x, y));
            work::words(3);
        }
    }
    trim(out)
}
pub fn modulo(f: &Fq, a: &[E2], u: &[E2]) -> Result<Poly, &'static str> {
    if u.len() < 2 || u.last() != Some(&E2::ONE) {
        return Err("monic positive-degree modulus required");
    }
    let mut out = trim(a.to_vec());
    while out.len() >= u.len() {
        let shift = out.len() - u.len();
        let factor = *out.last().ok_or("leading term")?;
        for (i, value) in u.iter().enumerate() {
            out[shift + i] = f.sub(&out[shift + i], &f.mul(&factor, value));
            work::words(3);
        }
        out = trim(out);
    }
    Ok(out)
}
pub fn eval(f: &Fq, a: &[E2], x: u64) -> E2 {
    a.iter().rev().fold(E2::ZERO, |acc, c| {
        work::words(1);
        f.add(&f.scale(&acc, x), c)
    })
}
pub fn inverse_linear(f: &Fq, c: &[E2], u: &[E2]) -> Result<Poly, &'static str> {
    if c.len() != 2 || u.len() != 3 || u[2] != E2::ONE || !u[0].in_fp() || !u[1].in_fp() {
        return Err("linear/quadratic inverse shape");
    }
    let d0 = f.sub(&c[0], &f.scale(&c[1], u[1].0[0]));
    let d1 = f.neg(&c[1]);
    let determinant = f.add(
        &f.mul(&c[0], &d0),
        &f.scale(&f.mul(&c[1], &c[1]), u[0].0[0]),
    );
    if determinant == E2::ZERO {
        return Err("remainder not coprime to denominator");
    }
    let inv = f.inv(&determinant);
    Ok(trim(vec![f.mul(&d0, &inv), f.mul(&d1, &inv)]))
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Remainder {
    pub key: [u64; 6],
    pub coefficient: i8,
}
impl Remainder {
    pub fn divisor(&self) -> Div<E2> {
        Div {
            u: vec![E2([self.key[0], 0]), E2([self.key[1], 0]), E2::ONE],
            v: trim(vec![
                E2([self.key[2], self.key[3]]),
                E2([self.key[4], self.key[5]]),
            ]),
        }
    }
}
fn canonical_remainder(f: &Fq, u: &[E2], v: &[E2]) -> Remainder {
    let a = v.first().copied().unwrap_or(E2::ZERO);
    let b = v.get(1).copied().unwrap_or(E2::ZERO);
    let (na, nb) = (f.neg(&a), f.neg(&b));
    let key = [u[0].0[0], u[1].0[0], a.0[0], a.0[1], b.0[0], b.0[1]];
    let neg = [u[0].0[0], u[1].0[0], na.0[0], na.0[1], nb.0[0], nb.0[1]];
    work::words(18);
    if neg < key {
        Remainder {
            key: neg,
            coefficient: -1,
        }
    } else {
        Remainder {
            key,
            coefficient: 1,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub terms: Vec<(usize, i8)>,
    pub remainder: Option<Remainder>,
    pub a: Vec<[u64; 2]>,
    pub c: Vec<[u64; 2]>,
    pub norm: Vec<u64>,
}
#[derive(Clone, Debug)]
pub struct Verified {
    certificate: Certificate,
}
impl Verified {
    pub fn certificate(&self) -> &Certificate {
        &self.certificate
    }
    pub fn into_certificate(self) -> Certificate {
        self.certificate
    }
}

/// Prove the support of A+C*y from its exact norm and point ordinates.
/// For seven points the remaining quadratic must be irreducible over Fp.
pub fn check_equation(
    f: &Fq,
    h: &[E2],
    base: &[BaseEl],
    a: &[E2],
    c: &[E2],
    norm: &[u64],
    terms: &[(usize, i8)],
    tail: &[u64],
) -> Result<Verified, &'static str> {
    if h.len() != 8
        || h.last() == Some(&E2::ZERO)
        || a.len() > 5
        || c.len() != 2
        || c[1] == E2::ZERO
        || norm.len() != 10
        || norm[9] == 0
        || ![7, 9].contains(&terms.len())
        || terms.windows(2).any(|w| w[0].0 >= w[1].0)
        || norm.iter().chain(tail).any(|&x| x >= f.p)
        || h.iter()
            .chain(a)
            .chain(c)
            .any(|x| x.0.iter().any(|&v| v >= f.p))
    {
        return Err("degree/canonical/support bounds");
    }
    let actual = sub(f, &mul(f, a, a), &mul(f, &mul(f, c, c), h));
    if actual != embed(norm) {
        return Err("principal norm identity");
    }
    let mut product = vec![E2::ONE];
    let mut previous_x = None;
    for &(index, sign) in terms {
        let b = base.get(index).ok_or("small point index")?;
        if ![-1, 1].contains(&sign)
            || b.x >= f.p
            || b.y == E2::ZERO
            || b.y.0.iter().any(|&v| v >= f.p)
        {
            return Err("small point shape");
        }
        if previous_x.is_some_and(|x| b.x <= x) {
            return Err("distinct ordered small abscissae required");
        }
        previous_x = Some(b.x);
        let y = if sign == 1 { b.y } else { f.neg(&b.y) };
        let den = eval(f, c, b.x);
        if den == E2::ZERO
            || f.mul(&y, &y) != eval(f, h, b.x)
            || f.add(&eval(f, a, b.x), &f.mul(&den, &y)) != E2::ZERO
        {
            return Err("point/ordinate/principal binding");
        }
        product = mul(f, &product, &[f.neg(&f.from_fp(b.x)), E2::ONE]);
        work::words(7);
    }
    if mul(f, &product, &embed(tail)) != embed(norm) {
        return Err("distinct support factorization");
    }
    let remainder = if terms.len() == 9 {
        if tail.len() != 1 || tail[0] == 0 {
            return Err("full support tail");
        }
        None
    } else {
        if tail.len() != 3 || tail[2] == 0 {
            return Err("quadratic tail");
        }
        let inv = f.inv(&f.from_fp(tail[2]));
        let u: Poly = tail.iter().map(|&x| f.mul(&f.from_fp(x), &inv)).collect();
        let disc = f.sub(&f.mul(&u[1], &u[1]), &f.scale(&u[0], 4));
        if disc == E2::ZERO || f.pow(&disc, (f.p - 1) as u128 / 2) != f.from_fp(f.p - 1) {
            return Err("irreducible Fp quadratic required");
        }
        let inv_c = inverse_linear(f, c, &u)?;
        let v = modulo(f, &mul(f, &modulo(f, a, &u)?, &inv_c), &u)?
            .iter()
            .map(|x| f.neg(x))
            .collect::<Poly>();
        if !modulo(f, &sub(f, &mul(f, &v, &v), h), &u)?.is_empty() {
            return Err("Mumford remainder validity");
        }
        Some(canonical_remainder(f, &u, &v))
    };
    Ok(Verified {
        certificate: Certificate {
            terms: terms.to_vec(),
            remainder,
            a: encode(a),
            c: encode(c),
            norm: norm.to_vec(),
        },
    })
}
/// Independent group-law control, not used by the partial norm constructor.
pub fn replay_group(jac: &Hyp<Fq>, base: &[BaseEl], row: &Certificate) -> bool {
    if row.terms.windows(2).any(|w| w[0].0 >= w[1].0)
        || (row.remainder.is_some() && row.terms.len() != 7)
        || (row.remainder.is_none() && row.terms.len() != 9)
    {
        return false;
    }
    let mut acc = jac.identity();
    for &(index, sign) in &row.terms {
        let Some(b) = base.get(index) else {
            return false;
        };
        if ![-1, 1].contains(&sign) {
            return false;
        }
        let d = if sign == 1 {
            b.d.clone()
        } else {
            jac.neg(&b.d)
        };
        if !jac.is_valid(&d) {
            return false;
        }
        acc = jac.add(&acc, &d);
    }
    if let Some(r) = &row.remainder {
        if ![-1, 1].contains(&r.coefficient) || r.key.iter().any(|&x| x >= jac.f.p) {
            return false;
        }
        let mut d = r.divisor();
        if !jac.is_valid(&d) {
            return false;
        }
        if r.coefficient == -1 {
            d = jac.neg(&d);
        }
        acc = jac.add(&acc, &d);
    }
    jac.is_identity(&acc)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) fn hand() -> (
        Fq,
        Poly,
        Vec<BaseEl>,
        Poly,
        Poly,
        Vec<u64>,
        Vec<(usize, i8)>,
        Vec<u64>,
    ) {
        let f = Fq::new(101);
        assert_eq!(f.w, 2);
        let u = vec![99, 0, 1];
        let mut fp = embed(&u);
        for x in 1..=7 {
            fp = mul(&f, &fp, &[f.neg(&f.from_fp(x)), E2::ONE]);
        }
        fp = fp.iter().map(|x| f.neg(x)).collect();
        let norm = fp.iter().map(|x| x.0[0]).collect::<Vec<_>>();
        let a0 = f.from_fp(90);
        assert_eq!(f.mul(&a0, &a0), fp[0]);
        let a1 = f.mul(&fp[1], &f.inv(&f.scale(&a0, 2)));
        let a = vec![a0, a1, E2([1, 3])];
        let c = vec![E2::ZERO, E2::ONE];
        let numerator = sub(&f, &mul(&f, &a, &a), &fp);
        assert_eq!(&numerator[..2], &[E2::ZERO, E2::ZERO]);
        let h = numerator[2..].to_vec();
        assert_eq!(h.len(), 8);
        assert_eq!(h[7], E2::ONE);
        let base = (1..=7)
            .map(|x| {
                let y = f.neg(&f.mul(&eval(&f, &a, x), &f.inv(&f.from_fp(x))));
                BaseEl {
                    x,
                    y,
                    d: Div {
                        u: vec![f.neg(&f.from_fp(x)), E2::ONE],
                        v: vec![y],
                    },
                }
            })
            .collect::<Vec<_>>();
        let terms = (0..7).map(|i| (i, 1)).collect::<Vec<_>>();
        let mut tail = embed(&u)
            .iter()
            .map(|x| f.neg(x))
            .map(|x| x.0[0])
            .collect::<Vec<_>>();
        tail[1] = 0;
        (f, h, base, a, c, norm, terms, tail)
    }
    #[test]
    fn exact_principal_remainder_agrees_with_cantor_group_law() {
        let (f, h, base, a, c, norm, terms, tail) = hand();
        let row = check_equation(&f, &h, &base, &a, &c, &norm, &terms, &tail).unwrap();
        assert!(row.certificate().remainder.is_some());
        let jac = Hyp { f: &f, h, g: 3 };
        assert!(replay_group(&jac, &base, row.certificate()));
    }
    #[test]
    fn corrupted_norm_ordinate_support_and_tail_are_refused() {
        let (f, h, mut base, a, c, mut norm, terms, tail) = hand();
        norm[0] = (norm[0] + 1) % f.p;
        assert!(check_equation(&f, &h, &base, &a, &c, &norm, &terms, &tail).is_err());
        norm[0] = (norm[0] + f.p - 1) % f.p;
        let mut bad = tail.clone();
        bad[0] = (bad[0] + 1) % f.p;
        assert!(check_equation(&f, &h, &base, &a, &c, &norm, &terms, &bad).is_err());
        let mut duplicates = terms.clone();
        duplicates[1] = duplicates[0];
        assert!(check_equation(&f, &h, &base, &a, &c, &norm, &duplicates, &tail).is_err());
        base[0].y = f.add(&base[0].y, &E2::ONE);
        assert!(check_equation(&f, &h, &base, &a, &c, &norm, &terms, &tail).is_err());
    }
    #[test]
    fn denominator_inverse_detects_non_coprimality() {
        let f = Fq::new(101);
        let u = vec![f.from_fp(99), E2::ZERO, E2::ONE];
        for a in 0..8 {
            for b in 0..8 {
                let c = vec![E2([a, b]), E2::ONE];
                if let Ok(inv) = inverse_linear(&f, &c, &u) {
                    assert_eq!(modulo(&f, &mul(&f, &c, &inv), &u).unwrap(), vec![E2::ONE]);
                }
            }
        }
        assert!(inverse_linear(&f, &[E2([0, 100]), E2::ONE], &u).is_err());
    }
}
