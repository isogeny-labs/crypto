fn bcadd(dst:&mut[u64;3],v:[u64;3]){for i in 0..3{dst[i]+=v[i]}}
fn big_polyjet_mul(e:&E,h:&[K],a:&[Vec<K>;2],b:&[Vec<K>;2])->[Vec<K>;2]{
 [br(e,bm(e,&a[0],&b[0]),h),br(e,ba(e,&bm(e,&a[0],&b[1]),&bm(e,&a[1],&b[0]),&e.c(1)),h)]
}
fn big_cached_numerator(e:&E,c:&TargetCache,ap:&Pt,bp_:&Pt)->[Vec<K>;2]{
 let(x,y)=ap.as_ref().unwrap();let(z,w)=bp_.as_ref().unwrap();let xp=[e.c(1),x.clone(),e.mulk(x,x)];let zp=[e.c(1),z.clone(),e.mulk(z,z)];
 let mut m0=vec![e.c(0)];let mut m1=vec![e.c(0)];for i in 0..3{for j in 0..3{let q=e.mulk(&xp[i],&zp[j]);m0=ba(e,&m0,&c.tc[3*i+j],&q);m1=ba(e,&m1,&c.jc[3*i+j],&q)}}m0=ba(e,&m0,&c.gamma,&e.mulk(&e.c(8),&e.mulk(y,w)));[m0,m1]
}
fn big_batch_query(e:&E,h:&[K],c:&TargetCache,gb:&[Pt],gg:&[Pt],p:&Pt,u:&Pt)->([K;2],[u64;3],[u64;3],Value){
 let start=big::counts();let rr=e.add(p,u);let mut outside=[e.c(1),e.c(0)];let mut reference=[e.c(1),e.c(0)];let mut surface_giants=vec![];let mut den=e.c(1);let mut maxslots=0;
 let mut validation=[0;3];let mut formation=[0;3];let mut merges=[0;3];let mut denominators=[0;3];let mut reference_blocks=vec![];
 for i in 0..3{
  let ap=e.add(p,&gg[i]);let bp_=e.add(&rr,&c.qg[i]);
  let st=big::counts();let mut explicit=[e.c(1),e.c(0)];
  for j in 1..=4{let(ha,ka)=hk(e,&ap,&gb[j]);let(hb,kb)=hk(e,&bp_,&c.qb[j]);let l=e.subk(&ha,&hb);let k=e.subk(&ka,&kb);let v0=e.subk(&e.mulk(&l,&l),&e.mulk(&k,&k));let wrong=e.addk(&ka,&kb);assert_ne!(v0,e.subk(&e.mulk(&l,&l),&e.mulk(&wrong,&wrong)));explicit=upd(e,&explicit,&[v0,e.mulk(&e.c(2),&l)]);}
  let d0=e.subk(&ap.as_ref().unwrap().0,&bp_.as_ref().unwrap().0);let direct_block=upd(e,&[d0.clone(),e.c(1)],&explicit);reference=upd(e,&reference,&direct_block);reference_blocks.push(direct_block.iter().map(field).collect::<Vec<_>>());bcadd(&mut validation,big::diff(st,big::counts()));
  outside=upd(e,&outside,&[d0,e.c(1)]);
  let(x,y)=ap.as_ref().unwrap();let(z,w)=bp_.as_ref().unwrap();surface_giants.push((x.clone(),y.clone(),z.clone(),w.clone()));
  let st=big::counts();let d=e.mulk(&be(e,h,&ap.as_ref().unwrap().0),&be(e,&c.hq,&bp_.as_ref().unwrap().0));let d=e.mulk(&d,&d);assert_ne!(d,e.c(0));den=if i==0{d}else{e.mulk(&den,&d)};bcadd(&mut denominators,big::diff(st,big::counts()));
 }
 assert!(maxslots<=8);let st=big::counts();let(nv,surface_record)=surface_contract_big(e,c,gb,&surface_giants);let branch="surface_evaluation";let norm_counts=big::diff(st,big::counts());let st=big::counts();assert_ne!(den,e.c(0));let baby=[e.quotient(&nv[0],&den),e.quotient(&nv[1],&den)];let division=big::diff(st,big::counts());let v=upd(e,&outside,&baby);assert_eq!(v,reference);
 let total=big::diff(start,big::counts());let selected=std::array::from_fn(|i|total[i]-validation[i]);
 (v,selected,validation,json!({"formation_counts":formation,"quotient_jet_merge_counts":merges,"denominator_counts":denominators,"surface_contraction_counts":norm_counts,"division_counts":division,"surface":surface_record,"final_norm_calls":0,"final_norm_branch":branch,"batch_constant_nonzero":nv[0]!=e.c(0),"complete_blocks":3,"unused_legacy_accumulator_slots":maxslots,"legacy_bound_not_surface_memory":8,"reference_block_coefficients":reference_blocks}))
}
