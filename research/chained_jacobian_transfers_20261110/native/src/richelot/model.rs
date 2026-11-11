//! Model-tagged genus-two classes and quadratic splittings over the FIXED F101.
//! Rational Weierstrass bases are required in the first source/target factors.
//! This parameterizes models and chart bases, not fields or arbitrary covers.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    sextic: Poly,
    root: i64,
    f: EP,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class {
    model: Model,
    d: ED,
}

fn integers(p: &[F2]) -> Result<Poly, &'static str> {
    if p.iter().any(|c| c.1 != 0) {
        return Err("coefficients did not descend to F101");
    }
    Ok(p.iter().map(|c| c.0).collect())
}
impl Model {
    pub fn new(sextic: Poly, root: i64) -> Result<Self, &'static str> {
        let sextic = trim(sextic);
        let root = fp(root);
        let q = base(&sextic);
        if sextic.len() != 7 {
            return Err("source model must have degree six");
        }
        if egcd(q.clone(), derivative(&q)).0 != base(&[1]) {
            return Err("singular sextic");
        }
        if value(&q, F2::base(root)) != F2::ZERO {
            return Err("base is not Weierstrass");
        }
        let f = chart_of_sextic(&q, root);
        if f.len() != 6 {
            return Err("chart is not a smooth odd quintic");
        }
        Ok(Self { sextic, root, f })
    }
    pub fn sextic(&self) -> &[i64] {
        &self.sextic
    }
    pub fn quintic(&self) -> Poly {
        integers(&self.f).unwrap()
    }
    pub fn root(&self) -> i64 {
        self.root
    }
    pub fn class(&self, u: Poly, v: Poly) -> Result<Class, &'static str> {
        Ok(Class {
            model: self.clone(),
            d: ED::new(base(&trim(u)), base(&trim(v)), &self.f)?,
        })
    }
    pub fn zero(&self) -> Class {
        Class {
            model: self.clone(),
            d: ED::zero(),
        }
    }
    pub fn elliptic_coefficients(&self, q: Quotient) -> Result<(i64, i64, i64), &'static str> {
        if [1, 3, 5].iter().any(|i| self.sextic[*i] != 0) {
            return Err("model is not bielliptic-even");
        }
        let (l, a, b, c) = match q {
            Quotient::F => (
                self.sextic[6],
                self.sextic[4],
                self.sextic[2],
                self.sextic[0],
            ),
            Quotient::G => (
                self.sextic[0],
                self.sextic[2],
                self.sextic[4],
                self.sextic[6],
            ),
        };
        if l == 0 {
            return Err("singular elliptic quotient");
        }
        Ok((a, fp(l * b), fp(l * l * c)))
    }
    pub fn elliptic_points(&self, q: Quotient) -> Result<Vec<Point>, &'static str> {
        let (a, b, c) = self.elliptic_coefficients(q)?;
        let mut pts = vec![None];
        for x in 0..P {
            for y in 0..P {
                if ec_on(point(x, y), a, b, c) {
                    pts.push(point(x, y));
                }
            }
        }
        Ok(pts)
    }
    /// Pullback of the FIRST quotient after X=L*x²,V=L*y normalization.
    pub fn pull(&self, q: Point) -> Result<Class, &'static str> {
        let (a, b, c) = self.elliptic_coefficients(Quotient::F)?;
        if !ec_on(q, a, b, c) {
            return Err("point not on source elliptic curve");
        }
        let (x, y) = match q {
            None => return Ok(self.zero()),
            Some(z) => z,
        };
        if x.1 != 0 || y.1 != 0 {
            return Err("source domain is rational F101 points");
        }
        let il = inv(self.sextic[6]);
        let u = vec![1, fp(2 * self.root), fp(self.root * self.root - x.0 * il)];
        let v = divrem(&[0, 0, 0, fp(y.0 * il)], &trim(u.clone())).1;
        self.class(u, v)
    }
    pub fn norm(&self, d: &Class, q: Quotient) -> Result<Point, &'static str> {
        if d.model != *self {
            return Err("curve/chart mismatch");
        }
        let (a, b, c) = self.elliptic_coefficients(q)?;
        let l = F2::base(match q {
            Quotient::F => self.sextic[6],
            Quotient::G => self.sextic[0],
        });
        let mut out = None;
        let (u, v) = d.coefficients();
        for t in roots(&u)? {
            let yy = eval(&v, t);
            let image = if t == F2::ZERO {
                match q {
                    Quotient::F => None,
                    Quotient::G => Some((F2::ZERO, l.times(yy))),
                }
            } else {
                let x = F2::base(self.root).plus(t.inverse());
                let y = yy.times(t.power(3).inverse());
                match q {
                    Quotient::F => Some((l.times(x.power(2)), l.times(y))),
                    Quotient::G => {
                        if x == F2::ZERO {
                            None
                        } else {
                            Some((
                                l.times(x.power(2).inverse()),
                                l.times(y).times(x.power(3).inverse()),
                            ))
                        }
                    }
                }
            };
            if !ec_on(image, a, b, c) {
                return Err("quotient support outside elliptic curve");
            }
            out = ec_add(out, image, a, b);
        }
        let base_x = match q {
            Quotient::F => F2::base(self.root).power(2),
            Quotient::G => F2::base(self.root).power(2).inverse(),
        };
        if (u.len() - 1) & 1 != 0 {
            out = ec_add(out, Some((l.times(base_x), F2::ZERO)), a, b);
        }
        if let Some((x, y)) = out {
            if x.1 != 0 || y.1 != 0 {
                return Err("norm did not descend");
            }
        }
        Ok(out)
    }
    /// All canonical rational classes for development checks; not a DLP solver.
    pub fn classes(&self) -> Vec<Class> {
        let f = self.quintic();
        let mut out = vec![self.zero()];
        for t in 0..P {
            for y in 0..P {
                if fp(y * y) == eval(&f, F2::base(t)).0 {
                    out.push(self.class(vec![fp(-t), 1], vec![y]).unwrap());
                }
            }
        }
        for b in 0..P {
            for c in 0..P {
                let u = vec![c, b, 1];
                let rem = divrem(&f, &u).1;
                let f0 = *rem.first().unwrap_or(&0);
                let f1 = *rem.get(1).unwrap_or(&0);
                for v1 in 0..P {
                    if v1 == 0 {
                        if f1 == 0 {
                            for v0 in 0..P {
                                if fp(v0 * v0) == f0 {
                                    out.push(self.class(u.clone(), vec![v0]).unwrap());
                                }
                            }
                        }
                    } else {
                        let v0 = fp((f1 + b * v1 * v1) * inv(2 * v1));
                        if fp(v0 * v0 - c * v1 * v1) == f0 {
                            out.push(self.class(u.clone(), vec![v0, v1]).unwrap());
                        }
                    }
                }
            }
        }
        out
    }
}
impl Class {
    pub fn coefficients(&self) -> (Poly, Poly) {
        (integers(&self.d.u).unwrap(), integers(&self.d.v).unwrap())
    }
    pub fn model(&self) -> &Model {
        &self.model
    }
    pub fn add(&self, b: &Self) -> Result<Self, &'static str> {
        if self.model != b.model {
            return Err("curve/chart mismatch");
        }
        Ok(Self {
            model: self.model.clone(),
            d: self.d.add(&b.d, &self.model.f),
        })
    }
    pub fn mul(&self, n: u64) -> Self {
        Self {
            model: self.model.clone(),
            d: self.d.mul(n, &self.model.f),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Richelot {
    source: Model,
    target: Model,
    g: [EP; 3],
    h: [EP; 3],
    delta: i64,
}
impl Richelot {
    pub fn new(
        factors: [[i64; 3]; 3],
        source_root: i64,
        target_root: i64,
    ) -> Result<Self, &'static str> {
        let g = factors.map(|p| base(&trim(p.to_vec())));
        if g.iter().any(|p| p.len() != 3) {
            return Err("three quadratic factors required");
        }
        let mut delta = F2::ZERO;
        for i in 0..3 {
            delta = delta.plus(g[0][i].times(g[1][(i + 1) % 3]).times(g[2][(i + 2) % 3]));
            delta = delta.minus(g[0][i].times(g[1][(i + 2) % 3]).times(g[2][(i + 1) % 3]));
        }
        if delta == F2::ZERO {
            return Err("degenerate Richelot splitting");
        }
        let h: [EP; 3] = std::array::from_fn(|i| {
            let a = &g[(i + 1) % 3];
            let b = &g[(i + 2) % 3];
            times(
                &minus(&times(&derivative(a), b), &times(a, &derivative(b))),
                &[delta.inverse()],
            )
        });
        let source = Model::new(integers(&times(&times(&g[0], &g[1]), &g[2]))?, source_root)?;
        let target = Model::new(integers(&times(&times(&h[0], &h[1]), &h[2]))?, target_root)?;
        if value(&g[0], F2::base(source.root)) != F2::ZERO
            || value(&h[0], F2::base(target.root)) != F2::ZERO
        {
            return Err("bases must lie on first source/target factors");
        }
        Ok(Self {
            source,
            target,
            g,
            h,
            delta: delta.0,
        })
    }
    pub fn source(&self) -> &Model {
        &self.source
    }
    pub fn target(&self) -> &Model {
        &self.target
    }
    pub fn determinant(&self) -> i64 {
        self.delta
    }
    pub fn transfer(&self, d: &Class) -> Result<Class, &'static str> {
        if d.model != self.source {
            return Err("source curve/chart mismatch");
        }
        let (u, v) = d.coefficients();
        let image = evaluate(
            &u,
            &v,
            &self.target.f,
            &self.g,
            &self.h,
            self.source.root,
            self.target.root,
        )?;
        Ok(Class {
            model: self.target.clone(),
            d: image,
        })
    }
    pub fn dual(&self, d: &Class) -> Result<Class, &'static str> {
        if d.model != self.target {
            return Err("target curve/chart mismatch");
        }
        let (u, v) = d.coefficients();
        let image = evaluate(
            &u,
            &v,
            &self.source.f,
            &self.h,
            &self.g,
            self.target.root,
            self.source.root,
        )?
        .neg();
        Ok(Class {
            model: self.source.clone(),
            d: image,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_fixture_agrees_and_curve_tags_reject_mixing() {
        let r = Richelot::new([[1, 0, 1], [-1, 10, 1], [-1, -10, 1]], 10, 10).unwrap();
        for q in ec_points() {
            let d = r.source.pull(q).unwrap();
            let image = r.transfer(&d).unwrap();
            assert_eq!(d.coefficients(), {
                let x = pull(q).unwrap();
                (x.u, x.v)
            });
            assert_eq!(
                image.coefficients(),
                transfer(&pull(q).unwrap()).unwrap().coefficients()
            );
            assert_eq!(
                r.target.norm(&image, Quotient::F).unwrap(),
                collapsed_endpoint(q)
            );
        }
        assert!(r.source.zero().add(&r.target.zero()).is_err());
        assert!(r.transfer(&r.target.zero()).is_err());
        assert!(Richelot::new([[1, 0, 1], [1, 0, 1], [-1, 1, 1]], 10, 10).is_err());
    }
    #[test]
    fn successor_full_domain_and_nonisomorphic_rational_two_torsion() {
        let r = Richelot::new([[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
        assert_eq!(r.determinant(), 91);
        assert_eq!(r.source.sextic(), &[85, 0, 9, 0, 6, 0, 1]);
        assert_eq!(r.target.sextic(), &[21, 0, 83, 0, 81, 0, 20]);
        let source = r.source.classes();
        let target = r.target.classes();
        assert_eq!(source.len(), target.len());
        let two_source = source
            .iter()
            .filter(|d| d.mul(2) == r.source.zero())
            .count();
        let two_target = target
            .iter()
            .filter(|d| d.mul(2) == r.target.zero())
            .count();
        assert_eq!((two_source, two_target), (4, 16));
        let mut fibers = std::collections::BTreeMap::new();
        let translate = &source[111];
        let it = r.transfer(translate).unwrap();
        for d in &source {
            let image = r.transfer(d).unwrap();
            *fibers.entry(image.coefficients()).or_insert(0_usize) += 1;
            assert_eq!(r.dual(&image).unwrap(), d.mul(2));
            assert_eq!(
                r.transfer(&d.add(translate).unwrap()).unwrap(),
                image.add(&it).unwrap()
            );
        }
        assert!(fibers.values().all(|n| *n == 4));
        assert_eq!(fibers.len() * 4, source.len());
        for d in &target {
            assert_eq!(r.transfer(&r.dual(d).unwrap()).unwrap(), d.mul(2));
        }
        let points = r.source.elliptic_points(Quotient::F).unwrap();
        let (a, b, _) = r.source.elliptic_coefficients(Quotient::F).unwrap();
        for q in &points {
            let d = r.source.pull(*q).unwrap();
            assert_eq!(r.source.norm(&d, Quotient::F).unwrap(), ec_mul(*q, 2, a, b));
        }
        let order = points.len() as u64;
        let prime = (3..=order)
            .rev()
            .find(|v| order.is_multiple_of(*v) && (2..*v).all(|d| !v.is_multiple_of(d)))
            .unwrap();
        let p = points
            .iter()
            .map(|q| ec_mul(*q, order / prime, a, b))
            .find(|q| q.is_some())
            .unwrap();
        let image = r.transfer(&r.source.pull(p).unwrap()).unwrap();
        assert_ne!(image, r.target.zero());
        assert_eq!(image.mul(prime), r.target.zero());
        let first = r.target.norm(&image, Quotient::F).unwrap();
        let second = r.target.norm(&image, Quotient::G).unwrap();
        assert!(first.is_some() || second.is_some());
        for k in 0..prime {
            let q = ec_mul(p, k, a, b);
            assert_eq!(
                r.transfer(&r.source.pull(q).unwrap()).unwrap(),
                image.mul(k)
            );
        }
        println!("successor: #J={}, two-torsion source={} target={}, #E={}, r={}, P={p:?}, normF={first:?}, normG={second:?}",source.len(),two_source,two_target,order,prime);
    }

    // Predeclared control: the standard elliptic quotient by (1,0), after
    // shifting U=u-1, and ALL F101 coordinate isomorphisms of its codomain.
    // Testing a finite function equality is not a geometric identity proof.
    #[test]
    fn successor_complete_elliptic_route_deduplication() {
        let r = Richelot::new([[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
        let (a, b, c) = r.target.elliptic_coefficients(Quotient::F).unwrap();
        assert_eq!((a, b, c), (81, 44, 17));
        let quotient = |q: Point| {
            q.and_then(|(x, y)| {
                let u = x.minus(F2::ONE);
                if u == F2::ZERO {
                    None
                } else {
                    Some((
                        u.plus(F2::base(9)).plus(F2::base(24).times(u.inverse())),
                        y.times(F2::ONE.minus(F2::base(24).times(u.power(2).inverse()))),
                    ))
                }
            })
        };
        let points = r.source.elliptic_points(Quotient::F).unwrap();
        let mut shortcuts = vec![];
        for k in 1..P {
            for s in 1..P {
                if fp(k * k) != pow(s, 3) {
                    continue;
                }
                let t = fp((83 * s - a) * inv(3));
                if fp(3 * t * t + 2 * a * t + b) != fp(86 * s * s)
                    || fp(t * t * t + a * t * t + b * t + c) != 0
                {
                    continue;
                }
                let matches = points.iter().all(|q| {
                    let endpoint = r
                        .target
                        .norm(
                            &r.transfer(&r.source.pull(*q).unwrap()).unwrap(),
                            Quotient::F,
                        )
                        .unwrap();
                    let direct = quotient(ec_mul(*q, 2, 6, 9)).map(|(x, y)| {
                        (F2::base(s).times(x).plus(F2::base(t)), F2::base(k).times(y))
                    });
                    assert!(ec_on(direct, a, b, c));
                    endpoint == direct
                });
                if matches {
                    shortcuts.push((s, t, k));
                }
            }
        }
        assert_eq!(shortcuts, vec![(100, 80, 91)]);
        println!(
            "successor complete {}-point endpoint shortcuts tau(X,Y)=(sX+t,kY): {shortcuts:?}",
            points.len()
        );
    }

    #[test]
    fn successor_both_norms_on_arbitrary_classes() {
        let r = Richelot::new([[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
        for model in [r.source(), r.target()] {
            let classes = model.classes();
            let h = &classes[111];
            for q in [Quotient::F, Quotient::G] {
                let (a, b, _) = model.elliptic_coefficients(q).unwrap();
                let nh = model.norm(h, q).unwrap();
                for d in &classes {
                    assert_eq!(
                        model.norm(&d.add(h).unwrap(), q).unwrap(),
                        ec_add(model.norm(d, q).unwrap(), nh, a, b)
                    );
                }
            }
        }
    }
}
