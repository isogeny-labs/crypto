//! Matched order-two/order-four GLS rho control, v2 64-jump/256-restart amendment.
#![allow(dead_code)]
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct F {
    a: u64,
    b: u64,
}
const Z: F = F { a: 0, b: 0 };
const ONE: F = F { a: 1, b: 0 };
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct P {
    x: F,
    y: F,
    inf: bool,
}
const O: P = P {
    x: Z,
    y: Z,
    inf: true,
};
#[derive(Clone, Copy)]
struct C {
    p: u64,
    d: u64,
    r: u64,
    cof: u64,
    c: F,
    a: F,
    b: F,
    alpha2: F,
    alpha3: F,
}
#[derive(Clone, Default)]
struct Ops {
    add: u64,
    fmul: u64,
    finv: u64,
    psi: u64,
    probes: u64,
    row: u64,
    minv: u64,
    msquare: u64,
}
fn m(x: i128, p: u64) -> u64 {
    x.rem_euclid(p as i128) as u64
}
fn pmul(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 * y as u128) % p as u128) as u64
}
fn mpow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 == 1 {
            z = pmul(z, x, p);
        }
        x = pmul(x, x, p);
        n >>= 1;
    }
    z
}
fn minv(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    mpow(x, p - 2, p)
}
fn prime(n: u64) -> bool {
    n >= 2 && (2..=((n as f64).sqrt() as u64)).all(|v| n % v != 0)
}
fn fa(x: F, y: F, p: u64) -> F {
    F {
        a: (x.a + y.a) % p,
        b: (x.b + y.b) % p,
    }
}
fn fn_eg(x: F, p: u64) -> F {
    F {
        a: m(-(x.a as i128), p),
        b: m(-(x.b as i128), p),
    }
}
fn fs(x: F, y: F, p: u64) -> F {
    fa(x, fn_eg(y, p), p)
}
fn fm(x: F, y: F, p: u64, d: u64, o: &mut Ops) -> F {
    o.fmul += 1;
    F {
        a: m(
            x.a as i128 * y.a as i128 + d as i128 * x.b as i128 * y.b as i128,
            p,
        ),
        b: m(x.a as i128 * y.b as i128 + x.b as i128 * y.a as i128, p),
    }
}
fn fi(x: F, p: u64, d: u64, o: &mut Ops) -> F {
    assert_ne!(x, Z);
    o.finv += 1;
    let norm = m(
        x.a as i128 * x.a as i128 - d as i128 * x.b as i128 * x.b as i128,
        p,
    );
    let z = minv(norm, p);
    F {
        a: pmul(x.a, z, p),
        b: pmul(m(-(x.b as i128), p), z, p),
    }
}
fn fpow(mut x: F, mut n: u64, p: u64, d: u64, o: &mut Ops) -> F {
    let mut z = ONE;
    while n > 0 {
        if n & 1 == 1 {
            z = fm(z, x, p, d, o);
        }
        x = fm(x, x, p, d, o);
        n >>= 1;
    }
    z
}
fn ffrob(x: F, p: u64) -> F {
    F {
        a: x.a,
        b: m(-(x.b as i128), p),
    }
}
fn pn(x: P, p: u64) -> P {
    if x.inf {
        x
    } else {
        P {
            x: x.x,
            y: fn_eg(x.y, p),
            inf: false,
        }
    }
}
fn on(x: P, c: &C, o: &mut Ops) -> bool {
    if x.inf {
        return true;
    }
    let x2 = fm(x.x, x.x, c.p, c.d, o);
    let x3 = fm(x2, x.x, c.p, c.d, o);
    fm(x.y, x.y, c.p, c.d, o) == fa(fa(x3, fm(c.a, x.x, c.p, c.d, o), c.p), c.b, c.p)
}
fn pa(x: P, y: P, c: &C, o: &mut Ops) -> P {
    if x.inf {
        return y;
    }
    if y.inf {
        return x;
    }
    o.add += 1;
    if x.x == y.x && fa(x.y, y.y, c.p) == Z {
        return O;
    }
    let lam = if x == y {
        let xx = fm(x.x, x.x, c.p, c.d, o);
        let num = fa(fm(F { a: 3, b: 0 }, xx, c.p, c.d, o), c.a, c.p);
        fm(
            num,
            fi(fm(F { a: 2, b: 0 }, x.y, c.p, c.d, o), c.p, c.d, o),
            c.p,
            c.d,
            o,
        )
    } else {
        fm(
            fs(y.y, x.y, c.p),
            fi(fs(y.x, x.x, c.p), c.p, c.d, o),
            c.p,
            c.d,
            o,
        )
    };
    let xx = fs(fs(fm(lam, lam, c.p, c.d, o), x.x, c.p), y.x, c.p);
    let yy = fs(fm(lam, fs(x.x, xx, c.p), c.p, c.d, o), x.y, c.p);
    P {
        x: xx,
        y: yy,
        inf: false,
    }
}
fn pm(mut n: u64, mut x: P, c: &C, o: &mut Ops) -> P {
    let mut z = O;
    while n > 0 {
        if n & 1 == 1 {
            z = pa(z, x, c, o);
        }
        n >>= 1;
        if n > 0 {
            x = pa(x, x, c, o);
        }
    }
    z
}
fn psi(x: P, c: &C, o: &mut Ops) -> P {
    o.psi += 1;
    if x.inf {
        return x;
    }
    P {
        x: fm(c.alpha2, ffrob(x.x, c.p), c.p, c.d, o),
        y: fm(c.alpha3, ffrob(x.y, c.p), c.p, c.d, o),
        inf: false,
    }
}
fn make_curve(p: u64, a0: u64, b0: u64, r: u64, cof: u64) -> C {
    assert!(prime(p) && prime(r));
    let d = (2..p).find(|&x| mpow(x, (p - 1) / 2, p) == p - 1).unwrap();
    let mut o = Ops::default();
    let c = (0..p)
        .flat_map(|a| (1..p).map(move |b| F { a, b }))
        .find(|&x| fpow(x, (p * p - 1) / 2, p, d, &mut o) != ONE)
        .unwrap();
    let c2 = fm(c, c, p, d, &mut o);
    let c3 = fm(c2, c, p, d, &mut o);
    let alpha = fpow(c, p * p - 1 - (p - 1) / 2, p, d, &mut o);
    let alpha2 = fm(alpha, alpha, p, d, &mut o);
    let alpha3 = fm(alpha2, alpha, p, d, &mut o);
    C {
        p,
        d,
        r,
        cof,
        c,
        a: fm(F { a: a0, b: 0 }, c2, p, d, &mut o),
        b: fm(F { a: b0, b: 0 }, c3, p, d, &mut o),
        alpha2,
        alpha3,
    }
}
fn enumerate(c: &C, o: &mut Ops) -> Vec<P> {
    let mut roots: HashMap<F, Vec<F>> = HashMap::new();
    for a in 0..c.p {
        for b in 0..c.p {
            let y = F { a, b };
            let y2 = fm(y, y, c.p, c.d, o);
            roots.entry(y2).or_default().push(y);
        }
    }
    let mut out = Vec::new();
    for a in 0..c.p {
        for b in 0..c.p {
            let x = F { a, b };
            let x2 = fm(x, x, c.p, c.d, o);
            let rhs = fa(
                fa(fm(x2, x, c.p, c.d, o), fm(c.a, x, c.p, c.d, o), c.p),
                c.b,
                c.p,
            );
            if let Some(ys) = roots.get(&rhs) {
                for &y in ys {
                    out.push(P { x, y, inf: false });
                }
            }
        }
    }
    out.sort();
    out
}
fn generator(c: &C, points: &[P], o: &mut Ops) -> P {
    for &q in points {
        let g = pm(c.cof, q, c, o);
        if !g.inf && pm(c.r, g, c, o) == O {
            return g;
        }
    }
    panic!("no selected subgroup generator")
}
fn lambda(c: &C, g: P, o: &mut Ops) -> u64 {
    let pg = psi(g, c, o);
    for x in 1..c.r {
        o.msquare += 1;
        if pmul(x, x, c.r) == c.r - 1 && pm(x, g, c, o) == pg {
            return x;
        }
    }
    panic!("psi has no scalar action on selected subgroup")
}

fn modulus(x: i128, r: u64) -> u64 {
    m(x, r)
}

fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

fn canonicalize(x: &mut P, a: &mut u64, b: &mut u64, c: &C, eigen: u64, fold4: bool, o: &mut Ops) -> u64 {
    let negative = pn(*x, c.p);
    let mut choices = vec![(*x, 1u64), (negative, c.r - 1)];
    if fold4 {
        let transformed = psi(*x, c, o);
        choices.push((transformed, eigen));
        choices.push((pn(transformed, c.p), c.r - eigen));
    }
    let mut best = choices[0];
    for choice in choices.into_iter().skip(1) {
        if choice.0 < best.0 {
            best = choice;
        }
    }
    *x = best.0;
    *a = pmul(*a, best.1, c.r);
    *b = pmul(*b, best.1, c.r);
    if fold4 { 3 } else { 1 }
}

fn rho(c: &C, g: P, target: P, known: u64, seed: usize, fold4: bool, expected_eigen: u64) {
    let started = Instant::now();
    let mut o = Ops::default();
    let eigen = if fold4 { lambda(c, g, &mut o) } else { 0 };
    if fold4 {
        assert_eq!(eigen, expected_eigen);
        assert_eq!(pmul(eigen, eigen, c.r), c.r - 1);
    }
    let mut found = None;
    let mut states = 0u64;
    let mut comparisons = 0u64;
    let mut collisions = 0u64;
    let mut restarts = 0u64;
    'again: for restart in 0..256u64 {
        restarts = restart;
        let mut seen = HashMap::<P, (u64, u64)>::new();
        let salt = mix(c.p ^ (target.x.a << 10) ^ (target.x.b << 20)
            ^ (target.y.a << 30) ^ (target.y.b << 40)
            ^ ((seed as u64) << 50) ^ restart);
        let jumps: Vec<(P, u64, u64)> = (0..64u64)
            .map(|j| {
                let ja = 1 + mix(salt ^ j ^ 0xa11ce) % (c.r - 1);
                let jb = 1 + mix(salt ^ j ^ 0xbadc0de) % (c.r - 1);
                (pa(pm(ja, g, c, &mut o), pm(jb, target, c, &mut o), c, &mut o), ja, jb)
            })
            .collect();
        let mut a = mix(salt ^ 0x1234) % c.r;
        let mut b = mix(salt ^ 0x5678) % c.r;
        let mut x = pa(pm(a, g, c, &mut o), pm(b, target, c, &mut o), c, &mut o);
        for _ in 0..(8 * c.r) {
            comparisons += canonicalize(&mut x, &mut a, &mut b, c, eigen, fold4, &mut o);
            states += 1;
            o.probes += 1;
            if let Some(&(aa, bb)) = seen.get(&x) {
                collisions += 1;
                let den = modulus(b as i128 - bb as i128, c.r);
                if den != 0 {
                    o.minv += 1;
                    let k = modulus((aa as i128 - a as i128) * minv(den, c.r) as i128, c.r);
                    if pm(k, g, c, &mut o) == target {
                        found = Some(k);
                        break 'again;
                    }
                }
                break;
            }
            seen.insert(x, (a, b));
            let idx = (mix(x.x.a ^ (x.x.b << 16) ^ (x.y.a << 32)
                ^ (x.y.b << 48) ^ salt) & 63) as usize;
            let (jump, ja, jb) = jumps[idx];
            x = pa(x, jump, c, &mut o);
            a = (a + ja) % c.r;
            b = (b + jb) % c.r;
        }
    }
    let verified = found.is_some_and(|k| pm(k, g, c, &mut Ops::default()) == target && k == known);
    let scalar = found.map_or_else(|| "null".to_string(), |k| k.to_string());
    println!("{{\"kind\":\"rho\",\"p\":{},\"r\":{},\"target_scalar_fixture\":{},\"seed\":{},\"arm\":\"{}\",\"lambda\":{},\"states\":{},\"canonical_comparisons\":{},\"collisions\":{},\"restarts\":{},\"recovered\":{},\"verified\":{},\"group_add\":{},\"f2_mul\":{},\"f2_inv\":{},\"psi_eval\":{},\"probes\":{},\"mod_inv\":{},\"total_ns\":{}}}",
        c.p, c.r, known, seed, if fold4 {"rho-psi4"} else {"rho-negation"},
        if fold4 {eigen.to_string()} else {"null".to_string()},
        states, comparisons, collisions, restarts, scalar, verified,
        o.add, o.fmul, o.finv, o.psi, o.probes, o.minv, started.elapsed().as_nanos());
}

fn main() {
    for (p, a0, b0, r, cof) in [(101, 3, 17, 401, 25), (211, 1, 1, 44221, 1), (307, 1, 6, 509, 185)] {
        let c = make_curve(p, a0, b0, r, cof);
        let mut setup = Ops::default();
        let points = enumerate(&c, &mut setup);
        assert_eq!(points.len() as u64 + 1, r * cof);
        let g = generator(&c, &points, &mut setup);
        let eigen = lambda(&c, g, &mut setup);
        assert_eq!(psi(psi(g, &c, &mut setup), &c, &mut setup), pn(g, p));
        println!("{{\"kind\":\"curve\",\"p\":{},\"a\":{},\"b\":{},\"r\":{},\"cofactor\":{},\"generator\":[{},{},{},{}],\"lambda\":{}}}",
            p, a0, b0, r, cof, g.x.a, g.x.b, g.y.a, g.y.b, eigen);
        for k in [37u64, 113, 271] {
            let target = pm(k, g, &c, &mut setup);
            for seed in 0..32 {
                rho(&c, g, target, k, seed, false, eigen);
                rho(&c, g, target, k, seed, true, eigen);
            }
        }
    }
}
