std::thread_local!{static DIRECT_INITIAL_POWER:std::cell::Cell<(usize,usize)>=const{std::cell::Cell::new((0,0))};}
impl SeededImage{
 fn new_direct_phase(modes:&RecoveryModes,b:u64,f:&[u64],i:u64,slot_cap:usize,initial_character:u64)->Result<Self,Error>{
  match Self::initial_direct_phase(modes,b,f,i,slot_cap,initial_character){Ok(x)=>{DIRECT_INITIAL_POWER.with(|c|{let(a,b)=c.get();c.set((a+1,b))});Ok(x)},Err(Error::Limit)=>{DIRECT_INITIAL_POWER.with(|c|{let(a,b)=c.get();c.set((a,b+1))});Self::new(modes,b,f,i,slot_cap,initial_character)},Err(e)=>Err(e)}
 }
 fn initial_direct_phase(modes:&RecoveryModes,b:u64,f:&[u64],i:u64,slot_cap:usize,initial_character:u64)->Result<Self,Error>{
  let r=modes.r;let m=modes.m;
  if m<3||((r-1)/m as u64)%4!=0||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r)||i>=r||mul(i,i,r)!=r-1||(initial_character!=1&&initial_character!=r-1){return Err(Error::Domain)}
  let n=usize::try_from((r-1)/(4*m as u64)).map_err(|_|Error::Limit)?;
  // Exact maximum squarefree phase degree; output slots, not peak RAM.
  let bh=pow(b,(r-1)/2,r);let normalized=mul(initial_character,bh,r);let phase_degree=if normalized==1{n/2+1}else{n};
  if n==0||m.checked_mul(2).and_then(|x|x.checked_mul(phase_degree)).ok_or(Error::Limit)?>slot_cap{return Err(Error::Limit)}
  let domains=direct_phase_domains(r,n,i,normalized)?;
  let a=mul(m as u64,bh,r);let mut branches=Vec::new();
  for (u,g) in domains{
   if g.len()==1{continue}
   let mut ring=QuadRing::new(r,&g)?;let mut phase=ring.power_v(n)?;phase.0[0][0]=(phase.0[0][0]+r-u)%r;
   let mut branch=CanonicalBranch{u,ring,canonical:CanonicalFiber::new(r,&g)?};branch.keep(phase)?;
   if branch.ring.g.len()==1{continue}
   let variable=Quad([vec![0],vec![1]]);let values=branch.ring.sections(m,&variable,slot_cap)?;
   let mut mode0=branch.ring.scale(&values[0],a);mode0.0[0][0]=(mode0.0[0][0]+r-f[0])%r;branch.keep(mode0)?;
   if branch.ring.g.len()==1{continue}
   let product=branch.ring.multiply(&values[1],&values[m-1])?;let product=branch.ring.multiply(&product,&variable)?;
   let mut equation=branch.ring.scale(&product,mul(a,a,r));equation.0[0][0]=(equation.0[0][0]+r-mul(f[1],f[2],r))%r;branch.keep(equation)?;branches.push(branch);
  }
  let t0=pow(pow(b,m as u64,r),r-2,r);
  SEEDED_IMAGE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});
  Ok(Self{r,m,t0,inv_t0:pow(t0,r-2,r),orientation:ReciprocalMode{r,n,h:Vec::new(),dn:Vec::new()},branches,zero:false,rounds:1,slot_cap})
 }
}
