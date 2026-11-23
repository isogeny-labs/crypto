use hec_native_ic::{jac::{Curve,Div},poly::Poly,instance::factor_base,oracle::{split,OracleStats}};
use crate::linear::{Linear,put};
use std::collections::BTreeMap;
#[path="../batched/inverse.rs"]mod inverse;
#[path="../native-public-2lp-cost/solver.rs"]mod eligibility;
#[derive(Debug,Default)]pub struct Report{pub k:Option<u64>,pub anchors:usize,pub degree_skips:usize,pub scanned:usize,pub u_zero:usize,pub buckets:usize,pub nonsplit:usize,pub accepted:usize,pub repeated:usize,pub overlaps:usize,pub dense_audits:usize,pub rows:usize,pub pivots:usize,pub small:usize,pub catalogue:usize,pub zero_anchor:bool}
fn brute(mut f:Poly,c:&Curve)->(bool,BTreeMap<u64,u32>){let mut roots=BTreeMap::new();for x in 0..c.fp.p{while f.deg()>0&&f.eval(x,&c.fp)==0{*roots.entry(x).or_insert(0)+=1;f=f.div_exact(&Poly::x_minus(x,&c.fp),&c.fp);}}(f.deg()==0,roots)}
pub fn solve(c:&Curve,d:&Div,target:&Div,ell:u64,limit:usize,audit:bool)->Report{
 eligibility::validate(c,d,target,ell);let fb=factor_base(c);let len=fb.len();let mut m=1usize;while(m as u128).pow(c.g as u32)<(len as u128).pow((c.g-1)as u32){m+=1;}let mut index=vec![usize::MAX;c.fp.p as usize];for(j,&(x,_))in fb.iter().enumerate(){index[x as usize]=j;}let mut linear=Linear::new(m,len,ell);let mut out=Report{small:m,catalogue:len,..Report::default()};let mut anchor=*target;
 for a in 1..=limit{out.anchors=a;anchor=c.add(&anchor,d);if c.is_zero(&anchor){let k=linear.f.neg((a as u64)%ell);assert_eq!(c.mul(d,k),*target);out.k=Some(k);out.zero_anchor=true;break;}if anchor.u.deg()!=c.g as isize{out.degree_skips+=1;continue;}
 let mut points=vec![];let mut den=vec![];for &(x,y)in &fb[..m]{out.scanned+=1;let u=anchor.u.eval(x,&c.fp);if u==0{out.u_zero+=1;continue;}points.push((x,y,anchor.v.eval(x,&c.fp)));den.push(u);}let inv=inverse::invert(&den,&c.fp);let mut buckets:BTreeMap<u64,Vec<(u64,u64)>>=BTreeMap::new();
 for(&(x,y,v),iv)in points.iter().zip(inv){let values=if y==0{vec![y]}else{vec![y,c.fp.neg(y)]};for yy in values{let t=c.fp.mul(c.fp.sub(yy,v),iv);buckets.entry(t).or_default().push((x,yy));}}
 for(t,known)in buckets{if known.len()<c.g-1{continue;}out.buckets+=1;let vt=anchor.v.add(&anchor.u.scale(t,&c.fp),&c.fp);let norm=c.f.sub(&vt.mul(&vt,&c.fp),&c.fp);let h=norm.div_exact(&anchor.u,&c.fp);assert_eq!(h.deg(),(c.g+1)as isize);assert_eq!(h.lc(),1);
 let mut remainder=h;let mut known_poly=Poly::one();let mut roots=BTreeMap::<u64,u32>::new();for &(x,y)in &known{assert_eq!(vt.eval(x,&c.fp),y);let factor=Poly::x_minus(x,&c.fp);remainder=remainder.div_exact(&factor,&c.fp);known_poly=known_poly.mul(&factor,&c.fp);*roots.entry(x).or_default()+=1;}
 assert!(remainder.deg()<=2);if audit{assert_eq!(known_poly.mul(&remainder,&c.fp),h);assert_eq!(anchor.u.mul(&h,&c.fp),norm);}let residual=split(&remainder,&c.fp,&mut OracleStats::default());let truth=if audit{Some(brute(h,c))}else{None};if let Some((all,_))=&truth{assert_eq!(residual.is_some(),*all);}let Some(residual)=residual else{out.nonsplit+=1;continue};for &(x,mult)in residual.as_slice(){*roots.entry(x).or_default()+=mult;}if let Some((_,expected))=truth{assert_eq!(roots,expected);}assert_eq!(roots.values().copied().sum::<u32>()as usize,c.g+1);
 out.repeated+=roots.values().filter(|&&v|v>1).count();out.overlaps+=roots.keys().filter(|&&x|anchor.u.eval(x,&c.fp)==0).count();let mut row=BTreeMap::new();let mut native=c.zero();for(&x,&mult)in &roots{let j=index[x as usize];assert_ne!(j,usize::MAX);let y=vt.eval(x,&c.fp);let sign=if y==fb[j].1{1}else{assert_eq!(y,c.fp.neg(fb[j].1));-1};let col=if j<m{j}else{j+1};put(&mut row,col,linear.f.from_i64(sign*mult as i64),&linear.f);if audit{for _ in 0..mult{native=c.add_place(&native,x,y);}}}
 if audit{assert!(c.is_zero(&c.add(&native,&anchor)),"principal divisor identity");}put(&mut row,m,1,&linear.f);out.accepted+=1;let rhs=linear.f.neg((a as u64)%ell);if let Some(k)=linear.push(row,rhs,audit){assert_eq!(c.mul(d,k),*target);out.k=Some(k);break;}
 }if out.k.is_some(){break;}
 }
 out.rows=linear.rows.len();out.pivots=linear.pivots.len();out.dense_audits=linear.audits;out
}
