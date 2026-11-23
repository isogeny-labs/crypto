use crate::field::{F,P,Z,scalar,poly};
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]pub struct D{pub u:P,pub v:P}
pub struct C<'a>{pub f:&'a F,pub ff:P,pub g:usize}
impl<'a>C<'a>{
 pub fn zero(&self)->D{D{u:poly(&[1]),v:vec![]}}
 pub fn neg(&self,a:&D)->D{D{u:a.u.clone(),v:self.f.ps(&vec![],&a.v)}}
 pub fn valid(&self,a:&D)->bool{!a.u.is_empty()&&a.u.last()==Some(&scalar(1))&&a.u.len()<=self.g+1&&a.v.len()<a.u.len()&&self.f.rem(&self.f.ps(&self.ff,&self.f.pm(&a.v,&a.v)),&a.u).is_empty()}
 pub fn scale(&self,a:&P,v:crate::field::E)->P{crate::field::trim(a.iter().map(|&x|self.f.mul(x,v)).collect())}
 pub fn exact(&self,a:&P,b:&P)->P{let(q,r)=self.f.div(a,b);assert!(r.is_empty(),"inexact Cantor quotient");q}
 pub fn xgcd(&self,a:&P,b:&P)->(P,P,P){let(mut r0,mut r1)=(a.clone(),b.clone());let(mut s0,mut s1)=(poly(&[1]),vec![]);let(mut t0,mut t1)=(vec![],poly(&[1]));while !r1.is_empty(){let(q,r)=self.f.div(&r0,&r1);let s=self.f.ps(&s0,&self.f.pm(&q,&s1));let t=self.f.ps(&t0,&self.f.pm(&q,&t1));r0=r1;r1=r;s0=s1;s1=s;t0=t1;t1=t;}if r0.is_empty(){return(r0,s0,t0)}let inv=self.f.inv(*r0.last().unwrap());(self.scale(&r0,inv),self.scale(&s0,inv),self.scale(&t0,inv))}
 pub fn reduce(&self,mut u:P,mut v:P)->D{u=self.f.monic(&u);v=self.f.rem(&v,&u);while u.len()>self.g+1{let u2=self.f.monic(&self.exact(&self.f.ps(&self.ff,&self.f.pm(&v,&v)),&u));v=self.f.rem(&self.f.ps(&vec![],&v),&u2);u=u2}let out=D{u,v};assert!(self.valid(&out));out}
 pub fn add(&self,a:&D,b:&D)->D{if a.u.len()==1{return b.clone()}if b.u.len()==1{return a.clone()}let(d1,e1,e2)=self.xgcd(&a.u,&b.u);let(d,c1,c2)=self.xgcd(&d1,&self.f.pa(&a.v,&b.v));let s1=self.f.pm(&c1,&e1);let s2=self.f.pm(&c1,&e2);let u=self.exact(&self.f.pm(&a.u,&b.u),&self.f.pm(&d,&d));let t1=self.f.pm(&self.f.pm(&s1,&a.u),&b.v);let t2=self.f.pm(&self.f.pm(&s2,&b.u),&a.v);let t3=self.f.pm(&c2,&self.f.pa(&self.f.pm(&a.v,&b.v),&self.ff));let v=self.exact(&self.f.pa(&self.f.pa(&t1,&t2),&t3),&d);self.reduce(u.clone(),self.f.rem(&v,&u))}
 pub fn mul(&self,a:&D,mut k:usize)->D{let mut r=self.zero();let mut b=a.clone();while k>0{if k&1!=0{r=self.add(&r,&b)}k>>=1;if k>0{b=self.add(&b,&b)}}r}
 pub fn branch(&self,a:crate::field::E)->D{assert_eq!(self.f.eval(&self.ff,a),Z);D{u:vec![self.f.sub(Z,a),scalar(1)],v:vec![]}}
}
