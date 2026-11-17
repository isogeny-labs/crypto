fn canonical_signature(s:&CanonicalFiber)->(Vec<u64>,Vec<u64>,Vec<u64>){(s.both.clone(),s.one.clone(),s.root.clone())}
fn canonical_checks(){
 let(mut memberships,mut orders,mut duplicate_controls,mut zero_history,mut inverses,mut merges)=(0,0,0,0,0,0);
 for r in [17u64,31,97]{let mut p=polynomial::Poly::new(r).unwrap();let mut ws:Vec<_>=(1..r).map(|v|(v+pow(v,r-2,r))%r).collect();ws.sort();ws.dedup();let g=p.product_tree(&ws).unwrap().polynomial().to_vec();let mut state=CanonicalFiber::new(r,&g).unwrap();let mut equations=Vec::new();let mut expected=vec![true;r as usize];expected[0]=false;
  for &w in ws.iter().filter(|&&w|w!=2&&w!=r-2).take(6){let v=(1..r).find(|&v|(v+pow(v,r-2,r))%r==w).unwrap();let other:Vec<_>=ws.iter().copied().filter(|&x|x!=w).collect();let mask=p.product_tree(&other).unwrap().polynomial().to_vec();let a:Vec<_>=mask.iter().map(|&x|mul(r-v,x,r)).collect();let pair=Quad([a,mask]);state.keep(&mut p,&pair).unwrap();equations.push(pair.clone());
   for x in 1..r{let w=(x+pow(x,r-2,r))%r;expected[x as usize]&=(eval(&pair.0[0],w,r)+mul(eval(&pair.0[1],w,r),x,r))%r==0;let got=eval(&state.both,w,r)==0||(eval(&state.one,w,r)==0&&eval(&state.root,w,r)==x);assert_eq!(got,expected[x as usize]);memberships+=1;}
   assert_eq!(p.gcd_monic(&state.both,&state.one,g.len()).unwrap().polynomial,vec![1]);assert_eq!(p.gcd_monic(&state.both,&[r-4,0,1],g.len()).unwrap().polynomial,vec![1]);let rr=p.mul(&state.root,&state.root);let wr=p.mul(&[0,1],&state.root);let relation=p.add(&p.sub(&rr,&wr),&[1]);assert_eq!(canonical_rem(&mut p,&relation,&state.one).unwrap(),vec![0]);
   let before=canonical_signature(&state);state.keep(&mut p,&pair).unwrap();assert_eq!(before,canonical_signature(&state));duplicate_controls+=1;
  }
  let mut reverse=CanonicalFiber::new(r,&g).unwrap();for pair in equations.iter().rev(){reverse.keep(&mut p,pair).unwrap();}assert_eq!(canonical_signature(&state),canonical_signature(&reverse));orders+=1;
  let before=canonical_signature(&state);for _ in 0..32{state.keep(&mut p,&Quad([vec![0],vec![0]])).unwrap();assert_eq!(before,canonical_signature(&state));zero_history+=1;}
  inverses+=state.inverses;merges+=state.crt_merges;assert!(CanonicalFiber::new(r,&[1,r-2,1]).is_err());assert!(canonical_inverse(&mut p,&g,&g,r).is_err());
 }
 assert!(inverses>0&&merges>0);
 println!("canonical_membership_checks={memberships} reversed_history_equalities={orders} duplicate_constraint_controls={duplicate_controls} zero_constraints_without_history_growth={zero_history} modular_inversions={inverses} crt_merges={merges} nonsquarefree_and_nonunit_rejections=6");
 let(mut states,mut old_slots,mut new_slots,mut max_pairs)=(0usize,0usize,0usize,0usize);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let(mut old,mut new):(Option<PersistentImage>,Option<CanonicalImage>)=(None,None);
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(old)=&mut old{old.update(&session,b,&f).unwrap();new.as_mut().unwrap().update(&session,b,&f).unwrap();}else{old=Some(PersistentImage::new(&session,b,&f,i).unwrap());new=Some(CanonicalImage::new(&session,b,&f,i).unwrap());}
    let old=old.as_mut().unwrap();let new=new.as_mut().unwrap();assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;
    for q in &old.branches{old_slots+=q.ring.g.len();max_pairs=max_pairs.max(q.constraints.len());for pair in &q.constraints{old_slots+=pair.0[0].len()+pair.0[1].len();}}
    for q in &new.branches{new_slots+=q.ring.g.len()+q.canonical.both.len()+q.canonical.one.len()+q.canonical.root.len();assert_eq!(q.canonical.support(&mut polynomial::Poly::new(r).unwrap()),q.ring.g);}
   }
  }
 }
 println!("canonical_history_states={states} reference_live_coefficient_slot_sum={old_slots} canonical_live_coefficient_slot_sum={new_slots} reference_max_pairs={max_pairs} canonical_history_vectors=0");
 for cap in [64usize,16]{CANONICAL_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_persistent(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_canonical(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(inits,updates,deferrals,materializations)=CANONICAL_COUNTS.with(|c|c.get());assert_eq!(unresolved,0);println!("canonical_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}
