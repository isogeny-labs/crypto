struct SignedImage{r:u64,m:usize,t0:u64,inv_t0:u64,orientation:ReciprocalMode,branches:Vec<CanonicalBranch>,zero:bool,rounds:usize}
std::thread_local!{static SIGNED_IMAGE_COUNTS:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl SignedImage{
 fn new(modes:&RecoveryModes,b:u64,f:&[u64],i:u64)->Result<Self,Error>{
  let r=modes.r;if modes.m<3||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r)||i>=r||mul(i,i,r)!=r-1{return Err(Error::Domain)}
  let mut orientation=ReciprocalMode::build(modes,((r-1)/modes.m as u64)as usize)?.ok_or(Error::Domain)?;
  let a=mul(modes.m as u64,pow(b,(r-1)/2,r),r);let t0=pow(pow(b,modes.m as u64,r),r-2,r);let c=mul(f[0],pow(a,r-2,r),r);let mut branches=Vec::new();
  for u in [1,r-1,i,r-i]{let g=squarefree(r,&orientation.branch(u,c)?)?;if g.len()==1{continue}let mut ring=QuadRing::new(r,&g)?;let mut phase=ring.power_v(orientation.n)?;phase.0[0][0]=(phase.0[0][0]+r-u)%r;let mut branch=CanonicalBranch{u,ring,canonical:CanonicalFiber::new(r,&g)?};branch.keep(phase)?;branches.push(branch);}
  // Only r,n are needed by the orientation lift after the initial branch construction.
  orientation.h=Vec::new();orientation.dn=Vec::new();
  let mut image=Self{r,m:modes.m,t0,inv_t0:pow(t0,r-2,r),orientation,branches,zero:true,rounds:0};image.absorb(modes,b,f,true)?;
  SIGNED_IMAGE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});Ok(image)
 }
 fn update(&mut self,modes:&RecoveryModes,b:u64,f:&[u64])->Result<(),Error>{self.absorb(modes,b,f,false)?;SIGNED_IMAGE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});Ok(())}
 fn absorb(&mut self,modes:&RecoveryModes,b:u64,f:&[u64],first:bool)->Result<(),Error>{
  let r=self.r;if modes.r!=r||modes.m!=self.m||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if self.rounds>=4{return Err(Error::Limit)}
  let h=(r-1)/2;let a=mul(self.m as u64,pow(b,h,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);
  let phase_scale=pow(scale,h/self.m as u64,r);
  self.zero=self.zero&&f[0]==a&&f[1]==0&&f[2]==0;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}
   if !first{let value=branch.ring.mode_scaled(modes,0,scale)?;let mut c0=branch.ring.scale(&value,a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}
   if branch.ring.g.len()==1{continue}
   let epsilon=mul(mul(branch.u,branch.u,r),phase_scale,r);
   let equation=branch.ring.signed_equation_scaled(modes,scale,[f[1],f[2]],epsilon)?;
   branch.keep(equation)?;
  }self.rounds+=1;Ok(())
 }
 fn materialize(&mut self,lift_cap:usize,image_cap:usize)->Result<Vec<u64>,Error>{
  if self.zero&&image_cap==0{return Err(Error::Limit)}let mut out=if self.zero{vec![0,1]}else{vec![1]};let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}let lift=branch.canonical.lift(&self.orientation,branch.u,lift_cap,&mut branch.ring)?;if out.len()-1+lift.len()-1>image_cap{return Err(Error::Limit)}out=p.mul(&out,&lift);}
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,self.r);lead=scale;scale=mul(scale,self.t0,self.r);}let inv=pow(lead,self.r-2,self.r);for x in &mut out{*x=mul(*x,inv,self.r)}Ok(out)
 }
}
