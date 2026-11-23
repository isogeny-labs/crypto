use hec_native_ic::fp::Fp;
use std::collections::BTreeMap;
type Sparse=BTreeMap<usize,u64>;
pub struct Dense{pub pivots:Vec<Option<Vec<u64>>>}
impl Dense{pub fn check(&mut self,raw:&Sparse,rhs:u64,m:usize,len:usize,f:&Fp,actual:&Sparse,arhs:u64,stored:bool){let mut v=vec![0;len+2];for(&j,&x)in raw{v[j]=x;}v[len+1]=rhs;let mut did_store=false;
 for j in m+1..=len{if v[j]==0{continue;}if let Some(ref pivot)=self.pivots[j]{let c=v[j];for k in 0..v.len(){v[k]=f.sub(v[k],f.mul(c,pivot[k]));}}else{let inv=f.inv(v[j]);for x in &mut v{*x=f.mul(*x,inv);}self.pivots[j]=Some(v.clone());did_store=true;break;}}
 let r:Sparse=(0..=len).filter(|&j|v[j]!=0).map(|j|(j,v[j])).collect();assert_eq!(&r,actual);assert_eq!(v[len+1],arhs);assert_eq!(did_store,stored);
}}
