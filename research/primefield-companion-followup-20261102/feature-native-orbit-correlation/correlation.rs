type Mat=[Vec<u64>;4];
fn matrix_product(p:&mut polynomial::Poly,a:&Mat,b:&Mat)->Mat{
 std::array::from_fn(|i|{let row=i/2;let col=i%2;let u=p.mul(&a[2*row],&b[col]);let v=p.mul(&a[2*row+1],&b[2+col]);p.add(&u,&v)})
}
// Product T(X+hi-1)...T(X+lo), with denominator product a(X+i+1).
fn coefficient_block(p:&mut polynomial::Poly,r:u64,a:u64,h:u64,lo:usize,hi:usize)->(Mat,Vec<u64>){
 if hi-lo==1{let i=lo as u64;let s=(1+a)%r;let den=vec![mul(a,i+1,r),a];return([vec![mul((i+r-h)%r,s,r),s],vec![(r-i)%r,r-1],den.clone(),vec![0]],den)}
 let mid=lo+(hi-lo)/2;let(l,dl)=coefficient_block(p,r,a,h,lo,mid);let(u,du)=coefficient_block(p,r,a,h,mid,hi);let matrix=matrix_product(p,&u,&l);let denominator=p.mul(&du,&dl);(matrix,denominator)
}
#[derive(Debug)]pub struct Correlation{r:u64,m:usize,d:usize,lag:usize,coefficients:Vec<u64>,pub karatsuba_calls:usize,pub newton_steps:usize}
impl Correlation{
 /// Exact prime-field recurrence; max_block and max_points checked before allocation.
 pub fn compile(f:&Fold,lag:usize,max_block:usize,max_points:usize)->Result<Self,Error>{
  if lag>=f.m{return Err(Error::Domain)}if f.m>max_block||f.d>max_points{return Err(Error::Limit)}
  let(r,m,d)=(f.r,f.m,f.d);let a=pow(f.zeta,lag as u64,r);let h=(r-1)/2;
  let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let(matrix,denominator)=coefficient_block(&mut p,r,a,h,0,m);
  let points:Vec<_>=(0..d).map(|j|(j as u64)*(m as u64)).collect();let tree=p.product_tree(&points).map_err(|_|Error::Domain)?;
  let mut values=Vec::with_capacity(4);for entry in &matrix{values.push(p.values(entry,&tree).map_err(|_|Error::Domain)?)}
  let den=p.values(&denominator,&tree).map_err(|_|Error::Domain)?;let mut prefix=vec![1;d+1];for j in 0..d{if den[j]==0{return Err(Error::Verification)}prefix[j+1]=mul(prefix[j],den[j],r)}
  let mut inverse=pow(prefix[d],r-2,r);if mul(inverse,prefix[d],r)!=1{return Err(Error::Verification)}let mut inverses=vec![0;d];for j in (0..d).rev(){inverses[j]=mul(inverse,prefix[j],r);inverse=mul(inverse,den[j],r)}
  let(mut current,mut previous)=(pow(a,h,r),0);let mut coefficients=Vec::with_capacity(d+1);coefficients.push(mul(m as u64,current,r));
  for j in 0..d{let next=mul((mul(values[0][j],current,r)+mul(values[1][j],previous,r))%r,inverses[j],r);let before=mul((mul(values[2][j],current,r)+mul(values[3][j],previous,r))%r,inverses[j],r);current=next;previous=before;coefficients.push(mul(m as u64,current,r));}
  Ok(Self{r,m,d,lag,coefficients,karatsuba_calls:p.karatsuba_calls,newton_steps:p.newton_steps})
 }
 pub fn coefficients(&self)->&[u64]{&self.coefficients}
 pub fn observed(&self,signs:&[i8])->Result<u64,Error>{if signs.len()!=self.m||signs.iter().any(|&s|!(-1..=1).contains(&s)){return Err(Error::Sign)}let mut value=0;for j in 0..self.m{let v=signs[j]*signs[(j+self.lag)%self.m];value=(value+if v<0{self.r-1}else{v as u64})%self.r}Ok(value)}
 pub fn constraint(&self,b:u64,signs:&[i8])->Result<Vec<u64>,Error>{if b==0||b>=self.r{return Err(Error::Domain)}let actual=self.observed(signs)?;let inverse=pow(pow(b,self.m as u64,self.r),self.r-2,self.r);let mut scale=1;let mut out=Vec::with_capacity(self.d+1);for &c in &self.coefficients{out.push(mul(c,scale,self.r));scale=mul(scale,inverse,self.r)}out[0]=(out[0]+self.r-actual)%self.r;Ok(out)}
 pub fn observe(&self,state:&mut RootSet,b:u64,signs:&[i8])->Result<Moments,Error>{if state.r!=self.r{return Err(Error::Domain)}let constraint=self.constraint(b,signs)?;if state.coefficients.len()==1{return Ok(state.moments())}let modulus=state.coefficients.clone();let rem=state.rem(&constraint,&modulus)?;state.coefficients=state.gcd(modulus,rem)?;Ok(state.moments())}
}
pub struct CorrelatedRecovery{base:SingleRecovery,correlations:Vec<Correlation>}
impl CorrelatedRecovery{
 pub fn new(base:SingleRecovery,lags:&[usize],max_lags:usize,max_block:usize,max_points:usize)->Result<Self,Error>{if lags.len()>max_lags{return Err(Error::Limit)}let mut correlations=Vec::with_capacity(lags.len());for &lag in lags{correlations.push(Correlation::compile(&base.inner.fold,lag,max_block,max_points)?)}Ok(Self{base,correlations})}
 pub fn recover(&self,target:Option<[u64;2]>)->Result<FiberOutcome,Error>{self.base.recover_correlated(target,&self.correlations)}
}
