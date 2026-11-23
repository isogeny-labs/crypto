use hec_native_ic::{fp::Fp,jac::{Curve,Div},poly::Poly};
#[derive(Default)]
pub struct Report {pub status:&'static str,pub scans:usize,pub evaluations:usize,pub inversions:usize,pub events:usize,pub fibres:Vec<Vec<(usize,i64)>>,pub graph_columns:usize,pub component:Vec<usize>,pub alpha:Option<Vec<u64>>}
fn root(p:&mut[usize],i:usize)->usize{if p[i]!=i{p[i]=root(p,p[i])}p[i]}
pub fn descend(c:&Curve,d:&Div,fb:&[(u64,u64)],anchor:&[bool],ell:u64)->Report{
let f=&c.fp;let lin=Fp::new(ell);let mut r=Report::default();r.status="precondition";
if !c.is_valid(d)||d.u.deg()!=c.g as isize||Poly::gcd(&d.u,&d.u.deriv(f),f).deg()!=0||Poly::gcd(&d.u,&d.v,f).deg()!=0{return r}
let mut buckets=vec![vec![];f.p as usize];
for(j,&(x,y))in fb.iter().enumerate(){r.scans+=1;r.evaluations+=1;let u=d.u.eval(x,f);if u==0{continue}r.evaluations+=1;let v=d.v.eval(x,f);let inv=f.inv(u);r.inversions+=1;let t=f.mul(f.sub(y,v),inv);buckets[t as usize].push((j,1));r.events+=1;if y!=0{let t=f.mul(f.sub(f.neg(y),v),inv);buckets[t as usize].push((j,-1));r.events+=1;}}
r.fibres=buckets.into_iter().filter(|v|v.len()==c.g+1).collect();let n=r.fibres.len();let mut parent:Vec<_>=(0..n).collect();let mut grounded=vec![false;n];
let mut occurs=vec![vec![];fb.len()];for(i,v)in r.fibres.iter().enumerate(){for &(j,sign)in v{if fb[j].1!=0{occurs[j].push((i,sign));}}}
for j in 0..fb.len(){if anchor[j]{continue}r.graph_columns+=1;match occurs[j].as_slice(){[]=>{},[(i,_)]=>grounded[*i]=true,[(i,s),(k,t)]=>{assert_eq!(s+t,0);let a=root(&mut parent,*i);let b=root(&mut parent,*k);parent[a]=b},_=>panic!("incidence")}}
let mut components=vec![vec![];n];let mut ground=vec![false;n];for i in 0..n{let q=root(&mut parent,i);components[q].push(i);ground[q]|=grounded[i];}
for(i,v)in components.into_iter().enumerate(){if v.is_empty()||ground[i]||v.len()as u64%ell==0{continue}let mut w=vec![0i64;fb.len()];for &a in &v{for &(j,sign)in &r.fibres[a]{if fb[j].1!=0{w[j]+=sign}}}for j in 0..fb.len(){if !anchor[j]{assert_eq!(w[j],0)}}let z=lin.neg(lin.inv(v.len()as u64%ell));r.alpha=Some(w.into_iter().map(|v|lin.mul(lin.from_i64(v),z)).collect());r.component=v;r.status="component";return r}
r.status="no_component";r}
