fn split_checks(){split_projection_checks();let mut states=0usize;
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let(mut old,mut new):(Option<CanonicalImage>,Option<SplitImage>)=(None,None);
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(old)=&mut old{old.update(&session,b,&f).unwrap();new.as_mut().unwrap().update(&session,b,&f).unwrap();}else{old=Some(CanonicalImage::new(&session,b,&f,i).unwrap());new=Some(SplitImage::new(&session,b,&f,i).unwrap());}
    let old=old.as_mut().unwrap();let new=new.as_mut().unwrap();assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;
    for (a,b) in old.branches.iter().zip(new.branches.iter()){assert_eq!(a.u,b.u);assert_eq!(canonical_signature(&a.canonical),canonical_signature(&b.canonical));assert_eq!(a.ring.g,b.ring.g);}
   }
  }
 }
 println!("split_canonical_states={states}");
 for cap in [64usize,16]{SPLIT_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_canonical(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_split(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(inits,updates,deferrals,materializations)=SPLIT_COUNTS.with(|c|c.get());assert_eq!(unresolved,0);println!("split_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}
fn split_projection_checks(){
 let(mut projections,mut mixed,mut selected,mut pairs,mut reads,mut scalar_products,mut pair_products)=(0,0,0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3)]{
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let mut p=polynomial::Poly::new(r).unwrap();let mut ws:Vec<_>=(1..r).map(|v|(v+pow(v,r-2,r))%r).collect();ws.sort();ws.dedup();let g=p.product_tree(&ws).unwrap().polynomial().to_vec();let mut state=CanonicalFiber::new(r,&g).unwrap();
  for step in 0..=ws.len(){
   if step>0{let w=ws[step-1];let v=(1..r).find(|&v|(v+pow(v,r-2,r))%r==w).unwrap();let other:Vec<_>=ws.iter().copied().filter(|&x|x!=w).collect();let mask=p.product_tree(&other).unwrap().polynomial().to_vec();let a=mask.iter().map(|&x|mul(r-v,x,r)).collect();state.keep(&mut p,&Quad([a,mask])).unwrap();}
   let total=state.support(&mut p);for scale in [1,2,r-1]{for e in 0..3{
    let mut full=QuadRing::new(r,&total).unwrap();let q=full.mode_scaled(&session,e,scale).unwrap();let mut split=SplitRing::new(&state).unwrap();let v=split.mode(&session,e,scale).unwrap();let length=session.stream(e).unwrap().left;assert_eq!(split.reads,length);
    if let Some(b)=&v.both{for j in 0..2{assert_eq!(canonical_rem(&mut p,&q.0[j],&state.both).unwrap(),b.0[j]);}assert_eq!(split.pair_products,2*(length-1));}
    if let Some(a)=&v.one{let br=p.mul(&q.0[1],&state.root);let value=p.add(&q.0[0],&br);assert_eq!(canonical_rem(&mut p,&value,&state.one).unwrap(),canonical_rem(&mut p,a,&state.one).unwrap());assert_eq!(split.scalar_products,2*(length-1));}
    mixed+=usize::from(v.both.is_some()&&v.one.is_some());selected+=usize::from(v.both.is_none()&&v.one.is_some());pairs+=usize::from(v.both.is_some()&&v.one.is_none());reads+=split.reads;scalar_products+=split.scalar_products;pair_products+=split.pair_products;projections+=1;
   }}
  }
 }
 assert!(mixed>0&&selected>0);println!("split_projection_checks={projections} mixed_modes={mixed} selected_only_modes={selected} unresolved_only_modes={pairs} coefficient_reads={reads} scalar_products={scalar_products} pair_products={pair_products}");
}
