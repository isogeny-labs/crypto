#[path = "../multi/algebra.rs"]
mod algebra;
#[path = "../setup/fast.rs"]
mod old;
mod prepare;
use hec_native_ic::{counted, fp::Fp, jac::Curve, poly::Poly};
fn main() {
    println!("g\tp\tsm\trep\tarm\tmodulus\tlambda\tcounts");
    let mut calls = 0;
    for (g, p, cc, ell) in [
        (2, 101, 2, 7481),
        (2, 251, 18, 67931),
        (2, 601, 7, 411491),
        (3, 113, 3, 1247429),
        (3, 421, 2, 77152909),
    ] {
        counted::disable();
        let c = Curve::new(Fp::new(p), {
            let mut f = vec![0; 2 * g + 2];
            f[0] = cc;
            f[2 * g + 1] = 1;
            Poly::from_coeffs(&f)
        });
        let (fb, _, _) = old::prepare(&c);
        let base = c.place(fb[0].0, fb[0].1);
        for sm in [1, 17] {
            let d = c.mul(&base, sm);
            let mut prior = None;
            for rep in 0..2 {
                let mut pair = vec![];
                for arm in 0..2 {
                    counted::reset();
                    assert!(c.is_valid(&d));
                    assert!(!c.is_zero(&d));
                    assert!(c.is_zero(&c.mul(&d, ell)));
                    let (fb, z, l) = if arm == 0 {
                        let (fb, z, a) = old::prepare(&c);
                        let (_, _, degree, l) = algebra::certificate(&a, ell);
                        assert_eq!(degree, 1);
                        (fb, z, l.unwrap())
                    } else {
                        let (fb, z) = prepare::prepare(&c);
                        let f = Fp::new(ell);
                        let n = (2 * g + 1) as u64;
                        assert_eq!((ell - 1) % n, 0);
                        let w = (2..ell)
                            .map(|a| f.pow(a, (ell - 1) / n))
                            .find(|&w| w != 1)
                            .unwrap();
                        assert_eq!(f.pow(w, n), 1);
                        let image = algebra::action(&c, &d, z);
                        let mut r = 1;
                        let mut found = vec![];
                        for _ in 1..n {
                            r = f.mul(r, w);
                            if c.mul(&d, r) == image {
                                found.push(r)
                            }
                        }
                        assert_eq!(found.len(), 1);
                        (fb, z, found[0])
                    };
                    let counts = counted::snapshot();
                    counted::disable();
                    assert_eq!(algebra::action(&c, &d, z), c.mul(&d, l));
                    for (modulus, v) in &counts {
                        assert_eq!(v[8], 2 * v[7] + v[5] + v[6]);
                        println!(
                            "{g}\t{p}\t{sm}\t{rep}\t{arm}\t{modulus}\t{l}\t{}",
                            v.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
                        );
                    }
                    pair.push((fb, z, l, counts));
                    calls += 1;
                }
                assert_eq!(
                    (&pair[0].0, pair[0].1, pair[0].2),
                    (&pair[1].0, pair[1].1, pair[1].2)
                );
                if let Some(old) = &prior {
                    assert_eq!(old, &pair)
                } else {
                    prior = Some(pair)
                }
            }
        }
    }
    assert_eq!(calls, 40);
    eprintln!("calls={calls} same_FB_root_lambda=true original_source_action=true repetitions=true complete_solve=false");
}
