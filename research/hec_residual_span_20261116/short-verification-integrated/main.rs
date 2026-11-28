mod baseline;mod phase;mod kernel;mod reduce;
#[path="../orbit-core/sparse.rs"]mod sparse;
use hec_native_ic::{counted,fp::Fp,poly::Poly,jac::Curve,rng::Rng,instance::factor_base};
fn main(){
 let expected:Vec<_>=include_str!("../short-verification/DERIVED-VECTORS.tsv").lines().skip(1).map(|l|l.split('\t').collect::<Vec<_>>()).collect();
 let old:Vec<_>=include_str!("../source-phases/CONTROL.tsv").lines().skip(1).map(|l|l.split('\t').collect::<Vec<_>>()).collect();let mut calls=0;
 for(p,cc,ell)in[(101u64,2u64,7481u64),(251,18,67931),(601,7,411491)]{counted::disable();let c=Curve::new(Fp::new(p),Poly::from_coeffs(&[cc,0,0,0,0,1]));let fb=factor_base(&c);let base=c.place(fb[0].0,fb[0].1);let mut rng=Rng::new(2026113000^p);
 for sm in[1,17]{let d=c.mul(&base,sm);for case in 0..2{let k=1+rng.below(ell-1);let t=c.mul(&d,k);let mut prior=None;for rep in 0..2{phase::reset();counted::reset();let report=baseline::solve(&c,&d,&t,ell,sm,false);let counts=counted::snapshot();counted::disable();assert_eq!(report.k,Some(k));assert_eq!(c.mul(&d,k),t);
 let row=expected.iter().find(|r|r[0]==p.to_string()&&r[1]==sm.to_string()&&r[2]==case.to_string()).unwrap();let v:Vec<u64>=row[3].split(',').map(|n|n.parse().unwrap()).collect();assert_eq!(counts[&p].as_slice(),v.as_slice());
 for(&m,v)in &counts{assert_eq!(v[8],2*v[7]+v[5]+v[6]);if m!=p{let mut ex=[0;16];for r in &old{if r[0]==p.to_string()&&r[1]==sm.to_string()&&r[2]==case.to_string()&&r[3]==rep.to_string()&&r[5]==m.to_string(){for(j,n)in r[6].split(',').map(|n|n.parse::<u64>().unwrap()).enumerate(){ex[j]+=n;}}}assert_eq!(*v,ex);}println!("COUNT p={p} sm={sm} case={case} rep={rep} modulus={m} vector={v:?}");}
 let signature=(counts,report.k,report.relations,report.trials,report.parameter,report.source_normalized);if let Some(ref old)=prior{assert_eq!(old,&signature)}else{prior=Some(signature)}calls+=1;
 }}}}
 assert_eq!(calls,24);eprintln!("PASS complete_calls=24 original_targets=12 all_scalars_native_verified=true derived_full_base_vectors_exact=true original_subgroup_vectors_exact=true repeat_counts_and_outcomes=true conditional_operator_contract=true no_wall_or_tenfold_claim=true");
}
