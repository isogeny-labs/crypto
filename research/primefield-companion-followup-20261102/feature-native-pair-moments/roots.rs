pub struct RootSet{r:u64,coefficients:Vec<u64>,poly:polynomial::Poly}
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
