// r2 microbench: affine prime add chain, current primitives vs Barrett mulmod.
use std::time::Instant;
include!("euclid.rs");
#[inline] fn mulmod_div(a:u64,b:u64,m:u64)->u64{ let prod=a*b; let rem:u32; unsafe{ std::arch::asm!("div {m:e}", m=in(reg) m as u32, inout("eax") prod as u32 => _, inout("edx") (prod>>32) as u32 => rem, options(pure,nomem,nostack)); } rem as u64 }
#[derive(Clone,Copy)] struct Bar{m:u64, mu:u64}
impl Bar{ fn new(m:u64)->Self{ Bar{m, mu: (u64::MAX / m)} }
 #[inline] fn mul(&self,a:u64,b:u64)->u64{ let p=a*b; let q=((p as u128 * self.mu as u128)>>64) as u64; let r=p.wrapping_sub(q.wrapping_mul(self.m)); let (d,bo)=r.overflowing_sub(self.m); std::hint::select_unpredictable(bo, r, d) } }
#[inline] fn inv(a:u64,m:u64)->u64{ let (g,t,neg)=euclid_u32_x86(m as u32,a as u32); assert!(g==1); (if neg {(m as u32 - t)% m as u32} else {t % m as u32}) as u64 }
#[inline] fn sub(a:u64,b:u64,m:u64)->u64{ if a>=b {a-b} else {m-(b-a)} }
#[inline(never)] fn walk<F:Fn(u64,u64)->u64>(mul:F, m:u64, mut x:u64, mut y:u64, jx:&[u64], jy:&[u64], n:usize)->(u64,u64){
  for _ in 0..n { let k=(x as usize)&15; let (bx,by)=(jx[k],jy[k]); if bx==x {x=(x+1)%m; continue}
    let dx=sub(bx,x,m); let i=inv(dx,m); let dy=sub(by,y,m); let l=mul(dy,i); let l2=mul(l,l); let x3=sub(sub(l2,x,m),bx,m); let y3=sub(mul(l,sub(x,x3,m)),y,m); x=x3; y=y3; }
  (x,y) }
fn main(){
  let m:u64=4294967291;
  let mut s=12345u64; let mut r=||{s^=s<<13;s^=s>>7;s^=s<<17;s%m};
  let jx:Vec<u64>=(0..16).map(|_|r()).collect(); let jy:Vec<u64>=(0..16).map(|_|r()).collect();
  let b=Bar::new(m);
  for _ in 0..5_000_000 { let (a,c)=(r(),r()); assert_eq!(b.mul(a,c), mulmod_div(a,c,m)); }
  for mm in [3u64, 5, 65521, 4294967291, 2147483647, 1000003] { let bb=Bar::new(mm); let mut s2=7u64; for _ in 0..200000 { s2^=s2<<13;s2^=s2>>7;s2^=s2<<17; let a=s2%mm; let c=(s2>>17)%mm; assert_eq!(bb.mul(a,c), mulmod_div(a,c,mm)); } for a in [0,1,mm-1,mm/2] { for c in [0,1,mm-1,mm/2] { assert_eq!(bb.mul(a,c), mulmod_div(a,c,mm)); } } }
  let n=3_000_000;
  for round in 0..5 {
    let t=Instant::now(); let o1=walk(|a,c|mulmod_div(a,c,m), m, 7, 9, &jx,&jy,n); let d1=t.elapsed();
    let t=Instant::now(); let o2=walk(|a,c|b.mul(a,c), m, 7, 9, &jx,&jy,n); let d2=t.elapsed();
    assert_eq!(o1,o2);
    println!("round {round}: div {:.1} ns/step, barrett {:.1} ns/step", d1.as_nanos() as f64/n as f64, d2.as_nanos() as f64/n as f64);
  }
}
