use hec_native_ic::fp::Fp;use std::collections::BTreeMap;
type V=BTreeMap<usize,u64>;
pub struct Sparse{f:Fp,n:usize,basis:Vec<Option<(V,V)>>,rows:usize}
fn product(f:&Fp,a:u64,b:u64)->u64{if a==0||b==0{0}else if a==1{b}else if b==1{a}else if a==f.p-1{f.neg(b)}else if b==f.p-1{f.neg(a)}else{f.mul(a,b)}}
fn subtract(f:&Fp,a:&mut V,b:&V,scale:u64){for(&j,&x)in b{let v=f.sub(*a.get(&j).unwrap_or(&0),product(f,scale,x));if v==0{a.remove(&j);}else{a.insert(j,v);}}}
impl Sparse{
 pub fn new(n:usize,ell:u64)->Self{Self{f:Fp::new(ell),n,basis:vec![None;n],rows:0}}
 pub fn k(&self)->Option<u64>{self.basis[self.n-1].as_ref().map(|(r,_)|*r.get(&self.n).unwrap_or(&0))}
 pub fn witness(&self)->Vec<u64>{let mut out=vec![0;self.rows];for(&i,&x)in &self.basis[self.n-1].as_ref().unwrap().1{out[i]=x;}out}
 pub fn insert(&mut self,v:&[u64],rhs:u64){assert_eq!(v.len(),self.n);let mut r:V=v.iter().copied().chain(std::iter::once(rhs)).enumerate().filter(|(_,x)|*x!=0).collect();let mut w=V::from([(self.rows,1)]);self.rows+=1;loop{let Some((&j,&a))=r.first_key_value()else{return};assert!(j<self.n,"inconsistent row");if let Some((row,q))=&self.basis[j]{subtract(&self.f,&mut r,row,a);subtract(&self.f,&mut w,q,a);}else{let iv=if a==1{1}else{self.f.inv(a)};for x in r.values_mut(){*x=product(&self.f,*x,iv)}for x in w.values_mut(){*x=product(&self.f,*x,iv)}self.basis[j]=Some((r,w));return}}}
}
