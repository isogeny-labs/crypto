use hec_native_ic::fp::Fp;
pub struct Sparse{f:Fp,n:usize,basis:Vec<Option<(Vec<u64>,Vec<u64>)>>,rows:usize}
impl Sparse{
 pub fn new(n:usize,ell:u64)->Self{Self{f:Fp::new(ell),n,basis:vec![None;n],rows:0}}
 pub fn k(&self)->Option<u64>{self.basis[self.n-1].as_ref().map(|(r,_)|r[self.n])}
 pub fn witness(&self)->Vec<u64>{self.basis[self.n-1].as_ref().unwrap().1.clone()}
 pub fn insert(&mut self,v:&[u64],rhs:u64){assert_eq!(v.len(),self.n);self.rows+=1;let mut row=v.to_vec();row.push(rhs);let mut w=vec![0;self.rows];w[self.rows-1]=1;
  for j in 0..self.n{let a=row[j];if a==0{continue}if let Some((r,q))=&self.basis[j]{for t in j..=self.n{row[t]=self.f.sub(row[t],self.f.mul(a,r[t]));}for t in 0..q.len(){w[t]=self.f.sub(w[t],self.f.mul(a,q[t]));}}
   else{let iv=self.f.inv(a);for x in &mut row[j..]{*x=self.f.mul(*x,iv)}for x in &mut w{*x=self.f.mul(*x,iv)}self.basis[j]=Some((row,w));return}}
  assert_eq!(row[self.n],0,"inconsistent row");
 }
}
