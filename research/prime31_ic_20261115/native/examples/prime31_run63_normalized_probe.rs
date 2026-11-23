//! Prime31 run63: actual affine-coordinate materialization cost after complete
//! fixed-generator Jacobian shifts, reusing the original Rust curve arithmetic.
//! This is a necessary stage only for *affine-coordinate* query implementations,
//! not for every conceivable projective/homogeneous polynomial oracle.
//! No ECDLP root extraction, target solving, or generic half-table is performed.
//! cargo run --release --example prime31_run63_normalized_probe
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use std::hint::black_box;
use std::time::Instant;

const P:u64=2_147_483_647;
const A:u64=847_514_846;
const G:(u64,u64)=(625_037_715,2_100_627_251);
const ITER:usize=250_000;
const REPS:usize=5;
const BATCH:usize=256;
const OLD_BSGS_MS:f64=170.113; // different CI runner: sensitivity only
const MODEL_QUERIES:f64=4_699_949.0; // synthetic run59 model, not measured extraction

#[inline(always)] fn add(a:u64,b:u64)->u64 {let t=a+b;if t>=P{t-P}else{t}}
#[inline(always)] fn sub(a:u64,b:u64)->u64 {if a>=b{a-b}else{a+P-b}}
#[inline(always)] fn mul(a:u64,b:u64)->u64 {
    debug_assert!(a<P && b<P);
    let t=a*b;
    let s=(t&P)+(t>>31);
    if s>=P{s-P}else{s}
}
fn pow(mut x:u64,mut k:u64)->u64 {
    let mut acc=1;
    while k>0 {if k&1==1{acc=mul(acc,x)}x=mul(x,x);k>>=1;}
    acc
}
#[derive(Clone,Copy,Debug)]
struct J{x:u64,y:u64,z:u64}
impl J {
    fn affine(p:(u64,u64))->Self{Self{x:p.0,y:p.1,z:1}}
    fn double(self)->Option<Self>{
        if self.y==0{return None}
        let xx=mul(self.x,self.x);
        let yy=mul(self.y,self.y);
        let yyyy=mul(yy,yy);
        let z2=mul(self.z,self.z);
        let t=mul(add(self.x,yy),add(self.x,yy));
        let d=add(sub(sub(t,xx),yyyy),sub(sub(t,xx),yyyy));
        let m=add(add(xx,add(xx,xx)),mul(A,mul(z2,z2)));
        let nx=sub(mul(m,m),add(d,d));
        let ny=sub(mul(m,sub(d,nx)),mul(8,yyyy));
        let nz=mul(add(self.y,self.y),self.z);
        Some(Self{x:nx,y:ny,z:nz})
    }
    fn mixed(self,g:(u64,u64))->Option<Self>{
        let zz=mul(self.z,self.z);
        let u=mul(g.0,zz);
        let s=mul(g.1,mul(self.z,zz));
        let h=sub(u,self.x);
        let r=sub(s,self.y);
        if h==0{return if r==0{self.double()}else{None}}
        let hh=mul(h,h);
        let hhh=mul(h,hh);
        let v=mul(self.x,hh);
        let nx=sub(sub(mul(r,r),hhh),add(v,v));
        let ny=sub(mul(r,sub(v,nx)),mul(self.y,hhh));
        Some(Self{x:nx,y:ny,z:mul(self.z,h)})
    }
}
#[inline(always)] fn shift(q:Option<J>)->Option<J>{
    match q{None=>Some(J::affine(G)),Some(p)=>p.mixed(G)}
}
// Montgomery's trick: one inversion per chunk; identity has no affine coordinates.
// The XY and X-only arms use identical Z inversions and original public points.
fn normalize(chunk:&[Option<J>],include_y:bool)->Vec<Option<(u64,u64)>>{
    let mut prefix=Vec::with_capacity(chunk.len());
    let mut product=1u64;
    for &q in chunk{
        prefix.push(product);
        if let Some(j)=q {
            assert_ne!(j.z,0);
            product=mul(product,j.z);
        }
    }
    let mut inv=pow(product,P-2);
    let mut out=vec![None;chunk.len()];
    for i in (0..chunk.len()).rev(){
        if let Some(j)=chunk[i]{
            let zi=mul(prefix[i],inv);
            inv=mul(inv,j.z);
            let zi2=mul(zi,zi);
            let x=mul(j.x,zi2);
            let y=if include_y{mul(j.y,mul(zi2,zi))}else{0};
            out[i]=Some((x,y));
        }
    }
    assert_eq!(inv,1);
    out
}
fn verify(ar:&mut Arithmetic){
    let g=Some(G);
    let boundary=[0,1,2,3,PRIME31.n-1,PRIME31.n-2,PRIME31.n/2,1000];
    let mut seed=0x63_2026_1122_1234u64;
    for case in 0..40{
        let k=if case<boundary.len(){boundary[case]}else{
            seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed%PRIME31.n
        };
        let mut q=ar.scalar(PRIME31,k,g);
        let mut projective=q.map(J::affine);
        let mut buffer=Vec::new();
        let mut reference=Vec::new();
        for step in 0..192 {
            q=ar.add(PRIME31,q,g);
            projective=shift(projective);
            buffer.push(projective);
            reference.push(q);
            if buffer.len()==BATCH || step==191{
                let xy=normalize(&buffer,true);
                let xonly=normalize(&buffer,false);
                for (i,pt) in reference.iter().enumerate(){
                    assert_eq!(xy[i],*pt,"XY mismatch case={case} step={step} i={i}");
                    assert_eq!(xonly[i].map(|(x,_)|x),pt.map(|(x,_)|x),
                               "X mismatch case={case} step={step} i={i}");
                }
                buffer.clear();
                reference.clear();
            }
        }
    }
    println!("run63 independent-original-Rust-reference=PASS cases=40 steps=7680 exceptional=O,G,-G,order-wrap");
}
fn median(v:&[f64])->f64{
    let mut x=v.to_vec();x.sort_by(|a,b|a.total_cmp(b));x[x.len()/2]
}
fn main()->Result<(),Box<dyn std::error::Error>>{
    assert_eq!(PRIME31.p,P); assert_eq!(PRIME31.a,A); assert_eq!(PRIME31.g,G);
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    verify(&mut ar);
    let mut times:[Vec<f64>;3]=std::array::from_fn(|_|Vec::new());
    let mut digest=0u64;
    for rep in 0..REPS{
        let k=1000+rep as u64*400_001;
        let initial=ar.scalar(PRIME31,k,Some(G)).unwrap();
        let expected=ar.scalar(PRIME31,k+ITER as u64,Some(G));
        for offset in 0..3{
            let arm=(rep+offset)%3; // rotate order
            let mut q=Some(J::affine(initial));
            let began=Instant::now();
            if arm==0{
                for _ in 0..ITER{
                    q=shift(q);
                    if let Some(j)=q{digest^=j.x^j.y^j.z;}
                    black_box(q);
                }
            }else{
                let mut done=0usize;
                while done<ITER{
                    let chunk_size=BATCH.min(ITER-done);
                    let mut chunk=Vec::with_capacity(chunk_size);
                    for _ in 0..chunk_size{
                        q=shift(q);
                        chunk.push(q);
                        black_box(q);
                    }
                    for point in normalize(&chunk,arm==2) {
                        if let Some((x,y))=point{digest^=x^y;}
                        black_box(point);
                    }
                    done+=chunk_size;
                }
            }
            let ns=began.elapsed().as_nanos() as f64/ITER as f64;
            // Verify whole-chain endpoint against the original repository arithmetic.
            let endpoint=normalize(&[q],true)[0];
            assert_eq!(endpoint,expected,"endpoint arm={arm} rep={rep}");
            times[arm].push(ns);
            println!("run63 rep={rep} arm={arm} ns_per_probe={ns:.4} endpoint=PASS");
        }
    }
    let shift_ns=median(&times[0]);
    let x_ns=median(&times[1]);
    let xy_ns=median(&times[2]);
    let budget=OLD_BSGS_MS*1e6/MODEL_QUERIES;
    println!("run63 summary shift_ns={shift_ns:.4} x256_ns={x_ns:.4} xy256_ns={xy_ns:.4} sensitivity_ns={budget:.4} x_floor_1024_ms={:.4} xy_floor_1024_ms={:.4} x_under_sensitivity={} xy_under_sensitivity={} digest={} classification=affine_probe_component_not_index_calculus_or_complete_ECDLP",
        x_ns*MODEL_QUERIES/1e6,xy_ns*MODEL_QUERIES/1e6,x_ns<budget,xy_ns<budget,digest);
    black_box(digest);
    Ok(())
}
