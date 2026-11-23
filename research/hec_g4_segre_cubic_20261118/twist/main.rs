const P: u32 = 31;
const Q: u32 = P * P * P;
type Poly = Vec<u32>;
fn unpack(a: u32) -> [u32; 3] {
    [a % P, a / P % P, a / (P * P)]
}
fn pack(v: [u32; 3]) -> u32 {
    v[0] + P * (v[1] + P * v[2])
}
fn add(a: u32, b: u32) -> u32 {
    let x = unpack(a);
    let y = unpack(b);
    pack(std::array::from_fn(|i| (x[i] + y[i]) % P))
}
fn neg(a: u32) -> u32 {
    pack(unpack(a).map(|x| (P - x) % P))
}
fn mul(a: u32, b: u32) -> u32 {
    let [x, y, z] = unpack(a);
    let [u, v, w] = unpack(b);
    pack([
        (x * u + 28 * (y * w + z * v)) % P,
        (x * v + y * u + 28 * z * w) % P,
        (x * w + y * v + z * u) % P,
    ])
}
fn pow(mut a: u32, mut n: u32) -> u32 {
    let mut b = 1;
    while n != 0 {
        if n % 2 == 1 {
            b = mul(b, a)
        }
        a = mul(a, a);
        n /= 2;
    }
    b
}
fn trim(p: &mut Poly) {
    while p.last() == Some(&0) {
        p.pop();
    }
}
fn monic(mut p: Poly) -> Poly {
    trim(&mut p);
    if let Some(&x) = p.last() {
        let iv = pow(x, Q - 2);
        assert_eq!(mul(iv, x), 1);
        for a in &mut p {
            *a = mul(*a, iv)
        }
    }
    p
}
fn rem(mut a: Poly, b: &Poly) -> Poly {
    assert_eq!(b.last(), Some(&1));
    trim(&mut a);
    while a.len() >= b.len() {
        let shift = a.len() - b.len();
        let lc = neg(*a.last().unwrap());
        for (i, &x) in b.iter().enumerate() {
            a[shift + i] = add(a[shift + i], mul(lc, x));
        }
        trim(&mut a);
    }
    a
}
fn product(a: &Poly, b: &Poly, m: &Poly) -> Poly {
    let mut v = vec![0; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            v[i + j] = add(v[i + j], mul(x, y));
        }
    }
    rem(v, m)
}
fn affine_roots(mut p: Poly, q: u32) -> usize {
    p = monic(p);
    assert!(!p.is_empty());
    if p.len() == 1 {
        return 0;
    }
    let mut a = rem(vec![0, 1], &p);
    let mut b = vec![1];
    let mut n = q;
    while n > 0 {
        if n % 2 == 1 {
            b = product(&a, &b, &p)
        }
        a = product(&a, &a, &p);
        n /= 2;
    }
    b.resize(b.len().max(2), 0);
    b[1] = add(b[1], neg(1));
    let mut d = monic(b);
    while !d.is_empty() {
        let r = rem(p, &d);
        p = d;
        d = monic(r);
    }
    p.len() - 1
}
fn main() {
    for x in 0..P {
        assert_ne!((x * x * x + 3) % P, 0);
    }
    let raw = include_str!("../CONTROL.txt");
    let m = raw.lines().find(|s| s.starts_with("MODEL ")).unwrap();
    let c: Vec<u32> = m
        .split("coeff=[")
        .nth(1)
        .unwrap()
        .trim_end_matches(']')
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut traces = Vec::new();
    for q in [P, Q] {
        let mut totals = vec![];
        for axis in 0..2 {
            let mut hist = [0usize; 4];
            for x in 0..=q {
                let powers = if x == q {
                    [0, 0, 0, 1]
                } else {
                    [1, x, mul(x, x), pow(x, 3)]
                };
                let p: Poly = (0..4)
                    .map(|j| {
                        (0..4).fold(0, |a, i| {
                            add(
                                a,
                                mul(c[if axis == 0 { 4 * i + j } else { 4 * j + i }], powers[i]),
                            )
                        })
                    })
                    .collect();
                let inf = usize::from(p[3] == 0);
                let n = affine_roots(p.clone(), q) + inf;
                if q == P {
                    let brute = (0..P)
                        .filter(|&y| p.iter().rev().fold(0, |v, &a| add(mul(v, y), a)) == 0)
                        .count()
                        + inf;
                    assert_eq!(n, brute);
                }
                hist[n] += 1;
            }
            let total: usize = hist.iter().enumerate().map(|(n, k)| n * k).sum();
            println!("COUNT field={q} axis={axis} hist={hist:?} points={total}");
            totals.push(total);
        }
        assert_eq!(totals[0], totals[1]);
        traces.push(q as i64 + 1 - totals[0] as i64);
    }
    assert_eq!(traces[0], 0);
    assert_eq!(traces[1].abs(), 360);
    println!(
        "TWIST traces={traces:?} quotient_trace3=-360 quotient_matches_model={}",
        traces[1] == -360
    );
}
