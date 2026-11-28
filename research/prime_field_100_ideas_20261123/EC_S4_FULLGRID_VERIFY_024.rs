// Independent exhaustive S4/grid replay for frozen public E/F_1019, #24.
// Uses a direct quadratic resultant identity and separate affine point arithmetic.
const MOD: i64 = 1019;
const ORDER: i64 = 1033;
const BX: [i64; 16] = [7, 8, 10, 13, 16, 18, 19, 20, 22, 23, 24, 28, 29, 31, 35, 36];
const EXPECT_ZERO: [usize; 11] = [12, 42, 48, 57, 36, 79, 30, 27, 0, 12, 21];
const EXPECT_WITNESS: [usize; 11] = [12, 42, 48, 57, 36, 126, 30, 27, 0, 12, 21];
const EXPECT_X: [i64; 11] = [0, 777, 622, 881, 748, 20, 577, 112, 56, 200, 38];

fn m(x: i64) -> i64 { x.rem_euclid(MOD) }
fn pow(mut a: i64, mut e: i64) -> i64 {
    let mut z = 1;
    while e > 0 { if e & 1 == 1 { z = m(z * a); } a = m(a * a); e >>= 1; }
    z
}
fn inv(a: i64) -> i64 { assert_ne!(m(a), 0); pow(m(a), MOD - 2) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P { x: i64, y: i64 }
type Point = Option<P>;
fn neg(a: Point) -> Point { a.map(|p| P { x: p.x, y: m(-p.y) }) }
fn add(a: Point, b: Point) -> Point {
    match (a, b) {
        (None, t) | (t, None) => t,
        (Some(u), Some(v)) => {
            if u.x == v.x && m(u.y + v.y) == 0 { return None; }
            let slope = if u == v { m((3*u.x*u.x + 1) * inv(2*u.y)) }
                        else { m((v.y-u.y) * inv(v.x-u.x)) };
            let x = m(slope*slope-u.x-v.x);
            Some(P { x, y: m(slope*(u.x-x)-u.y) })
        }
    }
}
fn mul(mut p: Point, mut k: i64) -> Point {
    let mut out = None;
    while k > 0 { if k & 1 == 1 { out = add(out, p); } p = add(p, p); k >>= 1; }
    out
}
fn on_curve(p: P) -> bool { m(p.y*p.y) == m(p.x*p.x*p.x + p.x + 20) }
fn lift_x(x: i64) -> P {
    let f = m(x*x*x + x + 20);
    let y = (0..MOD).find(|&v| m(v*v) == f).expect("base x has no point");
    let p = P { x, y }; assert!(on_curve(p)); p
}

// S3(x,y,t) = A t^2 + B t + C for y^2=x^3+x+20.
fn first_coeffs(x: i64, y: i64) -> (i64,i64,i64) {
    let a = m((x-y)*(x-y));
    let b = m(-2*((x+y)*(x*y+1)+40));
    let c = m((x*y-1)*(x*y-1)-80*(x+y));
    (a,b,c)
}
// S3(t,w,z) = D t^2 + E t + F.
fn second_coeffs(w: i64, z: i64) -> (i64,i64,i64) {
    let d = m((z-w)*(z-w));
    let e = m(-2*w*z*z - 2*(w*w+1)*z - 2*w - 80);
    let f = m(w*w*z*z - 2*(w+40)*z + 1 - 80*w);
    (d,e,f)
}
fn s4_zero(x: i64, y: i64, w: i64, z: i64) -> bool {
    let (a,b,c) = first_coeffs(x,y);
    let (d,e,f) = second_coeffs(w,z);
    // Resultant of two quadratics: (AF-CD)^2 - (AE-BD)(BF-CE).
    m(m(a*f-c*d).pow(2) - m(a*e-b*d)*m(b*f-c*e)) == 0
}

fn main() {
    let g = Some(P { x: 0, y: 64 });
    let q = Some(P { x: 38, y: 245 });
    assert!(on_curve(g.unwrap()) && on_curve(q.unwrap()));
    assert_eq!(mul(g, ORDER), None); assert_eq!(mul(q, ORDER), None);
    let base: Vec<Point> = BX.iter().map(|&x| Some(lift_x(x))).collect();
    let mut total_zero = 0usize;
    let mut total_witness = 0usize;
    let mut total_cells = 0usize;
    let mut total_patterns = 0usize;
    let mut total_adds = 0usize;
    for target in 0..11 {
        let (a,b) = if target < 10 { (target as i64 + 1, 0) } else { (0,1) };
        let r = add(mul(g,a),mul(q,b)).expect("target infinity");
        assert_eq!(r.x, EXPECT_X[target]);
        let mut zeros = 0usize;
        let mut witnesses = 0usize;
        for i in 0..16 { for j in 0..16 { for k in 0..16 {
            let zero = s4_zero(BX[i],BX[j],BX[k],r.x);
            let mut found = 0usize;
            for mask in 0..8 {
                let p1 = if mask & 1 == 0 { base[i] } else { neg(base[i]) };
                let p2 = if mask & 2 == 0 { base[j] } else { neg(base[j]) };
                let p3 = if mask & 4 == 0 { base[k] } else { neg(base[k]) };
                if add(add(p1,p2),p3) == Some(r) { found += 1; }
                total_patterns += 1; total_adds += 2;
            }
            assert_eq!(zero, found > 0, "S4/group mismatch a={a} b={b} i={} j={} k={}",i+1,j+1,k+1);
            zeros += usize::from(zero); witnesses += found; total_cells += 1;
        } } }
        assert_eq!(zeros, EXPECT_ZERO[target], "zero count target {target}");
        assert_eq!(witnesses, EXPECT_WITNESS[target], "witness count target {target}");
        total_zero += zeros; total_witness += witnesses;
        println!("verified_target a={a} b={b} x={} zero_triples={zeros} signed_witnesses={witnesses}",r.x);
    }
    assert_eq!((total_cells,total_patterns,total_adds,total_zero,total_witness),
               (45056,360448,720896,364,411));
    println!("verify_summary targets=11 triples={total_cells} signed_patterns={total_patterns} signed_sum_adds={total_adds} S4_zeros={total_zero} signed_witnesses={total_witness} all_equivalences=true");
    println!("complete independent full-grid EC S4 replay");
}
