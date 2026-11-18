//! Bounded split-leading non-split correspondence. Source support splits
//! in Fp²; coefficients lie in Fp³, so their compositum Fp⁶ suffices.
//! Target fibers remain quadratic Mumford pairs; no target roots are chosen.
use super::{construct, construct_for, Assessment, CubicField, E};
use crate::balanced::{Balanced, Class};
use crate::reference::{Fp2, Fp2Ctx, FpPoly};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::collections::BTreeMap;
use std::sync::OnceLock;
#[path = "closed_fibres.rs"]
mod closed_fibres;
type Result<T> = std::result::Result<T, String>;
#[path = "cubic_inverse15.rs"]
mod cubic_inverse15;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    a: E,
    b: E,
}
type Poly = Vec<Element>;
struct Field {
    k: CubicField,
    nonresidue: E,
    cubic_first15: bool,
    shortcut_calls15: std::cell::Cell<u64>,
    general_calls15: std::cell::Cell<u64>,
}
impl Field {
    fn new(k: CubicField) -> Self {
        let nr = Fp2Ctx::try_new(k.p).unwrap().nonresidue();
        let nonresidue = k.c(nr);
        assert_eq!(k.pow(&nonresidue, (k.p.pow(3) - 1) / 2), k.c(k.p - 1));
        Self {
            k,
            nonresidue,
            cubic_first15: false,
            shortcut_calls15: std::cell::Cell::new(0),
            general_calls15: std::cell::Cell::new(0),
        }
    }
    fn c(&self, n: u64) -> Element {
        Element {
            a: self.k.c(n),
            b: self.k.z(),
        }
    }
    fn lift3(&self, a: &E) -> Element {
        Element {
            a: a.clone(),
            b: self.k.z(),
        }
    }
    fn lift2(&self, x: Fp2) -> Element {
        Element {
            a: self.k.c(x.a),
            b: self.k.c(x.b),
        }
    }
    fn zero(&self, a: &Element) -> bool {
        a.a.is_zero() && a.b.is_zero()
    }
    fn add(&self, a: &Element, b: &Element) -> Element {
        Element {
            a: a.a.add(&b.a),
            b: a.b.add(&b.b),
        }
    }
    fn neg(&self, a: &Element) -> Element {
        Element {
            a: a.a.neg(),
            b: a.b.neg(),
        }
    }
    fn sub(&self, a: &Element, b: &Element) -> Element {
        self.add(a, &self.neg(b))
    }
    fn mul(&self, a: &Element, b: &Element) -> Element {
        Element {
            a: self
                .k
                .mul(&a.a, &b.a)
                .add(&self.k.mul(&self.nonresidue, &self.k.mul(&a.b, &b.b))),
            b: self.k.mul(&a.a, &b.b).add(&self.k.mul(&a.b, &b.a)),
        }
    }
    fn inv(&self, a: &Element) -> Element {
        if self.cubic_first15 && a.b.is_zero() {
            assert!(!self.zero(a), "zero in compositum inverse");
            self.shortcut_calls15.set(self.shortcut_calls15.get() + 1);
            return self.lift3(&self.k.inv(&a.a));
        }
        self.general_calls15.set(self.general_calls15.get() + 1);
        self.inv_original15(a)
    }
    fn inv_original15(&self, a: &Element) -> Element {
        assert!(!self.zero(a), "zero in compositum inverse");
        let norm = self
            .k
            .mul(&a.a, &a.a)
            .sub(&self.k.mul(&self.nonresidue, &self.k.mul(&a.b, &a.b)));
        let inverse = self.k.inv(&norm);
        Element {
            a: self.k.mul(&a.a, &inverse),
            b: self.k.mul(&a.b.neg(), &inverse),
        }
    }
    fn pow(&self, a: &Element, mut n: u64) -> Element {
        let mut a = a.clone();
        let mut out = self.c(1);
        while n != 0 {
            if n & 1 != 0 {
                out = self.mul(&out, &a);
            }
            a = self.mul(&a, &a);
            n >>= 1;
        }
        out
    }
    fn base(&self, a: &Element) -> Result<BigUint> {
        if !a.b.is_zero() || a.a.degree().is_some_and(|d| d > 0) {
            return Err(format!("completed image did not descend: {a:?}"));
        }
        Ok(a.a.coeff(0))
    }
    fn trim(&self, mut a: Poly) -> Poly {
        while a.last().is_some_and(|c| self.zero(c)) {
            a.pop();
        }
        a
    }
    fn addp(&self, a: &Poly, b: &Poly) -> Poly {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| {
                    self.add(
                        &a.get(i).cloned().unwrap_or_else(|| self.c(0)),
                        &b.get(i).cloned().unwrap_or_else(|| self.c(0)),
                    )
                })
                .collect(),
        )
    }
    fn negp(&self, a: &Poly) -> Poly {
        a.iter().map(|c| self.neg(c)).collect()
    }
    fn subp(&self, a: &Poly, b: &Poly) -> Poly {
        self.addp(a, &self.negp(b))
    }
    fn mulp(&self, a: &Poly, b: &Poly) -> Poly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut out = vec![self.c(0); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] = self.add(&out[i + j], &self.mul(x, y));
            }
        }
        self.trim(out)
    }
    fn scale(&self, a: &Poly, b: &Element) -> Poly {
        self.trim(a.iter().map(|c| self.mul(c, b)).collect())
    }
    fn divrem(&self, a: &Poly, b: &Poly) -> (Poly, Poly) {
        assert!(!b.is_empty());
        let mut r = a.clone();
        let mut q = vec![self.c(0); (a.len() + 1).saturating_sub(b.len())];
        let inverse = self.inv(b.last().unwrap());
        while !r.is_empty() && r.len() >= b.len() {
            let shift = r.len() - b.len();
            let value = self.mul(r.last().unwrap(), &inverse);
            q[shift] = value.clone();
            for (i, c) in b.iter().enumerate() {
                r[i + shift] = self.sub(&r[i + shift], &self.mul(c, &value));
            }
            r = self.trim(r);
        }
        (self.trim(q), r)
    }
    fn exact(&self, a: &Poly, b: &Poly) -> Result<Poly> {
        let (q, r) = self.divrem(a, b);
        if !r.is_empty() {
            return Err("nonexact compositum polynomial division".into());
        }
        Ok(q)
    }
    fn monic(&self, a: &Poly) -> Poly {
        self.scale(a, &self.inv(a.last().unwrap()))
    }
    fn xgcd(&self, a: &Poly, b: &Poly) -> (Poly, Poly, Poly) {
        let (mut a, mut b) = (a.clone(), b.clone());
        let (mut s, mut ss, mut t, mut tt) = (vec![self.c(1)], vec![], vec![], vec![self.c(1)]);
        while !b.is_empty() {
            let (q, r) = self.divrem(&a, &b);
            a = b;
            b = r;
            (s, ss) = (ss.clone(), self.subp(&s, &self.mulp(&q, &ss)));
            (t, tt) = (tt.clone(), self.subp(&t, &self.mulp(&q, &tt)));
        }
        let inverse = self.inv(a.last().unwrap());
        (
            self.scale(&a, &inverse),
            self.scale(&s, &inverse),
            self.scale(&t, &inverse),
        )
    }
    fn value(&self, a: &Poly, x: &Element) -> Element {
        a.iter()
            .rev()
            .fold(self.c(0), |v, c| self.add(&self.mul(&v, x), c))
    }
    fn lift_poly(&self, a: &FpPoly) -> Poly {
        a.coeffs
            .iter()
            .map(|c| self.c(c.to_u64().unwrap()))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Divisor {
    u: Poly,
    v: Poly,
    plus: u8,
}
struct Jacobian<'a> {
    k: &'a Field,
    f: Poly,
    infinity: Poly,
}
impl<'a> Jacobian<'a> {
    fn new(k: &'a Field, c: &Balanced) -> Self {
        let (f, infinity) = c.equation_and_infinity();
        Self {
            k,
            f: k.lift_poly(f),
            infinity: k.lift_poly(infinity),
        }
    }
    fn zero(&self) -> Divisor {
        Divisor {
            u: vec![self.k.c(1)],
            v: vec![],
            plus: 1,
        }
    }
    fn check(&self, d: &Divisor) -> Result<()> {
        if d.u.is_empty()
            || d.u.len() > 3
            || d.u.last() != Some(&self.k.c(1))
            || d.v.len() >= d.u.len()
            || d.u.len() - 1 + usize::from(d.plus) > 2
        {
            return Err("invalid compositum balanced representation".into());
        }
        if !self
            .k
            .divrem(&self.k.subp(&self.f, &self.k.mulp(&d.v, &d.v)), &d.u)
            .1
            .is_empty()
        {
            return Err("compositum divisor left target curve".into());
        }
        Ok(())
    }
    fn pair(&self, u: Poly, v: Poly, plus: u8) -> Result<Divisor> {
        let d = Divisor { u, v, plus };
        self.check(&d)?;
        Ok(d)
    }
    fn neg(&self, d: &Divisor) -> Divisor {
        Divisor {
            u: d.u.clone(),
            v: self.k.negp(&d.v),
            plus: 3 - d.u.len() as u8 - d.plus,
        }
    }
    fn add(&self, a: &Divisor, b: &Divisor) -> Result<Divisor> {
        self.check(a)?;
        self.check(b)?;
        if *a == self.zero() {
            return Ok(b.clone());
        }
        if *b == self.zero() {
            return Ok(a.clone());
        }
        let k = self.k;
        let (d1, e1, e2) = k.xgcd(&a.u, &b.u);
        let (d, c1, c2) = k.xgcd(&d1, &k.addp(&a.v, &b.v));
        let u = k.exact(&k.mulp(&a.u, &b.u), &k.mulp(&d, &d))?;
        let numerator = k.addp(
            &k.addp(
                &k.mulp(&k.mulp(&k.mulp(&c1, &e1), &a.u), &b.v),
                &k.mulp(&k.mulp(&k.mulp(&c1, &e2), &b.u), &a.v),
            ),
            &k.mulp(&c2, &k.addp(&k.mulp(&a.v, &b.v), &self.f)),
        );
        let v = k.divrem(&k.exact(&numerator, &d)?, &u).1;
        let plus = i64::from(a.plus) + i64::from(b.plus) + d.len() as i64 - 2;
        self.reduce(u, v, plus)
    }
    // Shared existing reduction, also used for a four-point closed fibre.
    // A raw finite4-2K divisor has signed plus=-1 before reduction.
    fn reduce(&self, u: Poly, v: Poly, plus: i64) -> Result<Divisor> {
        let k = self.k;
        let m = u.len() - 1;
        if m <= 2 && plus >= 0 && plus <= 2 - m as i64 {
            return self.pair(u, v, plus as u8);
        }
        let line = if m == 4 {
            v
        } else {
            let g = if plus >= 0 {
                self.infinity.clone()
            } else {
                k.negp(&self.infinity)
            };
            k.addp(&g, &k.divrem(&k.subp(&v, &g), &u).1)
        };
        let numerator = k.subp(&self.f, &k.mulp(&line, &line));
        if numerator.is_empty() {
            return Err("compositum sextic became a square".into());
        }
        let raw = k.exact(&numerator, &u)?;
        if raw.is_empty() {
            return Err("zero compositum reduced support".into());
        }
        let next = k.monic(&raw);
        let r = next.len() - 1;
        let valuation = if line.get(3) == Some(&k.c(1)) {
            4 - numerator.len() as i64
        } else {
            -3
        };
        let next_plus = plus - valuation - r as i64;
        if r > 2 || next_plus < 0 || next_plus > 2 - r as i64 {
            return Err("compositum infinity count did not reduce".into());
        }
        let next_v = k.divrem(&k.negp(&line), &next).1;
        self.pair(next, next_v, next_plus as u8)
    }
    fn descend(&self, d: &Divisor, target: &Balanced) -> Result<Class> {
        self.check(d)?;
        let poly = |p: &Poly| -> Result<FpPoly> {
            Ok(FpPoly::from_coeffs(
                p.iter()
                    .map(|c| self.k.base(c))
                    .collect::<Result<Vec<_>>>()?,
                BigUint::from(self.k.k.p),
            ))
        };
        target.wrap(poly(&d.u)?, poly(&d.v)?, d.plus)
    }
}

struct RichelotMap {
    k: Field,
    source: Balanced,
    target: Balanced,
    g: [Poly; 3],
    h: [Poly; 3],
    target_scale: u64,
    quadratic: Fp2Ctx,
    square_roots: OnceLock<BTreeMap<(u64, u64), Fp2>>,
}
impl RichelotMap {
    fn new() -> Self {
        Self::from_assessment(construct()).unwrap()
    }
    fn from_assessment(a: Assessment) -> Result<Self> {
        let pairing = a
            .pairs
            .iter()
            .find(|p| {
                let g = std::array::from_fn(|i| {
                    let (u, v) = p[i];
                    vec![
                        a.field.mul(&a.roots[u], &a.roots[v]),
                        a.roots[u].add(&a.roots[v]).neg(),
                        a.field.c(1),
                    ]
                });
                !a.field.determinant(&g).is_zero()
            })
            .copied()
            .ok_or("no nondegenerate stable kernel")?;
        // Preserve the frozen old-model choice; transported kernels use the
        // separately validated explicit constructor below.
        assert_eq!(pairing, [(0, 4), (1, 5), (2, 3)]);
        Self::from_selected_pairing(a, pairing)
    }
    fn from_selected_pairing(a: Assessment, pairing: super::Pairing) -> Result<Self> {
        let p = a.field.p;
        if a.roots.len() != 6 || a.source.len() != 7 {
            return Err("selected Richelot chart requires six branch roots and a sextic".into());
        }
        let mut used = [false; 6];
        for (u, v) in pairing {
            for i in [u, v] {
                if i >= 6 || used[i] {
                    return Err("selected kernel is not a disjoint six-root matching".into());
                }
                used[i] = true;
            }
        }
        if super::canonical_pairing(pairing) != pairing || !a.pairs.contains(&pairing) {
            return Err("selected kernel is not an inventoried canonical stable matching".into());
        }
        for (i, root) in a.roots.iter().enumerate() {
            if root.p != a.field.q.p
                || root.degree().is_some_and(|d| d >= 3)
                || a.roots[..i].contains(root)
            {
                return Err(
                    "branch roots are not six distinct canonical elements of the field".into(),
                );
            }
        }
        let mut permutation = [0; 6];
        for (i, root) in a.roots.iter().enumerate() {
            permutation[i] = a
                .roots
                .iter()
                .position(|r| *r == a.field.pow(root, p))
                .ok_or("branch roots are not Frobenius stable")?;
        }
        if !super::stable(&pairing, &permutation) {
            return Err("selected kernel is not Frobenius stable".into());
        }
        let k = Field::new(a.field);
        let raw_g = std::array::from_fn(|i| {
            let (u, v) = pairing[i];
            vec![
                k.k.mul(&a.roots[u], &a.roots[v]),
                a.roots[u].add(&a.roots[v]).neg(),
                k.k.c(1),
            ]
        });
        if k.k.product(&raw_g) != a.source.iter().map(|c| k.k.c(*c)).collect::<Vec<_>>() {
            return Err("selected factor product does not equal the source sextic".into());
        }
        let delta = k.k.determinant(&raw_g);
        if delta.is_zero() {
            return Err("selected Richelot determinant is zero".into());
        }
        let raw_h = k.k.dual_factors(&raw_g, &delta);
        let g = std::array::from_fn(|i| raw_g[i].iter().map(|c| k.lift3(c)).collect());
        let h = std::array::from_fn(|i| raw_h[i].iter().map(|c| k.lift3(c)).collect());
        let product = k.k.product(&raw_h);
        if product.iter().any(|c| c.degree().is_some_and(|d| d > 0)) {
            return Err("selected Richelot target coefficients do not descend".into());
        }
        let raw_target = product.iter().map(|c| k.k.base(c)).collect::<Vec<_>>();
        if raw_target.len() != 7 {
            return Err("odd target chart requires explicit coordinate conversion".into());
        }
        let leading = raw_target[6];
        let target_scale = (1..p)
            .find(|y| y * y % p == leading)
            .ok_or("nonsquare target leading requires explicit split-infinity chart")?;
        let leading_inverse = k.k.base(&k.k.inv(&k.k.c(leading)));
        let normalized = raw_target
            .iter()
            .map(|c| c * leading_inverse % p)
            .collect::<Vec<_>>();
        let source = Balanced::new(p, &a.source)?;
        let target = Balanced::new(p, &normalized)?;
        let quadratic = Fp2Ctx::try_new(p).map_err(|e| e.to_string())?;
        Ok(Self {
            k,
            source,
            target,
            g,
            h,
            target_scale,
            quadratic,
            square_roots: OnceLock::new(),
        })
    }
    fn fiber(
        &self,
        x: &Element,
        y: &Element,
        g: &[Poly; 3],
        h: &[Poly; 3],
        target_scale: u64,
        target: &Jacobian<'_>,
    ) -> Result<Divisor> {
        let k = &self.k;
        let g1 = k.value(&g[0], x);
        let g2 = k.value(&g[1], x);
        if k.zero(y) {
            if k.zero(&g1) {
                return target.pair(k.monic(&h[1]), vec![], 0);
            }
            if k.zero(&g2) {
                return target.pair(k.monic(&h[0]), vec![], 0);
            }
            if k.zero(&k.value(&g[2], x)) {
                return Ok(target.zero());
            }
            return Err("zero-y support outside branch locus".into());
        }
        let u = k.addp(&k.scale(&h[0], &g1), &k.scale(&h[1], &g2));
        if u.is_empty() {
            return Err("correspondence fiber has excess dimension".into());
        }
        let scale = k.inv(&k.mul(y, &k.c(target_scale)));
        let numerator = k.scale(
            &k.mulp(&h[0], &vec![x.clone(), k.neg(&k.c(1))]),
            &k.mul(&g1, &scale),
        );
        let monic = k.monic(&u);
        let v = k.divrem(&numerator, &monic).1;
        let m = u.len() - 1;
        if m == 2 {
            return target.pair(monic, v, 0);
        }
        if m > 2 {
            return Err("correspondence is not quadratic".into());
        }
        let cap = k.neg(&k.mul(&k.mul(&g1, h[0].last().unwrap()), &scale));
        let sign = if cap == k.c(1) {
            true
        } else if cap == k.neg(&k.c(1)) {
            false
        } else {
            return Err("degree-drop target infinity is not on curve".into());
        };
        let plus = if sign { (2 - m) as u8 } else { 0 };
        target.pair(monic, v, plus)
    }
    fn infinity(
        &self,
        g: &[Poly; 3],
        h: &[Poly; 3],
        source_scale: u64,
        target_scale: u64,
        target: &Jacobian<'_>,
    ) -> Result<Divisor> {
        let k = &self.k;
        let u = k.addp(
            &k.scale(&h[0], g[0].last().unwrap()),
            &k.scale(&h[1], g[1].last().unwrap()),
        );
        if u.len() != 3 {
            return Err("fixed source-infinity fiber requires explicit completion".into());
        }
        let u = k.monic(&u);
        let scale = k.mul(
            g[0].last().unwrap(),
            &k.inv(&k.c(source_scale * target_scale)),
        );
        let v = k.divrem(&k.scale(&h[0], &scale), &u).1;
        target.pair(u, v, 0)
    }
    fn roots(&self, u: &FpPoly) -> Result<Vec<Fp2>> {
        let ctx = self.quadratic;
        let m = u.degree().ok_or("zero source support")?;
        if m == 0 {
            return Ok(vec![]);
        }
        let b = ctx.from_fp(u.coeff(0).to_u64().unwrap());
        if m == 1 {
            return Ok(vec![ctx.neg(b)]);
        }
        if m != 2 {
            return Err("source support degree exceeds2".into());
        }
        let a = ctx.from_fp(u.coeff(1).to_u64().unwrap());
        let discriminant = ctx.sub(ctx.square(a), ctx.mul(ctx.from_fp(4), b));
        let root = self
            .square_roots
            .get_or_init(|| base_discriminant_root_table(ctx))
            .get(&(discriminant.a, discriminant.b))
            .ok_or("source quadratic discriminant did not split")?;
        let half = ctx.from_fp(self.k.k.p.div_ceil(2));
        Ok(vec![
            ctx.mul(ctx.add(ctx.neg(a), *root), half),
            ctx.mul(ctx.sub(ctx.neg(a), *root), half),
        ])
    }
    fn evaluate(&self, d: &Class, reverse: bool) -> Result<Class> {
        let (source, target, g, h, source_scale, target_scale) = if reverse {
            (
                &self.target,
                &self.source,
                &self.h,
                &self.g,
                self.target_scale,
                1,
            )
        } else {
            (
                &self.source,
                &self.target,
                &self.g,
                &self.h,
                1,
                self.target_scale,
            )
        };
        source.check(d)?;
        let target_ctx = Jacobian::new(&self.k, target);
        let (u, v, plus) = d.components();
        let mut out = target_ctx.zero();
        let quadratic = self.quadratic;
        for x in self.roots(u)? {
            let y = v.coeffs.iter().rev().fold(quadratic.zero(), |w, c| {
                quadratic.add(quadratic.mul(w, x), quadratic.from_fp(c.to_u64().unwrap()))
            });
            let raw_y = self.k.mul(&self.k.lift2(y), &self.k.c(source_scale));
            let image = self.fiber(&self.k.lift2(x), &raw_y, g, h, target_scale, &target_ctx)?;
            out = target_ctx.add(&out, &image)?;
        }
        let coefficient = 2 * i64::from(plus) + u.degree().unwrap() as i64 - 2;
        if coefficient != 0 {
            let infinity = self.infinity(g, h, source_scale, target_scale, &target_ctx)?;
            let infinity = if coefficient < 0 {
                target_ctx.neg(&infinity)
            } else {
                infinity
            };
            for _ in 0..coefficient.unsigned_abs() {
                out = target_ctx.add(&out, &infinity)?;
            }
        }
        if reverse {
            out = target_ctx.neg(&out);
        }
        target_ctx.descend(&out, target)
    }
    fn forward(&self, d: &Class) -> Result<Class> {
        self.evaluate(d, false)
    }
    fn dual(&self, d: &Class) -> Result<Class> {
        self.evaluate(d, true)
    }
}

/// The typed root API queries only base-field discriminants. In odd
/// characteristic, (a+b*s)^2 is real only if a=0 or b=0. These axes occur
/// in this order in the old lexicographic enumeration, preserving its
/// first root. This is deliberately not a general Fp2 root table.
fn base_discriminant_root_table(ctx: Fp2Ctx) -> BTreeMap<(u64, u64), Fp2> {
    let p = ctx.modulus();
    let mut roots = BTreeMap::new();
    for value in (0..p)
        .map(|b| ctx.element(0, b))
        .chain((1..p).map(|a| ctx.element(a, 0)))
    {
        let square = ctx.square(value);
        roots.entry((square.a, square.b)).or_insert(value);
    }
    roots
}

// New protocol14 harness; all preceding correspondence method bodies are reused.
const P14: u64 = 101;
const SOURCE14: [u64; 7] = [97, 29, 96, 18, 87, 99, 1];
const TARGET14: [u64; 7] = [16, 53, 24, 80, 73, 57, 1];
const ROOTS14: [u64; 6] = [1008151, 720554, 352389, 357036, 975881, 738000];
const PAIRING14: super::Pairing = [(0, 4), (1, 5), (2, 3)];
// Compile-time API binding: retain this import without constructing old fixtures.
const _: fn(u64, [u64; 3]) -> Result<Assessment> = construct_for;

#[derive(Clone, Copy, Debug)]
enum Mutation14 {
    None,
    DuplicateRoot,
    SourceConstant,
}

fn assessment14(old: bool, mutation: Mutation14) -> Result<Assessment> {
    let field = CubicField::with_cubic(P14, [3, 26, 3])?;
    let mut roots: Vec<_> = ROOTS14
        .iter()
        .map(|&n| {
            let w = field.element(n);
            if old {
                field.c(3).add(&field.inv(&w))
            } else {
                w
            }
        })
        .collect();
    let permutation = [1, 2, 0, 4, 5, 3];
    for (i, root) in roots.iter().enumerate() {
        assert_eq!(field.pow(root, P14), roots[permutation[i]]);
    }
    let pairs: Vec<_> = super::pairings()
        .into_iter()
        .filter(|pairing| super::stable(pairing, &permutation))
        .collect();
    assert_eq!(pairs.len(), 3);
    let mut source = if old {
        let inverse = field.base(&field.inv(&field.c(84)));
        [52, 0, 25, 0, 77, 0, 84]
            .into_iter()
            .map(|a| a * inverse % P14)
            .collect()
    } else {
        SOURCE14.to_vec()
    };
    match mutation {
        Mutation14::None => {}
        Mutation14::DuplicateRoot => roots[5] = roots[0].clone(),
        Mutation14::SourceConstant => source[0] = (source[0] + 1) % P14,
    }
    Ok(Assessment {
        field,
        roots,
        pairs,
        source,
        targets: vec![],
        degenerate: 0,
    })
}

fn prime_inverse14(a: u64) -> u64 {
    let field = CubicField::with_cubic(P14, [3, 26, 3]).unwrap();
    field.base(&field.inv(&field.c(a)))
}

fn signed14(c: &Balanced, d: &Class, sign: u64) -> Result<Class> {
    match sign {
        1 => Ok(d.clone()),
        100 => c.neg(d),
        _ => Err("conjugacy sign is not algebraically plus/minus one".into()),
    }
}

fn assert_model14(c: &super::Balanced, coefficients: &[u64]) {
    let expected = FpPoly::from_coeffs(
        coefficients.iter().copied().map(BigUint::from).collect(),
        BigUint::from(P14),
    );
    assert_eq!(c.equation_and_infinity().0, &expected);
}

fn run_unit14() -> Result<()> {
    use crate::balanced::{witnesses14, Bridge14};
    let map =
        RichelotMap::from_selected_pairing(assessment14(false, Mutation14::None)?, PAIRING14)?;
    assert_eq!(map.target_scale, 13);
    assert_model14(&map.source, &SOURCE14);
    assert_model14(&map.target, &TARGET14);
    let old = RichelotMap::from_selected_pairing(assessment14(true, Mutation14::None)?, PAIRING14)?;
    let source_bridge = Bridge14::new(&old.source, 3, 96)?;
    assert_model14(source_bridge.target(), &SOURCE14);
    let old_h_leading = old.target_scale * old.target_scale % P14 * prime_inverse14(84) % P14;
    let s = (1..P14)
        .find(|x| x * x % P14 == old_h_leading)
        .ok_or("old raw H leading has no prescribed base-field square root")?;
    let normalized_c = 4 * prime_inverse14(s) % P14;
    let target_bridge = Bridge14::new(&old.target, 3, normalized_c)?;
    assert_model14(target_bridge.target(), &TARGET14);
    // Forced minus from x-z=-(w-v)/(wv), fixed before divisor comparisons.
    let unsigned =
        96 * normalized_c % P14 * old.target_scale % P14 * prime_inverse14(map.target_scale) % P14;
    let kappa = (P14 - unsigned) % P14;
    assert!([1, P14 - 1].contains(&kappa));
    println!("CT14|normalization|old_scale={}|s={s}|c_normal={normalized_c}|new_scale={}|kappa={kappa}|source={SOURCE14:?}|target={TARGET14:?}",old.target_scale,map.target_scale);

    let witnesses = witnesses14(&map.source)?;
    assert_eq!(witnesses.len(), 17);
    let mut images = Vec::new();
    let mut closed_quadratic = 0;
    let mut conjugacies = 0;
    for (i, (label, d)) in witnesses.iter().enumerate() {
        println!("CT14|attempt|witness|i={i}|label={label}");
        map.source.check(d)?;
        let image = map.forward(d)?;
        map.target.check(&image)?;
        let (closed, route) = map.evaluate_closed(d, false)?;
        assert_eq!(closed, image, "forward closed mismatch at {label}");
        if matches!(
            label.as_str(),
            "quadratic_orbit" | "opposite_quadratic_orbit"
        ) {
            assert_eq!(route, closed_fibres::ClosedRoute::Quadratic);
            closed_quadratic += 1;
        }
        let dual = map.dual(&image)?;
        let doubled = map.source.mul(d, 2)?;
        assert_eq!(dual, doubled, "positive dual mismatch at {label}");
        assert_eq!(map.dual_closed(&image)?, dual);
        assert_eq!(map.forward(&dual)?, map.target.mul(&image, 2)?);
        assert_eq!(map.source.mul(&dual, 6159)?, *d);

        let old_d = source_bridge.backward(d)?;
        assert_eq!(source_bridge.forward(&old_d)?, *d);
        let old_image = old.forward(&old_d)?;
        let transported = signed14(&map.target, &target_bridge.forward(&old_image)?, kappa)?;
        assert_eq!(
            transported, image,
            "fixed-sign conjugacy mismatch at {label}"
        );
        let old_return = old.dual(&target_bridge.backward(&image)?)?;
        let chain_return = signed14(&map.source, &source_bridge.forward(&old_return)?, kappa)?;
        assert_eq!(
            chain_return, dual,
            "actual return-chain mismatch at {label}"
        );
        conjugacies += 1;
        println!("CT14|witness|i={i}|label={label}|source={d:?}|image={image:?}|dual={dual:?}|closed_route={route:?}");
        println!("CT14|conjugacy|i={i}|old_source={old_d:?}|old_image={old_image:?}|transported={transported:?}|chain_return={chain_return:?}");
        images.push(image);
    }
    assert_eq!(closed_quadratic, 2);
    let mut pairs = 0;
    for (i, ((_, left), left_image)) in witnesses.iter().zip(&images).enumerate() {
        for j in [i, (i + 1) % witnesses.len()] {
            println!("CT14|attempt|pair|i={i}|j={j}");
            let sum = map.source.add(left, &witnesses[j].1)?;
            map.source.check(&sum)?;
            let image = map.forward(&sum)?;
            let expected = map.target.add(left_image, &images[j])?;
            assert_eq!(image, expected, "homomorphism pair {i},{j}");
            assert_eq!(map.forward_closed(&sum)?, image);
            println!("CT14|pair|i={i}|j={j}|source_sum={sum:?}|image_sum={image:?}");
            pairs += 1;
        }
    }
    assert_eq!(pairs, 34);

    let mut scalar_cases = 0;
    let mut wrong_sign_controls = 0;
    for q in [109, 113] {
        println!("CT14|attempt|prime|q={q}");
        let mut chosen = None;
        for (i, (_, d)) in witnesses.iter().enumerate() {
            let g = map.source.mul(d, 12317 / q)?;
            if g != map.source.zero() {
                chosen = Some((i, g));
                break;
            }
        }
        let (index, g) = chosen.ok_or("no nonzero generator in frozen witness population")?;
        assert_eq!(map.source.mul(&g, q)?, map.source.zero());
        let image = map.forward(&g)?;
        assert_ne!(image, map.target.zero());
        assert_eq!(map.target.mul(&image, q)?, map.target.zero());
        let correct_dual = map.dual(&image)?;
        assert_eq!(correct_dual, map.source.mul(&g, 2)?);
        assert_ne!(map.source.neg(&correct_dual)?, map.source.mul(&g, 2)?);
        wrong_sign_controls += 1;
        println!("CT14|generator|q={q}|first_witness={index}|source={g:?}|image={image:?}");
        println!("CT14|control|wrong_dual_sign|q={q}|detected=true");
        for scalar in [0, 1, 2, 37, q - 1] {
            println!("CT14|attempt|scalar|q={q}|scalar={scalar}");
            let d = map.source.mul(&g, scalar)?;
            let actual = map.forward(&d)?;
            assert_eq!(actual, map.target.mul(&image, scalar)?);
            assert_eq!(map.forward_closed(&d)?, actual);
            let dual = map.dual(&actual)?;
            assert_eq!(dual, map.source.mul(&d, 2)?);
            assert_eq!(map.source.mul(&dual, 6159)?, d);
            println!(
                "CT14|scalar|q={q}|scalar={scalar}|source={d:?}|image={actual:?}|dual={dual:?}"
            );
            scalar_cases += 1;
        }
    }
    assert_eq!(scalar_cases, 10);
    assert_eq!(wrong_sign_controls, 2);
    let malformed = [
        (
            "repeated_pairing",
            Mutation14::None,
            [(0, 4), (1, 5), (2, 4)],
        ),
        ("duplicate_root", Mutation14::DuplicateRoot, PAIRING14),
        ("source_constant", Mutation14::SourceConstant, PAIRING14),
    ];
    let mut constructor_controls = 0;
    for (label, mutation, pairing) in malformed {
        println!("CT14|attempt|constructor_control|{label}");
        match RichelotMap::from_selected_pairing(assessment14(false, mutation)?, pairing) {
            Ok(_) => return Err(format!("malformed assessment accepted: {label}")),
            Err(error) => println!("CT14|control|{label}|detected=true|error={error}"),
        }
        constructor_controls += 1;
    }
    assert_eq!(constructor_controls, 3);
    println!("CT14|accepted|witnesses={}|ordered_pairs={pairs}|conjugacies={conjugacies}|closed_quadratic={closed_quadratic}|prime_generators=2|scalar_cases={scalar_cases}|constructor_controls={constructor_controls}|wrong_sign_controls={wrong_sign_controls}|kappa={kappa}|no_DLP_solver|unsigned",witnesses.len());
    Ok(())
}

#[cfg(test)]
mod tests_14 {
    #[test]
    fn frozen_cubic_inverse_15() {
        super::cubic_inverse15::run().unwrap();
    }
    #[test]
    fn frozen_joint_chart_divisor_map_14() {
        super::run_unit14().unwrap();
    }
}
