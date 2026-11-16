fn persistent_checks(){
 let(mut orientation_traps,mut history_caps)=(0,0);
 for r in [17u64,31]{for w in 0..r{let roots:Vec<_>=(1..r).filter(|&v|(mul(v,v,r)+1)%r==mul(w,v,r)).collect();if roots.len()!=2{continue}
  let mut branch=OrientedBranch{u:1,ring:QuadRing::new(r,&[(r-w)%r,1]).unwrap(),constraints:Vec::new()};branch.keep(Quad([vec![0],vec![0]])).unwrap();branch.keep(Quad([vec![r-roots[0]],vec![1]])).unwrap();assert_eq!(branch.ring.g.len(),2);branch.keep(Quad([vec![r-roots[1]],vec![1]])).unwrap();assert_eq!(branch.ring.g,vec![1]);orientation_traps+=1;
  let mut branch=OrientedBranch{u:1,ring:QuadRing::new(r,&[(r-w)%r,1]).unwrap(),constraints:Vec::new()};for _ in 0..8{branch.keep(Quad([vec![0],vec![0]])).unwrap();}assert!(matches!(branch.keep(Quad([vec![0],vec![0]])),Err(Error::Limit)));history_caps+=1;
 }}
 let(mut states,mut deferred,mut later_fit,mut max_history,mut fifth_rejects)=(0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let mut image:Option<PersistentImage>=None;let mut expected=ImageDomain::new(r,m).unwrap();let mut p=polynomial::Poly::new(r).unwrap();let(mut blocked,mut fitted)=(false,false);let mut last=vec![];
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(state)=&mut image{state.update(&session,b,&f).unwrap();}else{image=Some(PersistentImage::new(&session,b,&f,i).unwrap());}
    modes.intersect_modes(&mut p,&mut expected,b,&f).unwrap();let state=image.as_mut().unwrap();let got=state.materialize(r as usize,r as usize).unwrap();assert_eq!(Some(&got),expected.dense.as_ref());assert_eq!(eval(&got,pow(x,m as u64,r),r),0);states+=1;
    for branch in &state.branches{max_history=max_history.max(branch.constraints.len());for pair in &branch.constraints{for c in &pair.0{assert!(c.len()<branch.ring.g.len());}}}
    match state.materialize(7,7){Ok(small)=>{assert_eq!(small,got);if blocked&&!fitted{later_fit+=1;fitted=true;}},Err(Error::Limit)=>{deferred+=1;blocked=true;},Err(e)=>panic!("{e:?}")};assert_eq!(state.materialize(r as usize,r as usize).unwrap(),got);last=f;
   }
   let state=image.as_mut().unwrap();assert!(matches!(state.update(&session,1,&last),Err(Error::Limit)));assert_eq!(state.rounds,4);fifth_rejects+=1;
  }
 }
 assert!(orientation_traps>0&&later_fit>0&&max_history==8);
 println!("persistent_exact_states={states} small_cap_deferrals={deferred} later_same_cap_fit={later_fit} max_retained_constraints={max_history} fifth_radius_rejections={fifth_rejects} phase_degenerate_orientation_traps={orientation_traps} history_cap_rejections={history_caps}");
 for cap in [64usize,16]{FIBER_COUNTS.with(|c|c.set((0,0,0,0)));
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut changed_routes,mut unresolved,mut old_unresolved)=(0,0,0,0,0);
 for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_isolated(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_persistent(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if old.0.is_none(){old_unresolved+=1;}if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}if old.2!=new.2{changed_routes+=1;}calls+=new.1.calls;old_calls+=old.1.calls;}
 let(initializations,updates,deferrals,materializations)=FIBER_COUNTS.with(|c|c.get());assert!(initializations>0);if cap==64{assert_eq!(unresolved,0);}
 println!("persistent_public_cases={r} source_calls={calls} reference_source_calls={old_calls} changed_routes={changed_routes} unresolved={unresolved} initializations={initializations} subsequent_updates={updates} lift_deferrals={deferrals} materializations={materializations} reference_unresolved={old_unresolved} caller_pullback_cap={cap} image_cap={}",cap/m);
}
}
