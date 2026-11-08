// Caller certifies prime r. RecoveryModes validates the orbit order.
impl RecoveryModes<'_> {
 fn signed_observed(&self, signs:&[u64])->Result<[u64;2],Error>{
  let r=self.r;let h=(r-1)/2;
  if self.m<3||h%self.m as u64!=0||signs.len()!=self.m||signs.iter().any(|&s|s!=0&&s!=1&&s!=r-1){return Err(Error::Domain)}
  let mut out=[0;2];
  for (j,e) in [1,self.m-1].into_iter().enumerate(){
   let exponent=(e as u64+r-1-h)%(r-1);let root=pow(self.zeta,exponent,r);let mut weight=1;
   for &s in signs {out[j]=(out[j]+mul(weight,s,r))%r;weight=mul(weight,root,r);}
  }
  Ok(out)
 }
 // epsilon is a branch hypothesis x^h = +/-1, not a hidden-scalar input.
 // g is in W, with V^2-WV+1=0; x=0 is excluded from this chart.
 fn signed_norm_equation(&self,g:&[u64],f:[u64;2],epsilon:u64)->Result<Vec<u64>,Error>{
  let r=self.r;let h=(r-1)/2;
  if self.m<3||h%self.m as u64!=0||f.iter().any(|&v|v>=r)||(epsilon!=1&&epsilon!=r-1){return Err(Error::Domain)}
  let mut ring=QuadRing::new(r,g)?;
  let f1=ring.mode(self,1)?;
  let norm=ring.norm(&f1)?;
  let sigma=if h%2==0{1}else{r-1};
  let factor=mul(mul(self.m as u64,self.m as u64,r),mul(sigma,epsilon,r),r);
  let mut equation:Vec<_>=norm.iter().map(|&v|mul(factor,v,r)).collect();
  equation[0]=(equation[0]+r-mul(f[0],f[1],r))%r;
  Ok(equation)
 }
 fn signed_norm_filter(&self,g:&[u64],f:[u64;2],epsilon:u64)->Result<Vec<u64>,Error>{
  let equation=self.signed_norm_equation(g,f,epsilon)?;
  let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;
  Ok(p.gcd_monic(g,&equation,g.len()+equation.len()+2).map_err(|_|Error::Arithmetic)?.polynomial)
 }
}
