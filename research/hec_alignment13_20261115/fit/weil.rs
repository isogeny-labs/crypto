#[path = "arith.rs"]
#[allow(dead_code)]
mod arith;
use arith::{base, Curve, Div, Field, Pol, E, O, P, Z};
use hec_native_ic::{fp::Fp, jac, poly::Poly};
use std::{collections::HashMap, fs};
#[derive(Clone, Debug, PartialEq, Eq)]
struct H {
    a: Pol,
    b: Pol,
}
fn pow(a: &Pol, n: usize, f: &Field) -> Pol {
    (0..n).fold(Pol::one(), |s, _| s.mul(a, f))
}
fn inverse(a: &Pol, m: &Pol, f: &Field) -> Option<Pol> {
    let (g, s, _) = Pol::xgcd(a, m, f);
    (g == Pol::one()).then(|| s.rem(m, f))
}
fn derivative(a: &Pol, f: &Field) -> Pol {
    Pol::new(
        (1..a.0.len())
            .map(|i| f.mul(base(i as u64), a.coeff(i)))
            .collect(),
    )
}
fn squarefree(a: &Pol, f: &Field) -> bool {
    Pol::xgcd(a, &derivative(a, f), f).0 == Pol::one()
}
fn nullvector(mut m: Vec<Vec<E>>, f: &Field) -> Option<Vec<E>> {
    let cols = m[0].len();
    let mut pivots = Vec::new();
    for j in 0..cols {
        let row = pivots.len();
        let Some(i) = (row..m.len()).find(|&i| m[i][j] != Z) else {
            continue;
        };
        m.swap(i, row);
        let inv = f.inv(m[row][j]);
        for k in j..cols {
            m[row][k] = f.mul(m[row][k], inv)
        }
        for i in 0..m.len() {
            if i != row {
                let t = m[i][j];
                for k in j..cols {
                    m[i][k] = f.sub(m[i][k], f.mul(t, m[row][k]))
                }
            }
        }
        pivots.push(j);
    }
    if pivots.len() + 1 != cols {
        return None;
    }
    let free = (0..cols).find(|j| !pivots.contains(j)).unwrap();
    let mut v = vec![Z; cols];
    v[free] = O;
    for (i, &j) in pivots.iter().enumerate() {
        v[j] = f.neg(m[i][free])
    }
    let lead = f.inv(*v.iter().find(|&&x| x != Z).unwrap());
    for x in &mut v {
        *x = f.mul(*x, lead)
    }
    Some(v)
}
fn valid_h(c: &Curve, d: &Div, h: &H) -> bool {
    let f = &c.field;
    let u3 = pow(&d.u, 3, f);
    if h.a.degree() > 4 || h.b.degree() != 1 {
        return false;
    }
    let norm = h.a.mul(&h.a, f).sub(&c.f.mul(&h.b.mul(&h.b, f), f), f);
    let (q, r) = norm.divrem(&u3, f);
    if !r.0.is_empty() || q.degree() != 0 || q.coeff(0) == Z {
        return false;
    }
    let Some(mut w) = lift(c, d) else {
        return false;
    };
    w = h.a.add(&h.b.mul(&w, f), f).rem(&u3, f);
    w.0.is_empty()
}
fn lift(c: &Curve, d: &Div) -> Option<Pol> {
    let f = &c.field;
    if !c.valid(d) || d.u.degree() != 3 || !squarefree(&d.u, f) {
        return None;
    }
    let m = pow(&d.u, 3, f);
    let mut w = d.v.clone();
    for _ in 0..2 {
        let wi = inverse(&w, &m, f)?;
        w = w.add(&c.f.mul(&wi, f), f).scale(base(64), f).rem(&m, f)
    }
    if !w.mul(&w, f).sub(&c.f, f).rem(&m, f).0.is_empty() {
        return None;
    }
    Some(w)
}
fn construct(c: &Curve, d: &Div) -> Option<H> {
    if c.is_zero(d) {
        return Some(H {
            a: Pol::one(),
            b: Pol::zero(),
        });
    }
    let f = &c.field;
    let w = lift(c, d)?;
    let m = pow(&d.u, 3, f);
    let cols: Vec<_> = (0..5)
        .map(|i| {
            let mut c = vec![Z; i + 1];
            c[i] = O;
            Pol::new(c)
        })
        .chain([w.clone(), Pol::new(vec![Z, O]).mul(&w, f).rem(&m, f)])
        .collect();
    let matrix = (0..9)
        .map(|i| cols.iter().map(|p| p.coeff(i)).collect())
        .collect();
    let v = nullvector(matrix, f)?;
    let h = H {
        a: Pol::new(v[..5].to_vec()),
        b: Pol::new(v[5..].to_vec()),
    };
    valid_h(c, d, &h).then_some(h)
}
fn det(mut a: Vec<Vec<E>>, f: &Field) -> E {
    let n = a.len();
    let mut out = O;
    for j in 0..n {
        let Some(i) = (j..n).find(|&i| a[i][j] != Z) else {
            return Z;
        };
        if i != j {
            a.swap(i, j);
            out = f.neg(out)
        }
        let p = a[j][j];
        out = f.mul(out, p);
        let inv = f.inv(p);
        for i in j + 1..n {
            let t = f.mul(a[i][j], inv);
            for k in j + 1..n {
                a[i][k] = f.sub(a[i][k], f.mul(t, a[j][k]))
            }
        }
    }
    out
}
fn norm(a: &Pol, u: &Pol, f: &Field) -> E {
    let n = u.degree() as usize;
    let mut cols = Vec::new();
    let mut v = a.rem(u, f);
    for _ in 0..n {
        cols.push(v.clone());
        v = Pol::new(vec![Z, O]).mul(&v, f).rem(u, f)
    }
    det(
        (0..n)
            .map(|i| cols.iter().map(|v| v.coeff(i)).collect())
            .collect(),
        f,
    )
}
fn resultant(a: &Pol, b: &Pol, f: &Field) -> E {
    if b.0.is_empty() {
        return Z;
    }
    if b.degree() == 0 {
        return f.pow(b.coeff(0), a.degree() as u128);
    }
    let m = a.degree();
    let n = b.degree();
    let r = a.rem(b, f);
    if r.0.is_empty() {
        return Z;
    }
    let mut v = f.mul(
        f.pow(*b.0.last().unwrap(), (m - r.degree()) as u128),
        resultant(b, &r, f),
    );
    if m * n % 2 != 0 {
        v = f.neg(v)
    }
    v
}
fn evaluate(c: &Curve, p: &Div, h: &H, x: &Div) -> Option<E> {
    if c.is_zero(p) {
        return Some(O);
    }
    let f = &c.field;
    if !c.valid(x) || x.u.degree() != 3 || !squarefree(&x.u, f) {
        return None;
    }
    let inv = inverse(&p.u, &x.u, f)?;
    let r = x.v.add(&p.v, f).mul(&inv, f).rem(&x.u, f);
    let hv = h.a.add(&h.b.mul(&x.v, f), f);
    Some(f.mul(f.pow(r.coeff(2), 3), norm(&hv, &x.u, f)))
}
fn direct(c: &Curve, p: &Div, h: &H, pts: &[(E, E)]) -> Option<E> {
    if c.is_zero(p) {
        return Some(O);
    }
    let f = &c.field;
    let mut n = vec![Vec::new(); 3];
    let mut d = vec![Vec::new(); 3];
    let mut product = O;
    for &(x, y) in pts {
        let u = p.u.eval(x, f);
        if u == Z {
            return None;
        }
        n[0].push(O);
        n[1].push(x);
        n[2].push(f.mul(f.add(y, p.v.eval(x, f)), f.inv(u)));
        d[0].push(O);
        d[1].push(x);
        d[2].push(f.mul(x, x));
        product = f.mul(product, f.add(h.a.eval(x, f), f.mul(h.b.eval(x, f), y)));
    }
    let den = det(d, f);
    if den == Z {
        return None;
    }
    Some(f.mul(f.pow(f.mul(det(n, f), f.inv(den)), 3), product))
}
fn frobh(h: &H, f: &Field) -> H {
    let fr = |p: &Pol| Pol::new(p.0.iter().map(|&a| f.frob(a)).collect());
    H {
        a: fr(&h.a),
        b: fr(&h.b),
    }
}
fn emit(label: &str, k: usize, i: usize, h: &H) {
    print!("{label} {k} {i}");
    for p in [&h.a, &h.b] {
        print!(" {}", p.0.len());
        for a in &p.0 {
            for x in a {
                print!(" {x}")
            }
        }
    }
    println!()
}
fn main() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let input = fs::read_to_string(&args[2]).unwrap();
    let nums: Vec<u64> = input
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(&nums[..3], &[3, 127, 0]);
    let mut at = 5;
    let mut ps = Vec::new();
    for _ in 0..3 {
        let n = nums[at] as usize;
        at += 1;
        ps.push(Poly::from_coeffs(&nums[at..at + n]));
        at += n
    }
    let refc = jac::Curve::new(Fp::new(P), ps[0]);
    let d1 = jac::Div { u: ps[1], v: ps[2] };
    let text = fs::read_to_string(&args[1]).unwrap();
    assert!(text.lines().any(|l| l == "FIELD 127 13 10"));
    let field = Field {
        c: 10,
        q: (P as u128).pow(13),
    };
    assert!(field.irreducible());
    let c = Curve {
        field,
        f: Pol::embed(&refc.f),
    };
    let f = &c.field;
    let mut ks = [Vec::new(), Vec::new()];
    for line in text.lines().filter(|l| l.starts_with("POINT ")) {
        let ns: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        let k = ns[0] as usize;
        assert!(k < 2);
        assert_eq!(ns[1] as usize, ks[k].len());
        let mut at = 2;
        let mut p = Vec::new();
        for _ in 0..2 {
            let n = ns[at] as usize;
            at += 1;
            let mut v = Vec::new();
            for _ in 0..n {
                let a = std::array::from_fn(|i| ns[at + i]);
                at += 13;
                assert!(a.iter().all(|&x| x < P));
                v.push(a)
            }
            p.push(Pol::new(v))
        }
        assert_eq!(at, ns.len());
        let d = Div {
            u: p[0].clone(),
            v: p[1].clone(),
        };
        assert!(c.valid(&d));
        assert!(c.is_zero(&c.small(&d, 3)));
        ks[k].push(d)
    }
    assert!(ks.iter().all(|v| v.len() == 27));
    let mut hs = [Vec::new(), Vec::new()];
    let mut nonzero = 0;
    let mut corrupt = 0;
    for k in 0..2 {
        for (i, p) in ks[k].iter().enumerate() {
            let h = construct(&c, p).expect("unsupported or failed principal function");
            if !c.is_zero(p) {
                nonzero += 1;
                let mut bad = h.clone();
                bad.a = bad.a.add(&Pol::one(), f);
                assert!(!valid_h(&c, p, &bad));
                corrupt += 1
            }
            emit("H", k, i, &h);
            hs[k].push(h)
        }
    }
    let mut frobs = 0;
    for k in 0..2 {
        let ix: HashMap<_, _> = ks[k]
            .iter()
            .enumerate()
            .map(|(i, p)| (p.clone(), i))
            .collect();
        for (i, p) in ks[k].iter().enumerate() {
            let j = ix[&c.frob(p)];
            assert_eq!(frobh(&hs[k][i], f), hs[k][j]);
            frobs += 1
        }
    }
    let embedded = Div {
        u: Pol::embed(&d1.u),
        v: Pol::embed(&d1.v),
    };
    assert!(!c.is_zero(&c.small(&embedded, 3)));
    assert!(construct(&c, &embedded).is_none());
    let mut places = Vec::new();
    for x in 0..P {
        if let Some(y) = refc.fp.sqrt(refc.f.eval(x, &refc.fp)) {
            places.push((base(x), base(y.min(P - y))));
            if places.len() == 16 {
                break;
            }
        }
    }
    assert_eq!(places.len(), 16);
    let mut triples = Vec::new();
    'outer: for i in 0..16 {
        for j in i + 1..16 {
            for k in j + 1..16 {
                triples.push(vec![places[i], places[j], places[k]]);
                if triples.len() == 16 {
                    break 'outer;
                }
            }
        }
    }
    let mut comparisons = 0;
    let mut poles = 0;
    let mut zeros = 0;
    let mut traces = 0;
    for (t, pts) in triples.iter().enumerate() {
        let mut u = Pol::one();
        let mut v = Pol::zero();
        for &(x, _) in pts {
            u = u.mul(&Pol::new(vec![f.neg(x), O]), f)
        }
        for &(x, y) in pts {
            let b = u.exact(&Pol::new(vec![f.neg(x), O]), f);
            v = v.add(&b.scale(f.mul(y, f.inv(b.eval(x, f))), f), f)
        }
        let x = Div { u, v };
        assert!(c.valid(&x));
        for k in 0..2 {
            let mut sum = Z;
            let mut vals = Vec::new();
            for (i, p) in ks[k].iter().enumerate() {
                let val = evaluate(&c, p, &hs[k][i], &x);
                assert_eq!(val, direct(&c, p, &hs[k][i], pts));
                if let Some(z) = val {
                    comparisons += 1;
                    if z == Z {
                        zeros += 1
                    }
                    sum = f.add(sum, z);
                    vals.push(z)
                } else {
                    poles += 1
                }
            }
            assert_eq!(vals.len(), 27);
            for (i, p) in ks[k].iter().enumerate() {
                let j = ks[k].iter().position(|q| *q == c.frob(p)).unwrap();
                assert_eq!(f.frob(vals[i]), vals[j])
            }
            assert!(sum[1..].iter().all(|&z| z == 0));
            traces += 1;
            println!("TRACE triple={t} kernel={k} value={}", sum[0]);
        }
    }
    let mut norms = 0;
    let mut multiple_skips = 0;
    for n in 1..=8 {
        let d = refc.mul(&d1, n);
        let x = Div {
            u: Pol::embed(&d.u),
            v: Pol::embed(&d.v),
        };
        if x.u.degree() != 3 || !squarefree(&x.u, f) {
            multiple_skips += 1;
            continue;
        }
        assert_eq!(norm(&Pol::one(), &x.u, f), O);
        assert_eq!(norm(&x.u, &x.u, f), Z);
        assert_eq!(norm(&Pol::new(vec![base(7)]), &x.u, f), f.pow(base(7), 3));
        for k in 0..2 {
            for h in &hs[k] {
                let hv = h.a.add(&h.b.mul(&x.v, f), f);
                assert_eq!(norm(&hv, &x.u, f), resultant(&x.u, &hv.rem(&x.u, f), f));
                norms += 1
            }
        }
    }
    println!("TERMINAL nonzero_functions={nonzero} corruption_rejections={corrupt} frobenius_functions={frobs} direct_comparisons={comparisons} zero_values={zeros} poles={poles} base_traces={traces} norm_comparisons={norms} multiple_skips={multiple_skips} public_D1_rejected=true pass=true");
}
