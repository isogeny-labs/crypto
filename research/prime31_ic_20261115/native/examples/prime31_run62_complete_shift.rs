//! Prime31 run62: exceptional-safe fixed-G mixed Jacobian and Mersenne-fold ablation.
//! Reuses ORIGINAL prime31_algebraic_ic::Arithmetic as independent affine oracle.
//! NOT a blind F4/S3 extractor and NOT a complete ECDLP benchmark.
//! Run: cargo run --release --example prime31_run62_complete_shift
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use std::hint::black_box;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const G: (u64, u64) = (625_037_715, 2_100_627_251);
const A: u64 = 847_514_846;
const ITER: usize = 250_000;
const REPS: usize = 5;

#[inline(always)]
fn addp(a: u64, b: u64) -> u64 {
    debug_assert!(a < P && b < P);
    let x = a + b;
    if x >= P { x - P } else { x }
}
#[inline(always)]
fn subp(a: u64, b: u64) -> u64 {
    debug_assert!(a < P && b < P);
    if a >= b { a - b } else { a + P - b }
}
#[inline(always)]
fn mulp(a: u64, b: u64) -> u64 {
    debug_assert!(a < P && b < P);
    let t = a * b; // canonical inputs: product < p^2 < 2^62
    let z = (t & P) + (t >> 31);
    if z >= P { z - P } else { z }
}
#[inline(always)]
fn dbl(a: u64) -> u64 { addp(a, a) }
fn power(mut x: u64, mut exponent: u64) -> u64 {
    let mut z = 1;
    while exponent != 0 {
        if exponent & 1 != 0 { z = mulp(z, x); }
        x = mulp(x, x);
        exponent >>= 1;
    }
    z
}
#[derive(Clone, Copy, Debug)]
struct Jacobian { x: u64, y: u64, z: u64 }
impl Jacobian {
    fn from_affine(q: (u64,u64)) -> Self { Self { x:q.0, y:q.1, z:1 } }
    fn double(self) -> Option<Self> {
        if self.y == 0 { return None; } // ordinary elliptic group: 2-torsion or infinity
        let xx = mulp(self.x, self.x);
        let yy = mulp(self.y, self.y);
        let yyyy = mulp(yy, yy);
        let z2 = mulp(self.z, self.z);
        let z4 = mulp(z2, z2);
        let t = mulp(addp(self.x, yy), addp(self.x, yy));
        let d = dbl(subp(subp(t, xx), yyyy));
        let m = addp(addp(xx, dbl(xx)), mulp(A, z4));
        let nx = subp(mulp(m, m), dbl(d));
        let ny = subp(mulp(m, subp(d, nx)), dbl(dbl(dbl(yyyy))));
        let nz = mulp(dbl(self.y), self.z);
        Some(Self { x:nx, y:ny, z:nz })
    }
    fn add_fixed(self, g: (u64,u64)) -> Option<Self> {
        let zz = mulp(self.z, self.z);
        let u = mulp(g.0, zz);
        let s = mulp(g.1, mulp(self.z, zz));
        let h = subp(u, self.x);
        let r = subp(s, self.y);
        if h == 0 {
            return if r == 0 { self.double() } else { None };
        }
        let hh = mulp(h,h);
        let hhh = mulp(h,hh);
        let v = mulp(self.x,hh);
        let nx = subp(subp(mulp(r,r),hhh),dbl(v));
        let ny = subp(mulp(r,subp(v,nx)),mulp(self.y,hhh));
        Some(Self { x:nx, y:ny, z:mulp(self.z,h) })
    }
    fn affine(self) -> (u64,u64) {
        assert_ne!(self.z,0);
        let iz=power(self.z,P-2);
        let iz2=mulp(iz,iz);
        (mulp(self.x,iz2),mulp(self.y,mulp(iz2,iz)))
    }
}
#[inline(always)]
fn complete_add(q: Option<Jacobian>) -> Option<Jacobian> {
    match q { None => Some(Jacobian::from_affine(G)), Some(j) => j.add_fixed(G) }
}
fn complete_affine(q: Option<Jacobian>) -> Point { q.map(Jacobian::affine) }

// Exact run61 partial formula as an ablation only, used away from H=0.
#[derive(Clone, Copy, Debug)]
struct Partial { x:u64, y:u64, z:u64 }
impl Partial {
    fn from_affine(q:(u64,u64))->Self { Self{x:q.0,y:q.1,z:1} }
    #[inline(always)]
    fn mul(a:u64,b:u64)->u64 { a*b%P }
    #[inline(always)]
    fn sub(a:u64,b:u64)->u64 { (a+P-b)%P }
    #[inline(always)]
    fn add_fixed(&mut self) {
        let zz=Self::mul(self.z,self.z);
        let u=Self::mul(G.0,zz);
        let s=Self::mul(G.1,Self::mul(self.z,zz));
        let h=Self::sub(u,self.x);
        let r=Self::sub(s,self.y);
        assert_ne!(h,0,"partial formula must never be treated as complete");
        let hh=Self::mul(h,h);
        let hhh=Self::mul(h,hh);
        let v=Self::mul(self.x,hh);
        let nx=Self::sub(Self::sub(Self::mul(r,r),hhh),(2*v)%P);
        let ny=Self::sub(Self::mul(r,Self::sub(v,nx)),Self::mul(self.y,hhh));
        self.z=Self::mul(self.z,h);
        self.x=nx;self.y=ny;
    }
    fn affine(self)->(u64,u64) {
        let iz=power(self.z,P-2);
        let iz2=mulp(iz,iz);
        (mulp(self.x,iz2),mulp(self.y,mulp(iz2,iz)))
    }
}
fn random(seed:&mut u64)->u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn test_field() {
    let b=[0,1,2,3,P-3,P-2,P-1];
    for &a in &b { for &c in &b {
        assert_eq!(addp(a,c),(a+c)%P);
        assert_eq!(subp(a,c),(a+P-c)%P);
        assert_eq!(mulp(a,c),((a as u128*c as u128)%P as u128) as u64);
    }}
    let mut seed=0x62_fe11_d0ba_cafe_u64;
    for _ in 0..120_000 {
        let a=random(&mut seed)%P;
        let b=random(&mut seed)%P;
        assert_eq!(addp(a,b),(a+b)%P);
        assert_eq!(subp(a,b),(a+P-b)%P);
        assert_eq!(mulp(a,b),((a as u128*b as u128)%P as u128) as u64);
    }
}
fn test_group(ar:&mut Arithmetic) {
    let e=PRIME31; let g=Some(G);
    assert_eq!(complete_affine(complete_add(None)),g,"O+G");
    let minus_g=ar.neg(e,g).unwrap();
    assert_eq!(complete_affine(complete_add(Some(Jacobian::from_affine(minus_g)))),None,"-G+G");
    let two=ar.add(e,g,g);
    assert_eq!(complete_affine(complete_add(Some(Jacobian::from_affine(G)))),two,"G+G");
    let last=ar.scalar(e,e.n-1,g).unwrap();
    assert_eq!(complete_affine(complete_add(Some(Jacobian::from_affine(last)))),None,"(n-1)G+G");
    let mut seed=0x62_badf_0cab_ab1e_u64;
    let fixed=[0,1,2,3,e.n-1,e.n-2,e.n/2,1_000_000];
    for ix in 0..96 {
        let k=if ix<fixed.len() {fixed[ix]} else {random(&mut seed)%e.n};
        let mut affine=ar.scalar(e,k,g);
        let mut projective=affine.map(Jacobian::from_affine);
        for step in 0..128 {
            projective=complete_add(projective);
            affine=ar.add(e,affine,g);
            assert_eq!(complete_affine(projective),affine,
                       "mismatch at k={} step={}",k,step);
        }
    }
}
fn median(mut arr:Vec<f64>)->f64 {
    arr.sort_by(|a,b|a.total_cmp(b)); arr[arr.len()/2]
}
fn main()->Result<(),Box<dyn std::error::Error>> {
    assert_eq!(PRIME31.p,P);assert_eq!(PRIME31.a,A);assert_eq!(PRIME31.g,G);
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    test_field();
    test_group(&mut ar);
    println!("run62 correctness=PASS field_pairs=120049 group_chains=96 group_steps=12288 exceptions=O,-G,G,(n-1)G");
    let mut times=[Vec::<f64>::new(),Vec::<f64>::new(),Vec::<f64>::new()];
    let mut digest=0u64;
    for rep in 0..REPS {
        let k=1000+rep as u64*400_001;
        let start=ar.scalar(PRIME31,k,Some(G)).expect("start infinity");
        let endpoint=ar.scalar(PRIME31,k+ITER as u64,Some(G));
        // Rotate run order: complete=0, previous partial=1, original affine=2.
        for offset in 0..3 {
            let arm=(rep+offset)%3;
            let clock=Instant::now();
            let finish=match arm {
                0=>{
                    let mut q=Some(Jacobian::from_affine(start));
                    for _ in 0..ITER { q=complete_add(q); if let Some(j)=q {digest^=j.x^j.y^j.z;} black_box(q); }
                    complete_affine(q)
                }
                1=>{
                    let mut q=Partial::from_affine(start);
                    for _ in 0..ITER { q.add_fixed();digest^=q.x^q.y^q.z;black_box(q); }
                    Some(q.affine())
                }
                _=>{
                    let mut q=Some(start);
                    for _ in 0..ITER {q=ar.add(PRIME31,q,Some(G));if let Some((x,y))=q {digest^=x^y;}black_box(q);}
                    q
                }
            };
            let ns=clock.elapsed().as_nanos() as f64/ITER as f64;
            assert_eq!(finish,endpoint,"arm={} rep={} endpoint",arm,rep);
            times[arm].push(ns);
            println!("run62 rep={} arm={} ns_per_public_shift={:.4}",rep,arm,ns);
        }
    }
    let new=median(times[0].clone());
    let old=median(times[1].clone());
    let affine=median(times[2].clone());
    println!("run62 summary complete_folded_ns={:.4} partial_prior_ns={:.4} affine_original_ns={:.4}               original_over_complete={:.4} partial_over_complete={:.4}               budget_36p195ns_met={} projected_1024_shift_only_ms={:.4} digest={}               classification=arithmetic_only_no_blind_index_calculus_extraction",
             new,old,affine,affine/new,old/new,new<=36.195,new*4_699_949.0/1e6,digest);
    black_box(digest);
    Ok(())
}
