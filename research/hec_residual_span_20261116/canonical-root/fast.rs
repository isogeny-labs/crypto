use super::root;
use hec_native_ic::{instance::prime_factors, jac::Curve};
pub fn prepare(c: &Curve) -> (Vec<(u64, u64)>, u64, Vec<i64>) {
    let f = &c.fp;
    let n = 2 * c.g + 1;
    assert_eq!(c.f.deg(), n as isize);
    assert_eq!(c.f.lc(), 1);
    assert!(c.f.c[1..n].iter().all(|&v| v == 0));
    assert_ne!(c.f.c[0], 0);
    assert_eq!((f.p - 1) % n as u64, 0);
    // Enumerate canonical roots by (y+1)^2=y^2+2y+1.
    let mut roots = vec![None; f.p as usize];
    let (mut square, mut delta) = (0, 1);
    for y in 0..=(f.p - 1) / 2 {
        assert!(roots[square as usize].is_none());
        roots[square as usize] = Some(y);
        square = f.add(square, delta);
        delta = f.add(delta, 2 % f.p);
    }
    // Forward differences enumerate f(x) using additions after n+1 evaluations.
    let mut values: Vec<_> = (0..=n).map(|i| c.f.eval(i as u64, f)).collect();
    let mut differences = vec![];
    while !values.is_empty() {
        differences.push(values[0]);
        values = values.windows(2).map(|w| f.sub(w[1], w[0])).collect()
    }
    let mut fb = vec![];
    for x in 0..f.p {
        if let Some(y) = roots[differences[0] as usize] {
            fb.push((x, y))
        }
        for j in 0..n {
            differences[j] = f.add(differences[j], differences[j + 1]);
        }
    }
    let factors = prime_factors(f.p - 1);
    let generator = (2..f.p)
        .find(|&v| factors.iter().all(|&q| f.pow(v, (f.p - 1) / q) != 1))
        .unwrap();
    let w = f.pow(generator, (f.p - 1) / n as u64);
    let z = root::from_primitive(f,n as u64,w);
    let mut powers = vec![];
    let mut v = 1;
    for _ in 0..n {
        powers.push(v);
        v = f.mul(v, z)
    }
    assert_eq!(v, 1);
    let stride = powers.iter().position(|&v| v == w).unwrap();
    let (mut t, mut j) = (1, 0usize);
    let mut a = vec![0i64; n];
    for _ in 0..f.p - 1 {
        let b = f.add(t, c.f.c[0]);
        let eta = if b == 0 {
            0
        } else if roots[b as usize].is_some() {
            1
        } else {
            -1
        };
        a[j] -= eta;
        t = f.mul(t, generator);
        j = (j + stride) % n;
    }
    assert_eq!(t, 1);
    assert_eq!(j, 0);
    (fb, z, a)
}
