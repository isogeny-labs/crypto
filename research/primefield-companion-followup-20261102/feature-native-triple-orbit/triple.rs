type Mat3=[Vec<u64>;9];
fn multiply3(p:&mut polynomial::Poly,a:&Mat3,b:&Mat3)->Mat3{std::array::from_fn(|i|{let(row,col)=(i/3,i%3);let mut out=vec![0];for k in 0..3{let v=p.mul(&a[3*row+k],&b[3*k+col]);out=p.add(&out,&v)}out})}
fn block3(p:&mut polynomial::Poly,r:u64,q:[u64;4],h:u64,lo:usize,hi:usize)->(Mat3,Vec<u64>){
 if hi-lo==1{let mut matrix:Mat3=std::array::from_fn(|_|vec![0]);let den=vec![mul(q[0],lo as u64+1,r),q[0]];
  for i in 1..=3{let factor=(((h+1)as u128*i as u128+r as u128-(lo+1)as u128)%r as u128)as u64;matrix[i-1]=vec![mul(factor,q[i],r),(r-q[i])%r];}
  matrix[3]=den.clone();matrix[7]=den.clone();return(matrix,den)
 }
 let mid=lo+(hi-lo)/2;let(left,dl)=block3(p,r,q,h,lo,mid);let(right,dr)=block3(p,r,q,h,mid,hi);let matrix=multiply3(p,&right,&left);let denominator=p.mul(&dr,&dl);(matrix,denominator)
}
fn cubic(r:u64,a:u64,b:u64)->[u64;4]{let ab=mul(a,b,r);[(r-ab)%r,((a+b)%r+ab)%r,(r-((1+a)%r+b)%r)%r,1]}
fn cubic_prefix(f:&Fold,q:[u64;4])->Result<(Vec<u64>,usize,usize),Error>{
 let(r,m,d)=(f.r,f.m,f.d);let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let(matrix,denominator)=block3(&mut p,r,q,(r-1)/2,0,m);
 let points:Vec<_>=(0..d).map(|j|j as u64*m as u64).collect();let tree=p.product_tree(&points).map_err(|_|Error::Domain)?;let mut vals=Vec::with_capacity(9);for c in &matrix{vals.push(p.values(c,&tree).map_err(|_|Error::Domain)?)}
 let den=p.values(&denominator,&tree).map_err(|_|Error::Domain)?;let mut prefix=vec![1;d+1];for j in 0..d{if den[j]==0{return Err(Error::Verification)}prefix[j+1]=mul(prefix[j],den[j],r)}
 let mut inverse=pow(prefix[d],r-2,r);if mul(inverse,prefix[d],r)!=1{return Err(Error::Verification)}let mut inverses=vec![0;d];for j in(0..d).rev(){inverses[j]=mul(inverse,prefix[j],r);inverse=mul(inverse,den[j],r)}
 let mut state=[pow(q[0],(r-1)/2,r),0,0];let mut result=vec![state[0]];
 for j in 0..d{state=std::array::from_fn(|row|{let mut value=0;for k in 0..3{value=(value+mul(vals[3*row+k][j],state[k],r))%r}mul(value,inverses[j],r)});result.push(state[0]);}
 Ok((result,p.karatsuba_calls,p.newton_steps))
}
#[derive(Debug)]pub struct TripleCorrelation{r:u64,m:usize,d:usize,lags:[usize;2],coefficients:Vec<u64>,pub reflected_coefficients:usize,pub karatsuba_calls:usize,pub newton_steps:usize}
impl TripleCorrelation{
 pub fn compile(f:&Fold,lags:[usize;2],max_block:usize,max_points:usize)->Result<Self,Error>{
  if f.d%2!=0||lags[0]==0||lags[1]==0||lags[0]==lags[1]||lags.iter().any(|&l|l>=f.m){return Err(Error::Domain)}
  if f.m>max_block||f.d>max_points{return Err(Error::Limit)}let total=f.d.checked_mul(3).ok_or(Error::Limit)?/2;
  let(r,m,d)=(f.r,f.m,f.d);let a=pow(f.zeta,lags[0]as u64,r);let b=pow(f.zeta,lags[1]as u64,r);let q=cubic(r,a,b);let inverse=cubic(r,pow(a,r-2,r),pow(b,r-2,r));
  let(front,k1,n1)=cubic_prefix(f,q)?;let(back,k2,n2)=cubic_prefix(f,inverse)?;let phase=pow(q[0],(r-1)/2,r);
  // Overlap checks use only prefixes whose indices are < r.
  for t in total-d..=d{if front[t]!=mul(phase,back[total-t],r){return Err(Error::Verification)}}
  let mut coefficients=vec![0;d+1];for t in 0..=total{let v=if t<=d{front[t]}else{mul(phase,back[total-t],r)};let at=if t>d{t-d}else{t};coefficients[at]=(coefficients[at]+mul(m as u64,v,r))%r;}
  Ok(Self{r,m,d,lags,coefficients,reflected_coefficients:total-d,karatsuba_calls:k1+k2,newton_steps:n1+n2})
 }
 pub fn coefficients(&self)->&[u64]{&self.coefficients}
 pub fn observed(&self,signs:&[i8])->Result<u64,Error>{if signs.len()!=self.m||signs.iter().any(|&s|!(-1..=1).contains(&s)){return Err(Error::Sign)}let mut value=0;for j in 0..self.m{let v=signs[j]*signs[(j+self.lags[0])%self.m]*signs[(j+self.lags[1])%self.m];value=(value+if v<0{self.r-1}else{v as u64})%self.r}Ok(value)}
 pub fn constraint(&self,b:u64,signs:&[i8])->Result<Vec<u64>,Error>{if b==0||b>=self.r{return Err(Error::Domain)}let actual=self.observed(signs)?;let inverse=pow(pow(b,self.m as u64,self.r),self.r-2,self.r);let mut scale=pow(b,(self.r-1)/2,self.r);let mut out=Vec::with_capacity(self.d+1);for &c in &self.coefficients{out.push(mul(c,scale,self.r));scale=mul(scale,inverse,self.r)}out[0]=(out[0]+self.r-actual)%self.r;Ok(out)}
 pub fn observe(&self,state:&mut RootSet,b:u64,signs:&[i8])->Result<Moments,Error>{if state.r!=self.r{return Err(Error::Domain)}let c=self.constraint(b,signs)?;if state.coefficients.len()==1{return Ok(state.moments())}let modulus=state.coefficients.clone();let rem=state.rem(&c,&modulus)?;state.coefficients=state.gcd(modulus,rem)?;Ok(state.moments())}
}
pub struct HigherRecovery{base:SingleRecovery,pairs:Vec<Correlation>,triples:Vec<TripleCorrelation>}
impl HigherRecovery{
 pub fn new(base:SingleRecovery,pair_lags:&[usize],triple_lags:&[[usize;2]],max_features:usize,max_block:usize,max_points:usize)->Result<Self,Error>{
  if pair_lags.len().checked_add(triple_lags.len()).ok_or(Error::Limit)?>max_features{return Err(Error::Limit)}
  if !triple_lags.is_empty()&&base.inner.fold.d%2!=0{return Err(Error::Domain)}
  let mut pairs=Vec::with_capacity(pair_lags.len());let mut triples=Vec::with_capacity(triple_lags.len());for &l in pair_lags{pairs.push(Correlation::compile(&base.inner.fold,l,max_block,max_points)?)}for &ls in triple_lags{triples.push(TripleCorrelation::compile(&base.inner.fold,ls,max_block,max_points)?)}Ok(Self{base,pairs,triples})
 }
 pub fn recover(&self,target:Option<[u64;2]>)->Result<FiberOutcome,Error>{self.base.recover_higher(target,&self.pairs,&self.triples)}
}
