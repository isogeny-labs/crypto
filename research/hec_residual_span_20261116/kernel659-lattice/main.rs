#[path = "../canonical-root/fast.rs"]
mod fast;
#[allow(dead_code)]
#[path = "../canonical-root/root.rs"]
mod root;
use hec_native_ic::{counted, fp::Fp, jac::Curve, poly::Poly};
type Mat = Vec<Vec<i128>>;
fn add(a: i128, b: i128) -> i128 {
    a.checked_add(b).unwrap()
}
fn mul(a: i128, b: i128) -> i128 {
    a.checked_mul(b).unwrap()
}
fn identity(n: usize) -> Mat {
    (0..n)
        .map(|i| (0..n).map(|j| (i == j) as i128).collect())
        .collect()
}
fn mm(a: &Mat, b: &Mat) -> Mat {
    let n = a.len();
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| (0..n).fold(0, |s, k| add(s, mul(a[i][k], b[k][j]))))
                .collect()
        })
        .collect()
}
fn matrix(a: &[i128]) -> Mat {
    let d = a.len() - 1;
    let mut m = vec![vec![0; d]; d];
    for col in 0..d {
        let mut v = vec![0; 2 * d];
        for (j, &x) in a.iter().enumerate() {
            v[j + col] = add(v[j + col], x);
        }
        for k in (d..2 * d).rev() {
            let x = v[k];
            v[k] = 0;
            for j in 0..d {
                v[k - d + j] = add(v[k - d + j], x.checked_neg().unwrap());
            }
        }
        for i in 0..d {
            m[i][col] = v[i];
        }
    }
    m
}
fn characteristic(m: &Mat) -> Vec<i128> {
    let n = m.len();
    let mut b = identity(n);
    let mut c = vec![1];
    for k in 1..=n {
        b = mm(m, &b);
        let trace = (0..n).fold(0, |s, i| add(s, b[i][i]));
        assert_eq!(trace % k as i128, 0);
        let a = (trace / (k as i128)).checked_neg().unwrap();
        c.push(a);
        for i in 0..n {
            b[i][i] = add(b[i][i], a);
        }
    }
    assert!(b.iter().flatten().all(|x| *x == 0));
    c
}
fn det(mut a: Mat) -> i128 {
    let n = a.len();
    let (mut denominator, mut sign) = (1, 1);
    for k in 0..n - 1 {
        let Some(pivot) = (k..n).find(|&i| a[i][k] != 0) else {
            return 0;
        };
        if pivot != k {
            a.swap(k, pivot);
            sign = -sign;
        }
        let p = a[k][k];
        for i in k + 1..n {
            for j in k + 1..n {
                let v = mul(a[i][j], p).checked_sub(mul(a[i][k], a[k][j])).unwrap();
                assert_eq!(v % denominator, 0);
                a[i][j] = v / denominator;
            }
            a[i][k] = 0;
        }
        denominator = p;
    }
    mul(sign, a[n - 1][n - 1])
}
fn krylov(m: &Mat) -> Mat {
    let n = m.len();
    let mut k = vec![vec![0; n]; n];
    let mut v = vec![0; n];
    v[0] = 1;
    for j in 0..n {
        for i in 0..n {
            k[i][j] = v[i];
        }
        v = (0..n)
            .map(|i| (0..n).fold(0, |s, t| add(s, mul(m[i][t], v[t]))))
            .collect();
    }
    k
}
fn red(x: i128, p: u64) -> u64 {
    x.rem_euclid(p as i128) as u64
}
fn fm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn fp(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = fm(r, a, p);
        }
        a = fm(a, a, p);
        n >>= 1;
    }
    r
}
fn rank(m: &Mat, p: u64) -> usize {
    let n = m.len();
    let mut a: Vec<Vec<_>> = m
        .iter()
        .map(|r| r.iter().map(|&x| red(x, p)).collect())
        .collect();
    let mut r = 0;
    for j in 0..n {
        let Some(i) = (r..n).find(|&i| a[i][j] != 0) else {
            continue;
        };
        a.swap(r, i);
        let iv = fp(a[r][j], p - 2, p);
        for k in j..n {
            a[r][k] = fm(a[r][k], iv, p);
        }
        for i in r + 1..n {
            let q = a[i][j];
            for k in j..n {
                a[i][k] = (a[i][k] + p - fm(q, a[r][k], p)) % p;
            }
        }
        r += 1;
        if r == n {
            break;
        }
    }
    r
}
fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn sub(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    trim(
        (0..a.len().max(b.len()))
            .map(|j| (a.get(j).copied().unwrap_or(0) + p - b.get(j).copied().unwrap_or(0)) % p)
            .collect(),
    )
}
fn rem(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    assert!(!b.is_empty());
    let mut a = trim(a.to_vec());
    let iv = fp(*b.last().unwrap(), p - 2, p);
    while a.len() >= b.len() {
        let k = a.len() - b.len();
        let q = fm(*a.last().unwrap(), iv, p);
        for j in 0..b.len() {
            a[k + j] = (a[k + j] + p - fm(q, b[j], p)) % p;
        }
        a = trim(a);
    }
    a
}
fn gcd(mut a: Vec<u64>, mut b: Vec<u64>, p: u64) -> Vec<u64> {
    while !b.is_empty() {
        let v = rem(&a, &b, p);
        a = b;
        b = v;
    }
    let iv = fp(*a.last().unwrap(), p - 2, p);
    a.iter().map(|&x| fm(x, iv, p)).collect()
}
fn pm(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = (c[i + j] + fm(x, y, p)) % p;
        }
    }
    rem(&c, m, p)
}
fn pp(a: &[u64], mut n: u64, m: &[u64], p: u64) -> Vec<u64> {
    let mut r = vec![1];
    let mut a = a.to_vec();
    while n > 0 {
        if n & 1 == 1 {
            r = pm(&r, &a, m, p);
        }
        a = pm(&a, &a, m, p);
        n >>= 1;
    }
    r
}
fn ugcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
fn factors(f: &[u64], p: u64) -> Vec<usize> {
    let d = f.len() - 1;
    let mut counts = vec![0; d + 1];
    let x = vec![0, 1];
    let mut v = x.clone();
    for k in 1..=d {
        v = pp(&v, p, f, p);
        let n = gcd(f.to_vec(), sub(&v, &x, p), p).len() - 1;
        let old = (1..k)
            .filter(|&j| k % j == 0)
            .map(|j| j * counts[j])
            .sum::<usize>();
        assert!(n >= old && (n - old) % k == 0);
        counts[k] = (n - old) / k;
    }
    (1..=d)
        .flat_map(|k| std::iter::repeat_n(k, counts[k]))
        .collect()
}
type K = [i128; 6];
fn km(a: K, b: K) -> K {
    let mut aa = a.to_vec();
    aa.push(0);
    let m = matrix(&aa);
    std::array::from_fn(|i| (0..6).map(|j| mul(m[i][j], b[j])).sum())
}
fn bar(a: K) -> K {
    let mut v = [0; 7];
    for j in 0..6 {
        v[(7 - j) % 7] += a[j];
    }
    std::array::from_fn(|i| v[i] - v[6])
}
fn norm(a: K) -> i128 {
    let mut a = a.to_vec();
    a.push(0);
    det(matrix(&a))
}
fn evaluate(a: K, r: u64, f: &Fp) -> u64 {
    a.iter().rev().fold(0, |v, &a| {
        f.add(f.mul(v, r), a.rem_euclid(f.p as i128) as u64)
    })
}
fn rref(mut a: Vec<Vec<u64>>, f: &Fp) -> (Vec<Vec<u64>>, Vec<usize>) {
    let mut piv = vec![];
    for j in 0..6 {
        let Some(i) = (piv.len()..a.len()).find(|&i| a[i][j] != 0) else {
            continue;
        };
        let r = piv.len();
        a.swap(i, r);
        let inv = f.inv(a[r][j]);
        for k in j..6 {
            a[r][k] = f.mul(a[r][k], inv);
        }
        for i in 0..a.len() {
            if i == r {
                continue;
            }
            let c = a[i][j];
            for k in j..6 {
                a[i][k] = f.sub(a[i][k], f.mul(c, a[r][k]));
            }
        }
        piv.push(j);
        if piv.len() == a.len() {
            break;
        }
    }
    (a, piv)
}
fn preimage(a: Vec<Vec<u64>>, f: &Fp) -> Vec<K> {
    let (a, piv) = rref(a, f);
    (0..6)
        .map(|j| {
            let mut v = [0; 6];
            if piv.contains(&j) {
                v[j] = f.p as i128
            } else {
                v[j] = 1;
                for (i, &p) in piv.iter().enumerate() {
                    v[p] = -(a[i][j] as i128)
                }
            }
            v
        })
        .collect()
}
fn bdet(b: &[K]) -> i128 {
    det((0..6).map(|i| (0..6).map(|j| b[j][i]).collect()).collect()).abs()
}
fn in_order(a: K, f: &Fp) -> bool {
    evaluate(a, 410, f) == evaluate(a, 389, f) && evaluate(a, 307, f) == evaluate(a, 144, f)
}
fn pair(a: &[u64; 6], b: &[u64; 6], f: &Fp) -> u64 {
    [(0, 2), (1, 3), (4, 5)].into_iter().fold(0, |s, (i, j)| {
        f.add(s, f.sub(f.mul(a[i], b[j]), f.mul(a[j], b[i])))
    })
}
fn transpose(a: &Mat) -> Mat {
    (0..a[0].len())
        .map(|j| (0..a.len()).map(|i| a[i][j]).collect())
        .collect()
}
fn columns(b: &[K]) -> Mat {
    (0..6).map(|i| (0..6).map(|j| b[j][i]).collect()).collect()
}
fn op(a: K) -> Mat {
    let mut a = a.to_vec();
    a.push(0);
    matrix(&a)
}
fn transformed(m: &Mat, src: &Mat, dst: &Mat, den: i128) -> Option<Mat> {
    let target = mm(m, src);
    let divisor = mul(det(dst.clone()), den);
    assert_ne!(divisor, 0);
    let mut out = vec![vec![0; 6]; 6];
    for j in 0..6 {
        for i in 0..6 {
            let mut a = dst.clone();
            for k in 0..6 {
                a[k][i] = target[k][j];
            }
            let value = det(a);
            if value % divisor != 0 {
                return None;
            }
            out[i][j] = value / divisor;
        }
    }
    Some(out)
}
fn symplectic_conjugate(b: &Mat, g: &Mat) -> Mat {
    mm(&mm(&transpose(b), g), b)
}
fn reduce7(a: &[i128; 7]) -> K {
    std::array::from_fn(|i| a[i] - a[6])
}
fn source_form(xi: K) -> Mat {
    (0..6)
        .map(|i| {
            (0..6)
                .map(|j| {
                    let mut a = [0; 6];
                    a[i] = 1;
                    let mut b = [0; 6];
                    b[j] = 1;
                    let v = km(xi, km(a, bar(b)));
                    let trace = 6 * v[0] - v[1..].iter().sum::<i128>();
                    assert_eq!(trace % 7, 0);
                    trace / 7
                })
                .collect()
        })
        .collect()
}
fn crt(r: u64, f: &Fp) -> [u64; 6] {
    let mut p = Poly::one();
    let mut den = 1;
    for s in [410, 389, 307, 144, 55, 12] {
        if s == r {
            continue;
        }
        p = p.mul(&Poly::x_minus(s, f), f);
        den = f.mul(den, f.sub(r, s));
    }
    let p = p.scale(f.inv(den), f);
    let v = std::array::from_fn(|i| p.c[i]);
    for s in [410, 389, 307, 144, 55, 12] {
        assert_eq!(p.eval(s, f), u64::from(s == r));
    }
    v
}
fn modular_pair(a: &[u64; 6], b: &[u64; 6], g: &Mat, f: &Fp) -> u64 {
    (0..6).fold(0, |s, i| {
        (0..6).fold(s, |s, j| {
            f.add(s, f.mul(f.mul(a[i], b[j]), g[i][j].rem_euclid(659) as u64))
        })
    })
}
fn lift_basis(h: Vec<Vec<u64>>, f: &Fp) -> Mat {
    let (h, piv) = rref(h, f);
    assert_eq!(piv.len(), 3);
    let b: Vec<K> = (0..6)
        .map(|j| {
            if let Some(i) = piv.iter().position(|&p| p == j) {
                std::array::from_fn(|k| h[i][k] as i128)
            } else {
                let mut v = [0; 6];
                v[j] = 659;
                v
            }
        })
        .collect();
    columns(&b)
}
fn main() {
    counted::disable();
    let f = Fp::new(659);
    let cv = Curve::new(Fp::new(421), Poly::from_coeffs(&[2, 0, 0, 0, 0, 0, 0, 1]));
    let (_, zp, a) = fast::prepare(&cv);
    let mut raw = [0; 7];
    for i in 0..7 {
        raw[i] = a[i] as i128;
    }
    let pi = reduce7(&raw);
    let pimat = op(pi);
    let vmat = op(bar(pi));
    assert_eq!(
        mm(&pimat, &vmat),
        (0..6)
            .map(|i| (0..6).map(|j| if i == j { 421 } else { 0 }).collect())
            .collect::<Mat>()
    );
    let mut phi = vec![];
    for e in 1..7 {
        let r = cv.fp.pow(zp, e);
        let value = a.iter().rev().fold(0, |v, &a| {
            cv.fp.add(cv.fp.mul(v, r), a.rem_euclid(421) as u64)
        });
        if value == 0 {
            phi.push(e)
        }
    }
    assert_eq!(phi.len(), 3);
    for &e in &phi {
        assert!(!phi.contains(&(7 - e)))
    }
    let positive = (1..7)
        .find(|&j| phi.iter().all(|&e| (1..=3).contains(&((j * e) % 7))))
        .expect("primitive CM type/sign compatibility");
    let mut x = [0; 7];
    x[positive as usize] = 1;
    x[(7 - positive) as usize] = -1;
    let xi = reduce7(&x);
    assert_eq!(bar(xi), xi.map(|v| -v));
    let g0 = source_form(xi);
    assert_eq!(det(g0.clone()), 1);
    assert_eq!(
        transpose(&g0),
        g0.iter()
            .map(|r| r.iter().map(|v| -v).collect())
            .collect::<Mat>()
    );
    assert_eq!(
        symplectic_conjugate(&pimat, &g0),
        g0.iter()
            .map(|r| r.iter().map(|v| 421 * v).collect())
            .collect::<Mat>()
    );
    println!("SOURCE\tp=421\trotation={zp}\tcm_type={phi:?}\txi_exponent={positive}\txi_numerator={xi:?}\txi_denominator=7\tE={g0:?}\tF={pimat:?}");
    let ea = crt(410, &f);
    let eb = crt(389, &f);
    let mut fa = crt(307, &f);
    let mut fb = crt(144, &f);
    let ga = modular_pair(&ea, &fa, &g0, &f);
    let gb = modular_pair(&eb, &fb, &g0, &f);
    assert_ne!(ga, 0);
    assert_ne!(gb, 0);
    for v in &mut fa {
        *v = f.mul(*v, f.inv(ga))
    }
    for v in &mut fb {
        *v = f.mul(*v, f.inv(gb))
    }
    assert_eq!(modular_pair(&ea, &fa, &g0, &f), 1);
    assert_eq!(modular_pair(&eb, &fb, &g0, &f), 1);
    let mut objects = vec![];
    for (gamma, t) in [(55, 1), (12, 555)] {
        let eg = crt(gamma, &f);
        let h = vec![
            (0..6)
                .map(|i| f.add(ea[i], f.mul(t, eb[i])))
                .collect::<Vec<_>>(),
            (0..6)
                .map(|i| f.add(f.mul(f.neg(t), fa[i]), fb[i]))
                .collect(),
            eg.to_vec(),
        ];
        assert_eq!(rref(h.clone(), &f).1.len(), 3);
        for a in &h {
            for b in &h {
                let aa: [u64; 6] = a.clone().try_into().unwrap();
                let bb: [u64; 6] = b.clone().try_into().unwrap();
                assert_eq!(modular_pair(&aa, &bb, &g0, &f), 0);
            }
        }
        let b = lift_basis(h, &f);
        assert_eq!(det(b.clone()).abs(), 659i128.pow(3));
        let mut g = symplectic_conjugate(&b, &g0);
        for row in &mut g {
            for x in row {
                assert_eq!(*x % 659, 0);
                *x /= 659
            }
        }
        assert_eq!(det(g.clone()), 1);
        assert_eq!(
            transpose(&g),
            g.iter()
                .map(|r| r.iter().map(|v| -v).collect())
                .collect::<Mat>()
        );
        let ff = transformed(&pimat, &b, &b, 1).unwrap();
        let vv = transformed(&vmat, &b, &b, 1).unwrap();
        assert_eq!(mm(&ff, &vv), mm(&pimat, &vmat));
        assert_eq!(characteristic(&ff), characteristic(&pimat));
        assert_eq!(
            symplectic_conjugate(&ff, &g),
            g.iter()
                .map(|r| r.iter().map(|v| 421 * v).collect())
                .collect::<Mat>()
        );
        let mut sig = [0; 6];
        sig[1] = 1;
        assert!(transformed(&op(sig), &b, &b, 1).is_none());
        let ellid: Mat = (0..6)
            .map(|i| (0..6).map(|j| if i == j { 659 } else { 0 }).collect())
            .collect();
        let q = transformed(&ellid, &identity(6), &b, 1).unwrap();
        assert_eq!(det(q.clone()).abs(), 659i128.pow(3));
        assert_eq!(mm(&q, &pimat), mm(&ff, &q));
        assert_eq!(
            symplectic_conjugate(&q, &g),
            g0.iter()
                .map(|r| r.iter().map(|v| 659 * v).collect())
                .collect::<Mat>()
        );
        println!("QUOTIENT\tgamma_sigma={gamma}\tslope={t}\tbasis_numerator={b:?}\tbasis_denominator=659\tE={g:?}\tF={ff:?}\tV={vv:?}\tquotient_matrix={q:?}");
        objects.push((b, g, ff));
    }
    let w = [738, 470, 228, 98, 197, 370];
    let u = transformed(&op(w), &objects[0].0, &objects[1].0, 659).unwrap();
    assert_eq!(det(u.clone()).abs(), 1);
    assert_eq!(symplectic_conjugate(&u, &objects[1].1), objects[0].1);
    assert_eq!(mm(&u, &objects[0].2), mm(&objects[1].2, &u));
    println!("ISOMETRY\tmatrix={u:?}\tdeterminant={}", det(u.clone()));
    eprintln!("PASS source_integral_principal_form=true p_adic_CM_type_checked=true quotient_lattices=2 integral_F_V=true source_quotient_isogeny_matrices=true exact_polarized_Frobenius_isometry=true geometric_curve_and_map_not_constructed=true independent_review_pending=true speed_claim=false");
}
