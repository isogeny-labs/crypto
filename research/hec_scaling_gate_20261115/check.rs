use hec_native_ic::{fp::Fp, poly::Poly};
use std::{fs, path::Path};
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn substitute(f: &Poly, a: u64, t: u64, fp: &Fp) -> Poly {
    let linear = Poly::from_coeffs(&[t, a]);
    let mut out = Poly::from_coeffs(&[]);
    for i in (0..f.n).rev() {
        out = out.mul(&linear, fp).add(&Poly::from_coeffs(&[f.c[i]]), fp);
    }
    out
}
fn check(f: &Poly, p: u64, label: &str) -> (u64, u64) {
    let fp = Fp::new(p);
    let d = f.n - 1;
    assert!(d % 2 == 1 && d as u64 % p != 0 && f.c[d] == 1);
    let s = fp.neg(fp.mul(f.c[d - 1], fp.inv(d as u64 % p)));
    let centered = substitute(f, 1, s, &fp);
    assert_eq!(centered.c[d - 1], 0);
    let mut k = 0;
    for i in 0..d {
        if centered.c[i] != 0 {
            k = gcd(k, (d - i) as u64);
        }
    }
    let predicted = gcd(k, (p - 1) / 2);
    let mut admitted = 0;
    for a in 1..p {
        let b2 = fp.pow(a, d as u64);
        let t = fp.mul(s, fp.sub(1, a));
        let direct = substitute(f, a, t, &fp) == f.scale(b2, &fp) && fp.legendre(b2) == 1;
        let criterion = fp.pow(a, k) == 1 && fp.legendre(a) == 1;
        assert_eq!(direct, criterion, "{label} a={a}");
        admitted += direct as u64;
    }
    assert_eq!(admitted, predicted);
    println!("CHECK label={label} p={p} d={d} K={k} predicted={predicted} admitted={admitted} gap2={} gap3={}",centered.c[d-2],centered.c[d-3]);
    (k, admitted)
}
fn main() {
    let mut controls = 0;
    for p in [5, 7, 11, 17] {
        for d in [5, 7, 9] {
            if d as u64 % p == 0 {
                continue;
            }
            for pattern in 0..4 {
                let mut c = vec![0; d + 1];
                c[d] = 1;
                match pattern {
                    0 => c[0] = 1,
                    1 => c[1] = 1,
                    _ => {
                        c[d - 2] = 1;
                        c[d - 3] = 1;
                        c[0] = 1;
                    }
                }
                let f = Poly::from_coeffs(&c);
                let f = if pattern == 3 {
                    substitute(&f, 1, 1, &Fp::new(p))
                } else {
                    f
                };
                check(&f, p, &format!("control-{pattern}"));
                controls += 1;
            }
        }
    }
    let root = std::env::args().nth(1).unwrap();
    let mut fixtures = 0;
    let mut trivial = 0;
    for (g, p) in [
        (2, 1009),
        (2, 16381),
        (3, 127),
        (3, 1009),
        (4, 31),
        (4, 127),
    ] {
        let path = Path::new(&root).join(format!("select-g{g}-p{p}-f0.out"));
        let text = fs::read_to_string(path).unwrap();
        let tokens: Vec<_> = text.split_whitespace().collect();
        assert_eq!(tokens[0], "FIXTURE");
        assert_eq!(tokens[1].parse::<usize>().unwrap(), g);
        assert_eq!(tokens[2].parse::<u64>().unwrap(), p);
        let n: usize = tokens[6].parse().unwrap();
        assert_eq!(n, 2 * g + 2);
        let c: Vec<u64> = tokens[7..7 + n]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let (k, a) = check(&Poly::from_coeffs(&c), p, &format!("fixture-g{g}"));
        fixtures += 1;
        trivial += (k == 1 && a == 1) as usize;
    }
    println!("TERMINAL polynomial_controls={controls} fixtures={fixtures} trivial_affine_fixtures={trivial}");
}
