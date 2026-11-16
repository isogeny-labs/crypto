struct SeededImage{r:u64,m:usize,t0:u64,inv_t0:u64,orientation:ReciprocalMode,branches:Vec<CanonicalBranch>,zero:bool,rounds:usize,slot_cap:usize}
std::thread_local!{static SEEDED_IMAGE_COUNTS:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl SeededImage{
 fn new(modes:&RecoveryModes,b:u64,f:&[u64],i:u64,slot_cap:usize,initial_character:u64)->Result<Self,Error>{
  let r=modes.r;if initial_character!=1&&initial_character!=r-1{return Err(Error::Domain)}if modes.m<3||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r)||i>=r||mul(i,i,r)!=r-1{return Err(Error::Domain)}
  let mut orientation=ReciprocalMode::build(modes,((r-1)/modes.m as u64)as usize)?.ok_or(Error::Domain)?;
  let a=mul(modes.m as u64,pow(b,(r-1)/2,r),r);let t0=pow(pow(b,modes.m as u64,r),r-2,r);let c=mul(f[0],pow(a,r-2,r),r);let mut branches=Vec::new();
  let normalized=mul(initial_character,pow(b,(r-1)/2,r),r);
  for u in [1,r-1,i,r-i]{if mul(u,u,r)!=normalized{continue}let g=squarefree(r,&orientation.branch(u,c)?)?;if g.len()==1{continue}let mut ring=QuadRing::new(r,&g)?;let mut phase=ring.power_v(orientation.n)?;phase.0[0][0]=(phase.0[0][0]+r-u)%r;let mut branch=CanonicalBranch{u,ring,canonical:CanonicalFiber::new(r,&g)?};branch.keep(phase)?;branches.push(branch);}
  // Only r,n are needed by the orientation lift after the initial branch construction.
  orientation.h=Vec::new();orientation.dn=Vec::new();
  let mut image=Self{r,m:modes.m,t0,inv_t0:pow(t0,r-2,r),orientation,branches,zero:false,rounds:0,slot_cap};image.absorb(modes,b,f,true)?;
  SEEDED_IMAGE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});Ok(image)
 }
 fn update(&mut self,modes:&RecoveryModes,b:u64,f:&[u64])->Result<(),Error>{self.absorb(modes,b,f,false)?;SEEDED_IMAGE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});Ok(())}
 fn absorb(&mut self,modes:&RecoveryModes,b:u64,f:&[u64],first:bool)->Result<(),Error>{
  let r=self.r;if modes.r!=r||modes.m!=self.m||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if self.rounds>=4{return Err(Error::Limit)}
  let h=(r-1)/2;let a=mul(self.m as u64,pow(b,h,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);
  self.zero=self.zero&&f[0]==a&&f[1]==0&&f[2]==0;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}
   let variable=Quad([vec![0],vec![scale]]);
   let sections=match branch.ring.sections(self.m,&variable,self.slot_cap){Ok(v)=>Some(v),Err(Error::Limit)=>None,Err(e)=>return Err(e)};
   if let Some(values)=sections{
    if !first{let mut c0=branch.ring.scale(&values[0],a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}
    if branch.ring.g.len()==1{continue}
    let product=branch.ring.multiply(&values[1],&values[self.m-1])?;
    let product=branch.ring.multiply(&product,&variable)?;
    let mut equation=branch.ring.scale(&product,mul(a,a,r));equation.0[0][0]=(equation.0[0][0]+r-mul(f[1],f[2],r))%r;
    branch.keep(equation)?;
   }else{
    POWERED_EVAL.with(|c|{let(a,b,d)=c.get();c.set((a,b,d+1))});
    if !first{let value=branch.ring.mode_scaled(modes,0,scale)?;let mut c0=branch.ring.scale(&value,a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}
    if branch.ring.g.len()==1{continue}
    let epsilon=mul(mul(branch.u,branch.u,r),pow(scale,h/self.m as u64,r),r);
    let equation=branch.ring.signed_equation_scaled(modes,scale,[f[1],f[2]],epsilon)?;branch.keep(equation)?;
   }
  }self.rounds+=1;Ok(())
 }
 fn materialize(&mut self,lift_cap:usize,image_cap:usize)->Result<Vec<u64>,Error>{
  if self.zero&&image_cap==0{return Err(Error::Limit)}let mut out=if self.zero{vec![0,1]}else{vec![1]};let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}let lift=branch.canonical.lift(&self.orientation,branch.u,lift_cap,&mut branch.ring)?;if out.len()-1+lift.len()-1>image_cap{return Err(Error::Limit)}out=p.mul(&out,&lift);}
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,self.r);lead=scale;scale=mul(scale,self.t0,self.r);}let inv=pow(lead,self.r-2,self.r);for x in &mut out{*x=mul(*x,inv,self.r)}Ok(out)
 }
}
