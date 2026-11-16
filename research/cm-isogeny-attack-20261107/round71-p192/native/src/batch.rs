fn addcounts(dst:&mut[u64;3],v:[u64;3]){for i in 0..3{dst[i]+=v[i]}}
fn polyjet_mul<const P:i64,const D:i64>(h:&[F<P,D>],a:&[Vec<F<P,D>>;2],b:&[Vec<F<P,D>>;2])->[Vec<F<P,D>>;2]{
 [red(pm(&a[0],&b[0]),h),red(sum(&pm(&a[0],&b[1]),&pm(&a[1],&b[0]),F::c(1)),h)]
}
fn cached_numerator<const P:i64,const D:i64>(al:&Alg<P,D>,ap:Pt<P,D>,bp:Pt<P,D>)->[Vec<F<P,D>>;2]{
 let(x,y)=ap.unwrap();let(z,w)=bp.unwrap();let xp=[F::c(1),x,x*x];let zp=[F::c(1),z,z*z];let mut m0=vec![F::c(0)];let mut m1=vec![F::c(0)];
 for i in 0..3{for j in 0..3{let q=xp[i]*zp[j];pa(&mut m0,&al.tc[3*i+j],q);pa(&mut m1,&al.jc[3*i+j],q)}}pa(&mut m0,&al.gamma,F::c(8)*y*w);[trim(m0),trim(m1)]
}
fn paired<const P:i64,const D:i64>(g:&Base<P,D>,q:&Base<P,D>,p:Pt<P,D>,u:Pt<P,D>,a:i64,b:i64,al:&Alg<P,D>)->([F<P,D>;2],Value){
 if std::env::args().any(|s|s=="--per-block-norm"){return per_block_paired(g,q,p,u,a,b,al)}
 let start=counts();let rr=add(p,u,a);let mut outside=[F::c(1),F::c(0)];let mut reference=[F::c(1),F::c(0)];let mut den=F::c(1);
 let mut surface_giants=vec![];let mut complete=0;let mut maxslots=0;let mut blocks=vec![];let mut covered=vec![false;g.r as usize];let mut pairs=0;let mut singles=0;
 let mut validation=[0;3];let mut formation=[0;3];let mut merges=[0;3];let mut denominators=[0;3];
 for i in 0..g.giants.len(){let center=g.b+i as u64*(2*g.b+1);let ap=add(p,g.giants[i],a);let bp=add(rr,q.giants[i],a);
        let direct_start = counts();
        let mut block = [F::c(1), F::c(0)];
        if center < g.r {
            covered[center as usize] = true;
            let d = ap.unwrap().0 - bp.unwrap().0;
            block = upd(block, [d, F::c(1)]);
            singles += 1
        }
        for j in 1..=g.b {
            let plus = center + j < g.r;
            let minus = center >= j && center - j < g.r;
            if !plus && !minus {
                continue;
            }
            let (hg, kg) = hk(ap, g.baby[j as usize], a, b);
            let (hq, kq) = hk(bp, q.baby[j as usize], a, b);
            let aa = hg - hq;
            let bb = kg - kq;
            if plus && minus {
                let e = [aa * aa - bb * bb, F::c(2) * aa];
                assert_ne!(e[0], aa * aa - (kg + kq) * (kg + kq));
                block = upd(block, e);
                pairs += 1;
                for n in [center - j, center + j] {
                    assert!(!covered[n as usize]);
                    covered[n as usize] = true
                }
            } else {
                let idx = if plus { center + j } else { center - j };
                let d = if plus { aa - bb } else { aa + bb };
                block = upd(block, [d, F::c(1)]);
                singles += 1;
                assert!(!covered[idx as usize]);
                covered[idx as usize] = true
            }
        }

  if center+g.b<g.r{
   reference=upd(reference,block);addcounts(&mut validation,diff(direct_start,counts()));
   let d0=ap.unwrap().0-bp.unwrap().0;outside=upd(outside,[d0,F::c(1)]);
   let (x,y)=ap.unwrap();let(z,w)=bp.unwrap();surface_giants.push((x,y,z,w));
   let st=counts();let d=ev(&al.h,ap.unwrap().0)*ev(&al.hq,bp.unwrap().0);let d=d*d;assert_ne!(d,F::c(0));den=if complete==0{d}else{den*d};complete+=1;addcounts(&mut denominators,diff(st,counts()));
  }else{
   let st=counts();reference=upd(reference,block);addcounts(&mut validation,diff(st,counts()));outside=upd(outside,block);
  }
  blocks.push(block.map(|x|x.code()));
 }
 assert!(covered.iter().all(|&x|x));assert_eq!(2*pairs+singles,g.r);assert!(maxslots<=2*g.b as usize);
 let st=counts();let(nv,surface_record)=surface_contract_small(g,q,a,b,&surface_giants);let branch:Option<usize>=None;let norm_counts=diff(st,counts());let st=counts();let baby=if complete>0{assert_ne!(den,F::c(0));[nv[0]/den,nv[1]/den]}else{nv};let division=diff(st,counts());
 let v=upd(outside,baby);assert_eq!(v,reference);let total=diff(start,counts());let selected:[u64;3]=std::array::from_fn(|i|total[i]-validation[i]);
 (v,json!({"selected_response_counts":selected,"reference_validation_counts":validation,"formation_counts":formation,"quotient_jet_merge_counts":merges,"denominator_counts":denominators,"surface_contraction_counts":norm_counts,"division_counts":division,"surface":surface_record,"final_norm_calls":0,"final_norm_branch":branch,"batch_constant_nonzero":nv[0]!=F::c(0),"complete_blocks":complete,"unused_legacy_accumulator_slots":maxslots,"legacy_bound_not_surface_memory":2*g.b,"paired_factors":pairs,"single_factors":singles,"covered":g.r,"reference_block_coefficients":blocks}))
}
fn batch_controls<const P:i64,const D:i64>(g:Pt<P,D>,a:i64,b:i64,r:u64)->Value{
 let bg=base(g,a,r);let al=algebra(&bg,&bg,a,b);let roots:Vec<_>=bg.baby.iter().skip(1).map(|v|v.unwrap().0).collect();let t=roots[0];
 let unit=[vec![F::c(1)],vec![F::c(1)]];let simple=[vec![-t,F::c(1)],vec![F::c(1)]];
 let mut cases=vec![("empty",vec![]),("one_unit",vec![unit.clone()]),("two_units",vec![unit.clone(),unit.clone()]),("one_simple",vec![simple.clone()]),("same_root_twice",vec![simple.clone(),simple.clone()]),("zero_jet",vec![[vec![F::c(0)],vec![F::c(0)]]]),("zero_then_unit",vec![[vec![F::c(0)],vec![F::c(1)]],[vec![F::c(2)],vec![F::c(1)]]]),("zero_perturbation",vec![[vec![F::c(1)],vec![F::c(0)]],unit.clone()])];
 if roots.len()>1{cases.push(("distinct_roots",vec![simple,[vec![-roots[1],F::c(1)],vec![F::c(1)]]]))}
 let mut rows=vec![];let mut rejected=0;
 for(name,factors)in cases{
  let mut p=[vec![F::c(1)],vec![F::c(0)]];let mut by_norm=[F::c(1),F::c(0)];let mut explicit=[F::c(1),F::c(0)];
  for jet in &factors{p=polyjet_mul(&al.h,&p,jet);by_norm=upd(by_norm,field_normjet(&al.h,&jet[0],&jet[1]).0);for &root in &roots{explicit=upd(explicit,[ev(&jet[0],root),ev(&jet[1],root)])}}
  let(v,branch)=field_normjet(&al.h,&p[0],&p[1]);let(dense,_)=normjet_dense(&al,p[0].clone(),p[1].clone());assert_eq!(v,by_norm);assert_eq!(v,explicit);assert_eq!(v,dense);
  let mut wrong:Option<[Vec<F<P,D>>;2]>=None;for jet in &factors{wrong=Some(if let Some(old)=wrong{[red(pm(&old[0],&jet[0]),&al.h),red(pm(&old[1],&jet[1]),&al.h)]}else{jet.clone()})}
  let bad=wrong.map(|p|field_normjet(&al.h,&p[0],&p[1]).0!=v).unwrap_or(false);if bad{rejected+=1}rows.push(json!({"name":name,"factors":factors.len(),"jet":v.map(|x|x.code()),"branch":branch,"wrong_first_product_rejected":bad}));
 }
 assert!(rejected>0);json!({"p":P,"r":r,"rows":rows,"wrong_product_rejections":rejected})
}
