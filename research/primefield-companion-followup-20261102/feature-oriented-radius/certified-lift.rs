// Internal only: g must be a squarefree divisor of D_L-2epsilon.
// Its provenance is the character factory followed exclusively by intersections.
fn certified_reciprocal_lift(r:u64,g:&[u64],cap:usize)->Result<Vec<u64>,Error>{
 if g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&v|v>=r){return Err(Error::Domain)}
 let d=g.len()-1;if d.checked_mul(2).ok_or(Error::Limit)?>cap{return Err(Error::Limit)}
 if d==0{return Ok(vec![1])}let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
 let mut lifted=vec![1];for j in 1..=d{lifted=p.mul(&lifted,&[1,0,1]);lifted[j]=(lifted[j]+g[d-j])%r;}
 // W=+/-2 pulls back to (V-/+1)^2. Remove exactly one copy.
 for(w,v)in[(2,1),(r-2,r-1)]{if eval(g,w,r)==0{let(q,rem)=p.divrem(&lifted,&[r-v,1]).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}lifted=q;}}
 Ok(lifted)
}
fn certified_fiber_lift(fiber:&CanonicalFiber,cap:usize,ring:&mut QuadRing)->Result<Vec<u64>,Error>{
 let both=certified_reciprocal_lift(fiber.r,&fiber.both,cap)?;
 let one=certified_reciprocal_lift(fiber.r,&fiber.one,cap)?;
 let negative:Vec<_>=fiber.root.iter().map(|&v|(fiber.r-v)%fiber.r).collect();
 let one=ring.filter_lift(&Quad([negative,vec![1]]),&one)?;
 if both.len()-1+one.len()-1>cap{return Err(Error::Limit)}Ok(ring.p.mul(&both,&one))
}
fn certified_lift_checks(){let(mut lifts,mut endpoints,mut phase_equalities,mut limits)=(0,0,0,0);
 for r in [7u64,13,17,19,31,37,41,97,193,281]{let mut p=polynomial::Poly::new(r).unwrap();
  for l in 1..=(r as usize-1).min(32){for eps in [1,r-1]{for g in character_domains(r,l,eps,3,1024,64).unwrap(){
   let orientation=ReciprocalMode{r,n:l,h:vec![],dn:vec![]};let d=g.len()-1;
   let got=certified_reciprocal_lift(r,&g,2*d).unwrap();assert_eq!(got,orientation.lift(&g,eps,2*d).unwrap());assert_eq!(squarefree(r,&got).unwrap(),got);lifts+=1;
   for(w,v)in[(2,1),(r-2,r-1)]{assert_eq!(eval(&got,v,r)==0,eval(&g,w,r)==0);endpoints+=1;}
   if d>0{assert!(matches!(certified_reciprocal_lift(r,&g,2*d-1),Err(Error::Limit)));limits+=1;}
   let mut ring=QuadRing::new(r,&g).unwrap();let raw=CanonicalFiber::new(r,&g).unwrap();let before=canonical_signature(&raw);let mut filtered=CanonicalFiber::new(r,&g).unwrap();let mut phase=ring.power_v(l).unwrap();phase.0[0][0]=(phase.0[0][0]+r-eps)%r;filtered.keep(&mut p,&phase).unwrap();assert_eq!(before,canonical_signature(&filtered));phase_equalities+=1;
  }}}
 }
 // Certificate is necessary: arbitrary squarefree W factors are not admissible.
 let orientation=ReciprocalMode{r:19,n:3,h:vec![],dn:vec![]};assert_ne!(certified_reciprocal_lift(19,&[0,1],2).unwrap(),orientation.lift(&[0,1],1,2).unwrap());
 println!("certificate_lift_equalities={lifts} endpoint_controls={endpoints} initial_phase_noop_equalities={phase_equalities} exact_cap_controls={limits} arbitrary_domain_negative_control=pass");
}
