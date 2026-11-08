fn quadratic_checks(){
 let(mut pair_cases,mut norm_false,mut degenerates,mut ramified)=(0,0,0,0);
 for r in [7u64,17,31]{for w in 0..r{let roots:Vec<_>=(1..r).filter(|&v|(mul(v,v,r)+1)%r==mul(w,v,r)).collect();if roots.is_empty(){continue}
  let mut ring=QuadRing::new(r,&[(r-w)%r,1]).unwrap();
  for a in [0,1,2,r-1]{for b in [0,1,2,r-1]{for c in [0,1,2,r-1]{for d in [0,1,2,r-1]{
   let x=Quad([vec![a],vec![b]]);let y=Quad([vec![c],vec![d]]);let expected=roots.iter().any(|&v|(a+mul(b,v,r))%r==0&&(c+mul(d,v,r))%r==0);let kept=ring.common(&x,&y).unwrap();assert_eq!(kept.len()==2,expected);
   let nx=ring.norm(&x).unwrap();let ny=ring.norm(&y).unwrap();if nx==[0]&&ny==[0]&&!expected{norm_false+=1;}if b==0||d==0{degenerates+=1;}if roots.len()==1{ramified+=1;}pair_cases+=1;
  }}}}
 }}assert!(norm_false>0);
 let(mut configs,mut full,mut ring_values,mut capped)=(0,0,0,0);
 for r in [17u64,41,97,193,281]{for m in 3..=16usize{if(r-1)%(4*m as u64)!=0{continue}let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let session=RecoveryModes::from_modes(&modes).unwrap();let model=ReciprocalMode::build(&session,1024).unwrap().unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  // Both modes in a nontrivial product quotient agree with independent dense coefficients.
  let ws:Vec<_>=(1..r).map(|v|(v+pow(v,r-2,r))%r).take(5).collect();let mut unique=ws;unique.sort();unique.dedup();let g=polynomial::Poly::new(r).unwrap().product_tree(&unique).unwrap().polynomial().to_vec();let mut ring=QuadRing::new(r,&g).unwrap();
  for e in [1,2]{let q=ring.mode(&session,e).unwrap();let mut stream=modes.stream(e).unwrap();let mut coeff=Vec::new();while stream.left>0{coeff.extend(stream.take_desc(stream.left.min(7)).unwrap());}coeff.reverse();for v in 1..r{let w=(v+pow(v,r-2,r))%r;if eval(&g,w,r)==0{assert_eq!((eval(&q.0[0],w,r)+mul(eval(&q.0[1],w,r),v,r))%r,eval(&coeff,v,r));ring_values+=1;}}}
  for b in [1,2]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let t=pow(pow(b,m as u64,r),r-2,r);for f0 in [0,a,r-1]{for f1 in [0,1,r-1]{for f2 in [0,1,r-1]{let f=[f0,f1,f2];let got=model.quadratic_constraint(&session,a,t,&f,i,2*model.n+1).unwrap();let mut state=ImageDomain::new(r,m).unwrap();modes.intersect_modes(&mut polynomial::Poly::new(r).unwrap(),&mut state,b,&f).unwrap();assert_eq!(state.dense.as_ref(),Some(&got));full+=1;
   if got.len()>1{assert!(matches!(model.quadratic_constraint(&session,a,t,&f,i,0),Err(Error::Limit)));capped+=1;}
  }}}}configs+=1;
 }}
 println!("quadratic_pair_cases={pair_cases} norms_alone_false_positives={norm_false} zero_leading_controls={degenerates} ramified_controls={ramified} mode_configs={configs} exact_mode_values={ring_values} exact_full_intersections={full} cap_rejections={capped}");
 QUADRATIC_COUNTS.with(|c|c.set((0,0,0)));QUADRATIC_DEGREES.with(|c|c.set((0,0,0,0)));
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let mut calls=0;
 for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_isolated(&source,curve,bp,target,&[1,2,3,4],64,&extra,128).unwrap();let new=modes.recover_traced(&source,curve,bp,target,&[1,2,3,4],64,&extra,128).unwrap();assert_eq!(old.0,new.0);assert_eq!(new.0,Some(k));assert_eq!(old.2,new.2);let(os,ns)=(old.1,new.1);assert_eq!((os.calls,os.extra,os.replayed,os.lifted,os.image_count),(ns.calls,ns.extra,ns.replayed,ns.lifted,ns.image_count));calls+=ns.calls;}
 let(attempts,successes,fallbacks)=QUADRATIC_COUNTS.with(|c|c.get());let(branches,before,after,strict)=QUADRATIC_DEGREES.with(|c|c.get());assert_eq!(attempts,successes+fallbacks);assert!(strict>0&&successes>0);
 println!("quadratic_public_cases={r} source_calls={calls} unresolved=0 attempts={attempts} successes={successes} fallbacks={fallbacks} processed_branches={branches} degree_sum_before={before} degree_sum_after={after} strict_reductions={strict}");
}
