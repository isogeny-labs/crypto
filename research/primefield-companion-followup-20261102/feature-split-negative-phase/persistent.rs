impl QuadRing{
 fn block_scaled(&mut self,reader:&mut CoefficientReader<'_>,length:usize,scale:u64)->Result<(Quad,Quad),Error>{
  if length==0||scale==0||scale>=self.r{return Err(Error::Domain)}
  if length==1{return Ok((Quad([vec![reader.next()?],vec![0]]),Quad([vec![0],vec![scale]])))}
  let(a,ap)=self.block_scaled(reader,length/2,scale)?;let(b,bp)=self.block_scaled(reader,length-length/2,scale)?;let ab=self.multiply(&a,&bp)?;let value=self.add(&ab,&b);let power=self.multiply(&ap,&bp)?;Ok((value,power))
 }
 fn mode_scaled(&mut self,modes:&RecoveryModes,e:usize,scale:u64)->Result<Quad,Error>{let stream=modes.stream(e)?;let length=stream.left;let mut reader=CoefficientReader{stream,buffer:Vec::new(),at:0,remaining:length};Ok(self.block_scaled(&mut reader,length,scale)?.0)}
}
struct OrientedBranch{u:u64,ring:QuadRing,constraints:Vec<Quad>}
impl OrientedBranch{
 fn keep(&mut self,next:Quad)->Result<(),Error>{
  if self.ring.g.len()==1{return Ok(())}if self.constraints.len()>=8{return Err(Error::Limit)}
  let norm=self.ring.norm(&next)?;let bound=2*self.ring.g.len();let mut g=self.ring.p.gcd_monic(&self.ring.g,&norm,bound).map_err(|_|Error::Arithmetic)?.polynomial;
  for old in &self.constraints{let ad=self.ring.p.mul(&old.0[0],&next.0[1]);let bc=self.ring.p.mul(&old.0[1],&next.0[0]);let determinant=self.ring.p.sub(&ad,&bc);g=self.ring.p.gcd_monic(&g,&determinant,bound).map_err(|_|Error::Arithmetic)?.polynomial;}
  self.ring.g=g;if self.ring.g.len()==1{self.constraints.clear();return Ok(())}
  self.constraints.push(next);for pair in &mut self.constraints{for coefficient in &mut pair.0{*coefficient=self.ring.p.divrem(coefficient,&self.ring.g).map_err(|_|Error::Arithmetic)?.1;}}
  Ok(())
 }
}
struct PersistentImage{r:u64,m:usize,t0:u64,inv_t0:u64,orientation:ReciprocalMode,branches:Vec<OrientedBranch>,zero:bool,rounds:usize}
std::thread_local!{static FIBER_COUNTS:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl PersistentImage{
 fn new(modes:&RecoveryModes,b:u64,f:&[u64],i:u64)->Result<Self,Error>{
  let r=modes.r;if modes.m<3||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r)||i>=r||mul(i,i,r)!=r-1{return Err(Error::Domain)}
  let mut orientation=ReciprocalMode::build(modes,((r-1)/modes.m as u64)as usize)?.ok_or(Error::Domain)?;
  let a=mul(modes.m as u64,pow(b,(r-1)/2,r),r);let t0=pow(pow(b,modes.m as u64,r),r-2,r);let c=mul(f[0],pow(a,r-2,r),r);let mut branches=Vec::new();
  for u in [1,r-1,i,r-i]{let g=squarefree(r,&orientation.branch(u,c)?)?;if g.len()==1{continue}let mut ring=QuadRing::new(r,&g)?;let mut phase=ring.power_v(orientation.n)?;phase.0[0][0]=(phase.0[0][0]+r-u)%r;let mut branch=OrientedBranch{u,ring,constraints:Vec::new()};branch.keep(phase)?;branches.push(branch);}
  // Only r,n are needed by the orientation lift after the initial branch construction.
  orientation.h=Vec::new();orientation.dn=Vec::new();
  let mut image=Self{r,m:modes.m,t0,inv_t0:pow(t0,r-2,r),orientation,branches,zero:true,rounds:0};image.absorb(modes,b,f,true)?;
  FIBER_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});Ok(image)
 }
 fn update(&mut self,modes:&RecoveryModes,b:u64,f:&[u64])->Result<(),Error>{self.absorb(modes,b,f,false)?;FIBER_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});Ok(())}
 fn absorb(&mut self,modes:&RecoveryModes,b:u64,f:&[u64],first:bool)->Result<(),Error>{
  let r=self.r;if modes.r!=r||modes.m!=self.m||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if self.rounds>=4{return Err(Error::Limit)}
  let h=(r-1)/2;let a=mul(self.m as u64,pow(b,h,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);let c1=r-h;let c2=mul(mul(h,h-1,r),(r+1)/2,r);
  self.zero=self.zero&&f[0]==a&&mul(left,c2,r)==mul(right,mul(c1,c1,r),r);
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}
   if !first{let value=branch.ring.mode_scaled(modes,0,scale)?;let mut c0=branch.ring.scale(&value,a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}
   if branch.ring.g.len()==1{continue}let f1=branch.ring.mode_scaled(modes,1,scale)?;let f2=branch.ring.mode_scaled(modes,2,scale)?;let sq=branch.ring.multiply(&f1,&f1)?;let l=branch.ring.scale(&f2,left);let rr=branch.ring.scale(&sq,right);let c2=branch.ring.sub(&l,&rr);branch.keep(c2)?;
  }self.rounds+=1;Ok(())
 }
 fn materialize(&mut self,lift_cap:usize,image_cap:usize)->Result<Vec<u64>,Error>{
  if self.zero&&image_cap==0{return Err(Error::Limit)}let mut out=if self.zero{vec![0,1]}else{vec![1]};let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}let mut lift=self.orientation.lift(&branch.ring.g,branch.u,lift_cap)?;for pair in &branch.constraints{lift=branch.ring.filter_lift(pair,&lift)?;}if out.len()-1+lift.len()-1>image_cap{return Err(Error::Limit)}out=p.mul(&out,&lift);}
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,self.r);lead=scale;scale=mul(scale,self.t0,self.r);}let inv=pow(lead,self.r-2,self.r);for x in &mut out{*x=mul(*x,inv,self.r)}Ok(out)
 }
}
include!("persistent-adapter.rs");
include!("persistent-checks.rs");

include!("canonical.rs");
