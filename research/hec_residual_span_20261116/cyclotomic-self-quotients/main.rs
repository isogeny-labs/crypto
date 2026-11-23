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
fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = vec![];
    let mut q = 2;
    while q <= n / q {
        let mut e = 0;
        while n % q == 0 {
            n /= q;
            e += 1;
        }
        if e > 0 {
            out.push((q, e))
        }
        q = if q == 2 { 3 } else { q + 2 };
    }
    if n > 1 {
        out.push((n, 1))
    }
    out
}
fn cmul(a: [i128; 3], b: [i128; 3]) -> [i128; 3] {
    let mut v = [0; 5];
    for i in 0..3 {
        for j in 0..3 {
            v[i + j] += a[i] * b[j]
        }
    }
    for k in (3..5).rev() {
        let t = v[k];
        v[k] = 0;
        v[k - 1] -= t;
        v[k - 2] += 2 * t;
        v[k - 3] += t;
    }
    [v[0], v[1], v[2]]
}
fn cadd(a: [i128; 3], b: [i128; 3]) -> [i128; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn cp(a: [i128; 3], n: usize) -> [i128; 3] {
    (0..n).fold([1, 0, 0], |v, _| cmul(v, a))
}
fn norm(a: [i128; 3]) -> i128 {
    let mut m = vec![vec![0; 3]; 3];
    for j in 0..3 {
        let mut b = [0; 3];
        b[j] = 1;
        let v = cmul(a, b);
        for i in 0..3 {
            m[i][j] = v[i];
        }
    }
    det(m)
}
fn eval_num(a: i128, b: i128) -> i128 {
    a * a * a + a * a * b - 2 * a * b * b - b * b * b
}
fn arithmetic() {
    assert!(130i128 * 130 > 7i128.pow(5));
    assert!(720i128 * 64 * 130 < 5i128 * 46656 * 27); // pi>3 => Minkowski bound <5.
    assert_eq!(fp(2, 3, 7), 1);
    assert_ne!(fp(2, 1, 7), 1);
    assert_eq!(fp(3, 6, 7), 1);
    for d in [1, 2, 3] {
        assert_ne!(fp(3, d, 7), 1)
    }
    let t = [0, 1, 0];
    let s = [-2, 0, 1];
    assert_eq!(norm(t), 1);
    assert_eq!(norm(s), 1);
    assert_eq!(
        cadd(
            cadd(cp(s, 3), cp(s, 2)),
            cadd(cmul([-2, 0, 0], s), [-1, 0, 0])
        ),
        [0, 0, 0]
    );
    let ss = cadd(cp(s, 2), [-2, 0, 0]);
    let sss = cadd(cp(ss, 2), [-2, 0, 0]);
    assert_eq!(sss, t);
    // Three disjoint rational intervals isolate all roots of t^3+t^2-2t-1.
    assert!(eval_num(6, 5) < 0 && eval_num(5, 4) > 0);
    assert!(eval_num(-1, 2) > 0 && eval_num(-2, 5) < 0);
    assert!(eval_num(-2, 1) < 0 && eval_num(-3, 2) > 0);
    // t signs +--, s=t²-2 signs --+, and -1 signs ---.
    assert!(25 < 32);
    assert!(1 < 8);
    assert!(9 > 8);
    let signature = vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 1]];
    assert_eq!(rank(&signature, 2), 3);
    println!("ARITHMETIC\tminkowski_bound_lt_5=true\tord7_2=3\tord7_3=6\treal_unit_norms=1,1\tsignature_rank=3\trational_root_isolation=true");
}
fn main() {
    counted::disable();
    arithmetic();
    let mut models = 0;
    for (p, c, expected) in [
        (113u64, 3u64, 575889585954787i128),
        (421, 2, 469521548813867549),
    ] {
        let curve = Curve::new(Fp::new(p), Poly::from_coeffs(&[c, 0, 0, 0, 0, 0, 0, 1]));
        let (_, _, a) = fast::prepare(&curve);
        let a: Vec<_> = a.iter().map(|&x| x as i128).collect();
        let m = matrix(&a);
        let powers = krylov(&m);
        let index = det(powers.clone()).checked_abs().unwrap();
        assert_eq!(index, expected);
        let fs = factor(u64::try_from(index).unwrap());
        let product = fs.iter().fold(1u128, |v, &(q, e)| v * (q as u128).pow(e));
        assert_eq!(product, index as u128);
        let f = characteristic(&m);
        assert_ne!(f[3] % (p as i128), 0);
        println!("MODEL\tp={p}\tc={c}\tindex={index}\tfactors={fs:?}\tcharacteristic={f:?}");
        for (l, e) in fs {
            let r = rank(&powers, l);
            assert!(r < 6);
            if l == p || l == 7 {
                println!("EXCEPTION\tp={p}\tl={l}\tvaluation={e}\trank={r}\texcluded_characteristic_or_ramified=true");
                continue;
            }
            let polynomial: Vec<_> = f.iter().rev().map(|&x| red(x, l)).collect();
            let degrees = factors(&polynomial, l);
            let gcd = degrees.iter().fold(0, |v, &d| ugcd(v, d));
            let obstructed = 3 % gcd != 0;
            println!("EXCEPTION\tp={p}\tl={l}\tvaluation={e}\trank={r}\tdistinct_degrees={degrees:?}\tdimension_obstructed={obstructed}");
        }
        models += 1;
    }
    assert_eq!(models, 2);
    eprintln!("PASS models=2 exact_index_factorization=true field_arithmetic_support_checks=true no_kernel_enumeration_or_quotient_construction=true theorem_independent_review_pending=true speed_claim=false");
}
