//! Explicit Miller functions and intrinsic localized Kummer quotient charts.
use crate::{large_field::*, reverse06::dp, round02, round04};
use serde_json::{json, Value};
use std::path::Path;
#[path = "descent09.rs"]
pub(crate) mod descent09;
#[derive(Clone)]
struct Fn {
    a: Poly,
    b: Poly,
    d: Poly,
}
struct Curve<'a> {
    f: &'a F,
    ss: bool,
    c: u64,
}
impl Curve<'_> {
    fn constant(&self, v: &E) -> Fn {
        Fn {
            a: vec![v.clone()],
            b: vec![],
            d: vec![self.f.o()],
        }
    }
    fn sum_fn(&self, p: &Fn, q: &Fn) -> Fn {
        self.reduce(
            add(&mul(self.f, &p.a, &q.d), &mul(self.f, &q.a, &p.d)),
            add(&mul(self.f, &p.b, &q.d), &mul(self.f, &q.b, &p.d)),
            mul(self.f, &p.d, &q.d),
        )
    }
    fn inverse_fn(&self, p: &Fn) -> Fn {
        let f = self.f;
        let top = add(&p.a, &mul(f, &p.b, &self.h()));
        let den = add(
            &add(&square(f, &p.a), &mul(f, &mul(f, &p.a, &p.b), &self.h())),
            &mul(f, &square(f, &p.b), &self.rhs()),
        );
        self.reduce(mul(f, &top, &p.d), mul(f, &p.b, &p.d), den)
    }
    fn power_fn(&self, p: &Fn, n: u64) -> Fn {
        let mut z = self.one();
        for bit in (0..64 - n.leading_zeros()).rev() {
            z = self.mul(&z, &z);
            if n >> bit & 1 == 1 {
                z = self.mul(&z, p)
            }
        }
        z
    }
    fn equal_fn(&self, p: &Fn, q: &Fn) -> bool {
        mul(self.f, &p.a, &q.d) == mul(self.f, &q.a, &p.d)
            && mul(self.f, &p.b, &q.d) == mul(self.f, &q.b, &p.d)
    }
    fn translation(&self, r: &P) -> (Fn, Fn) {
        let f = self.f;
        let P::Affine { x, y } = r else {
            return (
                Fn {
                    a: vec![f.z(), f.o()],
                    b: vec![],
                    d: vec![f.o()],
                },
                Fn {
                    a: vec![],
                    b: vec![f.o()],
                    d: vec![f.o()],
                },
            );
        };
        let xvar = Fn {
            a: vec![f.z(), f.o()],
            b: vec![],
            d: vec![f.o()],
        };
        let yvar = Fn {
            a: vec![],
            b: vec![f.o()],
            d: vec![f.o()],
        };
        let l = Fn {
            a: vec![y.clone()],
            b: vec![f.o()],
            d: vec![x.clone(), f.o()],
        };
        let mut xx = self.sum_fn(&self.mul(&l, &l), &self.sum_fn(&xvar, &self.constant(x)));
        if !self.ss {
            xx = self.sum_fn(&xx, &self.sum_fn(&l, &self.constant(&f.e(self.c))));
        }
        let mut yy = self.sum_fn(&self.mul(&l, &self.sum_fn(&xvar, &xx)), &yvar);
        yy = self.sum_fn(
            &yy,
            &if self.ss {
                self.constant(&f.o())
            } else {
                xx.clone()
            },
        );
        (xx, yy)
    }
    fn evaluate_poly_fn(&self, p: &Poly, x: &Fn) -> Fn {
        p.iter().rev().fold(self.constant(&self.f.z()), |z, c| {
            self.sum_fn(&self.mul(&z, x), &self.constant(c))
        })
    }
    fn compose_translation(&self, p: &Fn, r: &P) -> Fn {
        let (x, y) = self.translation(r);
        let a = self.evaluate_poly_fn(&p.a, &x);
        let b = self.evaluate_poly_fn(&p.b, &x);
        let d = self.evaluate_poly_fn(&p.d, &x);
        self.mul(&self.sum_fn(&a, &self.mul(&b, &y)), &self.inverse_fn(&d))
    }
    fn h(&self) -> Poly {
        if self.ss {
            vec![self.f.o()]
        } else {
            vec![self.f.z(), self.f.o()]
        }
    }
    fn rhs(&self) -> Poly {
        if self.ss {
            vec![self.f.e(self.c), self.f.o(), self.f.z(), self.f.o()]
        } else {
            vec![self.f.o(), self.f.z(), self.f.e(self.c), self.f.o()]
        }
    }
    fn valid(&self, p: &P) -> bool {
        match p {
            P::Infinity => true,
            P::Affine { x, y } => {
                self.f.sq(y).add(&self.f.mul(&self.f.eval(&self.h(), x), y))
                    == self.f.eval(&self.rhs(), x)
            }
        }
    }
    fn neg(&self, p: &P) -> P {
        match p {
            P::Infinity => P::Infinity,
            P::Affine { x, y } => P::Affine {
                x: x.clone(),
                y: y.add(&self.f.eval(&self.h(), x)),
            },
        }
    }
    fn add(&self, p: &P, q: &P) -> P {
        let (P::Affine { x, y }, P::Affine { x: u, y: v }) = (p, q) else {
            return if *p == P::Infinity {
                q.clone()
            } else {
                p.clone()
            };
        };
        if x == u && y != v {
            return P::Infinity;
        }
        let f = self.f;
        let l = if p == q {
            if self.ss {
                f.sq(x).add(&f.o())
            } else {
                x.add(&f.mul(y, &f.inv(x)))
            }
        } else {
            f.mul(&y.add(v), &f.inv(&x.add(u)))
        };
        let nx = if self.ss {
            f.sq(&l).add(x).add(u)
        } else {
            f.sq(&l).add(&l).add(x).add(u).add(&f.e(self.c))
        };
        let ny = if self.ss {
            f.mul(&l, &x.add(&nx)).add(y).add(&f.o())
        } else {
            f.mul(&l, &x.add(&nx)).add(y).add(&nx)
        };
        let z = P::Affine { x: nx, y: ny };
        assert!(self.valid(&z));
        z
    }
    fn times(&self, p: &P, n: u64) -> P {
        (0..n).fold(P::Infinity, |z, _| self.add(&z, p))
    }
    fn one(&self) -> Fn {
        Fn {
            a: vec![self.f.o()],
            b: vec![],
            d: vec![self.f.o()],
        }
    }
    fn reduce(&self, a: Poly, b: Poly, d: Poly) -> Fn {
        let f = self.f;
        assert!(!d.is_empty());
        let g = gcdp(f, &gcdp(f, &a, &b), &d);
        let (a, b, d) = (div(f, &a, &g).0, div(f, &b, &g).0, div(f, &d, &g).0);
        let s = f.inv(d.last().unwrap());
        let (a, b, d) = (scale(f, &a, &s), scale(f, &b, &s), scale(f, &d, &s));
        assert!(a.len() <= 257 && b.len() <= 257 && d.len() <= 257);
        Fn { a, b, d }
    }
    fn mul(&self, p: &Fn, q: &Fn) -> Fn {
        let f = self.f;
        let bb = mul(f, &p.b, &q.b);
        let a = add(&mul(f, &p.a, &q.a), &mul(f, &bb, &self.rhs()));
        let b = add(
            &add(&mul(f, &p.a, &q.b), &mul(f, &p.b, &q.a)),
            &mul(f, &bb, &self.h()),
        );
        self.reduce(a, b, mul(f, &p.d, &q.d))
    }
    fn line(&self, p: &P, q: &P) -> Fn {
        if *p == P::Infinity || *q == P::Infinity {
            return self.one();
        }
        let f = self.f;
        let (P::Affine { x, y }, P::Affine { x: u, y: v }) = (p, q) else {
            unreachable!()
        };
        let sum = self.add(p, q);
        if sum == P::Infinity {
            return Fn {
                a: vec![x.clone(), f.o()],
                b: vec![],
                d: vec![f.o()],
            };
        }
        let l = if p == q {
            if self.ss {
                f.sq(x).add(&f.o())
            } else {
                x.add(&f.mul(y, &f.inv(x)))
            }
        } else {
            f.mul(&y.add(v), &f.inv(&x.add(u)))
        };
        let P::Affine { x: s, .. } = sum else {
            unreachable!()
        };
        Fn {
            a: vec![y.add(&f.mul(&l, x)), l],
            b: vec![f.o()],
            d: vec![s, f.o()],
        }
    }
    fn miller(&self, t: &P) -> (Fn, Vec<Value>) {
        if *t == P::Infinity {
            return (self.one(), vec![]);
        }
        let mut p = t.clone();
        let mut out = self.one();
        let mut chain = vec![];
        for bit in [0, 0, 1, 1] {
            out = self.mul(&self.mul(&out, &out), &self.line(&p, &p));
            let q = self.add(&p, &p);
            chain.push(json!({"operation":"double","P":pj(&p),"sum":pj(&q)}));
            p = q;
            if bit == 1 {
                out = self.mul(&out, &self.line(&p, t));
                let q = self.add(&p, t);
                chain.push(json!({"operation":"add_T","P":pj(&p),"sum":pj(&q)}));
                p = q;
            }
        }
        assert_eq!(p, P::Infinity);
        (out, chain)
    }
    fn eval(&self, p: &Fn, q: &P) -> Option<E> {
        let P::Affine { x, y } = q else { return None };
        let d = self.f.eval(&p.d, x);
        if d.is_zero() {
            None
        } else {
            Some(
                self.f.mul(
                    &self
                        .f
                        .eval(&p.a, x)
                        .add(&self.f.mul(&self.f.eval(&p.b, x), y)),
                    &self.f.inv(&d),
                ),
            )
        }
    }
    // Separate pointwise Miller recurrence; it never evaluates a generated polynomial.
    fn line_value(&self, p: &P, q: &P, r: &P) -> Option<E> {
        let f = self.f;
        let P::Affine { x: rx, y: ry } = r else {
            return None;
        };
        if *p == P::Infinity || *q == P::Infinity {
            return Some(f.o());
        }
        let (P::Affine { x, y }, P::Affine { x: u, y: v }) = (p, q) else {
            unreachable!()
        };
        let s = self.add(p, q);
        if s == P::Infinity {
            return Some(rx.add(x));
        }
        let l = if p == q {
            if self.ss {
                f.sq(x).add(&f.o())
            } else {
                x.add(&f.mul(y, &f.inv(x)))
            }
        } else {
            f.mul(&y.add(v), &f.inv(&x.add(u)))
        };
        let P::Affine { x: s, .. } = s else {
            unreachable!()
        };
        let d = rx.add(&s);
        if d.is_zero() {
            None
        } else {
            Some(f.mul(&ry.add(y).add(&f.mul(&l, &rx.add(x))), &f.inv(&d)))
        }
    }
    fn point_value(&self, t: &P, r: &P) -> Option<E> {
        if *t == P::Infinity {
            return Some(self.f.o());
        }
        let mut p = t.clone();
        let mut z = self.f.o();
        for bit in [0, 0, 1, 1] {
            z = self.f.mul(&self.f.sq(&z), &self.line_value(&p, &p, r)?);
            p = self.add(&p, &p);
            if bit == 1 {
                z = self.f.mul(&z, &self.line_value(&p, t, r)?);
                p = self.add(&p, t);
            }
        }
        assert_eq!(p, P::Infinity);
        Some(z)
    }
    fn shifted(&self, p: &Poly, x: &E) -> Poly {
        let f = self.f;
        let mut powers = vec![f.o()];
        for _ in 1..p.len() {
            powers.push(f.mul(powers.last().unwrap(), x));
        }
        let mut out = vec![f.z(); p.len()];
        for n in 0..p.len() {
            for k in 0..=n {
                if k & n == k {
                    out[k] = out[k].add(&f.mul(&p[n], &powers[n - k]));
                }
            }
        }
        trim(out)
    }
    fn valuation(&self, p: &Fn, q: &P) -> i64 {
        let f = self.f;
        let P::Affine { x, y } = q else {
            unreachable!()
        };
        let n = 64;
        let rhs = self.shifted(&self.rhs(), x);
        let mut ys = vec![f.z(); n];
        ys[0] = y.clone();
        let hi = if self.ss { f.o() } else { f.inv(x) };
        for k in 1..n {
            let mut v = rhs.get(k).cloned().unwrap_or_else(|| f.z());
            if k % 2 == 0 {
                v = v.add(&f.sq(&ys[k / 2]));
            }
            if !self.ss {
                v = v.add(&ys[k - 1]);
            }
            ys[k] = f.mul(&v, &hi);
        }
        let (a, b, d) = (
            self.shifted(&p.a, x),
            self.shifted(&p.b, x),
            self.shifted(&p.d, x),
        );
        let mut v = vec![f.z(); n];
        for k in 0..n {
            v[k] = a.get(k).cloned().unwrap_or_else(|| f.z());
            for i in 0..=k.min(b.len().saturating_sub(1)) {
                if let Some(b) = b.get(i) {
                    v[k] = v[k].add(&f.mul(b, &ys[k - i]));
                }
            }
        }
        let top = v.iter().position(|v| !v.is_zero()).expect("jet bound64");
        let bot = d.iter().position(|v| !v.is_zero()).unwrap();
        top as i64 - bot as i64
    }
    fn point(&self, x: &E) -> Option<P> {
        let f = self.f;
        if self.ss {
            Some(P::Affine {
                x: x.clone(),
                y: f.quadratic(&f.eval(&self.rhs(), x))?,
            })
        } else {
            f.point(&f.curve(f.e(self.c), f.o()), x)
        }
    }
}
fn fn_json(p: &Fn) -> Value {
    json!({"A":polyj(&p.a),"B":polyj(&p.b),"D":polyj(&p.d)})
}
fn fn_check(c: &Curve, t: &P, p: &Fn) -> Value {
    if *t == P::Infinity {
        return json!({"identity":true,"samples":0,"subgroup_valuations":[]});
    }
    let f = c.f;
    assert!(c.valid(t));
    assert_eq!(c.times(t, 19), P::Infinity);
    let aw = if p.a.is_empty() {
        -1
    } else {
        2 * (p.a.len() as i64 - 1)
    };
    let bw = if p.b.is_empty() {
        -1
    } else {
        2 * (p.b.len() as i64 - 1) + 3
    };
    let pole = aw.max(bw) - 2 * (p.d.len() as i64 - 1);
    assert_eq!(pole, 19);
    let leading = if aw > bw {
        p.a.last().unwrap()
    } else {
        p.b.last().unwrap()
    };
    assert_eq!(*leading, f.o());
    assert_eq!(*p.d.last().unwrap(), f.o());
    let mut denominator = p.d.clone();
    let mut roots = vec![];
    for i in 1..=9 {
        let P::Affine { x, .. } = c.times(t, i) else {
            unreachable!()
        };
        let mut mult = 0;
        while denominator.len() > 1 && f.eval(&denominator, &x).is_zero() {
            denominator = div(f, &denominator, &vec![x.clone(), f.o()]).0;
            mult += 1;
        }
        if mult > 0 {
            roots.push(json!({"multiple":i,"multiplicity":mult}));
        }
    }
    assert_eq!(denominator, vec![f.o()]);
    let mut vals = vec![];
    for i in 1..19 {
        let q = c.times(t, i);
        let v = c.valuation(p, &q);
        assert_eq!(v, if i == 1 { 19 } else { 0 });
        vals.push(v);
    }
    let mut samples = vec![];
    for x in 1..4096 {
        let Some(q) = c.point(&f.e(x)) else { continue };
        if c.times(&q, 19) == P::Infinity {
            continue;
        }
        let (Some(a), Some(b)) = (c.eval(p, &q), c.point_value(t, &q)) else {
            continue;
        };
        assert_eq!(a, b);
        assert!(!a.is_zero());
        samples.push(json!({"point":pj(&q),"value":ej(&a)}));
        if samples.len() == 16 {
            break;
        }
    }
    assert_eq!(samples.len(), 16);
    json!({"pole_order":pole,"leading_at_origin":1,"denominator_support":roots,"subgroup_valuations":vals,"samples":samples,"divisor":"19[T]-19[O]; denominator support, all finite valuations and origin pole checked"})
}
pub fn run(dir: &Path) {
    let input = round04::read(&dir.join("q5-mixed-translation-atlases.json"));
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let mut records = vec![];
    let mut functions = 0;
    let mut valuation_checks = 0;
    let mut sample_checks = 0;
    for (ri, row) in input["records"].as_array().unwrap().iter().enumerate() {
        let trace = row["target_trace"].as_i64().unwrap();
        let ordinary = Curve {
            f: &f,
            ss: false,
            c: if trace == 1 { 1 } else { 0 },
        };
        let ss = Curve {
            f: &f,
            ss: true,
            c: if trace == 1 { 1 } else { 0 },
        };
        let mut generators = vec![];
        let mut equations = vec![];
        for (gi, index) in [1, 19].into_iter().enumerate() {
            let kernel = &row["kernel"][index];
            let mut factors = vec![];
            for j in 0..5 {
                let c = if j == 4 { &ss } else { &ordinary };
                let t = dp(&f, &kernel["point"][j]);
                let (p, chain) = c.miller(&t);
                let check = fn_check(c, &t, &p);
                if t != P::Infinity {
                    valuation_checks += 18;
                    sample_checks += 16;
                }
                functions += 1;
                factors.push(json!({"factor":j,"torsion_point":pj(&t),"function":fn_json(&p),"chain":chain,"checks":check}));
            }
            generators.push(json!({"kernel_index":index,"factors":factors}));
            equations.push(format!("z{}^19 * product_(j=0..4) D_{}_j(x_j) + product_(j=0..4) (A_{}_j(x_j)+B_{}_j(x_j)*y_j) = 0",gi+1,gi+1,gi+1,gi+1));
        }
        let s1 = dp(&f, &row["kernel"][1]["point"][4]);
        let s2 = dp(&f, &row["kernel"][19]["point"][4]);
        assert!((0..19).all(|i| ss.times(&s1, i) != s2));
        records.push(json!({"graph_record":ri,"target_trace":trace,"supersingular_trace":2*trace,"generators":generators,"base_equations":"j<4:y_j^2+x_j*y_j=x_j^3+c*x_j^2+1; j=4:y_4^2+y_4=x_4^3+x_4+c; c=(trace+1)/2","intrinsic_Kummer_equations":equations,"localization":"invert every A_ij(x_j)+B_ij(x_j)y_j and D_ij(x_j); add w*Delta+1=0 for their product Delta","cover_degree":361,"dimension":5,"independent_graph_classes_checked":true,"jacobian_unit":"diagonal z_1^18*product D_1j, z_2^18*product D_2j; 19=1 in characteristic2; all factors are units on chart","identification":"written dual-isogeny/Kummer argument with rigidification at origin; see PROOF_MAP.md","global_normalization_and_F2_descent":"OPEN","Jacobian_realization":false}));
        println!("Q6 graph{ri}:10 explicit Miller functions, divisor checks and localized quotient equations");
    }
    round02::save(
        dir,
        "q6-intrinsic-Kummer-charts.json",
        &json!({"schema":"koblitz.round08/q6-v1","field":input["field"],"records":records,"functions":functions,"finite_valuation_checks":valuation_checks,"pointwise_value_checks":sample_checks,"scope":"explicit localized intrinsic Kummer equations over F2^360, exact Miller divisors and written dual-quotient identification; no full projective normalization/F2 descent or Jacobian equation"}),
    );
    println!("Q6:{functions} functions,{valuation_checks} valuations,{sample_checks} separate pointwise checks");
}

pub fn verify(dir: &Path) {
    let input = round04::read(&dir.join("q6-intrinsic-Kummer-charts.json"));
    let f = F::new(360, 45);
    let mut checked = 0;
    for row in input["records"].as_array().unwrap() {
        let trace = row["target_trace"].as_i64().unwrap();
        for generator in row["generators"].as_array().unwrap() {
            for factor in generator["factors"].as_array().unwrap() {
                let j = factor["factor"].as_u64().unwrap();
                let c = Curve {
                    f: &f,
                    ss: j == 4,
                    c: if trace == 1 { 1 } else { 0 },
                };
                let t = dp(&f, &factor["torsion_point"]);
                let q = &factor["function"];
                let p = Fn {
                    a: crate::reverse06::poly(&f, &q["A"]),
                    b: crate::reverse06::poly(&f, &q["B"]),
                    d: crate::reverse06::poly(&f, &q["D"]),
                };
                assert_eq!(fn_check(&c, &t, &p), factor["checks"]);
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 40);
    round02::save(
        dir,
        "q6b-parsed-Miller-check.json",
        &json!({"schema":"koblitz.round08/q6b-v1","parsed_functions":checked,"finite_valuations":720,"pointwise_checks":640,"scope":"parse coefficient arrays and recheck divisor support,jets,poles and separate pointwise Miller values; shared field primitives"}),
    );
    println!("Q6 parser:all40 stored coefficient functions checked");
}

pub fn glue(dir: &Path) {
    use num_bigint::BigUint;
    use num_traits::{One, Zero};
    let input = round04::read(&dir.join("q6-intrinsic-Kummer-charts.json"));
    let f = F::new(360, 45);
    let qminus = (BigUint::one() << 360) - BigUint::one();
    let m = &qminus / 19u64;
    assert_ne!(&m % 19u64, BigUint::zero());
    let k = (0..19u64)
        .find(|k| (BigUint::one() + &m * *k) % 19u64 == BigUint::zero())
        .unwrap();
    let root_exponent = (BigUint::one() + &m * k) / 19u64;
    let mut records = vec![];
    let mut total = 0;
    for row in input["records"].as_array().unwrap() {
        let trace = row["target_trace"].as_i64().unwrap();
        let mut shifts = vec![];
        let mut factor_checks = vec![];
        for j in 0..5 {
            let c = Curve {
                f: &f,
                ss: j == 4,
                c: if trace == 1 { 1 } else { 0 },
            };
            let (s, r) = (1..4096)
                .filter_map(|x| c.point(&f.e(x)))
                .find_map(|s| {
                    let r = c.times(&s, 19);
                    if (1..=5).all(|d| c.times(&r, 19 * d) != P::Infinity) {
                        Some((s, r))
                    } else {
                        None
                    }
                })
                .expect("translation sample budget");
            let mut bad = vec![P::Infinity];
            for gen in row["generators"].as_array().unwrap() {
                bad.push(dp(&f, &gen["factors"][j]["torsion_point"]));
            }
            let mut translated_bad = std::collections::BTreeSet::new();
            for kk in 0..6 {
                let rr = c.times(&r, kk);
                for t in &bad {
                    let p = c.add(t, &c.neg(&rr));
                    assert!(translated_bad.insert((kk, pj(&p).to_string())));
                }
            }
            // Distinct chart indices must have disjoint bad sets on this factor.
            for kk in 0..6 {
                for ll in 0..kk {
                    let a = c.times(&r, kk);
                    let b = c.times(&r, ll);
                    for u in &bad {
                        for v in &bad {
                            assert_ne!(c.add(u, &c.neg(&a)), c.add(v, &c.neg(&b)));
                        }
                    }
                }
            }
            shifts.push(json!({"factor":j,"S":pj(&s),"R19S":pj(&r),"chart_shifts":(0..6).map(|k|pj(&c.times(&r,k))).collect::<Vec<_>>(),"bad_sets_pairwise_disjoint":true}));
            for (gi, gen) in row["generators"].as_array().unwrap().iter().enumerate() {
                let factor = &gen["factors"][j];
                let t = dp(&f, &factor["torsion_point"]);
                let p = &factor["function"];
                let fun = Fn {
                    a: crate::reverse06::poly(&f, &p["A"]),
                    b: crate::reverse06::poly(&f, &p["B"]),
                    d: crate::reverse06::poly(&f, &p["D"]),
                };
                assert_eq!(
                    fun.d,
                    vec![f.o()],
                    "six-chart bad-set proof requires polynomial Miller functions"
                );
                for kk in 1..6 {
                    let rr = c.times(&r, kk);
                    let kappa = c.eval(&fun, &rr).expect("shift off divisor");
                    assert!(!kappa.is_zero());
                    assert_eq!(f.pow(&kappa, &m), f.o());
                    let root = f.pow(&kappa, &root_exponent);
                    assert_eq!(f.powu(&root, 19), kappa);
                    let P::Affine { x, .. } = &rr else {
                        unreachable!()
                    };
                    let vertical = Fn {
                        a: vec![x.clone(), f.o()],
                        b: vec![],
                        d: vec![f.o()],
                    };
                    let h = c.mul(
                        &c.line(&c.add(&t, &c.neg(&rr)), &rr),
                        &c.inverse_fn(&vertical),
                    );
                    let shifted = c.compose_translation(&fun, &rr);
                    let rhs = c.mul(&c.mul(&c.power_fn(&h, 19), &fun), &c.constant(&kappa));
                    assert!(
                        c.equal_fn(&shifted, &rhs),
                        "exact translated Miller identity"
                    );
                    let transition = c.mul(&h, &c.constant(&root));
                    factor_checks.push(json!({"factor":j,"generator":gi,"chart":kk,"kappa":ej(&kappa),"kappa_root19":ej(&root),"transition_from_zero":fn_json(&transition),"exact_identity_checked":true}));
                    total += 1;
                }
            }
        }
        records.push(json!({"graph_record":row["graph_record"],"shifts":shifts,"transition_factors":factor_checks,"chart_count":6,"cover_argument":"each factor excludes at most one chart;five factors,six charts","Kummer_coordinate_transition":"z_i,l=z_i,k*product_j rho_i,j,l(P_j)/rho_i,j,k(P_j);rho_i,j,0=1;functions evaluated in common unshifted base coordinates","cocycle":"ratios telescope identically;base coordinate transitions are translations by R_l-R_k","F2_coordinate_descent":"OPEN"}));
        println!(
            "Q6c graph{}:six charts,50 exact transition identities",
            row["graph_record"]
        );
    }
    assert_eq!(total, 200);
    round02::save(
        dir,
        "q6c-global-chart-gluing.json",
        &json!({"schema":"koblitz.round08/q6c-v1","records":records,"exact_translation_identities":total,"charts_each_graph":6,"root19_exponent":root_exponent.to_string(),"scope":"explicit covering atlas and Kummer transitions over F2^360;exact rational identities and roots;F2 coordinate descent remains open"}),
    );
}

fn parsed(f: &F, v: &Value) -> Fn {
    Fn {
        a: crate::reverse06::poly(f, &v["A"]),
        b: crate::reverse06::poly(f, &v["B"]),
        d: crate::reverse06::poly(f, &v["D"]),
    }
}
pub fn verify_glue(dir: &Path) {
    let charts = round04::read(&dir.join("q6c-global-chart-gluing.json"));
    let functions = round04::read(&dir.join("q6-intrinsic-Kummer-charts.json"));
    let f = F::new(360, 45);
    let mut checked = 0;
    let mut points = 0;
    for row in charts["records"].as_array().unwrap() {
        let ri = row["graph_record"].as_u64().unwrap() as usize;
        let base = &functions["records"][ri];
        let trace = base["target_trace"].as_i64().unwrap();
        for rec in row["transition_factors"].as_array().unwrap() {
            let j = rec["factor"].as_u64().unwrap() as usize;
            let gi = rec["generator"].as_u64().unwrap() as usize;
            let k = rec["chart"].as_u64().unwrap() as usize;
            let c = Curve {
                f: &f,
                ss: j == 4,
                c: if trace == 1 { 1 } else { 0 },
            };
            let r = dp(&f, &row["shifts"][j]["chart_shifts"][k]);
            let p = parsed(&f, &base["generators"][gi]["factors"][j]["function"]);
            assert_eq!(p.d, vec![f.o()]);
            let rho = parsed(&f, &rec["transition_from_zero"]);
            let root = crate::reverse06::decode(&f, &rec["kappa_root19"]);
            let kappa = crate::reverse06::decode(&f, &rec["kappa"]);
            assert_eq!(f.powu(&root, 19), kappa);
            assert!(c.equal_fn(
                &c.compose_translation(&p, &r),
                &c.mul(&p, &c.power_fn(&rho, 19))
            ));
            assert!(c.equal_fn(&c.mul(&rho, &c.inverse_fn(&rho)), &c.one()));
            checked += 1;
            if gi == 0 {
                let (xx, yy) = c.translation(&r);
                let h = c.evaluate_poly_fn(&c.h(), &xx);
                let rhs = c.evaluate_poly_fn(&c.rhs(), &xx);
                let zero = c.sum_fn(&c.sum_fn(&c.mul(&yy, &yy), &c.mul(&h, &yy)), &rhs);
                assert!(c.equal_fn(&zero, &c.constant(&f.z())));
                let mut local = 0;
                for x in 1..4096 {
                    let Some(p) = c.point(&f.e(x)) else { continue };
                    let (Some(a), Some(b)) = (c.eval(&xx, &p), c.eval(&yy, &p)) else {
                        continue;
                    };
                    assert_eq!(P::Affine { x: a, y: b }, c.add(&p, &r));
                    local += 1;
                    if local == 4 {
                        break;
                    }
                }
                assert_eq!(local, 4);
                points += local;
            }
        }
    }
    assert_eq!(checked, 200);
    assert_eq!(points, 400);
    round02::save(
        dir,
        "q6d-parsed-gluing-check.json",
        &json!({"schema":"koblitz.round08/q6d-v1","exact_parsed_transition_identities":checked,"pointwise_translation_checks":points,"exact_curve_translation_identities":100,"inverse_function_identities":checked,"scope":"parsed transition functions checked symbolically against original Miller functions;curve identities and separate point translations;shared field primitives"}),
    );
    println!("Q6d:200 parsed transitions,100 curve identities,400 point translations checked");
}
