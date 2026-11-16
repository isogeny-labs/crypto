fn reused_checks(){reused_power_checks();let mut states=0usize;
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let(mut old,mut new):(Option<SplitImage>,Option<ReusedImage>)=(None,None);
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(old)=&mut old{old.update(&session,b,&f).unwrap();new.as_mut().unwrap().update(&session,b,&f).unwrap();}else{old=Some(SplitImage::new(&session,b,&f,i).unwrap());new=Some(ReusedImage::new(&session,b,&f,i).unwrap());}
    let old=old.as_mut().unwrap();let new=new.as_mut().unwrap();assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;
    for (a,b) in old.branches.iter().zip(new.branches.iter()){assert_eq!(a.u,b.u);assert_eq!(canonical_signature(&a.canonical),canonical_signature(&b.canonical));assert_eq!(a.ring.g,b.ring.g);}
   }
  }
 }
 println!("reused_canonical_states={states}");
 for cap in [64usize,16]{REUSED_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_split(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_reused(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(inits,updates,deferrals,materializations)=REUSED_COUNTS.with(|c|c.get());assert_eq!(unresolved,0);println!("reused_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}
fn same_split(a:&SplitValue,b:&SplitValue){assert_eq!(a.one,b.one);assert_eq!(a.both.is_some(),b.both.is_some());if let(Some(a),Some(b))=(&a.both,&b.both){assert_eq!(a.0,b.0);}}
fn reused_power_checks(){
 let(mut modes_checked,mut powers_checked,mut old_products,mut new_products,mut cold_saved,mut warm_saved,mut max_entries,mut max_slots,mut resets)=(0,0,0,0,0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(257,4),(1009,4)]{
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let mut p=polynomial::Poly::new(r).unwrap();
  let mut ws:Vec<_>=(1..r).map(|v|(v+pow(v,r-2,r))%r).filter(|&w|w!=2&&w!=r-2).collect();ws.sort();ws.dedup();ws.truncate(6);
  for shape in 0..4{let mut roots=ws.clone();if shape==1||shape==2{roots.push(2);roots.push(r-2);}if shape==3{roots.clear();}let g=p.product_tree(&roots).unwrap().polynomial().to_vec();let mut state=CanonicalFiber::new(r,&g).unwrap();
   if shape==2{for &w in &ws{let v=(1..r).find(|&v|(v+pow(v,r-2,r))%r==w).unwrap();let other:Vec<_>=roots.iter().copied().filter(|&x|x!=w).collect();let mask=p.product_tree(&other).unwrap().polynomial().to_vec();let a=mask.iter().map(|&x|mul(r-v,x,r)).collect();state.keep(&mut p,&Quad([a,mask])).unwrap();}}
   assert_eq!(state.both.len()>1,shape<2);assert_eq!(state.one.len()>1,shape==1||shape==2);
   let mut cached=ReusedRing::new(&state).unwrap();assert!(cached.cached_power(3).is_err());
   for scale in [1,2,r-1,1]{for e in 0..3{let mut reference=SplitRing::new(&state).unwrap();let expected=reference.mode(&session,e,scale).unwrap();let before=cached.power_products+cached.value_products;let reads=cached.inner.reads;let old_power=cached.power_products;let cold=cached.scale_key!=Some(scale);let got=cached.mode(&session,e,scale).unwrap();same_split(&got,&expected);let length=session.stream(e).unwrap().left;assert_eq!(cached.inner.reads-reads,length);let used=cached.power_products+cached.value_products-before;let old=2*(length-1);assert!(used<=old);if cold{cold_saved+=old-used}else{warm_saved+=old-used;}if e==2{assert_eq!(cached.power_products,old_power);}old_products+=old;new_products+=used;modes_checked+=1;max_entries=max_entries.max(cached.powers.len());max_slots=max_slots.max(cached.cache_slots());}}
   resets+=cached.resets;assert_eq!(cached.resets,4);
   let mut reference=SplitRing::new(&state).unwrap();let base=reference.base(2);let mut v=reference.constant(1);cached.prepare(2).unwrap();for n in 0..66{if n>0{v=reference.multiply(&v,&base).unwrap();}let got=cached.cached_power(n).unwrap();same_split(&got,&v);powers_checked+=1;}
  }
 }
 assert!(cold_saved>0&&warm_saved>0);println!("reused_mode_equalities={modes_checked} independent_power_equalities={powers_checked} reference_ring_products_per_present_component={old_products} reused_ring_products_per_present_component={new_products} cold_products_removed={cold_saved} warm_products_removed={warm_saved} mode_cache_max_entries={max_entries} mode_cache_max_coefficient_slots={max_slots} scale_resets={resets} support_shapes=4");
}
