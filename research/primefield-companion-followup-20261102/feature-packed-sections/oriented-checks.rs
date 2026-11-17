// Enumeration is confined to this deterministic correctness oracle.
fn oriented_checks(){
 let mut factors=0;
 for r in [7u64,13,17,19,31,37,41,97,193,281]{
  let mut p=polynomial::Poly::new(r).unwrap();
  for l in 1..=(r as usize-1).min(32){for epsilon in [1,r-1]{for slots in [0,1024]{for trials in [0,64]{
   let domains=character_domains(r,l,epsilon,3,slots,trials).unwrap();let mut product=vec![1];
   for g in &domains{assert_eq!(squarefree(r,g).unwrap(),*g);assert_eq!(p.gcd_monic(&product,g,2*r as usize).unwrap().polynomial,vec![1]);product=p.mul(&product,g);}
   let mut equation=initial_dickson(&mut p,l);equation[0]=(equation[0]+r-mul(2,epsilon,r))%r;
   assert_eq!(product,squarefree(r,&equation).unwrap(),"domain r={r} l={l} epsilon={epsilon}");factors+=1;
  }}}}
 }
 let(mut states,mut old_equal,mut retained,mut wrong,mut zero,mut strict,mut late,mut anchor_strict,mut delayed)=(0,0,0,0,0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5),(281,35),(19,3),(31,3),(37,6),(13,6)]{
  let h=(r-1)/2;let l=h/m as u64;
  let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();
  let stream=StreamModes::new(r,m,zeta,64,2048,7).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();
  let radii=[2,1,3,2];
  let obs:Vec<Vec<[u64;3]>>=(0..r).map(|k|radii.iter().map(|&b|{
   let mut f=[0;3];let mut z=1;for _ in 0..m{let s=pow((k+r-mul(b,z,r))%r,h,r);f[0]=(f[0]+s)%r;f[1]=(f[1]+mul(z,s,r))%r;f[2]=(f[2]+mul(pow(z,r-2,r),s,r))%r;z=mul(z,zeta,r);}f
  }).collect()).collect();
  let chars:Vec<_>=(0..r).map(|j|pow(j,h,r)).collect();let images:Vec<_>=(0..r).map(|j|pow(j,m as u64,r)).collect();
  let ks:Vec<_>=if m==35{vec![0,1,2,3,7,17,r-1]}else{(0..r).collect()};
  for k in ks{for slots in [0usize,16,96,1024]{for sign in [1,r-1]{
   let mut new:Option<OrientedImage>=None;let mut old:Option<BalancedCharacterImage>=None;
   for(t,&b)in radii.iter().enumerate(){let f=obs[k as usize][t];
    if let Some(image)=&mut new{image.update(&modes,b,&f).unwrap();}else{new=Some(OrientedImage::new(&modes,b,&f,slots,sign,64).unwrap());}
    let image=new.as_mut().unwrap();assert_eq!(image.orientation.n,l as usize);assert!(image.branches.iter().all(|q|q.epsilon==mul(sign,pow(2,h,r),r)));
    let got=image.materialize(r as usize,r as usize).unwrap();
    let mut roots=std::collections::BTreeSet::new();
    let anchor=(0..=t).find(|&v|obs[k as usize][v][1]!=0&&obs[k as usize][v][2]!=0);
    let mut zero_roots=std::collections::BTreeSet::new();
    for j in 1..r as usize{if chars[j]==sign&&(0..=t).all(|v|obs[j][v][0]==obs[k as usize][v][0]&&mul(obs[j][v][1],obs[j][v][2],r)==mul(obs[k as usize][v][1],obs[k as usize][v][2],r)&&(obs[k as usize][v][1]!=0||obs[j][v][1]==0)&&(obs[k as usize][v][2]!=0||obs[j][v][2]==0)){zero_roots.insert(images[j]);if anchor.map_or(true,|a|(a..=t).all(|v|mul(obs[k as usize][a][1],obs[j][v][1],r)==mul(obs[k as usize][v][1],obs[j][a][1],r))){roots.insert(images[j]);}}}
    anchor_strict+=usize::from(roots.len()<zero_roots.len());delayed+=usize::from(anchor.map_or(false,|a|a>0));
    let mut p=polynomial::Poly::new(r).unwrap();let mut expected=vec![1];for root in roots{expected=p.mul(&expected,&[r-root,1]);}
    assert_eq!(got,expected,"state r={r} m={m} k={k} t={t} slots={slots} sign={sign}");states+=1;
    if let Some(prior)=&mut old{prior.update(&modes,b,&f).unwrap();}else{old=Some(BalancedCharacterImage::new(&modes,b,&f,slots,sign,64).unwrap());}
    let prior=old.as_mut().unwrap().materialize(r as usize,r as usize).unwrap();assert_eq!(p.divrem(&prior,&got).unwrap().1,vec![0]);old_equal+=1;if got.len()<prior.len(){strict+=1;if t>0{late+=1;}}

    if k==0{assert_ne!(eval(&got,0,r),0);zero+=1;}else if chars[k as usize]==sign{assert_eq!(eval(&got,images[k as usize],r),0);retained+=1;}else{assert_ne!(eval(&got,images[k as usize],r),0);wrong+=1;}
   }
  }}}
 }
 let(mut public,mut odd_public,mut compatible,mut updates,mut guards,mut routes,mut calls)=(0,0,0,0,0,0,0);
 for(p,a,b,r,base,m)in[(257u64,1u64,7u64,281u64,[1u64,3u64],5usize),(17,2,2,19,[5,1],3)]{
  let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};assert_eq!(multiple(curve,bp,r),None);
  let source=source::Source::new(p,a,b,r,base,64).unwrap();let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,zeta,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();
  for cap in [8usize,16,64]{for k in 0..r{for slots in [0usize,16,96,1024]{let target=multiple(curve,bp,k);ORIENTED_COUNTS.with(|c|c.set((0,0,0,0)));let got=modes.recover_oriented_image(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128,slots,64).unwrap();assert_eq!(got.0,Some(k),"public r={r} k={k} cap={cap} slots={slots}");assert!(got.1.calls<=128);updates+=ORIENTED_COUNTS.with(|c|c.get().1);public+=1;
   if r==281{let old=modes.recover_balanced_character_image(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128,slots,64).unwrap();assert_eq!(old.0,Some(k));routes+=usize::from(got.2!=old.2);calls+=usize::from(got.1.calls!=old.1.calls);compatible+=1;}else{odd_public+=1;}
  }}}
  for k in 1..r{let got=modes.recover_oriented_image(&source,curve,bp,multiple(curve,bp,k),&[1,2],64,&extra,1,1024,64).unwrap();assert_eq!(got.0,None);assert_eq!(got.1.calls,1);assert_eq!(got.2,"call-cap");guards+=1;}
  assert!(matches!(modes.recover_oriented_image(&source,curve,bp,None,&[1],64,&extra,128,1024,65),Err(Error::Domain)));
 }
 assert!(strict>0&&late>0&&anchor_strict>0);
 println!("anchor_only_reductions={anchor_strict} delayed_anchor_states={delayed} strict_support_reductions={strict} subsequent_radius_reductions={late} public_route_changes={routes} public_call_changes={calls}");
 println!("character_factorizations={factors} direct_sign_support_equalities={states} predecessor_support_inclusions={old_equal} true_images_retained={retained} wrong_sign_exclusions={wrong} zero_exclusions={zero} public_cases={public} odd_L_public_cases={odd_public} predecessor_public_comparisons={compatible} subsequent_updates={updates} single_call_cap_cases={guards}");
}
