fn shared_checks(){shared_mode_checks();let mut states=0usize;
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let(mut old,mut new):(Option<ReusedImage>,Option<SharedImage>)=(None,None);
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(old)=&mut old{old.update(&session,b,&f).unwrap();new.as_mut().unwrap().update(&session,b,&f).unwrap();}else{old=Some(ReusedImage::new(&session,b,&f,i).unwrap());new=Some(SharedImage::new(&session,b,&f,i).unwrap());}
    let old=old.as_mut().unwrap();let new=new.as_mut().unwrap();assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;
    assert_eq!(old.branches.len(),new.branches.len());for (a,b) in old.branches.iter().zip(new.branches.iter()){assert_eq!(a.u,b.u);assert_eq!(canonical_signature(&a.canonical),canonical_signature(&b.canonical));assert_eq!(a.ring.g,b.ring.g);}
   }
  }
 }
 println!("shared_canonical_states={states}");
 for cap in [64usize,16]{SHARED_STREAMS.with(|c|c.set((0,0,0,0,0,0)));SHARED_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_reused(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_shared(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(streams,reads,separate_streams,separate_reads,max_branches,max_cache_slots)=SHARED_STREAMS.with(|c|c.get());println!("shared_public_streams cap={cap} streams={streams} reader_coefficients={reads} equivalent_separate_streams={separate_streams} equivalent_separate_reader_coefficients={separate_reads} max_branches={max_branches} max_cache_coefficient_slots={max_cache_slots}");
  let(inits,updates,deferrals,materializations)=SHARED_COUNTS.with(|c|c.get());assert_eq!(unresolved,0);println!("shared_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}
fn shared_mode_checks(){
 SHARED_STREAMS.with(|c|c.set((0,0,0,0,0,0)));let(mut values,mut groups,mut coefficient_reads,mut separate_reads,mut rejects,mut max_result_slots)=(0,0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(257,4),(1009,4)]{
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let mut p=polynomial::Poly::new(r).unwrap();let mut ws:Vec<_>=(1..r).map(|v|(v+pow(v,r-2,r))%r).filter(|&w|w!=2&&w!=r-2).collect();ws.sort();ws.dedup();ws.truncate(6);let mut states=Vec::new();
  for shape in 0..4{let mut roots=ws.clone();if shape==1||shape==2{roots.push(2);roots.push(r-2);}if shape==3{roots.clear();}let g=if roots.is_empty(){vec![1]}else{p.product_tree(&roots).unwrap().polynomial().to_vec()};let mut state=CanonicalFiber::new(r,&g).unwrap();if shape==2{for &w in &ws{let v=(1..r).find(|&v|(v+pow(v,r-2,r))%r==w).unwrap();let other:Vec<_>=roots.iter().copied().filter(|&x|x!=w).collect();let mask=p.product_tree(&other).unwrap().polynomial().to_vec();let a=mask.iter().map(|&x|mul(r-v,x,r)).collect();state.keep(&mut p,&Quad([a,mask])).unwrap();}}states.push(state);}
  for width in 0..5{let mut shared:Vec<_>=states[..width].iter().map(|s|ReusedRing::new(s).unwrap()).collect();let mut reference:Vec<_>=states[..width].iter().map(|s|ReusedRing::new(s).unwrap()).collect();for scale in [1,2,1]{for e in 0..3{let before=SHARED_STREAMS.with(|c|c.get());let got=shared_mode(&mut shared,&session,e,scale).unwrap();assert_eq!(got.len(),width);let length=session.stream(e).unwrap().left;let after=SHARED_STREAMS.with(|c|c.get());assert_eq!(after.0-before.0,usize::from(width>0));assert_eq!(after.1-before.1,if width>0{length}else{0});coefficient_reads+=after.1-before.1;separate_reads+=width*length;
   for((v,ref_ring),shared_ring)in got.iter().zip(reference.iter_mut()).zip(shared.iter()){same_split(v,&ref_ring.mode(&session,e,scale).unwrap());assert_eq!(ref_ring.inner.reads,shared_ring.inner.reads);assert_eq!(ref_ring.inner.pair_products,shared_ring.inner.pair_products);assert_eq!(ref_ring.inner.scalar_products,shared_ring.inner.scalar_products);values+=1;}
   let slots:usize=got.iter().map(|v|v.both.as_ref().map_or(0,|q|q.0[0].len()+q.0[1].len())+v.one.as_ref().map_or(0,Vec::len)).sum();max_result_slots=max_result_slots.max(slots);groups+=1;
  }}}
  let mut five:Vec<_>=(0..5).map(|_|ReusedRing::new(&states[0]).unwrap()).collect();assert!(shared_mode(&mut five,&session,0,1).is_err());rejects+=1;
  let mut one=vec![ReusedRing::new(&states[0]).unwrap()];assert!(shared_mode(&mut one,&session,0,r).is_err());rejects+=1;
  let other=if r==17{41}else{17};let wrong=CanonicalFiber::new(other,&[1]).unwrap();let mut one=vec![ReusedRing::new(&wrong).unwrap()];assert!(shared_mode(&mut one,&session,0,1).is_err());rejects+=1;
 }
 let(streams,reads,old_streams,old_reads,max_branches,max_cache_slots)=SHARED_STREAMS.with(|c|c.get());assert_eq!(reads,coefficient_reads);assert_eq!(old_reads,separate_reads);assert!(old_reads>reads);println!("shared_mode_groups={groups} branch_value_equalities={values} streams={streams} reader_coefficients={reads} separate_streams={old_streams} separate_reader_coefficients={old_reads} max_branches={max_branches} max_cache_coefficient_slots={max_cache_slots} max_returned_coefficient_slots={max_result_slots} invalid_input_rejections={rejects}");
}
