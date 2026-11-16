fn opposite_checks(){
 let(mut supports,mut dzero,mut dnonzero,mut empty,mut states)=(0,0,0,0,0);
 for(r,m)in[(41u64,5usize),(73,3),(97,3),(193,3),(241,3),(241,5),(281,5)]{
 let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
 let stream=StreamModes::new(r,m,z,16,1024,7).unwrap();let session=RecoveryModes::from_modes(&stream).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let n=((r-1)/(4*m as u64))as usize;let fold=PhaseFold::build(&session,i,n).unwrap();let mut p=polynomial::Poly::new(r).unwrap();
 // Every possible observation pair for the two smallest fields; deterministic
 // diagonal and true-value panels for the larger fields. Enumeration is checker-only.
 let mut pairs=Vec::new();if r<=73{for a in 0..r{for b in 0..r{pairs.push((a,b));}}}else{for x in 0..r{pairs.push((x,x));pairs.push((eval(&dense.coeff[0],x,r),eval(&dense.coeff[0],(r-x)%r,r)));}}
 for (a,b) in pairs {for phase in 0..4{let first=fold.branch(phase,a).unwrap();let mut minus=dense.coeff[0].clone();for j in (1..minus.len()).step_by(2){minus[j]=(r-minus[j])%r;}minus[0]=(minus[0]+r-b)%r;let expected=p.gcd_monic(&first,&minus,4*n+4).unwrap().polynomial;let got=fold.opposite_branch(phase,a,b).unwrap();assert_eq!(got,expected,"r={r},m={m},phase={phase},a={a},b={b}");assert_eq!(squarefree(r,&got).unwrap(),got);if a==b{dzero+=1}else{dnonzero+=1;assert!(got.len()-1<=n/2)}if got.len()==1{empty+=1}supports+=1;}}
 for x in 0..r{let session=RecoveryModes::from_modes(&stream).unwrap();let obs=|b|{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect::<Vec<_>>()};
 let f=obs(1);let fm=obs(r-1);let mut old=DirectImage::new(&session,1,&f,i).unwrap();old.update(&session,r-1,&fm).unwrap();let mut new=DirectImage::opposite(&session,1,&f,&fm,i).unwrap();
 for b in [0,2,3]{if b!=0{let f=obs(b);old.update(&session,b,&f).unwrap();new.update(&session,b,&f).unwrap();}assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;}
 }
 }
 assert!(dzero>0&&dnonzero>0&&empty>0);println!("opposite_support_equalities={supports} equal_observation_controls={dzero} unequal_observation_controls={dnonzero} empty_support_controls={empty} candidate_states={states}");
 for cap in [64usize,16]{OPPOSITE_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_direct(&source,curve,bp,target,&[1,r-1,2,3],cap,&extra,128).unwrap();let new=modes.recover_opposite(&source,curve,bp,target,&[1,r-1,2,3],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(inits,updates,deferrals,materializations)=OPPOSITE_COUNTS.with(|c|c.get());println!("opposite_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}