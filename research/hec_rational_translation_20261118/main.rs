use hec_native_ic::{instance::{thread_curve,factor_base},jac::{Curve,Div},poly::Poly};
fn direct(c:&Curve,d:&Div,q:u64,y:u64)->Option<Div>{
 let f=&c.fp;if d.u.deg()!=2{return None}let(s,t)=(d.u.c[1],d.u.c[0]);let(v,w)=(d.v.c[1],d.v.c[0]);
 let q2=f.mul(q,q);let u=f.add(f.add(q2,f.mul(s,q)),t);if u==0{return None}
 let a=f.mul(f.sub(y,f.add(f.mul(v,q),w)),f.inv(u));let aa=f.mul(a,a);let b=f.mul(a,v);let b=f.add(b,b);
 let cc=f.add(f.sub(f.add(f.sub(c.f.c[3],t),f.mul(s,s)),f.mul(s,q)),q2);
 let sn=f.sub(f.sub(q,s),aa);let tn=f.sub(f.sub(cc,b),f.mul(aa,f.add(s,q)));
 let vn=f.neg(f.add(v,f.mul(a,f.sub(s,sn))));let wn=f.neg(f.add(w,f.mul(a,f.sub(t,tn))));
 Some(Div{u:Poly::from_coeffs(&[tn,sn,1]),v:Poly::from_coeffs(&[wn,vn])})
}
// Inputs contain no signed v coefficients. Fixed-point constants are prepared outside.
fn chain(c:&Curve,u:&Poly,prev:&Poly,q:u64,y:u64)->Option<(Poly,Div)>{
 let f=&c.fp;if u.deg()!=2||prev.deg()!=2{return None}let(s,t)=(u.c[1],u.c[0]);let(r,z)=(prev.c[1],prev.c[0]);
 let q2=f.mul(q,q);let uu=f.add(f.add(q2,f.mul(s,q)),t);let aa=f.sub(f.sub(q,s),r);
 let s2=f.mul(s,s);let cc=f.add(f.sub(f.add(f.sub(c.f.c[3],t),s2),f.mul(s,q)),q2);
 let b=f.sub(f.sub(cc,z),f.mul(aa,f.add(s,q)));if uu==0||aa==0||b==0||y==0{return None}
 let s2t=f.mul(s2,t);let three=f.add(f.add(s2t,s2t),s2t);
 let rem=f.add(f.sub(f.add(f.sub(f.mul(s2,s2),three),f.mul(t,t)),f.mul(c.f.c[2],s)),f.add(f.mul(c.f.c[3],f.sub(s2,t)),c.f.c[1]));
 let l=f.sub(f.sub(f.neg(f.mul(f.mul(aa,rem),f.inv(b))),f.mul(aa,uu)),f.mul(f.mul(f.add(f.add(q,q),s),b),f.inv(4)));
 let iu=f.inv(uu);let li=f.mul(l,iu);let yy=f.mul(y,y);let yi=f.mul(f.mul(yy,iu),iu);
 let four=|x|f.add(f.add(x,x),f.add(x,x));
 let ap=f.add(aa,f.add(four(li),four(yi)));let lb=f.mul(f.mul(li,b),f.inv(aa));let bp=f.add(b,f.add(lb,lb));
 let sn=f.sub(f.sub(q,s),ap);let tn=f.sub(f.sub(cc,bp),f.mul(ap,f.add(s,q)));
 let am=f.mul(l,f.inv(y));let v=f.mul(f.mul(b,am),f.inv(f.add(aa,aa)));let w=f.sub(f.sub(f.neg(y),f.mul(am,uu)),f.mul(q,v));
 Some((Poly::from_coeffs(&[tn,sn,1]),Div{u:*u,v:Poly::from_coeffs(&[w,v])}))
}
fn main(){
 for(p,k)in[(101,2),(251,11),(503,5),(1009,12),(2003,7),(4001,26)]{
 let c=thread_curve(p,k,2).unwrap();let(q,y)=factor_base(&c).into_iter().find(|&(_,y)|y!=0).unwrap();let step=c.place(q,y);let mut d=c.zero();let(mut direct_ok,mut chain_ok,mut direct_exception,mut chain_exception)=(0,0,0,0);
 for _ in 0..512{let next=c.add(&d,&step);let prev=c.add(&d,&c.neg(&step));
 if let Some(got)=direct(&c,&d,q,y){assert_eq!(got,next);assert!(c.is_valid(&got));direct_ok+=1}else{direct_exception+=1}
 if let Some((un,recovered))=chain(&c,&d.u,&prev.u,q,y){assert_eq!(un,next.u);assert_eq!(recovered,d);chain_ok+=1}else{chain_exception+=1}
 d=next;
 }assert!(direct_ok>400&&chain_ok>350);println!("p={p} q={q} y={y} direct_ok={direct_ok} direct_exception={direct_exception} chain_ok={chain_ok} chain_exception={chain_exception}");
 }println!("PASS exact direct translation, u-only recurrence, signed recovery; no timing inference");
}
