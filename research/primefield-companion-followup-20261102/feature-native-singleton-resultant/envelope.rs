fn monic_rotate(g:&[u64],z:u64,r:u64)->Vec<u64>{let mut scale=pow(pow(z,(g.len()-1)as u64,r),r-2,r);g.iter().map(|&a|{let v=mul(a,scale,r);scale=mul(scale,z,r);v}).collect()}
#[derive(Default,Debug)]struct EnvelopeStats{stages:usize,degrees:Vec<usize>}
fn stable_envelope(p:&mut polynomial::Poly,g:&[u64],parent:&[u64],zeta:u64,m:usize,r:u64,cap:usize)->Result<(Vec<u64>,EnvelopeStats),Error>{
 if g.is_empty()||parent.is_empty()||g.last()!=Some(&1)||parent.last()!=Some(&1)||g.iter().chain(parent).any(|&a|a>=r)||m<2||m as u64>=r||(r-1)%m as u64!=0||zeta==0||zeta>=r{return Err(Error::Domain)}
 if g.len()-1>cap{return Err(Error::Limit)}let mut z=1;for j in 1..=m{z=mul(z,zeta,r);if z==1&&j!=m{return Err(Error::Domain)}}if z!=1||monic_rotate(parent,zeta,r)!=parent{return Err(Error::Domain)}
 if p.divrem(parent,g).map_err(|_|Error::Arithmetic)?.1!=[0]{return Err(Error::Domain)}
 let bound=cap.min(parent.len()-1);let(mut out,mut span,mut z)=(g.to_vec(),1usize,zeta);let mut stats=EnvelopeStats{degrees:vec![g.len()-1],..Default::default()};
 while span<m{if monic_rotate(&out,zeta,r)==out{break}let rotated=monic_rotate(&out,z,r);let common=p.gcd_monic(&out,&rotated,parent.len()-1).map_err(|_|Error::Arithmetic)?.polynomial;
  let degree=(out.len()-1).checked_add(rotated.len()-1).ok_or(Error::Limit)?.checked_sub(common.len()-1).ok_or(Error::Arithmetic)?;if degree>bound{return Err(Error::Limit)}
  let(q,rem)=p.divrem(&rotated,&common).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}out=p.mul(&out,&q);if out.len()-1!=degree||p.divrem(parent,&out).map_err(|_|Error::Arithmetic)?.1!=[0]{return Err(Error::Arithmetic)}
  stats.stages+=1;stats.degrees.push(degree);span=span.checked_mul(2).ok_or(Error::Limit)?;z=mul(z,z,r);
 }
 if monic_rotate(&out,zeta,r)!=out{return Err(Error::Arithmetic)}Ok((out,stats))
}
std::thread_local!{static ENVELOPE_PROFILE:std::cell::RefCell<Vec<(usize,usize,usize,usize)>>=const{std::cell::RefCell::new(Vec::new())};}
fn envelope_history(p:&mut polynomial::Poly,initial:&[u64],history:&[(u64,u64)],m:usize,zeta:u64,r:u64,cap:usize)->Result<(Vec<u64>,usize),Error>{
 // Reuse all original orbit/history validation, without computing its residues.
 if m<2||initial.len()<2||initial.len()-1>cap||history.is_empty()||history[0].0!=0||(history.len()-1)%m!=0||history.iter().any(|&(a,s)|a>=r||s!=0&&s!=1&&s!=r-1){return Err(Error::Domain)}
 let(_,_) =stable_envelope(p,initial,initial,zeta,m,r,cap)?;
 for chunk in history[1..].chunks(m){if chunk[0].0==0{return Err(Error::Domain)}let mut a=chunk[0].0;for &(offset,_)in chunk{if offset!=a{return Err(Error::Domain)}a=mul(a,zeta,r)}}
 let mut state=replay(p,initial,0,history[0].1,r,cap)?;let mut powers=1;
 let inverse=pow(zeta,r-2,r);let mut scale=pow(zeta,(r-1)/2,r);let factors:Vec<_>=(0..initial.len()-1).map(|_|{let s=scale;scale=mul(scale,inverse,r);s}).collect();
 for chunk in history[1..].chunks(m){let(envelope,stats)=stable_envelope(p,&state,initial,zeta,m,r,cap)?;
  ENVELOPE_PROFILE.with(|v|v.borrow_mut().push((initial.len()-1,state.len()-1,envelope.len()-1,stats.stages)));
  if state==[1]{return Ok((state,powers))}let mut residue=character_residue(p,&envelope,chunk[0].0,r)?;powers+=1;
  for &(_,sign)in chunk{let mut c=residue.clone();c[0]=(c[0]+r-sign)%r;state=p.gcd_monic(&state,&c,cap).map_err(|_|Error::Arithmetic)?.polynomial;for(i,c)in residue.iter_mut().enumerate(){*c=mul(*c,factors[i],r)}}
 }Ok((state,powers))
}
include!("dynamic.rs");
fn closure_checks(){let(mut subsets,mut zeros,mut stages,mut cap_rejects,mut full_envelopes)=(0,0,0,0,0);
 for r in [7,11,13]{let parent:Vec<_>=(0..=r).map(|i|if i==1{r-1}else if i==r{1}else{0}).collect();for m in 2..r as usize{if(r-1)%m as u64!=0{continue}let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let mut p=polynomial::Poly::new(r).unwrap();
  for mask in 0..1usize<<r{let roots:Vec<_>=(0..r).filter(|&x|mask&(1usize<<x)!=0).collect();let g=if roots.is_empty(){vec![1]}else{p.product_tree(&roots).unwrap().polynomial().to_vec()};let(h,s)=stable_envelope(&mut p,&g,&parent,zeta,m,r,r as usize).unwrap();
   let expected:Vec<_>=(0..r).filter(|&x|{let mut y=x;for _ in 0..m{if roots.contains(&y){return true}y=mul(y,zeta,r)}false}).collect();let exact=if expected.is_empty(){vec![1]}else{p.product_tree(&expected).unwrap().polynomial().to_vec()};assert_eq!(h,exact);assert_eq!(p.divrem(&h,&g).unwrap().1,vec![0]);assert!(s.degrees.iter().all(|&n|n<=r as usize));assert!(s.stages<=(usize::BITS-(m-1).leading_zeros())as usize);subsets+=1;stages+=s.stages;if roots.contains(&0){zeros+=1}if h==parent{full_envelopes+=1}
   if h.len()>g.len(){assert_eq!(stable_envelope(&mut p,&g,&parent,zeta,m,r,h.len()-2).err(),Some(Error::Limit));cap_rejects+=1}
  }
 }}let mut p=polynomial::Poly::new(13).unwrap();assert_eq!(stable_envelope(&mut p,&[1,1],&[0,1],3,3,13,13).err(),Some(Error::Domain));println!("envelope_subsets={subsets} zero_subsets={zeros} closure_stages={stages} intermediate_cap_rejections={cap_rejects} full_parent_envelopes={full_envelopes}");}
fn zero_history_checks(){let mut histories=0;
 for r in [7,11,13]{let parent:Vec<_>=(0..=r).map(|i|if i==1{r-1}else if i==r{1}else{0}).collect();for m in 2..r as usize{if(r-1)%m as u64!=0{continue}let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let mut p=polynomial::Poly::new(r).unwrap();
 for target in 0..r{let mut history=vec![(0,pow(target,(r-1)/2,r))];for a in [1,2]{let mut z=1;for _ in 0..m{let offset=mul(a,z,r);history.push((offset,pow((target+offset)%r,(r-1)/2,r)));z=mul(z,zeta,r)}}let fixed=orbit_history(&mut p,&parent,&history,m,zeta,r,r as usize).unwrap().0;let dynamic=envelope_history(&mut p,&parent,&history,m,zeta,r,r as usize).unwrap().0;assert_eq!(fixed,dynamic);assert_eq!(eval(&dynamic,target,r),0);histories+=1;
 }}}
 let mut p=polynomial::Poly::new(13).unwrap();assert!(matches!(envelope_history(&mut p,&[0,1],&[(0,0)],0,1,13,1),Err(Error::Domain)));println!("dynamic_complete_histories={histories}");}
fn envelope_checks(){orbit_checks();closure_checks();zero_history_checks();let(mut cases,mut calls,mut powers,mut blocks,mut reduced,mut same,mut stages)=(0,0,0,0,0,0,0);
 for(p,a,b,r,base,m)in[(7,0,3,13,[1,2],3),(7,0,3,13,[1,2],6),(509,2,10,479,[3,68],2),(701,2,8,683,[2,106],11)]{let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=Modes::compile(r,m,z,16,256).unwrap();let scalars:Vec<u64>=if r==13{(0..r).collect()}else if r==479{include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1).map(|l|l.split('\t').next().unwrap().parse().unwrap()).collect()}else{vec![0,1,2,3,7,11,17,31,47,83,127,191,257,341,509,681,682]};
  for k in scalars{let t=multiple(c,bp,k);let extra:Vec<_>=(0..17.min(r)).collect();let(old,os,route)=modes.recover_orbit(&source,c,bp,t,&[1,2,3,4],64,&extra,128).unwrap();ORBIT_POWERS.with(|n|n.set(0));ENVELOPE_PROFILE.with(|n|n.borrow_mut().clear());let(new,ns,nroute)=modes.recover_enveloped(&source,c,bp,t,&[1,2,3,4],64,&extra,128).unwrap();assert_eq!(new,old);assert_eq!(new,Some(k));assert_eq!(route,nroute);assert_eq!((os.calls,os.replayed,os.extra,os.lifted,os.image_count),(ns.calls,ns.replayed,ns.extra,ns.lifted,ns.image_count));cases+=1;calls+=ns.calls;powers+=ORBIT_POWERS.with(|n|n.get());
   ENVELOPE_PROFILE.with(|v|for &(initial,current,envelope,n) in v.borrow().iter(){blocks+=1;stages+=n;if envelope<initial{reduced+=1}else{same+=1}println!("profile r={r} m={m} k={k} initial={initial} current={current} envelope={envelope} stages={n}")});
  }
 }println!("dynamic_public_equalities={cases} source_calls={calls} retained_powers={powers} envelope_blocks={blocks} reduced_blocks={reduced} unchanged_blocks={same} envelope_stages={stages}");}

include!("trace.rs");
