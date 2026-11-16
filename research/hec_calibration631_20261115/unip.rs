type U = Vec<u64>;
const P: u64 = 631;
fn trim(mut a: U) -> U {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn add(a: &[u64], b: &[u64], minus: bool) -> U {
    let mut c = vec![0; a.len().max(b.len())];
    for i in 0..c.len() {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        c[i] = if minus { (x + P - y) % P } else { (x + y) % P };
    }
    trim(c)
}
fn scale(a: &[u64], b: u64) -> U {
    trim(a.iter().map(|x| x * b % P).collect())
}
fn mul(a: &[u64], b: &[u64]) -> U {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] = (c[i + j] + x * y) % P;
        }
    }
    trim(c)
}
fn power(mut a: u64, mut e: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if e & 1 != 0 {
            z = z * a % P
        }
        a = a * a % P;
        e >>= 1;
    }
    z
}
fn div(a: &[u64], b: &[u64]) -> (U, U) {
    assert!(!b.is_empty());
    let mut a = trim(a.to_vec());
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    let z = power(*b.last().unwrap(), (P-2));
    while a.len() >= b.len() {
        let k = a.len() - b.len();
        let c = a[a.len() - 1] * z % P;
        q[k] = c;
        for (i, x) in b.iter().enumerate() {
            a[k + i] = (a[k + i] + P - c * x % P) % P;
        }
        a = trim(a);
    }
    (trim(q), a)
}
fn rem(a: &[u64], m: &[u64]) -> U {
    div(a, m).1
}
fn mm(a: &[u64], b: &[u64], m: &[u64]) -> U {
    rem(&mul(a, b), m)
}
fn gcd(mut a: U, mut b: U) -> U {
    while !b.is_empty() {
        let t = rem(&a, &b);
        a = b;
        b = t;
    }
    if a.is_empty() {
        a
    } else {
        scale(&a, power(*a.last().unwrap(), (P-2)))
    }
}
fn inverse(a: &[u64], m: &[u64]) -> U {
    let (mut r, mut s) = (m.to_vec(), rem(a, m));
    let (mut x, mut y) = (vec![], vec![1]);
    while !s.is_empty() {
        let (q, t) = div(&r, &s);
        r = s;
        s = t;
        let z = add(&x, &mul(&q, &y), true);
        x = y;
        y = z;
    }
    assert_eq!(r.len(), 1);
    rem(&scale(&x, power(r[0], (P-2))), m)
}
fn mpow(mut a: U, mut e: u64, m: &[u64]) -> U {
    let mut z = vec![1];
    while e > 0 {
        if e & 1 != 0 {
            z = mm(&z, &a, m)
        }
        a = mm(&a, &a, m);
        e >>= 1;
    }
    z
}
fn eval(a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |z, c| (z * x + c) % P)
}
fn deriv(a: &[u64]) -> U {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| i as u64 * c % P)
            .collect(),
    )
}
fn reduce(a: &[u64], m: &[u64]) -> U {
    rem(a, m)
}
