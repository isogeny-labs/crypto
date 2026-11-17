struct RadiusAnchorBranch{epsilon:u64,ring:QuadRing,canonical:CanonicalFiber,anchor:Option<Quad>}
impl RadiusAnchorBranch{fn keep(&mut self,pair:Quad)->Result<(),Error>{self.canonical.keep(&mut self.ring.p,&pair)?;self.ring.g=self.canonical.support(&mut self.ring.p);Ok(())}}
struct OrientedImage{r:u64,m:usize,t0:u64,inv_t0:u64,orientation:ReciprocalMode,branches:Vec<RadiusAnchorBranch>,zero:bool,rounds:usize,slot_cap:usize,anchor:Option<u64>}
std::thread_local!{static ORIENTED_COUNTS:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl OrientedImage{
 fn update(&mut self,modes:&RecoveryModes,b:u64,f:&[u64])->Result<(),Error>{self.absorb(modes,b,f,false)?;ORIENTED_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});Ok(())}
 fn absorb(&mut self,modes:&RecoveryModes,b:u64,f:&[u64],first:bool)->Result<(),Error>{
  let r=self.r;if modes.r!=r||modes.m!=self.m||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if self.rounds>=4{return Err(Error::Limit)}
  let h=(r-1)/2;let bh=pow(b,h,r);let a=mul(self.m as u64,bh,r);let normalized=mul(mul(b,bh,r),f[1],r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);
  let establish=self.anchor.is_none()&&f[1]!=0&&f[2]!=0;
  self.zero=self.zero&&f[0]==a&&f[1]==0&&f[2]==0;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}
   let variable=Quad([vec![0],vec![scale]]);let sections=match branch.ring.sections(self.m,&variable,self.slot_cap){Ok(v)=>Some(v),Err(Error::Limit)=>None,Err(e)=>return Err(e)};
   let (plus,minus)=if let Some(values)=sections{
    if !first{let mut c0=branch.ring.scale(&values[0],a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}if branch.ring.g.len()==1{continue}
    let product=branch.ring.multiply(&values[1],&values[self.m-1])?;let product=branch.ring.multiply(&product,&variable)?;let mut equation=branch.ring.scale(&product,mul(a,a,r));equation.0[0][0]=(equation.0[0][0]+r-mul(f[1],f[2],r))%r;branch.keep(equation)?;
    (values[1].clone(),values[self.m-1].clone())
   }else{
    POWERED_EVAL.with(|c|{let(a,b,d)=c.get();c.set((a,b,d+1))});if !first{let value=branch.ring.mode_scaled(modes,0,scale)?;let mut c0=branch.ring.scale(&value,a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}if branch.ring.g.len()==1{continue}
    let epsilon=mul(branch.epsilon,pow(scale,h/self.m as u64,r),r);let(product,plus,minus)=branch.ring.oriented_pair_scaled(modes,scale)?;let sigma=if h%2==0{1}else{r-1};let factor=mul(mul(self.m as u64,self.m as u64,r),mul(sigma,epsilon,r),r);let mut equation=branch.ring.scale(&product,factor);equation.0[0][0]=(equation.0[0][0]+r-mul(f[1],f[2],r))%r;branch.keep(equation)?;(plus,minus)
   };
   if branch.ring.g.len()==1{continue}
   if f[1]==0{branch.keep(plus.clone())?;}if branch.ring.g.len()==1{continue}
   if f[2]==0{branch.keep(minus)?;}if branch.ring.g.len()==1{continue}
   if let Some(fa)=self.anchor{
    let old=branch.anchor.as_ref().ok_or(Error::Arithmetic)?;
    let old=Quad([branch.ring.rem(&old.0[0])?,branch.ring.rem(&old.0[1])?]);
    let left=branch.ring.scale(&plus,fa);let right=branch.ring.scale(&old,normalized);let equation=branch.ring.sub(&left,&right);branch.keep(equation)?;
    if branch.ring.g.len()>1{branch.anchor=Some(Quad([branch.ring.rem(&old.0[0])?,branch.ring.rem(&old.0[1])?]));}
   }else if establish{branch.anchor=Some(Quad([branch.ring.rem(&plus.0[0])?,branch.ring.rem(&plus.0[1])?]));}
  }
  if establish{self.anchor=Some(normalized);}self.rounds+=1;Ok(())
 }
 fn materialize(&mut self,lift_cap:usize,image_cap:usize)->Result<Vec<u64>,Error>{
  let mut out=vec![1];let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}let lift=balanced_fiber_lift(&branch.canonical,lift_cap,&mut branch.ring)?;if out.len()-1+lift.len()-1>image_cap{return Err(Error::Limit)}out=p.mul(&out,&lift);}
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,self.r);lead=scale;scale=mul(scale,self.t0,self.r);}let inv=pow(lead,self.r-2,self.r);for x in &mut out{*x=mul(*x,inv,self.r)}Ok(out)
 }
}
