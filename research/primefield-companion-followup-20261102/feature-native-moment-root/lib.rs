//! Convert exact aggregate moments into a shrinking squarefree root polynomial.
include!("projector.rs");
pub struct RootSet{r:u64,coefficients:Vec<u64>,poly:polynomial::Poly}
impl Projector{
 fn to_roots(&self,limit:usize)->Result<RootSet,Error>{
  let n=self.moments()?.count as usize;if n>limit{return Err(Error::Limit)}
  let mut q=vec![0;n+1];q[0]=1;
  for j in 1..=n{let mut sum=0;for i in 1..=j{let moment=(self.r-self.coefficients.get(self.n-1-i).copied().unwrap_or(0))%self.r;sum=(sum+mul(moment,q[j-i],self.r))%self.r}q[j]=mul((self.r-sum)%self.r,self.inverses[j],self.r)}
  q.reverse();Ok(RootSet{r:self.r,coefficients:q,poly:polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?})
 }
}
impl RootSet{
 fn rem(&mut self,a:&[u64],m:&[u64])->Result<Vec<u64>,Error>{self.poly.divrem(a,m).map(|v|v.1).map_err(|_|Error::Domain)}
 fn power_mod(&mut self,a:&[u64],mut exponent:u64,m:&[u64])->Result<Vec<u64>,Error>{let mut base=self.rem(a,m)?;let mut out=self.rem(&[1],m)?;while exponent>0{if exponent&1!=0{let p=self.poly.mul(&out,&base);out=self.rem(&p,m)?}exponent>>=1;if exponent>0{let p=self.poly.mul(&base,&base);base=self.rem(&p,m)?}}Ok(out)}
 fn gcd(&mut self,mut a:Vec<u64>,mut b:Vec<u64>)->Result<Vec<u64>,Error>{while b!=[0]{let rem=self.rem(&a,&b)?;a=b;b=rem}let lead=*a.last().ok_or(Error::Domain)?;if lead==0{return Err(Error::Domain)}let inv=pow(lead,self.r-2,self.r);if mul(lead,inv,self.r)!=1{return Err(Error::Domain)}Ok(a.into_iter().map(|v|mul(v,inv,self.r)).collect())}
 pub fn observe(&mut self,offset:u64,sign:i8)->Result<Moments,Error>{
  if offset>=self.r{return Err(Error::Domain)}if !(-1..=1).contains(&sign){return Err(Error::Sign)}
  if self.coefficients.len()==1{return Ok(self.moments())}
  let modulus=self.coefficients.clone();let constraint=if sign==0{vec![offset,1]}else{let c=self.power_mod(&[offset,1],(self.r-1)/2,&modulus)?;self.poly.sub(&c,&[if sign==1{1}else{self.r-1}])};
  self.coefficients=self.gcd(modulus,constraint)?;Ok(self.moments())
 }
 pub fn moments(&self)->Moments{let n=self.coefficients.len()-1;let first=if n==0{0}else{(self.r-self.coefficients[n-1])%self.r};Moments{count:n as u64,first,singleton:if n==1{Some(first)}else{None}}}
 pub fn polynomial(&self)->&[u64]{&self.coefficients}
}
enum Representation{Dense(Projector),Roots(RootSet)}
pub struct Hybrid{state:Representation,root_limit:usize,pub root_updates:usize,pub conversion_degree:Option<usize>}
impl Hybrid{
 pub fn new(r:u64,max_coefficients:usize,root_limit:usize)->Result<Self,Error>{if root_limit==0{return Err(Error::Limit)}Ok(Self{state:Representation::Dense(Projector::new(r,max_coefficients)?),root_limit,root_updates:0,conversion_degree:None})}
 pub fn observe(&mut self,offset:u64,sign:i8)->Result<Moments,Error>{
  let moments=match &mut self.state{Representation::Dense(p)=>p.observe(offset,sign)?,Representation::Roots(p)=>{let m=p.observe(offset,sign)?;self.root_updates+=1;m}};
  if let Representation::Dense(p)=&self.state{if moments.count as u128<=self.root_limit as u128{let roots=p.to_roots(self.root_limit)?;if roots.moments()!=moments{return Err(Error::Verification)}self.conversion_degree=Some(moments.count as usize);self.state=Representation::Roots(roots)}}
  Ok(moments)
 }
 pub fn root_polynomial(&self)->Option<&[u64]>{match &self.state{Representation::Dense(_)=>None,Representation::Roots(p)=>Some(p.polynomial())}}
}
include!("recovery.rs");
#[cfg(test)]mod root_tests{
 use super::*;
 fn chi(x:u64,r:u64)->i8{if x==0{0}else if pow(x,(r-1)/2,r)==1{1}else{-1}}
 fn reference(roots:&[u64],r:u64)->Vec<u64>{let mut p=vec![1];for &t in roots{let mut q=vec![0;p.len()+1];for(j,&v)in p.iter().enumerate(){q[j]=(q[j]+mul(v,(r-t)%r,r))%r;q[j+1]=(q[j+1]+v)%r}p=q}p}
 #[test]fn exact_conversion_and_updates(){let mut checks=0;for r in[5,7,11,31]{for code in 0..27{let mut dense=Projector::new(r,31).unwrap();let mut hybrid=Hybrid::new(r,31,4).unwrap();let(mut c,mut transcript)=(code,vec![]);for a in[0,1,3]{let s=(c%3)as i8-1;c/=3;transcript.push((a,s));let m=dense.observe(a,s).unwrap();assert_eq!(hybrid.observe(a,s).unwrap(),m);let candidates:Vec<_>=(0..r).filter(|&x|transcript.iter().all(|&(a,s)|chi((x+a)%r,r)==s)).collect();let expected=reference(&candidates,r);assert_eq!(dense.to_roots(31).unwrap().polynomial(),expected);if let Some(p)=hybrid.root_polynomial(){assert_eq!(p,expected);assert_eq!(p.len(),candidates.len()+1)}checks+=1}}}assert_eq!(checks,324);}
 #[test]fn boundary_controls(){let mut p=Projector::new(7,7).unwrap();assert!(matches!(p.to_roots(7),Err(Error::NoConstraints)));p.observe(0,1).unwrap();assert!(matches!(p.to_roots(2),Err(Error::Limit)));let mut root=p.to_roots(3).unwrap();assert_eq!(root.observe(0,1).unwrap().count,3);assert_eq!(root.observe(0,-1).unwrap().count,0);assert_eq!(root.polynomial(),[1]);assert_eq!(root.observe(0,0).unwrap().count,0);assert_eq!(root.observe(7,1),Err(Error::Domain));assert_eq!(root.observe(0,2),Err(Error::Sign));assert!(matches!(Hybrid::new(7,7,0),Err(Error::Limit)));}
 #[test]fn frozen_recovery_and_direct_gcd_control(){let s=RecoverySource::new(509,2,10,479,[3,68],64,479,16,16).unwrap();let(mut cases,mut updates)=(0,0);for line in include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1){let v:Vec<i64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let target=if v[1]<0{None}else{Some([v[1]as u64,v[2]as u64])};let out=s.recover(target).unwrap();assert_eq!(out.value,v[0]as u64);assert!(out.conversion_degree.unwrap()<=16);assert!(!s.verify((out.value+1)%479,target));
   // Direct first-constraint polynomial, the earlier gcd extractor's representation.
   let d=Projector::new(479,479).unwrap();let sign=out.transcript[0];let first=if sign==0{vec![0,1]}else{let mut c=d.character(0);c[0]=if sign==1{478}else{1};c};let mut direct=RootSet{r:479,coefficients:first,poly:polynomial::Poly::new(479).unwrap()};assert_eq!(direct.moments().count,out.counts[0]);for(a,&sign)in out.transcript.iter().enumerate().skip(1){assert_eq!(direct.observe(a as u64,sign).unwrap().count,out.counts[a])}assert_eq!(direct.moments().singleton,Some(out.value));updates+=out.root_updates;cases+=1;
  }assert_eq!(cases,20);assert!(updates>0);println!("frozen_cases={cases} small_root_updates={updates}");let short=RecoverySource::new(509,2,10,479,[3,68],64,479,1,16).unwrap();assert!(matches!(short.recover(Some([3,68])),Err(Error::Unresolved)));}
}
