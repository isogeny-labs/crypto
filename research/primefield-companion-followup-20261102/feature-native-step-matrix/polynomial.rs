//! Native source polynomial engine: Karatsuba, Newton reversal division,
//! product trees and remainder-tree evaluation. Prime modulus is caller-certified.
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Nonunit,Order,ZeroDivisor,EmptyTree,Invariant}
pub struct Poly{p:u64,pub karatsuba_calls:usize,pub newton_steps:usize}
pub struct Tree{p:u64,polynomial:Vec<u64>,children:Option<(Box<Tree>,Box<Tree>)>}
impl Tree{pub fn polynomial(&self)->&[u64]{&self.polynomial}}
/// Multiplicative quotient representative, with zero adjoined. No additive API
/// and no raw-field accessor: this is not the full resultant.
#[derive(Debug,Clone,Copy)]
pub struct ResultantClass { p:u64, representative:u64 }
#[cfg(test)]std::thread_local!{static PROJECTIONS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn reset_projections(){PROJECTIONS.with(|n|n.set(0));}
#[cfg(test)]pub fn projections()->usize{PROJECTIONS.with(|n|n.get())}
impl ResultantClass{
 pub fn power(self,exponent:u64)->Self{
  if self.representative==0{return if exponent==0{Self{p:self.p,representative:1}}else{self}}
  let q=1u64<<(self.p-1).trailing_zeros();let(mut e,mut a,mut v)=(exponent%q,self.representative,1u64);
  while e>0{if e&1!=0{v=((v as u128*a as u128)%self.p as u128)as u64}a=((a as u128*a as u128)%self.p as u128)as u64;e>>=1}Self{p:self.p,representative:v}
 }
 pub fn inverse(self)->Result<Self,Error>{if self.is_zero(){return Err(Error::Nonunit)}let q=1u64<<(self.p-1).trailing_zeros();Ok(self.power(q-1))}

 pub fn combine(self,other:Self)->Result<Self,Error>{if self.p!=other.p{return Err(Error::Domain);}Ok(Self{p:self.p,representative:((self.representative as u128*other.representative as u128)%self.p as u128)as u64})}
 pub fn square(self)->Self{Self{p:self.p,representative:((self.representative as u128*self.representative as u128)%self.p as u128)as u64}}
 pub fn negate(self)->Self{Self{p:self.p,representative:(self.p-self.representative)%self.p}}
 pub fn is_zero(&self)->bool{self.representative==0}
 pub fn project(self)->u64{
  #[cfg(test)]PROJECTIONS.with(|n|n.set(n.get()+1));
  let order=1u64<<(self.p-1).trailing_zeros();let(mut a,mut e,mut v)=(self.representative,(self.p-1)/order,1u64);
  while e!=0{if e&1!=0{v=((v as u128*a as u128)%self.p as u128)as u64;}a=((a as u128*a as u128)%self.p as u128)as u64;e>>=1;}v
 }
}
type Matrix=[[Vec<u64>;2];2];
#[derive(Clone,Debug,PartialEq)]
struct Transition{m:usize,n:usize,k:Option<usize>,leading:u64}
struct Batch{matrix:Matrix,a:Vec<u64>,b:Vec<u64>,steps:Vec<Transition>}
fn degree(a:&[u64])->Option<usize>{if a==[0]{None}else{Some(a.len()-1)}}
fn identity()->Matrix{[[vec![1],vec![0]],[vec![0],vec![1]]]}
#[cfg(test)]std::thread_local!{static BATCH_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn reset_batches(){BATCH_CALLS.with(|n|n.set(0));}
#[cfg(test)]pub fn batches()->usize{BATCH_CALLS.with(|n|n.get())}
#[cfg(resultant_audit)]std::thread_local!{static AUDIT:std::cell::Cell<[usize;3]>=const{std::cell::Cell::new([0;3])};}
#[cfg(resultant_audit)]pub fn reset_audit(){AUDIT.with(|x|x.set([0;3]));}
#[cfg(resultant_audit)]pub fn audit()->[usize;3]{AUDIT.with(|x|x.get())}
#[cfg(resultant_audit)]fn bump(i:usize){AUDIT.with(|x|{let mut a=x.get();a[i]+=1;x.set(a)});}
impl Poly{
 pub fn new(p:u64)->Result<Self,Error>{if p<3||p>u64::MAX/2{return Err(Error::Domain);}Ok(Self{p,karatsuba_calls:0,newton_steps:0})}
 fn fm(&self,a:u64,b:u64)->u64{#[cfg(resultant_audit)]bump(0);((a as u128*b as u128)%self.p as u128)as u64}
 fn inverse(&self,a:u64)->Result<u64,Error>{#[cfg(resultant_audit)]bump(1);let(mut r,mut s,mut x,mut y)=(self.p as i128,(a%self.p)as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r==1{Ok(x.rem_euclid(self.p as i128)as u64)}else{Err(Error::Nonunit)}}
 fn trim(&self,a:&[u64])->Vec<u64>{let mut a:Vec<_>=a.iter().map(|x|x%self.p).collect();while a.len()>1&&a.last()==Some(&0){a.pop();}if a.is_empty(){a.push(0);}a}
 pub fn add(&self,a:&[u64],b:&[u64])->Vec<u64>{let(a,b)=(self.trim(a),self.trim(b));let mut c=vec![0;a.len().max(b.len())];for(i,x)in a.iter().enumerate(){c[i]=*x;}for(i,x)in b.iter().enumerate(){c[i]=(c[i]+x)%self.p;}self.trim(&c)}
 pub fn sub(&self,a:&[u64],b:&[u64])->Vec<u64>{let b:Vec<_>=b.iter().map(|x|(self.p-x%self.p)%self.p).collect();self.add(a,&b)}
 pub fn mul(&mut self,a:&[u64],b:&[u64])->Vec<u64>{
  let(a,b)=(self.trim(a),self.trim(b));if a==[0]||b==[0]{return vec![0];}
  if a.len().min(b.len())<=12{let mut c=vec![0;a.len()+b.len()-1];for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){c[i+j]=(c[i+j]+self.fm(x,y))%self.p;}}return self.trim(&c);}
  self.karatsuba_calls+=1;let k=(a.len().max(b.len())+1)/2;let(a0,a1)=a.split_at(k.min(a.len()));let(b0,b1)=b.split_at(k.min(b.len()));
  let lo=self.mul(a0,b0);let hi=self.mul(a1,b1);let aa=self.add(a0,a1);let bb=self.add(b0,b1);let cross=self.mul(&aa,&bb);let middle=self.sub(&self.sub(&cross,&lo),&hi);
  let mut shifted=vec![0;k];shifted.extend(middle);let left=self.add(&lo,&shifted);let mut shifted=vec![0;2*k];shifted.extend(hi);self.add(&left,&shifted)
 }
 /// Canonical low n coefficients. n=0 represents zero modulo X^0 as [0].
 pub fn mul_low(&mut self,a:&[u64],b:&[u64],n:usize)->Vec<u64>{
  if n==0{return vec![0];}
  let(a,b)=(self.trim(&a[..a.len().min(n)]),self.trim(&b[..b.len().min(n)]));
  if a==[0]||b==[0]{return vec![0];}
  let length=a.len()+b.len()-1;
  if n>=length{return self.mul(&a,&b);}
  if a.len().min(b.len())<=12{
   let mut c=vec![0;n];
   for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().take(n-i).enumerate(){c[i+j]=(c[i+j]+self.fm(x,y))%self.p;}}
   return self.trim(&c);
  }
  self.karatsuba_calls+=1;
  let k=(a.len().max(b.len())+1)/2;
  let(a0,a1)=a.split_at(k.min(a.len()));let(b0,b1)=b.split_at(k.min(b.len()));
  if n<=k{return self.mul_low(a0,b0,n);}
  let lo=self.mul_low(a0,b0,n);
  // hi is required to form the cross term even when its X^(2k) shift vanishes.
  let hi=self.mul_low(a1,b1,n-k);
  let aa=self.add(a0,a1);let bb=self.add(b0,b1);
  let cross=self.mul_low(&aa,&bb,n-k);
  let middle=self.sub(&self.sub(&cross,&lo[..lo.len().min(n-k)]),&hi);
  let mut result=vec![0;n];
  for(i,&v)in lo.iter().take(n).enumerate(){result[i]=v;}
  for(i,&v)in middle.iter().take(n-k).enumerate(){result[k+i]=(result[k+i]+v)%self.p;}
  if 2*k<n{for(i,&v)in hi.iter().take(n-2*k).enumerate(){result[2*k+i]=(result[2*k+i]+v)%self.p;}}
  self.trim(&result)
 }
 pub fn inv_series(&mut self,a:&[u64],n:usize)->Result<Vec<u64>,Error>{
  if n==0{return Err(Error::Order);}let a=self.trim(a);let mut v=vec![if a[0]==1{1}else{self.inverse(a[0])?}];let mut k=1;
  while k<n{k=k.saturating_mul(2).min(n);let mut av=self.mul_low(&a[..a.len().min(k)],&v,k);av.truncate(k);let correction=self.sub(&[2],&av);let mut next=self.mul_low(&v,&correction,k);next.truncate(k);v=self.trim(&next);self.newton_steps+=1;}
  Ok(v)
 }
 pub fn divrem(&mut self,a:&[u64],b:&[u64])->Result<(Vec<u64>,Vec<u64>),Error>{
  let(a,b)=(self.trim(a),self.trim(b));if b==[0]{return Err(Error::ZeroDivisor);}if a.len()<b.len(){return Ok((vec![0],a));}
  let n=a.len()-b.len()+1;let rev_a:Vec<_>=a.iter().rev().take(n).copied().collect();let rev_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&rev_b,n)?;let mut reverse_q=self.mul_low(&rev_a,&inv,n);reverse_q.truncate(n);reverse_q.resize(n,0);reverse_q.reverse();let q=self.trim(&reverse_q);let qb=self.mul(&q,&b);let rem=self.sub(&a,&qb);
  if rem!=[0]&&rem.len()>=b.len(){return Err(Error::Invariant);}Ok((q,rem))
 }
 pub fn product_tree(&mut self,points:&[u64])->Result<Tree,Error>{
  if points.is_empty(){return Err(Error::EmptyTree);}if points.len()==1{return Ok(Tree{p:self.p,polynomial:vec![(self.p-points[0]%self.p)%self.p,1],children:None});}
  let k=points.len()/2;let left=self.product_tree(&points[..k])?;let right=self.product_tree(&points[k..])?;let polynomial=self.mul(&left.polynomial,&right.polynomial);Ok(Tree{p:self.p,polynomial,children:Some((Box::new(left),Box::new(right)))})
 }
 /// Internal tree divisors are canonical and monic. The quotient is still
 /// obtained by Newton reversal; only coefficients below deg(b) are materialized
 /// for a-q*b. General divrem remains unchanged as an independent reference path.
 fn monic_remainder(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{
  if b.last()!=Some(&1){return Err(Error::Domain);}let a=self.trim(a);let d=b.len()-1;if d==0{return Ok(vec![0]);}if a.len()<b.len(){return Ok(a);}
  let n=a.len()-b.len()+1;let reverse_a:Vec<_>=a.iter().rev().take(n).copied().collect();let reverse_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&reverse_b,n)?;let mut q=self.mul_low(&reverse_a,&inv,n);q.truncate(n);q.resize(n,0);q.reverse();
  let mut low=self.mul_low(&q[..q.len().min(d)],&b[..d],d);low.truncate(d);let mut rem=Vec::with_capacity(d);
  for i in 0..d{rem.push((a.get(i).copied().unwrap_or(0)+self.p-low.get(i).copied().unwrap_or(0))%self.p);}Ok(self.trim(&rem))
 }
 pub fn values(&mut self,a:&[u64],tree:&Tree)->Result<Vec<u64>,Error>{
  if tree.p!=self.p{return Err(Error::Domain);}
  if tree.children.is_none(){let x=(self.p-tree.polynomial[0])%self.p;let mut coefficients=a.iter().rev();let mut value=coefficients.next().map_or(0,|c|c%self.p);for &coefficient in coefficients{value=(self.fm(value,x)+coefficient%self.p)%self.p;}return Ok(vec![value]);}
  let rem=self.monic_remainder(a,&tree.polynomial)?;let(left,right)=tree.children.as_ref().ok_or(Error::Invariant)?;let mut v=self.values(&rem,left)?;v.extend(self.values(&rem,right)?);Ok(v)
 }
 /// Product of evaluations, retaining full-field remainders and scalar returns.
 /// Standard Res(A,B), projected to the 2-primary image of order at most four.
 fn matrix_mul(&mut self,a:&Matrix,b:&Matrix)->Matrix{
  std::array::from_fn(|i|std::array::from_fn(|j|{let x=self.mul(&a[i][0],&b[0][j]);let y=self.mul(&a[i][1],&b[1][j]);self.add(&x,&y)}))
 }
 fn matrix_apply(&mut self,m:&Matrix,a:&[u64],b:&[u64])->(Vec<u64>,Vec<u64>){
  let x=self.mul(&m[0][0],a);let y=self.mul(&m[0][1],b);let c=self.add(&x,&y);
  let x=self.mul(&m[1][0],a);let y=self.mul(&m[1][1],b);let d=self.add(&x,&y);(c,d)
 }
 /// Left Euclidean action on a canonical matrix; move the retained row.
 fn step_left(&mut self,q:&[u64],matrix:Matrix)->Matrix{
  #[cfg(matrix_audit)]STEP_AUDIT.with(|a|{let mut v=a.get();v[0]+=1;v[1]+=matrix.iter().flatten().map(|c|if c==&[0]{0}else{c.len()}).sum::<usize>();a.set(v)});
  let [[a,b],[c,d]]=matrix;
  let qc=self.mul(q,&c);let qd=self.mul(q,&d);
  let left=self.sub(&a,&qc);let right=self.sub(&b,&qd);
  [[c,d],[left,right]]
 }
 #[cfg(matrix_audit)]
 fn step_matrix(&self,q:&[u64])->Matrix{[[vec![0],vec![1]],[vec![1],self.sub(&[0],q)]]}
 fn lifted(&mut self,mut batch:Batch,a:&[u64],b:&[u64],shift:usize)->Result<Batch,Error>{
  let(c,d)=self.matrix_apply(&batch.matrix,a,b);let len=batch.steps.len();
  for(i,t)in batch.steps.iter_mut().enumerate(){t.m+=shift;t.n+=shift;if i+1<len{t.k=Some(t.k.ok_or(Error::Invariant)?+shift);}else{t.k=degree(&d);}}
  if let Some(last)=batch.steps.last(){if degree(&c)!=Some(last.n)||c.last()!=Some(&last.leading){return Err(Error::Invariant);}}
  if d!=[0]&&c.len()<=d.len(){return Err(Error::Invariant);}
  batch.a=c;batch.b=d;Ok(batch)
 }
 fn half_gcd(&mut self,a:&[u64],b:&[u64])->Result<Batch,Error>{
  let d=a.len()-1;let half=(d+1)/2;
  if b==[0]||b.len()-1<half{return Ok(Batch{matrix:identity(),a:a.to_vec(),b:b.to_vec(),steps:vec![]});}
  if a.len()<=b.len(){return Err(Error::Invariant);}
  if d<=8{
   let(mut a,mut b,mut matrix,mut steps)=(a.to_vec(),b.to_vec(),identity(),vec![]);
   while b!=[0]&&b.len()-1>=half{let(q,r)=self.divrem(&a,&b)?;steps.push(Transition{m:a.len()-1,n:b.len()-1,k:degree(&r),leading:*b.last().unwrap()});matrix=self.step_left(&q,matrix);a=b;b=r;}
   return Ok(Batch{matrix,a,b,steps});
  }
  #[cfg(test)]BATCH_CALLS.with(|n|n.set(n.get()+1));
  let small=self.half_gcd(&a[half..],&b[half..])?;let mut first=self.lifted(small,a,b,half)?;
  if first.b==[0]||first.b.len()-1<half{return Ok(first);}
  let(q,r)=self.divrem(&first.a,&first.b)?;
  first.steps.push(Transition{m:first.a.len()-1,n:first.b.len()-1,k:degree(&r),leading:*first.b.last().unwrap()});
  let prefix=self.step_left(&q,first.matrix);
  if r==[0]||r.len()-1<half{return Ok(Batch{matrix:prefix,a:first.b,b:r,steps:first.steps});}
  let shift=2*half-(first.b.len()-1);
  let small=self.half_gcd(&first.b[shift..],&r[shift..])?;let second=self.lifted(small,&first.b,&r,shift)?;
  let matrix=self.matrix_mul(&second.matrix,&prefix);first.steps.extend(second.steps);
  if second.b!=[0]&&second.b.len()-1>=half{return Err(Error::Invariant);}
  Ok(Batch{matrix,a:second.a,b:second.b,steps:first.steps})
 }
 fn class_factor(&self,acc:u64,base:u64,exponent:u64)->u64{let factor=ResultantClass{p:self.p,representative:base}.power(exponent);((acc as u128*factor.representative as u128)%self.p as u128)as u64}
 pub fn resultant_class(&mut self,a:&[u64],b:&[u64])->Result<ResultantClass,Error>{
  let order=1u64<<(self.p-1).trailing_zeros();
  let(mut a,mut b)=(self.trim(a),self.trim(b));if a==[0]||b==[0]{return self.image_class(0);}
  let(mut acc,mut negative)=(1,false);
  loop{
   let(m,n)=(a.len()-1,b.len()-1);
   if n==0{acc=self.class_factor(acc,b[0],m as u64%order);break;}
   if m<n{if m&1==1&&n&1==1{negative=!negative;}std::mem::swap(&mut a,&mut b);continue;}
   let(next_a,next_b,steps)=if m>8&&m>n&&n>=(m+1)/2{
    let batch=self.half_gcd(&a,&b)?;if batch.steps.is_empty(){return Err(Error::Invariant);}(batch.a,batch.b,batch.steps)
   }else{let(_,r)=self.divrem(&a,&b)?;let step=Transition{m,n,k:degree(&r),leading:*b.last().unwrap()};(b,r,vec![step])};
   for t in steps{let Some(k)=t.k else{return self.image_class(0);};if k>=t.n||t.n>t.m{return Err(Error::Invariant);}
    acc=self.class_factor(acc,t.leading,(t.m-k)as u64%order);if t.m&1==1&&t.n&1==1{negative=!negative;}
   }
   a=next_a;b=next_b;
  }
  let class=self.image_class(acc)?;Ok(if negative{class.negate()}else{class})
 }
 pub fn image_class(&self,v:u64)->Result<ResultantClass,Error>{Ok(ResultantClass{p:self.p,representative:v%self.p})}
 pub fn projected_resultant(&mut self,a:&[u64],b:&[u64])->Result<u64,Error>{Ok(self.resultant_class(a,b)?.project())}
 /// Build only the product polynomial: no retained subproduct tree nodes.
 pub fn root_product(&mut self,roots:&[u64])->Result<Vec<u64>,Error>{#[cfg(resultant_audit)]bump(2);
  if roots.is_empty(){return Err(Error::EmptyTree);}
  if roots.len()==1{return Ok(vec![(self.p-roots[0]%self.p)%self.p,1]);}
  let k=roots.len()/2;let a=self.root_product(&roots[..k])?;let b=self.root_product(&roots[k..])?;Ok(self.mul(&a,&b))
 }
 /// Res(product(X-point), product(X-root)); stream singleton factors.
 pub fn root_resultant_class(&mut self,roots:&[u64],points:&[u64])->Result<ResultantClass,Error>{
  if roots.is_empty()||points.is_empty(){return Err(Error::EmptyTree);}
  if points.len()==1{
   let y=points[0]%self.p;let mut v=(y+self.p-roots[0]%self.p)%self.p;
   for &x in &roots[1..]{v=self.fm(v,(y+self.p-x%self.p)%self.p);}
   return self.image_class(v);
  }
  if roots.len()==1{
   let x=roots[0]%self.p;let mut v=(points[0]%self.p+self.p-x)%self.p;
   for &y in &points[1..]{v=self.fm(v,(y%self.p+self.p-x)%self.p);}
   return self.image_class(v);
  }
  let f=self.root_product(roots)?;let g=self.root_product(points)?;self.resultant_class(&g,&f)
 }
 pub fn root_projected_resultant(&mut self,roots:&[u64],points:&[u64])->Result<u64,Error>{
  Ok(self.root_resultant_class(roots,points)?.project())
 }
 pub fn value_product(&mut self,a:&[u64],tree:&Tree)->Result<u64,Error>{
  if tree.p!=self.p{return Err(Error::Domain);}
  if tree.children.is_none(){let x=(self.p-tree.polynomial[0])%self.p;let mut coefficients=a.iter().rev();let mut value=coefficients.next().map_or(0,|c|c%self.p);for &coefficient in coefficients{value=(self.fm(value,x)+coefficient%self.p)%self.p;}return Ok(value);}
  let rem=self.monic_remainder(a,&tree.polynomial)?;
  let(left,right)=tree.children.as_ref().ok_or(Error::Invariant)?;
  let l=self.value_product(&rem,left)?;let r=self.value_product(&rem,right)?;
  Ok(self.fm(l,r))
 }
 pub fn root_resultant(&mut self,roots:&[u64],points:&[u64])->Result<u64,Error>{let source=self.product_tree(roots)?;let destination=self.product_tree(points)?;self.value_product(source.polynomial(),&destination)}
}

#[cfg(matrix_audit)]std::thread_local!{static STEP_AUDIT:std::cell::Cell<[usize;2]>=const{std::cell::Cell::new([0;2])};}
#[cfg(matrix_audit)]include!("matrix-checks.rs");
