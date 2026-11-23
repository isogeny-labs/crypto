//! Prime31 signed-y^7 affine relation chart: 25 variables, 27 equations, degree <= 7.
//! Public-only polynomial generation; the sample assignment is used for validation
//! AFTER generating the system, not for selecting equations or solving indices.
//! Uses the original native Rust f4_fp::Poly/F4 engine; no cloned solver.
//! `--bounded-f4` optionally runs a degree-three INPUT ADMISSION probe, not
//! a complete root extraction and not an UNSAT decision.
use crypto_lib::cryptanalysis::f4_fp::{f4, F4Options, Ordering, Poly};
use std::collections::BTreeMap;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const N: usize = 25;
type Expr = BTreeMap<Vec<u32>, u64>;

fn c(k: u64) -> Expr {
    let mut r = Expr::new();
    let k = k % P;
    if k != 0 { r.insert(vec![0; N], k); }
    r
}
fn v(i: usize) -> Expr {
    assert!(i < N);
    let mut m = vec![0; N];
    m[i] = 1;
    let mut r = Expr::new();
    r.insert(m, 1);
    r
}
fn add(a: &Expr, b: &Expr) -> Expr {
    let mut r = a.clone();
    for (m, k) in b {
        let t = (r.get(m).copied().unwrap_or(0) + *k) % P;
        if t == 0 { r.remove(m); } else { r.insert(m.clone(), t); }
    }
    r
}
fn sub(a: &Expr, b: &Expr) -> Expr {
    let neg: Expr = b.iter().map(|(m,k)| (m.clone(), (P-*k)%P)).collect();
    add(a, &neg)
}
fn mul(a: &Expr, b: &Expr) -> Expr {
    let mut r: Expr = Expr::new();
    for (m,k) in a {
        for (n,l) in b {
            let e: Vec<u32> = m.iter().zip(n).map(|(x,y)| *x+*y).collect();
            let t = ((*k as u128 * *l as u128) % P as u128) as u64;
            let u = (r.get(&e).copied().unwrap_or(0)+t)%P;
            if u == 0 { r.remove(&e); } else { r.insert(e,u); }
        }
    }
    r
}
fn power(x: &Expr, mut k: u32) -> Expr {
    let mut z = c(1);
    let mut b = x.clone();
    while k > 0 {
        if k & 1 == 1 { z = mul(&z, &b); }
        k >>= 1;
        if k > 0 { b = mul(&b, &b); }
    }
    z
}
fn poly(e: Expr) -> Poly {
    let mut t: Poly = e.into_iter().collect();
    t.sort_by(|(x,_),(y,_)| {
        let dx: u32 = x.iter().sum();
        let dy: u32 = y.iter().sum();
        dy.cmp(&dx).then_with(|| {
            for (a,b) in x.iter().zip(y).rev() {
                if a != b { return a.cmp(b); }
            }
            std::cmp::Ordering::Equal
        })
    });
    t
}

/// Input is ONLY public Q and the seven signed coset constants.
/// Variables: (x0,y0,...,x6,y6), five (X,Y) partial sums, inverse guard.
fn public_system(q: [u64; 2], cs: [u64; 7]) -> Vec<Poly> {
    assert!(q.iter().all(|&x| x < P));
    let leaves: Vec<(Expr,Expr)> = (0..7).map(|i| (v(2*i), v(2*i+1))).collect();
    let mut eq = Vec::<Expr>::with_capacity(27);
    for (i,(x,y)) in leaves.iter().enumerate() {
        eq.push(sub(&power(y,7),&c(cs[i])));
        let f = add(&add(&power(x,3), &mul(&c(A),x)),&c(B));
        eq.push(sub(&power(y,2),&f));
    }
    let mut prev = leaves[0].clone();
    let mut product = c(1);
    for i in 0..6 {
        let (x,y) = &leaves[i+1];
        let (ux,uy) = &prev;
        let next = if i<5 { (v(14+2*i),v(15+2*i)) } else { (c(q[0]),c(q[1])) };
        let (rx,ry) = &next;
        let d = sub(x,ux);
        let e = sub(y,uy);
        // Exact slope-free oriented affine addition when d != 0:
        // d^2*(rx+ux+x)=e^2 and d*(ry+uy)=e*(ux-rx).
        eq.push(sub(&mul(&power(&d,2),&add(&add(rx,ux),x)),&power(&e,2)));
        eq.push(sub(&mul(&d,&add(ry,uy)),&mul(&e,&sub(ux,rx))));
        product = mul(&product,&d);
        prev=next;
    }
    eq.push(sub(&mul(&v(24),&product),&c(1))); // all denominators != 0
    assert_eq!(eq.len(),27);
    eq.into_iter().map(poly).collect()
}
fn fpow(mut x:u64, mut k:u32) -> u64 {
    let mut z=1u64;
    while k>0 {
        if k&1==1 { z=((z as u128*x as u128)%P as u128) as u64; }
        k>>=1;
        if k>0 { x=((x as u128*x as u128)%P as u128) as u64; }
    }
    z
}
fn eval(p:&Poly, assignment:&[u64;N]) -> u64 {
    p.iter().fold(0u64,|sum,(m,k)| {
        let mut t=*k;
        for (e,x) in m.iter().zip(assignment) {
            if *e!=0 { t=((t as u128*fpow(*x,*e) as u128)%P as u128) as u64; }
        }
        (sum+t)%P
    })
}
fn main() {
    // Frozen witness from archived hash-to-curve target no. 0.
    // Used only AFTER public_system; never given to F4 as a solve hint.
    let q = [1_179_293_847,345_895_309];
    let cs=[1_970_273_222,1_011_118_577,1_902_095_001,131_394_216,
            49_444_078,53_346_144,1_852_955_275];
    let witness:[u64;N]=[
        398_541_372,1_813_367_127,184_585_492,588_543_802,334_707_334,
        839_007_678,398_930_746,134_306_062,800_481_093,584_390_566,
        551_015_177,1_310_942_842,1_978_960_489,1_906_827_741,
        1_689_051_411,146_740_070,150_980_755,1_639_314_540,
        627_662_512,782_283_839,1_322_109_624,2_130_242_652,
        1_673_242_684,576_867_318,1_222_006_670];
    let system=public_system(q,cs);
    assert_eq!(system.len(),27);
    assert_eq!(system.iter().map(Vec::len).sum::<usize>(),203);
    for (i,e) in system.iter().enumerate() {
        assert_eq!(eval(e,&witness),0,"equation {i} invalid");
    }
    let wrong=public_system([(q[0]+1)%P,q[1]],cs);
    assert!(wrong.iter().any(|e|eval(e,&witness)!=0));
    let max_degree=system.iter().flat_map(|e|e.iter()).map(|(m,_)|m.iter().sum::<u32>()).max().unwrap();
    assert_eq!(max_degree,7);
    println!("SIGNED25_INPUT_PASS: 25 vars, 27 eqs, degree 7, 203 terms; one certificate + target poisoning");
    if std::env::args().any(|s|s=="--bounded-f4") {
        // Fixed low degree: no root/UNSAT can be concluded.
        let opts=F4Options::new(Ordering::Grevlex,3);
        let r=f4(&system,N,P,&opts);
        println!("F4_DEGREE3_ONLY: steps={} max_rows={} max_cols={} field_ops={} ms={}",
            r.steps,r.max_rows,r.max_cols,r.field_ops,r.ms);
    }
}
