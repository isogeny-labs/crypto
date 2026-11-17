use std::sync::atomic::{AtomicU64, Ordering};
static BN: [AtomicU64; 6] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];
fn bt(e: &E, mut v: Vec<K>) -> Vec<K> {
    while v.len() > 1 && v.last().unwrap() == &e.c(0) {
        v.pop();
    }
    v
}
fn bz(e: &E, v: &[K]) -> bool {
    v.iter().all(|x| x == &e.c(0))
}
fn bs(e: &E, v: &[K], c: &K) -> Vec<K> {
    bt(e, v.iter().map(|x| e.mulk(x, c)).collect())
}
fn ba(e: &E, a: &[K], b: &[K], c: &K) -> Vec<K> {
    let mut v = vec![e.c(0); a.len().max(b.len())];
    v[..a.len()].clone_from_slice(a);
    for (i, x) in b.iter().enumerate() {
        v[i] = e.addk(&v[i], &e.mulk(x, c))
    }
    bt(e, v)
}
fn bm(e: &E, a: &[K], b: &[K]) -> Vec<K> {
    let mut v = vec![e.c(0); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            v[i + j] = e.addk(&v[i + j], &e.mulk(x, y))
        }
    }
    bt(e, v)
}
fn be(e: &E, a: &[K], x: &K) -> K {
    a.iter()
        .rev()
        .fold(e.c(0), |v, c| e.addk(&e.mulk(&v, x), c))
}
fn bd(e: &E, a: &[K], b: &[K]) -> (Vec<K>, Vec<K>) {
    let b = bt(e, b.to_vec());
    assert!(!bz(e, &b));
    let mut v = bt(e, a.to_vec());
    let mut q = vec![
        e.c(0);
        if v.len() >= b.len() {
            v.len() - b.len() + 1
        } else {
            1
        }
    ];
    let inv = e.inverse(b.last().unwrap());
    while !bz(e, &v) && v.len() >= b.len() {
        let i = v.len() - b.len();
        let c = e.mulk(v.last().unwrap(), &inv);
        q[i] = e.addk(&q[i], &c);
        for (j, x) in b.iter().enumerate() {
            v[i + j] = e.subk(&v[i + j], &e.mulk(&c, x))
        }
        v = bt(e, v)
    }
    (bt(e, q), v)
}
fn br(e: &E, a: Vec<K>, h: &[K]) -> Vec<K> {
    bd(e, &a, h).1
}
fn bp(e: &E, x: &K, mut n: usize) -> K {
    let mut z = e.c(1);
    let mut x = x.clone();
    while n > 0 {
        if n & 1 == 1 {
            z = e.mulk(&z, &x)
        }
        x = e.mulk(&x, &x);
        n >>= 1
    }
    z
}
fn bres(e: &E, f: &[K], g: &[K]) -> K {
    let mut f = bt(e, f.to_vec());
    let mut g = bt(e, g.to_vec());
    let mut z = e.c(1);
    loop {
        let n = f.len() - 1;
        let m = g.len() - 1;
        if n == 0 {
            return e.mulk(&z, &bp(e, &f[0], m));
        }
        if bz(e, &g) {
            return e.c(0);
        }
        if m == 0 {
            return e.mulk(&z, &bp(e, &g[0], n));
        }
        let (_, r) = bd(e, &f, &g);
        if bz(e, &r) {
            return e.c(0);
        }
        if n * m % 2 == 1 {
            z = e.negk(&z)
        }
        z = e.mulk(&z, &bp(e, &g[m], n - r.len() + 1));
        f = g;
        g = r
    }
}
fn bxgcd(e: &E, h: &[K], a: &[K]) -> (Vec<K>, Option<Vec<K>>) {
    let mut r0 = h.to_vec();
    let mut r1 = bt(e, a.to_vec());
    let mut s0 = vec![e.c(0)];
    let mut s1 = vec![e.c(1)];
    while !bz(e, &r1) {
        if r1.len() == 1 {
            return (vec![e.c(1)], Some(br(e, bs(e, &s1, &e.inverse(&r1[0])), h)));
        }
        let (q, r) = bd(e, &r0, &r1);
        let s = br(e, ba(e, &s0, &bm(e, &q, &s1), &e.negk(&e.c(1))), h);
        r0 = r1;
        r1 = r;
        s0 = s1;
        s1 = s
    }
    let inv = e.inverse(r0.last().unwrap());
    (bs(e, &r0, &inv), None)
}
fn bnorm(e: &E, h: &[K], a: &[K], b: &[K]) -> ([K; 2], usize) {
    let a = br(e, a.to_vec(), h);
    let b = br(e, b.to_vec(), h);
    if bz(e, &b) {
        return ([bres(e, h, &a), e.c(0)], 5);
    }
    let (d, inv) = bxgcd(e, h, &a);
    if d.len() == 1 {
        let z = bres(e, h, &a);
        let der: Vec<_> = (1..h.len())
            .map(|i| e.mulk(&h[i], &e.c(i as u32)))
            .collect();
        let q = br(e, bm(e, &bm(e, &inv.unwrap(), &b), &der), h);
        let tr = q.get(h.len() - 2).cloned().unwrap_or(e.c(0));
        return ([z.clone(), e.mulk(&z, &tr)], 3);
    }
    if d.len() == 2 {
        let t = e.quotient(&e.negk(&d[0]), &d[1]);
        let (bar, rem) = bd(e, h, &d);
        assert!(bz(e, &rem));
        return ([e.c(0), e.mulk(&be(e, &b, &t), &bres(e, &bar, &a))], 4);
    }
    ([e.c(0), e.c(0)], 5)
}
fn bh(e: &E, xs: &[K]) -> Vec<K> {
    xs.iter()
        .fold(vec![e.c(1)], |h, x| bm(e, &h, &[e.negk(x), e.c(1)]))
}
fn bi(e: &E, xs: &[K], ys: &[K], h: &[K]) -> Vec<K> {
    let mut v = vec![e.c(0)];
    for (x, y) in xs.iter().zip(ys) {
        let (q, rem) = bd(e, h, &[e.negk(x), e.c(1)]);
        assert!(bz(e, &rem));
        let c = e.quotient(y, &be(e, &q, x));
        v = ba(e, &v, &q, &c)
    }
    v
}
