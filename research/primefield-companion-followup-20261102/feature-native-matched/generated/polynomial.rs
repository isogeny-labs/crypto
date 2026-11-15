//! Native source polynomial engine: Karatsuba, Newton reversal division,
//! product trees and remainder-tree evaluation. Prime modulus is caller-certified.
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Nonunit,Order,ZeroDivisor,EmptyTree,Invariant}
pub struct Poly{p:u64,pub karatsuba_calls:usize,pub newton_steps:usize}
pub struct Tree{p:u64,polynomial:Vec<u64>,children:Option<(Box<Tree>,Box<Tree>)>}
impl Tree{pub fn polynomial(&self)->&[u64]{&self.polynomial}}
impl Poly{
 pub fn new(p:u64)->Result<Self,Error>{if p<3||p>u64::MAX/2{return Err(Error::Domain);}Ok(Self{p,karatsuba_calls:0,newton_steps:0})}
 fn fm(&self,a:u64,b:u64)->u64{crate::meter::hit(crate::meter::Component::Polynomial,crate::meter::Op::Product);((a as u128*b as u128)%self.p as u128)as u64}
 fn inverse(&self,a:u64)->Result<u64,Error>{crate::meter::hit(crate::meter::Component::Polynomial,crate::meter::Op::Inverse);if a%self.p==1{crate::meter::hit(crate::meter::Component::Polynomial,crate::meter::Op::UnitInverse);}let(mut r,mut s,mut x,mut y)=(self.p as i128,(a%self.p)as i128,0i128,1i128);while s!=0{crate::meter::hit(crate::meter::Component::Polynomial,crate::meter::Op::EuclidStep);let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r==1{Ok(x.rem_euclid(self.p as i128)as u64)}else{Err(Error::Nonunit)}}
 fn trim(&self,a:&[u64])->Vec<u64>{let mut a:Vec<_>=a.iter().map(|x|x%self.p).collect();while a.len()>1&&a.last()==Some(&0){a.pop();}if a.is_empty(){a.push(0);}a}
 pub fn add(&self,a:&[u64],b:&[u64])->Vec<u64>{let(a,b)=(self.trim(a),self.trim(b));let mut c=vec![0;a.len().max(b.len())];for(i,x)in a.iter().enumerate(){c[i]=*x;}for(i,x)in b.iter().enumerate(){crate::meter::hit(crate::meter::Component::Polynomial,crate::meter::Op::AddSub);c[i]=(c[i]+x)%self.p;}self.trim(&c)}
 pub fn sub(&self,a:&[u64],b:&[u64])->Vec<u64>{let b:Vec<_>=b.iter().map(|x|crate::meter::value(crate::meter::Component::Polynomial,crate::meter::Op::Neg,(self.p-x%self.p)%self.p)).collect();self.add(a,&b)}
 pub fn mul(&mut self,a:&[u64],b:&[u64])->Vec<u64>{
  let(a,b)=(self.trim(a),self.trim(b));if a==[0]||b==[0]{return vec![0];}
  if a.len().min(b.len())<=12{let mut c=vec![0;a.len()+b.len()-1];for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){crate::meter::hit(crate::meter::Component::Polynomial,crate::meter::Op::AddSub);c[i+j]=(c[i+j]+self.fm(x,y))%self.p;}}return self.trim(&c);}
  self.karatsuba_calls+=1;let k=(a.len().max(b.len())+1)/2;let(a0,a1)=a.split_at(k.min(a.len()));let(b0,b1)=b.split_at(k.min(b.len()));
  let lo=self.mul(a0,b0);let hi=self.mul(a1,b1);let aa=self.add(a0,a1);let bb=self.add(b0,b1);let cross=self.mul(&aa,&bb);let middle=self.sub(&self.sub(&cross,&lo),&hi);
  let mut shifted=vec![0;k];shifted.extend(middle);let left=self.add(&lo,&shifted);let mut shifted=vec![0;2*k];shifted.extend(hi);self.add(&left,&shifted)
 }
 pub fn inv_series(&mut self,a:&[u64],n:usize)->Result<Vec<u64>,Error>{
  if n==0{return Err(Error::Order);}let a=self.trim(a);let mut v=vec![if a[0]==1{1}else{self.inverse(a[0])?}];let mut k=1;
  while k<n{k=k.saturating_mul(2).min(n);let mut av=self.mul(&a[..a.len().min(k)],&v);av.truncate(k);let correction=self.sub(&[2],&av);let mut next=self.mul(&v,&correction);next.truncate(k);v=self.trim(&next);self.newton_steps+=1;}
  Ok(v)
 }
 pub fn divrem(&mut self,a:&[u64],b:&[u64])->Result<(Vec<u64>,Vec<u64>),Error>{
  let(a,b)=(self.trim(a),self.trim(b));if b==[0]{return Err(Error::ZeroDivisor);}if a.len()<b.len(){return Ok((vec![0],a));}
  let n=a.len()-b.len()+1;let rev_a:Vec<_>=a.iter().rev().take(n).copied().collect();let rev_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&rev_b,n)?;let mut reverse_q=self.mul(&rev_a,&inv);reverse_q.truncate(n);reverse_q.resize(n,0);reverse_q.reverse();let q=self.trim(&reverse_q);let qb=self.mul(&q,&b);let rem=self.sub(&a,&qb);
  if rem!=[0]&&rem.len()>=b.len(){return Err(Error::Invariant);}Ok((q,rem))
 }
 pub fn product_tree(&mut self,points:&[u64])->Result<Tree,Error>{
  if points.is_empty(){return Err(Error::EmptyTree);}if points.len()==1{return Ok(Tree{p:self.p,polynomial:vec![crate::meter::value(crate::meter::Component::Polynomial,crate::meter::Op::Neg,(self.p-points[0]%self.p)%self.p),1],children:None});}
  let k=points.len()/2;let left=self.product_tree(&points[..k])?;let right=self.product_tree(&points[k..])?;let polynomial=self.mul(&left.polynomial,&right.polynomial);Ok(Tree{p:self.p,polynomial,children:Some((Box::new(left),Box::new(right)))})
 }
 /// Internal tree divisors are canonical and monic. The quotient is still
 /// obtained by Newton reversal; only coefficients below deg(b) are materialized
 /// for a-q*b. General divrem remains unchanged as an independent reference path.
 fn monic_remainder(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{
  if b.last()!=Some(&1){return Err(Error::Domain);}let a=self.trim(a);let d=b.len()-1;if d==0{return Ok(vec![0]);}if a.len()<b.len(){return Ok(a);}
  let n=a.len()-b.len()+1;let reverse_a:Vec<_>=a.iter().rev().take(n).copied().collect();let reverse_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&reverse_b,n)?;let mut q=self.mul(&reverse_a,&inv);q.truncate(n);q.resize(n,0);q.reverse();
  let mut low=self.mul(&q[..q.len().min(d)],&b[..d]);low.truncate(d);let mut rem=Vec::with_capacity(d);
  for i in 0..d{rem.push(crate::meter::value(crate::meter::Component::Polynomial,crate::meter::Op::AddSub,(a.get(i).copied().unwrap_or(0)+self.p-low.get(i).copied().unwrap_or(0))%self.p));}Ok(self.trim(&rem))
 }
 pub fn values(&mut self,a:&[u64],tree:&Tree)->Result<Vec<u64>,Error>{
  if tree.p!=self.p{return Err(Error::Domain);}
  if tree.children.is_none(){let x=crate::meter::value(crate::meter::Component::Polynomial,crate::meter::Op::Neg,(self.p-tree.polynomial[0])%self.p);let mut coefficients=a.iter().rev();let mut value=coefficients.next().map_or(0,|c|c%self.p);for &coefficient in coefficients{value=crate::meter::value(crate::meter::Component::Polynomial,crate::meter::Op::AddSub,(self.fm(value,x)+coefficient%self.p)%self.p);}return Ok(vec![value]);}
  let rem=self.monic_remainder(a,&tree.polynomial)?;let(left,right)=tree.children.as_ref().ok_or(Error::Invariant)?;let mut v=self.values(&rem,left)?;v.extend(self.values(&rem,right)?);Ok(v)
 }
 pub fn root_resultant(&mut self,roots:&[u64],points:&[u64])->Result<u64,Error>{let source=self.product_tree(roots)?;let destination=self.product_tree(points)?;let values=self.values(source.polynomial(),&destination)?;Ok(values.iter().fold(1,|v,&x|self.fm(v,x)))}
}
#[cfg(test)]mod tests{
 use super::*;
 fn trim(mut a:Vec<u64>)->Vec<u64>{while a.len()>1&&a.last()==Some(&0){a.pop();}if a.is_empty(){a.push(0);}a}
 fn school(a:&[u64],b:&[u64],p:u64)->Vec<u64>{if a.is_empty()||b.is_empty(){return vec![0];}let mut v=vec![0;a.len()+b.len()-1];for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){v[i+j]=((v[i+j]as u128+x as u128*y as u128)%p as u128)as u64;}}trim(v)}
 fn power(mut a:u64,mut n:u64,p:u64)->u64{let mut v=1;while n!=0{if n&1!=0{v=((v as u128*a as u128)%p as u128)as u64;}a=((a as u128*a as u128)%p as u128)as u64;n>>=1;}v}
 fn long_div(a:&[u64],b:&[u64],p:u64)->(Vec<u64>,Vec<u64>){let mut r=trim(a.to_vec());let b=trim(b.to_vec());if r.len()<b.len(){return(vec![0],r);}let mut q=vec![0;r.len()-b.len()+1];let inverse=power(*b.last().unwrap(),p-2,p);while r!=[0]&&r.len()>=b.len(){let k=r.len()-b.len();let c=*r.last().unwrap()*inverse%p;q[k]=c;for(j,&x)in b.iter().enumerate(){r[k+j]=(r[k+j]+p-c*x%p)%p;}r=trim(r);} (trim(q),r)}
 fn list(s:&str)->Vec<u64>{s.split(',').map(|x|x.parse().unwrap()).collect()}
 #[test]fn saved_source_resultants(){for line in include_str!("../../feature-native-polynomial/fixtures.tsv").lines(){let f:Vec<_>=line.split('\t').collect();let mut p=Poly::new(f[0].parse().unwrap()).unwrap();assert_eq!(p.root_resultant(&list(f[2]),&list(f[3])).unwrap(),f[4].parse::<u64>().unwrap());let a=p.product_tree(&list(f[5])).unwrap();let b=p.product_tree(&[f[6].parse().unwrap()]).unwrap();assert_eq!(p.values(a.polynomial(),&b).unwrap(),vec![f[7].parse::<u64>().unwrap()]);}}
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

#[cfg(test)]#[path="../../feature-native-polynomial/lib.rs"]mod baseline;
