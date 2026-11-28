struct FusedImage{r:u64,m:usize,t0:u64,inv_t0:u64,orientation:ReciprocalMode,branches:Vec<CanonicalBranch>,zero:bool,rounds:usize}
std::thread_local!{static FUSED_COUNTS:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl FusedImage{
 fn new(modes:&RecoveryModes,b:u64,f:&[u64],i:u64)->Result<Self,Error>{
  let r=modes.r;if modes.m<3||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r)||i>=r||mul(i,i,r)!=r-1{return Err(Error::Domain)}
  let mut orientation=ReciprocalMode::build(modes,((r-1)/modes.m as u64)as usize)?.ok_or(Error::Domain)?;
  let a=mul(modes.m as u64,pow(b,(r-1)/2,r),r);let t0=pow(pow(b,modes.m as u64,r),r-2,r);let c=mul(f[0],pow(a,r-2,r),r);let mut branches=Vec::new();
  for u in [1,r-1,i,r-i]{let g=squarefree(r,&orientation.branch(u,c)?)?;if g.len()==1{continue}let mut ring=QuadRing::new(r,&g)?;let mut phase=ring.power_v(orientation.n)?;phase.0[0][0]=(phase.0[0][0]+r-u)%r;let mut branch=CanonicalBranch{u,ring,canonical:CanonicalFiber::new(r,&g)?};branch.keep(phase)?;branches.push(branch);}
  // Only r,n are needed by the orientation lift after the initial branch construction.
  orientation.h=Vec::new();orientation.dn=Vec::new();
  let mut image=Self{r,m:modes.m,t0,inv_t0:pow(t0,r-2,r),orientation,branches,zero:true,rounds:0};image.absorb(modes,b,f,true)?;
  FUSED_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});Ok(image)
 }
 fn update(&mut self,modes:&RecoveryModes,b:u64,f:&[u64])->Result<(),Error>{self.absorb(modes,b,f,false)?;FUSED_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});Ok(())}
 fn absorb(&mut self,modes:&RecoveryModes,b:u64,f:&[u64],first:bool)->Result<(),Error>{
  let r=self.r;if modes.r!=r||modes.m!=self.m||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if self.rounds>=4{return Err(Error::Limit)}
  let h=(r-1)/2;let a=mul(self.m as u64,pow(b,h,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);let c1=r-h;let c2=mul(mul(h,h-1,r),(r+1)/2,r);
  self.zero=self.zero&&f[0]==a&&mul(left,c2,r)==mul(right,mul(c1,c1,r),r);
  if !first{
   let indices:Vec<_>=self.branches.iter().enumerate().filter(|(_,b)|b.ring.g.len()>1).map(|(j,_)|j).collect();let mut rings:Vec<_>=indices.iter().map(|&j|ReusedRing::new(&self.branches[j].canonical)).collect::<Result<_,_>>()?;
   let values=shared_mode(&mut rings,modes,0,scale)?;for ((j,ring),value) in indices.into_iter().zip(rings.iter()).zip(values){let mut eq=ring.scale(&value,a);ring.subtract_constant(&mut eq,f[0]);self.branches[j].keep_split(eq)?;}
  }
  let indices:Vec<_>=self.branches.iter().enumerate().filter(|(_,b)|b.ring.g.len()>1).map(|(j,_)|j).collect();let mut rings:Vec<_>=indices.iter().map(|&j|ReusedRing::new(&self.branches[j].canonical)).collect::<Result<_,_>>()?;
  let(f1,f2)=fused_modes(&mut rings,modes,scale)?;
  for (((j,ring),v1),v2) in indices.into_iter().zip(rings.iter_mut()).zip(f1).zip(f2){let sq=ring.multiply(&v1,&v1)?;let l=ring.scale(&v2,left);let rr=ring.scale(&sq,right);self.branches[j].keep_split(ring.sub(&l,&rr))?;}
  self.rounds+=1;Ok(())
 }
 fn materialize(&mut self,lift_cap:usize,image_cap:usize)->Result<Vec<u64>,Error>{
  if self.zero&&image_cap==0{return Err(Error::Limit)}let mut out=if self.zero{vec![0,1]}else{vec![1]};let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}let lift=branch.canonical.lift(&self.orientation,branch.u,lift_cap,&mut branch.ring)?;if out.len()-1+lift.len()-1>image_cap{return Err(Error::Limit)}out=p.mul(&out,&lift);}
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,self.r);lead=scale;scale=mul(scale,self.t0,self.r);}let inv=pow(lead,self.r-2,self.r);for x in &mut out{*x=mul(*x,inv,self.r)}Ok(out)
 }
}
