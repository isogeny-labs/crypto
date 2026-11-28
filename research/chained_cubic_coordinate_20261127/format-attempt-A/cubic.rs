use crate::divisor_a::{self, Div, H};
use crate::elliptic_a::{self, EP};
use crate::field49_a::{Quadratic, K};
use crate::field_d::{add, inv, mul, sub};
use crate::poly_d::{addp, bezout, derivative, divrem, evaluate, pow, scale, subp, times, P};
use crate::rational::{exact, PointFn, Rat};

pub const F: [u8; 4] = [1, 3, 1, 1];
pub const G3: [u8; 4] = [1, 4, 1, 6];
pub const N: [u8; 4] = [3, 1, 5, 1];
pub const R: [u8; 4] = [2, 1, 0, 2];

#[derive(Clone, Copy, Debug)]
pub struct CurveSpec {
    pub p: u8,
    pub a: u8,
    pub b: u8,
    pub order: usize,
}
pub const CURVE: CurveSpec = CurveSpec {
    p: 7,
    a: 2,
    b: 6,
    order: 11,
};
pub fn curve_identity(n: &[u8], r: &[u8]) -> P {
    subp(
        &addp(
            &addp(&pow(n, 3), &scale(&times(n, &pow(&G3, 2)), 2)),
            &scale(&pow(&G3, 3), 6),
        ),
        &times(&F, &pow(r, 2)),
    )
pub fn validate_inputs(e: &CurveSpec, g: EP, q: EP) {
    assert_eq!((e.p, e.a, e.b, e.order), (7, 2, 6, 11));
    assert!(crate::rho::ec_valid(g) && crate::rho::ec_valid(q));
    assert!(g.is_some());
    assert_eq!(crate::rho::ec_mul(g, e.order), None);
    assert_ne!(add(mul(4, mul(mul(e.a, e.a), e.a)), mul(6, mul(e.b, e.b))), 0);
}
pub fn setup() {
    assert_eq!(times(&F, &G3), H);
    assert!(curve_identity(&N, &R).is_empty());
    for (a, b) in [(&N[..], &G3[..]), (&R[..], &G3[..]), (&F[..], &G3[..])] {
        assert_eq!(bezout(a, b).0, vec![1]);
    }
}
pub fn fibre(p: EP) -> Div {
    let Some((x, y)) = p else {
        return Div {
            u: scale(&G3, inv(6)),
            v: vec![],
        };
    };
    assert_ne!(sub(1, mul(6, x)), 0, "infinity fibre exception");
    let u0 = subp(&N, &scale(&G3, x));
    let u = scale(&u0, inv(*u0.last().unwrap()));
    let (gcd, inverse_r, _) = bezout(&R, &u);
    assert_eq!(gcd, vec![1]);
    let v = divrem(&scale(&times(&pow(&G3, 2), &inverse_r), y), &u).1;
    assert_eq!(u.len(), 4);
    assert!(divrem(&subp(&H, &pow(&v, 2)), &u).1.is_empty());
    Div { u, v }
}

// Compose the two effective cubic fibres. Their common infinity
// counterweight is 3*Dinf; every reduction removes equal weights.
pub fn balanced_pair(a: &Div, b: &Div) -> Div {
    for p in [a, b] {
        assert_eq!(p.u.len(), 4);
        assert_eq!(p.u.last(), Some(&1));
        assert!(p.v.len() < p.u.len());
        assert!(divrem(&subp(&H, &pow(&p.v, 2)), &p.u).1.is_empty());
    }
    let (d1, e1, e2) = bezout(&a.u, &b.u);
    let (d, c1, c2) = bezout(&d1, &addp(&a.v, &b.v));
    let (s1, s2, s3) = (times(&c1, &e1), times(&c1, &e2), c2);
    let mut u = exact(&times(&a.u, &b.u), &pow(&d, 2));
    let numerator = addp(
        &addp(
            &times(&times(&s1, &a.u), &b.v),
            &times(&times(&s2, &b.u), &a.v),
        ),
        &times(&s3, &addp(&times(&a.v, &b.v), &H)),
    );
    let mut v = divrem(&exact(&numerator, &d), &u).1;
    assert!([1, 3, 5, 7].contains(&u.len()));
    while u.len() > 3 {
        let next = exact(&subp(&H, &pow(&v, 2)), &u);
        assert!(!next.is_empty() && next.len() < u.len());
        assert!([1, 3, 5].contains(&next.len()));
        u = scale(&next, inv(*next.last().unwrap()));
        v = divrem(&scale(&v, 6), &u).1;
    }
    let result = Div { u, v };
    assert!(divisor_a::valid(&result));
    result
}
pub fn pull(p: EP) -> Div {
    if p.is_none() {
        return divisor_a::identity();
    }
    balanced_pair(&fibre(p), &fibre(None))
}

// The scalar witness is confined to main.rs. This API receives coordinates
// only, including an arbitrary public generator and target of this curve.
pub fn eval_point(e: &CurveSpec, g: EP, q: EP) -> (Div, Div) {
    validate_inputs(e, g, q);
    setup();
    (pull(g), pull(q))
}

fn eval_k(q: Quadratic, p: &[u8], x: K) -> K {
    p.iter()
        .rev()
        .fold(q.base(0), |a, &c| q.add(q.mul(a, x), q.base(c)))
}
pub fn push(a: &Div) -> EP {
    assert!(divisor_a::valid(a));
    if a == &divisor_a::identity() {
        return None;
    }
    let packet = divisor_a::push(a);
    let q = packet.field;
    let mut result = None;
    for x in packet.roots {
        let y = eval_k(q, &a.v, x);
        let den = eval_k(q, &G3, x);
        assert_ne!(den, q.base(0), "irreducible cubic has no degree <=2 root");
        let dx = q.inv(den);
        let point = Some((
            q.mul(eval_k(q, &N, x), dx),
            q.mul(q.mul(y, eval_k(q, &R, x)), q.mul(dx, dx)),
        ));
        assert!(elliptic_a::valid(q, point));
        result = elliptic_a::add(q, result, point);
    }
    elliptic_a::descend(result)
}

pub fn global_certificate() {
    setup();
    for (name, a, b) in [
        ("N,G", &N[..], &G3[..]),
        ("R,G", &R[..], &G3[..]),
        ("F,G", &F[..], &G3[..]),
    ] {
        let (g, s, t) = bezout(a, b);
        assert_eq!(g, vec![1]);
        println!("FULL_BEZOUT\t{name}\tg={g:?}\ts={s:?}\tt={t:?}");
    }
    for (name, p) in [("F", &F), ("G", &G3)] {
        let values: Vec<_> = (0..7).map(|x| evaluate(p, x)).collect();
        assert!(values.iter().all(|&x| x != 0));
        println!("IRREDUCIBLE_CUBIC\t{name}\tall7-values={values:?}");
    }
    let differential = subp(&times(&derivative(&N), &G3), &times(&N, &derivative(&G3)));
    assert_eq!(differential, times(&[5, 3], &R));
    println!("FULL_COVER_IDENTITY\tH=F*G\tN^3+2NG^2+6G^3-FR^2={:?}\tNprimeG-NGprime={differential:?}",curve_identity(&N,&R));
    let p1 = PointFn {
        x: Rat::polynomial(&[5, 0, 3]),
        r: Rat::constant(1),
    };
    let p2 = PointFn {
        x: Rat::new(vec![1], vec![0, 0, 1]),
        r: Rat::new(vec![1], vec![0, 0, 0, 1]),
    };
    let h3 = PointFn {
        x: Rat::new(N.to_vec(), G3.to_vec()),
        r: Rat::new(R.to_vec(), pow(&G3, 2)),
    };
    assert!(p1.valid() && p2.valid() && h3.valid());
    let doubled = h3.doubled();
    let twice_p2 = p2.doubled();
    let genuine = p1.added(&p2.frobenius().added(&twice_p2));
    assert_eq!(doubled, genuine);
    println!("GENERIC_FULL_FUNCTION_FIELD\t[2]H3=p1+(pi+2)p2\tx={:?}\ty-factor={:?}",doubled.x,doubled.r);
    let wrong = p1.added(&p2.added(&twice_p2));
    let dx = doubled.x.sub(&wrong.x);
    let dy = doubled.r.sub(&wrong.r);
    assert!(!dx.n.is_empty() || !dy.n.is_empty());
    println!("GENERIC_FROBENIUS_MUTANT\tpi-to-identity\tnonzero-x-difference={dx:?}\tnonzero-y-factor-difference={dy:?}");
    let mut finite = 0;
    for w in 0..7 {
        for y in 0..7 {
            if mul(y, y) != evaluate(&H, w) {
                continue;
            }
            let g = evaluate(&G3, w);
            assert_ne!(g, 0);
            let h = Some((mul(evaluate(&N, w), inv(g)), mul(mul(y, evaluate(&R, w)), inv(mul(g, g)))));
            let first = Some((add(mul(3, mul(w, w)), 5), y));
            let second = if w == 0 { None } else { Some((inv(mul(w,w)),mul(y,inv(mul(mul(w,w),w))))) };
            let wrong_finite = elliptic_a::add_base(first, elliptic_a::mul_base(second, 3));
            assert_eq!(elliptic_a::mul_base(h, 2), wrong_finite);
            assert!(crate::rho::ec_valid(h));
            finite += 1;
            println!("ALL_FINITE_POINTS\tW={w}\tY={y}\tH3={h:?}\twrong-generic-map-agrees-over-F7=true");
        }
    }
    assert_eq!(finite, 14);
    for (name, n, r) in [
        ("N0+1", { let mut n=N; n[0]=add(n[0],1); n }, R),
        ("R0+1", N, { let mut r=R; r[0]=add(r[0],1); r }),
    ] {
        let residual = curve_identity(&n,&r);
        assert!(!residual.is_empty());
        println!("COEFFICIENT_MUTANT\t{name}\tfull-residual={residual:?}\tpristine-restored=true");
    }
    assert!(curve_identity(&N,&R).is_empty());
    assert_eq!(subp(&N,&scale(&G3,3)),scale(&[0,6,4,1],4));
    let u0=vec![0,6,4,1];
    let v0=vec![1,2,5];
    assert!(divrem(&subp(&H,&pow(&v0,2)),&u0).1.is_empty());
    assert_eq!(divrem(&subp(&times(&v0,&R),&scale(&pow(&G3,2),2)),&u0).1,vec![]);
    assert_eq!(fibre(Some((3,2))),Div{u:u0,v:v0});
    let q49=Quadratic{u0:1,u1:0};
    q49.validate();
    let eta=K([0,1]);
    assert_eq!(q49.mul(eta,eta),q49.base(6));
    let infinities=[Some((q49.base(6),q49.scale(eta,2))),Some((q49.base(6),q49.scale(eta,5)))];
    assert!(infinities.iter().all(|&p|elliptic_a::valid(q49,p)));
    assert_eq!(elliptic_a::add(q49,infinities[0],infinities[1]),None);
    println!("PROPER_FIBRES\tG-Weierstrass-poles=x:2,y:3,tE:1\tinfinity-images={infinities:?}\tinfinity-trace=O\tnormalized-origin-fibre-u={:?}\tv={:?}\tramification-W=3,Y=+-1",fibre(Some((3,2))).u,fibre(Some((3,2))).v);
}
