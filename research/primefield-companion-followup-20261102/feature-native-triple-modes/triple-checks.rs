fn triple_checks(){
 let(mut readers,mut coefficients,mut quotients,mut states)=(0usize,0usize,0usize,0usize);
 for(r,m)in[(7u64,3usize),(13,3),(17,4),(41,5),(97,3),(281,5),(281,35),(1009,63)]{
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let dense=Modes::compile(r,m,z,m,r as usize).unwrap();
  for cap in[1usize,2,7,16,64]{let stream=StreamModes::new(r,m,z,m,r as usize,cap).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();let mut reader=TripleReader::new(&modes).unwrap();let length=reader.remaining;assert_eq!(length,((r-1)/2)as usize/m);
   for j in(0..length).rev(){let got=reader.next().unwrap();for e in 0..3{assert_eq!(got[e],dense.coeff[e][j]);coefficients+=1;}}
   assert_eq!(dense.coeff[0][length],1);assert_eq!(reader.reads,length);assert_eq!(reader.transitions,length-1);assert_eq!(reader.batches,(length+cap-1)/cap);assert!(reader.max_denominators<=3*cap&&reader.max_buffer<=cap);assert_eq!(reader.next().err(),Some(Error::Limit));readers+=1;
   let gs=[vec![0,1],vec![1,0,1],vec![1,2,1],vec![0,0,0,1]];
   for scale in[0,1,3%r,r-1]{let mut rings=gs.iter().map(|g|ReusedRing::scalar_variable(r,g).unwrap()).collect::<Vec<_>>();let values=triple_modes(&mut rings,&modes,scale).unwrap();let mut p=polynomial::Poly::new(r).unwrap();
    for(j,g)in gs.iter().enumerate(){for e in 0..3{let mut power=1;let expected:Vec<_>=dense.coeff[e].iter().map(|&c|{let v=mul(c,power,r);power=mul(power,scale,r);v}).collect();let expected=p.divrem(&expected,g).unwrap().1;assert!(values[j][e].both.is_none());assert_eq!(values[j][e].one.as_ref().unwrap(),&expected);quotients+=1;}}
   }
  }
 }
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5),(281,35)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let stream=StreamModes::new(r,m,z,m,r as usize,7).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();let dense=Modes::compile(r,m,z,m,r as usize).unwrap();let i=(2..r).find(|&i|mul(i,i,r)==r-1).unwrap();
  for x in 0..r{let(mut old,mut new):(Option<DirectImage>,Option<DirectImage>)=(None,None);for b in[2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
   if let Some(o)=&mut old{o.update(&modes,b,&f).unwrap();new.as_mut().unwrap().update_triple(&modes,b,&f).unwrap();}else{old=Some(DirectImage::new(&modes,b,&f,i).unwrap());new=Some(DirectImage::new(&modes,b,&f,i).unwrap());}
   let(o,n)=(old.as_mut().unwrap(),new.as_mut().unwrap());assert_eq!(o.zero,n.zero);assert_eq!(o.rounds,n.rounds);assert_eq!(o.branches.len(),n.branches.len());for(a,b)in o.branches.iter().zip(&n.branches){assert_eq!(a.u,b.u);assert_eq!(a.g,b.g);}assert_eq!(o.materialize(r as usize,r as usize).unwrap(),n.materialize(r as usize,r as usize).unwrap());states+=1;
  }}
 }
 let(r,m)=(13u64,4usize);let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let stream=StreamModes::new(r,m,z,m,20,2).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();assert_eq!(TripleReader::new(&modes).err(),Some(Error::Domain));
 println!("triple_readers={readers} coefficient_equalities={coefficients} quotient_equalities={quotients} candidate_state_equalities={states} buffer_bound=3cap_denominators unsupported_alignment_rejected=true");
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,m,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut cases,mut updates)=(0,0);
 for cap in[8usize,16,64]{for k in 0..r{let target=multiple(curve,bp,k);DIRECT_COUNTS.with(|c|c.set((0,0,0,0)));let old=modes.recover_direct(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let old_counts=DIRECT_COUNTS.with(|c|c.get());DIRECT_COUNTS.with(|c|c.set((0,0,0,0)));let new=modes.recover_triple(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new_counts=DIRECT_COUNTS.with(|c|c.get());assert_eq!(old_counts,new_counts);updates+=new_counts.1;assert_eq!(old.0,new.0);assert_eq!(old.2,new.2);assert_eq!((old.1.calls,old.1.extra,old.1.replayed,old.1.lifted,old.1.image_count),(new.1.calls,new.1.extra,new.1.replayed,new.1.lifted,new.1.image_count));if let Some(v)=new.0{assert_eq!(v,k)}cases+=1;}}
 assert!(updates>0);println!("triple_public_cases={cases} exercised_updates={updates} outcomes_routes_source_calls_equal=true");
}
fn main(){if std::env::args().any(|a|a=="--inherited"){inherited_checks()}else{triple_checks();terminal_checks();progression_inverse_checks();meter_checks();}}
