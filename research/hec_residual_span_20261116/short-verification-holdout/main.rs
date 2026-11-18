mod batch;mod pencil;mod walk;mod kernel;mod reduce;
#[path="../setup/fast.rs"]mod fast;
use hec_native_ic::{counted,fp::Fp,jac::{Curve,Div},poly::Poly,rng::Rng};
include!("run.rs");
fn main(){
 let old:Vec<_>=include_str!("../holdout/CONTROL.tsv").lines().skip(1).map(|l|l.split('\t').collect::<Vec<_>>()).collect();let oldpublic=include_str!("../holdout/CONTROL.stderr");let mut calls=0;let mut workloads=0;
 for(g,p,cc,ell,h)in[(2usize,251u64,3u64,101u64,55u64),(2,1021,4,631,1655),(3,211,2,301267,29),(3,421,2,77152909,1),(4,181,2,48781,4445),(4,271,2,397,112347)]{
 counted::disable();let n=2*g+1;let mut cs=vec![0;n+1];cs[0]=cc;cs[n]=1;let c=Curve::new(Fp::new(p),Poly::from_coeffs(&cs));let(fb,z,pc)=fast::prepare(&c);let source=fb.iter().map(|&(x,y)|c.mul(&c.place(x,y),h)).find(|d|!c.is_zero(d)).unwrap();let mut relation:Vec<_>=pc.iter().map(|v|*v as i128).collect();relation[0]-=1;let mut rng=Rng::new(20261117^((g as u64)<<32)^p);
 for sm in[1u64,17]{let d=c.mul(&source,sm);assert!(c.is_valid(&d));assert!(!c.is_zero(&d));assert!(c.is_zero(&c.mul(&d,ell)));assert!(c.is_zero(&kernel::evaluate(&c,&d,z,&relation)));assert!(c.is_zero(&kernel::evaluate(&c,&d,z,&vec![1;n])));let mut knowns=vec![];
 for case in 0..2{let mut known=1+rng.below(ell-1);while knowns.contains(&known){known=1+rng.below(ell-1);}knowns.push(known);let t=c.mul(&d,known);
 let receipt=format!("public g={g} p={p} sm={sm} case={case} expected={known} source={d:?} target={t:?}");assert!(oldpublic.lines().any(|l|l==receipt));
 let(coeff,witness,moves)=reduce::short(known,&relation);assert_eq!(kernel::evaluate(&c,&d,z,&coeff),t);let(wrong,_,_)=reduce::short((known+1)%ell,&relation);assert_ne!(kernel::evaluate(&c,&d,z,&wrong),t);assert_ne!(kernel::evaluate(&c,&d,z,&coeff),c.neg(&t));assert_ne!(kernel::evaluate(&c,&c.neg(&d),z,&coeff),t);
 println!("CERTIFICATE g={g} p={p} sm={sm} case={case} k={known} R={relation:?} short={coeff:?} witness={witness:?} greedy_moves={moves}");
 let mut scalar=None;let mut polynomial=None;
 for name in["scalar","polynomial"]{let mut prior=None;for rep in 0..2{counted::reset();let answer=if name=="scalar"{c.mul(&d,known)}else{let(co,_,_)=reduce::short(known,&relation);kernel::evaluate(&c,&d,z,&co)};let counts=counted::snapshot();counted::disable();assert_eq!(answer,t);if let Some(ref old)=prior{assert_eq!(old,&counts);}else{prior=Some(counts.clone());}let v=counts[&p];assert_eq!(v[8],2*v[7]+v[5]+v[6]);println!("KERNEL g={g} p={p} sm={sm} case={case} name={name} rep={rep} vector={v:?}");if name=="scalar"{scalar=Some(v);}else{polynomial=Some(v);}}}
 let sv=scalar.unwrap();let pv=polynomial.unwrap();
 for arm in 0..4{let name=["batch","pencil","folded","expanded"][arm];let mut prior=None;for rep in 0..2{counted::reset();let outcome=run(arm,&c,&d,&t,ell,case);let counts=counted::snapshot();counted::disable();assert_eq!(outcome.0,Some(known));assert_eq!(c.mul(&d,known),t);
 for(&modulus,v)in &counts{let row=old.iter().find(|r|r[0]==g.to_string()&&r[1]==p.to_string()&&r[2]==sm.to_string()&&r[3]==case.to_string()&&r[4]==name&&r[5]==rep.to_string()&&r[6]==modulus.to_string()).unwrap();let ov:Vec<u64>=row[8..24].iter().map(|x|x.parse().unwrap()).collect();let mut expected=ov.clone();if modulus==p{for j in 0..16{assert!(expected[j]>=sv[j]);expected[j]=expected[j]-sv[j]+pv[j];}}assert_eq!(v.as_slice(),expected.as_slice());assert_eq!(v[8],2*v[7]+v[5]+v[6]);println!("COMPLETE g={g} p={p} sm={sm} case={case} arm={name} rep={rep} modulus={modulus} old_redc={} new_redc={} vector={v:?}",ov[8],v[8]);}
 if let Some(ref old)=prior{assert_eq!(old,&(outcome,counts.clone()));}else{prior=Some((outcome,counts));}calls+=1;
 }}workloads+=1;
 }}}
 assert_eq!(calls,192);assert_eq!(workloads,24);eprintln!("PASS complete_calls=192 workloads=24 all_three_genera=true old_public_input_receipts_exact=true all_scalars_native_verified=true old_minus_scalar_plus_polynomial_full_vectors_exact=true subgroup_vectors_unchanged=true mutations_rejected=true repeats_exact=true conditional_operator_contract=true no_wall_or_tenfold_claim=true");
}
