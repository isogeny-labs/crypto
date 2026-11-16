use crate::cantor::Curve;
use crate::crit::{crit, recip};
use crate::enumchi::chi_enum;
use crate::fp::{is_prime_u64, Fp};
use crate::gsp::{charpoly, sample_sp};
use crate::lagr2;
use crate::order::{chi_search, chi_vec, count_n1, curve_f};
use crate::poly::Poly;
use crate::rng::{curve_seed, SplitMix64};

#[test]
fn t01_splitmix() {
    let mut r = SplitMix64::new(0);
    assert_eq!(r.next(), 0xE220A8397B1DCDAF);
    assert_eq!(r.next(), 0x6E789E6AA1B965F4);
}

#[test]
fn t02_x7_plus_1_over_f5() {
    let fp = Fp::new(5);
    let f = Poly::from(&[1, 0, 0, 0, 0, 0, 0, 1], 5);
    assert!(fp.is_squarefree(&f));
    assert_eq!(count_n1(&fp, &f), 6);
    let mut d = fp.ddf_degrees(&f);
    d.sort_unstable_by(|a, b| b.cmp(a));
    assert_eq!(d, vec![6, 1]);
}

#[test]
fn t03_fp_basics() {
    let f7 = Fp::new(7);
    assert_eq!(f7.legendre(3), -1);
    let s = f7.sqrt(2);
    assert!(s == 3 || s == 4);
    assert_eq!(Fp::new(5).nu, 2);
    assert_eq!(Fp::new(7).nu, 3);
    for p in [5u64, 7, 11, 101, 257] {
        let fp = Fp::new(p);
        for a in 0..p {
            assert_eq!(fp.legendre(a), fp.legendre_pow(a));
        }
    }
}

#[test]
fn t04_miller_rabin() {
    assert!(is_prime_u64((1u64 << 31) - 1));
    assert!(!is_prime_u64(3215031751));
    assert!(!is_prime_u64(1));
    assert!(is_prime_u64(2));
}

#[test]
fn t05_cantor() {
    let fp = Fp::new(5);
    let f = Poly::from(&[1, 0, 0, 0, 0, 0, 0, 1], 5);
    let c = Curve::new(&fp, f);
    let a = c.point_divisor(0, 1);
    let b = c.point_divisor(0, 4);
    assert!(c.is_id(&c.add(&a, &b)));
    let t = crate::cantor::Div { u: Poly::from(&[1, 1], 5), v: Poly::zero() };
    assert!(c.is_id(&c.dbl(&t)));
    let mut rng = SplitMix64::new(77);
    for _ in 0..20 {
        let d = c.random_class(&mut rng);
        assert!(c.is_id(&c.add(&d, &c.neg(&d))));
    }
    let fp = Fp::new(101);
    let mut i = 0;
    let f = loop {
        let f = curve_f(101, i);
        if fp.is_squarefree(&f) {
            break f;
        }
        i += 1;
    };
    let c = Curve::new(&fp, f);
    let mut rng = SplitMix64::new(78);
    for _ in 0..20 {
        let (a, b, d) = (c.random_class(&mut rng), c.random_class(&mut rng), c.random_class(&mut rng));
        assert_eq!(c.add(&c.add(&a, &b), &d), c.add(&a, &c.add(&b, &d)));
        let x = (rng.uniform(1000)) as i64;
        let y = (rng.uniform(1000)) as i64;
        assert_eq!(c.mul(x + y, &a), c.add(&c.mul(x, &a), &c.mul(y, &a)));
    }
}

fn first_sqfree(p: u64, from: u64, fp: &Fp) -> (u64, Poly) {
    let mut i = from;
    loop {
        let f = curve_f(p, i);
        if fp.is_squarefree(&f) {
            return (i, f);
        }
        i += 1;
    }
}

#[test]
fn t06_enum_vs_cantor() {
    for &p in &[11u64, 13] {
        let fp = Fp::new(p);
        let mut i = 0;
        let mut done = 0;
        let pi = p as i128;
        while done < 20 {
            let (j, f) = first_sqfree(p, i, &fp);
            i = j + 1;
            if crate::order::count_n1(&fp, &f) < 4 {
                continue;
            }
            done += 1;
            let (_n, s) = chi_enum(&fp, &f);
            let chi = chi_vec(pi, s[0], s[1], s[2]);
            let c1: i128 = chi.iter().sum();
            let cm1: i128 = chi.iter().enumerate().map(|(k, &a)| if k % 2 == 0 { a } else { -a }).sum();
            let cj = Curve::new(&fp, f);
            let ft = fp.pscale(&f, fp.nu);
            let ct = Curve::new(&fp, ft);
            let mut rng = SplitMix64::new(1000 + i);
            for _ in 0..5 {
                let d = cj.random_class(&mut rng);
                assert!(cj.is_id(&cj.mul(c1 as i64, &d)), "chi(1) fails p={} i={}", p, j);
                let d = ct.random_class(&mut rng);
                assert!(ct.is_id(&ct.mul(cm1 as i64, &d)), "chi(-1) fails p={} i={}", p, j);
            }
        }
    }
    let fp = Fp::new(101);
    let mut i = 0;
    let (mut agree, mut amb) = (0, 0);
    for _ in 0..5 {
        let (j, f) = first_sqfree(101, i, &fp);
        i = j + 1;
        let (_n, s) = chi_enum(&fp, &f);
        let r = chi_search(&fp, &f, curve_seed(101, j));
        if r.status == "ok" {
            assert_eq!([r.s1, r.s2, r.s3], s);
            agree += 1;
        } else {
            amb += 1;
        }
    }
    assert!(agree > 0, "all ambiguous ({})", amb);
}

#[test]
fn t07_chi_mod2() {
    let fp = Fp::new(5);
    let f = Poly::from(&[1, 0, 0, 0, 0, 0, 0, 1], 5);
    let (_n, s) = chi_enum(&fp, &f);
    let chi = chi_vec(5, s[0], s[1], s[2]);
    let m2: Vec<u64> = chi.iter().map(|&a| a.rem_euclid(2) as u64).collect();
    let want = lagr2::chi2_from_cycle_type(&[6, 1, 1]);
    assert_eq!(want.to_vec(), vec![1, 0, 0, 0, 0, 0, 1]);
    assert_eq!(m2, want.to_vec());
}

fn prod_poly(fl: &Fp, factors: &[Vec<u64>]) -> Poly {
    let mut r = Poly::one();
    for f in factors {
        r = fl.pmul(&r, &Poly::from(f, fl.p));
    }
    r
}
fn lin(a: u64, l: u64) -> Vec<u64> {
    vec![(l - a) % l, 1]
}

#[test]
fn t08_criterion() {
    // l = 5, q = 1
    let f5 = Fp::new(5);
    let c = prod_poly(&f5, &[lin(2, 5), lin(2, 5), lin(2, 5), lin(3, 5), lin(3, 5), lin(3, 5)]);
    let r = crit(&f5, &c, 1);
    assert!(r.fire && r.n_h == 2 && r.class == "fire" && r.g4());
    let c = prod_poly(&f5, &[lin(2, 5), lin(2, 5), lin(3, 5), lin(3, 5), lin(1, 5), lin(1, 5)]);
    let r = crit(&f5, &c, 1);
    assert!(!r.fire && r.class == "undetermined" && r.g4());
    // l = 5, q = 2
    let c = prod_poly(&f5, &[lin(1, 5), lin(1, 5), lin(2, 5), lin(2, 5), lin(3, 5), lin(4, 5)]);
    let r = crit(&f5, &c, 2);
    assert!(r.fire && r.n_h == 4 && r.g4());
    // l = 2, q = 1
    let f2 = Fp::new(2);
    let c = Poly::from(&[1, 1, 1, 1, 1, 1, 1], 2);
    let r = crit(&f2, &c, 1);
    assert!(r.fire && r.n_h == 2 && r.ext_deg == Some(7) && r.g4());
    // l = 3
    let f3 = Fp::new(3);
    let c = prod_poly(&f3, &vec![lin(1, 3); 6]);
    let r = crit(&f3, &c, 1);
    assert!(!r.fire && r.class == "undetermined");
    let c = prod_poly(&f3, &[lin(1, 3), lin(1, 3), lin(1, 3), lin(2, 3), lin(2, 3), lin(2, 3)]);
    let r = crit(&f3, &c, 2);
    assert!(r.fire && r.n_h == 2 && r.ext_deg == Some(3), "{:?}", r);
    // l = 7
    let f7 = Fp::new(7);
    let c = prod_poly(&f7, &[vec![1, 0, 1], lin(2, 7), lin(4, 7), lin(3, 7), lin(5, 7)]);
    let r = crit(&f7, &c, 1);
    assert!(!r.fire && r.class == "none" && r.g4());
    let c = prod_poly(&f7, &(1..=6).map(|a| lin(a, 7)).collect::<Vec<_>>());
    let r = crit(&f7, &c, 3);
    assert!(r.fire && r.n_h == 8 && r.class == "fire" && r.g4());
    // h^*
    let h = prod_poly(&f5, &[lin(1, 5), lin(1, 5), lin(3, 5)]);
    let hs = recip(&f5, &h, 2);
    let want = prod_poly(&f5, &[lin(2, 5), lin(2, 5), lin(4, 5)]);
    assert_eq!(hs, want);
}

#[test]
fn t09_charpoly() {
    let f7 = Fp::new(7);
    let mut id = vec![];
    for j in 0..6 {
        let mut v = [0u64; 6];
        v[j] = 1;
        id.push(v);
    }
    let c = charpoly(&f7, &id, 6);
    assert_eq!(c, prod_poly(&f7, &vec![lin(1, 7); 6]));
    // companion matrix of the l=7,q=3 polynomial
    let target = prod_poly(&f7, &(1..=6).map(|a| lin(a, 7)).collect::<Vec<_>>());
    // columns: companion e_j -> e_{j+1}, last column = -coefficients
    let mut cols = vec![];
    for j in 0..6 {
        let mut v = [0u64; 6];
        if j < 5 {
            v[j + 1] = 1;
        } else {
            for r in 0..6 {
                v[r] = (7 - target.get(r)) % 7;
            }
        }
        cols.push(v);
    }
    assert_eq!(charpoly(&f7, &cols, 6), target);
    let mut rng = SplitMix64::new(5);
    for _ in 0..10 {
        let (a, b, c, d) = (rng.uniform(7), rng.uniform(7), rng.uniform(7), rng.uniform(7));
        // matrix [[a,b],[c,d]]: columns (a,c),(b,d)
        let mut c0 = [0u64; 6];
        c0[0] = a;
        c0[1] = c;
        let mut c1 = [0u64; 6];
        c1[0] = b;
        c1[1] = d;
        let cp = charpoly(&f7, &[c0, c1], 2);
        let want = Poly::from(&[(a * d + 7 * 7 - b * c) % 7, (14 - a - d) % 7, 1], 7);
        assert_eq!(cp, want);
    }
}

#[test]
fn t10_sampler() {
    let f5 = Fp::new(5);
    let mut rng = SplitMix64::new(4242);
    let m = 2u64;
    for _ in 0..1000 {
        let mut cols = sample_sp(&mut rng, 3, 5, &f5);
        for k in 3..6 {
            for t in 0..6 {
                cols[k][t] = cols[k][t] * m % 5;
            }
        }
        for i in 0..6 {
            for j in 0..6 {
                let e = if i < 3 && j == i + 3 {
                    m
                } else if j < 3 && i == j + 3 {
                    5 - m
                } else {
                    0
                };
                assert_eq!(crate::gsp::om(3, 5, &cols[i], &cols[j]), e);
            }
        }
        let c = charpoly(&f5, &cols, 6);
        assert_eq!(c.get(0), 3);
    }
}

#[test]
fn t11_lagrangians() {
    let lag = lagr2::build();
    assert_eq!((lag.total, lag.pair, lag.other), (135, 105, 30));
    let r = lag.rows.iter().find(|r| r.cycle_type == vec![7, 1]).unwrap();
    assert_eq!((r.stable_pair, r.stable_other, r.pair_rule, r.fire_2), (0, 2, false, true));
    let r = lag.rows.iter().find(|r| r.cycle_type == vec![1; 8]).unwrap();
    assert_eq!((r.stable_pair, r.stable_other), (105, 30));
    assert_eq!(lag.rows.len(), 15);
}
