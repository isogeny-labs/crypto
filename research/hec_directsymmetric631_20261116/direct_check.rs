include!("../hec_inverse631_20261116/verify_poly.rs");
use std::{collections::BTreeMap, fs};
#[derive(Debug)]
enum V {
    N(i64),
    S(String),
    L(Vec<V>),
}
impl V {
    fn list(&self) -> &[V] {
        if let Self::L(a) = self {
            a
        } else {
            panic!("expected list")
        }
    }
    fn num(&self) -> i64 {
        if let Self::N(a) = self {
            *a
        } else {
            panic!("expected integer")
        }
    }
    fn string(&self) -> &str {
        if let Self::S(a) = self {
            a
        } else {
            panic!("expected string")
        }
    }
}
fn parse(b: &[u8], i: &mut usize) -> V {
    while *i < b.len() && (b[*i].is_ascii_whitespace() || b[*i] == b',') {
        *i += 1
    }
    match b[*i] {
        b'[' => {
            *i += 1;
            let mut a = Vec::new();
            loop {
                while b[*i].is_ascii_whitespace() || b[*i] == b',' {
                    *i += 1
                }
                if b[*i] == b']' {
                    *i += 1;
                    return V::L(a);
                }
                a.push(parse(b, i));
            }
        }
        b'\'' => {
            *i += 1;
            let s = *i;
            while b[*i] != b'\'' {
                *i += 1
            }
            let a = String::from_utf8(b[s..*i].to_vec()).unwrap();
            *i += 1;
            V::S(a)
        }
        _ => {
            let s = *i;
            if b[*i] == b'-' {
                *i += 1
            }
            while *i < b.len() && b[*i].is_ascii_digit() {
                *i += 1
            }
            assert!(*i > s);
            V::N(std::str::from_utf8(&b[s..*i]).unwrap().parse().unwrap())
        }
    }
}
fn poly(v: &V) -> U {
    let mut v = v;
    while v.list().len() == 1 {
        v = &v.list()[0]
    }
    let a = v.list();
    let degree = a[0].num();
    let c: U = a[1]
        .list()
        .iter()
        .map(|n| n.num().rem_euclid(631) as u64)
        .collect();
    assert!(degree < 0 && c.is_empty() || degree as usize + 1 == c.len());
    trim(c)
}
#[derive(Clone)]
struct Term {
    c: u64,
    e: [u8; 8],
}
fn equations(path: &str) -> Vec<Vec<Term>> {
    let s = fs::read_to_string(path).unwrap();
    let mut ls = s.lines();
    assert_eq!(ls.next().unwrap(), "w,u0,u1,u2,v0,v1,v2,t");
    assert_eq!(ls.next().unwrap(), "631");
    let names = ["w", "u0", "u1", "u2", "v0", "v1", "v2", "t"];
    ls.collect::<Vec<_>>()
        .join("")
        .split(',')
        .map(|p| {
            p.trim()
                .split('+')
                .filter(|t| !t.is_empty())
                .map(|t| {
                    let mut z = t.split('*');
                    let c = z.next().unwrap().parse().unwrap();
                    let mut e = [0; 8];
                    for f in z {
                        let mut f = f.split('^');
                        let n = f.next().unwrap();
                        let i = names.iter().position(|x| *x == n).unwrap();
                        e[i] = f.next().map(|n| n.parse().unwrap()).unwrap_or(1);
                    }
                    Term { c, e }
                })
                .collect()
        })
        .collect()
}
fn residual(ps: &[Vec<Term>], xs: &[U], m: &[u64]) -> bool {
    ps.iter().all(|p| {
        let mut z = vec![];
        for t in p {
            let mut v = vec![t.c];
            for i in 0..8 {
                v = mm(&v, &mpow(xs[i].clone(), t.e[i] as u64, m), m);
            }
            z = rem(&add(&z, &v, false), m);
        }
        z.is_empty()
    })
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(
        a.len(),
        4,
        "verifier input.ms output.res control-point-or-dash"
    );
    let b = fs::read(&a[2]).unwrap();
    let root = parse(&b, &mut 0);
    let root = root.list();
    assert_eq!(root[0].num(), 0, "positive dimensional or failed solve");
    let v = root[1].list();
    assert_eq!(v[0].num(), 631);
    assert_eq!(v[1].num(), 8);
    let degree = v[2].num();
    assert!(
        degree == 6 || degree == 12,
        "unexpected fiber degree {degree}"
    );
    let names = ["w", "u0", "u1", "u2", "v0", "v1", "v2", "t"];
    assert_eq!(v[3].list().iter().map(V::string).collect::<Vec<_>>(), names);
    let params = v[5].list();
    assert_eq!(params[0].num(), 1);
    let p = params[1].list();
    let elim = poly(&p[0]);
    assert_eq!(elim.len(), degree as usize + 1);
    assert_eq!(gcd(elim.clone(), deriv(&elim)).len(), 1);
    let den = poly(&p[1]);
    let inv = inverse(&den, &elim);
    let cs = p[2].list();
    assert_eq!(cs.len(), 7);
    let nums: Vec<_> = cs.iter().map(poly).collect();
    let ps = equations(&a[1]);
    assert_eq!(ps.len(), 12);
    let mut valid = Vec::new();
    for sign in [1, 630] {
        let mut xs: Vec<_> = nums
            .iter()
            .map(|n| mm(&scale(n, sign), &inv, &elim))
            .collect();
        xs.push(rem(&[0, 1], &elim));
        if residual(&ps, &xs, &elim) {
            valid.push((sign, xs));
        }
    }
    assert_eq!(valid.len(), 1, "ambiguous or invalid parametrization");
    let (sign, xs) = valid.pop().unwrap();
    if a[3] != "-" {
        let cp: Vec<u64> = fs::read_to_string(&a[3])
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        assert_eq!(cp.len(), 8);
        assert_eq!(eval(&elim, cp[7]), 0);
        for i in 0..8 {
            assert_eq!(eval(&xs[i], cp[7]), cp[i]);
        }
        println!("KNOWN_CONTROL recovered=true");
    }
    println!("PARAMETER_VERIFY dimension=0 degree={degree} equations=12 exact_modular_residuals=true convention={sign}");
    println!("ELIM {elim:?}");
    for (i, x) in xs.iter().enumerate() {
        println!("COORD {i} {x:?}");
    }
    let mut data = BTreeMap::new();
    data.insert("elimination", elim);
    for (i, x) in xs.into_iter().enumerate() {
        let key = Box::leak(format!("coord{i}").into_boxed_str());
        data.insert(key, x);
    }
    fs::write(format!("{}.verified", a[2]), format!("{data:?}\n")).unwrap();
}
pub fn check(path:&str,uv:Vec<U>,e:U){let ps=equations(path);assert_eq!(ps.len(),12);let mut x=vec![vec![1]];x.extend(uv);x.push(x[1].clone());let eval=|p:&Vec<Term>,x:&[U]|{let mut z=vec![];for t in p{let mut a=vec![t.c];for i in 0..8{a=mm(&a,&mpow(x[i].clone(),t.e[i]as u64,&e),&e);}z=rem(&add(&z,&a,false),&e);}z};let h0=add(&eval(&ps[10],&x),&[1],false);x[0]=inverse(&h0,&e);assert!(residual(&ps,&x,&e),"direct_full_algebra_residual");let mut bad=x.clone();bad[4]=add(&bad[4],&[1],false);assert!(!residual(&ps,&bad,&e));println!("PASS direct_symmetric_equations=12 full_ordered_algebra_degree={} saturation_invertible=true corrupt_interpolation_rejected=true direct_solver_not_executed=true",e.len()-1);}
