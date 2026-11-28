use hec_native_ic::{fp::Fp,solve::{Row,SolveStats,solve_fast}};
use std::collections::BTreeMap;
pub type Sparse=BTreeMap<usize,u64>;
pub fn put(r:&mut Sparse,j:usize,v:u64,f:&Fp){let n=f.add(r.get(&j).copied().unwrap_or(0),v);if n==0{r.remove(&j);}else{r.insert(j,n);}}
pub struct Linear{pub f:Fp,pub m:usize,pub len:usize,pub admitted:usize,pub audits:usize,pub rows:Vec<(Row,u64)>,pub pivots:BTreeMap<usize,()>,parent:Vec<usize>,size:Vec<usize>,weight:Vec<u64>,shift:Vec<Sparse>,small:crate::small::Sparse,raw:Vec<(Row,u64)>}
impl Linear{
pub fn new(m:usize,len:usize,ell:u64)->Self{let n=len-m;Self{f:Fp::new(ell),m,len,admitted:0,audits:0,rows:vec![],pivots:BTreeMap::new(),parent:(0..=n).collect(),size:vec![1;n+1],weight:vec![1;n+1],shift:vec![Sparse::new();n+1],small:crate::small::Sparse::new(m+1,ell),raw:vec![]}}
fn product(&self,a:u64,b:u64)->u64{if a==0||b==0{0}else if a==1{b}else if b==1{a}else{self.f.mul(a,b)}}
fn add_scaled(&self,q:&mut Sparse,a:u64,h:&Sparse){for(&j,&v)in h{put(q,j,self.product(a,v),&self.f);}}
fn find(&mut self,x:usize)->(usize,u64,Sparse){if self.parent[x]==x{return(x,1,Sparse::new());}let(p,w,h)=(self.parent[x],self.weight[x],self.shift[x].clone());let(r,v,t)=self.find(p);let mut q=h;self.add_scaled(&mut q,w,&t);let w=self.product(w,v);self.parent[x]=r;self.weight[x]=w;self.shift[x]=q.clone();(r,w,q)}
pub fn scan_indices(&self)->Vec<usize>{let ground=self.len-self.m;let mut indices:Vec<usize>=(0..self.m).collect();for j in 0..ground{let mut r=j;while self.parent[r]!=r{r=self.parent[r];}if r==ground{indices.push(self.m+j);}}indices}
fn insert(&mut self,row:&Sparse,rhs:u64){let ground=self.len-self.m;let mut q:Sparse=row.range(..=self.m).map(|(&j,&v)|(j,v)).collect();put(&mut q,self.m+1,self.f.neg(rhs),&self.f);let mut roots=Vec::<(usize,u64)>::new();for(&j,&a)in row.range(self.m+1..){let(r,w,h)=self.find(j-self.m-1);self.add_scaled(&mut q,a,&h);if r==ground{continue;}let b=self.product(a,w);if let Some(e)=roots.iter_mut().find(|e|e.0==r){e.1=self.f.add(e.1,b);}else{roots.push((r,b));}}roots.retain(|e|e.1!=0);assert!(roots.len()<=2,"at most two ungrounded residual roots");
if roots.is_empty(){let rhs=self.f.neg(q.remove(&(self.m+1)).unwrap_or(0));if q.is_empty(){assert_eq!(rhs,0);return;}let mut v=vec![0;self.m+1];for(&j,&x)in &q{v[j]=x;}self.rows.push((q.into_iter().map(|(j,v)|(j as u32,v)).collect(),rhs));self.small.insert(&v,rhs);return;}
if roots.len()==2&&self.size[roots[0].0]>self.size[roots[1].0]{roots.swap(0,1);}let(root,a)=roots[0];let inv=self.f.inv(a);let h=q.into_iter().map(|(j,v)|(j,self.f.neg(self.product(v,inv)))).collect();if roots.len()==1{self.parent[root]=ground;self.weight[root]=0;}else{let(other,b)=roots[1];self.parent[root]=other;self.weight[root]=self.f.neg(self.product(b,inv));self.size[other]+=self.size[root];}self.shift[root]=h;self.pivots.insert(self.pivots.len(),());
}
pub fn push(&mut self,row:Sparse,rhs:u64,audit:bool)->Option<u64>{self.admitted+=1;if audit{let mut raw:Row=row.iter().map(|(&j,&v)|((if j==self.m{self.len}else if j>self.m{j-1}else{j})as u32,v)).collect();raw.sort_unstable_by_key(|e|e.0);self.raw.push((raw,rhs));}self.insert(&row,rhs);let k=self.small.k();if audit&&(self.admitted<=128||k.is_some()){let expected=solve_fast(&self.raw,self.len,self.f.p,&mut SolveStats::default());assert_eq!(k,expected,"forest full-system target prefix");self.audits+=1;}k}
}
