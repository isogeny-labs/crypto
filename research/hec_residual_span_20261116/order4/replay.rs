use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
type Mat = Vec<Vec<u64>>;
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = r * a % p
        }
        a = a * a % p;
        n >>= 1
    }
    r
}
fn id(n: usize) -> Mat {
    let mut m = vec![vec![0; n]; n];
    for i in 0..n {
        m[i][i] = 1
    }
    m
}
fn mul(a: &Mat, b: &Mat, p: u64) -> Mat {
    let n = a.len();
    let mut m = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                m[i][j] = (m[i][j] + a[i][k] * b[k][j]) % p
            }
        }
    }
    m
}
fn det(mut m: Mat, p: u64) -> u64 {
    let mut value = 1;
    for j in 0..m.len() {
        let Some(k) = (j..m.len()).find(|&k| m[k][j] != 0) else {
            return 0;
        };
        if k != j {
            m.swap(k, j);
            value = (p - value) % p
        }
        let lead = m[j][j];
        value = value * lead % p;
        let inverse = pow(lead, p - 2, p);
        for k in j + 1..m.len() {
            let a = m[k][j] * inverse % p;
            for t in j..m.len() {
                m[k][t] = (m[k][t] + p - a * m[j][t] % p) % p
            }
        }
    }
    value
}
fn times_linear(a: &[u64], root: u64, p: u64) -> Vec<u64> {
    let mut b = vec![0; a.len() + 1];
    for i in 0..a.len() {
        b[i] = (b[i] + p - a[i] * root % p) % p;
        b[i + 1] = (b[i + 1] + a[i]) % p
    }
    b
}
fn characteristic(f: &[i128], mut n: u64, p: u64) -> Vec<u64> {
    let d = f.len() - 1;
    let mut a = vec![vec![0; d]; d];
    for j in 0..d - 1 {
        a[j + 1][j] = 1
    }
    for j in 0..d {
        a[j][d - 1] = (-f[j]).rem_euclid(p as i128) as u64
    }
    let mut b = id(d);
    while n > 0 {
        if n & 1 != 0 {
            b = mul(&b, &a, p)
        }
        a = mul(&a, &a, p);
        n >>= 1
    }
    let mut evaluations = vec![];
    for t in 0..=d {
        let mut m = vec![vec![0; d]; d];
        for i in 0..d {
            for j in 0..d {
                m[i][j] = if i == j {
                    (t as u64 + p - b[i][j]) % p
                } else {
                    (p - b[i][j]) % p
                }
            }
        }
        evaluations.push(det(m, p))
    }
    let mut out = vec![0; d + 1];
    for j in 0..=d {
        let mut basis = vec![1];
        let mut denominator = 1;
        for k in 0..=d {
            if j == k {
                continue;
            }
            basis = times_linear(&basis, k as u64, p);
            denominator = denominator * ((j as u64 + p - k as u64) % p) % p
        }
        let scale = evaluations[j] * pow(denominator, p - 2, p) % p;
        for k in 0..=d {
            out[k] = (out[k] + scale * basis[k]) % p
        }
    }
    assert_eq!(out[d], 1);
    out
}
fn trim(v: &mut Vec<u64>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}
fn remainder(mut a: Vec<u64>, b: &[u64], p: u64) -> Vec<u64> {
    let inverse = pow(*b.last().unwrap(), p - 2, p);
    while a.len() >= b.len() && !a.is_empty() {
        let shift = a.len() - b.len();
        let factor = *a.last().unwrap() * inverse % p;
        for j in 0..b.len() {
            a[j + shift] = (a[j + shift] + p - factor * b[j] % p) % p
        }
        trim(&mut a)
    }
    a
}
fn squarefree(mut a: Vec<u64>, p: u64) -> bool {
    let mut b: Vec<_> = (1..a.len()).map(|i| i as u64 * a[i] % p).collect();
    trim(&mut b);
    while !b.is_empty() {
        let r = remainder(a, &b, p);
        a = b;
        b = r
    }
    a.len() == 1
}
fn primes(p: u64) -> bool {
    p > 2 && (2..).take_while(|x| x * x <= p).all(|x| p % x != 0)
}
fn main() {
    let root = "research/hec_residual_span_20261116/order4";
    let mut expected_phi: Vec<_> = (0..=8192u64).collect();
    for i in 2..=8192 {
        if expected_phi[i] == i as u64 {
            for j in (i..=8192).step_by(i) {
                expected_phi[j] -= expected_phi[j] / i as u64
            }
        }
    }
    let expected: BTreeSet<_> = (1..=8192)
        .filter(|&i| expected_phi[i] <= 64)
        .map(|x| x as u64)
        .collect();
    assert_eq!(expected.len(), 127);
    let mut f = BTreeMap::new();
    for l in fs::read_to_string(format!("{root}/WEIL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let coefficients: Vec<i128> = v[2]
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        assert_eq!(coefficients.len(), 9);
        assert_eq!(coefficients[8], 1);
        f.insert(
            (v[0].parse::<u64>().unwrap(), v[1].parse::<usize>().unwrap()),
            coefficients,
        );
    }
    let mut declared = BTreeSet::new();
    for l in fs::read_to_string(format!("{root}/CONTROL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        if v[12] == "true" {
            declared.insert((v[0].parse::<u64>().unwrap(), v[1].parse::<usize>().unwrap()));
        }
    }
    assert_eq!(declared.len(), 2);
    let negative = [-1i128, 0, 0, 0, 0, 0, 0, 0, 1];
    assert!(!squarefree(characteristic(&negative, 2, 10007), 10007));
    let mut positive = vec![1i128];
    let mut product = vec![1u64];
    for j in 1..=8i128 {
        let mut out = vec![0; positive.len() + 1];
        for i in 0..positive.len() {
            out[i] -= j * positive[i];
            out[i + 1] += positive[i]
        }
        positive = out;
        product = times_linear(&product, (j * j) as u64, 10007)
    }
    let computed = characteristic(&positive, 2, 10007);
    assert_eq!(computed, product);
    assert!(squarefree(computed, 10007));
    println!("p\tmodel\tm\tprime\tindependent_squarefree");
    let mut covered: BTreeMap<_, BTreeSet<u64>> = BTreeMap::new();
    let (mut verified, mut unresolved) = (0, 0);
    for l in fs::read_to_string(format!("{root}/WITNESSES.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let key = (v[0].parse::<u64>().unwrap(), v[1].parse::<usize>().unwrap());
        let m = v[2].parse::<u64>().unwrap();
        assert!(expected.contains(&m));
        if v[3] == "none" {
            unresolved += 1;
            continue;
        }
        let r = v[3].parse::<u64>().unwrap();
        assert!(primes(r));
        assert!(r > 8);
        assert!(squarefree(characteristic(&f[&key], m, r), r));
        assert!(covered.entry(key).or_default().insert(m));
        verified += 1;
        println!("{}\t{}\t{m}\t{r}\ttrue", key.0, key.1)
    }
    for key in &declared {
        assert_eq!(&covered[key], &expected)
    }
    let even = &f[&(13, 1)];
    assert!(even[0] != 0 && (1..=7).step_by(2).all(|j| even[j] == 0));
    eprintln!("verified_positive_witnesses={verified} retained_unresolved={unresolved} declared_absolute_models={} all_candidate_exponents=127 independent_det_interpolation=true even_polynomial_negative_verified=true",declared.len());
}
