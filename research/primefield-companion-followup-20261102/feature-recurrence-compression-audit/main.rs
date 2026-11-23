#[path="../feature-native-signed-image/metered-polynomial.rs"]mod polynomial;
#[path="../feature-native-signed-image/meter.rs"]mod meter;
type Q=Vec<u64>;type P=Vec<Q>;
#[derive(Clone)]struct Tri{a:P,b:P,d:P}
#[derive(Clone)]struct Val{a:Q,b:Q,d:Q}
struct Ring{r:u64,g:Q,p:polynomial::Poly}
fn mul(a:u64,b:u64,r:u64)->u64{(a as u128*b as u128%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}n>>=1;if n>0{a=mul(a,a,r)}}v}
impl Ring{
 fn rem(&mut self,a:&Q)->Q{self.p.divrem(a,&self.g).unwrap().1}
 fn qm(&mut self,a:&Q,b:&Q)->Q{let c=self.p.mul(a,b);self.rem(&c)}
 fn qa(&self,a:&Q,b:&Q)->Q{self.p.add(a,b)}
 fn scale(&mut self,a:&Q,c:u64)->Q{let v=a.iter().map(|&x|mul(x,c,self.r)).collect();self.rem(&v)}
 fn pa(&self,a:&P,b:&P)->P{let mut v=vec![vec![0];a.len().max(b.len())];for(i,c)in a.iter().enumerate(){v[i]=self.qa(&v[i],c)}for(i,c)in b.iter().enumerate(){v[i]=self.qa(&v[i],c)}while v.len()>1&&v.last()==Some(&vec![0]){v.pop();}v}
 fn pm(&mut self,a:&P,b:&P)->P{let mut v=vec![vec![0];a.len()+b.len()-1];for(i,x)in a.iter().enumerate(){for(j,y)in b.iter().enumerate(){let c=self.qm(x,y);v[i+j]=self.qa(&v[i+j],&c);}}while v.len()>1&&v.last()==Some(&vec![0]){v.pop();}v}
 fn compose(&mut self,outer:&Tri,inner:&Tri)->Tri{let a=self.pm(&outer.a,&inner.a);let x=self.pm(&outer.b,&inner.a);let y=self.pm(&outer.d,&inner.b);let b=self.pa(&x,&y);let d=self.pm(&outer.d,&inner.d);Tri{a,b,d}}
 fn vc(&mut self,outer:&Val,inner:&Val)->Val{let a=self.qm(&outer.a,&inner.a);let x=self.qm(&outer.b,&inner.a);let y=self.qm(&outer.d,&inner.b);let b=self.qa(&x,&y);let d=self.qm(&outer.d,&inner.d);Val{a,b,d}}
 fn eval(&mut self,p:&P,x:u64)->Q{let mut v=vec![0];for c in p.iter().rev(){v=self.scale(&v,x);v=self.qa(&v,c);}v}
 fn ve(&mut self,p:&Tri,x:u64)->Val{Val{a:self.eval(&p.a,x),b:self.eval(&p.b,x),d:self.eval(&p.d,x)}}
 fn leaf(&mut self,m:usize,j:usize,v:&Q)->Tri{
  let r=self.r;let h=(r-1)/2;let mut alpha=vec![if m%2==0{1}else{r-1}];let mut delta=vec![1];
  for t in 0..m{let offset=(m*j+t)as u64;alpha=self.p.mul(&alpha,&[(h+r-1-offset%r)%r,r-m as u64]);delta=self.p.mul(&delta,&[(2+offset)%r,m as u64]);}
  let a:P=alpha.into_iter().map(|c|self.scale(v,c)).collect();let d:P=delta.into_iter().map(|c|vec![c]).collect();Tri{b:a.clone(),a,d}
 }
 fn block(&mut self,m:usize,first:usize,len:usize,v:&Q)->Tri{
  if len==1{return self.leaf(m,first,v)}let left=self.block(m,first,len/2,v);let right=self.block(m,first+len/2,len-len/2,v);self.compose(&right,&left)
 }
 fn blocked(&mut self,m:usize,v:&Q,block:usize,degree_cap:usize)->Result<(Q,usize),&'static str>{
  let h=(self.r-1)/2;
  if m<3||m as u64>h||h%m as u64!=0||block==0||v.iter().any(|&x|x>=self.r){return Err("domain")}
  let l=(h/m as u64)as usize;let n=l-1;let c1=if h%2==1{h}else{self.r-h};if n==0{return Ok((vec![c1],0))}
  let block=block.min(n);let degree=m.checked_mul(block).ok_or("degree")?;if degree>degree_cap{return Err("degree")}
  let template=self.block(m,0,block,v);assert_eq!(template.d.len()-1,degree);
  let mut acc=Val{a:vec![1],b:vec![0],d:vec![1]};let complete=n/block;
  for j in 0..complete{let next=self.ve(&template,(j*block)as u64);acc=self.vc(&next,&acc);}
  for j in complete*block..n{let leaf=self.leaf(m,j,v);let next=self.ve(&leaf,0);acc=self.vc(&next,&acc);}
  if acc.d.len()!=1||acc.d[0]==0{return Err("denominator")}
  let inverse=pow(acc.d[0],self.r-2,self.r);if mul(inverse,acc.d[0],self.r)!=1{return Err("inverse")}
  let sum=self.qa(&acc.b,&acc.d);Ok((self.scale(&sum,mul(c1,inverse,self.r)),degree))
 }
}
include!("checks.rs");
include!("power.rs");
fn main(){checks();section_checks()}
