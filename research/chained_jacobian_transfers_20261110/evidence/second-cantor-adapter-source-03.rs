//! Reuse the pre-existing generic odd-model Cantor engine after an explicit
//! sextic-to-quintic chart. No producer tower,balanced addition or map import.
use super::parse_polynomial;
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpr::{FpR, ER};
use isogeny_algos::poly;

#[path = "legacy_jacobian.rs"]
mod legacy;

struct Flat<'a>(&'a FpR);
impl legacy::Fld for Flat<'_> {
    type E = ER;
    fn zero(&self) -> ER {
        self.0.zero()
    }
    fn one(&self) -> ER {
        self.0.one()
    }
    fn add(&self, a: &ER, b: &ER) -> ER {
        self.0.add(*a, *b)
    }
    fn sub(&self, a: &ER, b: &ER) -> ER {
        self.0.sub(*a, *b)
    }
    fn neg(&self, a: &ER) -> ER {
        self.0.neg(*a)
    }
    fn mul(&self, a: &ER, b: &ER) -> ER {
        self.0.mul(*a, *b)
    }
    fn inv(&self, a: &ER) -> ER {
        assert_ne!(*a, self.0.zero());
        self.0.inv(*a)
    }
}

fn replay_group(witness: &str, coefficients: [u64; 7], prefixed: bool) {
    let f = FpR::from_modulus(101, &[33, 46, 2, 18, 56, 12, 1]);
    let lift = |v: &[u64]| v.iter().map(|c| f.from_u64(*c)).collect::<Vec<_>>();
    let gamma = f.gen();
    let s = f.div(
        poly::eval(&f, &lift(&[68, 19, 6, 1]), gamma),
        poly::eval(&f, &lift(&[15, 12, 3]), gamma),
    );
    let t = f.sub(gamma, s);
    assert_eq!(f.sq(s), f.from_u64(2));
    assert_eq!(poly::eval(&f, &lift(&[56, 13, 6, 1]), t), f.zero());
    let basis = [f.one(), t, f.sq(t), s, f.mul(s, t), f.mul(s, f.sq(t))];
    let curve = lift(&coefficients);
    let mut classes = vec![];
    for (i, line) in witness.lines().skip(1).enumerate() {
        let columns = line.split('\t').collect::<Vec<_>>();
        let offset = usize::from(prefixed);
        assert_eq!(columns.len(), 4 + offset);
        if prefixed {
            assert_eq!(columns[0], "actualPsiKernel");
        }
        assert_eq!(columns[offset].parse::<usize>().unwrap(), i);
        let u = parse_polynomial(&f, columns[1 + offset], &basis);
        let v = parse_polynomial(&f, columns[2 + offset], &basis);
        let plus = columns[3 + offset].parse::<usize>().unwrap();
        if i == 0 {
            assert_eq!((&u, &v, plus), (&vec![f.one()], &vec![], 1));
        } else {
            assert_eq!((u.len(), plus), (3, 0));
        }
        assert!(poly::rem(&f, &poly::sub(&f, &curve, &poly::mul(&f, &v, &v)), &u).is_empty());
        classes.push((u, v));
    }
    assert_eq!(classes.len(), 49);
    let mut roots = poly::roots(&f, &curve, &mut Rng::new(0x0026_c017));
    roots.sort();
    assert_eq!(roots.len(), 6);
    let a = *roots
        .iter()
        .find(|a| {
            classes
                .iter()
                .all(|(u, _)| poly::eval(&f, u, **a) != f.zero())
        })
        .expect("one actual branch chart must avoid all49 finite supports");
    let l = poly::eval(&f, &poly::derivative(&f, &curve), a);
    assert_ne!(l, f.zero());
    // H(z)=z^6 F(a+1/z),then G(w)=L^4 H(w/L) is MONIC of degree5.
    let mut h = vec![];
    let az1 = vec![f.one(), a];
    let mut power = vec![f.one()];
    for (i, c) in curve.iter().enumerate() {
        let mut term = vec![f.zero(); 6 - i];
        term.extend(poly::scale(&f, &power, *c));
        h = poly::add(&f, &h, &term);
        power = poly::mul(&f, &power, &az1);
    }
    assert_eq!(h.len(), 6);
    assert_eq!(h[5], l);
    let quintic = h
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let scale = if i <= 4 {
                f.pow(l, (4 - i) as u128)
            } else {
                f.inv(l)
            };
            f.mul(*c, scale)
        })
        .collect::<Vec<_>>();
    assert_eq!(quintic.last(), Some(&f.one()));
    assert_eq!(
        poly::gcd(&f, &quintic, &poly::derivative(&f, &quintic)),
        vec![f.one()]
    );
    let field = Flat(&f);
    let j = legacy::Hyp {
        f: &field,
        h: quintic.clone(),
        g: 2,
    };
    assert!(j.is_identity(&j.identity()));
    for k in 1..=7 {
        let w = f.from_u64(k);
        let x = f.add(a, f.div(l, w));
        let rhs = f.div(f.mul(f.pow(w, 6), poly::eval(&f, &curve, x)), f.sq(l));
        assert_eq!(legacy::peval(&field, &quintic, &w), rhs);
    }
    let mut transformed = vec![j.identity()];
    for (u, v) in classes.iter().skip(1) {
        let ua = poly::eval(&f, u, a);
        assert_ne!(ua, f.zero());
        let new_u = vec![
            f.div(f.sq(l), ua),
            f.div(f.mul(l, f.add(f.mul(f.from_u64(2), a), u[1])), ua),
            f.one(),
        ];
        let v1 = v.get(1).copied().unwrap_or(f.zero());
        let new_v = poly::rem(
            &f,
            &vec![f.zero(), f.zero(), v1, f.div(poly::eval(&f, v, a), l)],
            &new_u,
        );
        let d = legacy::Div { u: new_u, v: new_v };
        assert!(j.is_valid(&d));
        assert!(!transformed.contains(&d));
        assert_eq!(j.mul(&d, 7), j.identity());
        assert_eq!(j.add(&d, &j.neg(&d)), j.identity());
        assert_eq!(j.sub(&d, &d), j.identity());
        transformed.push(d);
    }
    assert_eq!(transformed.len(), 49);
    for i in 0..49 {
        for k in 0..49 {
            let sum = ((i / 7 + k / 7) % 7) * 7 + (i % 7 + k % 7) % 7;
            assert_eq!(
                j.add(&transformed[i], &transformed[k]),
                transformed[sum],
                "second Cantor group curve{coefficients:?} row{i}+{k}"
            );
        }
    }
    println!("PASS second EXISTING generic Cantor engine on curve{coefficients:?}:49 explicit sextic classes mapped to monic quintic,all2401 actual additions/all7-annihilators/inverses/distinctness;checked branch chart a={a:?},L={l:?}. No producer tower/balanced addition/map imported. Global chart/Picard bridge remains written;independent map-kernel membership is not asserted.");
}

#[test]
fn existing_cantor_replays_both_actual_kernel_groups() {
    replay_group(
        include_str!("../fixtures/explicit-psi-kernel-07.tsv"),
        [31, 75, 74, 54, 71, 55, 1],
        true,
    );
    replay_group(
        include_str!("../fixtures/explicit-psi2-kernel-12.tsv"),
        [95, 26, 19, 27, 94, 91, 1],
        false,
    );
}

#[test]
fn existing_cantor_source_extraction_is_exact() {
    let original = include_str!("../../../../src/cryptanalysis/jv_cover.rs");
    let lines = original.lines().collect::<Vec<_>>();
    let expected = format!(
        "// Frozen existing generic Cantor engine;see legacy.sha256.\n{}\n{}\n}}\n",
        lines[39..52].join("\n"),
        lines[463..690].join("\n")
    );
    let styled = expected
        .replace("pub type Poly<E> = Vec<E>;\n", "pub type Poly<E> = Vec<E>;\npub(crate) type Bezout<E> = (Poly<E>, Poly<E>, Poly<E>);\n")
        .replace("-> (Poly<F::E>, Poly<F::E>, Poly<F::E>) {", "-> Bezout<F::E> {");
    assert_eq!(include_str!("legacy_jacobian.rs"), styled);
}
