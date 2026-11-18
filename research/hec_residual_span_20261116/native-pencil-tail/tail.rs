use hec_native_ic::{jac::{Curve,Div},poly::Poly};
pub fn extract(c:&Curve,r:&Div,t:u64,known:&[(u64,u64)])->Poly{
 let g=c.g;let k=known.len();assert!(k>=g-1&&k<=g+1);if k==g+1{return Poly::one();}
 let f=&c.fp;let mut s1=0;let mut s2=0;for(i,&(x,_))in known.iter().enumerate(){if k==g-1&&i>0{s2=f.add(s2,f.mul(s1,x));}s1=f.add(s1,x);}
 let h=f.sub(f.sub(c.f.c[2*g],f.mul(t,t)),r.u.c[g-1]);let a=f.add(h,s1);if k==g{return Poly::from_coeffs(&[a,1]);}
 let top=f.add(r.v.c[g-1],f.mul(t,r.u.c[g-1]));let j=f.sub(f.sub(f.sub(c.f.c[2*g-1],f.mul(f.add(t,t),top)),f.mul(r.u.c[g-1],h)),r.u.c[g-2]);let b=f.sub(f.add(j,f.mul(a,s1)),s2);Poly::from_coeffs(&[b,a,1])
}
