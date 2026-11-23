fn neighbor_checks(){neighbor_sequence_checks();neighbor_value_checks();let mut states=0usize;
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let(mut old,mut new):(Option<SharedImage>,Option<NeighborImage>)=(None,None);
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(old)=&mut old{old.update(&session,b,&f).unwrap();new.as_mut().unwrap().update(&session,b,&f).unwrap();}else{old=Some(SharedImage::new(&session,b,&f,i).unwrap());new=Some(NeighborImage::new(&session,b,&f,i).unwrap());}
    let old=old.as_mut().unwrap();let new=new.as_mut().unwrap();assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;
    assert_eq!(old.branches.len(),new.branches.len());for (a,b) in old.branches.iter().zip(new.branches.iter()){assert_eq!(a.u,b.u);assert_eq!(canonical_signature(&a.canonical),canonical_signature(&b.canonical));assert_eq!(a.ring.g,b.ring.g);}
   }
  }
 }
 println!("neighbor_canonical_states={states}");
 for cap in [64usize,16]{NEIGHBOR_WORK.with(|c|c.set((0,0,0,0,0,0)));NEIGHBOR_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_shared(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_neighbor(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(pairs,anchor_coefficients,anchor_transitions,adjacent_inversions,adjacent_multiplications,max_buffer_pairs)=NEIGHBOR_WORK.with(|c|c.get());println!("neighbor_public_work cap={cap} pairs={pairs} anchor_coefficients={anchor_coefficients} anchor_transitions={anchor_transitions} adjacent_inversions={adjacent_inversions} adjacent_multiplications_outside_exponentiation={adjacent_multiplications} max_buffer_pairs={max_buffer_pairs}");
  let(inits,updates,deferrals,materializations)=NEIGHBOR_COUNTS.with(|c|c.get());assert_eq!(unresolved,0);println!("neighbor_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}
fn neighbor_sequence_checks(){
 let(mut sequences,mut pairs,mut batches,mut short,mut full,mut top_h,mut exhaustions)=(0,0,0,0,0,0,0);
 for r in [7u64,13,17,31,41,97,193,257,281,1009]{for m in 3..r as usize{if(r-1)%m as u64!=0{continue}let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let dense=Modes::compile(r,m,z,1024,1024).unwrap();
  for cap in [1usize,2,7,16,64]{let modes=StreamModes::new(r,m,z,1024,1024,cap).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let mut reader=NeighborReader::new(&session).unwrap();let length=reader.remaining;let h=(r-1)/2;assert_eq!(length,dense.coeff[1].len());assert_eq!(length,dense.coeff[2].len());let start=reader.anchor.index;if start+1==h{top_h+=1;}if length==1{short+=1;}if m==r as usize-1{full+=1;}
   for j in 0..length{let v=reader.next().unwrap();assert_eq!(v,[dense.coeff[1][length-1-j],dense.coeff[2][length-1-j]]);let n=start-j as u64*m as u64;assert!(n>=1&&n+1<=h&&n+1<r);pairs+=1;}
   assert_eq!(reader.transitions,length-1);assert_eq!(reader.batches,(length+cap-1)/cap);assert!(reader.max_pairs<=cap);assert!(reader.next().is_err());exhaustions+=1;batches+=reader.batches;sequences+=1;
  }
 }}
 let mut rejects=0;for(h,m,start,a)in[(8,4,8,vec![1]),(8,4,0,vec![1]),(8,4,1,vec![1,2]),(8,4,5,vec![17]),(7,4,5,vec![1]),(8,2,5,vec![1])]{assert!(adjacent_coefficients(17,h,m,start,&a).is_err());rejects+=1;}
 assert!(short>0&&full>0&&top_h>0);println!("neighbor_sequences={sequences} coefficient_pairs={pairs} adjacent_batch_inversions={batches} singleton_sequences={short} maximal_order_sequences={full} top_index_reaches_h={top_h} exhausted_reader_rejections={exhaustions} invalid_neighbor_inputs={rejects}");
}
fn neighbor_value_checks(){
 let(mut equalities,mut groups,mut products)=(0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(257,4),(1009,4)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,1024,1024,7).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let mut p=polynomial::Poly::new(r).unwrap();let mut ws:Vec<_>=(1..5).map(|v|(v+pow(v,r-2,r))%r).collect();ws.sort();ws.dedup();let g=p.product_tree(&ws).unwrap().polynomial().to_vec();let state=CanonicalFiber::new(r,&g).unwrap();
  for width in 0..5{let mut old:Vec<_>=(0..width).map(|_|ReusedRing::new(&state).unwrap()).collect();let mut new:Vec<_>=(0..width).map(|_|ReusedRing::new(&state).unwrap()).collect();for scale in [1,2,1]{let a=shared_mode(&mut old,&session,1,scale).unwrap();let b=shared_mode(&mut old,&session,2,scale).unwrap();let(c,d)=neighbor_modes(&mut new,&session,scale).unwrap();assert_eq!(a.len(),c.len());assert_eq!(b.len(),d.len());for j in 0..width{same_split(&a[j],&c[j]);same_split(&b[j],&d[j]);assert_eq!(old[j].inner.scalar_products,new[j].inner.scalar_products);assert_eq!(old[j].inner.pair_products,new[j].inner.pair_products);equalities+=2;products+=new[j].inner.scalar_products+new[j].inner.pair_products;}groups+=1;}}
 }
 println!("neighbor_mode_groups={groups} branch_mode_equalities={equalities} ring_product_counters_equal=true cumulative_counter_observation_sum={products}");
}
