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
fn main() {
    counted::disable();
    let f = Fp::new(659);
    let eq: Vec<Vec<_>> = [(410, 389), (307, 144)]
        .iter()
        .map(|&(a, b)| (0..6).map(|j| f.sub(f.pow(a, j), f.pow(b, j))).collect())
        .collect();
    assert_eq!(rref(eq.clone(), &f).1.len(), 2);
    let basis = preimage(eq, &f);
    assert_eq!(bdet(&basis), 659i128.pow(2));
    for &a in &basis {
        assert!(in_order(a, &f));
        assert!(in_order(bar(a), &f));
        for &b in &basis {
            assert!(in_order(km(a, b), &f));
        }
    }
    let gram: Mat = basis
        .iter()
        .map(|&a| {
            basis
                .iter()
                .map(|&b| {
                    let v = km(a, b);
                    6 * v[0] - v[1..].iter().sum::<i128>()
                })
                .collect()
        })
        .collect();
    let disc = det(gram);
    assert_eq!(disc, -7i128.pow(5) * 659i128.pow(4));
    let ceq: Vec<Vec<_>> = [410, 389, 307, 144]
        .iter()
        .map(|&a| (0..6).map(|j| f.pow(a, j)).collect())
        .collect();
    assert_eq!(rref(ceq.clone(), &f).1.len(), 4);
    let cbasis = preimage(ceq, &f);
    assert_eq!(bdet(&cbasis), 659i128.pow(4));
    for &a in &cbasis {
        for j in 0..6 {
            let mut v = [0; 6];
            v[j] = 1;
            let p = km(a, v);
            for r in [410, 389, 307, 144] {
                assert_eq!(evaluate(p, r, &f), 0)
            }
        }
    }
    let mut count = 0;
    for gamma in [55, 12] {
        let sigma = [410, 389, 307, 144, gamma, f.inv(gamma)];
        for t in 1..659 {
            let h = [
                [1, t, 0, 0, 0, 0],
                [0, 0, f.neg(t), 1, 0, 0],
                [0, 0, 0, 0, 1, 0],
            ];
            let mut constraints = vec![];
            for a in &h {
                for b in &h {
                    constraints.push(
                        (0..6)
                            .map(|k| {
                                let u = std::array::from_fn(|i| f.mul(a[i], f.pow(sigma[i], k)));
                                pair(&u, b, &f)
                            })
                            .collect(),
                    );
                }
            }
            assert_eq!(rref(constraints.clone(), &f).1.len(), 2);
            for &v in &basis {
                for row in &constraints {
                    let value = (0..6).fold(0, |s, j| {
                        f.add(s, f.mul(row[j], v[j].rem_euclid(659) as u64))
                    });
                    assert_eq!(value, 0);
                }
            }
            count += 1;
        }
    }
    assert_eq!(count, 1316);
    println!("ORDER\tindex={}\tdiscriminant={disc}\tbasis={basis:?}\tconductor_norm={}\tconductor_basis={cbasis:?}\tkernel_stabilizers_checked={count}",659u64.pow(2),659u64.pow(4));
    let rho = f.mul(389, f.inv(410));
    assert_eq!(f.pow(rho, 7), 1);
    assert_ne!(rho, 1);
    let mut classes = std::collections::BTreeSet::new();
    let mut seen = std::collections::BTreeSet::new();
    for t in 1..659 {
        if seen.contains(&t) {
            continue;
        }
        let mut orb = vec![];
        let mut x = t;
        for _ in 0..7 {
            assert!(seen.insert(x));
            orb.push(x);
            x = f.mul(x, rho);
        }
        assert_eq!(x, t);
        classes.insert(*orb.iter().min().unwrap());
    }
    assert_eq!(classes.len(), 94);
    assert_eq!(seen.len(), 658);
    println!("AUTOMORPHISM\tslope_multiplier={rho}\torbits_per_family=94\torbit_size=7");
    let mut found = None;
    let mut checked = 0;
    for code in 0..9u64.pow(6) {
        let mut code = code;
        let a: K = std::array::from_fn(|_| {
            let v = (code % 9) as i128 - 4;
            code /= 9;
            v
        });
        if evaluate(a, 55, &f) != 0 {
            continue;
        }
        checked += 1;
        if norm(a) == 659 {
            found = Some(a);
            break;
        }
    }
    println!(
        "SEARCH\tnorm_candidates_checked={checked}\tfound={}",
        found.is_some()
    );
    let Some(alpha) = found else {
        eprintln!(
            "INCOMPLETE bounded_generator_search_no_hit=true no_cross_family_count_claim=true"
        );
        std::process::exit(2)
    };
    assert_eq!(norm(bar(alpha)), 659);
    let mut va = bar(alpha).to_vec();
    va.push(0);
    let mat = matrix(&va);
    let mut b = [0; 6];
    for j in 0..6 {
        let mut a = mat.clone();
        for i in 0..6 {
            a[i][j] = if i == 0 { 659 } else { 0 };
        }
        let n = det(a);
        assert_eq!(n % 659, 0);
        b[j] = n / 659;
    }
    let mut rhs = [0; 6];
    rhs[0] = 659;
    assert_eq!(km(bar(alpha), b), rhs);
    let w = km(alpha, b);
    let mut ww = [0; 6];
    ww[0] = 659 * 659;
    assert_eq!(km(w, bar(w)), ww);
    let zeros: Vec<_> = [410, 389, 307, 144, 55, 12]
        .into_iter()
        .filter(|&r| evaluate(alpha, r, &f) == 0)
        .collect();
    assert_eq!(zeros, vec![55]);
    let ua = f.mul(
        evaluate(alpha, 410, &f),
        f.inv(evaluate(bar(alpha), 410, &f)),
    );
    let ub = f.mul(
        evaluate(alpha, 389, &f),
        f.inv(evaluate(bar(alpha), 389, &f)),
    );
    let c = f.mul(ub, f.inv(ua));
    let canonical = |mut x: u64| {
        let mut min = x;
        for _ in 0..7 {
            min = min.min(x);
            x = f.mul(x, rho);
        }
        min
    };
    let images: std::collections::BTreeSet<_> =
        classes.iter().map(|&x| canonical(f.mul(c, x))).collect();
    assert_eq!(images, classes);
    println!("BRIDGE\talpha={alpha:?}\tnorm=659\tzero_sigma=55\tu_numerator={w:?}\tu_denominator=659\tu_conjugate_norm=1\tslope_multiplier={c}\tclass_bijection=94");
    eprintln!("PASS kernel_stabilizers=1316 exact_order_basis_and_discriminant=true conductor_ideal=true source_aut_orbits=188 cross_family_class_bijection=94 abstract_lattice_interpretation_review_pending=true no_curve_or_isogeny_constructed=true speed_claim=false");
}
