use hec_native_ic::{fp::Fp,solve::{Row,SolveStats,solve_fast}};
use crate::dense::Dense;
use std::collections::BTreeMap;
pub type Sparse=BTreeMap<usize,u64>;
pub fn put(r:&mut Sparse,j:usize,v:u64,f:&Fp){let n=f.add(r.get(&j).copied().unwrap_or(0),v);if n==0{r.remove(&j);}else{r.insert(j,n);}}
pub struct Linear{pub f:Fp,pub m:usize,pub len:usize,pub admitted:usize,pub audits:usize,pub rows:Vec<(Row,u64)>,pub pivots:BTreeMap<usize,(Sparse,u64)>,dense:Dense,wanted:usize}
impl Linear{
 pub fn new(m:usize,len:usize,ell:u64)->Self{Self{f:Fp::new(ell),m,len,admitted:0,audits:0,rows:vec![],pivots:BTreeMap::new(),dense:Dense{pivots:vec![None;len+1]},wanted:m+8}}
 pub fn push(&mut self,mut row:Sparse,mut rhs:u64,audit:bool)->Option<u64>{assert!(row.range(self.m+1..).count()<=2);let raw=if audit&&self.admitted<128{Some((row.clone(),rhs))}else{None};self.admitted+=1;let mut stored=false;
 loop{let Some((&col,&co))=row.range(self.m+1..).next()else{break};if let Some((p,prhs))=self.pivots.get(&col){for(&j,&v)in p{put(&mut row,j,self.f.neg(self.f.mul(co,v)),&self.f);}rhs=self.f.sub(rhs,self.f.mul(co,*prhs));}else{let inv=self.f.inv(co);for v in row.values_mut(){*v=self.f.mul(*v,inv);}rhs=self.f.mul(rhs,inv);self.pivots.insert(col,(row.clone(),rhs));stored=true;break;}}
 assert!(row.range(self.m+1..).count()<=2);if let Some((raw,rr))=raw{self.dense.check(&raw,rr,self.m,self.len,&self.f,&row,rhs,stored);self.audits+=1;}if stored{return None;}if row.is_empty(){assert_eq!(rhs,0);return None;}
 self.rows.push((row.into_iter().map(|(j,v)|(j as u32,v)).collect(),rhs));if self.rows.len()>=self.wanted{let k=solve_fast(&self.rows,self.m,self.f.p,&mut SolveStats::default());if k.is_some(){return k;}self.wanted+=8+self.m/16;}None
 }
}
