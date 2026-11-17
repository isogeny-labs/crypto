#[path="../generic-kummer/kernel.rs"]
mod original;
pub use original::{best_add,decode,points,project,recover,same};
use hec_native_ic::{fp::Fp,jac::{Curve,Div}};
type K=[u64;4];
pub struct Compiled { generic:original::Compiled, constants:Option<[u64;8]> }
fn cm(f:&Fp,a:u64,b:u64)->u64 {if a==0||b==0{0}else if b==1{a}else if b==f.p-1{f.neg(a)}else{f.mul(a,b)}}
impl Compiled {
 pub fn new(c:&Curve,d:&Div)->Self {
  let generic=original::Compiled::new(c,d);
  let constants=if d.u.deg()==1 {let f=&c.fp;let r=f.neg(d.u.c[0]);let r2=f.mul(r,r);let r3=f.mul(r2,r);let r4=f.mul(r2,r2);let r5=f.mul(r4,r);let s=f.add(r5,c.f.c[0]);Some([r,r2,f.add(r3,r3),cm(f,r4,5),f.add(s,s),r3,r4,r5])}else{None};
  Self{generic,constants}
 }
 pub fn check_diagonal(&self,f:&Fp,p:&K){self.generic.check_diagonal(f,p)}
 pub fn step(&self,f:&Fp,p:&K,d:&K)->(K,usize){
  if d[0]==0||self.constants.is_none(){return self.generic.step(f,p,d)}
  let [r,r2,two_r3,five_r4,two_s,_,_,_]=self.constants.unwrap();
  let [x,y,k,z]=*p;
  let h=f.sub(y,cm(f,x,r));let u=f.sub(k,cm(f,h,r));
  let a=f.sub(f.sub(z,cm(f,y,r2)),cm(f,x,two_r3));
  let j=cm(f,x,two_s);
  let b0=f.mul(u,u);let b1=f.add(f.mul(u,a),f.mul(j,x));
  let l2=f.add(f.sub(cm(f,a,r),cm(f,u,r2)),cm(f,x,five_r4));
  let b2=f.add(f.mul(u,l2),f.mul(j,h));
  let l3=f.sub(f.add(f.add(cm(f,a,r2),cm(f,u,two_r3)),cm(f,y,five_r4)),j);
  let hh=f.mul(h,h);let b3=f.add(f.mul(u,l3),cm(f,hh,two_s));
  let mut w=[0;4];for(i,b)in [b0,b1,b2,b3].iter().enumerate(){w[i]=if i==0{f.mul(*b,d[0])}else{let q=f.mul(*b,d[0]);f.sub(f.add(q,q),f.mul(b0,d[i]))}}
  assert!(w.iter().any(|v|*v!=0));(w,0)
 }
}
