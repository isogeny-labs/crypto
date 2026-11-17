fn phase_union(image:&mut SeededImage,u:u64)->Vec<u64>{let mut p=polynomial::Poly::new(image.r).unwrap();let mut out=vec![1];for branch in &mut image.branches{if branch.u!=u||branch.ring.g.len()==1{continue}let v=branch.canonical.lift(&image.orientation,u,image.r as usize,&mut branch.ring).unwrap();out=p.mul(&out,&v);}out}
fn negative_split_checks(){let(mut trace_controls,mut factorizations,mut phase_unions,mut split_boundaries)=(0,0,0,0);
 for r in [17u64,41,73,97,193,257,281]{let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();assert_eq!(eighth_trace(r,0).unwrap(),None);let before=EIGHTH_SEARCH.with(|c|c.get().1);let root=eighth_trace(r,64).unwrap().unwrap();assert!(EIGHTH_SEARCH.with(|c|c.get().1)-before<=64);assert_eq!(mul(root,root,r),2);trace_controls+=1;let mut p=polynomial::Poly::new(r).unwrap();
  for n in (2..(r as usize).min(34)).step_by(2){let domains=split_negative_domains(r,n,i,root).unwrap();assert_eq!(domains.len(),4);let dn=initial_dickson(&mut p,n);for u in [i,r-i]{let parts:Vec<_>=domains.iter().filter(|x|x.0==u).collect();assert_eq!(parts.len(),2);assert_eq!(parts[0].1.len()-1,n/2);assert_eq!(p.mul(&parts[0].1,&parts[1].1),dn);assert_eq!(p.gcd_monic(&parts[0].1,&parts[1].1,n+2).unwrap().polynomial,vec![1]);for part in parts{assert_eq!(squarefree(r,&part.1).unwrap(),part.1);}factorizations+=1;}}
  assert!(matches!(split_negative_domains(r,2,i,0),Err(Error::Domain)));assert!(matches!(split_negative_domains(r,1,i,root),Err(Error::Domain)));assert!(matches!(split_negative_domains(r,2,1,root),Err(Error::Domain)));
 }
 assert_eq!(eighth_trace(17,1).unwrap(),None);assert_eq!(eighth_trace(13,64).unwrap(),None);assert!(matches!(eighth_trace(17,65),Err(Error::Domain)));
 let mut dickson_equalities=0;for r in [17u64,41,97,281]{let mut p=polynomial::Poly::new(r).unwrap();let(mut a,mut b)=(vec![2],vec![0,1]);for n in 0..33{assert_eq!(initial_dickson(&mut p,n),a);let wb=p.mul(&[0,1],&b);let next=p.sub(&wb,&a);a=b;b=next;dickson_equalities+=1;}}
 let(mut states,mut retained,mut wrong,mut zero,mut public,mut route_changes,mut call_changes,mut updates)=(0,0,0,0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5),(281,35)]{
  let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();let stream=StreamModes::new(r,m,zeta,64,2048,7).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();let l=((r-1)/2/m as u64)as usize;
  let n=((r-1)/(4*m as u64))as usize;let mut poly=polynomial::Poly::new(r).unwrap();let dn=initial_dickson(&mut poly,n);
  for u in [1,r-1,i,r-i]{let mut d=dn.clone();d[0]=(d[0]+r-(u+pow(u,3,r))%r)%r;let got=squarefree(r,&d).unwrap().len()-1;let expected=if u==1{n/2+1}else if u==r-1{(n+1)/2}else{n};assert_eq!(got,expected);}
  for sign in [1,r-1]{let degree=if sign==1{n/2+1}else{n};let bound=2*m*degree;assert!(SeededImage::initial_sections(&modes,1,&[0,0,0],i,bound,sign).is_ok());assert!(matches!(SeededImage::initial_sections(&modes,1,&[0,0,0],i,bound-1,sign),Err(Error::Limit)));}
if n%2==0{let bound=m*n;assert!(SeededImage::initial_split_negative(&modes,1,&[0,0,0],i,bound,r-1,64).is_ok());assert!(matches!(SeededImage::initial_split_negative(&modes,1,&[0,0,0],i,bound-1,r-1,64),Err(Error::Limit)));assert!(matches!(SeededImage::initial_split_negative(&modes,1,&[0,0,0],i,bound,r-1,0),Err(Error::Limit)));assert!(SeededImage::initial_split_negative(&modes,1,&[0,0,0],i,2*bound,r-1,0).is_ok());split_boundaries+=1;}
    let ks:Vec<_>=if m==35{vec![0,1,2,3,7,17,r-1]}else{(0..r).collect()};
  for k in ks{for slots in [0usize,16,96,1024]{for sign in [1,r-1]{let(mut old,mut new):(Option<SeededImage>,Option<SeededImage>)=(None,None);
   for b in [2,1,3,2]{let mut z=1;let signs:Vec<_>=(0..m).map(|_|{let s=pow((k+r-mul(b,z,r))%r,(r-1)/2,r);z=mul(z,zeta,r);s}).collect();let pair=modes.signed_observed(&signs).unwrap();let f=[signs.iter().fold(0,|a,&s|(a+s)%r),pair[0],pair[1]];
    if let Some(old)=&mut old{old.update(&modes,b,&f).unwrap();new.as_mut().unwrap().update(&modes,b,&f).unwrap();}else{
     assert!(matches!(SeededImage::new_split_negative(&modes,b,&f,i,slots,0,64),Err(Error::Domain)));assert!(matches!(SeededImage::new_split_negative(&modes,b,&f,i,slots,2,64),Err(Error::Domain)));
     old=Some(SeededImage::new_direct_phase(&modes,b,&f,i,slots,sign).unwrap());new=Some(SeededImage::new_split_negative(&modes,b,&f,i,slots,sign,64).unwrap());
     let normalized=mul(sign,pow(b,(r-1)/2,r),r);assert_eq!([1,r-1,i,r-i].into_iter().filter(|&u|mul(u,u,r)==normalized).count(),2);
    }
    let(old,new)=(old.as_mut().unwrap(),new.as_mut().unwrap());let dense=old.materialize(r as usize,r as usize).unwrap();let got=new.materialize(r as usize,r as usize).unwrap();let mut condition=vec![0;l+1];condition[0]=r-sign;condition[l]=1;let mut p=polynomial::Poly::new(r).unwrap();let expected=p.gcd_monic(&dense,&condition,2*r as usize).unwrap().polynomial;
    assert_eq!(got,expected,"r={r} m={m} k={k} b={b} slots={slots} sign={sign}");assert!(!new.zero);assert_eq!(old.rounds,new.rounds);states+=1;
    if new.rounds==4{for u in [1,r-1,i,r-i]{assert_eq!(phase_union(old,u),phase_union(new,u));phase_unions+=1;}}
    if k==0{assert_ne!(eval(&got,0,r),0);zero+=1;}else if sign==pow(k,(r-1)/2,r){assert_eq!(eval(&got,pow(k,m as u64,r),r),0);retained+=1;}else{assert_ne!(eval(&got,pow(k,m as u64,r),r),0);wrong+=1;}
   }
  }}}
 }
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();
 for cap in [8usize,16,64]{for k in 0..r{let target=multiple(curve,bp,k);for slots in [0usize,16,96,1024]{let old=modes.recover_direct_phase_image(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128,slots).unwrap();SEEDED_IMAGE_COUNTS.with(|c|c.set((0,0,0,0)));let new=modes.recover_split_negative_image(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128,slots,64).unwrap();assert_eq!(old.0,Some(k));assert_eq!(new.0,Some(k));assert_eq!(old.2,new.2);assert_eq!((old.1.calls,old.1.extra,old.1.replayed,old.1.lifted,old.1.image_count),(new.1.calls,new.1.extra,new.1.replayed,new.1.lifted,new.1.image_count));assert!(new.1.calls<=128);route_changes+=usize::from(old.2!=new.2);call_changes+=usize::from(old.1.calls!=new.1.calls);updates+=SEEDED_IMAGE_COUNTS.with(|c|c.get().1);public+=1;
  if k==0{assert_eq!(new.1.calls,1);assert_eq!(new.2,"identity");}
 }}}
 // Exhausting the budget immediately must not issue a second sign query.
 for k in 1..r{let q=modes.recover_split_negative_image(&source,curve,bp,multiple(curve,bp,k),&[1,2],64,&extra,1,1024,64).unwrap();assert_eq!(q.0,None);assert_eq!(q.1.calls,1);assert_eq!(q.2,"call-cap");}
 assert!(retained>0&&wrong>0&&zero>0&&updates>0);
 println!("support_equalities={states} true_images_retained={retained} wrong_sign_exclusions={wrong} zero_exclusions={zero} public_target_capacity_cases={public} route_changes={route_changes} call_changes={call_changes} subsequent_updates={updates} single_call_cap_cases=280 invalid_signs=rejected phase_choices=2_of_4");
 let(powered,fallback)=SPLIT_INITIAL_POWER.with(|c|c.get());assert!(powered>0&&fallback>0);println!("dickson_equalities={dickson_equalities} powered_initializations={powered} streaming_fallbacks={fallback} public_outcomes_routes_stats_equal=true prior_phase_degree_checks=24 prior_capacity_boundaries=12");
 for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_direct_phase_image(&source,curve,bp,target,&[1,2,3,4],64,&extra,128,1024).unwrap();let new=modes.recover_split_negative_image(&source,curve,bp,target,&[1,2,3,4],64,&extra,128,1024,0).unwrap();assert_eq!(new.0,Some(k));assert_eq!(new.2,old.2);assert_eq!(new.1.calls,old.1.calls);}
 assert!(matches!(modes.recover_split_negative_image(&source,curve,bp,None,&[1],64,&extra,128,1024,65),Err(Error::Domain)));
 let(searches,candidates,roots)=EIGHTH_SEARCH.with(|c|c.get());let splits=NEGATIVE_SPLITS.with(|c|c.get());assert!(splits>0&&roots>0);println!("trace_field_controls={trace_controls} coprime_factorizations={factorizations} phase_union_equalities={phase_unions} zero_search_public_cases={r} root_searches={searches} candidate_tests={candidates} roots_found={roots} split_domain_builds={splits} exhausted_search_fallback=pass split_capacity_boundary_groups={split_boundaries}");
}
