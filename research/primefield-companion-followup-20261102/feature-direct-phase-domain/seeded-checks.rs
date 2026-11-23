fn seeded_checks(){let(mut states,mut retained,mut wrong,mut zero,mut public,mut route_changes,mut call_changes,mut updates)=(0,0,0,0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5),(281,35)]{
  let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();let stream=StreamModes::new(r,m,zeta,64,2048,7).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();let l=((r-1)/2/m as u64)as usize;
  let ks:Vec<_>=if m==35{vec![0,1,2,3,7,17,r-1]}else{(0..r).collect()};
  for k in ks{for slots in [0usize,16,1024]{for sign in [1,r-1]{let(mut old,mut new):(Option<PoweredImage>,Option<SeededImage>)=(None,None);
   for b in [2,1,3,2]{let mut z=1;let signs:Vec<_>=(0..m).map(|_|{let s=pow((k+r-mul(b,z,r))%r,(r-1)/2,r);z=mul(z,zeta,r);s}).collect();let pair=modes.signed_observed(&signs).unwrap();let f=[signs.iter().fold(0,|a,&s|(a+s)%r),pair[0],pair[1]];
    if let Some(old)=&mut old{old.update(&modes,b,&f).unwrap();new.as_mut().unwrap().update(&modes,b,&f).unwrap();}else{
     assert!(matches!(SeededImage::new(&modes,b,&f,i,slots,0),Err(Error::Domain)));assert!(matches!(SeededImage::new(&modes,b,&f,i,slots,2),Err(Error::Domain)));
     old=Some(PoweredImage::new(&modes,b,&f,i,slots).unwrap());new=Some(SeededImage::new(&modes,b,&f,i,slots,sign).unwrap());
     let normalized=mul(sign,pow(b,(r-1)/2,r),r);assert_eq!([1,r-1,i,r-i].into_iter().filter(|&u|mul(u,u,r)==normalized).count(),2);
    }
    let(old,new)=(old.as_mut().unwrap(),new.as_mut().unwrap());let dense=old.materialize(r as usize,r as usize).unwrap();let got=new.materialize(r as usize,r as usize).unwrap();let mut condition=vec![0;l+1];condition[0]=r-sign;condition[l]=1;let mut p=polynomial::Poly::new(r).unwrap();let expected=p.gcd_monic(&dense,&condition,2*r as usize).unwrap().polynomial;
    assert_eq!(got,expected,"r={r} m={m} k={k} b={b} slots={slots} sign={sign}");assert!(!new.zero);assert_eq!(old.rounds,new.rounds);states+=1;
    for branch in &new.branches{let prior=old.branches.iter().find(|a|a.u==branch.u).unwrap();assert_eq!(canonical_signature(&prior.canonical),canonical_signature(&branch.canonical));}
    if k==0{assert_ne!(eval(&got,0,r),0);zero+=1;}else if sign==pow(k,(r-1)/2,r){assert_eq!(eval(&got,pow(k,m as u64,r),r),0);retained+=1;}else{assert_ne!(eval(&got,pow(k,m as u64,r),r),0);wrong+=1;}
   }
  }}}
 }
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();
 for cap in [8usize,16,64]{for k in 0..r{let target=multiple(curve,bp,k);for slots in [0usize,16,1024]{let old=modes.recover_powered_image(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128,slots).unwrap();SEEDED_IMAGE_COUNTS.with(|c|c.set((0,0,0,0)));let new=modes.recover_seeded_image(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128,slots).unwrap();assert_eq!(old.0,Some(k));assert_eq!(new.0,Some(k));assert!(new.1.calls<=128);route_changes+=usize::from(old.2!=new.2);call_changes+=usize::from(old.1.calls!=new.1.calls);updates+=SEEDED_IMAGE_COUNTS.with(|c|c.get().1);public+=1;
  if k==0{assert_eq!(new.1.calls,1);assert_eq!(new.2,"identity");}
 }}}
 // Exhausting the budget immediately must not issue a second sign query.
 for k in 1..r{let q=modes.recover_seeded_image(&source,curve,bp,multiple(curve,bp,k),&[1,2],64,&extra,1,1024).unwrap();assert_eq!(q.0,None);assert_eq!(q.1.calls,1);assert_eq!(q.2,"call-cap");}
 assert!(retained>0&&wrong>0&&zero>0&&updates>0);
 println!("support_equalities={states} true_images_retained={retained} wrong_sign_exclusions={wrong} zero_exclusions={zero} public_target_capacity_cases={public} route_changes={route_changes} call_changes={call_changes} subsequent_updates={updates} single_call_cap_cases=280 invalid_signs=rejected phase_choices=2_of_4");
}
