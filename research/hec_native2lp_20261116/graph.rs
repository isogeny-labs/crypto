use hec_native_ic::fp::Fp;
use std::collections::BTreeMap;
pub type Row=Vec<(u32,u64)>;
pub fn combine(f:&Fp,a:&Row,ca:u64,b:&Row,cb:u64)->Row{let mut out=BTreeMap::new();for(r,c)in[(a,ca),(b,cb)]{for&(i,v)in r{let e=out.entry(i).or_insert(0);*e=f.add(*e,f.mul(v,c));}}out.into_iter().filter(|&(_,v)|v!=0).collect()}
pub fn value(f:&Fp,row:&Row,x:&[u64])->u64{row.iter().fold(0,|z,&(i,a)|f.add(z,f.mul(a,x[i as usize])))}
#[derive(Default,Debug)]pub struct Stats{pub merges:u64,pub anchors:u64,pub full:u64,pub balanced_cycles:u64,pub anchored_cycles:u64,pub max_potential_entries:usize}
pub struct Graph{f:Fp,parent:Vec<usize>,size:Vec<usize>,scale:Vec<u64>,potential:Vec<Row>,dim:usize,entries:usize,pub stats:Stats}
impl Graph{
 pub fn new(n:usize,dim:usize,ell:u64)->Self{Self{f:Fp::new(ell),parent:(0..=n).collect(),size:vec![1;n+1],scale:vec![1;n+1],potential:vec![vec![];n+1],dim,entries:0,stats:Stats::default()}}
 fn set_potential(&mut self,v:usize,row:Row){self.entries=self.entries-self.potential[v].len()+row.len();assert!(self.entries<=20_000_000,"potential_storage_cap20M");self.stats.max_potential_entries=self.stats.max_potential_entries.max(self.entries);self.potential[v]=row;}
 pub fn find(&mut self,v:usize)->(usize,u64,Row){let p=self.parent[v];if p==v{return(v,1,vec![])}let(r,c,e)=self.find(p);let s=self.scale[v];let out=combine(&self.f,&self.potential[v],1,&e,s);let c=self.f.mul(s,c);self.parent[v]=r;self.scale[v]=c;self.set_potential(v,out.clone());(r,c,out)}
 pub fn insert(&mut self,large:&[(usize,u64)],row:Row)->Option<Row>{assert!(large.len()<=2);assert!(row.iter().all(|&(i,a)|i as usize<=self.dim&&a<self.f.p));assert!(large.iter().all(|&(i,c)|i>0&&i<self.parent.len()&&c>0&&c<self.f.p));if large.is_empty(){self.stats.full+=1;return Some(row)}let(u,alpha)=large[0];let(v,beta)=if large.len()==2{large[1]}else{(0,1)};let(ru,su,eu)=self.find(u);let(rv,sv,ev)=self.find(v);let p=self.f.mul(alpha,su);let q=self.f.mul(beta,sv);let mut r=combine(&self.f,&row,1,&eu,alpha);r=combine(&self.f,&r,1,&ev,beta);
 if ru==rv{if ru==0{self.stats.full+=1;self.stats.anchored_cycles+=1;return Some(r)}let d=self.f.add(p,q);if d==0{self.stats.full+=1;self.stats.balanced_cycles+=1;return Some(r)}let inv=self.f.inv(d);self.parent[ru]=0;self.scale[ru]=1;let e=combine(&self.f,&r,self.f.neg(inv),&vec![],0);self.set_potential(ru,e);self.size[0]+=self.size[ru];self.stats.anchors+=1;}else{let(child,root,c,den)=if ru==0||(rv!=0&&self.size[ru]>=self.size[rv]){(rv,ru,p,q)}else{(ru,rv,q,p)};let inv=self.f.inv(den);self.parent[child]=root;self.scale[child]=self.f.neg(self.f.mul(c,inv));let e=combine(&self.f,&r,self.f.neg(inv),&vec![],0);self.set_potential(child,e);self.size[root]+=self.size[child];self.stats.merges+=1;}
 None}
 pub fn check(&mut self,small:&[u64],large:&[u64]){assert_eq!(small.len(),self.dim+1);assert_eq!(large.len(),self.parent.len());assert_eq!(large[0],0);for v in 0..large.len(){let(r,c,e)=self.find(v);assert_eq!(large[v],self.f.add(self.f.mul(c,large[r]),value(&self.f,&e,small)),"affine_graph_invariant v={v}");}}
}
