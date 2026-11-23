fn rescue_stream_values(r:u64,m:usize,cap:usize,context:&RecoveryProducts)->Vec<u64>{
 let mut result=Vec::new();for e in 0..3{let mut s=SharedCoeffStream::new(r,m,e,cap,context).unwrap();while s.left>0{result.extend(s.take_desc(s.left.min(cap)).unwrap());}}result
}
fn shared_rescue_checks(){
 let(mut panels,mut coefficients,mut shared_poly,mut fixed_poly,mut native_poly,mut shared_grid,mut fixed_grid,mut native_grid,mut shared_batches,mut fixed_batches,mut native_batches,mut rejects,mut interleaved)=(0,0,0,0,0,0,0,0,0,0,0,0,0);
 for(r,m)in[(281u64,35usize),(1009,63),(2017,63),(2521,315)]{assert!((2..r).take_while(|d|d*d<=r).all(|d|r%d!=0));let mm=2*m;let eta=(2..r).find(|&z|pow(z,mm as u64,r)==1&&(1..mm).all(|j|pow(z,j as u64,r)!=1)).unwrap();
  for cap in [1usize,2,7,16,64]{
   let base=RecoveryProducts::new(r,m,cap).unwrap();let fixed_base=RecoveryProducts::new(r,m,cap).unwrap();let native_base=RecoveryProducts::new(r,m,cap).unwrap();
   let a=rescue_stream_values(r,m,cap,&base);assert_eq!(a,rescue_stream_values(r,m,cap,&fixed_base));assert_eq!(a,rescue_stream_values(r,m,cap,&native_base));coefficients+=a.len();
   let stream=StreamModes::new(r,mm,eta,mm,r as usize,cap).unwrap();let shared=base.for_doubled(&stream).unwrap();assert!(std::ptr::eq(&*base.inner,&*shared.inner));assert!(matches!(base.inner,EngineStorage::Owned(_)));assert!(matches!(shared.inner,EngineStorage::Borrowed(_)));
   let fixed=RecoveryProducts::new(r,mm,cap).unwrap();fixed.inner.borrow_mut().width=base.inner.borrow().width;
   let native=RecoveryProducts::new(r,mm,cap).unwrap();let a=rescue_stream_values(r,mm,cap,&shared);assert_eq!(a,rescue_stream_values(r,mm,cap,&fixed));assert_eq!(a,rescue_stream_values(r,mm,cap,&native));coefficients+=a.len();
   let q=base.inner.borrow();let f0=fixed_base.inner.borrow();let f=fixed.inner.borrow();let n0=native_base.inner.borrow();let n=native.inner.borrow();assert_eq!(q.evaluation_batches,f0.evaluation_batches+f.evaluation_batches);assert!(q.polynomial_setups<=1&&q.progression_setups<=1);assert!(q.max_points<=q.width&&q.width<=cap);
   shared_poly+=q.polynomial_setups;fixed_poly+=f0.polynomial_setups+f.polynomial_setups;native_poly+=n0.polynomial_setups+n.polynomial_setups;
   shared_grid+=q.progression_setups;fixed_grid+=f0.progression_setups+f.progression_setups;native_grid+=n0.progression_setups+n.progression_setups;
   shared_batches+=q.evaluation_batches;fixed_batches+=f0.evaluation_batches+f.evaluation_batches;native_batches+=n0.evaluation_batches+n.evaluation_batches;drop(q);drop(f0);drop(f);drop(n0);drop(n);
   let badfield=StreamModes{r:r+2,m:mm,zeta:eta,block:cap};let badstride=StreamModes{r,m:mm+1,zeta:eta,block:cap};let badcap=StreamModes{r,m:mm,zeta:eta,block:cap+1};for bad in [&badfield,&badstride,&badcap]{assert!(base.for_doubled(bad).is_err());rejects+=1;}
   let guard=base.inner.borrow_mut();assert!(shared.products(&[(1,2)]).is_err());assert!(base.for_doubled(&stream).is_err());rejects+=2;drop(guard);
   assert!(SharedCoeffStream::new(r,m,0,cap,&shared).is_err());rejects+=1;
   let mut x=SharedCoeffStream::new(r,m,0,cap,&base).unwrap();let mut y=SharedCoeffStream::new(r,mm,0,cap,&shared).unwrap();let mut xr=CoeffStream::new(r,m,0,cap).unwrap();let mut yr=CoeffStream::new(r,mm,0,cap).unwrap();while x.left>0||y.left>0{if x.left>0{assert_eq!(x.take_desc(1).unwrap(),xr.take_desc(1).unwrap());}if y.left>0{assert_eq!(y.take_desc(1).unwrap(),yr.take_desc(1).unwrap());}}interleaved+=1;drop(x);drop(y);drop(shared);assert!(base.inner.try_borrow_mut().is_ok());panels+=1;
  }
 }
 assert!(shared_poly<fixed_poly&&shared_grid<fixed_grid);println!("shared_rescue_panels={panels} coefficient_equalities={coefficients} shared_polynomial_setups={shared_poly} fixed_width_polynomial_setups={fixed_poly} native_width_polynomial_setups={native_poly} shared_grid_setups={shared_grid} fixed_width_grid_setups={fixed_grid} native_width_grid_setups={native_grid} shared_evaluation_batches={shared_batches} fixed_width_evaluation_batches={fixed_batches} native_width_evaluation_batches={native_batches} rejected_controls={rejects} interleaved_stream_pairs={interleaved}");
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();
 for cap in [8usize,10,16,20,32,64]{RESCUE_SHARES.with(|c|c.set([0;4]));let(mut calls,mut recovered,mut changes)=(0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_doubled(&source,curve,bp,target,&[1,r-1,2,3],cap,&extra,128).unwrap();let new=modes.recover_shared_rescue(&source,curve,bp,target,&[1,r-1,2,3],cap,&extra,128).unwrap();assert_eq!(old.0,new.0);assert_eq!(old.1.calls,new.1.calls);if old.2!=new.2{changes+=1;}assert_eq!(old.2,new.2);if let Some(v)=new.0{assert_eq!(v,k);recovered+=1;}calls+=new.1.calls;
  }
  let s=RESCUE_SHARES.with(|c|c.get());println!("shared_rescue_public_cases={r} cap={cap} source_calls={calls} recovered={recovered} route_changes={changes} shared_engines={} warm_shares={} cold_shares={} retained_width_sum={}",s[0],s[1],s[2],s[3]);
 }
}
