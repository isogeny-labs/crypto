fn direct_checks(){direct_fold_checks();let mut states=0usize;
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5)]{let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,1024,7).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();
  for x in 0..r{let session=RecoveryModes::from_modes(&modes).unwrap();let(mut old,mut new):(Option<FusedImage>,Option<DirectImage>)=(None,None);
   for b in [2,1,3,4]{let a=mul(m as u64,pow(b,(r-1)/2,r),r);let y=mul(x,pow(b,r-2,r),r);let v=pow(y,m as u64,r);let f:Vec<_>=(0..3).map(|e|mul(mul(a,pow(y,e as u64,r),r),eval(&dense.coeff[e],v,r),r)).collect();
    if let Some(old)=&mut old{old.update(&session,b,&f).unwrap();new.as_mut().unwrap().update(&session,b,&f).unwrap();}else{old=Some(FusedImage::new(&session,b,&f,i).unwrap());new=Some(DirectImage::new(&session,b,&f,i).unwrap());}
    let old=old.as_mut().unwrap();let new=new.as_mut().unwrap();assert_eq!(old.materialize(r as usize,r as usize).unwrap(),new.materialize(r as usize,r as usize).unwrap());assert_eq!(old.zero,new.zero);states+=1;
    for q in &old.branches{let old_poly=q.canonical.lift(&old.orientation,q.u,r as usize,&mut QuadRing::new(r,&[0,1]).unwrap()).unwrap();let new_poly=new.branches.iter().find(|b|b.u==q.u).map_or(vec![1],|b|b.g.clone());assert_eq!(old_poly,new_poly);}

   }
  }
 }
 println!("direct_candidate_states={states}");
 for cap in [64usize,16]{DIRECT_COUNTS.with(|c|c.set((0,0,0,0)));
  let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut old_calls,mut route_changes,mut unresolved)=(0,0,0,0);
  for k in 0..r{let target=multiple(curve,bp,k);let old=modes.recover_fused(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();let new=modes.recover_direct(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();if let Some(v)=new.0{assert_eq!(v,k)}else{unresolved+=1;}calls+=new.1.calls;old_calls+=old.1.calls;if old.2!=new.2{route_changes+=1;}}
  let(inits,updates,deferrals,materializations)=DIRECT_COUNTS.with(|c|c.get());assert_eq!(unresolved,0);println!("direct_public_cases={r} cap={cap} source_calls={calls} reference_source_calls={old_calls} route_changes={route_changes} unresolved={unresolved} initializations={inits} subsequent_updates={updates} deferrals={deferrals} materializations={materializations}");
 }
}
fn direct_fold_checks(){
 let(mut configs,mut residues,mut supports,mut zero_equations,mut constant_nonzero,mut caps,mut coefficient_reads)=(0,0,0,0,0,0,0);
 for r in [17u64,41,97,193,281]{for m in 2..=16usize{if(r-1)%(4*m as u64)!=0{continue}let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();let stream=StreamModes::new(r,m,z,16,1024,7).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();let dense=Modes::compile(r,m,z,16,1024).unwrap();let n=((r-1)/(4*m as u64))as usize;let fold=PhaseFold::build(&modes,i,n).unwrap();let reciprocal=ReciprocalMode::build(&modes,n).unwrap().unwrap();assert_eq!(fold.reads,n+1);coefficient_reads+=fold.reads;configs+=1;assert!(PhaseFold::build(&modes,i,n-1).is_err());caps+=1;
  let mut p=polynomial::Poly::new(r).unwrap();for k in 0..4{let u=fold.phases[k];let mut domain=vec![0;n+1];domain[0]=r-u;domain[n]=1;let expected=canonical_rem(&mut p,&dense.coeff[0],&domain).unwrap();let got=p.add(&fold.residues[k],&[0]);assert_eq!(expected,got);residues+=1;
   for c in 0..r{let mut equation=dense.coeff[0].clone();equation[0]=(equation[0]+r-c)%r;let expected=p.gcd_monic(&domain,&equation,2*n+1).unwrap().polynomial;let got=fold.branch(k,c).unwrap();assert_eq!(expected,got);let g=reciprocal.branch(u,c).unwrap();let old=reciprocal.lift(&g,u,2*n+2).unwrap();assert_eq!(old,got);assert_eq!(squarefree(r,&got).unwrap(),got);supports+=1;let zero=fold.residues[k].iter().enumerate().all(|(j,&a)|a==if j==0{c}else{0});if zero{zero_equations+=1;assert_eq!(got,domain);}else if fold.residues[k][1..].iter().all(|&a|a==0){constant_nonzero+=1;assert_eq!(got,vec![1]);}}
  }
 }}
 assert!(zero_equations>0&&constant_nonzero>0);println!("direct_fold_configs={configs} folded_residue_equalities={residues} oriented_support_equalities={supports} zero_equation_controls={zero_equations} constant_nonzero_controls={constant_nonzero} initial_degree_cap_rejections={caps} upper_coefficient_reads={coefficient_reads} reciprocal_transform_constructed=false");
}
