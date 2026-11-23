use hec_native_ic::{instance::{thread_curve,factor_base},oracle::{split,OracleStats},rng::Rng};
use std::collections::BTreeMap;
#[derive(Default,Debug,Clone,Copy)]struct Ops{m:u64,a:u64}
struct Field{p:u64,o:Ops}
impl Field{
 fn add(&mut self,x:u64,y:u64)->u64{self.o.a+=1;(x+y)%self.p}
 fn sub(&mut self,x:u64,y:u64)->u64{self.o.a+=1;(x+self.p-y)%self.p}
 fn mul(&mut self,x:u64,y:u64)->u64{self.o.m+=1;((x as u128*y as u128)%self.p as u128)as u64}
}
struct Rel{time:usize,terms:Vec<(usize,u64)>}
struct Trace{n:usize,steps:Vec<usize>,rels:Vec<Rel>}
fn forward(t:&Trace,x:&[u64],f:&mut Field)->Vec<u64>{
 let mut accum=f.add(x[t.n-2],x[t.n-1]);let mut ri=0;let mut out=Vec::new();
 for(time,&j)in t.steps.iter().enumerate(){accum=f.add(accum,x[j]);while ri<t.rels.len()&&t.rels[ri].time==time+1{let mut s=0;for &(c,v)in &t.rels[ri].terms{let z=f.mul(v,x[c]);s=f.add(s,z)}out.push(f.sub(s,accum));ri+=1}}
 assert_eq!(ri,t.rels.len());out
}
fn transpose(t:&Trace,y:&[u64],f:&mut Field)->Vec<u64>{
 let mut out=vec![0;t.n];let mut suffix=0;let mut ri=t.rels.len();
 for time in (1..=t.steps.len()).rev(){while ri>0&&t.rels[ri-1].time==time{ri-=1;let w=y[ri];suffix=f.add(suffix,w);for &(c,v)in &t.rels[ri].terms{let z=f.mul(v,w);out[c]=f.add(out[c],z)}}let c=t.steps[time-1];out[c]=f.sub(out[c],suffix)}
 out[t.n-2]=f.sub(out[t.n-2],suffix);out[t.n-1]=f.sub(out[t.n-1],suffix);assert_eq!(ri,0);out
}
fn explicit(t:&Trace,p:u64)->Vec<Vec<(usize,u64)>>{
 let mut counts=vec![0;t.n];counts[t.n-2]=1;counts[t.n-1]=1;let mut ri=0;let mut out=Vec::new();
 for(time,&j)in t.steps.iter().enumerate(){counts[j]=(counts[j]+1)%p;while ri<t.rels.len()&&t.rels[ri].time==time+1{let mut a:BTreeMap<usize,u64>=counts.iter().enumerate().filter(|(_,v)|**v!=0).map(|(j,&v)|(j,p-v)).collect();for &(c,v)in &t.rels[ri].terms{let z=a.entry(c).or_default();*z=(*z+v)%p}out.push(a.into_iter().filter(|&(_,v)|v!=0).collect());ri+=1}}
 out
}
fn dense_forward(a:&[Vec<(usize,u64)>],x:&[u64],f:&mut Field)->Vec<u64>{a.iter().map(|r|{let mut s=0;for &(j,v)in r{let z=f.mul(v,x[j]);s=f.add(s,z)}s}).collect()}
fn dense_transpose(a:&[Vec<(usize,u64)>],y:&[u64],n:usize,f:&mut Field)->Vec<u64>{let mut out=vec![0;n];for(r,&w)in a.iter().zip(y){for &(j,v)in r{let z=f.mul(v,w);out[j]=f.add(out[j],z)}}out}
fn main(){
 for(g,p,cc,ell)in[(2,101,2,4663),(2,251,11,61667),(2,503,5,65179),(2,1009,12,241259),(2,2003,7,2001787),(2,4001,26,16050337),(3,61,5,124459),(3,211,12,4620611),(3,401,7,31525763),(4,31,16,239753),(4,61,2,16790591)]{
 let c=thread_curve(p,cc,g).unwrap();let fb=factor_base(&c);let m=fb.len();let d=c.place(fb[0].0,fb[0].1);let target=c.mul(&d,17);let mut r=c.add(&d,&target);let mut tr=Trace{n:m+2,steps:Vec::new(),rels:Vec::new()};let mut rng=Rng::new(2026112700^p^((g as u64)<<32));let mut os=OracleStats::default();
 for time in 1..=2048{let roll=rng.below(4);let j=if roll==0{m+1}else if roll==1{m}else{rng.below(m.min(64)as u64)as usize};tr.steps.push(j);r=if j==m{c.add(&r,&target)}else if j==m+1{c.add(&r,&d)}else{c.add_place(&r,fb[j].0,fb[j].1)};
 if let Some(roots)=split(&r.u,&c.fp,&mut os){let mut terms=Vec::new();let mut rebuilt=c.zero();for &(x,mult)in roots.as_slice(){let idx=fb.binary_search_by_key(&x,|&(x,_)|x).unwrap();let yy=c.f.eval(x,&c.fp);assert_eq!(c.fp.mul(r.v.eval(x,&c.fp),r.v.eval(x,&c.fp)),yy);let positive=r.v.eval(x,&c.fp)==fb[idx].1;let coeff=if positive{mult as u64}else{ell-mult as u64};terms.push((idx,coeff));let place=c.place(x,r.v.eval(x,&c.fp));for _ in 0..mult{rebuilt=c.add(&rebuilt,&place)}}assert_eq!(rebuilt,r);tr.rels.push(Rel{time,terms})}
 }
 assert!(!tr.rels.is_empty());let a=explicit(&tr,ell);let nnz:usize=a.iter().map(Vec::len).sum();let sparse:usize=tr.rels.iter().map(|r|r.terms.len()).sum();
 for rep in 0..8{let x:Vec<_>=(0..tr.n).map(|_|rng.below(ell)).collect();let y:Vec<_>=(0..a.len()).map(|_|rng.below(ell)).collect();let mut f=Field{p:ell,o:Ops::default()};let ax=forward(&tr,&x,&mut f);let fo=f.o;f.o=Ops::default();let at=transpose(&tr,&y,&mut f);let to=f.o;f.o=Ops::default();assert_eq!(ax,dense_forward(&a,&x,&mut f));let eo=f.o;f.o=Ops::default();assert_eq!(at,dense_transpose(&a,&y,tr.n,&mut f));let lhs=ax.iter().zip(&y).fold(0,|s,(&v,&w)|((s as u128+v as u128*w as u128)%ell as u128)as u64);let rhs=x.iter().zip(&at).fold(0,|s,(&v,&w)|((s as u128+v as u128*w as u128)%ell as u128)as u64);assert_eq!(lhs,rhs);assert_eq!(fo.m,sparse as u64);assert_eq!(to.m,sparse as u64);assert_eq!(eo.m,nnz as u64);
 if rep==0{println!("g={g} p={p} cols={} trials={} rows={} nnz={nnz} sparse={sparse} explicit_m={} explicit_a={} forward_m={} forward_a={} transpose_m={} transpose_a={}",tr.n,tr.steps.len(),a.len(),eo.m,eo.a,fo.m,fo.a,to.m,to.a)}
 }
 }println!("PASS original-model trace geometry, exact forward/transpose and adjoint controls; no solver performance inference");
}
