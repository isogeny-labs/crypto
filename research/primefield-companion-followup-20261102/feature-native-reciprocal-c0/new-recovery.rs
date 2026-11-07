fn main(){reciprocal_checks();context_checks();cached_shift_checks();progression_checks();packed_checks();interval_checks();interval_public_checks();stream_checks();residue_constraint_checks();implicit_checks();previous_main();let(mut cases,mut calls,mut unresolved)=(0,0,0);
 for(p,a,b,r,base,m)in[(17,2,2,19,[0,6],2),(41,1,5,47,[0,13],2),(97,0,5,79,[1,43],3),(193,2,7,191,[0,59],2),(257,1,7,281,[1,3],5)]{
  let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=Modes::compile(r,m,z,16,256).unwrap();let streamed=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17.min(r)).collect();
  for k in 0..r{let target=multiple(c,bp,k);let(old,os,route)=modes.recover_enveloped(&source,c,bp,target,&[1,2,3,4],64,&extra,128).unwrap();let(new,ns,nroute)=streamed.recover_traced(&source,c,bp,target,&[1,2,3,4],64,&extra,128).unwrap();assert_eq!(new,old);assert_eq!(route,nroute);assert_eq!((os.calls,os.replayed,os.extra,os.lifted,os.image_count),(ns.calls,ns.replayed,ns.extra,ns.lifted,ns.image_count));if let Some(v)=new{assert_eq!(v,k)}else{unresolved+=1}calls+=ns.calls;cases+=1;}
 }
 println!("streamed_recovery_cases={cases} source_calls={calls} unresolved={unresolved}");assert_eq!(unresolved,0);
}
