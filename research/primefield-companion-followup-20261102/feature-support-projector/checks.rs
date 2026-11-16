fn observe(r:u64,m:usize,zeta:u64,k:u64,b:u64)->Observation{let mut z=1;let mut f=[0;3];for _ in 0..m{let s=pow((k+r-mul(b,z,r))%r,(r-1)/2,r);f[0]=(f[0]+s)%r;f[1]=(f[1]+mul(z,s,r))%r;f[2]=(f[2]+mul(pow(z,r-2,r),s,r))%r;z=mul(z,zeta,r)}Observation{b,f}}
fn caps()->Caps{Caps{terms:128,stored:500000,pairs:2000000,output:64}}
fn root_polynomial(r:u64,roots:&[u64])->Vec<u64>{let mut p=polynomial::Poly::new(r).unwrap();let mut q=vec![1];for &y in roots{q=p.mul(&q,&[(r-y)%r,1]);}q}
fn sparse_eval(f:&Sparse,y:u64,r:u64)->u64{f.iter().fold(0,|s,(&e,&c)|(s+mul(c,pow(y,e as u64,r),r))%r)}
fn projector_checks(){let(mut subsets,mut singletons)=(0,0);
 for(r,l)in[(17u64,8usize),(19,3)]{for epsilon in [1,r-1]{let phase:Vec<_>=(1..r).filter(|&y|pow(y,l as u64,r)==epsilon).collect();assert_eq!(phase.len(),l);
  for mask in 0usize..1<<l{let roots:Vec<_>=phase.iter().enumerate().filter_map(|(j,&y)|if mask>>j&1==1{Some(y)}else{None}).collect();let mut indicator=Sparse::new();for e in 0..l{let c=mul(pow(l as u64,r-2,r),roots.iter().fold(0,|s,&y|(s+pow(pow(y,r-2,r),e as u64,r))%r),r);if c!=0{indicator.insert(e,c);}}
   let mut ring=PhaseAlgebra::new(r,l,epsilon,caps()).unwrap();assert_eq!(ring.annihilator(&indicator).unwrap(),root_polynomial(r,&roots));assert_eq!(ring.product(&indicator,&indicator).unwrap(),indicator);
   for &y in &phase{assert_eq!(sparse_eval(&indicator,y,r),u64::from(roots.contains(&y)));}
   if !roots.is_empty(){assert!(indicator.len()*roots.len()>=l);}if roots.len()==1{assert_eq!(indicator.len(),l);singletons+=1;}subsets+=1;
  }
 }}
 let(mut systems,mut memberships,mut max_pairs,mut max_stored,mut max_terms)=(0,0,0,0,0);
 for(r,m)in[(19u64,3usize),(41,5),(97,3),(281,7)]{let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let ks:Vec<_>=if r==19{(1..r).collect()}else{vec![1,2,3,7,17,r-1]};let radii=[2,1,3,2];
  for k in ks{for t in [1,4]{for sign in [1,r-1]{let obs:Vec<_>=radii[..t].iter().map(|&b|observe(r,m,zeta,k,b)).collect();let problem=ProjectorProblem::build(r,m,sign,&obs,100000).unwrap();let(q,indicator,stats)=problem.extract(caps()).unwrap();let roots:std::collections::BTreeSet<_>=(1..r).filter(|&j|pow(j,(r-1)/2,r)==sign&&radii[..t].iter().zip(&obs).all(|(&b,o)|observe(r,m,zeta,j,b)==*o)).map(|j|pow(j,m as u64,r)).collect();let roots:Vec<_>=roots.into_iter().collect();assert_eq!(q,root_polynomial(r,&roots),"r={r} m={m} k={k} t={t} sign={sign}");
   for y in 1..r{if pow(y,problem.l as u64,r)==sign{assert_eq!(sparse_eval(&indicator,y,r),u64::from(roots.contains(&y)));memberships+=1;}}
   if !roots.is_empty(){assert!(indicator.len()*roots.len()>=problem.l);}max_pairs=max_pairs.max(stats.pairs);max_stored=max_stored.max(stats.stored);max_terms=max_terms.max(stats.max_terms);systems+=1;
  }}}
 }
 let r=19;let zeta=7;assert_eq!(pow(zeta,3,r),1);let obs=[observe(r,3,zeta,2,2)];let problem=ProjectorProblem::build(r,3,pow(2,9,r),&obs,10000).unwrap();
 for(which,c)in[("pairs",Caps{pairs:0,..caps()}),("stored",Caps{stored:0,..caps()}),("terms",Caps{terms:0,..caps()})]{let(e,s)=problem.extract(c).err().unwrap();assert_eq!(e,Error::Limit);assert_eq!(s.limit,Some(which));}
 let mut ring=PhaseAlgebra::new(17,8,1,Caps{output:0,..caps()}).unwrap();let one=ring.constant(1).unwrap();assert!(matches!(ring.annihilator(&one),Err(Error::Limit)));assert_eq!(ring.stats.limit,Some("output"));
 // Ordinary truncation misses cyclic wrap contributions.
 let mut ring=PhaseAlgebra::new(17,8,16,caps()).unwrap();let a=Sparse::from([(7,1)]);let b=Sparse::from([(1,1)]);assert_eq!(ring.product(&a,&b).unwrap(),Sparse::from([(0,16)]));
 let r=998244353;let m=7;let zeta=pow(3,(r-1)/m as u64,r);let obs:Vec<_>=[2,1,3,2].iter().map(|&b|observe(r,m,zeta,17,b)).collect();let problem=ProjectorProblem::build(r,m,pow(17,(r-1)/2,r),&obs,100000).unwrap();let(e,stats)=problem.extract(Caps{terms:64,stored:100000,pairs:2000000,output:8}).err().unwrap();assert_eq!(e,Error::Limit);assert_eq!(stats.limit,Some("terms"));println!("large_L={} refusal={:?} pairs_before_refusal={} stored_before_refusal={} max_terms={}",problem.l,stats.limit,stats.pairs,stats.stored,stats.max_terms);
 println!("subset_projectors={subsets} dense_singleton_controls={singletons} transcript_annihilators={systems} phase_memberships={memberships} max_pairs={max_pairs} max_stored_coefficients={max_stored} max_terms={max_terms} cap_controls=4 cyclic_truncation_counterexamples=1");
}
fn projector_public_checks(){let(mut cases,mut extracted,mut endpoints,mut unresolved,mut limited,mut calls,mut pairs,mut guards)=(0,0,0,0,0,0,0,0);
 for(p,a,b,r,base,m)in[(17u64,2u64,2u64,19u64,[5u64,1u64],3usize),(257,1,7,281,[1,3],5)]{let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};assert_eq!(multiple(curve,bp,r),None);let source=source::Source::new(p,a,b,r,base,64).unwrap();let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let ks:Vec<_>=if r==19{(0..r).collect()}else{vec![0,1,2,3,7,17,31,63,127,280]};
  for k in ks{let target=multiple(curve,bp,k);let got=recover_projector(&source,curve,bp,target,r,m,zeta,&[2,1,3,2],1+4*m,100000,caps()).unwrap();if let Some(j)=got.scalar{assert_eq!(j,k);}match got.route{"projector-singleton"=>{assert_eq!(got.degree,Some(1));extracted+=1},"endpoint"=>endpoints+=1,"image-unresolved"=>unresolved+=1,"projector-cap"=>limited+=1,_=>panic!("unexpected route")};cases+=1;calls+=got.calls;pairs+=got.stats.pairs;
   if k!=0{let guard=recover_projector(&source,curve,bp,target,r,m,zeta,&[2],1,100000,caps()).unwrap();assert_eq!(guard.route,"call-cap");assert_eq!(guard.calls,1);guards+=1;}
  }
 }
 assert!(extracted>0);println!("public_cases={cases} extracted_and_verified={extracted} identity_endpoints={endpoints} unresolved={unresolved} capped={limited} source_calls={calls} projector_pairs={pairs} call_guards={guards}");
}
