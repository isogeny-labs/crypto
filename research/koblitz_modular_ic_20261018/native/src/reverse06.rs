//! A complete pullback fiber as explicit finite-etale point coordinates.
use crate::{large_field::*, round02, round04};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::path::Path;
pub(crate) fn decode(f: &F, v: &Value) -> E {
    E::from_biguint(
        &BigUint::parse_bytes(v.as_str().unwrap().as_bytes(), 16).unwrap(),
        f.m(),
    )
}
pub(crate) fn dp(f: &F, v: &Value) -> P {
    if v.is_array() {
        P::Affine {
            x: decode(f, &v[0]),
            y: decode(f, &v[1]),
        }
    } else {
        P::Infinity
    }
}
pub(crate) fn poly(f: &F, v: &Value) -> Poly {
    v.as_array().unwrap().iter().map(|v| decode(f, v)).collect()
}
pub(crate) fn px(p: &P) -> E {
    match p {
        P::Affine { x, .. } => x.clone(),
        _ => panic!("finite point"),
    }
}
pub(crate) fn image_point(f: &F, c: &crate::binary_ecc::BinaryCurve, g: &P, p: &P) -> P {
    let mut x = px(p);
    let mut y = match p {
        P::Affine { y, .. } => y.clone(),
        _ => panic!(),
    };
    let mut roots = vec![];
    for i in 1..11 {
        let t = times(c, g, i);
        let z = plus(c, p, &t);
        let (P::Affine { x: tx, y: ty }, P::Affine { x: zx, y: zy }) = (t, z) else {
            panic!("Velu exceptional point")
        };
        x = x.add(&zx).add(&tx);
        y = y.add(&zy).add(&ty);
        if i <= 5 {
            roots.push(tx);
        }
    }
    let sum = roots.iter().fold(f.z(), |v, x| v.add(x));
    y = y.add(&sum);
    let q = P::Affine { x, y };
    let dest = f.curve(c.a.clone(), qb(f, &c.b, &roots));
    assert!(dest.is_on_curve(&q));
    q
}
fn miller(f: &F, c: &crate::binary_ecc::BinaryCurve, p: &P, r: &P) -> Option<E> {
    fn line(f: &F, c: &crate::binary_ecc::BinaryCurve, p: &P, q: &P, r: &P) -> Option<E> {
        let (P::Affine { x, y }, P::Affine { x: u, y: v }, P::Affine { x: rx, y: ry }) = (p, q, r)
        else {
            return None;
        };
        if x == u && y != v {
            return Some(rx.add(x)).filter(|v| !v.is_zero());
        }
        let slope = if p == q {
            x.add(&f.mul(y, &f.inv(x)))
        } else {
            f.mul(&y.add(v), &f.inv(&x.add(u)))
        };
        let val = ry.add(&f.mul(&slope, &rx.add(x))).add(y);
        if val.is_zero() {
            return None;
        }
        match plus(c, p, q) {
            P::Infinity => Some(val),
            P::Affine { x: s, .. } => {
                let d = rx.add(&s);
                if d.is_zero() {
                    None
                } else {
                    Some(f.mul(&val, &f.inv(&d)))
                }
            }
        }
    }
    let mut v = p.clone();
    let mut a = f.o();
    for bit in [0, 1, 1] {
        a = f.mul(&f.sq(&a), &line(f, c, &v, &v, r)?);
        v = plus(c, &v, &v);
        if bit == 1 {
            a = f.mul(&a, &line(f, c, &v, p, r)?);
            v = plus(c, &v, p);
        }
    }
    assert_eq!(v, P::Infinity);
    Some(a)
}
pub(crate) fn weil(f: &F, c: &crate::binary_ecc::BinaryCurve, p: &P, q: &P) -> E {
    for x in 1..1024 {
        let Some(s) = f.point(c, &f.e(x)) else {
            continue;
        };
        let ns = neg(&s);
        if let (Some(a), Some(b), Some(d), Some(e)) = (
            miller(f, c, p, &plus(c, q, &s)),
            miller(f, c, p, &s),
            miller(f, c, q, &ns),
            miller(f, c, q, &plus(c, p, &ns)),
        ) {
            let v = f.mul(&f.mul(&a, &d), &f.inv(&f.mul(&b, &e)));
            assert_eq!(f.powu(&v, 11), f.o());
            return v;
        }
    }
    panic!("pairing auxiliary budget")
}
pub fn screen(dir: &Path, negative: bool) {
    let s = round04::Field::find(5);
    let e = round04::Curve { f: s, a: 1, b: 1 };
    let mut target = e
        .points()
        .into_iter()
        .find(|p| *p != round04::Pt::O && e.times(*p, 11) == round04::Pt::O)
        .unwrap();
    if negative {
        target = e.neg(target);
    }
    let js = crate::modular_geometry::triple(s, target).unwrap();
    assert!(js.iter().all(|j| *j != 0));
    let b = s.inv(js[0]);
    let original = round04::Curve { f: s, a: 0, b };
    let t = 33 - original.points().len() as i64;
    let mat = vec![vec![0u64, 1], vec![1, t.rem_euclid(11) as u64]];
    let mut a = vec![vec![1, 0], vec![0, 1]];
    let mut ord = 0;
    loop {
        let mut b = vec![vec![0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    b[i][j] = (b[i][j] + a[i][k] * mat[k][j]) % 11;
                }
            }
        }
        a = b;
        ord += 1;
        if a == vec![vec![1, 0], vec![0, 1]] {
            break;
        }
        assert!(ord < 133);
    }
    let m = 120;
    assert_eq!(m % (5 * ord), 0);
    println!("R3: underlying trace{t}, Frobenius11 order{ord}, full torsion field degree{m}");
    let f = F::find(m);
    let root = f.root(37);
    let emb = |x: u32| f.embed_bits(x as u64, &root);
    let c = f.curve(f.z(), emb(b));
    let mut s0 = BigInt::from(2);
    let mut s1 = BigInt::from(t);
    for _ in 2..=m / 5 {
        let z = &s1 * t - &s0 * 32;
        (s0, s1) = (s1, z);
    }
    let n = ((BigInt::one() << m) + BigInt::one() - s1)
        .to_biguint()
        .unwrap();
    let mut h = n.clone();
    while &h % 11u32 == BigUint::zero() {
        h /= 11u32;
    }
    let mut basis = vec![];
    for x in 1..4096 {
        let Some(p) = f.point(&c, &f.e(x)) else {
            continue;
        };
        let mut p = crate::binary_ecc::curve::scalar_mul(&c, &p, &h);
        if p == P::Infinity {
            continue;
        }
        while times(&c, &p, 11) != P::Infinity {
            p = times(&c, &p, 11);
        }
        if basis.is_empty() {
            basis.push(p);
        } else if !(0..11).any(|i| times(&c, &basis[0], i) == p) {
            basis.push(p);
            break;
        }
    }
    assert_eq!(basis.len(), 2);
    let zeta = (2..4096)
        .map(|i| f.pow(&f.e(i), &(((BigUint::one() << m) - 1u32) / 11u32)))
        .find(|v| *v != f.o())
        .unwrap();
    let val = weil(&f, &c, &basis[0], &basis[1]);
    let power = (1..11).find(|i| f.powu(&zeta, *i) == val).unwrap();
    let inv = (1..11).find(|i| i * power % 11 == 1).unwrap();
    basis[1] = times(&c, &basis[1], inv);
    assert_eq!(weil(&f, &c, &basis[0], &basis[1]), zeta);
    let k1 = crate::modular_geometry::kernels7(s, b, js[1]);
    let k2 = crate::modular_geometry::kernels7(s, s.inv(js[1]), js[2]);
    assert_eq!(k1.len(), 1);
    assert_eq!(k2.len(), 1);
    let first: Poly = k1[0].iter().map(|v| emb(*v)).collect();
    let second: Poly = k2[0].iter().map(|v| emb(*v)).collect();
    let first = f.roots(&first);
    let second = f.roots(&second);
    let mut rows = vec![];
    for i in 0..=11 {
        let g = if i == 11 {
            basis[1].clone()
        } else {
            plus(&c, &basis[0], &times(&c, &basis[1], i))
        };
        let comp = if i == 11 {
            basis[0].clone()
        } else {
            basis[1].clone()
        };
        let k: Vec<_> = (1..=5).map(|j| px(&times(&c, &g, j))).collect();
        let b0 = qb(&f, &c.b, &k);
        let transport7 = velu(&f, &k, &first);
        let transport11 = velu(&f, &first, &k);
        let second_new = velu(&f, &transport11, &second);
        let b70 = qb(&f, &b0, &transport7);
        let b490 = qb(&f, &b70, &second_new);
        let source = reconstruct(&f, &[f.inv(&b0), f.inv(&b70), f.inv(&b490)]);
        let dual = image_point(&f, &c, &g, &comp);
        let source_curve = f.curve(c.a.clone(), b0.clone());
        assert_eq!(times(&source_curve, &dual, 11), P::Infinity);
        let dualroots: Vec<_> = (1..=5)
            .map(|j| px(&times(&source_curve, &dual, j)))
            .collect();
        assert_eq!(qb(&f, &b0, &dualroots), c.b);
        let back7 = velu(&f, &dualroots, &transport7);
        assert_eq!(from_roots(&f, &back7), from_roots(&f, &first));
        let dual7 = velu(&f, &transport7, &dualroots);
        let backsecond = velu(&f, &dual7, &second_new);
        assert_eq!(from_roots(&f, &backsecond), from_roots(&f, &second));
        let psi = division(&f, &b0, 11);
        let known = from_roots(&f, &dualroots);
        let (remaining, remainder) = div(&f, &psi, &known);
        assert!(remainder.is_empty());
        let mut xq = vec![f.z(), f.o()];
        for _ in 0..m {
            xq = div(&f, &square(&f, &xq), &psi).1;
        }
        let rational = gcdp(&f, &psi, &add(&xq, &[f.z(), f.o()]));
        assert!([6, 61].contains(&rational.len()));
        if rational.len() == 6 {
            assert_eq!(rational, known);
        }
        rows.push(json!({"neighbor":i,"source_modular_point":pj(&source),"source_a":ej(&c.a),"source_b":ej(&b0),"source_first7_roots":transport7.iter().map(ej).collect::<Vec<_>>(),"source_second7_roots":second_new.iter().map(ej).collect::<Vec<_>>(),"dual11_generator":pj(&dual),"known11_kernel":polyj(&known),"remaining11_polynomial":polyj(&remaining),"rational11_x_degree":rational.len()-1,"missing_complement_extension_degree":if rational.len()==61{1}else{11},"expected_geometric_fiber_size":55}));
        println!(
            "R3 neighbor{i}: reverse49 transport checked, rational11 x degree{}",
            rational.len() - 1
        );
    }
    round02::save(
        dir,
        if negative {
            "r3-reverse-fiber-screen-negative.json"
        } else {
            "r3-reverse-fiber-screen.json"
        },
        &json!({"schema":"koblitz.round06/r3-v1","target_F32":target.json(),"target_modular_j_triple_F32":js,"field":{"degree":m,"low_terms":f.irr.low_terms},"F32_embedding_root":ej(&root),"underlying_trace_F32":t,"frobenius_order_mod11":ord,"underlying_order_over_field":n.to_string(),"target_curve_b":ej(&c.b),"basis11":basis.iter().map(pj).collect::<Vec<_>>(),"fixed_pairing":ej(&zeta),"neighbors":rows,"neighbor_count":12,"expected_full_fiber_degree":660,"full_fiber_evaluated":false,"scope":"all12 reverse base neighbors, dual kernels and backward49 transport evaluated; next step explicit degree11 etale complement algebras, no reduced Jacobian class"}),
    );
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct A {
    pub(crate) u: Poly,
    pub(crate) v: Poly,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum AP {
    O,
    Pt(A, A),
}
pub(crate) struct R<'a> {
    pub(crate) f: &'a F,
    pub(crate) p: Poly,
    pub(crate) a: E,
    pub(crate) b: E,
    pub(crate) fx: Poly,
}
impl<'a> R<'a> {
    pub(crate) fn red(&self, p: Poly) -> Poly {
        div(self.f, &p, &self.p).1
    }
    pub(crate) fn z(&self) -> A {
        A {
            u: vec![],
            v: vec![],
        }
    }
    pub(crate) fn o(&self) -> A {
        self.c(&self.f.o())
    }
    pub(crate) fn c(&self, e: &E) -> A {
        A {
            u: if e.is_zero() { vec![] } else { vec![e.clone()] },
            v: vec![],
        }
    }
    pub(crate) fn add(&self, a: &A, b: &A) -> A {
        A {
            u: add(&a.u, &b.u),
            v: add(&a.v, &b.v),
        }
    }
    pub(crate) fn mul(&self, a: &A, b: &A) -> A {
        if a.v.is_empty() && b.v.is_empty() {
            return A {
                u: self.red(mul(self.f, &a.u, &b.u)),
                v: vec![],
            };
        }
        let vv = mul(self.f, &a.v, &b.v);
        A {
            u: self.red(add(&mul(self.f, &a.u, &b.u), &mul(self.f, &vv, &self.fx))),
            v: self.red(add(
                &add(&mul(self.f, &a.u, &b.v), &mul(self.f, &a.v, &b.u)),
                &mul(self.f, &vv, &[self.f.z(), self.f.o()]),
            )),
        }
    }
    pub(crate) fn sq(&self, a: &A) -> A {
        if a.v.is_empty() {
            return A {
                u: self.red(square(self.f, &a.u)),
                v: vec![],
            };
        }
        self.mul(a, a)
    }
    pub(crate) fn inv(&self, a: &A) -> A {
        if a.v.is_empty() {
            return A {
                u: invmod(self.f, &a.u, &self.p),
                v: vec![],
            };
        }
        let x = vec![self.f.z(), self.f.o()];
        let norm = self.red(add(
            &add(
                &square(self.f, &a.u),
                &mul(self.f, &mul(self.f, &a.u, &a.v), &x),
            ),
            &mul(self.f, &square(self.f, &a.v), &self.fx),
        ));
        let ni = invmod(self.f, &norm, &self.p);
        A {
            u: self.red(mul(self.f, &add(&a.u, &mul(self.f, &a.v, &x)), &ni)),
            v: self.red(mul(self.f, &a.v, &ni)),
        }
    }
    pub(crate) fn pow(&self, a: &A, n: u64) -> A {
        let mut v = self.o();
        for i in (0..64 - u64::leading_zeros(n)).rev() {
            v = self.sq(&v);
            if n >> i & 1 == 1 {
                v = self.mul(&v, a);
            }
        }
        v
    }
    pub(crate) fn cp(&self, p: &P) -> AP {
        match p {
            P::Infinity => AP::O,
            P::Affine { x, y } => AP::Pt(self.c(x), self.c(y)),
        }
    }
    pub(crate) fn neg(&self, p: &AP) -> AP {
        match p {
            AP::O => AP::O,
            AP::Pt(x, y) => AP::Pt(x.clone(), self.add(x, y)),
        }
    }
    pub(crate) fn plus(&self, p: &AP, q: &AP) -> AP {
        let (AP::Pt(x, y), AP::Pt(u, v)) = (p, q) else {
            return if *p == AP::O { q.clone() } else { p.clone() };
        };
        if x == u && self.add(y, v) == *x {
            return AP::O;
        }
        let l = if p == q {
            self.add(x, &self.mul(y, &self.inv(x)))
        } else {
            self.mul(&self.add(y, v), &self.inv(&self.add(x, u)))
        };
        let nx = self.add(
            &self.add(&self.add(&self.sq(&l), &l), &self.add(x, u)),
            &self.c(&self.a),
        );
        let ny = self.add(&self.add(&self.mul(&l, &self.add(x, &nx)), &nx), y);
        AP::Pt(nx, ny)
    }
    pub(crate) fn times(&self, p: &AP, n: u64) -> AP {
        let mut v = AP::O;
        for i in (0..64 - u64::leading_zeros(n)).rev() {
            v = self.plus(&v, &v);
            if n >> i & 1 == 1 {
                v = self.plus(&v, p);
            }
        }
        v
    }
    pub(crate) fn line(&self, p: &AP, q: &AP, r: &AP) -> A {
        let (AP::Pt(x, y), AP::Pt(u, v), AP::Pt(rx, ry)) = (p, q, r) else {
            panic!("Miller finite evaluation")
        };
        if x == u && y != v {
            return self.add(rx, x);
        }
        let l = if p == q {
            self.add(x, &self.mul(y, &self.inv(x)))
        } else {
            self.mul(&self.add(y, v), &self.inv(&self.add(x, u)))
        };
        let val = self.add(&self.add(ry, &self.mul(&l, &self.add(rx, x))), y);
        match self.plus(p, q) {
            AP::O => val,
            AP::Pt(s, _) => self.mul(&val, &self.inv(&self.add(rx, &s))),
        }
    }
    pub(crate) fn miller(&self, p: &AP, r: &AP) -> A {
        let mut v = p.clone();
        let mut z = self.o();
        for bit in [0, 1, 1] {
            z = self.mul(&self.sq(&z), &self.line(&v, &v, r));
            v = self.plus(&v, &v);
            if bit == 1 {
                z = self.mul(&z, &self.line(&v, p, r));
                v = self.plus(&v, p);
            }
        }
        assert_eq!(v, AP::O);
        z
    }
    pub(crate) fn weil(&self, p: &AP, q: &AP, s: &AP) -> A {
        let ns = self.neg(s);
        let x = self.miller(p, &self.plus(q, s));
        let y = self.miller(q, &ns);
        let z = self.miller(p, s);
        let w = self.miller(q, &self.plus(p, &ns));
        self.mul(&self.mul(&x, &y), &self.inv(&self.mul(&z, &w)))
    }
}
pub fn algebras(dir: &Path, negative: bool) {
    let input = round04::read(&dir.join(if negative {
        "r3-reverse-fiber-screen-negative.json"
    } else {
        "r3-reverse-fiber-screen.json"
    }));
    let m = input["field"]["degree"].as_u64().unwrap() as u32;
    let terms: Vec<u32> = serde_json::from_value(input["field"]["low_terms"].clone()).unwrap();
    let low = terms.iter().fold(0u64, |v, i| v | (1 << i));
    let f = F::new(m, low);
    let zeta = decode(&f, &input["fixed_pairing"]);
    let mut total = 0;
    for row in input["neighbors"].as_array().unwrap() {
        let index = row["neighbor"].as_u64().unwrap();
        let name = format!(
            "r3b-{}fiber-neighbor-{index:02}.json",
            if negative { "negative-" } else { "" }
        );
        let a = decode(&f, &row["source_a"]);
        let b = decode(&f, &row["source_b"]);
        let p = poly(&f, &row["remaining11_polynomial"]);
        let c = f.curve(a.clone(), b.clone());
        let g = dp(&f, &row["dual11_generator"]);
        let r = R {
            f: &f,
            p: p.clone(),
            a: a.clone(),
            b: b.clone(),
            fx: vec![b.clone(), f.z(), a, f.o()],
        };
        let q = AP::Pt(
            A {
                u: vec![f.z(), f.o()],
                v: vec![],
            },
            A {
                u: vec![],
                v: vec![f.o()],
            },
        );
        let gp = r.cp(&g);
        assert_eq!(r.times(&q, 11), AP::O);
        let s = (1..1024)
            .filter_map(|i| f.point(&c, &f.e(i)))
            .find(|p| times(&c, p, 11) != P::Infinity)
            .expect("auxiliary budget");
        println!("R3b neighbor{index}: compute Weil resolvent in degree110 etale algebra");
        let e = r.weil(&gp, &q, &r.cp(&s));
        assert_eq!(r.pow(&e, 11), r.o());
        let sym = r.add(&e, &r.inv(&e));
        assert!(sym.v.is_empty());
        let mut factors = vec![];
        for scale in 1..=5 {
            let inverse = (1..11).find(|i| i * scale % 11 == 1).unwrap();
            let z = f.powu(&zeta, inverse);
            let symz = z.add(&f.inv(&z));
            let factor = gcdp(&f, &p, &add(&sym.u, &[symz]));
            assert_eq!(factor.len(), 12, "paired complement factor degree");
            let yy = div(
                &f,
                &mul(&f, &add(&e.u, &[z.clone()]), &invmod(&f, &e.v, &factor)),
                &factor,
            )
            .1;
            let yy2 = square(&f, &yy);
            let residual = add(&add(&yy2, &mul(&f, &[f.z(), f.o()], &yy)), &r.fx);
            assert!(div(&f, &residual, &factor).1.is_empty());
            let mut x = vec![f.z(), f.o()];
            for _ in 0..m {
                x = div(&f, &square(&f, &x), &factor).1;
            }
            assert_eq!(gcdp(&f, &factor, &add(&x, &[f.z(), f.o()])).len(), 1);
            let mut x = vec![f.z(), f.o()];
            for _ in 0..m * 11 {
                x = div(&f, &square(&f, &x), &factor).1;
            }
            assert_eq!(x, vec![f.z(), f.o()]);
            factors.push(json!({"P_scale":scale,"P":pj(&times(&c,&g,scale)),"Q_x_minpoly":polyj(&factor),"Q_y_polynomial":polyj(&yy),"degree_over_evaluation_field":11,"Weil_pairing":ej(&zeta),"Q_y_orientation_pairing_to_unscaled_G":ej(&z),"irreducibility_checked":true}));
            total += 11;
        }
        let product = factors
            .iter()
            .map(|v| poly(&f, &v["Q_x_minpoly"]))
            .fold(vec![f.o()], |v, g| mul(&f, &v, &g));
        assert_eq!(product, p);
        round02::save(
            dir,
            &name,
            &json!({"schema":"koblitz.round06/r3b-v1","neighbor":index,"source_modular_point":row["source_modular_point"],"source_a":row["source_a"],"source_b":row["source_b"],"auxiliary":pj(&s),"pairing_polynomial_u":polyj(&e.u),"pairing_polynomial_v":polyj(&e.v),"paired_components":factors,"geometric_degree":55,"all_Q_have_order11":true,"scope":"explicit etale point coordinates, not reduced Jacobian representation"}),
        );
        println!("R3b neighbor{index}: five irreducible degree11 paired components certified");
    }
    assert_eq!(total, 660);
    round02::save(
        dir,
        if negative {
            "r3c-complete-pullback-fiber-negative.json"
        } else {
            "r3c-complete-pullback-fiber.json"
        },
        &json!({"schema":"koblitz.round06/r3c-v1","target_F32":input["target_F32"],"evaluation_field":input["field"],"components_over_evaluation_field":60,"component_degree":11,"geometric_fiber_degree":total,"map":"phi_0, the P-line quotient on determinant-normalized full11 level","fiber_evaluated_as_etale_algebras":true,"whole_reverse_20_fibers_evaluated":false,"reduced_Jacobian_class_evaluated":false,"scope":"one complete degree660 ordinary pullback fiber in explicit paired torsion algebras; not all reverse divisors or Jacobian reduction"}),
    );
}
pub fn split_fiber(dir: &Path, negative: bool) {
    let input = round04::read(&dir.join(if negative {
        "r3-reverse-fiber-screen-negative.json"
    } else {
        "r3-reverse-fiber-screen.json"
    }));
    let f = F::new(120, 27);
    assert!(f.irreducible());
    let zeta = decode(&f, &input["fixed_pairing"]);
    let n = BigUint::parse_bytes(
        input["underlying_order_over_field"]
            .as_str()
            .unwrap()
            .as_bytes(),
        10,
    )
    .unwrap();
    let mut cofactor = n;
    while &cofactor % 11u32 == BigUint::zero() {
        cofactor /= 11u32;
    }
    let root = decode(&f, &input["F32_embedding_root"]);
    let target = if input["target_F32"].is_array() {
        P::Affine {
            x: f.embed_bits(input["target_F32"][0].as_u64().unwrap(), &root),
            y: f.embed_bits(input["target_F32"][1].as_u64().unwrap(), &root),
        }
    } else {
        panic!()
    };
    let mut rows = vec![];
    for row in input["neighbors"].as_array().unwrap() {
        assert_eq!(row["rational11_x_degree"], 60, "not split in this field");
        let a = decode(&f, &row["source_a"]);
        let b = decode(&f, &row["source_b"]);
        let c = f.curve(a, b.clone());
        let g = dp(&f, &row["dual11_generator"]);
        let mut q = None;
        for x in 1..4096 {
            let Some(p) = f.point(&c, &f.e(x)) else {
                continue;
            };
            let mut p = crate::binary_ecc::curve::scalar_mul(&c, &p, &cofactor);
            if p == P::Infinity {
                continue;
            }
            while times(&c, &p, 11) != P::Infinity {
                p = times(&c, &p, 11);
            }
            if !(0..11).any(|i| times(&c, &g, i) == p) {
                q = Some(p);
                break;
            }
        }
        let mut q = q.expect("split11 basis budget");
        let val = weil(&f, &c, &g, &q);
        let e = (1..11).find(|e| f.powu(&zeta, *e) == val).unwrap();
        let ie = (1..11).find(|i| i * e % 11 == 1).unwrap();
        q = times(&c, &q, ie);
        assert_eq!(weil(&f, &c, &g, &q), zeta);
        let first = poly(&f, &row["source_first7_roots"]);
        let second = poly(&f, &row["source_second7_roots"]);
        let b7 = qb(&f, &b, &first);
        let mut targets = vec![];
        for j in 0..=11 {
            let z = if j == 11 {
                q.clone()
            } else {
                plus(&c, &g, &times(&c, &q, j))
            };
            let k: Vec<_> = (1..=5).map(|i| px(&times(&c, &z, i))).collect();
            let b0 = qb(&f, &b, &k);
            let t7 = velu(&f, &k, &first);
            let t11 = velu(&f, &first, &k);
            let b70 = qb(&f, &b0, &t7);
            assert_eq!(b70, qb(&f, &b7, &t11));
            let last = velu(&f, &t11, &second);
            let b49 = qb(&f, &b70, &last);
            targets.push(reconstruct(&f, &[f.inv(&b0), f.inv(&b70), f.inv(&b49)]));
        }
        assert_eq!(targets[0], target, "actual phi0 fiber");
        let mut bases = vec![];
        for scale in 1..=5 {
            let inv = (1..11).find(|i| i * scale % 11 == 1).unwrap();
            for r in 0..11 {
                let p = times(&c, &g, scale);
                let qq = plus(&c, &times(&c, &q, inv), &times(&c, &g, r));
                assert_eq!(times(&c, &p, 11), P::Infinity);
                assert_eq!(times(&c, &qq, 11), P::Infinity);
                assert_eq!(weil(&f, &c, &p, &qq), zeta);
                bases.push(json!({"scale":scale,"complement":r,"P":pj(&p),"Q":pj(&qq)}));
            }
        }
        rows.push(json!({"neighbor":row["neighbor"],"source_modular_point":row["source_modular_point"],"source_a":row["source_a"],"source_b":row["source_b"],"paired_basis":[pj(&g),pj(&q)],"hecke_targets":targets.iter().map(pj).collect::<Vec<_>>(),"bases":bases,"geometric_points":55}));
        println!(
            "R3 split neighbor{}:55 paired bases and12 Hecke targets verified",
            row["neighbor"]
        );
    }
    round02::save(
        dir,
        if negative {
            "r3b-complete-split-fiber-negative.json"
        } else {
            "r3b-complete-split-fiber.json"
        },
        &json!({"schema":"koblitz.round06/r3b-split-v1","target_F32":input["target_F32"],"target_large":pj(&target),"field":input["field"],"F32_embedding_root":input["F32_embedding_root"],"fixed_pairing":input["fixed_pairing"],"neighbors":rows,"geometric_fiber_points":660,"all_phi0_images_equal_target":true,"all_Weil_pairings_checked":660,"scope":"complete split phi0 pullback fiber, not a reduced Jacobian class"}),
    );
}

#[derive(Clone)]
struct Seed {
    base: P,
    a: E,
    b: E,
    modulus: Poly,
    px: Poly,
    py: Poly,
    qx: Poly,
    qy: Poly,
}
pub(crate) fn finite_polys(_f: &F, p: &P) -> (Poly, Poly) {
    match p {
        P::Affine { x, y } => (
            if x.is_zero() { vec![] } else { vec![x.clone()] },
            if y.is_zero() { vec![] } else { vec![y.clone()] },
        ),
        _ => panic!("nonzero basis required"),
    }
}
fn seed_point(_r: &R, p: (&Poly, &Poly)) -> AP {
    AP::Pt(
        A {
            u: p.0.clone(),
            v: vec![],
        },
        A {
            u: p.1.clone(),
            v: vec![],
        },
    )
}
pub(crate) fn scalar_coefficients(f: &F, p: &[E], n: u32) -> Poly {
    p.iter().map(|x| f.frob(x, n)).collect()
}
pub(crate) fn basiskey(base: &P, p: &P, q: &P) -> String {
    let a = format!("{}:{}:{}", pj(base), pj(p), pj(q));
    let b = format!("{}:{}:{}", pj(base), pj(&neg(p)), pj(&neg(q)));
    a.min(b)
}
pub fn divisor(dir: &Path) {
    use std::collections::BTreeSet;
    let positive = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let negative = round04::read(&dir.join("r3-reverse-fiber-screen-negative.json"));
    let f = F::new(120, 27);
    let e49 = f.curve(f.o(), f.o());
    let p0 = dp(&f, &positive["target_large"]);
    assert_eq!(times(&e49, &p0, 5), f.frobp(&p0, 1));
    let zeta = decode(&f, &positive["fixed_pairing"]);
    let mut pos = vec![];
    for row in positive["neighbors"].as_array().unwrap() {
        for v in row["bases"].as_array().unwrap() {
            let p = dp(&f, &v["P"]);
            let q = dp(&f, &v["Q"]);
            let (px, py) = finite_polys(&f, &p);
            let (qx, qy) = finite_polys(&f, &q);
            pos.push(Seed {
                base: dp(&f, &row["source_modular_point"]),
                a: decode(&f, &row["source_a"]),
                b: decode(&f, &row["source_b"]),
                modulus: vec![f.z(), f.o()],
                px,
                py,
                qx,
                qy,
            });
        }
    }
    let mut negs = vec![];
    for row in negative["neighbors"].as_array().unwrap() {
        let i = row["neighbor"].as_u64().unwrap();
        let data = round04::read(&dir.join(format!("r3b-negative-fiber-neighbor-{i:02}.json")));
        for component in data["paired_components"].as_array().unwrap() {
            let p = dp(&f, &component["P"]);
            let (px, py) = finite_polys(&f, &p);
            negs.push(Seed {
                base: dp(&f, &row["source_modular_point"]),
                a: decode(&f, &row["source_a"]),
                b: decode(&f, &row["source_b"]),
                modulus: poly(&f, &component["Q_x_minpoly"]),
                px,
                py,
                qx: vec![f.z(), f.o()],
                qy: poly(&f, &component["Q_y_polynomial"]),
            });
        }
    }
    assert_eq!(pos.len(), 660);
    assert_eq!(negs.len(), 60);
    let point_from = |x: &Poly, y: &Poly| P::Affine {
        x: x.first().cloned().unwrap_or_else(|| f.z()),
        y: y.first().cloned().unwrap_or_else(|| f.z()),
    };
    let mut keys = BTreeSet::new();
    for s in &pos {
        keys.insert(basiskey(
            &s.base,
            &point_from(&s.px, &s.py),
            &point_from(&s.qx, &s.qy),
        ));
    }
    assert_eq!(keys.len(), 660);
    for s in &pos {
        let p = f.frobp(&point_from(&s.px, &s.py), 5);
        let q = neg(&f.frobp(&point_from(&s.qx, &s.qy), 5));
        assert!(
            keys.contains(&basiskey(&f.frobp(&s.base, 5), &p, &q)),
            "split seed Frobenius5"
        );
    }
    let mut neg_descent = 0;
    for s in &negs {
        let base = f.frobp(&s.base, 5);
        let p = f.frobp(&point_from(&s.px, &s.py), 5);
        let gm = scalar_coefficients(&f, &s.modulus, 5);
        let y = add(&scalar_coefficients(&f, &s.qy, 5), &[f.z(), f.o()]);
        let found = negs
            .iter()
            .find(|t| {
                t.base == base && t.modulus == gm && {
                    let q = point_from(&t.px, &t.py);
                    q == p || q == neg(&p)
                }
            })
            .expect("etale seed Frobenius5 component");
        let otherp = point_from(&found.px, &found.py);
        let expected = if otherp == p {
            y
        } else {
            add(&y, &[f.z(), f.o()])
        };
        assert_eq!(expected, found.qy, "etale Q orientation");
        neg_descent += 1;
    }
    let mut terms = vec![];
    let mut degrees = [0usize; 2];
    for i in 0..5 {
        for eps in [1i64, -1] {
            let line = (eps * (2i64.pow(i)) as i64).rem_euclid(11) as u64;
            let iline = (1..11).find(|v| v * line % 11 == 1).unwrap();
            for positive_term in [true, false] {
                let (seed, j) = if positive_term {
                    (&negs, i + 1)
                } else {
                    (&pos, i)
                };
                let invtwo = (1..11).find(|v| v * (2u64.pow(j)) % 11 == 1).unwrap();
                let mut components = vec![];
                for (sindex, s) in seed.iter().enumerate() {
                    let a = f.frob(&s.a, j);
                    let b = f.frob(&s.b, j);
                    let r = R {
                        f: &f,
                        p: s.modulus.clone(),
                        a: a.clone(),
                        b: b.clone(),
                        fx: vec![b.clone(), f.z(), a.clone(), f.o()],
                    };
                    let mut p = seed_point(&r, (&s.px, &s.py));
                    let mut q = seed_point(&r, (&s.qx, &s.qy));
                    let frob_ap = |p: AP| match p {
                        AP::O => AP::O,
                        AP::Pt(mut x, mut y) => {
                            for _ in 0..j {
                                x = r.sq(&x);
                                y = r.sq(&y);
                            }
                            AP::Pt(x, y)
                        }
                    };
                    p = frob_ap(p);
                    q = r.times(&frob_ap(q), invtwo);
                    let newp = r.neg(&r.times(&q, line));
                    let newq = r.plus(&r.times(&p, iline), &q);
                    assert_eq!(
                        r.plus(&newp, &r.times(&newq, line)),
                        p,
                        "actual requested eleven line"
                    );
                    assert_eq!(r.times(&newp, 11), AP::O);
                    assert_eq!(r.times(&newq, 11), AP::O);
                    if sindex == 0 {
                        let aux = (1..1024)
                            .filter_map(|x| f.point(&f.curve(a.clone(), b.clone()), &f.e(x)))
                            .find(|p| times(&f.curve(a.clone(), b.clone()), p, 11) != P::Infinity)
                            .unwrap();
                        assert_eq!(r.weil(&newp, &newq, &r.cp(&aux)), r.c(&zeta));
                    }
                    let (AP::Pt(px, py), AP::Pt(qx, qy)) = (newp, newq) else {
                        panic!()
                    };
                    assert!(
                        px.v.is_empty() && py.v.is_empty() && qx.v.is_empty() && qy.v.is_empty()
                    );
                    let degree = s.modulus.len() - 1;
                    degrees[if positive_term { 0 } else { 1 }] += degree;
                    components.push(json!({"seed_component":sindex,"field_modulus_over_F120":polyj(&s.modulus),"degree":degree,"source_modular_point":pj(&f.frobp(&s.base,j)),"source_a":ej(&a),"source_b":ej(&b),"P_x":polyj(&px.u),"P_y":polyj(&py.u),"Q_x":polyj(&qx.u),"Q_y":polyj(&qy.u)}));
                }
                let target = if positive_term {
                    f.frobp(&neg(&p0), j)
                } else {
                    f.frobp(&p0, j)
                };
                let pi = f.frobp(&p0, i);
                assert_eq!(
                    target,
                    if positive_term {
                        times(&e49, &pi, 6)
                    } else {
                        pi
                    }
                );
                terms.push(json!({"i":i,"line":line,"coefficient":if positive_term{1}else{-1},"target":pj(&target),"seed":if positive_term{"negative"}else{"positive"},"seed_frobenius_power":j,"geometric_degree":660,"components":components}));
                println!(
                    "R3D: term{} line{line}, {} components",
                    terms.len(),
                    seed.len()
                );
            }
        }
    }
    assert_eq!(degrees, [6600, 6600]);
    let mut successor = vec![];
    for t in &terms {
        let line = t["line"].as_u64().unwrap() * 2 % 11;
        let p = f.frobp(&dp(&f, &t["target"]), 1);
        let next = terms
            .iter()
            .position(|v| {
                v["line"] == line
                    && v["coefficient"] == t["coefficient"]
                    && dp(&f, &v["target"]) == p
            })
            .expect("F2 divisor-term permutation");
        successor.push(next);
    }
    assert_eq!(successor.iter().copied().collect::<BTreeSet<_>>().len(), 20);
    let gram = round04::read(&dir.join("s1-reverse-gram.json"));
    let ff: Vec<Vec<i64>> =
        serde_json::from_value(gram["signs"][0]["forward_first4_F"].clone()).unwrap();
    let mut forward = vec![P::Infinity; 5];
    for t in &terms {
        let a = t["line"].as_u64().unwrap() as usize;
        let coefficient = t["coefficient"].as_i64().unwrap();
        let target = dp(&f, &t["target"]);
        for i in 0..5 {
            let l = if i < 4 {
                ff[i][a]
            } else {
                -ff.iter().map(|r| r[a]).sum::<i64>()
            };
            let scalar = coefficient * 640 * l;
            let image = times(&e49, &target, scalar.unsigned_abs());
            forward[i] = plus(
                &e49,
                &forward[i],
                &if scalar < 0 { neg(&image) } else { image },
            );
        }
    }
    let original: Vec<_> = (0..5).map(|i| f.frobp(&p0, i)).collect();
    assert_eq!(
        original.iter().fold(P::Infinity, |v, p| plus(&e49, &v, p)),
        P::Infinity
    );
    assert_eq!(forward, original, "Gram-correspondence reverse composition");
    round02::save(
        dir,
        "r3d-even-reverse-divisor.json",
        &json!({"schema":"koblitz.round06/r3d-v1","source_A5_point":original.iter().map(pj).collect::<Vec<_>>(),"source_F32_point":positive["target_F32"],"evaluation_field":positive["field"],"coordinate_algebra_degrees":[1,11],"terms":terms,"positive_degree":6600,"negative_degree":6600,"total_degree":0,"split_seed_Frobenius5_checks":660,"etale_seed_Frobenius5_checks":neg_descent,"term_frobenius_permutation":successor,"F2_divisor_descent_checked":true,"forward_image_via_proven_Gram":forward.iter().map(pj).collect::<Vec<_>>(),"forward_image_equals_source":true,"forward_check_scope":"correspondence-based check with proven Gram and evaluated target points; not independent pointwise norm of all7200 point-algebra entries","all20_fibers_in_explicit_coordinates":true,"Jacobian_reduction_implemented":false,"odd_reverse_divisor_evaluated":false,"scope":"complete even reverse map on one order11 generator as an unreduced F2-rational divisor; finite-etale coordinates and descent checked; no reduced genus2841 divisor arithmetic"}),
    );
}
pub fn verify(dir: &Path) {
    let f = F::new(120, 27);
    let positive = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let negative = round04::read(&dir.join("r3-reverse-fiber-screen-negative.json"));
    let zeta = decode(&f, &positive["fixed_pairing"]);
    let mut split_points = 0;
    let mut algebra_points = 0;
    let mut seeds = vec![vec![], vec![]];
    for row in positive["neighbors"].as_array().unwrap() {
        let c = f.curve(decode(&f, &row["source_a"]), decode(&f, &row["source_b"]));
        for entry in row["bases"].as_array().unwrap() {
            let p = dp(&f, &entry["P"]);
            let q = dp(&f, &entry["Q"]);
            assert!(c.is_on_curve(&p) && c.is_on_curve(&q));
            assert_eq!(times(&c, &p, 11), P::Infinity);
            assert_eq!(times(&c, &q, 11), P::Infinity);
            assert_eq!(weil(&f, &c, &p, &q), zeta);
            seeds[0].push(p);
            split_points += 1;
        }
    }
    for row in negative["neighbors"].as_array().unwrap() {
        let id = row["neighbor"].as_u64().unwrap();
        let data = round04::read(&dir.join(format!("r3b-negative-fiber-neighbor-{id:02}.json")));
        let a = decode(&f, &row["source_a"]);
        let b = decode(&f, &row["source_b"]);
        let aux = dp(&f, &data["auxiliary"]);
        for point in data["paired_components"].as_array().unwrap() {
            let g = poly(&f, &point["Q_x_minpoly"]);
            let y = poly(&f, &point["Q_y_polynomial"]);
            assert_eq!(g.len(), 12);
            let r = R {
                f: &f,
                p: g.clone(),
                a: a.clone(),
                b: b.clone(),
                fx: vec![b.clone(), f.z(), a.clone(), f.o()],
            };
            let q = AP::Pt(
                A {
                    u: vec![f.z(), f.o()],
                    v: vec![],
                },
                A {
                    u: y.clone(),
                    v: vec![],
                },
            );
            let p = dp(&f, &point["P"]);
            assert_eq!(r.times(&q, 11), AP::O);
            assert_eq!(r.weil(&r.cp(&p), &q, &r.cp(&aux)), r.c(&zeta));
            let residual = add(&add(&square(&f, &y), &mul(&f, &[f.z(), f.o()], &y)), &r.fx);
            assert!(div(&f, &residual, &g).1.is_empty());
            seeds[1].push(p);
            algebra_points += 1;
        }
    }
    assert_eq!(split_points, 660);
    assert_eq!(algebra_points, 60);
    let divisor = round04::read(&dir.join("r3d-even-reverse-divisor.json"));
    let mut entries = 0;
    let mut degree = [0usize; 2];
    for term in divisor["terms"].as_array().unwrap() {
        let seed = if term["seed"] == "positive" { 0 } else { 1 };
        let j = term["seed_frobenius_power"].as_u64().unwrap() as u32;
        let line = term["line"].as_u64().unwrap();
        for component in term["components"].as_array().unwrap() {
            let modulus = poly(&f, &component["field_modulus_over_F120"]);
            let deg = modulus.len() - 1;
            assert_eq!(component["degree"], deg);
            assert_eq!(deg, if seed == 0 { 1 } else { 11 });
            let a = decode(&f, &component["source_a"]);
            let b = decode(&f, &component["source_b"]);
            let r = R {
                f: &f,
                p: modulus,
                a: a.clone(),
                b: b.clone(),
                fx: vec![b, f.z(), a, f.o()],
            };
            let p = AP::Pt(
                A {
                    u: poly(&f, &component["P_x"]),
                    v: vec![],
                },
                A {
                    u: poly(&f, &component["P_y"]),
                    v: vec![],
                },
            );
            let q = AP::Pt(
                A {
                    u: poly(&f, &component["Q_x"]),
                    v: vec![],
                },
                A {
                    u: poly(&f, &component["Q_y"]),
                    v: vec![],
                },
            );
            for v in [&p, &q] {
                let AP::Pt(x, y) = v else { panic!() };
                let rhs = r.add(
                    &r.add(&r.mul(&r.sq(x), x), &r.mul(&r.c(&r.a), &r.sq(x))),
                    &r.c(&r.b),
                );
                assert_eq!(r.add(&r.sq(y), &r.mul(x, y)), rhs);
            }
            let index = component["seed_component"].as_u64().unwrap() as usize;
            let expected = f.frobp(&seeds[seed][index], j);
            assert_eq!(r.plus(&p, &r.times(&q, line)), r.cp(&expected));
            degree[if term["coefficient"] == 1 { 0 } else { 1 }] += deg;
            entries += 1;
        }
    }
    assert_eq!(degree, [6600, 6600]);
    assert_eq!(entries, 7200);
    round02::save(
        dir,
        "r3e-coordinate-checks.json",
        &json!({"schema":"koblitz.round06/r3e-v1","split_seed_paired_bases":split_points,"etale_seed_paired_components":algebra_points,"all_reverse_point_algebra_entries":entries,"all_requested_line_checks":entries,"all_curve_equation_checks":2*entries,"positive_degree":degree[0],"negative_degree":degree[1],"status":"PASS","scope":"separate parse-and-evaluate coordinate/algebra checker; shares field/group primitives; does not independently compute pointwise forward norms or Jacobian reduction"}),
    );
    println!("R3e:660 split bases,60 etale components and7200 reverse coordinate entries checked");
}
