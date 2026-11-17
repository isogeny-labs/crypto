//! Native source polynomial engine: Karatsuba, Newton reversal division,
//! product trees and remainder-tree evaluation. Prime modulus is caller-certified.
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Nonunit,Order,ZeroDivisor,EmptyTree,Invariant}
pub struct Poly{p:u64,pub karatsuba_calls:usize,pub newton_steps:usize}
pub struct Tree{p:u64,polynomial:Vec<u64>,children:Option<(Box<Tree>,Box<Tree>)>}
impl Tree{pub fn polynomial(&self)->&[u64]{crate::meter::hit(41);&self.polynomial}}
/// Multiplicative quotient representative, with zero adjoined. No additive API
/// and no raw-field accessor: this is not the full resultant.
#[derive(Debug)]
pub struct ResultantClass { p:u64, representative:u64 }
#[cfg(test)]std::thread_local!{static PROJECTIONS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn reset_projections(){crate::meter::hit(42);PROJECTIONS.with(|n|n.set(0));}
#[cfg(test)]pub fn projections()->usize{crate::meter::hit(43);PROJECTIONS.with(|n|n.get())}
impl ResultantClass{
 pub fn combine(self,other:Self)->Result<Self,Error>{crate::meter::hit(23);if self.p!=other.p{return Err(Error::Domain);}Ok(Self{p:self.p,representative:{crate::meter::hit(0);((self.representative as u128*other.representative as u128)%self.p as u128)as u64}})}
 pub fn square(self)->Self{crate::meter::hit(24);Self{p:self.p,representative:{crate::meter::hit(0);((self.representative as u128*self.representative as u128)%self.p as u128)as u64}}}
 pub fn negate(self)->Self{crate::meter::hit(25);Self{p:self.p,representative:{crate::meter::hit(3);(self.p-self.representative)%self.p}}}
 pub fn is_zero(&self)->bool{crate::meter::hit(26);self.representative==0}
 pub fn project(self)->u64{crate::meter::hit(27);crate::meter::hit(7);
  #[cfg(test)]PROJECTIONS.with(|n|n.set(n.get()+1));
  let order=1u64<<(self.p-1).trailing_zeros();let(mut a,mut e,mut v)=(self.representative,(self.p-1)/order,1u64);
  while e!=0{if e&1!=0{v={crate::meter::hit(0);((v as u128*a as u128)%self.p as u128)as u64};}a={crate::meter::hit(0);((a as u128*a as u128)%self.p as u128)as u64};e>>=1;}v
 }
}
type Matrix=[[Vec<u64>;2];2];
#[derive(Clone,Debug,PartialEq)]
struct Transition{m:usize,n:usize,k:Option<usize>,leading:u64}
struct Batch{matrix:Matrix,a:Vec<u64>,b:Vec<u64>,steps:Vec<Transition>}
fn degree(a:&[u64])->Option<usize>{crate::meter::hit(40);if a==[0]{None}else{Some(a.len()-1)}}
fn identity()->Matrix{crate::meter::hit(39);[[vec![1],vec![0]],[vec![0],vec![1]]]}
#[cfg(test)]std::thread_local!{static BATCH_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn reset_batches(){crate::meter::hit(44);BATCH_CALLS.with(|n|n.set(0));}
#[cfg(test)]pub fn batches()->usize{crate::meter::hit(45);BATCH_CALLS.with(|n|n.get())}
impl Poly{
 pub fn new(p:u64)->Result<Self,Error>{crate::meter::hit(8);if p<3||p>u64::MAX/2{return Err(Error::Domain);}Ok(Self{p,karatsuba_calls:0,newton_steps:0})}
 fn fm(&self,a:u64,b:u64)->u64{crate::meter::hit(9);{crate::meter::hit(0);((a as u128*b as u128)%self.p as u128)as u64}}
 fn inverse(&self,a:u64)->Result<u64,Error>{crate::meter::hit(10);crate::meter::hit(5);let(mut r,mut s,mut x,mut y)=(self.p as i128,({crate::meter::hit(4);a%self.p})as i128,0i128,1i128);while s!=0{crate::meter::hit(6);let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r==1{Ok({crate::meter::hit(4);x.rem_euclid(self.p as i128)}as u64)}else{Err(Error::Nonunit)}}
 fn trim(&self,a:&[u64])->Vec<u64>{crate::meter::hit(11);let mut a:Vec<_>=a.iter().map(|x|{crate::meter::hit(4);x%self.p}).collect();while a.len()>1&&a.last()==Some(&0){a.pop();}if a.is_empty(){a.push(0);}a}
 pub fn add(&self,a:&[u64],b:&[u64])->Vec<u64>{crate::meter::hit(12);let(a,b)=(self.trim(a),self.trim(b));let mut c=vec![0;a.len().max(b.len())];for(i,x)in a.iter().enumerate(){c[i]=*x;}for(i,x)in b.iter().enumerate(){c[i]={crate::meter::hit(1);(c[i]+x)%self.p};}self.trim(&c)}
 pub fn sub(&self,a:&[u64],b:&[u64])->Vec<u64>{crate::meter::hit(13);let b:Vec<_>=b.iter().map(|x|{crate::meter::hit(4);crate::meter::hit(3);(self.p-x%self.p)%self.p}).collect();self.add(a,&b)}
 pub fn mul(&mut self,a:&[u64],b:&[u64])->Vec<u64>{crate::meter::hit(14);
  let(a,b)=(self.trim(a),self.trim(b));if a==[0]||b==[0]{return vec![0];}
  if a.len().min(b.len())<=12{let mut c=vec![0;a.len()+b.len()-1];for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){c[i+j]={crate::meter::hit(1);(c[i+j]+self.fm(x,y))%self.p};}}return self.trim(&c);}
  self.karatsuba_calls+=1;let k=(a.len().max(b.len())+1)/2;let(a0,a1)=a.split_at(k.min(a.len()));let(b0,b1)=b.split_at(k.min(b.len()));
  let lo=self.mul(a0,b0);let hi=self.mul(a1,b1);let aa=self.add(a0,a1);let bb=self.add(b0,b1);let cross=self.mul(&aa,&bb);let middle=self.sub(&self.sub(&cross,&lo),&hi);
  let mut shifted=vec![0;k];shifted.extend(middle);let left=self.add(&lo,&shifted);let mut shifted=vec![0;2*k];shifted.extend(hi);self.add(&left,&shifted)
 }
 /// Canonical low n coefficients. n=0 represents zero modulo X^0 as [0].
 pub fn mul_low(&mut self,a:&[u64],b:&[u64],n:usize)->Vec<u64>{crate::meter::hit(15);
  if n==0{return vec![0];}
  let(a,b)=(self.trim(&a[..a.len().min(n)]),self.trim(&b[..b.len().min(n)]));
  if a==[0]||b==[0]{return vec![0];}
  let length=a.len()+b.len()-1;
  if n>=length{return self.mul(&a,&b);}
  if a.len().min(b.len())<=12{
   let mut c=vec![0;n];
   for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().take(n-i).enumerate(){c[i+j]={crate::meter::hit(1);(c[i+j]+self.fm(x,y))%self.p};}}
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
  for(i,&v)in middle.iter().take(n-k).enumerate(){result[k+i]={crate::meter::hit(1);(result[k+i]+v)%self.p};}
  if 2*k<n{for(i,&v)in hi.iter().take(n-2*k).enumerate(){result[2*k+i]={crate::meter::hit(1);(result[2*k+i]+v)%self.p};}}
  self.trim(&result)
 }
 pub fn inv_series(&mut self,a:&[u64],n:usize)->Result<Vec<u64>,Error>{crate::meter::hit(16);
  if n==0{return Err(Error::Order);}let a=self.trim(a);let mut v=vec![if a[0]==1{1}else{self.inverse(a[0])?}];let mut k=1;
  while k<n{k=k.saturating_mul(2).min(n);let mut av=self.mul_low(&a[..a.len().min(k)],&v,k);av.truncate(k);let correction=self.sub(&[2],&av);let mut next=self.mul_low(&v,&correction,k);next.truncate(k);v=self.trim(&next);self.newton_steps+=1;}
  Ok(v)
 }
 pub fn divrem(&mut self,a:&[u64],b:&[u64])->Result<(Vec<u64>,Vec<u64>),Error>{crate::meter::hit(17);
  let(a,b)=(self.trim(a),self.trim(b));if b==[0]{return Err(Error::ZeroDivisor);}if a.len()<b.len(){return Ok((vec![0],a));}
  let n=a.len()-b.len()+1;let rev_a:Vec<_>=a.iter().rev().take(n).copied().collect();let rev_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&rev_b,n)?;let mut reverse_q=self.mul_low(&rev_a,&inv,n);reverse_q.truncate(n);reverse_q.resize(n,0);reverse_q.reverse();let q=self.trim(&reverse_q);let qb=self.mul(&q,&b);let rem=self.sub(&a,&qb);
  if rem!=[0]&&rem.len()>=b.len(){return Err(Error::Invariant);}Ok((q,rem))
 }
 pub fn product_tree(&mut self,points:&[u64])->Result<Tree,Error>{crate::meter::hit(18);
  if points.is_empty(){return Err(Error::EmptyTree);}if points.len()==1{return Ok(Tree{p:self.p,polynomial:vec![{crate::meter::hit(4);crate::meter::hit(3);(self.p-points[0]%self.p)%self.p},1],children:None});}
  let k=points.len()/2;let left=self.product_tree(&points[..k])?;let right=self.product_tree(&points[k..])?;let polynomial=self.mul(&left.polynomial,&right.polynomial);Ok(Tree{p:self.p,polynomial,children:Some((Box::new(left),Box::new(right)))})
 }
 /// Internal tree divisors are canonical and monic. The quotient is still
 /// obtained by Newton reversal; only coefficients below deg(b) are materialized
 /// for a-q*b. General divrem remains unchanged as an independent reference path.
 fn monic_remainder(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{crate::meter::hit(19);
  if b.last()!=Some(&1){return Err(Error::Domain);}let a=self.trim(a);let d=b.len()-1;if d==0{return Ok(vec![0]);}if a.len()<b.len(){return Ok(a);}
  let n=a.len()-b.len()+1;let reverse_a:Vec<_>=a.iter().rev().take(n).copied().collect();let reverse_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&reverse_b,n)?;let mut q=self.mul_low(&reverse_a,&inv,n);q.truncate(n);q.resize(n,0);q.reverse();
  let mut low=self.mul_low(&q[..q.len().min(d)],&b[..d],d);low.truncate(d);let mut rem=Vec::with_capacity(d);
  for i in 0..d{rem.push({crate::meter::hit(2);(a.get(i).copied().unwrap_or(0)+self.p-low.get(i).copied().unwrap_or(0))%self.p});}Ok(self.trim(&rem))
 }
 pub fn values(&mut self,a:&[u64],tree:&Tree)->Result<Vec<u64>,Error>{crate::meter::hit(20);
  if tree.p!=self.p{return Err(Error::Domain);}
  if tree.children.is_none(){let x={crate::meter::hit(3);(self.p-tree.polynomial[0])%self.p};let mut coefficients=a.iter().rev();let mut value=coefficients.next().map_or(0,|c|{crate::meter::hit(4);c%self.p});for &coefficient in coefficients{value={crate::meter::hit(4);crate::meter::hit(1);(self.fm(value,x)+coefficient%self.p)%self.p};}return Ok(vec![value]);}
  let rem=self.monic_remainder(a,&tree.polynomial)?;let(left,right)=tree.children.as_ref().ok_or(Error::Invariant)?;let mut v=self.values(&rem,left)?;v.extend(self.values(&rem,right)?);Ok(v)
 }
 /// Product of evaluations, retaining full-field remainders and scalar returns.
 /// Standard Res(A,B), projected to the 2-primary image of order at most four.
 fn matrix_mul(&mut self,a:&Matrix,b:&Matrix)->Matrix{crate::meter::hit(34);
  std::array::from_fn(|i|std::array::from_fn(|j|{let x=self.mul(&a[i][0],&b[0][j]);let y=self.mul(&a[i][1],&b[1][j]);self.add(&x,&y)}))
 }
 fn matrix_apply(&mut self,m:&Matrix,a:&[u64],b:&[u64])->(Vec<u64>,Vec<u64>){crate::meter::hit(35);
  let x=self.mul(&m[0][0],a);let y=self.mul(&m[0][1],b);let c=self.add(&x,&y);
  let x=self.mul(&m[1][0],a);let y=self.mul(&m[1][1],b);let d=self.add(&x,&y);(c,d)
 }
 fn step_matrix(&self,q:&[u64])->Matrix{crate::meter::hit(36);[[vec![0],vec![1]],[vec![1],self.sub(&[0],q)]]}
 fn lifted(&mut self,mut batch:Batch,a:&[u64],b:&[u64],shift:usize)->Result<Batch,Error>{crate::meter::hit(37);
  let(c,d)=self.matrix_apply(&batch.matrix,a,b);let len=batch.steps.len();
  for(i,t)in batch.steps.iter_mut().enumerate(){t.m+=shift;t.n+=shift;if i+1<len{t.k=Some(t.k.ok_or(Error::Invariant)?+shift);}else{t.k=degree(&d);}}
  if let Some(last)=batch.steps.last(){if degree(&c)!=Some(last.n)||c.last()!=Some(&last.leading){return Err(Error::Invariant);}}
  if d!=[0]&&c.len()<=d.len(){return Err(Error::Invariant);}
  batch.a=c;batch.b=d;Ok(batch)
 }
 fn half_gcd(&mut self,a:&[u64],b:&[u64])->Result<Batch,Error>{crate::meter::hit(38);
  let d=a.len()-1;let half=(d+1)/2;
  if b==[0]||b.len()-1<half{return Ok(Batch{matrix:identity(),a:a.to_vec(),b:b.to_vec(),steps:vec![]});}
  if a.len()<=b.len(){return Err(Error::Invariant);}
  if d<=8{
   let(mut a,mut b,mut matrix,mut steps)=(a.to_vec(),b.to_vec(),identity(),vec![]);
   while b!=[0]&&b.len()-1>=half{let(q,r)=self.divrem(&a,&b)?;steps.push(Transition{m:a.len()-1,n:b.len()-1,k:degree(&r),leading:*b.last().unwrap()});let step=self.step_matrix(&q);matrix=self.matrix_mul(&step,&matrix);a=b;b=r;}
   return Ok(Batch{matrix,a,b,steps});
  }
  #[cfg(test)]BATCH_CALLS.with(|n|n.set(n.get()+1));
  let small=self.half_gcd(&a[half..],&b[half..])?;let mut first=self.lifted(small,a,b,half)?;
  if first.b==[0]||first.b.len()-1<half{return Ok(first);}
  let(q,r)=self.divrem(&first.a,&first.b)?;
  first.steps.push(Transition{m:first.a.len()-1,n:first.b.len()-1,k:degree(&r),leading:*first.b.last().unwrap()});
  let step=self.step_matrix(&q);let prefix=self.matrix_mul(&step,&first.matrix);
  if r==[0]||r.len()-1<half{return Ok(Batch{matrix:prefix,a:first.b,b:r,steps:first.steps});}
  let shift=2*half-(first.b.len()-1);
  let small=self.half_gcd(&first.b[shift..],&r[shift..])?;let second=self.lifted(small,&first.b,&r,shift)?;
  let matrix=self.matrix_mul(&second.matrix,&prefix);first.steps.extend(second.steps);
  if second.b!=[0]&&second.b.len()-1>=half{return Err(Error::Invariant);}
  Ok(Batch{matrix,a:second.a,b:second.b,steps:first.steps})
 }
 pub fn resultant_class(&mut self,a:&[u64],b:&[u64])->Result<ResultantClass,Error>{crate::meter::hit(28);
  let order=1u64<<(self.p-1).trailing_zeros();if order>4{return Err(Error::Domain);}
  let(mut a,mut b)=(self.trim(a),self.trim(b));if a==[0]||b==[0]{return self.image_class(0);}
  let(mut acc,mut negative)=(1,false);
  loop{
   let(m,n)=(a.len()-1,b.len()-1);
   if n==0{for _ in 0..(m as u64%order){acc=self.fm(acc,b[0]);}break;}
   if m<n{if m&1==1&&n&1==1{negative=!negative;}std::mem::swap(&mut a,&mut b);continue;}
   let(next_a,next_b,steps)=if m>8&&m>n&&n>=(m+1)/2{
    let batch=self.half_gcd(&a,&b)?;if batch.steps.is_empty(){return Err(Error::Invariant);}(batch.a,batch.b,batch.steps)
   }else{let(_,r)=self.divrem(&a,&b)?;let step=Transition{m,n,k:degree(&r),leading:*b.last().unwrap()};(b,r,vec![step])};
   for t in steps{let Some(k)=t.k else{return self.image_class(0);};if k>=t.n||t.n>t.m{return Err(Error::Invariant);}
    for _ in 0..((t.m-k)as u64%order){acc=self.fm(acc,t.leading);}if t.m&1==1&&t.n&1==1{negative=!negative;}
   }
   a=next_a;b=next_b;
  }
  let class=self.image_class(acc)?;Ok(if negative{class.negate()}else{class})
 }
 pub fn image_class(&self,v:u64)->Result<ResultantClass,Error>{crate::meter::hit(29);if 1u64<<(self.p-1).trailing_zeros()>4{return Err(Error::Domain);}Ok(ResultantClass{p:self.p,representative:{crate::meter::hit(4);v%self.p}})}
 pub fn projected_resultant(&mut self,a:&[u64],b:&[u64])->Result<u64,Error>{crate::meter::hit(30);Ok(self.resultant_class(a,b)?.project())}
 /// Build only the product polynomial: no retained subproduct tree nodes.
 pub fn root_product(&mut self,roots:&[u64])->Result<Vec<u64>,Error>{crate::meter::hit(31);
  if roots.is_empty(){return Err(Error::EmptyTree);}
  if roots.len()==1{return Ok(vec![{crate::meter::hit(4);crate::meter::hit(3);(self.p-roots[0]%self.p)%self.p},1]);}
  let k=roots.len()/2;let a=self.root_product(&roots[..k])?;let b=self.root_product(&roots[k..])?;Ok(self.mul(&a,&b))
 }
 pub fn root_resultant_class(&mut self,roots:&[u64],points:&[u64])->Result<ResultantClass,Error>{crate::meter::hit(32);let f=self.root_product(roots)?;let g=self.root_product(points)?;self.resultant_class(&g,&f)}
 pub fn root_projected_resultant(&mut self,roots:&[u64],points:&[u64])->Result<u64,Error>{crate::meter::hit(33);
  let f=self.root_product(roots)?;let g=self.root_product(points)?;
  // Existing source convention is product_j f(points[j]) = Res(g,f).
  self.projected_resultant(&g,&f)
 }
 pub fn value_product(&mut self,a:&[u64],tree:&Tree)->Result<u64,Error>{crate::meter::hit(21);
  if tree.p!=self.p{return Err(Error::Domain);}
  if tree.children.is_none(){let x={crate::meter::hit(3);(self.p-tree.polynomial[0])%self.p};let mut coefficients=a.iter().rev();let mut value=coefficients.next().map_or(0,|c|{crate::meter::hit(4);c%self.p});for &coefficient in coefficients{value={crate::meter::hit(4);crate::meter::hit(1);(self.fm(value,x)+coefficient%self.p)%self.p};}return Ok(value);}
  let rem=self.monic_remainder(a,&tree.polynomial)?;
  let(left,right)=tree.children.as_ref().ok_or(Error::Invariant)?;
  let l=self.value_product(&rem,left)?;let r=self.value_product(&rem,right)?;
  Ok(self.fm(l,r))
 }
 pub fn root_resultant(&mut self,roots:&[u64],points:&[u64])->Result<u64,Error>{crate::meter::hit(22);let source=self.product_tree(roots)?;let destination=self.product_tree(points)?;self.value_product(source.polynomial(),&destination)}
}
#[cfg(test)]mod tests{
 use super::*;
 fn trim(mut a:Vec<u64>)->Vec<u64>{while a.len()>1&&a.last()==Some(&0){a.pop();}if a.is_empty(){a.push(0);}a}
 fn school(a:&[u64],b:&[u64],p:u64)->Vec<u64>{if a.is_empty()||b.is_empty(){return vec![0];}let mut v=vec![0;a.len()+b.len()-1];for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){v[i+j]=((v[i+j]as u128+x as u128*y as u128)%p as u128)as u64;}}trim(v)}
 fn power(mut a:u64,mut n:u64,p:u64)->u64{let mut v=1;while n!=0{if n&1!=0{v=((v as u128*a as u128)%p as u128)as u64;}a=((a as u128*a as u128)%p as u128)as u64;n>>=1;}v}
 fn long_div(a:&[u64],b:&[u64],p:u64)->(Vec<u64>,Vec<u64>){let mut r=trim(a.to_vec());let b=trim(b.to_vec());if r.len()<b.len(){return(vec![0],r);}let mut q=vec![0;r.len()-b.len()+1];let inverse=power(*b.last().unwrap(),p-2,p);while r!=[0]&&r.len()>=b.len(){let k=r.len()-b.len();let c=*r.last().unwrap()*inverse%p;q[k]=c;for(j,&x)in b.iter().enumerate(){r[k+j]=(r[k+j]+p-c*x%p)%p;}r=trim(r);} (trim(q),r)}
 fn list(s:&str)->Vec<u64>{s.split(',').map(|x|x.parse().unwrap()).collect()}
 #[test]fn saved_source_resultants(){for line in include_str!("../feature-native-polynomial/fixtures.tsv").lines(){let f:Vec<_>=line.split('\t').collect();let mut p=Poly::new(f[0].parse().unwrap()).unwrap();assert_eq!(p.root_resultant(&list(f[2]),&list(f[3])).unwrap(),f[4].parse::<u64>().unwrap());let a=p.product_tree(&list(f[5])).unwrap();let b=p.product_tree(&[f[6].parse().unwrap()]).unwrap();assert_eq!(p.values(a.polynomial(),&b).unwrap(),vec![f[7].parse::<u64>().unwrap()]);}}
 #[test]fn multiplication_threshold_and_unbalanced(){let mut p=Poly::new(509).unwrap();for n in [1,12,13,25,67]{for m in [1,13,26]{let a:Vec<_>=(0..n).map(|i|((i*i+17)%509)as u64).collect();let b:Vec<_>=(0..m).map(|i|((3*i+8)%509)as u64).collect();assert_eq!(p.mul(&a,&b),school(&a,&b,509));}}assert!(p.karatsuba_calls>0);assert_eq!(p.mul(&[],&[1]),vec![0]);assert_eq!(p.mul(&[509,0],&[3]),vec![0]);}
 #[test]fn newton_and_long_division(){let mut p=Poly::new(509).unwrap();for n in [1,2,3,17,33]{let a=[3,0,2,1];let v=p.inv_series(&a,n).unwrap();let mut prod=school(&a,&v,509);prod.resize(n,0);prod.truncate(n);assert_eq!(prod[0],1);assert!(prod[1..].iter().all(|x|*x==0));}for n in [5,20,65]{for m in [1,3,14]{let a:Vec<_>=(0..n).map(|i|((i*7+3)%509)as u64).collect();let b:Vec<_>=(0..m).map(|i|((i*11+17)%509)as u64).collect();assert_eq!(p.divrem(&a,&b).unwrap(),long_div(&a,&b,509));}}assert!(p.newton_steps>0);}
 #[test]fn evaluation_order_repeats_and_errors(){let mut p=Poly::new(509).unwrap();let points=vec![3,0,7,3,508,2,19,41,80,92,113,190,201,250,300,321,400,401,480,500,1,9,15,23,33,45,67];let tree=p.product_tree(&points).unwrap();let a:Vec<_>=(0..45).map(|i|((i*i+1)%509)as u64).collect();let expected:Vec<_>=points.iter().map(|&x|a.iter().rev().fold(0,|v,&c|(v*x+c)%509)).collect();assert_eq!(p.values(&a,&tree).unwrap(),expected);assert_eq!(p.root_resultant(&[1,2],&[2,3]).unwrap(),0);assert!(matches!(p.product_tree(&[]),Err(Error::EmptyTree)));assert_eq!(p.divrem(&[1],&[0]),Err(Error::ZeroDivisor));assert_eq!(p.inv_series(&[0],2),Err(Error::Nonunit));assert_eq!(p.inv_series(&[1],0),Err(Error::Order));let other=Poly::new(7).unwrap();assert_eq!(p.values(&[1],&Tree{p:other.p,polynomial:vec![1,1],children:None}),Err(Error::Domain));}
}

#[cfg(test)]mod monic_tests{
 use super::*;
 use super::baseline;
 #[test]fn monic_remainders_match_general_division(){for p in [7u64,509]{let mut candidate=Poly::new(p).unwrap();let mut reference=baseline::Poly::new(p).unwrap();for d in [0usize,1,2,5,14,27]{let mut b:Vec<u64>=(0..d).map(|i|((i*i+3)as u64)%p).collect();b.push(1);for n in [0usize,1,2,5,13,29,65]{let a:Vec<u64>=(0..n).map(|i|((i*11+7)as u64)%(2*p)).collect();let got=candidate.monic_remainder(&a,&b).unwrap();let(_,expected)=reference.divrem(&a,&b).unwrap();assert_eq!(got,expected);}}assert_eq!(candidate.monic_remainder(&[1],&[0]),Err(Error::Domain));}}
 #[test]fn ordered_values_and_resultants(){for p in [7u64,509]{let mut candidate=Poly::new(p).unwrap();let mut reference=baseline::Poly::new(p).unwrap();for n in [1usize,2,3,13,27]{let points:Vec<u64>=(0..n).map(|i|((i*i+3)as u64)%p).collect();let a:Vec<u64>=(0..(n*2+1)).map(|i|((i*17+2)as u64)%(2*p)).collect();let c=candidate.product_tree(&points).unwrap();let b=reference.product_tree(&points).unwrap();let got=candidate.values(&a,&c).unwrap();assert_eq!(got,reference.values(&a,&b).unwrap());let horner:Vec<_>=points.iter().map(|&x|a.iter().rev().fold(0,|v,&c|(v*x+c)%p)).collect();assert_eq!(got,horner);assert_eq!(candidate.values(&[],&c).unwrap(),vec![0;n]);}assert_eq!(candidate.root_resultant(&[1,2,3],&[2,4]).unwrap(),0);}}
 #[test]fn series_and_foreign_domain(){let mut p=Poly::new(509).unwrap();let mut b=baseline::Poly::new(509).unwrap();for a in [vec![1,2,3],vec![3,0,1]]{for n in [1,2,17,33]{assert_eq!(p.inv_series(&a,n).unwrap(),b.inv_series(&a,n).unwrap());}}let other=Poly::new(7).unwrap().product_tree(&[1]).unwrap();assert_eq!(p.values(&[1],&other),Err(Error::Domain));assert!(p.inv_series(&[0],3).is_err());}
}

#[cfg(test)]#[path="../feature-native-polynomial/lib.rs"]mod baseline;

#[cfg(test)]mod low_product_tests{
 use super::*;
 fn canonical(mut a:Vec<u64>)->Vec<u64>{while a.len()>1&&a.last()==Some(&0){a.pop();}if a.is_empty(){a.push(0);}a}
 fn school(a:&[u64],b:&[u64],p:u64,n:usize)->Vec<u64>{let mut c=vec![0;n];for(i,&x)in a.iter().take(n).enumerate(){for(j,&y)in b.iter().take(n-i).enumerate(){c[i+j]=((c[i+j]as u128+(x%p)as u128*(y%p)as u128)%p as u128)as u64;}}canonical(c)}
 #[test]fn truncated_products_match_two_references(){
  for p in [7u64,509]{for la in [0usize,1,12,13,25,67]{for lb in [0usize,1,12,13,26,65]{
   let a:Vec<_>=(0..la).map(|i|((i*i+17)as u64)%(2*p)).collect();let b:Vec<_>=(0..lb).map(|i|((i*7+3)as u64)%(2*p)).collect();
   for n in [0usize,1,2,12,13,24,25,26,32,64,65,127,140]{let mut c=Poly::new(p).unwrap();let got=c.mul_low(&a,&b,n);let mut refpoly=baseline::Poly::new(p).unwrap();let mut full=refpoly.mul(&a,&b);full.truncate(n);assert_eq!(got,canonical(full));assert_eq!(got,school(&a,&b,p,n));}
  }}}
 }
 #[test]fn cancellation_and_sparse_boundaries(){let p=509;let mut c=Poly::new(p).unwrap();for n in [1usize,13,25,26,27,51,52,53]{let mut a=vec![0;53];a[0]=1;a[26]=508;a[52]=1;let mut b=vec![0;53];b[0]=1;b[26]=1;b[52]=1;assert_eq!(c.mul_low(&a,&b,n),school(&a,&b,p,n));}assert_eq!(c.mul_low(&[509,0],&[3],20),vec![0]);}
}

#[cfg(test)]#[path="../feature-native-low-product/polynomial.rs"]mod reference;
#[cfg(test)]mod product_tests{use super::*;include!("product-tests.rs");}

#[cfg(test)]mod euclid_tests{use super::*;include!("euclid-tests.rs");}

#[cfg(test)]mod class_tests{use super::*;include!("class-tests.rs");}

#[cfg(test)]#[path="../feature-native-resultant-class/polynomial.rs"]mod reference_euclid;
#[cfg(test)]mod half_tests{use super::*;include!("half-tests.rs");}
