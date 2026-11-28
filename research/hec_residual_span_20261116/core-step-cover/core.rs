#![allow(dead_code,unused_imports)]
#[derive(Clone,Debug,PartialEq,Eq)]pub struct Primitive{pub a:u64,pub t:u64,pub points:Vec<(u64,u64)>}

#[allow(dead_code)]#[path="../canonical-root/root.rs"]mod root;
#[allow(dead_code)]#[path="../action-consensus/ring.rs"]mod ring;
#[path="../batched/inverse.rs"]mod inverse;
#[path="../setup/fast.rs"]mod fast;
#[path="../multi/algebra.rs"]mod algebra;
use hec_native_ic::{counted,jac::{Curve,Div},fp::Fp,poly::Poly,rng::Rng};
use ring::{R,Ring,ZERO,ONE};
use std::collections::{BTreeMap,BTreeSet};
type Label=[R;3];
#[derive(Clone,Debug,PartialEq,Eq)]pub struct Trace{pub anchor_certificates:Vec<crate::certificate::Certificate>,pub primitives:Vec<Primitive>,pub answer:Option<(Option<u64>,u64)>,pub z:u64,pub points:Vec<(u64,u64)>,pub labels:Vec<Label>,pub scans:usize,pub visits:usize,pub rows:usize,pub anchors:usize,pub survivors:usize,pub seed_trials:usize}
fn add_orbit(c:&Curve,z:u64,point:(u64,u64),h:Label,points:&mut Vec<(u64,u64)>,labels:&mut Vec<Label>,map:&mut BTreeMap<u64,(usize,usize)>){let i=points.len();assert_ne!(point.0,0);points.push(point);labels.push(h);let mut x=point.0;for e in 0..5{assert!(map.insert(x,(i,e)).is_none());x=c.fp.mul(x,z);}assert_eq!(x,point.0);}
pub fn solve(c:&Curve,core:(u64,u64),d:&Div,target:&Div,ell:u64,sm:u64,seed_ordinal:usize,gate:bool,consensus:bool)->Trace{
 if consensus{assert_eq!(c.g,2);assert_eq!(c.f.deg(),5);assert_eq!(c.f.lc(),1);assert!(c.f.c[1..5].iter().all(|x|*x==0));assert!([(101,2,7481),(251,18,67931),(601,7,411491)].contains(&(c.fp.p,c.f.c[0],ell)));assert!(!c.is_zero(d));}assert_eq!(c.mul(&c.place(core.0,core.1),sm),*d);let f=&c.fp;let z=root::power_map(f,5).0;let r=Ring::new(ell);let mut constraints=ring::Dsu::new(0,ell);let mut points=vec![];let mut labels=vec![];let mut map=BTreeMap::new();let invsm=r.f.inv(sm);let step=c.place(core.0,core.1);let initial=[ZERO,ZERO,r.constant(invsm)];add_orbit(c,z,core,initial,&mut points,&mut labels,&mut map);
 let mut seed=None;let mut seed_trials=0;let mut found=0;let mut excluded:BTreeSet<u64>=map.keys().copied().collect();for x in 0..f.p{if x==0||excluded.contains(&x){continue}seed_trials+=1;if let Some(y)=f.sqrt(c.f.eval(x,f)){found+=1;if found==seed_ordinal{seed=Some((x,y.min(f.neg(y))));break}let mut xx=x;for _ in 0..5{excluded.insert(xx);xx=f.mul(xx,z);}}}
 add_orbit(c,z,seed.unwrap(),[ZERO,ONE,ZERO],&mut points,&mut labels,&mut map);
 let mut anchor_certificates=vec![];let mut primitives=vec![];let mut cache=Vec::<Div>::new();let mut seen=BTreeSet::new();let(mut scans,mut visits,mut rows)=(0,0,0);let mut answer=None;
 'closure:loop{let before=points.len();for ai in 0..16{if cache.len()==ai{let prev=cache.last().unwrap_or(target);let proof=crate::certificate::build(c,prev,&step);cache.push(proof.out);anchor_certificates.push(proof);}let anchor=cache[ai];scans+=1;if anchor.u.deg()!=2{continue}let mut entries=vec![];let mut den=vec![];
 for(i,&(x,y))in points.iter().enumerate(){let mut xx=x;for e in 0..5{visits+=1;let u=anchor.u.eval(xx,f);if u!=0{entries.push((i,e,xx,y));den.push(u);}xx=f.mul(xx,z);}}
 let inv=inverse::invert(&den,f);let mut buckets=BTreeMap::<u64,Vec<(usize,usize,u64,u64)>>::new();for(&(i,e,x,y),&iv)in entries.iter().zip(&inv){let v=anchor.v.eval(x,f);for(sign,yy)in[y,f.neg(y)].into_iter().enumerate(){if sign==1&&y==0{continue}let t=f.mul(f.sub(yy,v),iv);buckets.entry(t).or_default().push((i,e,x,yy));}}
 for(t,b)in buckets{if b.len()<2||seen.contains(&(ai,t)){continue}let a=b[0];let Some(&q)=b.iter().find(|x|x.2!=a.2)else{continue};let w=anchor.v.add(&anchor.u.scale(t,f),f);let x=f.sub(f.sub(f.add(f.mul(t,t),anchor.u.c[1]),a.2),q.2);let y=w.eval(x,f);if gate{let h=c.f.sub(&w.mul(&w,f),f).div_exact(&anchor.u,f);assert_eq!(h.deg(),3);assert_eq!(h.lc(),1);let pair=Poly::x_minus(a.2,f).mul(&Poly::x_minus(q.2,f),f);let residual=h.div_exact(&pair,f);assert_eq!(residual.deg(),1);assert_eq!(residual.lc(),1);assert_eq!(x,f.neg(residual.c[0]));}seen.insert((ai,t));
 let mut sum=[ONE,ZERO,r.constant(if invsm==1{(ai+1)as u64}else{r.f.mul((ai+1)as u64,invsm)})];let mut group=anchor;for(i,e,xx,yy)in[a,q]{let weight=if yy==points[i].1{r.z(e)}else{assert_eq!(yy,f.neg(points[i].1));r.neg(r.z(e))};for k in 0..3{sum[k]=r.add(sum[k],r.mul(weight,labels[i][k]));}if gate{assert_eq!(w.eval(xx,f),yy);group=c.add_place(&group,xx,yy);}}if gate{group=c.add_place(&group,x,y);assert!(c.is_zero(&group));}
 if let Some(&(i,e))=map.get(&x){let weight=if y==points[i].1{r.z(e)}else{assert_eq!(y,f.neg(points[i].1));r.neg(r.z(e))};for k in 0..3{sum[k]=r.add(sum[k],r.mul(weight,labels[i][k]));}constraints.insert(&[sum[1],sum[0]],r.neg(sum[2]));}else{let yy=y.min(f.neg(y));let sign=if yy==y{1}else{ell-1};let label=sum.map(|v|r.scale(r.neg(v),sign));add_orbit(c,z,(x,yy),label,&mut points,&mut labels,&mut map);}
 primitives.push(Primitive{a:(ai+1)as u64,t,points:vec![(a.2,a.3),(q.2,q.3),(x,y)]});rows+=1;if consensus{if let Some(k)=constraints.common_k(){let before=counted::snapshot();assert_eq!(c.mul(d,k),*target);crate::cost::record(before,counted::snapshot());let eigen=constraints.unique().map(|x|x.0);answer=Some((eigen,k));break 'closure;}}else if let Some((lambda,Some(k)))=constraints.unique(){assert_eq!(c.mul(d,k),*target);assert_eq!(c.mul(d,lambda),algebra::action(c,d,z));answer=Some((Some(lambda),k));break 'closure;}
 }
 }if points.len()==before{break}assert!(points.len()<f.p as usize);}
 Trace{anchor_certificates,primitives,answer,z,points,labels,scans,visits,rows,anchors:cache.len(),survivors:constraints.survivors(),seed_trials}
}
