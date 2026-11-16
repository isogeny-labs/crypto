fn fused_checks(){fused_sequence_checks();fused_value_checks();let mut states=0usize;
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let(mut old,mut new):(Option<NeighborImage>,Option<FusedImage>)=(None,None);
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(old)=&mut old{old.update(&session,b,&f).unwrap();new.as_mut().unwrap().update(&session,b,&f).unwrap();}else{old=Some(NeighborImage::new(&session,b,&f,i).unwrap());new=Some(FusedImage::new(&session,b,&f,i).unwrap());}
    let old=old.as_mut().unwrap();let new=new.as_mut().unwrap();assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;
    assert_eq!(old.branches.len(),new.branches.len());for (a,b) in old.branches.iter().zip(new.branches.iter()){assert_eq!(a.u,b.u);assert_eq!(canonical_signature(&a.canonical),canonical_signature(&b.canonical));assert_eq!(a.ring.g,b.ring.g);}
   }
  }
 }
 println!("fused_canonical_states={states}");
 for cap in [64usize,16]{FUSED_WORK.with(|c|c.set((0,0,0,0,0,0,0)));FUSED_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_neighbor(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_fused(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(pairs,coefficients,transitions,inversions,multiplications,removed_inversions,max_denominators)=FUSED_WORK.with(|c|c.get());let reference_inversions=inversions+removed_inversions;let reference_multiplications=multiplications+removed_inversions;println!("fused_public_work cap={cap} pairs={pairs} coefficients={coefficients} transitions={transitions} inversions={inversions} reference_inversions={reference_inversions} multiplications_outside_exponentiation={multiplications} reference_multiplications_outside_exponentiation={reference_multiplications} max_denominators={max_denominators}");
  let(inits,updates,deferrals,materializations)=FUSED_COUNTS.with(|c|c.get());assert_eq!(unresolved,0);println!("fused_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}
fn fused_value_checks(){
 let(mut equalities,mut groups,mut products)=(0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(257,4),(1009,4)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,1024,1024,7).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let mut p=polynomial::Poly::new(r).unwrap();let mut ws:Vec<_>=(1..5).map(|v|(v+pow(v,r-2,r))%r).collect();ws.sort();ws.dedup();let g=p.product_tree(&ws).unwrap().polynomial().to_vec();let state=CanonicalFiber::new(r,&g).unwrap();
  for width in 0..5{let mut old:Vec<_>=(0..width).map(|_|ReusedRing::new(&state).unwrap()).collect();let mut new:Vec<_>=(0..width).map(|_|ReusedRing::new(&state).unwrap()).collect();for scale in [1,2,1]{let(a,b)=neighbor_modes(&mut old,&session,scale).unwrap();let(c,d)=fused_modes(&mut new,&session,scale).unwrap();assert_eq!(a.len(),c.len());assert_eq!(b.len(),d.len());for j in 0..width{same_split(&a[j],&c[j]);same_split(&b[j],&d[j]);assert_eq!(old[j].inner.scalar_products,new[j].inner.scalar_products);assert_eq!(old[j].inner.pair_products,new[j].inner.pair_products);equalities+=2;products+=new[j].inner.scalar_products+new[j].inner.pair_products;}groups+=1;}}
 }
 println!("fused_mode_groups={groups} branch_mode_equalities={equalities} ring_product_counters_equal=true cumulative_counter_observation_sum={products}");
}
fn fused_sequence_checks(){
 let(mut sequences,mut coefficients,mut boundaries,mut inversions,mut old_inversions,mut multiplications,mut old_multiplications,mut max_denominators)=(0,0,0,0,0,0,0,0);
 for r in [7u64,13,17,31,41,97,193,257,281,1009]{for m in 3..r as usize{if(r-1)%m as u64!=0{continue}let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();
  for cap in [1usize,2,7,16,64]{let modes=StreamModes::new(r,m,z,1024,1024,cap).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let reference_session=RecoveryModes::from_modes(&modes).unwrap();let mut old=NeighborReader::new(&reference_session).unwrap();let mut new=FusedReader::new(&session).unwrap();let length=new.remaining;let(mut expected_i,mut expected_m)=(0,0);
   for _ in 0..length{if old.at==old.buffer.len(){let c=old.remaining.min(cap);let s=c.min(old.anchor.left-1);expected_i+=1+usize::from(s>0);expected_m+=5*(s+c)+1+usize::from(s>0);boundaries+=1;}
    assert_eq!(old.next().unwrap(),new.next().unwrap());assert_eq!((old.anchor.value,old.anchor.index,old.anchor.left,old.remaining,old.at),(new.anchor.value,new.anchor.index,new.anchor.left,new.remaining,new.at));coefficients+=1;
   }
   assert_eq!(new.batches+new.removed_inversions,expected_i);assert_eq!(new.multiplications+new.removed_inversions,expected_m);assert_eq!(old.transitions,new.transitions);assert!(new.max_denominators<=2*cap);assert!(new.next().is_err());assert!(new.refill().is_err());inversions+=new.batches;old_inversions+=expected_i;multiplications+=new.multiplications;old_multiplications+=expected_m;max_denominators=max_denominators.max(new.max_denominators);sequences+=1;
  }
 }}
 let mut inverse_checks=0;for r in [7u64,17,41,97]{for n in 1..=16{let ds:Vec<_>=(0..n).map(|j|1+(j as u64*3)%(r-1)).collect();let mut count=0;let inv=fused_invert(r,&ds,&mut count).unwrap();assert_eq!(count,3*n+1);for(&d,&v)in ds.iter().zip(&inv){assert_eq!(mul(d,v,r),1);inverse_checks+=1;}}}
 let mut rejected=0;for(r,d)in[(17,vec![]),(17,vec![0]),(17,vec![17]),(15,vec![3])]{assert!(fused_invert(r,&d,&mut 0).is_err());rejected+=1;}
 assert!(old_inversions>inversions);println!("fused_sequences={sequences} coefficient_and_state_equalities={coefficients} chunk_boundaries={boundaries} inversions={inversions} reference_inversions={old_inversions} multiplications_outside_exponentiation={multiplications} reference_multiplications_outside_exponentiation={old_multiplications} max_denominators={max_denominators} inverse_identity_checks={inverse_checks} invalid_inverse_inputs={rejected}");
}
