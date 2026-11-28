use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
};
const NONE: usize = usize::MAX;
// Public metadata contract: exact N and prime ell are supplied, not searched.
pub fn prepare(
    c: &Curve,
    d: &Div,
    ell: u64,
    n: u64,
) -> (Vec<(u64, u64)>, Vec<(usize, u64)>, Vec<usize>) {
    assert_eq!(c.g, 4);
    assert_eq!(n % ell, 0);
    let h = n / ell;
    assert_eq!(h, 2);
    assert!(c.is_valid(d));
    assert!(!c.is_zero(d));
    assert!(c.is_zero(&c.mul(d, ell)));
    let i = c.fp.sqrt(c.fp.p - 1).unwrap();
    let f = Fp::new(ell);
    let root = f.sqrt(ell - 1).unwrap();
    // Direct u(-x),i*v(-x) with monic correction for general reduced degree.
    let mut image = *d;
    for j in 0..image.u.n {
        if (d.u.deg() as usize + j) % 2 != 0 {
            image.u.c[j] = c.fp.neg(image.u.c[j])
        }
    }
    for j in 0..image.v.n {
        image.v.c[j] =
            c.fp.mul(image.v.c[j], if j % 2 == 0 { i } else { c.fp.neg(i) })
    }
    assert!(c.is_valid(&image));
    let roots: Vec<_> = [root, ell - root]
        .into_iter()
        .filter(|&l| c.mul(d, l) == image)
        .collect();
    assert_eq!(roots.len(), 1);
    let lambda = roots[0];
    let fb = factor_base(c);
    let mut map = vec![(NONE, 0); fb.len()];
    let mut reps = vec![];
    for j in 0..fb.len() {
        if fb[j].1 == 0 {
            assert!(c.is_zero(&c.mul(&c.place(fb[j].0, 0), h)));
            continue;
        }
        if map[j].0 != NONE {
            continue;
        }
        let r = reps.len();
        reps.push(j);
        map[j] = (r, 1);
        let x = c.fp.neg(fb[j].0);
        let y = c.fp.mul(i, fb[j].1);
        let k = fb.iter().position(|v| v.0 == x).unwrap();
        assert_ne!(j, k);
        assert_eq!(map[k].0, NONE);
        let w = if fb[k].1 == y {
            lambda
        } else {
            assert_eq!(fb[k].1, c.fp.neg(y));
            f.neg(lambda)
        };
        map[k] = (r, w);
        // N=2ell and nonzero ell-order source imply cyclic ell-torsion; the checked eigenvalue applies to all projected points.
    }
    assert!(fb
        .iter()
        .enumerate()
        .all(|(j, v)| v.1 == 0 || map[j].0 != NONE));
    (fb, map, reps)
}
