fn reciprocal_checks(){
 let(mut configs,mut values,mut branches,mut lifted,mut images,mut rejected_orientation,mut ramified,mut cap_rejects)=(0,0,0,0,0,0,0,0);
 for r in [17u64,41,97,193,257,281]{for m in 2..=16usize{if (r-1)%(4*m as u64)!=0{continue}
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let model=ReciprocalMode::build(&session,1024).unwrap().unwrap();let n=model.n;
  assert!(matches!(ReciprocalMode::build(&session,n-1),Err(Error::Limit)));
  let mut stream=modes.stream(0).unwrap();let mut f=Vec::new();while stream.left>0{f.extend(stream.take_desc(stream.left.min(7)).unwrap());}f.reverse();assert_eq!(f.len(),2*n+1);for j in 0..=2*n{assert_eq!(f[j],f[2*n-j]);}
  for v in 1..r{let w=(v+pow(v,r-2,r))%r;assert_eq!(eval(&f,v,r),mul(pow(v,n as u64,r),eval(&model.h,w,r),r));assert_eq!(eval(&model.dn,w,r),(pow(v,n as u64,r)+pow(pow(v,r-2,r),n as u64,r))%r);values+=1;}
  let i=(2..r).find(|&i|mul(i,i,r)==r-1).unwrap();let cs:Vec<_>=if r<=97{(0..r).collect()}else{vec![0,1,7,r-1]};
  for c in cs{for u in [1,r-1,i,r-i]{let g=model.branch(u,c).unwrap();let vpoly=model.lift(&g,u,2*n+1).unwrap();
   for w in 0..r{assert_eq!(eval(&g,w,r)==0,eval(&model.dn,w,r)==(u+pow(u,3,r))%r&&mul(u,eval(&model.h,w,r),r)==c);branches+=1;}
   for v in 1..r{let w=(v+pow(v,r-2,r))%r;let before=eval(&g,w,r)==0;let expected=before&&pow(v,n as u64,r)==u;assert_eq!(eval(&vpoly,v,r)==0,expected);if before&&!expected{rejected_orientation+=1;}if expected&&(v==1||v==r-1){ramified+=1;}lifted+=1;}
   if g.len()>1{assert!(matches!(model.lift(&g,u,2*(g.len()-1)-1),Err(Error::Limit)));cap_rejects+=1;}
  }
  for b in [1,2,r-1]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let t=pow(pow(b,m as u64,r),r-2,r);let f0=mul(a,c,r);let got=model.image_constraint(a,t,f0,i,2*n+1).unwrap();let mut constraint=f.clone();let mut scale=a;for x in &mut constraint{*x=mul(*x,scale,r);scale=mul(scale,t,r);}constraint[0]=(constraint[0]+r-f0)%r;let mut state=ImageDomain::new(r,m).unwrap();state.intersect(&mut polynomial::Poly::new(r).unwrap(),&constraint).unwrap();assert_eq!(state.dense.as_ref(),Some(&got));images+=1;}
  }
  assert!(model.branch(0,0).is_err());assert!(model.image_constraint(1,0,0,i,64).is_err());assert!(model.image_constraint(1,1,0,1,64).is_err());configs+=1;
 }}
 assert!(rejected_orientation>0&&ramified>0&&cap_rejects>0);
 // Unsupported parity is explicit and retains the original route.
 let modes=StreamModes::new(19,2,18,2,32,7).unwrap();assert!(ReciprocalMode::build(&RecoveryModes::from_modes(&modes).unwrap(),32).unwrap().is_none());
 println!("reciprocal_configs={configs} exact_nonzero_values={values} branch_point_checks={branches} lifted_point_checks={lifted} exact_image_polynomials={images} orientation_rejections={rejected_orientation} ramified_fibers={ramified} lift_cap_rejections={cap_rejects}");
 RECIPROCAL_COUNTS.with(|c|c.set((0,0,0)));
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let mut calls=0;
 for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_isolated(&source,curve,bp,target,&[1,2,3,4],64,&extra,128).unwrap();let new=modes.recover_reciprocal_control(&source,curve,bp,target,&[1,2,3,4],64,&extra,128).unwrap();assert_eq!(old.0,new.0);assert_eq!(new.0,Some(k));assert_eq!(old.2,new.2);let(os,ns)=(old.1,new.1);assert_eq!((os.calls,os.extra,os.replayed,os.lifted,os.image_count),(ns.calls,ns.extra,ns.replayed,ns.lifted,ns.image_count));calls+=ns.calls;}
 let(attempts,successes,fallbacks)=RECIPROCAL_COUNTS.with(|c|c.get());assert!(successes>0);assert_eq!(attempts,successes+fallbacks);
 println!("reciprocal_public_cases={r} source_calls={calls} unresolved=0 compressed_attempts={attempts} compressed_successes={successes} bounded_lift_fallbacks={fallbacks}");
}
