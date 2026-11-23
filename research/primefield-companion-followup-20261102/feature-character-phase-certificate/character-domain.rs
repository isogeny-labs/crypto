struct CharacterBranch{epsilon:u64,ring:QuadRing,canonical:CanonicalFiber}
impl CharacterBranch{fn keep(&mut self,pair:Quad)->Result<(),Error>{self.canonical.keep(&mut self.ring.p,&pair)?;self.ring.g=self.canonical.support(&mut self.ring.p);Ok(())}}
fn character_domains(r:u64,l:usize,epsilon:u64,m:usize,slots:usize,trials:usize)->Result<Vec<Vec<u64>>,Error>{
 if r<7||r>u64::MAX/2||l==0||l as u64>=r||m<3||(epsilon!=1&&epsilon!=r-1)||trials>64{return Err(Error::Domain)}
 let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
 if l%2==1{let(a,b)=second_pair(&mut p,l/2);return Ok(vec![if epsilon==1{let sum=p.add(&a,&b);p.mul(&[r-2,1],&sum)}else{let delta=p.sub(&a,&b);p.mul(&[2,1],&delta)}])}
 let n=l/2;
 if epsilon==1{
  let(a,b)=second_pair(&mut p,n/2);let(plus,minus)=if n%2==0{let plus=p.mul(&[r-4,0,1],&b);let twice=p.add(&a,&a);let wb=p.mul(&[0,1],&b);(plus,p.sub(&twice,&wb))}else{let sum=p.add(&a,&b);let delta=p.sub(&a,&b);(p.mul(&[r-2,1],&sum),p.mul(&[2,1],&delta))};return Ok(vec![plus,minus])
 }
 if n%2==0&&m.checked_mul(n).ok_or(Error::Limit)?<=slots{if let Some(s)=eighth_trace(r,trials)?{let dt=initial_dickson(&mut p,n/2);let mut minus=dt.clone();minus[0]=(minus[0]+r-s)%r;let mut plus=dt;plus[0]=(plus[0]+s)%r;return Ok(vec![minus,plus])}}
 Ok(vec![initial_dickson(&mut p,n)])
}
impl CharacterImage{
 fn new(modes:&RecoveryModes,b:u64,f:&[u64],slot_cap:usize,initial_character:u64,trace_trials:usize)->Result<Self,Error>{
  let r=modes.r;let m=modes.m;let h=(r-1)/2;
  if m<3||h%m as u64!=0||b==0||b>=r||f.len()!=3||f.iter().any(|&v|v>=r)||(initial_character!=1&&initial_character!=r-1)||trace_trials>64{return Err(Error::Domain)}
  let l=usize::try_from(h/m as u64).map_err(|_|Error::Limit)?;let epsilon=mul(initial_character,pow(b,h,r),r);
  let domains=character_domains(r,l,epsilon,m,slot_cap,trace_trials)?;let mut branches=Vec::new();
  for g in domains{let mut ring=QuadRing::new(r,&g)?;let mut phase=ring.power_v(l)?;phase.0[0][0]=(phase.0[0][0]+r-epsilon)%r;let mut branch=CharacterBranch{epsilon,ring,canonical:CanonicalFiber::new(r,&g)?};branch.keep(phase)?;branches.push(branch);}
  let t0=pow(pow(b,m as u64,r),r-2,r);
  let mut image=Self{r,m,t0,inv_t0:pow(t0,r-2,r),orientation:ReciprocalMode{r,n:l,h:Vec::new(),dn:Vec::new()},branches,zero:false,rounds:0,slot_cap};
  // Unlike the old constructor, no mode0 equation was imposed before absorb.
  image.absorb(modes,b,f,false)?;CHARACTER_IMAGE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});Ok(image)
 }
}
