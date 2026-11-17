use crate::fp::Fp;
use crate::poly::Poly;

fn mul3(a: [u64; 3], b: [u64; 3], ca1: i64, ca0: i64, p: i64) -> [u64; 3] {
    let c0 = (a[0] * b[0]) as i64;
    let c1 = (a[0] * b[1] + a[1] * b[0]) as i64;
    let c2 = (a[0] * b[2] + a[1] * b[1] + a[2] * b[0]) as i64;
    let c3 = (a[1] * b[2] + a[2] * b[1]) as i64;
    let c4 = (a[2] * b[2]) as i64;
    let r0 = c0 - ca0 * c3;
    let r1 = c1 - ca1 * c3 - ca0 * c4;
    let r2 = c2 - ca1 * c4;
    [r0.rem_euclid(p) as u64, r1.rem_euclid(p) as u64, r2.rem_euclid(p) as u64]
}

/// Returns (N1, N2, N3) and (s1, s2, s3).
pub fn chi_enum(fp: &Fp, f: &Poly) -> ([i128; 3], [i128; 3]) {
    let p = fp.p;
    let pu = p as usize;
    let cf: Vec<u64> = (0..7).map(|i| f.get(i)).collect();
    assert!(f.n == 8 && f.get(7) == 1);
    // N1
    let mut n1: i128 = 1;
    for x in 0..p {
        let w = fp.peval(f, x);
        n1 += if w == 0 { 1 } else if fp.legendre(w) == 1 { 2 } else { 0 };
    }
    // N2
    let nu = fp.nu;
    let mut sq2 = vec![false; pu * pu];
    for y0 in 0..p {
        for y1 in 0..p {
            if y0 == 0 && y1 == 0 {
                continue;
            }
            let a = (y0 * y0 + y1 * y1 % p * nu) % p;
            let b = 2 * y0 * y1 % p;
            sq2[(a + b * p) as usize] = true;
        }
    }
    let mut n2: i128 = 1;
    for x1 in 0..p {
        for x0 in 0..p {
            let (mut a0, mut a1) = (1u64, 0u64);
            for i in (0..7).rev() {
                let n0 = (a0 * x0 + a1 * x1 % p * nu) % p;
                let nn1 = (a0 * x1 + a1 * x0) % p;
                a0 = (n0 + cf[i]) % p;
                a1 = nn1;
            }
            if a0 == 0 && a1 == 0 {
                n2 += 1;
            } else if sq2[(a0 + a1 * p) as usize] {
                n2 += 2;
            }
        }
    }
    // F_{p^3} = F_p[t]/(t^3 + ca1 t + ca0), first (a1 outer, a0 inner) without a root
    let (mut ca1, mut ca0) = (0u64, 0u64);
    'find: for a1 in 0..p {
        for a0 in 1..p {
            let mut root = false;
            for t in 0..p {
                if (t * t % p * t + a1 * t + a0) % p == 0 {
                    root = true;
                    break;
                }
            }
            if !root {
                ca1 = a1;
                ca0 = a0;
                break 'find;
            }
        }
    }
    assert!(ca0 != 0);
    let (i1, i0, ip) = (ca1 as i64, ca0 as i64, p as i64);
    let p3 = pu * pu * pu;
    let mut sq3 = vec![false; p3];
    for y2 in 0..p {
        for y1 in 0..p {
            for y0 in 0..p {
                if y0 == 0 && y1 == 0 && y2 == 0 {
                    continue;
                }
                let s = mul3([y0, y1, y2], [y0, y1, y2], i1, i0, ip);
                sq3[(s[0] + s[1] * p + s[2] * p * p) as usize] = true;
            }
        }
    }
    let mut n3: i128 = 1;
    for x2 in 0..p {
        for x1 in 0..p {
            for x0 in 0..p {
                let x = [x0, x1, x2];
                let mut acc = [1u64, 0, 0];
                for i in (0..7).rev() {
                    acc = mul3(acc, x, i1, i0, ip);
                    acc[0] = (acc[0] + cf[i]) % p;
                }
                if acc == [0, 0, 0] {
                    n3 += 1;
                } else if sq3[(acc[0] + acc[1] * p + acc[2] * p * p) as usize] {
                    n3 += 2;
                }
            }
        }
    }
    let pp = p as i128;
    let sk1 = pp + 1 - n1;
    let sk2 = pp * pp + 1 - n2;
    let sk3 = pp * pp * pp + 1 - n3;
    let s1 = sk1;
    let t2 = s1 * sk1 - sk2;
    assert!(t2 % 2 == 0);
    let s2 = t2 / 2;
    let t3 = s2 * sk1 - s1 * sk2 + sk3;
    assert!(t3 % 3 == 0);
    let s3 = t3 / 3;
    ([n1, n2, n3], [s1, s2, s3])
}
