fn context_checks(){
 let(mut sequences,mut coefficients,mut isolated_poly,mut isolated_grid,mut shared_poly,mut shared_grid)=(0,0,0,0,0,0);
 for cap in [1usize,2,7,16,64]{
  let context=RecoveryProducts::new(1009,56,cap).unwrap();
  for _ in 0..3{for e in 0..3{
   let mut old=CoeffStream::new(1009,56,e,cap).unwrap();let mut new=SharedCoeffStream::new(1009,56,e,cap,&context).unwrap();
   while old.left>0{let count=old.left.min(cap);let a=old.take_desc(count).unwrap();let b=new.take_desc(count).unwrap();assert_eq!(a,b);assert_eq!((new.left,new.index,new.value),(old.left,old.index,old.value));coefficients+=a.len();}
   sequences+=1;isolated_poly+=old.products.polynomial_setups;isolated_grid+=old.products.progression_setups;
  }}
  let q=context.inner.borrow();assert!(q.width<=cap);assert_eq!(q.request_cap,cap*2);assert!(q.polynomial_setups<=1&&q.progression_setups<=1);shared_poly+=q.polynomial_setups;shared_grid+=q.progression_setups;drop(q);
  // Fresh sessions do not inherit setup; malformed identities cannot use the cache.
  let fresh=RecoveryProducts::new(1009,56,cap).unwrap();assert!(fresh.inner.borrow().polynomial.is_none());assert!(fresh.inner.borrow().progression.is_none());
  assert!(SharedCoeffStream::new(1009,112,0,cap,&context).is_err());assert!(SharedCoeffStream::new(2017,56,0,cap,&context).is_err());assert!(SharedCoeffStream::new(1009,56,0,cap+1,&context).is_err());
  let guard=context.inner.borrow_mut();assert!(context.products(&[(1,2)]).is_err());drop(guard);assert_eq!(context.products(&[(1,2)]).unwrap(),vec![2]);
  // Simultaneous stream lifetimes, with interleaved calls and no long-lived borrow.
  let mut a=SharedCoeffStream::new(1009,56,0,cap,&context).unwrap();let mut b=SharedCoeffStream::new(1009,56,1,cap,&context).unwrap();let mut ar=CoeffStream::new(1009,56,0,cap).unwrap();let mut br=CoeffStream::new(1009,56,1,cap).unwrap();
  while a.left>0||b.left>0{if a.left>0{assert_eq!(a.take_desc(1).unwrap(),ar.take_desc(1).unwrap());}if b.left>0{assert_eq!(b.take_desc(1).unwrap(),br.take_desc(1).unwrap());}}
 }
 assert!(isolated_poly>shared_poly&&isolated_grid>shared_grid);
 println!("context_sequences={sequences} exact_coefficients={coefficients} isolated_polynomial_setups={isolated_poly} shared_polynomial_setups={shared_poly} isolated_progression_setups={isolated_grid} shared_progression_setups={shared_grid} cap_panels=5 mismatched_keys_rejected=15 borrow_conflicts_rejected=5 interleaved_pairs=5");
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],56usize);let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,m,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut poly,mut grid,mut lazy)=(0,0,0,0);
 for k in 0..r{
  let target=multiple(c,bp,k);let old=modes.recover_isolated(&source,c,bp,target,&[1,2,3,4],64,&extra,256).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();assert!(session.products.inner.borrow().polynomial.is_none());
  let new=session.recover_traced(&source,c,bp,target,&[1,2,3,4],64,&extra,256).unwrap();assert_eq!(old.0,new.0);assert_eq!(new.0,Some(k));assert_eq!(old.2,new.2);let(os,ns)=(old.1,new.1);assert_eq!((os.calls,os.extra,os.replayed,os.lifted,os.image_count),(ns.calls,ns.extra,ns.replayed,ns.lifted,ns.image_count));calls+=ns.calls;
  let q=session.products.inner.borrow();assert!(q.polynomial_setups<=1&&q.progression_setups<=1);poly+=q.polynomial_setups;grid+=q.progression_setups;if q.polynomial_setups==0{lazy+=1;}
 }
 println!("context_recovery_cases={r} source_calls={calls} unresolved=0 polynomial_setups={poly} progression_setups={grid} no_setup_recoveries={lazy} fresh_context_per_target=true");
}
