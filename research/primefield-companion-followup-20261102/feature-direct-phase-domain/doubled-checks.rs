fn doubled_checks(){
 let(mut mode_values,mut reindexed,mut wrong_sign,mut sections,mut rejects)=(0,0,0,0,0);
 for(r,m)in[(41u64,5usize),(73,3),(97,3),(193,3),(241,3),(241,5),(281,5)]{
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let eta=r-pow(z,((m+1)/2)as u64,r);
  assert_eq!(pow(eta,2,r),z);assert_eq!(pow(eta,m as u64,r),r-1);
  let small=StreamModes::new(r,m,z,32,1024,7).unwrap();let big=StreamModes::new(r,2*m,eta,32,1024,7).unwrap();let dense=Modes::compile(r,m,z,32,1024).unwrap();let dense2=Modes::compile(r,2*m,eta,32,1024).unwrap();
  for e in 0..3{let even:Vec<_>=dense.coeff[e].iter().step_by(2).copied().collect();assert_eq!(even,dense2.coeff[e]);sections+=1;}
  for x in 0..r{for b in [1,2,3]{let obs=|rad|{let a=mul(m as u64,pow(rad,(r-1)/2,r),r);let y=mul(x,pow(rad,r-2,r),r);let v=pow(y,m as u64,r);(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect::<Vec<_>>()};let plus=obs(b);let minus=obs(r-b);let got=doubled_observation(r,&plus,&minus).unwrap();let y=mul(x,pow(b,r-2,r),r);let a=mul((2*m)as u64,pow(b,(r-1)/2,r),r);let v=pow(y,(2*m)as u64,r);for e in 0..3{assert_eq!(got[e],mul(mul(a,pow(y,e as u64,r),r),eval(&dense2.coeff[e],v,r),r));mode_values+=1;}}}
  // Exhaust all 3^6 sign vectors for m=3; a fixed 729-vector panel for m=5.
  for seed in 0..729usize{let mut code=seed;let mut signs=vec![0;2*m];for s in &mut signs{*s=[0,1,r-1][code%3];code/=3;}
   let plus=&signs[..m];let minus=&signs[m..];let a=small.observed(plus).unwrap();let b=small.observed(minus).unwrap();let combined=doubled_observation(r,&a,&b).unwrap();
   let mut full=Vec::new();for j in 0..m{full.push(plus[j]);full.push(minus[(j+(m+1)/2)%m]);}
   assert_eq!(combined,big.observed(&full).unwrap());reindexed+=1;if (a[1]+b[1])%r!=combined[1]{wrong_sign+=1;}
  }
  let session=RecoveryModes::from_modes(&small).unwrap();let i=(2..r).find(|&v|mul(v,v,r)==r-1).unwrap();assert!(session.doubled_phase(0,&[0;3],&[0;3],i,64).is_err());rejects+=1;
 }
 for (a,b) in [(vec![0;2],vec![0;3]),(vec![0;3],vec![0;4]),(vec![41,0,0],vec![0;3])]{assert!(doubled_observation(41,&a,&b).is_err());rejects+=1;}
 assert!(wrong_sign>0);println!("doubled_mode_values={mode_values} reindexed_sign_vectors={reindexed} wrong_sign_controls={wrong_sign} coefficient_sections={sections} invalid_rejections={rejects}");
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();
 for cap in [8usize,10,16,20,32,64]{DOUBLED_COUNTS.with(|c|c.set([0;7]));let(mut calls,mut old_calls,mut changes,mut recovered,mut old_recovered,mut early,mut saved_targets)=(0,0,0,0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_direct(&source,curve,bp,target,&[1,r-1,2,3],cap,&extra,128).unwrap();let new=modes.recover_doubled(&source,curve,bp,target,&[1,r-1,2,3],cap,&extra,128).unwrap();if let Some(v)=old.0{assert_eq!(v,k);old_recovered+=1;}if let Some(v)=new.0{assert_eq!(v,k);recovered+=1;}assert!(new.1.calls<=old.1.calls);if old.0.is_some(){assert!(new.0.is_some());}if old.1.calls<=1+m{assert_eq!(new.0,old.0);assert_eq!(new.1.calls,old.1.calls);assert_eq!(new.2,old.2);early+=1;}if new.1.calls<old.1.calls{saved_targets+=1;}if new.2!=old.2{changes+=1;}calls+=new.1.calls;old_calls+=old.1.calls;
  }
  let c=DOUBLED_COUNTS.with(|c|c.get());println!("doubled_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} recovered={recovered} reference_recovered={old_recovered} route_changes={changes} preserved_early_exits={early} targets_fewer_calls={saved_targets} attempts={} contexts={} cap_misses={} non_singletons={} zero_denominators={} phase_candidates={} upper_f0_reads={}",c[0],c[1],c[2],c[3],c[4],c[5],c[6]);
 }
 let mut fallbacks=0;for cap in [8usize,16,64]{for k in [0,1,17,140,280]{let target=multiple(curve,bp,k);let old=modes.recover_direct(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_doubled(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();assert_eq!(old.0,new.0);assert_eq!(old.1.calls,new.1.calls);assert_eq!(old.2,new.2);fallbacks+=1;}}
 println!("doubled_nonopposite_fallbacks={fallbacks}");
}
