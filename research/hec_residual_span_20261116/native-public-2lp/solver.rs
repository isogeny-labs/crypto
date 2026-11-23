use hec_native_ic::{fp::Fp,jac::{Curve,Div},instance::factor_base,oracle::{split,OracleStats},rng::Rng,solve::{Row,SolveStats,solve_fast}};
use std::collections::BTreeMap;
type Sparse=BTreeMap<usize,u64>;
#[derive(Debug,Default)]pub struct Report{pub k:Option<u64>,pub trials:u64,pub admitted:u64,pub partials:u64,pub pivots:usize,pub full_rows:usize,pub audits:usize,pub small:usize,pub catalogue:usize,pub max_lp:usize,pub solve:SolveStats}
pub struct Params{pub seed:u64,pub limit:u64,pub audit_prefix:usize}
fn put(row:&mut Sparse,j:usize,v:u64,f:&Fp){let n=f.add(row.get(&j).copied().unwrap_or(0),v);if n==0{row.remove(&j);}else{row.insert(j,n);}}
struct Dense{pivots:Vec<Option<Vec<u64>>>}
impl Dense{fn check(&mut self,raw:&Sparse,rhs:u64,m:usize,len:usize,f:&Fp,actual:&Sparse,arhs:u64,stored:bool){let mut v=vec![0;len+2];for(&j,&x)in raw{v[j]=x;}v[len+1]=rhs;let mut did_store=false;
 for j in m+1..=len{if v[j]==0{continue;}if let Some(ref pivot)=self.pivots[j]{let c=v[j];for k in 0..v.len(){v[k]=f.sub(v[k],f.mul(c,pivot[k]));}}else{let inv=f.inv(v[j]);for x in &mut v{*x=f.mul(*x,inv);}self.pivots[j]=Some(v.clone());did_store=true;break;}}
 let r:Sparse=(0..=len).filter(|&j|v[j]!=0).map(|j|(j,v[j])).collect();assert_eq!(&r,actual);assert_eq!(v[len+1],arhs);assert_eq!(did_store,stored);
}}
/// Public source/target API. No known scalar, source index, cofactor, or log table.
/// Dense prefix and native decomposition checks are correctness diagnostics, not timed production work.
pub fn solve(c:&Curve,d:&Div,t:&Div,ell:u64,params:&Params)->Report{
 assert!(ell>2&&ell<(1u64<<62));assert!((2..).take_while(|q|q*q<=ell).all(|q|ell%q!=0));assert!(c.is_valid(d)&&c.is_valid(t)&&!c.is_zero(d));assert!(c.is_zero(&c.mul(d,ell))&&c.is_zero(&c.mul(t,ell)));
 let mut root=1u128;while root*root<c.fp.p as u128{root+=1;}let upper=(root+1).pow((2*c.g)as u32);assert!((ell as u128)*(ell as u128)>upper,"requires unique ell-part via conservative Hasse bound");
 let f=Fp::new(ell);let fb=factor_base(c);let len=fb.len();assert!(len>0);let mut m=1usize;while(m as u128).pow(c.g as u32)<(len as u128).pow((c.g-1)as u32){m+=1;}m=m.min(len);
 let mut out=Report{small:m,catalogue:len,..Report::default()};let mut index=vec![usize::MAX;c.fp.p as usize];for(j,&(x,_))in fb.iter().enumerate(){index[x as usize]=j;}
 let mut rng=Rng::new(params.seed);let branches=m.min(64);let mut steps:Vec<_>=(0..m).collect();for i in 0..branches{let j=i+rng.below((m-i)as u64)as usize;steps.swap(i,j);}steps.truncate(branches);
 let(mut a,mut b)=(1u64,1u64);let mut current=c.add(d,t);let mut taken=vec![0u64;m];let mut pivots:BTreeMap<usize,(Sparse,u64)>=BTreeMap::new();let mut rows:Vec<(Row,u64)>=vec![];let mut wanted=m+8;let mut dense=Dense{pivots:vec![None;len+1]};let mut stats=OracleStats::default();
 while out.trials<params.limit{out.trials+=1;match rng.below(4){0=>{a=f.add(a,1);current=c.add(&current,d)},1=>{b=f.add(b,1);current=c.add(&current,t)},_=>{let j=steps[rng.below(branches as u64)as usize];taken[j]=f.add(taken[j],1);current=c.add_place(&current,fb[j].0,fb[j].1);}}
 let Some(roots)=split(&current.u,&c.fp,&mut stats)else{continue};let mut row=Sparse::new();let mut diagnostic=c.zero();let auditing=out.admitted<(params.audit_prefix as u64);
 for &(x,mult)in roots.as_slice(){let j=index[x as usize];assert_ne!(j,usize::MAX);let y=current.v.eval(x,&c.fp);let sign=if y==fb[j].1{1i64}else{assert_eq!(y,c.fp.neg(fb[j].1));-1};let col=if j<m{j}else{j+1};put(&mut row,col,f.from_i64(sign*mult as i64),&f);
 if auditing{for _ in 0..mult{diagnostic=c.add_place(&diagnostic,x,y);}}}
 for j in 0..m{put(&mut row,j,f.neg(taken[j]),&f);}put(&mut row,m,f.neg(b),&f);let lp=row.range(m+1..).count();if lp>2{continue;}if auditing{assert_eq!(diagnostic,current);}
 out.admitted+=1;out.max_lp=out.max_lp.max(lp);if lp>0{out.partials+=1;}let raw=if auditing{Some(row.clone())}else{None};let mut rhs=a;let mut stored=false;
 loop{let Some((&col,&coefficient))=row.range(m+1..).next()else{break};if let Some((pivot,prhs))=pivots.get(&col){for(&j,&v)in pivot{put(&mut row,j,f.neg(f.mul(coefficient,v)),&f);}rhs=f.sub(rhs,f.mul(coefficient,*prhs));}else{let inv=f.inv(coefficient);for v in row.values_mut(){*v=f.mul(*v,inv);}rhs=f.mul(rhs,inv);assert_eq!(row[&col],1);pivots.insert(col,(row.clone(),rhs));stored=true;break;}}
 assert!(row.range(m+1..).count()<=2);if let Some(raw)=raw{dense.check(&raw,a,m,len,&f,&row,rhs,stored);out.audits+=1;}
 if stored{continue;}assert!(row.keys().all(|&j|j<=m));if row.is_empty(){assert_eq!(rhs,0);continue;}rows.push((row.into_iter().map(|(j,v)|(j as u32,v)).collect(),rhs));
 if rows.len()>=wanted{if let Some(k)=solve_fast(&rows,m,ell,&mut out.solve){assert_eq!(c.mul(d,k),*t,"original-source scalar verification");out.k=Some(k);break;}wanted+=8+m/16;}
 }
 out.pivots=pivots.len();out.full_rows=rows.len();out
}
