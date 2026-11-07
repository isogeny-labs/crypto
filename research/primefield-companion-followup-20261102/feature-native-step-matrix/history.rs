fn trace_history(p:&mut polynomial::Poly,initial:&[u64],history:&[(u64,u64)],m:usize,zeta:u64,r:u64,cap:usize)->Result<(Vec<u64>,usize),Error>{
 // Reuse all original orbit/history validation, without computing its residues.
 if m<2||initial.len()<2||initial.len()-1>cap||history.is_empty()||history[0].0!=0||(history.len()-1)%m!=0||history.iter().any(|&(a,s)|a>=r||s!=0&&s!=1&&s!=r-1){return Err(Error::Domain)}
 let(_,_) =stable_envelope(p,initial,initial,zeta,m,r,cap)?;
 for chunk in history[1..].chunks(m){if chunk[0].0==0{return Err(Error::Domain)}let mut a=chunk[0].0;for &(offset,_)in chunk{if offset!=a{return Err(Error::Domain)}a=mul(a,zeta,r)}}
 let mut state=replay(p,initial,0,history[0].1,r,cap)?;let mut powers=1;
 let inverse=pow(zeta,r-2,r);let mut scale=pow(zeta,(r-1)/2,r);let factors:Vec<_>=(0..initial.len()-1).map(|_|{let s=scale;scale=mul(scale,inverse,r);s}).collect();
 for chunk in history[1..].chunks(m){let data=adaptive_image_workspace(p,&state,m,r,cap,cap,cap.checked_mul(2).ok_or(Error::Limit)?,cap)?;let envelope=data.envelope;let stats=EnvelopeStats::default();if p.divrem(initial,&envelope).map_err(|_|Error::Arithmetic)?.1!=[0]{return Err(Error::Arithmetic)}
  ENVELOPE_PROFILE.with(|v|v.borrow_mut().push((initial.len()-1,state.len()-1,envelope.len()-1,stats.stages)));
  if state==[1]{return Ok((state,powers))}let mut residue=character_residue(p,&envelope,chunk[0].0,r)?;powers+=1;
  for &(_,sign)in chunk{let mut c=residue.clone();c[0]=(c[0]+r-sign)%r;state=p.gcd_monic(&state,&c,cap).map_err(|_|Error::Arithmetic)?.polynomial;for(i,c)in residue.iter_mut().enumerate(){*c=mul(*c,factors[i],r)}}
 }Ok((state,powers))
}